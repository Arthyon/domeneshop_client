use async_trait::async_trait;
use http_types::{Request, Response};

use crate::{client::DomeneshopError, http::HttpClient};

/// A mock client that can be used for testing code using this crate
pub struct MockClient<F>
where
    F: std::future::Future<Output = Result<Response, DomeneshopError>> + Send,
{
    /// Callback that is invoked when a request is received
    // pub req_received: fn(Request) -> Result<Response, DomeneshopError>,
    pub req_received: fn(Request) -> F,
}

#[async_trait]
impl<F> HttpClient for MockClient<F>
where
    F: std::future::Future<Output = Result<Response, DomeneshopError>> + Send,
{
    async fn execute_request(&self, request: Request) -> Result<Response, DomeneshopError> {
        (self.req_received)(request).await
    }
}
