use serde::Deserialize;
use serde_json::Value;

use crate::{client::Client, EasyBit, Error};

pub struct TransactionDetails {
    send: String,
    receive: String,
    amount: f64,
    receive_address: String,
    receive_network: Option<String>,
    send_network: Option<String>,
    receive_tag: Option<String>,
}

pub struct PaymentDetails {
    extra_fee_override: Option<f64>,
    vpm: Option<String>,
}

pub struct UserInformation {
    user_device_id: Option<String>,
    user_id: Option<String>,
    refund_address: Option<String>,
    refund_tag: Option<String>,
    payload: Option<String>,
}


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

pub async fn create_order(
    client: &Client,
    transaction_details: TransactionDetails,
    user_info: UserInformation,
    payment_details: PaymentDetails,
) -> Result<Order, Error> {
    // Define the URL.
    let path = "/order";

    // Make the request.
    let response = reqwest::Client::new()
        .post(format!("{}{}", client.get_url(), path))
        .header("API-KEY", client.get_api_key())
        .json(&serde_json::json!({
            "send": transaction_details.send,
            "receive": transaction_details.receive,
            "amount": transaction_details.amount,
            "receiveAddress": transaction_details.receive_address,
            "payload": user_info.payload,
            "userDeviceId": user_info.user_device_id,
            "userId": user_info.user_id,
            "sendNetwork": transaction_details.send_network,
            "receiveNetwork": transaction_details.receive_network,
            "receiveTag": transaction_details.receive_tag,
            "extraFeeOverride": payment_details.extra_fee_override,
            "vpm": payment_details.vpm,
            "refundAddress": user_info.refund_address,
            "refundTag": user_info.refund_tag,
        }))
        .send()
        .await?;
    let json: Value = response.json().await?;
    match json.get("data") {
        Some(data) => {
            log::info!("ORDER:{:?}", data);
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

    #[tokio::test]
    async fn test_place_simple_order() {
        let client = Client::new(env::var("URL").unwrap(), env::var("API_KEY").unwrap());
        let order = create_order(
            &client,
            TransactionDetails {
                send: "BTC".to_string(),
                receive: "ETH".to_string(),
                amount: 0.1,
                receive_address: "0xeB2629a2734e272Bcc07BDA959863f316F4bD4Cf".to_string(),
                receive_network: None,
                send_network: None,
                receive_tag: Some("test".to_string()),
            },
            UserInformation {
                user_device_id: None,
                user_id: None,
                refund_address: None,
                refund_tag: None,
                payload: None,
            },
            PaymentDetails {
                extra_fee_override: None,
                vpm: None,
            },
        )
        .await;

        log::info!("{:?}", order);

        assert!(order.is_ok());
    } 
}
