use http_types::{Method, Request, StatusCode};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::client::{handle_response_error, set_body, DomeneshopClient, DomeneshopError};

use super::domains::DomainId;

/// A HTTP forward
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HttpForward {
    /// The subdomain this forward applies to, without the domain part.
    /// For instance, `www` in the context of the `example.com` domain signifies a forward for `www.example.com`.
    pub host: String,
    /// Whether to enable frame forwarding using an iframe embed. NOT recommended for a variety of reasons.
    pub frame: bool,
    /// The URL to forward to. Must include scheme, e.g. `https://` or `ftp://`s.
    pub url: Url,
}

impl DomeneshopClient {
    /// List all forwards for the specified domain.
    pub async fn list_forwards(&self, id: DomainId) -> Result<Vec<HttpForward>, DomeneshopError> {
        let url = self.create_url(format!("/domains/{}/forwards", id))?;

        self.get_response(url).await
    }

    /// Deletes forward by host
    /// Example `www`.
    /// Subdomain for the forward, `@` for the root domain
    pub async fn delete_forward(
        &self,
        domain_id: DomainId,
        host: String,
    ) -> Result<(), DomeneshopError> {
        let url = self.create_url(format!("/domains/{}/forwards/{}", domain_id, host))?;

        let request = Request::new(Method::Delete, url);
        let response = self.send(request).await;
        response.map(|_| Ok(()))?
    }

    /// Attempts to find an existing forward by host.
    /// Example: `www`
    /// Subdomain of the forward, `@` for the root domain
    pub async fn find_forward_by_host(
        &self,
        id: DomainId,
        host: String,
    ) -> Result<Option<HttpForward>, DomeneshopError> {
        let url = self.create_url(format!("/domains/{}/forwards/{}", id, host))?;

        let request = Request::new(Method::Get, url);

        let response = self.send_no_validation(request).await?;
        match response.status() {
            StatusCode::Ok => self
                .deserialize_response::<HttpForward>(response)
                .await
                .map(Some),
            StatusCode::NotFound => Ok(None),
            _ => Err(handle_response_error(response).await),
        }
    }

    /// Create a forwarding for the specified domain, to a given URL.
    /// The forward must not collide with any existing forwarding or DNS record of types `A`, `AAAA`, `ANAME` or `CNAME`.
    pub async fn add_forward(
        &self,
        id: DomainId,
        forward: HttpForward,
    ) -> Result<(), DomeneshopError> {
        let url = self.create_url(format!("/domains/{}/forwards", id))?;

        let mut request = Request::new(Method::Post, url);
        set_body(&mut request, forward);

        let response = self.send(request).await?;
        match response.status() {
            StatusCode::Created => Ok(()),
            _ => Err(DomeneshopError {
                help: format!(
                    "Encountered unexpected response status {}",
                    response.status()
                ),
                code: "UnexpectedStatus".to_string(),
            }),
        }
    }

    /// Update a forwarding for the specified domain, to a given URL.
    /// The `host` field must not be changed. In that case, delete the existing forwarding and recreate it for the new host/subdomain.
    pub async fn update_forward(
        &self,
        id: DomainId,
        forward: HttpForward,
    ) -> Result<(), DomeneshopError> {
        let url = self.create_url(format!("/domains/{}/forwards/{}", id, forward.host))?;

        let mut request = Request::new(Method::Put, url);
        set_body(&mut request, forward);

        let response = self.send(request).await?;
        match response.status() {
            StatusCode::Ok => Ok(()),
            _ => Err(DomeneshopError {
                help: format!(
                    "Encountered unexpected response status {}",
                    response.status()
                ),
                code: "UnexpectedStatus".to_string(),
            }),
        }
    }
}
