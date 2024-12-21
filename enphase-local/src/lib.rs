// Copyright 2024 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! A client library for the Enphase Envoy local API.

mod home;

pub use home::Home;
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
}
