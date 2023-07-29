use async_trait::async_trait;
use http_types::{Request, Response};

use crate::errors::DomeneshopError;

#[async_trait]
/// A trait used to abstract the transport mechanism for sending requests.
/// This is mostly added to make testing easier.
///
/// If one wants to use another crate than `reqwest`, a separate implementation of this trait can be supplied when creating the [`DomeneshopClient`](./client/struct.DomeneshopClient.html).
///
/// # Example
/// To implement this trait, you need the [`async_trait`](https://docs.rs/async-trait/latest/async_trait/)-crate.
/// ```
///use async_trait::async_trait;
///use http_types::{Request, Response};
///use domeneshop_client::{errors::DomeneshopError, http::HttpClient};
///
///struct MyClient;
///#[async_trait]
///impl HttpClient for MyClient
///{
///    async fn execute_request(&self, request: Request) -> Result<Response, DomeneshopError> {
///        todo!();
///    }
///}
///
/// ```
pub trait HttpClient {
    /// Executes a request
    /// This defaults to a provided `reqwest`-implementation, but custom implementations can be created
    async fn execute_request(&self, request: Request) -> Result<Response, DomeneshopError>;
}
