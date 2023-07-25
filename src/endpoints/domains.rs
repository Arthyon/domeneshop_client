use std::fmt::Display;

use chrono::NaiveDate;
use http_types::{Method, Request};
use serde::Deserialize;

use crate::client::{DomeneshopClient, DomeneshopError};

/// Id of a domain
pub type DomainId = i32;

/// The status of the domain
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DomainStatus {
    /// The domain is active
    Active,
    /// The domain has expired
    Expired,
    /// The domain is deactivated
    Deactivated,
    /// The domain is marked for deletion, but can be restored
    PendingDeleteRestorable,
}

/// The type of web hotel connected to the domain
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WebhotelType {
    /// No webhotel registered
    None,
    /// Medium
    WebMedium,
    /// Starter
    WebSmall,
    /// Large
    WebLarge,
    /// Extra large
    WebXLarge,
}

/// Information about which domain services that are active for the domain
#[derive(Debug, Clone, Deserialize)]
pub struct DomainServices {
    /// Domeneshop is registrar for the domain
    pub registrar: bool,
    /// DNS services
    pub dns: bool,
    /// Email services
    pub email: bool,
    /// Web hotel
    pub webhotel: WebhotelType,
}

/// The available data of a domain
#[derive(Debug, Clone, Deserialize)]
pub struct Domain {
    /// Id
    pub id: DomainId,
    /// Hostname
    pub domain: String,
    /// Expiration date
    pub expiry_date: NaiveDate,
    /// Optional date of registration
    pub registered_date: Option<NaiveDate>,
    /// Is auto-renew enabled?
    pub renew: bool,
    /// Name of the registrant
    pub registrant: String,
    /// Status of the domain
    pub status: DomainStatus,
    /// List of active nameservers for the domain
    pub nameservers: Vec<String>,
    /// Information about which services are enabled for the domain
    pub services: DomainServices,
}

impl DomeneshopClient {
    /// Gets a single domain by id
    ///
    /// Domeneshop returns unauthorized when requesting a non-owned id. Cannot differentiate between credentials-error and this.
    /// This should really return `Result<Option<Domain>, DomeneShopError>`
    pub async fn get_domain(&self, id: DomainId) -> Result<Domain, DomeneshopError> {
        let url = self.create_url(format!("/domains/{}", id))?;

        let request = Request::new(Method::Get, url);
        let response = self.send(request).await?;

        self.deserialize_response(response).await
    }

    /// Lists all domains for the current user
    pub async fn list_domains(&self) -> Result<Vec<Domain>, DomeneshopError> {
        let url = self.create_url("/domains")?;

        let request = Request::new(Method::Get, url);
        let response = self.send(request).await?;

        self.deserialize_response(response).await
    }

    /// Lists all domain for the current user.
    /// Only returns domains containing the text in `filter`.
    pub async fn list_domains_with_filter(
        &self,
        filter: impl Into<String> + Display,
    ) -> Result<Vec<Domain>, DomeneshopError> {
        let mut url = self.create_url("/domains")?;
        url.set_query(Some(format!("domain={}", filter).as_str()));

        let request = Request::new(Method::Get, url);
        let response = self.send(request).await?;

        self.deserialize_response(response).await
    }
}
