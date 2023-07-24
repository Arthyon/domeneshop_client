use std::fmt::Display;

use crate::client::DomeneshopError;

pub fn map_http_types_error(
    context: impl Into<String> + Display,
    error: http_types::Error,
) -> DomeneshopError {
    DomeneshopError {
        help: format!("{}: {}", context, error),
        code: "InfrastructureError".to_string(),
    }
}
