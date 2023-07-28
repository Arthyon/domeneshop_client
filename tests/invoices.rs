use domeneshop_client::{
    self, endpoints::invoices::InvoiceStatus, errors::DomeneshopError,
    http_client::mock::MockClient,
};
use http_types::{Request, Response, StatusCode};

use crate::common::{assert_url_equal, create_client};
mod common;

#[tokio::test]
async fn list_invoices_with_status_adds_correct_query_parameter() {
    async fn receive_request(req: Request) -> Result<Response, DomeneshopError> {
        let mut response = Response::new(StatusCode::Ok);
        response.set_body("[]");
        assert_url_equal(req.url(), "/invoices?status=paid");
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    _ = client
        .list_invoices_with_status(InvoiceStatus::Paid)
        .await
        .unwrap();
}

#[tokio::test]
async fn find_invoice_by_number_serializes_correctly() {
    async fn receive_request(req: Request) -> Result<Response, DomeneshopError> {
        let mut response = Response::new(StatusCode::Ok);
        response.set_body("{\r\n\r\n    \"id\": 1,    \"type\": \"invoice\",    \"amount\": 120,   \"currency\": \"NOK\",   \"due_date\": \"2019-08-24\",   \"issued_date\": \"2019-08-24\",    \"paid_date\": \"2019-08-24\",   \"status\": \"paid\",    \"url\": \"https://www.domeneshop.no/invoice?nr=1&code=\"}");
        assert_url_equal(req.url(), "/invoices/2");
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    let response = client.find_invoice_by_id(2).await.unwrap();

    assert!(response.is_some());
}

#[tokio::test]
async fn find_invoice_by_number_404_returns_none() {
    async fn receive_request(_: Request) -> Result<Response, DomeneshopError> {
        let response = Response::new(StatusCode::NotFound);
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    let response = client.find_invoice_by_id(2).await.unwrap();

    assert!(response.is_none());
}
