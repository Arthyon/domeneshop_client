use domeneshop_client::{
    self,
    client::DomeneshopError,
    endpoints::{domains::Domain, invoices::InvoiceStatus},
    http_client::mock::MockClient,
};
use http_types::{Request, Response, StatusCode};

use crate::common::{assert_url_equal, create_client, set_body};
mod common;

#[tokio::test]
async fn list_invoices_with_status_adds_correct_query_parameter() {
    fn receive_request(req: &Request) -> Result<Response, DomeneshopError> {
        let mut response = Response::new(StatusCode::Ok);
        let body: Vec<Domain> = Vec::new();
        set_body(&mut response, body);
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
