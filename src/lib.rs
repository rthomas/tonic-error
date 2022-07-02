use serde::{Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;
use tonic::metadata::errors::{InvalidMetadataValue, ToStrError};
use tonic::Status;

pub use tonic_error_impl::*;

static CUSTOM_ERROR: &str = "x-custom-tonic-error";

#[derive(Debug, Error)]
pub enum TonicErrorError {
    #[error("serde_json error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("ToStrError: {0}")]
    MetadataStrError(#[from] ToStrError),

    #[error("the error key '{0}' was not found in the metadata")]
    ErrorKeyNotFound(String),

    #[error("status code was not internal error: {0}")]
    InvalidStatusCode(tonic::Code),
}

pub trait TonicError<'de>: Serialize + Deserialize<'de> + Display {
    fn to_status(&self) -> Status {
        let metadata_val = match serde_json::to_string(&self) {
            Ok(s) => match s.parse() {
                Ok(m) => m,
                Err(e) => {
                    return Status::internal(format!(
                        "error creating metadata value from previous error: {e}"
                    ))
                }
            },
            Err(e) => {
                return Status::internal(format!("error converting previous error to json: {e}"))
            }
        };

        let mut status = Status::internal(format!("internal error: {self}"));

        status.metadata_mut().insert(CUSTOM_ERROR, metadata_val);
        status
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
