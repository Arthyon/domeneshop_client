use reqwest::{Error, Response};
use serde::de::DeserializeOwned;

use crate::client::DomeneshopError;

pub fn map_reqwest_err(err: Error) -> DomeneshopError {
    DomeneshopError {
        help: format!("Infrastructure error (StatusCode: {:#?})", err.status()),
        code: format!("{}", err),
    }
}

pub async fn map_unhandled_response(response: Response) -> DomeneshopError {
    match response.json::<DomeneshopError>().await {
        Ok(error) => error,
        Err(error) => map_reqwest_err(error),
    }
}

// pub async fn parse_optional_response<T>(response: Response) -> Result<Option<T>, DomeneshopError>
// where
//     T: DeserializeOwned,
// {
//     match response.status() {
//         reqwest::StatusCode::OK => match response.json::<T>().await {
//             Ok(parsed) => Ok(Some(parsed)),
//             Err(error) => Err(map_reqwest_err(error)),
//         },
//         _ => Err(map_unhandled_response(response).await),
//     }
// }

pub async fn parse_response<T>(response: Response) -> Result<T, DomeneshopError>
where
    T: DeserializeOwned,
{
    match response.status() {
        reqwest::StatusCode::OK => match response.json::<T>().await {
            Ok(parsed) => Ok(parsed),
            Err(error) => Err(map_reqwest_err(error)),
        },
        _ => Err(map_unhandled_response(response).await),
    }
}
