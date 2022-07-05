use serde::{Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;
use tonic::metadata::errors::ToStrError;

pub use tonic_error_impl::*;

/// A derive trait to implement the `TryFrom` and `From` traits to and from
/// `tonic::Status` for a custom error type.
pub trait TonicError<'de>: Serialize + Deserialize<'de> + Display {}

/// This is the error type for the `TryFrom` impl for `tonic::Status`. If the
/// invalid `tonic::Code` is set in the `tonic::Status`; or if the metadata
/// within the status is not set; or if invalid strings are used; or if the json
/// cannot be parsed.  
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("missing metadata entry")]
    MissingMetadata,

    #[error("invalid status code set")]
    InvalidStatusCode(tonic::Status),

    #[error("could not parse metadata to string")]
    MetadataParseError(#[from] ToStrError),

    #[error("serde json error")]
    JsonError(#[from] serde_json::Error),
}
