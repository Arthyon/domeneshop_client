use async_trait::async_trait;
use http_types::{Request, Response};

use crate::{client::DomeneshopError, http::HttpClient};

/// A mock client that can be used for testing code using this crate
pub struct MockClient {
    /// Callback that is invoked when a request is received
    pub req_received: fn(&Request) -> Result<Response, DomeneshopError>,
}

#[async_trait]
impl HttpClient for MockClient {
    async fn execute_request(&self, request: Request) -> Result<Response, DomeneshopError> {
        (self.req_received)(&request)
        // (self.req_received)(&request)
    }
}
