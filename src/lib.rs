pub mod client;
pub mod http;

mod error_mapping;

pub mod http_client {
    #[cfg(feature = "reqwest")]
    pub mod reqwest;

    #[cfg(feature = "mock")]
    pub mod mock;
}
pub mod endpoints {
    pub mod domains;
    mod dyndns;
    pub mod invoices;
}
