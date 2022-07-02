use serde::{Deserialize, Serialize};
use thiserror::Error;
use tonic_error::{TonicError, TonicErrorError};

#[derive(Debug, Eq, PartialEq, Error, Deserialize, Serialize, TonicError)]
pub enum TestError {
    #[error("some error: {0}")]
    AnError(String),
}

#[test]
fn test_convert() {
    let err = TestError::AnError("something".to_string());
    let s = err.to_status();
    let err2 = TonicError::from_status(&s).unwrap();
    assert_eq!(err, err2);
}

#[test]
fn test_no_metadata() {
    let err = TestError::AnError("something".to_string());
    let mut s = err.to_status();
    s.metadata_mut().clear();
    let res: Result<TestError, TonicErrorError> = TonicError::from_status(&s);
    match res {
        Err(TonicErrorError::ErrorKeyNotFound(_)) => (),
        _ => panic!("key error did not come back when metadata was cleared"),
    }
}

#[test]
fn test_invalid_status_code() {
    let s = tonic::Status::deadline_exceeded("this is not the right status code...");
    let res: Result<TestError, TonicErrorError> = TonicError::from_status(&s);
    match res {
        Err(TonicErrorError::InvalidStatusCode(_)) => (),
        _ => panic!("did not raise an error when the wrong status code was used"),
    }
}
