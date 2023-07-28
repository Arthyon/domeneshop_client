use domeneshop_client::{
    client::{DomeneshopClient, DomeneshopClientConfiguration, API_VERSION},
    errors::DomeneshopError,
    http_client::mock::MockClient,
};
use http_types::Response;
use url::Url;

pub const TEST_BASE_URL: &str = "https://test.local";

pub fn create_client<F>(underlying_client: MockClient<F>) -> DomeneshopClient
where
    F: std::future::Future<Output = Result<Response, DomeneshopError>> + Send + 'static,
{
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

pub fn assert_url_equal(url: &Url, relative_url: &str) {
    let url_string = url.to_string();
    assert_eq!(
        url_string,
        format!("{}/{}{}", TEST_BASE_URL, API_VERSION, relative_url)
    );
}
