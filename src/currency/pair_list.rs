use reqwest::StatusCode;
use serde_json::Value;

use crate::{client::Client, EasyBit, Error};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::Client;
    use std::env;

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
}
