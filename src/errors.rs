use std::{error::Error, fmt::Display};

use serde::Deserialize;

/// The error returned when operations fail against the API
#[derive(Debug, Clone)]
pub struct DomeneshopError {
    message: String,
}

impl Error for DomeneshopError {}

impl Display for DomeneshopError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

impl DomeneshopError {
    /// Creates a new `DomeneshopError`
    pub fn new(message: impl Into<String>) -> DomeneshopError {
        DomeneshopError {
            message: message.into(),
        }
    }

    pub(crate) fn from(text: impl Display) -> Self {
        Self {
            message: text.to_string(),
        }
    }
}

/// Error returned from the Domeneshop Api
#[derive(Deserialize, Debug)]
pub(crate) struct DomeneshopApiError {
    help: String,
    code: String,
}

impl Into<DomeneshopError> for DomeneshopApiError {
    fn into(self) -> DomeneshopError {
        DomeneshopError {
            message: format!("{}: {}", self.code, self.help),
        }
    }
}

pub(crate) fn to_domain_error(err: impl Display) -> DomeneshopError {
    DomeneshopError::from(err)
}

pub(crate) fn to_domain_error_with_context(
    context: impl Display,
    err: impl Display,
) -> DomeneshopError {
    DomeneshopError::from(format!("{}: {}", context, err))
}
