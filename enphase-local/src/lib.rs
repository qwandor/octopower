// Copyright 2024 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! A client library for the Enphase Envoy local API.

pub mod home;
pub mod inventory;
mod timestamp_string;

use home::Home;
use inventory::Inventory;
use reqwest::{Client, Error, Url};

/// Client for the Enphase Envoy local API.
#[derive(Clone, Debug)]
pub struct Envoy {
    base_url: Url,
    auth_token: String,
    client: Client,
}

impl Envoy {
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

    pub async fn home(&self) -> Result<Home, Error> {
        self.client
            .get(self.base_url.join("home.json").unwrap())
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

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
}
