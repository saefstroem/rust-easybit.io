use serde::Deserialize;
use thiserror::Error;

mod account;
pub mod client;
mod currency;
use std::fmt;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct EasyBit {
    pub errorMessage: String,
    pub errorCode: i32,
}

impl fmt::Display for EasyBit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EasyBit {}: {}", self.errorCode, self.errorMessage)
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("JSON deserialization error: {0}")]
    DeserializeError(#[from] serde_json::Error),
    #[error("{0}")]
    ApiError(EasyBit),
}



