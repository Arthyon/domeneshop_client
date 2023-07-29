use std::fmt::{self, Debug, Display, Formatter};

use http_types::{Method, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    client::{set_body, DomeneshopClient},
    errors::{to_domain_error_with_context, DomeneshopError},
};

use super::domains::DomainId;

/// Enum representing a type of DNS record
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub enum DnsType {
    /// A-Record
    A,
    /// AAAA-Record
    AAAA,
    /// CNAME-Record
    CNAME,
    /// MX-Record
    MX,
    /// SRC-Record
    SRV,
    /// TXT-Record
    TXT,
}

impl Display for DnsType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(
            match self {
                DnsType::A => "A",
                DnsType::AAAA => "AAAA",
                DnsType::CNAME => "CNAME",
                DnsType::MX => "MX",
                DnsType::SRV => "SRV",
                DnsType::TXT => "TXT",
            },
            f,
        )
    }
}

/// Id of a DNS record
type DnsId = i32;

/// Represents an existing record
#[derive(Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub struct ExistingDnsRecord {
    /// ID of DNS record
    pub id: DnsId,
    /// Data about the record
    #[serde(flatten)]
    pub data: DnsRecordData,
}

/// Represents DNS record data
#[derive(Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum DnsRecordData {
    /// A-Record
    A(ARecordData),
    /// AAAA-Record
    AAAA(AAAARecordData),
    /// CNAME-Record
    CNAME(CNAMERecordData),
    /// MX-Record
    MX(MXRecordData),
    /// SRC-Record
    SRV(SRVRecordData),
    /// TXT-Record
    TXT(TXTRecordData),
}

/// Represents data about an A-record
#[derive(Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub struct ARecordData {
    /// The host/subdomain the DNS record applies to
    pub host: String,
    /// TTL of DNS record in seconds. Must be a multiple of 60.
    pub ttl: i16,
    /// IPv4 address
    pub data: String,
}

/// Represents data about an AAAA-record
#[derive(Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub struct AAAARecordData {
    /// The host/subdomain the DNS record applies to
    pub host: String,
    /// TTL of DNS record in seconds. Must be a multiple of 60.
    pub ttl: i16,
    /// IPv6 address
    pub data: String,
}

/// Represents data about a CNAME-record
#[derive(Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub struct CNAMERecordData {
    /// The host/subdomain the DNS record applies to
    pub host: String,
    /// TTL of DNS record in seconds. Must be a multiple of 60.
    pub ttl: i16,
    /// The target hostname
    pub data: String,
}

/// Represents data about an MX-record
#[derive(Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub struct MXRecordData {
    /// The host/subdomain the DNS record applies to
    pub host: String,
    /// TTL of DNS record in seconds. Must be a multiple of 60.
    pub ttl: i16,
    /// The target MX host.    
    pub data: String,
    /// MX record priority, also known as preference. Lower values are usually preferred first, but this is not guaranteed
    pub priority: i16,
}

/// Represents data about a SRV-record
#[derive(Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub struct SRVRecordData {
    /// The host/subdomain the DNS record applies to
    pub host: String,
    /// TTL of DNS record in seconds. Must be a multiple of 60.
    pub ttl: i16,
    /// The target hostname
    pub data: String,
    /// SRV record priority, also known as preference. Lower values are usually preferred first
    pub priority: i16,
    /// SRV record weight. Relevant if multiple records have same preference
    pub weight: i16,
    /// SRV record port. The port where the service is found.
    pub port: i16,
}

/// Represents data about a TXT-record
#[derive(Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub struct TXTRecordData {
    /// The host/subdomain the DNS record applies to
    pub host: String,
    /// TTL of DNS record in seconds. Must be a multiple of 60.
    pub ttl: i16,
    /// Freeform text field
    pub data: String,
}

/// Response when adding a new DNS record to a domain
#[derive(Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub struct AddDnsRecordResponse {
    /// Id of the created DNS record
    pub id: DnsId,
    /// Url to the DNS record resource that was created
    pub url: Option<Url>,
}

