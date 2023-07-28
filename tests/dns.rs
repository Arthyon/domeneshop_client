use domeneshop_client::{
    self,
    client::DomeneshopError,
    endpoints::dns::{CNAMERecordData, DnsRecordData, DnsType, ExistingDnsRecord},
    http_client::mock::MockClient,
};
use http_types::{Request, Response, StatusCode};

use crate::common::{assert_url_equal, create_client};
mod common;

#[tokio::test]
async fn get_dns_deserializes_correctly() {
    async fn receive_request(req: Request) -> Result<Response, DomeneshopError> {
        assert_url_equal(req.url(), "/domains/3/dns/2");
        let mut response = Response::new(StatusCode::Ok);
        response
            .set_body("{\"id\": 2, \"host\":\"t\", \"ttl\": 1, \"type\": \"A\", \"data\": \"a\"}");
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    let response = client.get_dns_record(3, 2).await.unwrap();
    assert_eq!(2, response.id);
    match response.data {
        DnsRecordData::A(_) => {}
        _ => panic!("Wrong record type"),
    }
}

#[tokio::test]
async fn list_dns_formats_url_correctly() {
    async fn receive_request(req: Request) -> Result<Response, DomeneshopError> {
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
    async fn receive_request(req: Request) -> Result<Response, DomeneshopError> {
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
    async fn receive_request(req: Request) -> Result<Response, DomeneshopError> {
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
    async fn receive_request(req: Request) -> Result<Response, DomeneshopError> {
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
    async fn receive_request(_: Request) -> Result<Response, DomeneshopError> {
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
    match record.data {
        DnsRecordData::A(_) => {}
        _ => panic!("Wrong record type"),
    }
}

#[tokio::test]
async fn list_dns_deserializes_aaaa_record_correctly() {
    async fn receive_request(_: Request) -> Result<Response, DomeneshopError> {
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
    match record.data {
        DnsRecordData::AAAA(_) => {}
        _ => panic!("Wrong record type"),
    }
}

#[tokio::test]
async fn list_dns_deserializes_cname_record_correctly() {
    async fn receive_request(_: Request) -> Result<Response, DomeneshopError> {
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
    match record.data {
        DnsRecordData::CNAME(_) => {}
        _ => panic!("Wrong record type"),
    }
}

#[tokio::test]
async fn list_dns_deserializes_mx_record_correctly() {
    async fn receive_request(_: Request) -> Result<Response, DomeneshopError> {
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
    match record.data {
        DnsRecordData::MX(_) => {}
        _ => panic!("Wrong record type"),
    }
}

#[tokio::test]
async fn list_dns_deserializes_srv_record_correctly() {
    async fn receive_request(_: Request) -> Result<Response, DomeneshopError> {
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
    match record.data {
        DnsRecordData::SRV(_) => {}
        _ => panic!("Wrong record type"),
    }
}

#[tokio::test]
async fn list_dns_deserializes_txt_record_correctly() {
    async fn receive_request(_: Request) -> Result<Response, DomeneshopError> {
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
    match record.data {
        DnsRecordData::TXT(_) => {}
        _ => panic!("Wrong record type"),
    }
}

#[tokio::test]
async fn add_dns_serializes_response_correctly() {
    async fn receive_request(mut req: Request) -> Result<Response, DomeneshopError> {
        assert_url_equal(req.url(), "/domains/3/dns");
        let json = req.body_string().await.unwrap();
        let expected = "{\"type\":\"CNAME\",\"host\":\"t\",\"ttl\":1,\"data\":\"a\"}";
        assert_eq!(json, expected);

        let mut response = Response::new(StatusCode::Created);
        response.set_body("{\"id\": 5}");
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    let record = DnsRecordData::CNAME(CNAMERecordData {
        host: "t".to_string(),
        ttl: 1,
        data: "a".to_string(),
    });

    let response = client.add_dns_record(3, record).await.unwrap();
    assert_eq!(response.id, 5);
}

#[tokio::test]
async fn add_dns_gets_url_from_location_header() {
    async fn receive_request(_: Request) -> Result<Response, DomeneshopError> {
        let mut response = Response::new(StatusCode::Created);
        response.append_header("Location", "https://example.com");
        response.set_body("{\"id\": 5}");
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    let record = DnsRecordData::CNAME(CNAMERecordData {
        host: "t".to_string(),
        ttl: 1,
        data: "a".to_string(),
    });

    let response = client.add_dns_record(3, record).await.unwrap();
    assert_eq!(response.url.unwrap().to_string(), "https://example.com/");
}

#[tokio::test]
async fn add_dns_invalid_url_in_location_header_results_in_none() {
    async fn receive_request(_: Request) -> Result<Response, DomeneshopError> {
        let mut response = Response::new(StatusCode::Created);
        response.append_header("Location", "relative/url");
        response.set_body("{\"id\": 5}");
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    let record = DnsRecordData::CNAME(CNAMERecordData {
        host: "t".to_string(),
        ttl: 1,
        data: "a".to_string(),
    });

    let response = client.add_dns_record(3, record).await.unwrap();
    assert!(response.url.is_none())
}

#[tokio::test]
async fn add_dns_missing_location_header_results_in_none_url() {
    async fn receive_request(_: Request) -> Result<Response, DomeneshopError> {
        let mut response = Response::new(StatusCode::Created);
        response.set_body("{\"id\": 5}");
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    let record = DnsRecordData::CNAME(CNAMERecordData {
        host: "t".to_string(),
        ttl: 1,
        data: "a".to_string(),
    });

    let response = client.add_dns_record(3, record).await.unwrap();
    assert!(response.url.is_none());
}

#[tokio::test]
async fn update_dns_serializes_correctly() {
    async fn receive_request(mut req: Request) -> Result<Response, DomeneshopError> {
        assert_url_equal(req.url(), "/domains/3/dns/1");
        let json = req.body_string().await.unwrap();
        let expected = "{\"type\":\"CNAME\",\"host\":\"t\",\"ttl\":1,\"data\":\"a\"}";
        assert_eq!(json, expected);
        Ok(Response::new(StatusCode::NoContent))
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    let record = ExistingDnsRecord {
        id: 1,
        data: DnsRecordData::CNAME(CNAMERecordData {
            host: "t".to_string(),
            ttl: 1,
            data: "a".to_string(),
        }),
    };

    client.update_dns_record(3, record).await.unwrap();
}
