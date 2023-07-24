use std::net::IpAddr;

use http_types::{Method, Request, StatusCode};

use crate::client::{DomeneshopClient, DomeneshopError};

impl DomeneshopClient {
    pub async fn update_dyndns<S>(
        &self,
        hostname: S,
        ip: Option<IpAddr>,
    ) -> Result<(), DomeneshopError>
    where
        S: Into<String>,
    {
        let mut url = self.create_url("/dyndns/update")?;
        let mut query_parameters = vec![("hostname", hostname.into())];
        if let Some(ip) = ip {
            query_parameters.push(("myip", ip.to_string()));
        }

        let query = query_parameters
            .into_iter()
            .map(|(name, value)| format!("{}:{}", name, value))
            .collect::<Vec<_>>()
            .join("&");

        url.set_query(Some(query.as_str()));

        let request = Request::new(Method::Get, url);
        let response = self.send(request).await?;

        match response.status() {
            StatusCode::NoContent => Ok(()),
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
