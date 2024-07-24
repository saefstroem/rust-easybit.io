use serde::Deserialize;
use serde_json::Value;

use crate::{client::Client, EasyBit, Error};

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
/**
   **Order information.**
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

pub async fn place_order(
    client: &Client,
    send: String,
    receive: String,
    amount: f64,
    receive_address: String,
    payload: Option<String>,
    user_device_id: Option<String>,
    user_id: Option<String>,
    send_network: Option<String>,
    receive_network: Option<String>,
    receive_tag: Option<String>,
    extra_fee_override: Option<f64>,
    vpm: Option<String>,
    refund_address: Option<String>,
    refund_tag: Option<String>,
) -> Result<Order, Error> {
    // Define the URL.
    let path = "/order";

    // Make the request.
    let response = reqwest::Client::new()
        .post(format!("{}{}", client.get_url(), path))
        .header("API-KEY", client.get_api_key())
        .json(&serde_json::json!({
            "send": send,
            "receive": receive,
            "amount": amount,
            "receiveAddress": receive_address,
            "payload": payload,
            "userDeviceId": user_device_id,
            "userId": user_id,
            "sendNetwork": send_network,
            "receiveNetwork": receive_network,
            "receiveTag": receive_tag,
            "extraFeeOverride": extra_fee_override,
            "vpm": vpm,
            "refundAddress": refund_address,
            "refundTag": refund_tag,
        }))
        .send()
        .await?;
    let json: Value = response.json().await?;
    match json.get("data") {
        Some(data) => {
            log::info!("{:?}", data);
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
    use std::env;
    use super::*;
    // The order section needs more testing.
    #[tokio::test]
    async fn test_place_simple_order() {
        let client = Client::new(env::var("URL").unwrap(), env::var("API_KEY").unwrap());
        let order = place_order(
            &client,
            "BTC".to_string(),
            "ETH".to_string(),
            0.1,
            "0xeB2629a2734e272Bcc07BDA959863f316F4bD4Cf".to_string(),
            None,
            Some("test".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await;

        log::info!("{:?}", order);

        assert!(order.is_ok());
    }
}
