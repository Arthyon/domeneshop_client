use chrono::Utc;
use domeneshop_client::{
    self,
    client::DomeneshopError,
    endpoints::domains::{Domain, DomainServices, DomainStatus, WebhotelType},
    http_client::mock::MockClient,
};
use http_types::{Request, Response, StatusCode};

use crate::common::{assert_url_equal, create_client, set_body};
mod common;

#[tokio::test]
async fn get_domain_formats_url_correctly() {
    fn receive_request(req: &Request) -> Result<Response, DomeneshopError> {
        let mut response = Response::new(StatusCode::Ok);
        set_body(
            &mut response,
            Domain {
                domain: String::from("d"),
                registrant: String::from("d"),
                expiry_date: Utc::now().naive_utc().into(),
                id: 3,
                registered_date: None,
                renew: false,
                status: DomainStatus::Active,
                nameservers: vec!["da".to_string()],
                services: DomainServices {
                    registrar: true,
                    dns: true,
                    email: false,
                    webhotel: WebhotelType::None,
                },
            },
        );
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
    fn receive_request(req: &Request) -> Result<Response, DomeneshopError> {
        let mut response = Response::new(StatusCode::Ok);
        let body: Vec<Domain> = Vec::new();
        set_body(&mut response, body);
        assert_url_equal(req.url(), "/domains?domain=.no");
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    _ = client.list_domains_with_filter(".no").await.unwrap();
}
