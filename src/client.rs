use std::fmt::Display;

use reqwest::{Client, RequestBuilder, Response};
use serde::Deserialize;

use crate::helpers::map_reqwest_err;

#[derive(Default)]
pub struct DomeneshopClientConfiguration {
    user_agent: Option<String>,
    base_url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct DomeneshopError {
    pub help: String,
    pub code: String,
}

pub struct DomeneshopClient {
    pub(crate) client: Client,
    pub(crate) base_url: String,
    token: String,
    secret: String,
}

const DEFAULT_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_REPOSITORY"),
    " - version ",
    env!("CARGO_PKG_VERSION")
);
const DEFAULT_BASE_URL: &str = "https://api.domeneshop.no/";

impl DomeneshopClient {
    pub fn new(
        token: String,
        secret: String,
        configuration: DomeneshopClientConfiguration,
    ) -> Result<Self, DomeneshopError> {
        let user_agent = configuration
            .user_agent
            .unwrap_or(DEFAULT_USER_AGENT.to_string());
        let base_url = configuration
            .base_url
            .unwrap_or(DEFAULT_BASE_URL.to_string());

        let client = Client::builder()
            .user_agent(user_agent)
            .build()
            .map_err(map_reqwest_err)?;

        Ok(DomeneshopClient {
            client,
            base_url,
            token,
            secret,
        })
    }

    pub(crate) fn create_absolute_url<S>(&self, relative_url: S) -> String
    where
        S: Into<String> + Display,
    {
        format!("{}/v0{}", self.base_url, relative_url)
    }

    pub(crate) async fn send(&self, req: RequestBuilder) -> Result<Response, DomeneshopError> {
        req.basic_auth(&self.token, Some(&self.secret))
            .send()
            .await
            .map_err(map_reqwest_err)
    }
}
