use std::fmt::Debug;

use domeneshop_client::{
    client::{DomeneshopClient, DomeneshopClientConfiguration, API_VERSION},
    http_client::mock::MockClient,
};
use http_types::Response;
use serde::Serialize;
use url::Url;

pub const TEST_BASE_URL: &str = "https://test.local";

pub fn create_client(underlying_client: MockClient) -> DomeneshopClient {
    DomeneshopClient::new(
        String::from("token"),
        String::from("secret"),
        DomeneshopClientConfiguration {
            user_agent: None,
            base_url: Some(TEST_BASE_URL.to_string()),
            underlying_client: Some(Box::new(underlying_client)),
        },
    )
    .unwrap()
}

pub fn set_body<T>(response: &mut Response, model: T)
where
    T: Serialize + Debug,
{
    let json = serde_json::to_string(&model).unwrap();
    response.set_body(json);
}

pub fn assert_url_equal(url: &Url, relative_url: &str) {
    let url_string = url.to_string();
    assert_eq!(
        url_string,
        format!("{}/{}{}", TEST_BASE_URL, API_VERSION, relative_url)
    );
}
