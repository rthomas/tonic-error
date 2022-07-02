use serde::{Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;
use tonic::metadata::errors::ToStrError;
use tonic::Status;

pub use tonic_error_impl::*;

static CUSTOM_ERROR: &str = "x-custom-tonic-error";

/// Represents an error in converting a custom error type into a
/// `tonic::Status`.
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

/// A trait to convert a custom error type into a `tonic::Status` and back
/// again. The custom error is serialized to json using `serde_json` and stored
/// in the `Status` metadata.
///
/// The `tonic::Code::Internal` is used when converting to the `Status`, and
/// required when converting from a `Status`. A
/// `TonicErrorError::InvalidStatusCode` will be returned if the `Status` is any
/// other code. A `TonicErrorError::ErrorKeyNotFound` will be returned if the
/// data is not present in the `Status` metadata map.
pub trait TonicError<'de>: Serialize + Deserialize<'de> + Display {
    /// Convert this type into a `tonic::Status` by serializing it into the
    /// metadata map. If `serde_json` fails, or if the `String` does not parse
    /// into a `MetadataValue<Ascii>` then this will be reported in the `Status`
    /// itself as an internal error.
    fn to_status(&self) -> Status {
        let metadata_val = match serde_json::to_string(&self) {
            Ok(s) => match s.parse() {
                Ok(m) => m,
                Err(e) => {
                    return Status::internal(format!(
                        "error creating metadata value from previous error: {e} - {self}"
                    ))
                }
            },
            Err(e) => {
                return Status::internal(format!(
                    "error converting previous error to json: {e} - {self}"
                ))
            }
        };

        let mut status = Status::internal(format!("internal error: {self}"));

        status.metadata_mut().insert(CUSTOM_ERROR, metadata_val);
        status
    }

    /// Deserialize this type out of a `tonic::Status` metadata field. Errors
    /// will be returned if the `Status` is not `Code::Internal` or if there is
    /// no data present in the metadata map that we can deserialize.
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
