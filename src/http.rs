use async_trait::async_trait;
use http_types::{Request, Response};

use crate::client::DomeneshopError;

#[async_trait]
pub trait HttpClient {
    async fn execute_request(&self, request: Request) -> Result<Response, DomeneshopError>;
}
