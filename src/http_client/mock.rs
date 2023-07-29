use async_trait::async_trait;
use http_types::{Request, Response};

use crate::{errors::DomeneshopError, http::HttpClient};
/// A mock client that can be used for testing code using this crate.
///
/// # Example
/// ```
/// use domeneshop_client::client::{ DomeneshopClientConfiguration, DomeneshopClient };
/// use domeneshop_client::http_client::mock::MockClient;
/// use domeneshop_client::errors::DomeneshopError;
/// use http_types::{ StatusCode, Response, Request };
///
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// # tokio_test::block_on(async {
/// async fn receive_request(req: Request) -> Result<Response, DomeneshopError> {
///     let mut response = Response::new(StatusCode::Ok);
///     response.set_body("[]");
///     Ok(response)
/// }
///
/// let mock = MockClient {
///     req_received: receive_request,
/// };
///
/// let client = DomeneshopClient::new(
///     String::from("token"),
///     String::from("secret"),
///     DomeneshopClientConfiguration {
///         user_agent: None,
///         base_url: Some("https://localhost".to_string()),
///         underlying_client: Some(Box::new(mock)),
///     }
/// )?;
/// let domains = client.list_domains_with_filter(".no").await?;
///
/// assert_eq!(domains.len(), 0);
/// #
/// #     Ok(())
/// # })
/// # }
/// ```
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
