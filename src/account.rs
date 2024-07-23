use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value;

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
            let json: Value = response.json().await?;
            match json.get("data") {
                Some(data) => {
                    let account: Account = serde_json::from_value(data.clone())?;
                    Ok(account)
                }
                None => {
                    let error: EasyBit = serde_json::from_value(json)?;
                    log::error!("EasyBit error: {:?}", error);
                    return Err(Error::ApiError(error));
                }
            }
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
