tonic::include_proto!("example");

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tonic_error::TonicError;

#[derive(Debug, Error, TonicError, Serialize, Deserialize)]
pub enum MathsError {
    #[error("division by zero for inputs: a={0} b={1}")]
    DivByZero(i32, i32),
}
