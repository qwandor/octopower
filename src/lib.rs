use graphql_client::{reqwest::post_graphql, GraphQLQuery, Response};
use reqwest::Client;
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
