use chrono::NaiveDate;
use serde::Deserialize;

use crate::{
    client::{DomeneshopClient, DomeneshopError},
    helpers::parse_response,
};

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
        let request = self
            .client
            .get(self.create_absolute_url(format!("/domains/{}", id)));

        let result = self.send(request).await?;
        parse_response(result).await
    }

    pub async fn list_domains(&self) -> Result<Vec<Domain>, DomeneshopError> {
        self.list_domains_internal::<String>(None).await
    }

    pub async fn list_domains_with_filter<S>(
        &self,
        filter: S,
    ) -> Result<Vec<Domain>, DomeneshopError>
    where
        S: Into<String> + serde::Serialize,
    {
        self.list_domains_internal(Some(filter)).await
    }

    async fn list_domains_internal<S>(
        &self,
        filter: Option<S>,
    ) -> Result<Vec<Domain>, DomeneshopError>
    where
        S: Into<String> + serde::Serialize,
    {
        let mut request = self.client.get(self.create_absolute_url("/domains"));

        if let Some(filter) = filter {
            request = request.query(&[("domain", filter)]);
        }

        let result = self.send(request).await?;
        parse_response(result).await
    }
}
