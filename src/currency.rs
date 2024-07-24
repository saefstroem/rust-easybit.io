use crate::{client::Client, EasyBit, Error};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
/**
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

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
/*
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

pub async fn get_pair_list(client: &Client) -> Result<Vec<String>, Error> {
    // Define the URL.
    let path = "/pairList";

    // Make the request and set API key.
    let response = reqwest::Client::new()
        .get(format!("{}{}", client.get_url(), path))
        .header("API-KEY", client.get_api_key())
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => {
            // Convert the response to a Vec<String>
            let json: Value = response.json().await?;
            match json.get("data") {
                Some(data) => {
                    let pair_list: Vec<String> = serde_json::from_value(data.clone())?;
                    Ok(pair_list)
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

pub async fn validate_address(
    client: &Client,
    currency: String,
    address: String,
    network: Option<String>,
    tag: Option<String>,
) -> Result<(), Error> {
    // Define the path.
    let path = "/validateAddress";

    log::info!("{:?}", format!("{}{}", client.get_url(), path));
    // Make the GET request and set API key. The query should only contain items that are not None.
    let request = reqwest::Client::new()
        .get(format!("{}{}", client.get_url(), path))
        .header("API-KEY", client.get_api_key());

    // Even if the network and tag parameters are empty, the API complains.
    // So we must only include them if they are Some.
    let mut query_tuple_array: Vec<(&str, String)> = Vec::new();

    query_tuple_array.push(("currency", currency));
    query_tuple_array.push(("address", address));


    if let Some(network) = network {
        query_tuple_array.push(("network", network));
    }

    if let Some(tag) = tag {
        query_tuple_array.push(("tag", tag));
    }

    let response = request.query(&query_tuple_array).send().await?;

    match response.status() {
        StatusCode::OK => Ok(()),
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

    #[tokio::test]
    async fn test_get_pair_list() {
        let client = Client::new(env::var("URL").unwrap(), env::var("API_KEY").unwrap());
        let pair_list = get_pair_list(&client).await.unwrap();

        // Print the first three pairs.
        for pair in pair_list.iter().take(1) {
            println!("{:?}", pair);
        }

        assert!(pair_list.len() > 0);
    }

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

    #[tokio::test]
    async fn test_validate_address() {
        let client = Client::new(env::var("URL").unwrap(), env::var("API_KEY").unwrap());
        let result = validate_address(
            &client,
            "BTC".to_string(),
            "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
            Some("BTC".to_string()),
            None,
        )
        .await
        .unwrap();
        log::info!("{:?}", result);

        assert!(result == ());
    }
}
