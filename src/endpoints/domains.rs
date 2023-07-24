use std::fmt::Display;

use chrono::NaiveDate;
use http_types::{Method, Request};
use serde::Deserialize;

use crate::client::{DomeneshopClient, DomeneshopError};

pub type DomainId = i32;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DomainStatus {
    Active,
    Expired,
    Deactivated,
    PendingDeleteRestorable,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WebhotelType {
    None,
    WebMedium,
    WebSmall,
    WebLarge,
    WebXLarge,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DomainServices {
    pub registrar: bool,
    pub dns: bool,
    pub email: bool,
    pub webhotel: WebhotelType,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Domain {
    pub id: DomainId,
    pub domain: String,
    pub expiry_date: NaiveDate,
    pub registered_date: Option<NaiveDate>,
    pub renew: bool,
    pub registrant: String,
    pub status: DomainStatus,
    pub nameservers: Vec<String>,
    pub services: DomainServices,
}

impl DomeneshopClient {
    // Domeneshop returns unauthorized when requesting a non-owned id. Cannot differentiate between credentials-error and this.
    // Should really be Result<Option<Domain>, DomeneShopError>-return value
    pub async fn get_domain(&self, id: DomainId) -> Result<Domain, DomeneshopError> {
        let url = self.create_url(format!("/domains/{}", id))?;

        let request = Request::new(Method::Get, url);
        let response = self.send(request).await?;

        self.deserialize_response(response).await
    }

    pub async fn list_domains(&self) -> Result<Vec<Domain>, DomeneshopError> {
        let url = self.create_url("/domains")?;

        let request = Request::new(Method::Get, url);
        let response = self.send(request).await?;

        self.deserialize_response(response).await
    }

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
