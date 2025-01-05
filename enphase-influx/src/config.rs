// Copyright 2025 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

use eyre::{Report, WrapErr};
use influx_db_client::{reqwest::Url, Client};
use serde::{Deserialize, Deserializer};
use std::{fs::read_to_string, time::Duration};

const DEFAULT_DATABASE: &str = "enphase";
const DEFAULT_INFLUXDB_URL: &str = "http://localhost:8086";
const CONFIG_FILENAME: &str = "enphase-influx.toml";

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(
        deserialize_with = "de_duration_seconds",
        rename = "poll_period_seconds"
    )]
    pub poll_period: Duration,
    #[serde(default)]
    pub influxdb: InfluxDbConfig,
    pub enphase: EnphaseConfig,
}

pub fn de_duration_seconds<'de, D: Deserializer<'de>>(d: D) -> Result<Duration, D::Error> {
    let seconds = u64::deserialize(d)?;
    Ok(Duration::from_secs(seconds))
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
}

impl Default for InfluxDbConfig {
    fn default() -> InfluxDbConfig {
        InfluxDbConfig {
            url: DEFAULT_INFLUXDB_URL.parse().unwrap(),
            username: None,
            password: None,
            database: DEFAULT_DATABASE.to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnphaseConfig {
    pub base_url: Url,
    pub token: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Parsing the example config file should not give any errors.
    #[test]
    fn example_config() {
        Config::read("enphase-influx.example.toml").unwrap();
    }

    /// Parsing an empty config file should give an error.
    #[test]
    fn empty_config() {
        assert!(toml::from_str::<Config>("").is_err());
    }
}
