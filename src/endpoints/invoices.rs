use chrono::NaiveDate;
use http_types::{Method, Request};
use serde::Deserialize;

use crate::client::{DomeneshopClient, DomeneshopError};

/// Id of an invoice
pub type InvoiceId = i32;

/// The status an invoice can have.
/// Settled is only applicable to the InvoiceType [`CreditNote`](InvoiceType::CreditNode)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InvoiceStatus {
    /// An unpaid invoice
    Unpaid,
    /// A paid invoice
    Paid,
    /// A settled creditnote
    Settled,
}

/// Type of invoice
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceType {
    /// A normal invoice
    Invoice,
    /// A credit note
    /// *NOTE*: It is unclear if the name is a typo in the docs. This may crash.
    CreditNode,
}

/// The currency of an invoice
#[derive(Debug, Clone, Deserialize)]
pub enum InvoiceCurrency {
    /// Norwegian krone
    NOK,
    /// Swedish krone
    SEK,
    /// Danish krone
    DKK,
    /// British pound
    GBP,
    /// US dollar
    USD,
}

/// The available data of an invoice
#[derive(Debug, Clone, Deserialize)]
pub struct Invoice {
    /// Id of the invoice
    pub id: InvoiceId,
    /// Type of the invoice
    pub r#type: InvoiceType,
    /// Amount
    pub amount: i32,
    /// Due date
    pub due_date: Option<NaiveDate>,
    /// Issued date
    pub issued_date: NaiveDate,
    /// Paid date
    pub paid_date: Option<NaiveDate>,
    /// Status
    pub status: InvoiceStatus,
    /// Link to the url on Domeneshop's site
    pub url: String,
}

impl DomeneshopClient {
    /// Lists all the invoices for your account
    pub async fn list_invoices(&self) -> Result<Vec<Invoice>, DomeneshopError> {
        let url = self.create_url("/invoices")?;

        let request = Request::new(Method::Get, url);
        let response = self.send(request).await?;

        self.deserialize_response(response).await
    }

    /// Lists all the invoices for your account, filtered on a specific status
    pub async fn list_invoices_with_status(
        &self,
        status: InvoiceStatus,
    ) -> Result<Vec<Invoice>, DomeneshopError> {
        let mapped_status = Self::map_invoice_status(status);
        let url = self.create_url_with_parameters("/invoices", &[("status", mapped_status)])?;

        let request = Request::new(Method::Get, url);
        let response = self.send(request).await?;
        self.deserialize_response(response).await
    }

    fn map_invoice_status(status: InvoiceStatus) -> String {
        let s = match status {
            InvoiceStatus::Paid => "paid",
            InvoiceStatus::Settled => "settled",
            InvoiceStatus::Unpaid => "unpaid",
        };
        s.to_string()
    }
}
