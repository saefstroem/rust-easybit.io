use zeroize::ZeroizeOnDrop;

use crate::{
    account::{get_account, set_fee},
    currency::{
        exchange_rate::get_exchange_rate,
        info::{get_currency_list, get_single_currency},
        pair_info::get_pair_info,
        pair_list::get_pair_list,
        validate_address::validate_address,
    },
    kyc::update::Proof,
    orders::{all::all_orders, create::create_order, status::order_status},
    Error,
};

pub use crate::account::Account;
pub use crate::currency::exchange_rate::ExchangeRate;
pub use crate::currency::info::Currency;
pub use crate::currency::pair_info::Pair;
pub use crate::orders::all::Summary;
pub use crate::orders::create::{Network, Order, Transaction, User};
pub use crate::orders::status::Status;

#[derive(ZeroizeOnDrop)]
/**
 * **Client for interacting with the easybit.io API.**
 */
pub struct Client {
    url: String,
    api_key: String,
}

impl Client {
    /**
     * Create new client with the given URL and API key.
     */
    pub fn new(url: String, api_key: String) -> Client {
        Client { url, api_key }
    }

    /**
     * Get the API key.
     */
    pub fn get_api_key(&self) -> String {
        self.api_key.clone()
    }

    /**
     * Get the URL.
     */
    pub fn get_url(&self) -> String {
        self.url.clone()
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

    /**
    ### Retrieves information about a single currency pair from the API.

    **Parameters**
    - `send`: Currency code for the currency to send
    - `receive`: Currency code for the currency to receive
    - `send_network`: Optional network code for the network to send on
    - `receive_network`: Optional network code for the network to receive on
    - `amount_type`: Optional amount type for if you want the amount parameter to be the amount of currency to receive. Set this to "receive" for this behavior.
    */
    pub async fn get_pair_info(
        &self,
        send: String,
        receive: String,
        send_network: Option<String>,
        receive_network: Option<String>,
        amount_type: Option<String>,
    ) -> Result<Pair, Error> {
        get_pair_info(
            self,
            send,
            receive,
            send_network,
            receive_network,
            amount_type,
        )
        .await
    }

    /**
    ### Retrieves the exchange rate for a currency pair from the API.

    **Parameters**
    - `send`: Currency code for the currency to send
    - `receive`: Currency code for the currency to receive
    - `amount`: Amount of currency to send
    - `send_network`: Optional network code for the network to send on
    - `receive_network`: Optional network code for the network to receive on
    - `amount_type`: Optional amount type for if you want the amount parameter to be the amount of currency to receive. Set this to "receive" for this behavior.
    - `extra_fee_override`: Optional extra fee override for the exchange rate, useful for discounts or promotions.
    */
    #[allow(clippy::too_many_arguments)]
    pub async fn get_exchange_rate(
        &self,
        send: String,
        receive: String,
        amount: f64,
        send_network: Option<String>,
        receive_network: Option<String>,
        amount_type: Option<String>,
        extra_fee_override: Option<f64>,
    ) -> Result<ExchangeRate, Error> {
        get_exchange_rate(
            self,
            send,
            receive,
            amount,
            send_network,
            receive_network,
            amount_type,
            extra_fee_override,
        )
        .await
    }

    /**
    ### Validates an address for a currency from the API.

    **Parameters**
    - `currency`: Currency code for the currency to validate
    - `address`: Address to validate
    - `network`: Optional network code for the network to validate on
    - `tag`: Optional tag for the address
     */
    pub async fn validate_address(
        &self,
        currency: String,
        address: String,
        network: Option<String>,
        tag: Option<String>,
    ) -> Result<(), Error> {
        validate_address(self, currency, address, network, tag).await
    }

    /**
    ### Places an order with the API.

    **Parameters**
    - `transaction`: Transaction information
    - `user`: User information
    - `network`: Network information
    */
    pub async fn place_order(
        &self,
        transaction: Transaction,
        user: User,
        network: Network,
    ) -> Result<Order, Error> {
        create_order(self, transaction, user, network).await
    }

    /**
    ### Retrieves the status of an order from the API.

    **Parameters**
    - `order_id`: Unique Order ID
     */
    pub async fn get_order_status(&self, order_id: String) -> Result<Status, Error> {
        order_status(self, order_id).await
    }

    /**
    ### Retrieves all orders from the API.

    **Parameters**
    - `id`: Optional Order ID
    - `limit`: Optional limit for the number of orders to return
    - `date_from`: Optional date to start from
    - `date_to`: Optional date to end at
    - `sort_direction`: Optional sort direction DESC or ASC
    - `status`: Optional status to filter by "Awaiting Deposit" or "Confirming Deposit" or "Exchanging" or "Sending" or "Complete" or "Refund" or "Failed" or "Volatility Protection" or "Action Request" or "Request Overdue"
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
     */
    pub async fn get_all_orders(
        &self,
        id: Option<String>,
        limit: Option<String>,
        date_from: Option<String>,
        date_to: Option<String>,
        sort_direction: Option<String>,
        status: Option<String>,
    ) -> Result<Vec<Summary>, Error> {
        all_orders(self, id, limit, date_from, date_to, sort_direction, status).await
    }

    /**
    ### Updates the KYC information for an order that requires KYC validation.
    *This function is not available at the moment due to lack of testing possibilities.*

    **Note: If a customer does not want to provide KYC information, you can refund the order.**

    **Parameters**
    - `proof`: KYC proof information
     */
    pub async fn update_order_kyc(&self, _proof: Proof) {
        todo!("Limited ways to test current implementation. Wait for future updates.");
        // update_kyc(self, proof).await;
    }

    /**
    ### Refunds an order that requires KYC validation.
    *This function is not available at the moment due to lack of testing possibilities.*

    **Parameters**
    - `order_id`: Unique Order ID
    - `refund_address`: Address to refund to
    - `refund_tag`: Optional tag to refund to

    ### To be able to refund the order the following conditions should be met:

    1. The order "status" is "Action Request".
    2. The order "validationStatus" has any of the following values: null, "awaiting", "failed_allow_retry", "failed_deny_retry"

     */
    pub async fn refund_order(
        &self,
        _order_id: String,
        _refund_address: String,
        _refund_tag: Option<String>,
    ) {
        todo!("Limited ways to test current implementation. Wait for future updates.");
        // refund(self, order_id, refund_address, refund_tag).await;
    }
}
