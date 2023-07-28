use std::net::IpAddr;

use http_types::{Method, Request, StatusCode};

use crate::{client::DomeneshopClient, errors::DomeneshopError};

impl DomeneshopClient {
    /// Update DNS using the "IP update protocol".
    /// A DNS record for the given hostname will be created if it does not exist, or updated if it does.
    /// The record type (A or AAAA will automatically be detected).
    ///
    /// If `ip` is not provided, the IP of the client making the API request will be used.
    pub async fn update_dyndns<S>(
        &self,
        hostname: S,
        ip: Option<IpAddr>,
    ) -> Result<(), DomeneshopError>
    where
        S: Into<String>,
    {
        let mut query_parameters = vec![("hostname", hostname.into())];
        if let Some(ip) = ip {
            query_parameters.push(("myip", ip.to_string()));
        }
        let url = self.create_url_with_parameters("/dyndns/update", query_parameters)?;

        let request = Request::new(Method::Get, url);
        let response = self.send(request).await?;

        match response.status() {
            StatusCode::NoContent => Ok(()),
            _ => Err(DomeneshopError::new(format!(
                "Encountered unexpected response status {}",
                response.status()
            ))),
        }
    }
}