#[derive(Debug, Deserialize)]
struct DomeneshopAddDnsRecordResponse {
    pub id: i32,
}

/// Operations concerning DNS Records
impl DomeneshopClient {
    /// Get DNS Record by id
    pub async fn get_dns_record(
        &self,
        domain_id: DomainId,
        dns_id: DnsId,
    ) -> Result<ExistingDnsRecord, DomeneshopError> {
        let url = self.create_url(format!("/domains/{}/dns/{}", domain_id, dns_id))?;

        self.get_response(url).await
    }

    /// Lists all DNS records for a domain
    pub async fn list_dns_records(
        &self,
        id: DomainId,
    ) -> Result<Vec<ExistingDnsRecord>, DomeneshopError> {
        let url = self.create_url(format!("/domains/{}/dns", id))?;

        self.get_response(url).await
    }

    /// Lists filtered subset of DNS records for a domain
    pub async fn list_dns_records_with_filter(
        &self,
        id: DomainId,
        host_filter: Option<String>,
        type_filter: Option<DnsType>,
    ) -> Result<Vec<ExistingDnsRecord>, DomeneshopError> {
        let mut query_parameters = Vec::new();
        if let Some(host) = host_filter {
            query_parameters.push(("host", host));
        }
        if let Some(dns_type) = type_filter {
            query_parameters.push(("type", dns_type.to_string()));
        }
        let url =
            self.create_url_with_parameters(format!("/domains/{}/dns", id), query_parameters)?;

        self.get_response(url).await
    }

    /// adds a new  DNS record for the given domain
    pub async fn add_dns_record(
        &self,
        domain_id: DomainId,
        record: DnsRecordData,
    ) -> Result<AddDnsRecordResponse, DomeneshopError> {
        let url = self.create_url(format!("/domains/{}/dns", domain_id))?;

        let mut request = Request::new(Method::Post, url);
        set_body(&mut request, record);

        let mut response = self.send(request).await?;
        match response.status() {
            StatusCode::Created => {
                let location = parse_location_header(&response);
                let body = response
                    .body_json::<DomeneshopAddDnsRecordResponse>()
                    .await
                    .map_err(|err| {
                        to_domain_error_with_context(
                            "DNS record created successfully, but could not deserialize response",
                            err,
                        )
                    })?;
                Ok(AddDnsRecordResponse {
                    id: body.id,
                    url: location,
                })
            }
            _ => Err(DomeneshopError::new(format!(
                "Encountered unexpected response status {}",
                response.status()
            ))),
        }
    }

    /// Updates an existing DNS record for the given domain
    pub async fn update_dns_record(
        &self,
        domain_id: DomainId,
        record: ExistingDnsRecord,
    ) -> Result<(), DomeneshopError> {
        let url = self.create_url(format!("/domains/{}/dns/{}", domain_id, record.id))?;

        let mut request = Request::new(Method::Put, url);
        set_body(&mut request, record.data);

        let response = self.send(request).await?;
        match response.status() {
            StatusCode::NoContent => Ok(()),
            _ => Err(DomeneshopError::new(format!(
                "Encountered unexpected response status {}",
                response.status()
            ))),
        }
    }

    /// Deletes a dns record using the given id
    pub async fn delete_dns_record(
        &self,
        domain_id: DomainId,
        dns_id: DnsId,
    ) -> Result<(), DomeneshopError> {
        let url = self.create_url(format!("/domains/{}/dns/{}", domain_id, dns_id))?;

        let request = Request::new(Method::Delete, url);
        let response = self.send(request).await;
        response.map(|_| Ok(()))?
    }
}

fn parse_location_header(response: &Response) -> Option<Url> {
    match response.header("Location") {
        None => None,
        Some(header) => match Url::parse(header.last().as_str()) {
            Err(_) => None,
            Ok(url) => Some(url),
        },
    }
}
