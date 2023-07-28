use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use domeneshop_client::{self, errors::DomeneshopError, http_client::mock::MockClient};
use http_types::{Request, Response, StatusCode};

use crate::common::{assert_url_equal, create_client};
mod common;

#[tokio::test]
async fn update_dyndns_creates_correct_url() {
    async fn receive_request(req: Request) -> Result<Response, DomeneshopError> {
        let response = Response::new(StatusCode::NoContent);
        assert_url_equal(req.url(), "/dyndns/update?hostname=example.com");
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    _ = client.update_dyndns("example.com", None).await.unwrap();
}

#[tokio::test]
async fn update_dyndns_ipv4_address_is_set_correctly() {
    async fn receive_request(req: Request) -> Result<Response, DomeneshopError> {
        let response = Response::new(StatusCode::NoContent);
        assert_url_equal(
            req.url(),
            "/dyndns/update?hostname=example.com&myip=192.168.0.1",
        );
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    let ip = Ipv4Addr::new(192, 168, 0, 1);
    _ = client
        .update_dyndns("example.com", Some(IpAddr::V4(ip)))
        .await
        .unwrap();
}

#[tokio::test]
async fn update_dyndns_ipv6_address_is_set_correctly() {
    async fn receive_request(req: Request) -> Result<Response, DomeneshopError> {
        let response = Response::new(StatusCode::NoContent);
        assert_url_equal(
            req.url(),
            "/dyndns/update?hostname=example.com&myip=2001%3A4860%3A4860%3A%3A8888",
        );
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);
    let ip: Ipv6Addr = "2001:4860:4860::8888".parse().unwrap();
    _ = client
        .update_dyndns("example.com", Some(IpAddr::V6(ip)))
        .await
        .unwrap();
}
