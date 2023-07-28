use std::borrow::Borrow;

use base64::{engine::general_purpose, Engine};
use http_types::{Method, Request, Response};
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

use crate::{
    errors::{to_domain_error, to_domain_error_with_context, DomeneshopApiError, DomeneshopError},
    http::HttpClient,
};

/// Configuration of the created client
pub struct DomeneshopClientConfiguration {
    /// Overrides default user agent-header if set
    pub user_agent: Option<String>,
    /// Overrides default base url if set
    pub base_url: Option<String>,
    /// Sets an optional underlying client (only with `reqwest` feature enabled)
    #[cfg(feature = "reqwest")]
    pub underlying_client: Option<Box<dyn HttpClient + 'static>>,
    /// Sets a required underlying client (only with `reqwest` feature disabled)
    #[cfg(not(feature = "reqwest"))]
    pub underlying_client: Box<dyn HttpClient>,
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

/// The client used to interact with the domeneshop API.
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

/// Determines the API version the client will use
pub const API_VERSION: &str = "v0";

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
            base_url: format!("{}/{}", strip_trailing_slash(base_url), API_VERSION),
            auth_header: header,
            user_agent,
        })
    }

    pub(crate) fn create_url(
        &self,
        relative_url: impl Into<String>,
    ) -> Result<Url, DomeneshopError> {
        let url = format!("{}/{}", self.base_url, strip_leading_slash(relative_url));
        Url::parse(url.as_str()).map_err(to_domain_error)
    }

    pub(crate) fn create_url_with_parameters<S, I, K, V>(
        &self,
        relative_url: S,
        query_parameters: I,
    ) -> Result<Url, DomeneshopError>
    where
        S: Into<String>,
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let url = format!("{}/{}", self.base_url, strip_leading_slash(relative_url));
        Url::parse_with_params(url.as_str(), query_parameters).map_err(to_domain_error)
    }

    pub(crate) async fn send_no_validation(
        &self,
        mut req: Request,
    ) -> Result<Response, DomeneshopError> {
        req.insert_header("Authorization", &self.auth_header);
        req.insert_header("User-Agent", &self.user_agent);
        self.client.execute_request(req).await
    }

    pub(crate) async fn send(&self, req: Request) -> Result<Response, DomeneshopError> {
        let response = self.send_no_validation(req).await?;
        if !response.status().is_success() {
            Err(handle_response_error(response).await)
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
        response.body_json().await.map_err(to_domain_error)
    }

    pub(crate) async fn get_response<T>(&self, url: Url) -> Result<T, DomeneshopError>
    where
        T: DeserializeOwned,
    {
        let request = Request::new(Method::Get, url);
        let response = self.send(request).await?;

        self.deserialize_response(response).await
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

fn strip_trailing_slash(s: impl Into<String>) -> String {
    let s: String = s.into();
    if s.ends_with('/') {
        s[..s.len() - 1].to_string()
    } else {
        s
    }
}

fn strip_leading_slash(s: impl Into<String>) -> String {
    let s: String = s.into();
    if s.starts_with('/') {
        s[1..].to_string()
    } else {
        s
    }
}

fn create_basic_auth_header(token: String, secret: String) -> String {
    let val = format!("{}:{}", token, secret);
    let encoded = general_purpose::STANDARD_NO_PAD.encode(val);
    format!("Basic {}", encoded)
}

pub(crate) fn set_body<T>(request: &mut Request, model: T)
where
    T: Serialize,
{
    let json = serde_json::to_vec(&model).unwrap();
    request.set_body(json);
}

pub(crate) async fn handle_response_error(mut response: Response) -> DomeneshopError {
    match response.body_json::<DomeneshopApiError>().await {
        Ok(error) => error.into(),
        Err(err) => to_domain_error_with_context("Failed to deserialize error response", err),
    }
}

#[cfg(test)]
mod tests {
    use tokio;

    use http_types::{Request, Response, StatusCode};

    use crate::{
        client::{strip_leading_slash, strip_trailing_slash},
        http_client::mock::MockClient,
    };

    use super::{create_basic_auth_header, DomeneshopClient, DomeneshopError};

    async fn return_ok(_: Request) -> Result<Response, DomeneshopError> {
        Ok(Response::new(StatusCode::Ok))
    }

    #[test]
    fn create_basic_auth_header_creates_valid_header() {
        let token = String::from("token");
        let secret = String::from("secret");
        let result = create_basic_auth_header(token, secret);

        assert_eq!(result, "Basic dG9rZW46c2VjcmV0");
    }

    #[test]
    fn create_url_creates_valid_absolute_urls() {
        let mock = MockClient {
            req_received: return_ok,
        };

        let client = create_client(mock);

        let url = client.create_url("/test").unwrap();
        let url2 = client.create_url("test").unwrap();

        assert_eq!(url.to_string(), "https://api.domeneshop.no/v0/test");
        assert_eq!(url2.to_string(), "https://api.domeneshop.no/v0/test");
    }

    #[test]
    fn strip_trailing_slash_strips_correctly() {
        assert_eq!(strip_trailing_slash(""), "");
        assert_eq!(strip_trailing_slash("/"), "");
        assert_eq!(strip_trailing_slash("/a"), "/a");
        assert_eq!(strip_trailing_slash("/a/"), "/a");
        assert_eq!(strip_trailing_slash("/a/b/"), "/a/b");
    }

    #[test]
    fn strip_leading_slash_strips_correctly() {
        assert_eq!(strip_leading_slash(""), "");
        assert_eq!(strip_leading_slash("/"), "");
        assert_eq!(strip_leading_slash("/a"), "a");
        assert_eq!(strip_leading_slash("/a/"), "a/");
        assert_eq!(strip_leading_slash("/a/b/"), "a/b/");
    }

    #[test]
    fn create_url_with_parameters_creates_valid_absolute_urls() {
        let mock = MockClient {
            req_received: return_ok,
        };

        let client = create_client(mock);

        let url = client
            .create_url_with_parameters("/test", &[("test", "val")])
            .unwrap();
        let url2 = client
            .create_url_with_parameters("test", &[("test", "val"), ("test2", "val2")])
            .unwrap();

        assert_eq!(
            url.to_string(),
            "https://api.domeneshop.no/v0/test?test=val"
        );
        assert_eq!(
            url2.to_string(),
            "https://api.domeneshop.no/v0/test?test=val&test2=val2"
        );
    }

    #[tokio::test]
    async fn send_adds_auth_header() {
        async fn received(req: Request) -> Result<Response, DomeneshopError> {
            let val = req.header("Authorization").unwrap();
            assert_eq!(val, "Basic dG9rZW46c2VjcmV0");
            Ok(Response::new(StatusCode::Ok))
        }
        let mock = MockClient {
            req_received: received,
        };

        let client = create_client(mock);

        _ = client
            .send(Request::get("https://api.domeneshop.no/v0/test"))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn send_adds_useragent_header() {
        const USER_AGENT: &str = "testagent";
        async fn received(req: Request) -> Result<Response, DomeneshopError> {
            let val = req.header("User-Agent").unwrap();
            assert_eq!(val, USER_AGENT);
            Ok(Response::new(StatusCode::Ok))
        }
        let mock = MockClient {
            req_received: received,
        };
        let client = DomeneshopClient::new(
            String::from("token"),
            String::from("secret"),
            super::DomeneshopClientConfiguration {
                user_agent: Some(USER_AGENT.to_string()),
                base_url: None,
                underlying_client: Some(Box::new(mock)),
            },
        )
        .unwrap();

        _ = client
            .send(Request::get("https://api.domeneshop.no/v0/test"))
            .await
            .unwrap();
    }

    fn create_client<'a, F>(client: MockClient<F>) -> DomeneshopClient
    where
        F: std::future::Future<Output = Result<Response, DomeneshopError>> + Send + 'static,
    {
        DomeneshopClient::new(
            String::from("token"),
            String::from("secret"),
            super::DomeneshopClientConfiguration {
                user_agent: None,
                base_url: None,
                underlying_client: Some(Box::new(client)),
            },
        )
        .unwrap()
    }
}
