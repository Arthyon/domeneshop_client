use domeneshop_client::{
    self, endpoints::forwards::HttpForward, errors::DomeneshopError, http_client::mock::MockClient,
};
use http_types::{Method, Request, Response, StatusCode};
use url::Url;

use crate::common::{assert_url_equal, create_client};
mod common;

#[tokio::test]
async fn find_forward_deserializes_correctly() {
    async fn receive_request(req: Request) -> Result<Response, DomeneshopError> {
        assert_url_equal(req.url(), "/domains/3/forwards/www");
        let mut response = Response::new(StatusCode::Ok);
        response.set_body("{\"host\":\"t\", \"frame\": false, \"url\": \"https://example.com\"}");
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    let response = client
        .find_forward_by_host(3, "www".to_string())
        .await
        .unwrap();

    assert!(response.is_some());
}

#[tokio::test]
async fn find_forward_404_returns_none() {
    async fn receive_request(_: Request) -> Result<Response, DomeneshopError> {
        let response = Response::new(StatusCode::NotFound);
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    let response = client
        .find_forward_by_host(3, "www".to_string())
        .await
        .unwrap();

    assert!(response.is_none());
}

#[tokio::test]
async fn list_forwards_deserializes_correctly() {
    async fn receive_request(req: Request) -> Result<Response, DomeneshopError> {
        let mut response = Response::new(StatusCode::Ok);
        response.set_body("[{\"host\":\"t\",\"frame\":false,\"url\":\"https://example.com\"}]");
        assert_url_equal(req.url(), "/domains/3/forwards");
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    let response = client.list_forwards(3).await.unwrap();

    assert_eq!(1, response.len())
}

#[tokio::test]
async fn add_forward_serializes_response_correctly() {
    async fn receive_request(mut req: Request) -> Result<Response, DomeneshopError> {
        assert_url_equal(req.url(), "/domains/3/forwards");
        let json = req.body_string().await.unwrap();
        let expected = "{\"host\":\"www\",\"frame\":false,\"url\":\"https://example.com/\"}";
        assert_eq!(json, expected);

        let mut response = Response::new(StatusCode::Created);
        response.set_body("{\"host\":\"www\", \"frame\": false, \"url\": \"https://example.com\"}");
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    let forward = HttpForward {
        frame: false,
        host: "www".to_string(),
        url: Url::parse("https://example.com").unwrap(),
    };

    client.add_forward(3, forward).await.unwrap();
}

#[tokio::test]
async fn update_forward_serializes_correctly() {
    async fn receive_request(mut req: Request) -> Result<Response, DomeneshopError> {
        assert_url_equal(req.url(), "/domains/3/forwards/www");
        let json = req.body_string().await.unwrap();
        let expected = "{\"host\":\"www\",\"frame\":false,\"url\":\"https://example.com/\"}";
        assert_eq!(json, expected);

        let mut response = Response::new(StatusCode::Ok);
        response.set_body("{\"host\":\"www\", \"frame\": false, \"url\": \"https://example.com\"}");
        Ok(response)
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    let forward = HttpForward {
        frame: false,
        host: "www".to_string(),
        url: Url::parse("https://example.com").unwrap(),
    };

    client.update_forward(3, forward).await.unwrap();
}

#[tokio::test]
async fn delete_forward_succeeds() {
    async fn receive_request(req: Request) -> Result<Response, DomeneshopError> {
        assert_url_equal(req.url(), "/domains/3/forwards/www");
        assert_eq!(req.method(), Method::Delete);
        Ok(Response::new(StatusCode::NoContent))
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    client.delete_forward(3, "www".to_string()).await.unwrap();
}

#[tokio::test]
async fn delete_forward_404_returns_err() {
    async fn receive_request(_: Request) -> Result<Response, DomeneshopError> {
        Ok(Response::new(StatusCode::NotFound))
    }

    let mock = MockClient {
        req_received: receive_request,
    };

    let client = create_client(mock);

    let response = client.delete_forward(3, "www".to_string()).await;
    assert!(response.is_err());
}
