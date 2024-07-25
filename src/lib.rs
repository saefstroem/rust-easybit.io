use serde::Deserialize;
use std::fmt;
use thiserror::Error;

mod account;
mod currency;
mod kyc;
mod orders;


/**
# Easybit.io API client.
   Fully asynchronous wrapper for the easybit.io API.

## Getting an API Key

Head over to [easybit.io](https://easybit.io) and sign up for an account. Once you have an account, you can
request to retrieve an API key by sending an email to the EasyBit team.

### Usage
```rust
use easybit::client::Client;
use std::env;

#[tokio::main]
async fn main() {
    let client = Client::new(env::var("URL").expect("URL must be set"),
    env::var("API_KEY").expect("API_KEY must be set"));
    let account = client.get_account().await.unwrap();
    println!("{:?}", account);
}
```
*/
pub mod client;


#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
/**
 * Common error structure for the EasyBit API.
 */
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
/**
### Catch-all error structure for this library.
 * If a deserialization error occurs, it is likely that the underlying API has changed and the library needs to be updated. Kindly create an issue on GitHub.
 * If a network error occurs, it is likely that the API is down or the URL is incorrect.
 * If an API error occurs, the API has returned an error message, and you should review your code.
 */
pub enum Error {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("JSON deserialization error: {0}")]
    DeserializeError(#[from] serde_json::Error),
    #[error("{0}")]
    ApiError(EasyBit),
}
