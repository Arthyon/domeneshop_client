use domeneshop_client::{self, client::DomeneshopError, http_client::mock::MockClient};
use http_types::{Request, Response, StatusCode};

use crate::common::{assert_url_equal, create_client};
mod common;

#[tokio::test]
async fn get_domain_formats_url_correctly() {
    async fn receive_request(req: Request) -> Result<Response, DomeneshopError> {
        let mut response = Response::new(StatusCode::Ok);
        response.set_body("{ \"domain\": \"d\", \"registrant\": \"d\", \"status\": \"active\", \"expiry_date\": \"2023-04-05\", \"id\": 3, \"renew\": false, \"nameservers\": [], \"services\": { \"registrar\": false, \"dns\": false, \"email\": false, \"webhotel\": \"none\" } }");
        assert_url_equal(req.url(), "/domains/3");
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    let response = client.get_domain(3).await.unwrap();

    assert_eq!(3, response.id)
}

#[tokio::test]
async fn list_domains_with_filter_adds_correct_query_parameter() {
    async fn receive_request(req: Request) -> Result<Response, DomeneshopError> {
        let mut response = Response::new(StatusCode::Ok);
        response.set_body("[]");
        assert_url_equal(req.url(), "/domains?domain=.no");
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    _ = client.list_domains_with_filter(".no").await.unwrap();
}
