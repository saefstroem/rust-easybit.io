use reqwest::StatusCode;
use serde::Deserialize;

use crate::{client::Client, EasyBit, Error};

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct Account {
    pub level: i32,
    pub volume: f64,
    pub fee: f64,
    pub extraFee: f64,
    pub totalFee: f64,
}

pub async fn get_account(client: &Client) -> Result<Account, Error> {
    // Define the URL.
    let url = "https://api.easybit.com/account";

    // Make the request.
    let response = reqwest::Client::new()
        .get(url)
        .header("API-KEY", client.get_api_key())
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => {
            // Convert the response to an object. Do not use unwrap.
            let account: Account = response.json().await?;

            // Return the account object.
            Ok(account)
        }
        _ => {
            let error: EasyBit = response.json().await?;
            log::error!("EasyBit error: {:?}", error);
            return Err(Error::ApiError(error));
        }
    }
}

pub async fn set_fee(client: &Client, fee: f64) -> Result<(), Error> {
    // Define the URL.
    let url = "https://api.easybit.com/setExtraFee";

    let field_name = "extraFee";

    // Make the request.
    let response = reqwest::Client::new()
        .post(url)
        .header("API-KEY", client.get_api_key())
        .json(&serde_json::json!({field_name:fee}))
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => Ok(()),
        _ => {
            let error: EasyBit = response.json().await?;
            log::error!("EasyBit error: {:?}", error);
            return Err(Error::ApiError(error));
        }
    }
}
