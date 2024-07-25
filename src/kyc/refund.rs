use reqwest::StatusCode;
use serde_json::Value;

use crate::{client::Client, EasyBit, Error};

#[allow(dead_code)]
pub async fn refund(
    client: &Client,
    order_id: String,
    refund_address: String,
    refund_tag: Option<String>,
) -> Result<(), Error> {
    // Define the path.
    let path = "/refundOrder";

    // Make the POST request and set API key.
    let response = reqwest::Client::new()
        .post(format!("{}{}", client.get_url(), path))
        .header("API-KEY", client.get_api_key())
        .json(&serde_json::json!({
            "id": order_id,
            "refundAddress": refund_address,
            "refundTag": refund_tag
        }))
        .send()
        .await?;

    let status: StatusCode = response.status();

    match status {
        StatusCode::OK => Ok(()),
        _ => {
            let json: Value = response.json().await?;
            let error: EasyBit = serde_json::from_value(json)?;
            log::error!("{:?}", error);
            Err(Error::ApiError(error))
        }
    }
}
