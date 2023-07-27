use std::fmt::{self, Display, Formatter};

use http_types::{Method, Request, StatusCode};
use serde::{Deserialize, Serialize};

use crate::client::{set_body, DomeneshopClient, DomeneshopError};

use super::domains::DomainId;

/// Enum representing a type of DNS record
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
        match self {
            DnsType::A => write!(f, "A"),
            DnsType::AAAA => write!(f, "AAAA"),
            DnsType::CNAME => write!(f, "CNAME"),
            DnsType::MX => write!(f, "MX"),
            DnsType::SRV => write!(f, "SRV"),
            DnsType::TXT => write!(f, "TXT"),
        }
    }
}

/// Id of a DNS record
type DnsId = i32;
/// Represents a DNS record for a domain
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum DnsRecord {
    /// A-Record
    A(ARecord),
    /// AAAA-Record
    AAAA(AAAARecord),
    /// CNAME-Record
    CNAME(CNAMERecord),
    /// MX-Record
    MX(MXRecord),
    /// SRC-Record
    SRV(SRVRecord),
    /// TXT-Record
    TXT(TXTRecord),
}

/// Represents data about an A-record
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ARecord {
    /// ID of DNS record
    pub id: DnsId,
    /// The host/subdomain the DNS record applies to
    pub host: String,
    /// TTL of DNS record in seconds. Must be a multiple of 60.
    pub ttl: i16,
    /// IPv4 address
    pub data: String,
}

/// Represents data about an AAAA-record
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AAAARecord {
    /// ID of DNS record
    pub id: DnsId,
    /// The host/subdomain the DNS record applies to
    pub host: String,
    /// TTL of DNS record in seconds. Must be a multiple of 60.
    pub ttl: i16,
    /// IPv6 address
    pub data: String,
}

/// Represents data about a CNAME-record
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CNAMERecord {
    /// ID of DNS record
    pub id: DnsId,
    /// The host/subdomain the DNS record applies to
    pub host: String,
    /// TTL of DNS record in seconds. Must be a multiple of 60.
    pub ttl: i16,
    /// The target hostname
    pub data: String,
}

/// Represents data about an MX-record
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MXRecord {
    /// ID of DNS record
    pub id: DnsId,
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
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SRVRecord {
    /// ID of DNS record
    pub id: DnsId,
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
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TXTRecord {
    /// ID of DNS record
    pub id: DnsId,
    /// The host/subdomain the DNS record applies to
    pub host: String,
    /// TTL of DNS record in seconds. Must be a multiple of 60.
    pub ttl: i16,
    /// Freeform text field
    pub data: String,
}

impl DnsRecord {
    fn get_id(&self) -> DnsId {
        match self {
            DnsRecord::A(rec) => rec.id,
            DnsRecord::AAAA(rec) => rec.id,
            DnsRecord::CNAME(rec) => rec.id,
            DnsRecord::MX(rec) => rec.id,
            DnsRecord::SRV(rec) => rec.id,
            DnsRecord::TXT(rec) => rec.id,
        }
    }
}

impl DomeneshopClient {
    /// Lists all DNS records for a domain
    pub async fn list_dns_records(&self, id: DomainId) -> Result<Vec<DnsRecord>, DomeneshopError> {
        let url = self.create_url(format!("/domains/{}/dns", id))?;

        self.get_response(url).await
    }

    /// Lists filtered subset of DNS records for a domain
    pub async fn list_dns_records_with_filter(
        &self,
        id: DomainId,
        host_filter: Option<String>,
        type_filter: Option<DnsType>,
    ) -> Result<Vec<DnsRecord>, DomeneshopError> {
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

    /// Updates an existing DNS record for the given domain
    pub async fn update_dns_record(
        &self,
        domain_id: DomainId,
        record: DnsRecord,
    ) -> Result<(), DomeneshopError> {
        let dns_id: DnsId = record.get_id();
        let url = self.create_url(format!("/domains/{}/dns/{}", domain_id, dns_id))?;

        let mut request = Request::new(Method::Post, url);
        set_body(&mut request, record);

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
