use zeroize::ZeroizeOnDrop;

use crate::{
    account::{get_account, set_fee, Account},
    currency::{get_currency_list, get_pair_list, get_single_currency, Currency},
    Error,
};

/**
 # Easybit.io API client.
 ### Uses reqwest for HTTP requests.
 ### Fully asynchronous.
*/
#[derive(ZeroizeOnDrop)]
pub struct Client {
    url: String,
    api_key: String,
}

impl Client {
    pub fn new(url: String, api_key: String) -> Client {
        Client { url, api_key }
    }

    pub fn get_api_key(&self) -> String {
        self.api_key.clone()
    }

    /**
    ### Retrieves account information from the API.

    **Field Descriptions**
    - `level`: Account level
    - `volume`: Total volume traded in USDT for the last month
    - `fee`: easybit.io fee
    - `extraFee`: extra fee you set
    - `totalFee`: total fee for your users
    */
    pub async fn get_account(&self) -> Result<Account, Error> {
        log::info!("Getting account info");
        get_account(self).await
    }

    /**
    ### Sets the fee for the account.

    **Parameters**
    - `fee`: Set your account API extra fee. The allowed value range is 0-0.1 and the maximum step size 0.0001. If you want for example to set an API fee of 0.4% the extraFee parameter must be 0.004.

    Does **not** return anything if successful.
    */
    pub async fn set_fee(&self, fee: f64) -> Result<(), Error> {
        log::info!("Setting fee to {}", fee);
        set_fee(self, fee).await
    }

    /**
    ### Retrieves a list of supported currencies from the API.

    **Field Descriptions**
    - `currency`: Currency code
    - `name`: Currency name
    - `sendStatusAll`: If the system can send through at least one network
    - `receiveStatusAll`: If the system can receive through at least one network
    - `networkList`: List of networks for the currency

    **Network Field Descriptions**
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
    pub async fn get_currency_list(&self) -> Result<Vec<Currency>, Error> {
        log::info!("Getting currency list");
        get_currency_list(self).await
    }

    /**
    ### Retrieves information about a single currency from the API.

    **Field Descriptions**
    - `currency`: Currency code
    - `name`: Currency name
    - `sendStatusAll`: If the system can send through at least one network
    - `receiveStatusAll`: If the system can receive through at least one network
    - `networkList`: List of networks for the currency

    **Network Field Descriptions**
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
    pub async fn get_single_currency(&self, currency: String) -> Result<Currency, Error> {
        get_single_currency(self, currency).await
    }

    /**
    ### Retrieves a list of supported currency pairs from the API.

    **Example**
    - `"BTC_BTC_ETH_ETH"`: sendCurrency_sendNetwork_receiveCurrency_receiveNetwork
    The above response is returned as an array of strings, which will require manual parsing.

    Library does not parse this response due to the risk of breaking changes if the API changes.

     */
    pub async fn get_pair_list(&self) -> Result<Vec<String>, Error> {
        get_pair_list(self).await
    }

    pub async fn get_pair_info(&self, pair: String) -> String {
        format!(
            "GET {} with token {} and pair {}",
            self.url, self.api_key, pair
        )
    }

    pub async fn get_exchange_rate(&self) -> String {
        format!("GET {} with token {}", self.url, self.api_key)
    }

    pub async fn validate_address(&self, address: String) -> String {
        format!(
            "POST {} with token {} and address {}",
            self.url, self.api_key, address
        )
    }

    pub async fn place_order(&self, pair: String, side: String, price: f64, amount: f64) -> String {
        format!(
            "POST {} with token {} and order {} {} {} {}",
            self.url, self.api_key, pair, side, price, amount
        )
    }

    pub async fn get_order_status(&self, order_id: String) -> String {
        format!(
            "GET {} with token {} and order_id {}",
            self.url, self.api_key, order_id
        )
    }

    pub async fn get_all_orders(&self) -> String {
        format!("GET {} with token {}", self.url, self.api_key)
    }

    pub async fn update_order_premium(&self, order_id: String, premium: f64) -> String {
        format!(
            "POST {} with token {} and order_id {} and premium {}",
            self.url, self.api_key, order_id, premium
        )
    }

    pub async fn pause_order_premium(&self, order_id: String) -> String {
        format!(
            "POST {} with token {} and order_id {}",
            self.url, self.api_key, order_id
        )
    }

    pub async fn resume_order_premium(&self, order_id: String) -> String {
        format!(
            "POST {} with token {} and order_id {}",
            self.url, self.api_key, order_id
        )
    }

    pub async fn refund_order_premium(&self, order_id: String) -> String {
        format!(
            "POST {} with token {} and order_id {}",
            self.url, self.api_key, order_id
        )
    }
}
