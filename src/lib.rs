use chrono::{DateTime, Utc};
use graphql_client::{reqwest::post_graphql, GraphQLQuery, Response};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use thiserror::Error;

/// A JWT token used for authenticated API requests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthToken(String);

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("HTTP request error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("GraphQL errors: {0:?}")]
    GraphQlErrors(Option<Vec<graphql_client::Error>>),
    #[error("REST error {status}: {body}")]
    RestError { status: StatusCode, body: String },
}

pub async fn authenticate(email: &str, password: &str) -> Result<AuthToken, ApiError> {
    let client = Client::new();
    let variables = authenticate_query::Variables {
        email: email.to_owned(),
        password: password.to_owned(),
    };
    let response: Response<authenticate_query::ResponseData> =
        post_graphql::<AuthenticateQuery, &str>(
            &client,
            "https://api.octopus.energy/v1/graphql/",
            variables,
        )
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
}

pub async fn consumption(
    auth_token: &AuthToken,
    meter_type: MeterType,
    mpxn: &str,
    serial: &str,
    page: u32,
    page_size: usize,
) -> Result<Readings, ApiError> {
    let client = Client::new();
    let url = format!(
        "https://api.octopus.energy/v1/{}/{}/meters/{}/consumption/?page={}&page_size={}",
        meter_type.path_component(),
        mpxn,
        serial,
        page,
        page_size,
    );
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

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Readings {
    pub count: usize,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Consumption>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Consumption {
    pub consumption: f32,
    pub interval_start: DateTime<Utc>,
    pub interval_end: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn deserialize_consumption() {
        assert_eq!(
            serde_json::from_str::<Consumption>(
                r#"{
                    "consumption": 0.42,
                    "interval_start": "2021-12-31T22:00:00Z",
                    "interval_end": "2021-12-31T22:30:00Z"
                }"#
            )
            .unwrap(),
            Consumption {
                consumption: 0.42,
                interval_start: Utc.ymd(2021, 12, 31).and_hms(22, 0, 0),
                interval_end: Utc.ymd(2021, 12, 31).and_hms(22, 30, 0)
            }
        );
    }
}
