use reqwest::StatusCode;

use crate::{client::Client, EasyBit, Error};

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
