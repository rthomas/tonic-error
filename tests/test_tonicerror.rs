use serde::{Deserialize, Serialize};
use thiserror::Error;
use tonic::Status;
use tonic_error::TonicError;

#[derive(Clone, Debug, Eq, PartialEq, Error, Deserialize, Serialize, TonicError)]
pub enum TestError {
    #[error("some error: {0}")]
    AnError(String),
}

#[test]
fn test_convert() {
    let err = TestError::AnError("something".to_string());
    let s: Status = err.clone().try_into().unwrap();
    let err2: TestError = s.try_into().unwrap();
    assert_eq!(err, err2);
}

#[test]
fn test_no_metadata() {
    let err = TestError::AnError("something".to_string());
    let mut s: Status = err.try_into().unwrap();
    s.metadata_mut().clear();
    let res: Result<TestError, anyhow::Error> = s.try_into();
    match res {
        Err(_) => (),
        Ok(_) => panic!("key error did not come back when metadata was cleared"),
    }
}

#[test]
fn test_invalid_status_code() {
    let s = tonic::Status::deadline_exceeded("this is not the right status code...");
    let res: Result<TestError, anyhow::Error> = s.try_into();
    match res {
        Err(_) => (),
        Ok(_) => panic!("did not raise an error when the wrong status code was used"),
    }
}
