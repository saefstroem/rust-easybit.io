use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value;

use crate::{client::Client, EasyBit, Error};

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
/**
    ### Exchange rate information.

    - `rate`: Exchange rate
    - `sendAmount`: Amount to send
    - `receiveAmount`: Amount to receive
    - `networkFee`: Network fee
    - `confirmations`: Number of confirmations required
    - `processingTime`: Processing time
*/
pub struct ExchangeRate {
    pub rate: String,
    pub sendAmount: String,
    pub receiveAmount: String,
    pub networkFee: String,
    pub confirmations: i32,
    pub processingTime: String,
}

#[allow(clippy::too_many_arguments)]
pub async fn get_exchange_rate(
    client: &Client,
    send: String,
    receive: String,
    amount: f64,
    send_network: Option<String>,
    receive_network: Option<String>,
    amount_type: Option<String>,
    extra_fee_override: Option<f64>,
) -> Result<ExchangeRate, Error> {
    // Define the path.
    let path = "/rate";

    // Make the request and set API key.
    let response = reqwest::Client::new()
        .get(format!("{}{}", client.get_url(), path))
        .header("API-KEY", client.get_api_key())
        .query(&[
            ("send", send),
            ("receive", receive),
            ("amount", amount.to_string()),
            ("sendNetwork", send_network.unwrap_or_default()),
            ("receiveNetwork", receive_network.unwrap_or_default()),
            ("amountType", amount_type.unwrap_or_default()),
            (
                "extraFeeOverride",
                extra_fee_override.unwrap_or_default().to_string(),
            ),
        ])
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => {
            let json: Value = response.json().await?;
            match json.get("data") {
                Some(data) => {
                    let exchange_rate: ExchangeRate = serde_json::from_value(data.clone())?;
                    Ok(exchange_rate)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::Client;
    use std::env;

    #[tokio::test]
    async fn test_get_exchange_rate() {
        let client = Client::new(env::var("URL").unwrap(), env::var("API_KEY").unwrap());
        let exchange_rate = get_exchange_rate(
            &client,
            "BTC".to_string(),
            "ETH".to_string(),
            1.0,
            Some("BTC".to_string()),
            Some("ETH".to_string()),
            None,
            None,
        )
        .await
        .unwrap();
        log::info!("{:?}", exchange_rate);

        assert!(exchange_rate.rate.parse::<f64>().unwrap() > 0.0);
    }
}
