use serde::{Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;
use tonic::metadata::errors::{InvalidMetadataValue, ToStrError};
use tonic::Status;

pub use tonic_error_impl::*;

static CUSTOM_ERROR: &str = "x-custom-tonic-error";

#[derive(Debug, Error)]
pub enum TonicErrorError {
    #[error("could not set metadata value: {0}")]
    MetadataError(#[from] InvalidMetadataValue),

    #[error("serde_json error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("ToStrError: {0}")]
    MetadataStrError(#[from] ToStrError),

    #[error("the error key '{0}' was not found in the metadata")]
    ErrorKeyNotFound(String),

    #[error("status code was not internal error: {0}")]
    InvalidStatusCode(tonic::Code),
}

pub trait TonicError<'de>: Serialize + Deserialize<'de> + Display + Sized {
    fn to_status(&self) -> Result<Status, TonicErrorError> {
        let mut status = Status::internal(format!("internal error: {self}"));
        status
            .metadata_mut()
            .insert(CUSTOM_ERROR, serde_json::to_string(&self)?.parse()?);
        Ok(status)
    }

    fn from_status(s: &'de Status) -> Result<Self, TonicErrorError> {
        match s.code() {
            tonic::Code::Internal => {
                if let Some(err) = s.metadata().get(CUSTOM_ERROR) {
                    Ok(serde_json::from_str(err.to_str()?)?)
                } else {
                    Err(TonicErrorError::ErrorKeyNotFound(CUSTOM_ERROR.to_string()))
                }
            }
            _ => Err(TonicErrorError::InvalidStatusCode(s.code())),
        }
    }
}
