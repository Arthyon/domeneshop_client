use std::net::IpAddr;

use crate::{
    client::{DomeneshopClient, DomeneshopError},
    helpers::map_unhandled_response,
};

impl DomeneshopClient {
    pub async fn update_dyndns<S>(
        &self,
        hostname: S,
        ip: Option<IpAddr>,
    ) -> Result<(), DomeneshopError>
    where
        S: Into<String> + serde::Serialize,
    {
        let mut query_parameters: Vec<(&str, String)> = vec![("hostname", hostname.into())];
        if let Some(ip) = ip {
            query_parameters.push(("myip", ip.to_string()));
        }

        let request = self
            .client
            .get(self.create_absolute_url("/dyndns/update"))
            .query(&query_parameters);

        let result = self.send(request).await?;

        match result.status() {
            reqwest::StatusCode::NO_CONTENT => Ok(()),
            _ => Err(map_unhandled_response(result).await),
        }
    }
}
