use std::fmt::Display;

use crate::client::DomeneshopError;

pub fn to_domain_error(err: impl Display) -> DomeneshopError {
    DomeneshopError::from(err)
}

pub fn to_domain_error_with_context(context: impl Display, err: impl Display) -> DomeneshopError {
    DomeneshopError::from(format!("{}: {}", context, err))
}
