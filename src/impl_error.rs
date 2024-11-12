#![allow(unused)]
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum MyError {
    ServerError,
    ValidationError {
        field_name: String,
        failure_str: String
    },
    NetworkError(String)
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::ServerError => write!(f, "{}", "Internal Server Error"),
            MyError::ValidationError { field_name, failure_str } =>
            write!(f, "Field: {} Failure: {}", field_name, failure_str),
            MyError::NetworkError(msg) => write!(f, "Network failure: {}", msg)
        }
    }
}

impl Error for MyError {}