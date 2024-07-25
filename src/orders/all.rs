use serde::Deserialize;
use serde_json::Value;

use crate::{client::Client, EasyBit, Error};

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
/**
   ### Order Summary
   - `id`: Order ID
   - `send`: Currency code for the currency to send
   - `receive`: Currency code for the currency to receive
   - `sendNetwork`: Network code for the network to send on
   - `receiveNetwork`: Network code for the network to receive on
   - `sendAmount`: Finalized amount of currency to send
   - `receiveAmount`: Finalized amount of currency to receive
   - `estimatedSendAmount`: Estimated amount of currency to send at the time of order creation
   - `estimatedReceiveAmount`: Estimated amount of currency to receive at the time of order creation
   - `sendAddress`: Address to send to
   - `sendTag`: Tag to send to
   - `receiveAddress`: Address to receive from
   - `receiveTag`: Tag to receive from
   - `refundAddress`: Address to refund to
   - `refundTag`: Tag to refund to
   - `vpm`: Volatility Protection Mode. "off" if not set.
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
   - `hashIn`: Hash of the transaction in
   - `hashOut`: Hash of the transaction out
   - `networkFee`: Network fee
   - `earned`: Your earnings from the order
   - `validationStatus`: Possible values: "null", "awaiting", "pending", "failed_allow_retry", "failed_deny_retry", "complete", "failed"
       - `null`: No validation has been requested.
       - `awaiting`: The order has Action Requests that need to be completed.
       - `pending`: The order is awaiting validation.
       - `failed_allow_retry`: The order has failed validation, but can be retried.
       - `failed_deny_retry`: The order has failed validation, because the customer is not allowed to retry. Refund within 48 hours.
       - `complete`: The order has passed validation.
       - `failed`: The order has failed validation (status after refund post failed_deny_retry).
   - `createdAt`: Timestamp the order was created (milliseconds)
   - `updatedAt`: Timestamp the order was last updated (milliseconds)
*/
pub struct Summary {
    pub id: String,
    pub send: String,
    pub receive: String,
    pub sendNetwork: String,
    pub receiveNetwork: String,
    pub sendAmount: String,
    pub receiveAmount: String,
    pub estimatedSendAmount: String,
    pub estimatedReceiveAmount: String,
    pub sendAddress: String,
    pub sendTag: Option<String>,
    pub receiveAddress: String,
    pub receiveTag: Option<String>,
    pub refundAddress: Option<String>,
    pub refundTag: Option<String>,
    pub vpm: String,
    pub status: String,
    pub hashIn: Option<String>,
    pub hashOut: Option<String>,
    pub networkFee: String,
    pub earned: String,
    pub validationStatus: Option<String>,
    pub createdAt: i128,
    pub updatedAt: i128,
}

pub async fn all_orders(
    client: &Client,
    id: Option<String>,
    limit: Option<String>,
    date_from: Option<String>,
    date_to: Option<String>,
    sort_direction: Option<String>,
    status: Option<String>,
) -> Result<Vec<Summary>, Error> {
    // Define the path.
    let path = "/orders";

    // Make the GET request and set API key.
    let request = reqwest::Client::new()
        .get(format!("{}{}", client.get_url(), path))
        .header("API-KEY", client.get_api_key())
        .query(&[
            ("id", id),
            ("limit", limit),
            ("dateFrom", date_from),
            ("dateTo", date_to),
            ("sortDirection", sort_direction),
            ("status", status),
        ])
        .send()
        .await?;

    let json: Value = request.json().await?;

    match json.get("data") {
        Some(data) => {
            log::info!("Raw status: {:?}", data);
            let orders: Vec<Summary> = serde_json::from_value(data.clone())?;
            Ok(orders)
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
    async fn test_all_orders() {
        let client = Client::new(env::var("URL").unwrap(), env::var("API_KEY").unwrap());
        let result = all_orders(&client, None, None, None, None, None, None).await;
        assert!(result.is_ok());
    }
}
