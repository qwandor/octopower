// Copyright 2024 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! A client library for the Enphase Envoy local API.
//!
//! # Example
//!
//! ```
//! use enphase_local::Envoy;
//! use reqwest::Url;
//!
//! const AUTH_TOKEN: &str = "...";
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let envoy = Envoy::new(Url::parse("https://envoy.local/")?, AUTH_TOKEN);
//! let production = envoy.production().await?;
//! println!("Production: {:?}", production);
//! # Ok(()) }
//! ```

pub mod home;
pub mod inventory;
pub mod inverters;
pub mod meters;
pub mod production;
mod timestamp_string;

use home::Home;
use inventory::Inventory;
use inverters::Inverter;
use meters::{Reading, Report};
use production::Production;
use reqwest::{Client, Error, Url};

/// Client for the Enphase Envoy local API.
#[derive(Clone, Debug)]
pub struct Envoy {
    base_url: Url,
    auth_token: String,
    client: Client,
}

impl Envoy {
    /// Constructs a new Enphase Envoy local API client with the given base URL and auth token.
    pub fn new(base_url: Url, auth_token: &str) -> Self {
        Self {
            base_url,
            auth_token: auth_token.to_owned(),
            client: Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap(),
        }
    }

    /// Returns a summary of the gateway status.
    pub async fn home(&self) -> Result<Home, Error> {
        self.client
            .get(self.base_url.join("home.json").unwrap())
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    /// Returns an inventory of devices in the system.
    pub async fn inventory(&self, include_deleted: bool) -> Result<Inventory, Error> {
        let mut url = self.base_url.join("inventory.json").unwrap();
        url.set_query(Some(&format!(
            "deleted={}",
            if include_deleted { 1 } else { 0 }
        )));
        self.client
            .get(url)
            .bearer_auth(&self.auth_token)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    /// Returns statistics about current and past production and consumption.
    pub async fn production(&self) -> Result<Production, Error> {
        self.client
            .get(self.base_url.join("production.json?details=1").unwrap())
            .bearer_auth(&self.auth_token)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    /// Gets readings from all meters.
    pub async fn meter_readings(&self) -> Result<Vec<Reading>, Error> {
        self.client
            .get(self.base_url.join("ivp/meters/readings").unwrap())
            .bearer_auth(&self.auth_token)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    /// Gets reports from all meters.
    pub async fn meter_reports(&self) -> Result<Vec<Report>, Error> {
        self.client
            .get(self.base_url.join("ivp/meters/reports").unwrap())
            .bearer_auth(&self.auth_token)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    /// Gets individual inverter production data from the v1 API.
    pub async fn inverters(&self) -> Result<Vec<Inverter>, Error> {
        self.client
            .get(self.base_url.join("api/v1/production/inverters").unwrap())
            .bearer_auth(&self.auth_token)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }
}
