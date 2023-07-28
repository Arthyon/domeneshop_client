use std::fmt::Display;

use async_trait::async_trait;
use http_types::{Method, Request, Response, StatusCode};

use crate::{errors::to_domain_error, errors::DomeneshopError, http::HttpClient};

#[async_trait]
impl HttpClient for ::reqwest::Client {
    async fn execute_request(&self, mut request: Request) -> Result<Response, DomeneshopError> {
        let url = request.url();
        let method = request.method();
        let mut request_builder = self.request(map_method(method)?, url.clone());
        for (name, value) in request.iter() {
            request_builder = request_builder.header(name.as_str(), value.as_str());
        }

        let body = request.take_body();
        if let Some(is_empty) = body.is_empty() {
            if !is_empty {
                let body_data = body.into_bytes().await.map_err(to_domain_error)?;
                request_builder = request_builder.body(body_data);
            }
        }

        let reqwest_request = request_builder
            .build()
            .map_err(|err| map_reqwest_error("Failed to build reqwest request", err))?;

        let rsp = self
            .execute(reqwest_request)
            .await
            .map_err(|err| map_reqwest_error("Sending request failed", err))?;

        let status = rsp.status();
        let body = rsp
            .text()
            .await
            .map_err(|err| map_reqwest_error("Failed to extract reqwest body", err))?;

        let mut resp = Response::new(map_status(status)?);
        resp.set_body(body);
        Ok(resp)
    }
}

fn map_method(method: Method) -> Result<::reqwest::Method, DomeneshopError> {
    match method {
        Method::Delete => Ok(::reqwest::Method::DELETE),
        Method::Get => Ok(::reqwest::Method::GET),
        Method::Post => Ok(::reqwest::Method::POST),
        Method::Put => Ok(::reqwest::Method::PUT),
        _ => Err(DomeneshopError::new(format!("Invalid method: {}", method))),
    }
}

fn map_status(status: ::reqwest::StatusCode) -> Result<StatusCode, DomeneshopError> {
    let status = u16::from(status);
    StatusCode::try_from(status).map_err(to_domain_error)
}

pub fn map_reqwest_error(
    context: impl Into<String> + Display,
    error: reqwest::Error,
) -> DomeneshopError {
    DomeneshopError::new(format!("{}: {}", context, error))
}
