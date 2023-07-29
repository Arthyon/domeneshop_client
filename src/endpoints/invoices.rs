use std::fmt::{self, Display, Formatter};

use chrono::NaiveDate;
use http_types::{Method, Request, StatusCode};
use serde::{Deserialize, Serialize};

use crate::{
    client::{handle_response_error, DomeneshopClient},
    errors::DomeneshopError,
};

/// Id of an invoice
pub type InvoiceId = i32;

/// The status an invoice can have.
/// Settled is only applicable to the InvoiceType [`CreditNote`](InvoiceType::CreditNode)
#[derive(Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum InvoiceStatus {
    /// An unpaid invoice
    Unpaid,
    /// A paid invoice
    Paid,
    /// A settled creditnote
    Settled,
}

impl Display for InvoiceStatus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            InvoiceStatus::Unpaid => "Unpaid",
            InvoiceStatus::Paid => "Paid",
            InvoiceStatus::Settled => "Settled",
        }
        .fmt(f)
    }
}

/// Type of invoice
#[derive(Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceType {
    /// A normal invoice
    Invoice,
    /// A credit note
    /// *NOTE*: It is unclear if the name is a typo in the docs. This may crash.
    CreditNode,
}

impl Display for InvoiceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InvoiceType::Invoice => "Invoice",
            InvoiceType::CreditNode => "Credit Note",
        }
        .fmt(f)
    }
}

/// The currency of an invoice
#[derive(Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
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

impl Display for InvoiceCurrency {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InvoiceCurrency::DKK => "DKK",
            InvoiceCurrency::GBP => "GBP",
            InvoiceCurrency::NOK => "NOK",
            InvoiceCurrency::SEK => "SEK",
            InvoiceCurrency::USD => "USD",
        }
        .fmt(f)
    }
}

/// The available data of an invoice
#[derive(Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub struct Invoice {
    /// Id of the invoice
    pub id: InvoiceId,
    /// Type of the invoice
    pub r#type: InvoiceType,
    /// Amount
    pub amount: i32,
    /// Currency
    pub currency: InvoiceCurrency,
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

/// Operations concerning invoices
impl DomeneshopClient {
    /// Get invoice by invoice number
    pub async fn get_invoice(&self, id: InvoiceId) -> Result<Option<Invoice>, DomeneshopError> {
        let url = self.create_url(format!("/invoices/{}", id))?;

        let request = Request::new(Method::Get, url);

        let response = self.send_no_validation(request).await?;
        match response.status() {
            StatusCode::Ok => self
                .deserialize_response::<Invoice>(response)
                .await
                .map(Some),
            StatusCode::NotFound => Ok(None),
            _ => Err(handle_response_error(response).await),
        }
    }

    /// List invoices for your account. Only invoices from the past 3 years are returned.
    pub async fn list_invoices(&self) -> Result<Vec<Invoice>, DomeneshopError> {
        let url = self.create_url("/invoices")?;

        self.get_response(url).await
    }

    /// Lists all the invoices for your account, filtered on a specific status
    pub async fn list_invoices_with_status(
        &self,
        status: InvoiceStatus,
    ) -> Result<Vec<Invoice>, DomeneshopError> {
        let url = self.create_url_with_parameters(
            "/invoices",
            &[("status", to_status_query_param(&status))],
        )?;

        self.get_response(url).await
    }
}

fn to_status_query_param(status: &InvoiceStatus) -> &str {
    match status {
        InvoiceStatus::Unpaid => "unpaid",
        InvoiceStatus::Paid => "paid",
        InvoiceStatus::Settled => "settled",
    }
}

#[cfg(test)]
mod tests {
    use crate::endpoints::invoices::InvoiceStatus;

    #[test]
    fn invoice_status_displayed_correctly() {
        assert_eq!("paid", InvoiceStatus::Paid.to_string());
        assert_eq!("settled", InvoiceStatus::Settled.to_string());
        assert_eq!("unpaid", InvoiceStatus::Unpaid.to_string());
    }
}
