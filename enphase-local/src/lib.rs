// Copyright 2024 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! A client library for the Enphase Envoy local API.

pub mod home;
pub mod inventory;
pub mod production;
mod timestamp_string;

use home::Home;
use inventory::Inventory;
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
}
