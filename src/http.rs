use async_trait::async_trait;
use http_types::{Request, Response};

use crate::client::DomeneshopError;

#[async_trait]
/// A trait used to abstract the transport mechanism for sending requests.
/// This is mostly added to make testing easier.
pub trait HttpClient {
    /// Executes a request
    /// This defaults to a provided `reqwest`-implementation, but custom implementations can be created
    async fn execute_request(&self, request: Request) -> Result<Response, DomeneshopError>;
}
