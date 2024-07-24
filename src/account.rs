use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value;

use crate::{client::Client, EasyBit, Error};

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct Account {
    pub level: i32,
    pub volume: String,
    pub fee: String,
    pub extraFee: String,
    pub totalFee: String,
}

pub async fn get_account(client: &Client) -> Result<Account, Error> {
    // Define the URL.
    let path = "/account";

    // Make the request.
    let response = reqwest::Client::new()
        .get(format!("{}{}", client.get_url(), path))
        .header("API-KEY", client.get_api_key())
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => {
            let json: Value = response.json().await?;
            match json.get("data") {
                Some(data) => {
                    let account: Account = serde_json::from_value(data.clone())?;
                    Ok(account)
                }
                None => {
                    let error: EasyBit = serde_json::from_value(json)?;
                    log::error!("{:?}", error);
                    Err(Error::ApiError(error))
                }
            }
        }
        _ => {
            let error: EasyBit = response.json().await?;
            log::error!("{:?}", error);
            Err(Error::ApiError(error))
        }
    }
}

pub async fn set_fee(client: &Client, fee: f64) -> Result<(), Error> {
    // Define the URL.
    let path = "/setExtraFee";

    let field_name = "extraFee";

    // Make the request.
    let response = reqwest::Client::new()
        .post(format!("{}{}", client.get_url(), path))
        .header("API-KEY", client.get_api_key())
        .json(&serde_json::json!({field_name:fee}))
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => Ok(()),
        _ => {
            let error: EasyBit = response.json().await?;
            log::error!("{:?}", error);
            Err(Error::ApiError(error))
        }
    }
}

// Test account endpoints
#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::Client;
    use std::env;

    #[tokio::test]
    async fn test_get_account() {
        // Test the get_account function
        env_logger::builder().is_test(true).try_init().unwrap();
        // Read API_KEY from environment variable
        let client = Client::new(
            env::var("URL").expect("URL must be set"),
            env::var("API_KEY").expect("API_KEY must be set"),
        );
        let account = get_account(&client).await.unwrap();

        // Print the account information
        log::info!("Account: {:?}", account);
    }

    #[tokio::test]
    async fn test_set_fee() {
        // Test the set_fee function
        // Read API_KEY from environment variable
        let client = Client::new(
            env::var("URL").expect("URL must be set"),
            env::var("API_KEY").expect("API_KEY must be set"),
        );
        let initial_fee = "0";
        let new_fee = "0.002";

        log::info!("Initial fee: {}", initial_fee);

        // Set new fee
        set_fee(&client, new_fee.parse::<f64>().unwrap())
            .await
            .unwrap();

        // Get account information
        let account = get_account(&client).await.unwrap();

        log::info!("New fee: {}", account.extraFee);

        // Check if the fee has been set
        assert_eq!(account.extraFee, new_fee);

        // Reset the fee
        set_fee(&client, initial_fee.parse::<f64>().unwrap())
            .await
            .unwrap();

        // Get account information
        let account = get_account(&client).await.unwrap();

        log::info!("Reset fee back to: {}", account.extraFee);

        // Check if the fee has been reset
        assert_eq!(account.extraFee, initial_fee);
    }

    #[tokio::test]
    async fn test_invalid_api_key() {
        // Test the get_account function with an invalid API key
        // Read API_KEY from environment variable
        let client = Client::new(
            env::var("URL").expect("URL must be set"),
            "invalid_api_key".to_string(),
        );
        let result = get_account(&client).await;

        // Check if the error is an API error
        match result {
            Ok(_) => panic!("Expected an error"),
            Err(Error::ApiError(_)) => (),
            Err(_) => panic!("Expected an API error"),
        }
    }
}
