use chrono::NaiveDate;
use http_types::{Method, Request};
use serde::Deserialize;

use crate::client::{DomeneshopClient, DomeneshopError};

pub type InvoiceId = i32;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InvoiceStatus {
    Unpaid,
    Paid,
    Settled,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceType {
    Invoice,
    CreditNode,
}

#[derive(Debug, Clone, Deserialize)]
pub enum InvoiceCurrency {
    NOK,
    SEK,
    DKK,
    GBP,
    USD,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Invoice {
    pub id: InvoiceId,
    pub r#type: InvoiceType,
    pub amount: i32,
    pub due_date: Option<NaiveDate>,
    pub issued_date: NaiveDate,
    pub paid_date: Option<NaiveDate>,
    pub status: InvoiceStatus,
    pub url: String,
}

impl DomeneshopClient {
    pub async fn list_invoices(&self) -> Result<Vec<Invoice>, DomeneshopError> {
        let url = self.create_url("/invoices")?;

        let request = Request::new(Method::Get, url);
        let response = self.send(request).await?;

        self.deserialize_response(response).await
    }

    pub async fn list_invoices_with_status(
        &self,
        status: InvoiceStatus,
    ) -> Result<Vec<Invoice>, DomeneshopError> {
        let mut url = self.create_url("/invoices")?;
        url.set_query(Some(
            format!("status={}", Self::map_invoice_status(status)).as_str(),
        ));

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
