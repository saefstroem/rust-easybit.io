use crate::{client::Client, EasyBit, Error};
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
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

#[derive(Deserialize)]
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

#[derive(Deserialize)]
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
    pub confirmtions: i32,
    pub processingTime: String,
}

pub async fn get_currency_list(client: &Client) -> Result<Vec<Currency>, Error> {
    // Define the URL.
    let url = "https://api.easybit.com/currencyList";

    // Make the request and set API key.
    let response = reqwest::Client::new()
        .get(url)
        .header("API-KEY", client.get_api_key())
        .send()
        .await?;

    // If the response is not a 200, return an easybit error.
    if response.status() != reqwest::StatusCode::OK {
        let error: EasyBit = response.json().await?;
        log::error!("EasyBit error: {:?}", error);
        return Err(Error::ApiError(error));
    }

    // Convert the response to an object. Do not use unwrap.
    let currency_list: Vec<Currency> = response.json().await?;

    // Return the currency list.
    Ok(currency_list)
}

pub async fn get_single_currency(client: &Client, currency: String) -> Result<Currency, Error> {
    // Define the URL with the currency as a query parameter.
    let url = format!("https://api.easybit.com/currencyList?currency={}", currency);

    // Make the request and set API key.
    let response = reqwest::Client::new()
        .get(url)
        .header("API-KEY", client.get_api_key())
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => {
            // Convert the response to an object. Do not use unwrap.
            let currency: Currency = response.json().await?;
            Ok(currency)
        }
        _ => {
            let error: EasyBit = response.json().await?;
            log::error!("EasyBit error: {:?}", error);
            return Err(Error::ApiError(error));
        }
    }
}

pub async fn get_pair_list(client: &Client) -> Result<Vec<String>, Error> {
    // Define the URL.
    let url = "https://api.easybit.com/pairList";

    // Make the request and set API key.
    let response = reqwest::Client::new()
        .get(url)
        .header("API-KEY", client.get_api_key())
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => {
            // Convert the response to a Vec<String>
            let pair_list: Vec<String> = response.json().await?;
            Ok(pair_list)
        }
        _ => {
            let error: EasyBit = response.json().await?;
            log::error!("EasyBit error: {:?}", error);
            return Err(Error::ApiError(error));
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
    // Define the URL.
    let url = "https://api.easybit.com/pairInfo";

    // Make the request and set API key.
    let response = reqwest::Client::new()
        .get(url)
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
            // Convert the response to a general object that we can get values from using serde
            

            
        }
        _ => {
            let error: EasyBit = response.json().await?;
            log::error!("EasyBit error: {:?}", error);
            return Err(Error::ApiError(error));
        }
    }
}
