#![deny(missing_docs)]

//! # domeneshop_client
//! Rust wrapper for the [Domeneshop API](https://api.domeneshop.no/docs).

//!
//!
//! ## Prerequisites
//! To use this crate, you need to generate API-credentials for the Domeneshop API.
//! See their [Authentication](https://api.domeneshop.no/docs/#section/Authentication)-section for more information.
//!
//! ## Creating the client
//!
//! ```rust
//! # let token = "".to_string();
//! # let secret = "".to_string();
//! use domeneshop_client::client::{ DomeneshopClient, DomeneshopClientConfiguration };
//!
//! let client = DomeneshopClient::new(token, secret, DomeneshopClientConfiguration::default());
//! ```
//!
//! The client will have functions for all operations in the API.
//!
//! By default, the `reqwest` feature is enabled.
//! This uses the [reqwest](reqwest)-crate to perform the requests.
//! It is possible to supply your own implementation of the [`HttpClient`](http::HttpClient)-trait.
//!
//! If so, you must supply the custom implementation as part of [`DomeneshopClientConfiguration`](client::DomeneshopClientConfiguration).
//! (the `default`-function is not present when including this crate without the `reqwest`-feature).
//!
//! ```rust
//! # let token = "".to_string();
//! # let secret = "".to_string();
//! # let client = reqwest::Client::new();
//! # use domeneshop_client::client::{ DomeneshopClient, DomeneshopClientConfiguration };
//!
//! let client = DomeneshopClient::new(token, secret, DomeneshopClientConfiguration {
//!     user_agent: None,
//!     base_url: None,
//!     underlying_client: Some(Box::new(client)),
//! });
//! ```
//!
//! ## Additional configuration
//! Creation of the client accepts a struct [`DomeneshopClientConfiguration`](client::DomeneshopClientConfiguration).
//! This can be used to override some settings:
//! - `user_agent`: Replace the default user agent (defaults to this crate's github-url and crate version).
//! - `base_url`: Used to override the base url of the API. Can be used for testing or proxying requests.
//! - `underlying_client`: An implementation of the trait [`HttpClient`](http::HttpClient) to use instead of the default `reqwest`-implementation.
//!
//! # Testing
//! If you want to test code using this client, enable the `mock` feature.
//! This will make a new [`MockClient`](http_client::mock::MockClient) available that can be passed to the client using the `underlying_client` ocnfiguration option.
//!
//! [reqwest]: https://crates.io/crates/reqwest

/// Module containing the DomeneshopClient
pub mod client;
/// Module for http-specifics (HttpClient-trait)
pub mod http;

mod error_mapping;

/// Module containing clients implementing the HttpClient-trait
pub mod http_client {
    #[cfg(feature = "reqwest")]
    pub(crate) mod reqwest;

    /// Mock-client for testing. Only available when the `mock`-feature is enabled.
    #[cfg(any(feature = "mock"))]
    pub mod mock;
}

/// Modules containing structs for the endpoint groups in the API
pub mod endpoints {
    /// Contains dns-related structs
    pub mod dns;
    /// Contains domain-related structs
    pub mod domains;
    mod dyndns;
    /// Contains invoice-related structs
    pub mod invoices;
}
