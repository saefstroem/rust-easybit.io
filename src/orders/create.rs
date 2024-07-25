use serde::Deserialize;
use serde_json::Value;

use crate::{client::Client, EasyBit, Error};

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
/**
   ### Order information.
   - `id`: Order ID
   - `send`: Currency code for the currency to send
   - `receive`: Currency code for the currency to receive
   - `sendNetwork`: Network code for the network to send on
   - `receiveNetwork`: Network code for the network to receive on
   - `sendAmount`: Amount of currency to send
   - `receiveAmount`: Amount of currency to receive
   - `sendAddress`: Address to send to
   - `sendTag`: Tag to send to
   - `receiveAddress`: Address to receive from
   - `receiveTag`: Tag to receive from
   - `refundAddress`: Address to refund to
   - `refundTag`: Tag to refund to
   - `vpm`: Volatility Protection Mode. "off" if not set.
   - `createdAt`: Timestamp the order was created (milliseconds)
*/
pub struct Order {
    pub id: String,
    pub send: String,
    pub receive: String,
    pub sendNetwork: String,
    pub receiveNetwork: String,
    pub sendAmount: String,
    pub receiveAmount: String,
    pub sendAddress: String,
    pub sendTag: Option<String>,
    pub receiveAddress: String,
    pub receiveTag: Option<String>,
    pub refundAddress: Option<String>,
    pub refundTag: Option<String>,
    pub vpm: String,
    pub createdAt: i128,
}

#[derive(Debug)]
/**
   ### User information.
    - `user_device_id`: Unique User device ID. Required if payload is not set.
    - `user_id`: Unique User ID from your end, if user is a guest exlude this field.
    - `payload`: Hash generated from easybit identification script. Strongly recommended to use for user identification. Potentially privacy compromising.
*/
pub struct User {
    pub user_device_id: Option<String>,
    pub user_id: Option<String>,
    pub payload: Option<String>,
}

#[derive(Debug)]
/**
   ### Network information.
    - `send_network`: Network code for the network to send on
    - `receive_network`: Network code for the network to receive on
    - `receive_tag`: Tag to receive from
*/
pub struct Network {
    pub send_network: Option<String>,
    pub receive_network: Option<String>,
    pub receive_tag: Option<String>,
}

#[derive(Debug)]
/**
   ### Transaction information.
    - `send`: Currency code for the currency to send
    - `receive`: Currency code for the currency to receive
    - `amount`: Amount of currency to send
    - `receive_address`: Address to receive from
    - `extra_fee_override`: Override the extra fee
    - `vpm`: Volatility Protection Mode. "off" if not set.
    - `refund_address`: Address to refund to
    - `refund_tag`: Tag to refund to
*/
pub struct Transaction {
    pub send: String,
    pub receive: String,
    pub amount: f64,
    pub receive_address: String,
    pub extra_fee_override: Option<f64>,
    pub vpm: Option<String>,
    pub refund_address: Option<String>,
    pub refund_tag: Option<String>,
}

pub async fn create_order(
    client: &Client,
    transaction: Transaction,
    user: User,
    network: Network,
) -> Result<Order, Error> {
    // Define the URL.
    let path = "/order";

    // Make the request.
    let response = reqwest::Client::new()
        .post(format!("{}{}", client.get_url(), path))
        .header("API-KEY", client.get_api_key())
        .json(&serde_json::json!({
            "send": transaction.send,
            "receive": transaction.receive,
            "amount": transaction.amount,
            "receiveAddress": transaction.receive_address,
            "payload": user.payload,
            "userDeviceId": user.user_device_id,
            "userId": user.user_id,
            "sendNetwork": network.send_network,
            "receiveNetwork": network.receive_network,
            "receiveTag": network.receive_tag,
            "extraFeeOverride": transaction.extra_fee_override,
            "vpm": transaction.vpm,
            "refundAddress": transaction.refund_address,
            "refundTag": transaction.refund_tag,
        }))
        .send()
        .await?;
    let json: Value = response.json().await?;
    match json.get("data") {
        Some(data) => {
            let order: Order = serde_json::from_value(data.clone())?;
            Ok(order)
        }
        None => {
            let error: EasyBit = serde_json::from_value(json)?;
            log::error!("{:?}", error);
            Err(Error::ApiError(error))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    // The order section needs more testing.
    #[tokio::test]
    async fn test_place_simple_order() {
        let client = Client::new(env::var("URL").unwrap(), env::var("API_KEY").unwrap());

        let order = create_order(
            &client,
            Transaction {
                send: "BTC".to_string(),
                receive: "ETH".to_string(),
                amount: 0.1,
                receive_address: "0xeB2629a2734e272Bcc07BDA959863f316F4bD4Cf".to_string(),
                extra_fee_override: None,
                vpm: None,
                refund_address: None,
                refund_tag: None,
            },
            User {
                user_device_id: Some("test".to_string()),
                user_id: None,
                payload: None,
            },
            Network {
                send_network: None,
                receive_network: None,
                receive_tag: None,
            },
        )
        .await;

        log::info!("{:?}", order);

        assert!(order.is_ok());
    }
}
