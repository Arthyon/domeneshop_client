use async_trait::async_trait;
use http_types::{Request, Response};

use crate::{client::DomeneshopError, http::HttpClient};

/// A mock client that can be used for testing code using this crate
pub struct MockClient {}

#[async_trait]
impl HttpClient for MockClient {
    async fn execute_request(&self, request: Request) -> Result<Response, DomeneshopError> {
        Err(DomeneshopError {
            help: "".to_string(),
            code: "".to_string(),
        })
    }
}
