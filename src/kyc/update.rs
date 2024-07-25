use std::fmt;

use reqwest::StatusCode;
use serde::Serialize;
use serde_json::Value;

use crate::{client::Client, EasyBit, Error};
#[derive(Debug, Serialize)]
pub enum DocumentType {
    Passport,
    IdCard,
    DriverLicense,
    ResidencePermit,
}

impl fmt::Display for DocumentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DocumentType::Passport => write!(f, "PASSPORT"),
            DocumentType::IdCard => write!(f, "ID_CARD"),
            DocumentType::DriverLicense => write!(f, "DRIVERS"),
            DocumentType::ResidencePermit => write!(f, "RESIDENCE_PERMIT"),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum Side {
    Front,
    Back,
    Single,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Side::Front => write!(f, "FRONT_SIDE"),
            Side::Back => write!(f, "BACK_SIDE"),
            Side::Single => write!(f, "SINGLE_PAGE"),
        }
    }
}
/**
   **KYC Document information.**
   - `document_type`: Type of document. [Passport, IdCard, DriverLicense, ResidencePermit]
   - `side`: Side of the document. [Front, Back, Single]
   - `uri`: Data URI of the media, could be a URL or BASE64 encoded. All common image formats are acceptable
   - `selfie`: The array of objects containing user selfies and the document, both clearly visible on the same image. Data URI of the media, could be a URL or BASE64 encoded. All common image formats are acceptable
*/
#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Document {
    pub documentType: Option<DocumentType>,
    pub side: Option<Side>,
    pub uri: Option<String>,
    pub selfie: Option<Vec<String>>,
}

/**
   **KYC Validation data.**
   - `country`: Country code for the user's country. [ISO 3166-1 alpha-3 standard](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-3)
   - `documents`: List of documents for the KYC proof.
*/
#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct ValidationData {
    pub country: Option<String>,
    pub documents: Option<Vec<Document>>,
}

/**
   **KYC Proof information.**
   - `id`: Unique Order ID.
   - `user_id`: Unique User ID from your end, if user is a guest exlude this field.
   - `validation_data`: Validation data for the KYC proof.
*/
#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Proof {
    pub id: String,
    pub userId: Option<String>,
    pub validationData: Option<ValidationData>,
}

// Untested function.
#[allow(dead_code)]
pub async fn update_kyc(client: &Client, proof: Proof) -> Result<(), Error> {
    // Define the path.
    let path = "/updateOrder";

    // Make the POST request and set API key.
    let response = reqwest::Client::new()
        .post(format!("{}{}", client.get_url(), path))
        .header("API-KEY", client.get_api_key())
        .json(&proof)
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
