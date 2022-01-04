// Copyright 2022 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

use eyre::{Report, WrapErr};
use influx_db_client::{reqwest::Url, Client};
use serde::Deserialize;
use std::fs::read_to_string;

const DEFAULT_DATABASE: &str = "octopower";
const DEFAULT_MEASUREMENT: &str = "octopower";
const DEFAULT_INFLUXDB_URL: &str = "http://localhost:8086";
const DEFAULT_NUM_READINGS: usize = 1000;
const CONFIG_FILENAME: &str = "octo-influx.toml";

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub influxdb: InfluxDbConfig,
    pub octopus: OctopusConfig,
    #[serde(default = "default_num_readings")]
    pub num_readings: usize,
}

fn default_num_readings() -> usize {
    DEFAULT_NUM_READINGS
}

impl Config {
    pub fn from_file() -> Result<Config, Report> {
        Config::read(CONFIG_FILENAME)
    }

    fn read(filename: &str) -> Result<Config, Report> {
        let config_file =
            read_to_string(filename).wrap_err_with(|| format!("Reading {}", filename))?;
        Ok(toml::from_str(&config_file)?)
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct InfluxDbConfig {
    pub url: Url,
    pub username: Option<String>,
    pub password: Option<String>,
    pub database: String,
    pub measurement: String,
}

impl Default for InfluxDbConfig {
    fn default() -> InfluxDbConfig {
        InfluxDbConfig {
            url: DEFAULT_INFLUXDB_URL.parse().unwrap(),
            username: None,
            password: None,
            database: DEFAULT_DATABASE.to_owned(),
            measurement: DEFAULT_MEASUREMENT.to_owned(),
        }
    }
}

/// Construct a new InfluxDB [`Client`] based on the given configuration options, for the given
/// database.
pub fn get_influxdb_client(config: &InfluxDbConfig) -> Result<Client, Report> {
    let mut influxdb_client = Client::new(config.url.to_owned(), &config.database);
    if let (Some(username), Some(password)) = (&config.username, &config.password) {
        influxdb_client = influxdb_client.set_authentication(username, password);
    }
    Ok(influxdb_client)
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OctopusConfig {
    pub email_address: String,
    pub password: String,
    pub account_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parsing the example config file should not give any errors.
    #[test]
    fn example_config() {
        Config::read("octo-influx.example.toml").unwrap();
    }

    /// Parsing an empty config file should give an error.
    #[test]
    fn empty_config() {
        assert!(toml::from_str::<Config>("").is_err());
    }
}
