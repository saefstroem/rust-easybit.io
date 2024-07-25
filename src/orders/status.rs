use serde::Deserialize;
use serde_json::Value;

use crate::{client::Client, EasyBit, Error};

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
/**
    ### Status information.
    - `id`: Order ID
    - `status`: Possible values: "Awaiting Deposit" or "Confirming Deposit" or "Exchanging" or "Sending" or "Complete" or "Refund" or "Failed" or "Volatility Protection" or "Action Request" or "Request Overdue".
        - `Awaiting Deposit`: The order is awaiting a deposit.
        - `Confirming Deposit`: The order is confirming the deposit.
        - `Exchanging`: The order is exchanging the currency.
        - `Sending`: The order is sending the currency.
        - `Complete`: The order is complete.
        - `Refund`: The order is refunding the currency.
        - `Failed`: The order has failed.
        - `Volatility Protection`: The VPM was triggered, leading to a refund.
        - `Action Request`: The order requires KYC/AML action.
        - `Request Overdue`: The order has not been completed in time.
    - `receiveAmount`: Amount of currency received
    - `hashIn`: Hash of the transaction in
    - `hashOut`: Hash of the transaction out
    - `validationStatus`: Possible values: "null", "awaiting", "pending", "failed_allow_retry", "failed_deny_retry", "complete", "failed"
        - `null`: No validation has been requested.
        - `awaiting`: The order has Action Requests that need to be completed.
        - `pending`: The order is awaiting validation.
        - `failed_allow_retry`: The order has failed validation, but can be retried.
        - `failed_deny_retry`: The order has failed validation, because the customer is not allowed to retry. Refund within 48 hours.
        - `complete`: The order has passed validation.
        - `failed`: The order has failed validation (status after refund post failed_deny_retry).
*/
pub struct Status {
    pub id: String,
    pub status: String,
    pub receiveAmount: String,
    pub hashIn: Option<String>,
    pub hashOut: Option<String>,
    pub validationStatus: Option<String>,
    pub createdAt: i128,
    pub updatedAt: i128,
}

pub async fn order_status(client: &Client, id: String) -> Result<Status, Error> {
    // Define the path.
    let path = "/orderStatus";

    // Make the GET request and set API key. The query should only contain items that are not None.
    let response = reqwest::Client::new()
        .get(format!("{}{}", client.get_url(), path))
        .header("API-KEY", client.get_api_key())
        .query(&[("id", id)])
        .send()
        .await?;

    let json: Value = response.json().await?;
    match json.get("data") {
        Some(data) => {
            log::info!("Raw status: {:?}", data);
            let order: Status = serde_json::from_value(data.clone())?;
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
    use crate::client::Client;
    use std::env;

    #[tokio::test]
    async fn test_order_status() {
        let client = Client::new(env::var("URL").unwrap(), env::var("API_KEY").unwrap());

        // Create order
        let order = crate::orders::create::create_order(
            &client,
            crate::orders::create::Transaction {
                send: "BTC".to_string(),
                receive: "ETH".to_string(),
                amount: 0.1,
                receive_address: "0x742d35Cc6634C0532925a3b844Bc454e4438f44e".to_string(),
                extra_fee_override: None,
                vpm: None,
                refund_address: None,
                refund_tag: None,
            },
            crate::orders::create::User {
                payload: None,
                user_device_id: Some("test".to_string()),
                user_id: None,
            },
            crate::orders::create::Network {
                send_network: None,
                receive_network: None,
                receive_tag: None,
            },
        )
        .await
        .unwrap();
        let status = order_status(&client, order.id.clone()).await.unwrap();

        log::info!("{:?}", status);

        assert!(status.id == order.id);
    }
}
