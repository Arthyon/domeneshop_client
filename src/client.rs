use std::fmt::Display;

use base64::{engine::general_purpose, Engine};
use http_types::{Request, Response};
use serde::{de::DeserializeOwned, Deserialize};
use url::Url;

use crate::{error_mapping::map_http_types_error, http::HttpClient};

/// Configuration of the created client
pub struct DomeneshopClientConfiguration {
    user_agent: Option<String>,
    base_url: Option<String>,
    #[cfg(feature = "reqwest")]
    underlying_client: Option<Box<dyn HttpClient>>,
    #[cfg(not(feature = "reqwest"))]
    underlying_client: Box<dyn HttpClient>,
}

#[cfg(feature = "reqwest")]
impl DomeneshopClientConfiguration {
    /// Creates a default configuration for the domeneshop client
    pub fn default() -> DomeneshopClientConfiguration {
        DomeneshopClientConfiguration {
            user_agent: None,
            base_url: None,
            underlying_client: None,
        }
    }
}

/// The error structure returned from the Domeneshop API.
/// This is also used for all other errors emitted from this crate.
#[derive(Deserialize, Debug)]
pub struct DomeneshopError {
    /// Additional information about the error
    pub help: String,
    /// A shorter code describing the error
    pub code: String,
}

/// The client used to interact with the domeneshop API.
/// TODO Add example
pub struct DomeneshopClient {
    client: Box<dyn HttpClient>,
    base_url: String,
    auth_header: String,
    user_agent: String,
}

const DEFAULT_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_REPOSITORY"),
    " - version ",
    env!("CARGO_PKG_VERSION")
);
const DEFAULT_BASE_URL: &str = "https://api.domeneshop.no/";
const API_VERSION: &str = "v0";

impl DomeneshopClient {
    /// Creates a new domeneshop client
    pub fn new(
        token: String,
        secret: String,
        configuration: DomeneshopClientConfiguration,
    ) -> Result<Self, DomeneshopError> {
        let user_agent = configuration
            .user_agent
            .clone()
            .unwrap_or(DEFAULT_USER_AGENT.to_string());
        let base_url = configuration
            .base_url
            .clone()
            .unwrap_or(DEFAULT_BASE_URL.to_string());

        let client = create_client(configuration)?;
        let header = create_basic_auth_header(token, secret);
        Ok(DomeneshopClient {
            client,
            base_url,
            auth_header: header,
            user_agent,
        })
    }

    pub(crate) fn create_url<S>(&self, relative_url: S) -> Result<Url, DomeneshopError>
    where
        S: Into<String> + Display,
    {
        Url::parse(format!("{}/{}{}", self.base_url, API_VERSION, relative_url).as_str()).map_err(
            |err| DomeneshopError {
                help: format!("Failed to parse url: {}", err),
                code: "InfrastructureError".to_string(),
            },
        )
    }

    pub(crate) async fn send(&self, mut req: Request) -> Result<Response, DomeneshopError> {
        req.insert_header("Authorization", &self.auth_header);
        req.insert_header("User-Agent", &self.user_agent);
        let mut response = self.client.execute_request(req).await?;

        if !response.status().is_success() {
            match response.body_json::<DomeneshopError>().await {
                Ok(error) => Err(error),
                Err(err) => Err(map_http_types_error(
                    "Failed to deserialize error response",
                    err,
                )),
            }
        } else {
            Ok(response)
        }
    }

    pub(crate) async fn deserialize_response<T>(
        &self,
        mut response: Response,
    ) -> Result<T, DomeneshopError>
    where
        T: DeserializeOwned,
    {
        response
            .body_json()
            .await
            .map_err(|err| map_http_types_error("Failed to deserialize response body", err))
    }
}

#[cfg(not(feature = "reqwest"))]
fn create_client(
    config: DomeneshopClientConfiguration,
) -> Result<Box<dyn HttpClient>, DomeneshopError> {
    Ok(config.underlying_client)
}

#[cfg(feature = "reqwest")]
fn create_client(
    config: DomeneshopClientConfiguration,
) -> Result<Box<dyn HttpClient>, DomeneshopError> {
    use crate::http_client;

    if let Some(client) = config.underlying_client {
        Ok(client)
    } else {
        let client = reqwest::Client::builder().build().map_err(|err| {
            http_client::reqwest::map_reqwest_error("Failed to build reqwest client", err)
        })?;

        Ok(Box::new(client))
    }
}

fn create_basic_auth_header(token: String, secret: String) -> String {
    let val = format!("{}:{}", token, secret);
    let encoded = general_purpose::STANDARD_NO_PAD.encode(val);
    format!("Basic {}", encoded)
}
