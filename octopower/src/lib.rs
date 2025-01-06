// Copyright 2022 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! A client library for the [Octopus Energy API](https://developer.octopus.energy/docs/api/).
//!
//! # Usage
//!
//! To login and fetch account information:
//!
//! ```rust
//! use octopower::{authenticate, get_account};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let token = authenticate("email@address.example", "password").await?;
//! let account = get_account(&token, "A-1234ABCD").await?;
//! println!("Account information: {:?}", account);
//! # Ok(()) }
//! ```

pub mod results;

use graphql_client::{GraphQLQuery, Response};
use reqwest::{Client, StatusCode, Url};
use results::{account::Account, consumption::Readings, standing_unit_rate::StandingUnitRates};
use std::fmt::{self, Display, Formatter};
use thiserror::Error;
use url::ParseError;

/// A JWT token used for authenticated API requests.
///
/// This can be obtained by calling [`authenticate`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthToken(String);

/// An error communicating with the Octopus API.
#[derive(Debug, Error)]
pub enum ApiError {
    /// There was an error making the HTTP request.
    #[error("HTTP request error: {0}")]
    HttpError(#[from] reqwest::Error),
    /// A GraphQL query returned an error.
    #[error("GraphQL errors: {0:?}")]
    GraphQlErrors(Option<Vec<graphql_client::Error>>),
    /// A REST API method returned an error status.
    #[error("REST error {status}: {body}")]
    RestError { status: StatusCode, body: String },
    /// There was an error parsing a URL from a string.
    #[error("Error parsing URL: {0}")]
    UrlParseError(#[from] ParseError),
}

/// Authenticate to the Octopus API using the given email address and password, returning an
/// authentication token which can be used for subsequent authenticated requests.
pub async fn authenticate(email: &str, password: &str) -> Result<AuthToken, ApiError> {
    let client = Client::new();
    let variables = authenticate_query::Variables {
        email: email.to_owned(),
        password: password.to_owned(),
    };
    let query = AuthenticateQuery::build_query(variables);
    let response: Response<authenticate_query::ResponseData> = client
        .post("https://api.octopus.energy/v1/graphql/")
        .json(&query)
        .send()
        .await?
        .json()
        .await?;
    if let Some(authenticate_query::ResponseData {
        obtain_kraken_token: Some(token),
    }) = response.data
    {
        Ok(AuthToken(token.token))
    } else {
        Err(ApiError::GraphQlErrors(response.errors))
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/authenticate.graphql"
)]
struct AuthenticateQuery;

/// Fetch information about the given account from the Octopus REST API.
pub async fn get_account(auth_token: &AuthToken, account_id: &str) -> Result<Account, ApiError> {
    let client = Client::new();
    let url = format!("https://api.octopus.energy/v1/accounts/{}/", account_id);
    let response = client
        .get(url)
        .header("Authorization", &auth_token.0)
        .send()
        .await?;

    let status = response.status();
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let body = response.text().await?;
        Err(ApiError::RestError { status, body })
    }
}

/// The type of meter, either electricity or gas.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MeterType {
    Electricity,
    Gas,
}

impl MeterType {
    fn path_component(self) -> &'static str {
        match self {
            Self::Electricity => "electricity-meter-points",
            Self::Gas => "gas-meter-points",
        }
    }

    fn tariffs_path(self) -> &'static str {
        match self {
            Self::Electricity => "electricity-tariffs",
            Self::Gas => "gas-tariffs",
        }
    }
}

impl Display for MeterType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Electricity => "electricity",
            Self::Gas => "gas",
        })
    }
}

/// The level of aggregation with which to group electricity or gas consumption records.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Grouping {
    Hour,
    Day,
    Week,
    Month,
    Quarter,
}

impl Grouping {
    fn as_str(self) -> &'static str {
        match self {
            Self::Hour => "hour",
            Self::Day => "day",
            Self::Week => "week",
            Self::Month => "month",
            Self::Quarter => "quarter",
        }
    }
}

impl Display for Grouping {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Fetch electricity or gas consumption records from the meter with the given `mpxn` (MPAN or MPRN)
/// and serial.
///
/// Electricity meters have an MPAN (Meter Point Administration Number), also known as an
/// Electricity Supply Number, while gas meters have an MPRN (Meter Point Reference Number), also
/// known as a Gas Supply Number.
///
/// If `grouping` is `None` then raw half-hourly records will be returned.
///
/// Because there may be a large number of records, they can be fetched in multiple pages. Page 0
/// has the most recent `page_size` records, subsequent pages have older records.
pub async fn get_consumption(
    auth_token: &AuthToken,
    meter_type: MeterType,
    mpxn: &str,
    serial: &str,
    page: u32,
    page_size: usize,
    grouping: Option<Grouping>,
) -> Result<Readings, ApiError> {
    let client = Client::new();
    let mut url = Url::parse(&format!(
        "https://api.octopus.energy/v1/{}/{}/meters/{}/consumption/?page={}&page_size={}",
        meter_type.path_component(),
        mpxn,
        serial,
        page + 1,
        page_size,
    ))?;
    if let Some(grouping) = grouping {
        url.query_pairs_mut()
            .append_pair("group_by", grouping.as_str());
    }

    let response = client
        .get(url)
        .header("Authorization", &auth_token.0)
        .send()
        .await?;

    let status = response.status();
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let body = response.text().await?;
        Err(ApiError::RestError { status, body })
    }
}

/// Fetch Agile Octopus Electricity-tariffs
pub async fn get_standard_unit_rates(
    auth_token: &AuthToken,
    meter_type: MeterType,
    product_code: &str,
    tariff_code: &str,
    page: u32,
    page_size: usize,
) -> Result<StandingUnitRates, ApiError> {
    let client = Client::new();
    let url = Url::parse(&format!(
        "https://api.octopus.energy/v1/products/{}/{}/{}/standard-unit-rates/?page={}&page_size={}",
        product_code,
        meter_type.tariffs_path(),
        tariff_code,
        page + 1,
        page_size,
    ))?;
    dbg!(&url);
    let response = client
        .get(url)
        .header("Authorization", &auth_token.0)
        .send()
        .await?;

    let status = response.status();
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let body = response.text().await?;
        Err(ApiError::RestError { status, body })
    }
}
