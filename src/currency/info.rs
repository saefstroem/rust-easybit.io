use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value;

use crate::{client::Client, EasyBit, Error};

#[derive(Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
/**
    ### Currency information.
    
    - `currency`: Currency code
    - `name`: Currency name
    - `sendStatusAll`: If the system can send this currency through at least one network
    - `receiveStatusAll`: If the system can receive this currency through at least one network
    - `networkList`: List of networks
*/
pub struct Currency {
    pub currency: String,
    pub name: String,
    pub sendStatusAll: bool,
    pub receiveStatusAll: bool,
    pub networkList: Vec<Network>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
/**
    - `network`: Network code
    - `name`: Network name
    - `isDefault`: If the network is the default network
    - `sendStatus`: If the system can send through this network
    - `receiveStatus`: If the system can receive through this network
    - `receiveDecimals`: Number of decimals for the currency
    - `confirmationsMinimum`: Minimum number of confirmations required
    - `confirmationsMaximum`: Maximum number of confirmations required
    - `explorer`: URL for the explorer
    - `explorerHash`: URL for the hash explorer
    - `explorerAddress`: URL for the address explorer
    - `hasTag`: If the network requires a tag
    - `tagName`: Name of the tag
    - `contractAddress`: Contract address for the network
    - `explorerContract`: URL for the contract explorer
*/
pub struct Network {
    pub network: String,
    pub name: String,
    pub isDefault: bool,
    pub sendStatus: bool,
    pub receiveStatus: bool,
    pub receiveDecimals: i32,
    pub confirmationsMinimum: i32,
    pub confirmationsMaximum: i32,
    pub explorer: String,
    pub explorerHash: String,
    pub explorerAddress: String,
    pub hasTag: bool,
    pub tagName: Option<String>,
    pub contractAddress: Option<String>,
    pub explorerContract: Option<String>,
}

pub async fn get_currency_list(client: &Client) -> Result<Vec<Currency>, Error> {
    // Define the URL.
    let path = "/currencyList";

    // Make the request and set API key.
    let response = reqwest::Client::new()
        .get(format!("{}{}", client.get_url(), path))
        .header("API-KEY", client.get_api_key())
        .send()
        .await?;

    let json: Value = response.json().await?;
    match json.get("data") {
        Some(data) => {
            let currency_list: Vec<Currency> = serde_json::from_value(data.clone())?;
            Ok(currency_list)
        }
        None => {
            let error: EasyBit = serde_json::from_value(json)?;
            log::error!("{:?}", error);
            Err(Error::ApiError(error))
        }
    }
}

pub async fn get_single_currency(client: &Client, currency: String) -> Result<Currency, Error> {
    // Define the URL with the currency as a query parameter.
    let path = format!("/currencyList?currency={}", currency);

    // Make the request and set API key.
    let response = reqwest::Client::new()
        .get(format!("{}{}", client.get_url(), path))
        .header("API-KEY", client.get_api_key())
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => {
            // Convert the response to an object. Do not use unwrap.
            let json: Value = response.json().await?;
            match json.get("data") {
                Some(data) => {
                    // Print the data.
                    let currency: Vec<Currency> = serde_json::from_value(data.clone())?;

                    if currency.is_empty() {
                        return Err(Error::ApiError(EasyBit {
                            errorMessage: "Currency not found".to_string(),
                            errorCode: 404,
                        }));
                    }
                    Ok(currency[0].clone())
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
    async fn test_get_currency_list() {
        let client = Client::new(env::var("URL").unwrap(), env::var("API_KEY").unwrap());
        let currency_list = get_currency_list(&client).await.unwrap();

        // Print the first three currencies.
        for currency in currency_list.iter().take(1) {
            println!("{:?}", currency);
        }

        assert!(currency_list.len() > 0);
    }

    #[tokio::test]
    async fn test_get_single_currency() {
        let client = Client::new(env::var("URL").unwrap(), env::var("API_KEY").unwrap());
        let currency = get_single_currency(&client, "BTC".to_string())
            .await
            .unwrap();
        println!("{:?}", currency);
        assert_eq!(currency.currency, "BTC");
    }
}
