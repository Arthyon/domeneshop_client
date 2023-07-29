#![deny(missing_docs)]
// only enables the `doc_cfg` feature when
// the `docsrs` configuration attribute is defined
#![cfg_attr(docsrs, feature(doc_cfg))]
//! # domeneshop_client
//! API bindings for the [Domeneshop API](https://api.domeneshop.no/docs).
//!
//!
//! ## Prerequisites
//! To use this crate, you need to generate API-credentials for the Domeneshop API.
//! See their [Authentication](https://api.domeneshop.no/docs/#section/Authentication)-section for more information.
//!
//! ## Creating the client
//!
//! ```
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
//! # Features
//! - `reqwest` (default feature): Uses [`reqwest`](reqwest) to perform the requests. Consumers must supply their own implementation of [`HttpClient`](http::HttpClient) if this is disabled.
//! - `mock`: Adds [`MockClient`](http_client::mock::MockClient) that can be used for testing.
//!
//! [reqwest]: https://crates.io/crates/reqwest

/// Module containing the DomeneshopClient
pub mod client;
/// Module for error structs
pub mod errors;
/// Module for http-specifics (HttpClient-trait)
pub mod http;

/// Module containing clients implementing the HttpClient-trait
pub mod http_client {
    #[cfg(feature = "reqwest")]
    #[cfg_attr(docsrs, doc(cfg(feature = "reqwest")))]
    pub(crate) mod reqwest;

    /// Mock-client for testing. Only available when the `mock`-feature is enabled.
    #[cfg_attr(docsrs, doc(cfg(feature = "mock")))]
    #[cfg(any(feature = "mock"))]
    pub mod mock;
}

/// Modules containing structs for the endpoint groups in the API
pub mod endpoints {
    /// These endpoints allow you to list, create and delete DNS records for any domain that has active DNS service.
    pub mod dns;
    /// Contains domain-related structs
    pub mod domains;
    mod dyndns;
    /// These endpoints allow you to list, create and delete HTTP forwards ("WWW forwarding") for any domain that has active DNS service.
    /// While frame forwards (an `<iframe>` embed) are supported through setting the `frame` field, the use is strongly discouraged. Use DNS records instead.
    pub mod forwards;
    /// Contains invoice-related structs
    pub mod invoices;
}
