use std::fmt::{self, Display, Formatter};

use chrono::NaiveDate;
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

impl Display for InvoiceStatus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            InvoiceStatus::Unpaid => write!(f, "unpaid"),
            InvoiceStatus::Paid => write!(f, "paid"),
            InvoiceStatus::Settled => write!(f, "settled"),
        }
    }
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

        self.get_response(url).await
    }

    /// Lists all the invoices for your account, filtered on a specific status
    pub async fn list_invoices_with_status(
        &self,
        status: InvoiceStatus,
    ) -> Result<Vec<Invoice>, DomeneshopError> {
        let url =
            self.create_url_with_parameters("/invoices", &[("status", status.to_string())])?;

        self.get_response(url).await
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
