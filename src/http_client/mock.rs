use async_trait::async_trait;
use http_types::{Request, Response};

use crate::{client::DomeneshopError, http::HttpClient};

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
