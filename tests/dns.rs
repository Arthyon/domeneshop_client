use domeneshop_client::{
    self,
    client::DomeneshopError,
    endpoints::dns::{DnsRecord, DnsType},
    http_client::mock::MockClient,
};
use http_types::{Request, Response, StatusCode};

use crate::common::{assert_url_equal, create_client};
mod common;

#[tokio::test]
async fn list_dns_formats_url_correctly() {
    fn receive_request(req: &Request) -> Result<Response, DomeneshopError> {
        let mut response = Response::new(StatusCode::Ok);
        response.set_body("[]");
        assert_url_equal(req.url(), "/domains/3/dns");
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    let response = client.list_dns_records(3).await.unwrap();

    assert_eq!(0, response.len())
}

#[tokio::test]
async fn list_dns_with_host_filter_formats_url_correctly() {
    fn receive_request(req: &Request) -> Result<Response, DomeneshopError> {
        let mut response = Response::new(StatusCode::Ok);
        response.set_body("[]");
        assert_url_equal(req.url(), "/domains/3/dns?host=test");
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    client
        .list_dns_records_with_filter(3, Some(String::from("test")), None)
        .await
        .unwrap();
}

#[tokio::test]
async fn list_dns_with_type_filter_formats_url_correctly() {
    fn receive_request(req: &Request) -> Result<Response, DomeneshopError> {
        let mut response = Response::new(StatusCode::Ok);
        response.set_body("[]");
        assert_url_equal(req.url(), "/domains/3/dns?type=SRV");
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    client
        .list_dns_records_with_filter(3, None, Some(DnsType::SRV))
        .await
        .unwrap();
}

#[tokio::test]
async fn list_dns_with_both_filters_formats_url_correctly() {
    fn receive_request(req: &Request) -> Result<Response, DomeneshopError> {
        let mut response = Response::new(StatusCode::Ok);
        response.set_body("[]");
        assert_url_equal(req.url(), "/domains/3/dns?host=test&type=SRV");
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    client
        .list_dns_records_with_filter(3, Some(String::from("test")), Some(DnsType::SRV))
        .await
        .unwrap();
}

#[tokio::test]
async fn list_dns_deserializes_a_record_correctly() {
    fn receive_request(_: &Request) -> Result<Response, DomeneshopError> {
        let mut response = Response::new(StatusCode::Ok);
        response.set_body(
            "[{\"id\": 1, \"host\":\"t\", \"ttl\": 1, \"type\": \"A\", \"data\": \"a\"}]",
        );
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    let response = client.list_dns_records(3).await.unwrap();
    let record = response.first().unwrap();
    match record {
        DnsRecord::A(_) => {}
        _ => panic!("Wrong record type"),
    }
}

#[tokio::test]
async fn list_dns_deserializes_aaaa_record_correctly() {
    fn receive_request(_: &Request) -> Result<Response, DomeneshopError> {
        let mut response = Response::new(StatusCode::Ok);
        response.set_body(
            "[{\"id\": 1, \"host\":\"t\", \"ttl\": 1, \"type\": \"AAAA\", \"data\": \"a\"}]",
        );
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    let response = client.list_dns_records(3).await.unwrap();
    let record = response.first().unwrap();
    match record {
        DnsRecord::AAAA(_) => {}
        _ => panic!("Wrong record type"),
    }
}

#[tokio::test]
async fn list_dns_deserializes_cname_record_correctly() {
    fn receive_request(_: &Request) -> Result<Response, DomeneshopError> {
        let mut response = Response::new(StatusCode::Ok);
        response.set_body(
            "[{\"id\": 1, \"host\":\"t\", \"ttl\": 1, \"type\": \"CNAME\", \"data\": \"a\"}]",
        );
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    let response = client.list_dns_records(3).await.unwrap();
    let record = response.first().unwrap();
    match record {
        DnsRecord::CNAME(_) => {}
        _ => panic!("Wrong record type"),
    }
}

#[tokio::test]
async fn list_dns_deserializes_mx_record_correctly() {
    fn receive_request(_: &Request) -> Result<Response, DomeneshopError> {
        let mut response = Response::new(StatusCode::Ok);
        response.set_body(
            "[{\"id\": 1, \"host\":\"t\", \"ttl\": 1, \"type\": \"MX\", \"data\": \"a\", \"priority\": 1}]",
        );
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    let response = client.list_dns_records(3).await.unwrap();
    let record = response.first().unwrap();
    match record {
        DnsRecord::MX(_) => {}
        _ => panic!("Wrong record type"),
    }
}

#[tokio::test]
async fn list_dns_deserializes_srv_record_correctly() {
    fn receive_request(_: &Request) -> Result<Response, DomeneshopError> {
        let mut response = Response::new(StatusCode::Ok);
        response.set_body(
            "[{\"id\": 1, \"host\":\"t\", \"ttl\": 1, \"type\": \"SRV\", \"data\": \"a\", \"priority\": 1, \"weight\": 1, \"port\": 1}]",
        );
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    let response = client.list_dns_records(3).await.unwrap();
    let record = response.first().unwrap();
    match record {
        DnsRecord::SRV(_) => {}
        _ => panic!("Wrong record type"),
    }
}

#[tokio::test]
async fn list_dns_deserializes_txt_record_correctly() {
    fn receive_request(_: &Request) -> Result<Response, DomeneshopError> {
        let mut response = Response::new(StatusCode::Ok);
        response.set_body(
            "[{\"id\": 1, \"host\":\"t\", \"ttl\": 1, \"type\": \"TXT\", \"data\": \"a\"}]",
        );
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    let response = client.list_dns_records(3).await.unwrap();
    let record = response.first().unwrap();
    match record {
        DnsRecord::TXT(_) => {}
        _ => panic!("Wrong record type"),
    }
}
