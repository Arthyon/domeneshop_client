use chrono::NaiveDate;
use serde::Deserialize;

use crate::{
    client::{DomeneshopClient, DomeneshopError},
    helpers::parse_response,
};

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
        let request = self.client.get(self.create_absolute_url("/invoices"));

        let result = self.send(request).await?;
        parse_response(result).await
    }

    pub async fn list_invoices_with_status(
        &self,
        status: InvoiceStatus,
    ) -> Result<Vec<Invoice>, DomeneshopError> {
        let request = self
            .client
            .get(self.create_absolute_url("/invoices"))
            .query(&[("status", Self::map_invoice_status(status))]);

        let result = self.send(request).await?;
        parse_response(result).await
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
