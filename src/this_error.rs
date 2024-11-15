#![allow(unused)]
use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum ThisError {
    #[error("Internal Server Error")]
    ServerError,
    #[error("Validation Error: Field: {} Failure: {}", .field_name, .failure_str)]
    ValidationError {
        field_name: String,
        failure_str: String
    },
    #[error("Network error: {}", .0)]
    NetworkError(io::Error)
}