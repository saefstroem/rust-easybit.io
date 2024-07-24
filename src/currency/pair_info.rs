use crate::{client::Client, EasyBit, Error};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
/*
    - `minimumAmount`: Minimum amount that can be sent
    - `maximumAmount`: Maximum amount that can be sent
    - `networkFee`: Network fee
    - `confirmtions`: Number of confirmations required
    - `processingTime`: Processing time
*/
pub struct Pair {
    pub minimumAmount: String,
    pub maximumAmount: String,
    pub networkFee: String,
    pub confirmations: i32,
    pub processingTime: String,
}

#[allow(non_snake_case)]
pub async fn get_pair_info(
    client: &Client,
    send: String,
    receive: String,
    sendNetwork: Option<String>,
    receiveNetwork: Option<String>,
    amountType: Option<String>,
) -> Result<Pair, Error> {
    // Define the path.
    let path = "/pairInfo";

    // Make the request and set API key.
    let response = reqwest::Client::new()
        .get(format!("{}{}", client.get_url(), path))
        .header("API-KEY", client.get_api_key())
        .query(&[
            ("send", send),
            ("receive", receive),
            ("sendNetwork", sendNetwork.unwrap_or_default()),
            ("receiveNetwork", receiveNetwork.unwrap_or_default()),
            ("amountType", amountType.unwrap_or_default()),
        ])
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => {
            let json: Value = response.json().await?;
            match json.get("data") {
                Some(data) => {
                    let pair: Pair = serde_json::from_value(data.clone())?;
                    Ok(pair)
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
    async fn test_get_pair_info() {
        let client = Client::new(env::var("URL").unwrap(), env::var("API_KEY").unwrap());
        let pair = get_pair_info(
            &client,
            "BTC".to_string(),
            "ETH".to_string(),
            Some("BTC".to_string()),
            Some("ETH".to_string()),
            None,
        )
        .await
        .unwrap();
        log::info!("{:?}", pair);
    }
}
