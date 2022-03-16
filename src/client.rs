use super::auth::BitsoCredentials;
use super::model::private::*;
use super::model::public::*;
use super::model::JSONResponse;
use anyhow::Result;
use hex::encode;
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::sign::Signer;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use reqwest::Method;
use reqwest::StatusCode;
use serde::de::Deserialize;
use serde_json::map::Map;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::time::{SystemTime, UNIX_EPOCH};

lazy_static! {
    // HTTP Client
    static ref CLIENT: Client = Client::new();
}

const EMPTY_CREDENTIALS_MSG: &str = "You need to set your Bitso API \
                                     credentials. You can do this \
                                     by setting environment variables \
                                     in a `.env` file: \
                                     API_KEY=your api_key \
                                     API_SECRET=your_api_secret. \
                                     For more information visit: \
                                     `https://bitso.com/api_info#generating-api-keys`";

fn convert_map_to_string<
    K: Debug + Eq + Hash + ToString,
    V: Debug + ToString,
    S: ::std::hash::BuildHasher,
>(
    map: &HashMap<K, V, S>,
) -> String {
    let mut string: String = String::new();
    for (key, value) in map.iter() {
        string.push_str(&key.to_string());
        string.push('=');
        string.push_str(&value.to_string());
        string.push('&');
    }
    string
}

/// API Type that indicates whether a method
/// corresponds to the public or private API.
pub enum ApiType {
    Public,
    Private,
}

/// API Errors associated to the Bitso API object
#[derive(Debug, Deserialize)]
pub enum ApiError {
    /// A regular error is derived from
    /// Bitso's API responses. For more information
    /// see: <https://bitso.com/api_info#error-codes>
    #[serde(alias = "error")]
    RegularError {
        success: bool,
        code: String,
        message: String,
    },
    Other(u16),
}

/// Generic optional parameters for methods
pub struct OptionalParams<'a> {
    pub marker: Option<&'a u32>,
    pub sort: Option<&'a str>,
    pub limit: Option<&'a u8>,
}

/// Optional parameters for an order.
///
/// For more info see: <https://bitso.com/api_info#place-an-order>
pub struct OptionalOrderParams<'a> {
    pub major: Option<&'a str>,
    pub minor: Option<&'a str>,
    pub price: Option<&'a str>,
    pub stop: Option<&'a str>,
    pub time_in_force: Option<&'a str>,
    pub origin_id: Option<&'a str>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::RegularError {
                success: _,
                code,
                message,
            } => write!(f, "Bitso API error code {}: {}", code, message),
            ApiError::Other(s) => write!(f, "Bitso API reported error code {}", s),
        }
    }
}

/// A regular error from Bitso's API. It holds [ErrorDetails]
///
/// See: <https://bitso.com/api_info#error-codes>
#[derive(Debug, Deserialize)]
pub struct RegularError {
    pub success: bool,
    pub error: ErrorDetails,
}

/// Details for the API calls.
///
/// See: <https://bitso.com/api_info#error-codes>
#[derive(Debug, Deserialize)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,
}

impl ApiError {
    async fn from_response(response: reqwest::Response) -> Self {
        match response.status() {
            StatusCode::BAD_REQUEST => {
                let error = response.json::<RegularError>().await.unwrap();
                ApiError::RegularError {
                    success: error.success,
                    code: error.error.code,
                    message: error.error.message,
                }
            }
            status => ApiError::Other(status.as_u16()),
        }
    }
}

/// Bitso API object
pub struct Bitso {
    pub prefix: String,
    pub client_credentials_manager: Option<BitsoCredentials>,
}

impl Bitso {
    /// Bitso instance
    pub fn default() -> Bitso {
        Bitso {
            prefix: "https://api.bitso.com".to_owned(),
            client_credentials_manager: None,
        }
    }

    /// Set prefix
    pub fn prefix(mut self, prefix: &str) -> Bitso {
        self.prefix = prefix.to_owned();
        self
    }

    /// Set client credentials
    pub fn client_credentials_manager(
        mut self,
        client_credential_manager: BitsoCredentials,
    ) -> Bitso {
        self.client_credentials_manager = Some(client_credential_manager);
        self
    }

    /// Build Bitso API object
    pub fn build(self) -> Bitso {
        self
    }

    /// Construct authorization headers.
    /// See: <https://bitso.com/api_info#creating-and-signing-requests>
    pub fn auth_headers(
        &self,
        method: &Method,
        request_path: &str,
        payload: Option<&Value>,
    ) -> String {
        let payload_string: String;
        if method != Method::POST {
            payload_string = "".to_owned();
        } else if let Some(json) = payload {
            payload_string = json.to_string();
        } else {
            panic!("POST method must have a payload.")
        }

        let api_key = self.client_credentials_manager.as_ref().unwrap().get_key();
        let api_secret = self
            .client_credentials_manager
            .as_ref()
            .unwrap()
            .get_secret();

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();
        let message = format!(
            "{}{}{}{}",
            nonce,
            method.as_str().to_owned(),
            request_path.to_owned(),
            payload_string
        );
        let key = PKey::hmac(api_secret.as_bytes()).unwrap();
        let mut signer = Signer::new(MessageDigest::sha256(), &key).unwrap();
        signer.update(message.as_bytes()).unwrap();
        let signature = encode(signer.sign_to_vec().unwrap());
        format!("Bitso {}:{}:{}", api_key, nonce, signature,)
    }

    async fn internal_call(
        &self,
        method: Method,
        url: &str,
        payload: Option<&Value>,
        api_type: ApiType,
    ) -> Result<String> {
        let mut url: Cow<str> = url.into();

        let mut headers = HeaderMap::new();
        if let ApiType::Private = api_type {
            headers.insert(
                AUTHORIZATION,
                self.auth_headers(&method, &url, payload).parse().unwrap(),
            );
            headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        }

        if !url.starts_with("http") {
            url = [self.prefix.as_str(), &url].concat().into();
        }

        let response = {
            let mut builder = CLIENT.request(method, &url.into_owned());
            if let ApiType::Private = api_type {
                builder = builder.headers(headers);
            }
            if let Some(json) = payload {
                builder = builder.json(json);
            };
            builder.send().await?
        };

        if response.status().is_success() {
            match response.text().await {
                Ok(text) => Ok(text),
                Err(e) => Err(anyhow!("Error getting text out of response {}", e)),
            }
        } else {
            Err(anyhow!(ApiError::from_response(response).await))
        }
    }

    /// Makes get requests
    async fn get(
        &self,
        url: &str,
        params: &mut HashMap<String, String>,
        api_type: ApiType,
    ) -> Result<String> {
        if !params.is_empty() {
            let param: String = convert_map_to_string(params);
            let mut url_with_params = url.to_owned();
            url_with_params.push('?');
            url_with_params.push_str(&param);
            self.internal_call(Method::GET, &url_with_params, None, api_type)
                .await
        } else {
            self.internal_call(Method::GET, url, None, api_type).await
        }
    }

    /// Makes post requests
    async fn post(&self, url: &str, payload: &Value, api_type: ApiType) -> Result<String> {
        self.internal_call(Method::POST, url, Some(payload), api_type)
            .await
    }

    /// Makes delete requests
    async fn delete(
        &self,
        url: &str,
        params: &mut HashMap<String, String>,
        api_type: ApiType,
    ) -> Result<String> {
        if !params.is_empty() {
            let param: String = convert_map_to_string(params);
            let mut url_with_params = url.to_owned();
            url_with_params.push('?');
            url_with_params.push_str(&param);
            self.internal_call(Method::DELETE, &url_with_params, None, api_type)
                .await
        } else {
            self.internal_call(Method::DELETE, url, None, api_type)
                .await
        }
    }

    /// Function to convert result to models
    pub fn convert_result<'a, T: Deserialize<'a>>(&self, input: &'a str) -> Result<T> {
        let result = serde_json::from_str::<T>(input).map_err(|e| {
            format_err!(
                "Convert result failed, reason: {:?}; content: [{:?}]",
                e,
                input
            )
        })?;
        Ok(result)
    }

    /// Make a request to get available books
    /// See: <https://bitso.com/api_info/#available-books>
    pub async fn get_available_books(&self) -> Result<JSONResponse<Vec<AvailableBook>>> {
        let url = String::from("/v3/available_books/");
        let result = self.get(&url, &mut HashMap::new(), ApiType::Public).await?;
        self.convert_result::<JSONResponse<Vec<AvailableBook>>>(&result)
    }

    /// Make a request to get ticker
    /// See: <https://bitso.com/api_info/#ticker>
    pub async fn get_ticker(&self, book: &str) -> Result<JSONResponse<BookTicker>> {
        let mut params = HashMap::new();
        params.insert("book".to_owned(), book.to_string());
        let url = String::from("/v3/ticker/");
        let result = self.get(&url, &mut params, ApiType::Public).await?;
        self.convert_result::<JSONResponse<BookTicker>>(&result)
    }

    /// Make a request to get a specific order book
    /// See: <https://bitso.com/api_info/#order-book>
    pub async fn get_order_book(
        &self,
        book: &str,
        aggregate: bool,
    ) -> Result<JSONResponse<OrderBookPayload>> {
        let mut params = HashMap::new();
        params.insert("book".to_owned(), book.to_string());
        params.insert("aggregate".to_owned(), aggregate.to_string());
        let url = String::from("/v3/order_book/");
        let result = self.get(&url, &mut params, ApiType::Public).await?;
        self.convert_result::<JSONResponse<OrderBookPayload>>(&result)
    }

    /// Make a request to get a specific trade
    /// See: <https://bitso.com/api_info/#trades>
    pub async fn get_trades(
        &self,
        book: &str,
        optional_params: Option<OptionalParams<'_>>,
    ) -> Result<JSONResponse<Vec<Trade>>> {
        let mut params = HashMap::new();
        params.insert("book".to_owned(), book.to_string());
        if let Some(op) = optional_params {
            if let Some(m) = op.marker {
                params.insert("marker".to_owned(), m.to_string());
            }
            if let Some(s) = op.sort {
                params.insert("sort".to_owned(), s.to_string());
            }
            if let Some(l) = op.limit {
                params.insert("limit".to_owned(), l.to_string());
            }
        }
        let url = String::from("/v3/trades/");
        let result = self.get(&url, &mut params, ApiType::Public).await?;
        self.convert_result::<JSONResponse<Vec<Trade>>>(&result)
    }

    /// Make a request to get account status
    /// See: <https://bitso.com/api_info#account-status>
    pub async fn get_account_status(&self) -> Result<JSONResponse<AccountStatusPayload>> {
        let url = String::from("/v3/account_status/");
        let client_credentials = self.client_credentials_manager.as_ref();
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(anyhow!(EMPTY_CREDENTIALS_MSG));
                }
            }
            None => return Err(anyhow!(EMPTY_CREDENTIALS_MSG)),
        }
        let result = self
            .get(&url, &mut HashMap::new(), ApiType::Private)
            .await?;
        self.convert_result::<JSONResponse<AccountStatusPayload>>(&result)
    }

    /// Make a request to get account balance
    /// See: <https://bitso.com/api_info#account-balance>
    pub async fn get_account_balance(&self) -> Result<JSONResponse<Balances>> {
        let url = String::from("/v3/balance/");
        let client_credentials = self.client_credentials_manager.as_ref();
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(anyhow!(EMPTY_CREDENTIALS_MSG));
                }
            }
            None => return Err(anyhow!(EMPTY_CREDENTIALS_MSG)),
        }
        let result = self
            .get(&url, &mut HashMap::new(), ApiType::Private)
            .await?;
        self.convert_result::<JSONResponse<Balances>>(&result)
    }

    /// Make a request to get fees
    /// See: <https://bitso.com/api_info#fees>
    pub async fn get_fees(&self) -> Result<JSONResponse<FeesPayload>> {
        let url = String::from("/v3/fees/");
        let client_credentials = self.client_credentials_manager.as_ref();
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(anyhow!(EMPTY_CREDENTIALS_MSG));
                }
            }
            None => return Err(anyhow!(EMPTY_CREDENTIALS_MSG)),
        }
        let result = self
            .get(&url, &mut HashMap::new(), ApiType::Private)
            .await?;
        self.convert_result::<JSONResponse<FeesPayload>>(&result)
    }

    /// Make a request to get ledger
    /// See: <https://bitso.com/api_info#ledger>
    pub async fn get_ledger<'a>(
        &self,
        operation_type: Option<&str>,
        optional_params: Option<OptionalParams<'_>>,
    ) -> Result<JSONResponse<Vec<LedgerInstance>>> {
        let mut url = String::from("/v3/ledger/");
        let mut params = HashMap::new();
        if let Some(o_t) = operation_type {
            url.push_str(o_t);
            url.push('/');
        }
        if let Some(op) = optional_params {
            if let Some(m) = op.marker {
                params.insert("marker".to_owned(), m.to_string());
            }
            if let Some(s) = op.sort {
                params.insert("sort".to_owned(), s.to_string());
            }
            if let Some(l) = op.limit {
                params.insert("limit".to_owned(), l.to_string());
            }
        }
        let client_credentials = self.client_credentials_manager.as_ref();
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(anyhow!(EMPTY_CREDENTIALS_MSG));
                }
            }
            None => return Err(anyhow!(EMPTY_CREDENTIALS_MSG)),
        }
        let result = self.get(&url, &mut params, ApiType::Private).await?;
        self.convert_result::<JSONResponse<Vec<LedgerInstance>>>(&result)
    }

    /// Make a request to get withdrawals
    /// See: <https://bitso.com/api_info#withdrawals>
    pub async fn get_withdrawals<'a>(
        &self,
        wid: Option<&str>,
        wids: Option<Vec<&str>>,
        origin_ids: Option<Vec<&str>>,
        optional_params: Option<OptionalParams<'_>>,
        method: Option<&str>,
    ) -> Result<JSONResponse<Vec<WithdrawalsPayload>>> {
        let mut url = String::from("/v3/withdrawals/");
        let mut params = HashMap::new();
        let client_credentials = self.client_credentials_manager.as_ref();
        if let Some(w) = wid {
            url.push_str(w);
            url.push('/');
        } else if let Some(ws) = wids {
            let joined_wids = ws.join(",");
            params.insert("wids".to_owned(), joined_wids);
        } else if let Some(oids) = origin_ids {
            let joined_origin_ids = oids.join(",");
            params.insert("origin_ids".to_owned(), joined_origin_ids);
        }

        // Add generic optional parameters
        if let Some(op) = optional_params {
            if let Some(m) = op.marker {
                params.insert("marker".to_owned(), m.to_string());
            }
            if let Some(s) = op.sort {
                params.insert("sort".to_owned(), s.to_string());
            }
            if let Some(l) = op.limit {
                params.insert("limit".to_owned(), l.to_string());
            }
        }
        if let Some(m) = method {
            params.insert("method".to_owned(), m.to_string());
        }
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(anyhow!(EMPTY_CREDENTIALS_MSG));
                }
            }
            None => return Err(anyhow!(EMPTY_CREDENTIALS_MSG)),
        }
        let result = self.get(&url, &mut params, ApiType::Private).await?;
        self.convert_result::<JSONResponse<Vec<WithdrawalsPayload>>>(&result)
    }

    /// Make a request to get fundings
    /// See: <https://bitso.com/api_info#fundings>
    pub async fn get_fundings<'a>(
        &self,
        fid: Option<&str>,
        fids: Option<Vec<&str>>,
        optional_params: Option<OptionalParams<'_>>,
        txids: Option<Vec<&str>>,
        method: Option<&str>,
    ) -> Result<JSONResponse<Vec<FundingsPayload>>> {
        let mut url = String::from("/v3/fundings/");
        let mut params = HashMap::new();
        let client_credentials = self.client_credentials_manager.as_ref();
        if let Some(f) = fid {
            url.push_str(f);
            url.push('/');
        } else if let Some(fs) = fids {
            let joined_fids = fs.join("-");
            url.push_str(&joined_fids[..]);
            url.push('/');
        }

        // Add generic optional parameters
        if let Some(op) = optional_params {
            if let Some(m) = op.marker {
                params.insert("marker".to_owned(), m.to_string());
            }
            if let Some(s) = op.sort {
                params.insert("sort".to_owned(), s.to_string());
            }
            if let Some(l) = op.limit {
                params.insert("limit".to_owned(), l.to_string());
            }
        }
        if let Some(m) = method {
            params.insert("method".to_owned(), m.to_string());
        }
        if let Some(ts) = txids {
            let joined_ts = ts.join(",");
            params.insert("txids".to_owned(), joined_ts);
        }
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(anyhow!(EMPTY_CREDENTIALS_MSG));
                }
            }
            None => return Err(anyhow!(EMPTY_CREDENTIALS_MSG)),
        }
        let result = self.get(&url, &mut params, ApiType::Private).await?;
        self.convert_result::<JSONResponse<Vec<FundingsPayload>>>(&result)
    }

    /// Make a request to get user trades
    /// See: <https://bitso.com/api_info#user-trades>
    pub async fn get_user_trades(
        &self,
        book: Option<&str>,
        tid: Option<&str>,
        tids: Option<Vec<&str>>,
        optional_params: Option<OptionalParams<'_>>,
    ) -> Result<JSONResponse<Vec<UserTradesPayload>>> {
        let mut url = String::from("/v3/user_trades/");
        let mut params = HashMap::new();
        let client_credentials = self.client_credentials_manager.as_ref();
        // params.insert("book".to_owned(), book.to_string());
        if let Some(b) = book {
            params.insert("book".to_owned(), b.to_string());
        }
        if let Some(t) = tid {
            url.push_str(t);
            url.push('/');
        } else if let Some(ts) = tids {
            let joined_tids = ts.join("-");
            url.push_str(&joined_tids[..]);
            url.push('/');
        }

        // Add generic optional parameters
        if let Some(op) = optional_params {
            if let Some(m) = op.marker {
                params.insert("marker".to_owned(), m.to_string());
            }
            if let Some(s) = op.sort {
                params.insert("sort".to_owned(), s.to_string());
            }
            if let Some(l) = op.limit {
                params.insert("limit".to_owned(), l.to_string());
            }
        }
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(anyhow!(EMPTY_CREDENTIALS_MSG));
                }
            }
            None => return Err(anyhow!(EMPTY_CREDENTIALS_MSG)),
        }
        let result = self.get(&url, &mut params, ApiType::Private).await?;
        self.convert_result::<JSONResponse<Vec<UserTradesPayload>>>(&result)
    }

    /// Make a request to get order trades
    /// See: <https://bitso.com/api_info#order-trades>
    pub async fn get_order_trades(
        &self,
        oid: Option<&str>,
        origin_id: Option<&str>,
    ) -> Result<JSONResponse<Vec<OrderTradesPayload>>> {
        let mut url = String::from("/v3/order_trades");
        let mut params = HashMap::new();
        if let Some(o) = oid {
            url.push('/');
            url.push_str(o);
            url.push('/');
        }
        if let Some(or) = origin_id {
            params.insert("origin_id".to_owned(), or.to_string());
        }
        let client_credentials = self.client_credentials_manager.as_ref();
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(anyhow!(EMPTY_CREDENTIALS_MSG));
                }
            }
            None => return Err(anyhow!(EMPTY_CREDENTIALS_MSG)),
        }
        let result = self.get(&url, &mut params, ApiType::Private).await?;
        self.convert_result::<JSONResponse<Vec<OrderTradesPayload>>>(&result)
    }

    /// Make a request to get open orders
    /// See: <https://bitso.com/api_info#open-orders>
    pub async fn get_open_orders<'a>(
        &self,
        book: Option<&str>,
        optional_params: Option<OptionalParams<'_>>,
    ) -> Result<JSONResponse<Vec<OpenOrdersPayload>>> {
        let url = String::from("/v3/open_orders");
        let mut params = HashMap::new();
        let client_credentials = self.client_credentials_manager.as_ref();
        if let Some(b) = book {
            params.insert("book".to_owned(), b.to_string());
        }
        // Add generic optional parameters
        if let Some(op) = optional_params {
            if let Some(m) = op.marker {
                params.insert("marker".to_owned(), m.to_string());
            }
            if let Some(s) = op.sort {
                params.insert("sort".to_owned(), s.to_string());
            }
            if let Some(l) = op.limit {
                params.insert("limit".to_owned(), l.to_string());
            }
        }
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(anyhow!(EMPTY_CREDENTIALS_MSG));
                }
            }
            None => return Err(anyhow!(EMPTY_CREDENTIALS_MSG)),
        }
        let result = self.get(&url, &mut params, ApiType::Private).await?;
        self.convert_result::<JSONResponse<Vec<OpenOrdersPayload>>>(&result)
    }

    /// Make a request to get lookup orders
    /// See: <https://bitso.com/api_info#lookup-orders>
    pub async fn get_lookup_orders(
        &self,
        oid: Option<&str>,
        oids: Option<Vec<&str>>,
        origin_ids: Option<Vec<&str>>,
    ) -> Result<JSONResponse<Vec<LookupOrdersPayload>>> {
        let mut url = String::from("/v3/orders/");
        let mut params = HashMap::new();
        let client_credentials = self.client_credentials_manager.as_ref();
        if let Some(o) = oid {
            url.push_str(o);
            url.push('/');
        } else if let Some(os) = oids {
            let joined_oids = os.join(",");
            params.insert("oids".to_owned(), joined_oids);
        } else if let Some(oids) = origin_ids {
            let joined_origin_ids = oids.join(",");
            params.insert("origin_ids".to_owned(), joined_origin_ids);
        }
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(anyhow!(EMPTY_CREDENTIALS_MSG));
                }
            }
            None => return Err(anyhow!(EMPTY_CREDENTIALS_MSG)),
        }
        let result = self.get(&url, &mut params, ApiType::Private).await?;
        self.convert_result::<JSONResponse<Vec<LookupOrdersPayload>>>(&result)
    }

    /// Make a request to cancel order
    /// See: <https://bitso.com/api_info#cancel-order>
    pub async fn cancel_order(
        &self,
        all: bool,
        oid: Option<&str>,
        oids: Option<Vec<&str>>,
        origin_ids: Option<Vec<&str>>,
    ) -> Result<JSONResponse<Vec<String>>> {
        let mut url = String::from("/v3/orders/");
        let mut params = HashMap::new();
        let client_credentials = self.client_credentials_manager.as_ref();
        if all {
            url.push_str("all");
        } else if let Some(o) = oid {
            url.push_str(o);
            url.push('/');
        } else if let Some(os) = oids {
            let joined_oids = os.join(",");
            params.insert("oids".to_owned(), joined_oids);
        } else if let Some(oids) = origin_ids {
            let joined_origin_ids = oids.join(",");
            params.insert("origin_ids".to_owned(), joined_origin_ids);
        }
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(anyhow!(EMPTY_CREDENTIALS_MSG));
                }
            }
            None => return Err(anyhow!(EMPTY_CREDENTIALS_MSG)),
        }
        let result = self
            .delete(&url, &mut HashMap::new(), ApiType::Private)
            .await?;
        self.convert_result::<JSONResponse<Vec<String>>>(&result)
    }

    /// Make a post request to place an order
    /// See: <https://bitso.com/api_info#place-an-order>
    pub async fn place_order<'a>(
        &self,
        book: &str,
        side: &str,
        r#type: &str,
        optional_order_params: Option<OptionalOrderParams<'_>>,
    ) -> Result<JSONResponse<PlaceOrderPayload>> {
        let url = String::from("/v3/orders/");
        // Create map for parameters
        let mut params_map = Map::new();
        params_map.insert("book".to_owned(), Value::String(book.to_owned()));
        params_map.insert("side".to_owned(), Value::String(side.to_owned()));
        params_map.insert("type".to_owned(), Value::String(r#type.to_owned()));
        // Add optional params
        if let Some(op) = optional_order_params {
            if let Some(ma) = op.major {
                params_map.insert("major".to_owned(), Value::String(ma.to_owned()));
            }
            if let Some(mi) = op.minor {
                params_map.insert("minor".to_owned(), Value::String(mi.to_owned()));
            }
            if let Some(p) = op.price {
                params_map.insert("price".to_owned(), Value::String(p.to_owned()));
            }
            if let Some(s) = op.stop {
                params_map.insert("stop".to_owned(), Value::String(s.to_owned()));
            }
            if let Some(tif) = op.time_in_force {
                params_map.insert("time_in_force".to_owned(), Value::String(tif.to_owned()));
            }
            if let Some(oi) = op.origin_id {
                params_map.insert("origin_id".to_owned(), Value::String(oi.to_owned()));
            }
        }
        let params = json!(params_map);
        let client_credentials = self.client_credentials_manager.as_ref();
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(anyhow!(EMPTY_CREDENTIALS_MSG));
                }
            }
            None => return Err(anyhow!(EMPTY_CREDENTIALS_MSG)),
        }
        let result = self.post(&url, &params, ApiType::Private).await?;
        self.convert_result::<JSONResponse<PlaceOrderPayload>>(&result)
    }

    /// Make a request to get lookup orders
    /// See: <https://bitso.com/api_info#lookup-orders>
    pub async fn get_funding_destination(
        &self,
        fund_currency: &str,
    ) -> Result<JSONResponse<FundingDestination>> {
        let url = String::from("/v3/funding_destination/");
        let mut params = HashMap::new();
        params.insert("fund_currency".to_owned(), fund_currency.to_string());
        let client_credentials = self.client_credentials_manager.as_ref();
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(anyhow!(EMPTY_CREDENTIALS_MSG));
                }
            }
            None => return Err(anyhow!(EMPTY_CREDENTIALS_MSG)),
        }
        let result = self.get(&url, &mut params, ApiType::Private).await?;
        self.convert_result::<JSONResponse<FundingDestination>>(&result)
    }

    /// Make a request to place an crypto withdrawal
    /// See: <https://bitso.com/api_info#crypto-withdrawals>
    pub async fn crypto_withdrawal(
        &self,
        currency: &str,
        amount: &str,
        address: &str,
        max_fee: Option<&str>,
        destination_tag: Option<&str>,
    ) -> Result<JSONResponse<Withdrawal<CryptoWithdrawal>>> {
        let url = String::from("/v3/crypto_withdrawal/");
        let mut params_map = Map::new();
        params_map.insert("currency".to_owned(), Value::String(currency.to_owned()));
        params_map.insert("amount".to_owned(), Value::String(amount.to_owned()));
        params_map.insert("address".to_owned(), Value::String(address.to_owned()));
        // Add optional params
        if let Some(mf) = max_fee {
            params_map.insert("max_fee".to_owned(), Value::String(mf.to_owned()));
        }
        if let Some(dt) = destination_tag {
            params_map.insert("destination_tag".to_owned(), Value::String(dt.to_owned()));
        }
        let params = json!(params_map);
        let client_credentials = self.client_credentials_manager.as_ref();
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(anyhow!(EMPTY_CREDENTIALS_MSG));
                }
            }
            None => return Err(anyhow!(EMPTY_CREDENTIALS_MSG)),
        }
        let result = self.post(&url, &params, ApiType::Private).await?;
        self.convert_result::<JSONResponse<Withdrawal<CryptoWithdrawal>>>(&result)
    }

    /// Make a request to place a speri withdrawal
    /// See: <https://bitso.com/api_info#spei-withdrawal>
    pub async fn spei_withdrawal(
        &self,
        amount: &str,
        recipient_given_names: &str,
        recipient_family_names: &str,
        clabe: &str,
        notes_ref: Option<&str>,
        numeric_ref: Option<&str>,
    ) -> Result<JSONResponse<Withdrawal<SPEIWithdrawal>>> {
        let url = String::from("/v3/spei_withdrawal/");
        let mut params_map = Map::new();
        params_map.insert("amount".to_owned(), Value::String(amount.to_owned()));
        params_map.insert(
            "recipient_given_names".to_owned(),
            Value::String(recipient_given_names.to_owned()),
        );
        params_map.insert(
            "recipient_family_names".to_owned(),
            Value::String(recipient_family_names.to_owned()),
        );
        params_map.insert("clabe".to_owned(), Value::String(clabe.to_owned()));
        // Add optional params
        if let Some(nor) = notes_ref {
            params_map.insert("notes_ref".to_owned(), Value::String(nor.to_owned()));
        }
        if let Some(nur) = numeric_ref {
            params_map.insert("numeric_ref".to_owned(), Value::String(nur.to_owned()));
        }
        let params = json!(params_map);
        let client_credentials = self.client_credentials_manager.as_ref();
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(anyhow!(EMPTY_CREDENTIALS_MSG));
                }
            }
            None => return Err(anyhow!(EMPTY_CREDENTIALS_MSG)),
        }
        let result = self.post(&url, &params, ApiType::Private).await?;
        self.convert_result::<JSONResponse<Withdrawal<SPEIWithdrawal>>>(&result)
    }

    /// Make a request to get bank codes
    /// See: <https://bitso.com/api_info#bank-codes>
    pub async fn get_bank_codes(&self) -> Result<JSONResponse<Vec<BankCode>>> {
        let url = String::from("/v3/mx_bank_codes/");
        let client_credentials = self.client_credentials_manager.as_ref();
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(anyhow!(EMPTY_CREDENTIALS_MSG));
                }
            }
            None => return Err(anyhow!(EMPTY_CREDENTIALS_MSG)),
        }
        let result = self
            .get(&url, &mut HashMap::new(), ApiType::Private)
            .await?;
        self.convert_result::<JSONResponse<Vec<BankCode>>>(&result)
    }

    /// Make a post request to make a debit-card ithdrawal
    /// See: <https://bitso.com/api_info#debit-card-withdrawal>
    pub async fn debit_card_withdrawal(
        &self,
        amount: &str,
        recipient_given_names: &str,
        recipient_family_names: &str,
        card_number: &str,
        bank_code: &str,
    ) -> Result<JSONResponse<Withdrawal<DebitWithdrawal>>> {
        let url = String::from("/v3/debit_card_withdrawal/");
        let params = json!({
            "amount": amount,
            "recipient_given_names": recipient_given_names,
            "recipient_family_names": recipient_family_names,
            "card_number": card_number,
            "bank_code": bank_code
        });
        let client_credentials = self.client_credentials_manager.as_ref();
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(anyhow!(EMPTY_CREDENTIALS_MSG));
                }
            }
            None => return Err(anyhow!(EMPTY_CREDENTIALS_MSG)),
        }
        let result = self.post(&url, &params, ApiType::Private).await?;
        self.convert_result::<JSONResponse<Withdrawal<DebitWithdrawal>>>(&result)
    }

    /// Make a post request to make a phone-number withdrawal
    /// See: <https://bitso.com/api_info#phone-number-withdrawal>
    pub async fn phone_number_withdrawal(
        &self,
        amount: &str,
        recipient_given_names: &str,
        recipient_family_names: &str,
        phone_number: &str,
        bank_code: &str,
    ) -> Result<JSONResponse<Withdrawal<PhoneWithdrawal>>> {
        let url = String::from("/v3/phone_withdrawal/");
        let params = json!({
            "amount": amount,
            "recipient_given_names": recipient_given_names,
            "recipient_family_names": recipient_family_names,
            "phone_number": phone_number,
            "bank_code": bank_code
        });
        let client_credentials = self.client_credentials_manager.as_ref();
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(anyhow!(EMPTY_CREDENTIALS_MSG));
                }
            }
            None => return Err(anyhow!(EMPTY_CREDENTIALS_MSG)),
        }
        let result = self.post(&url, &params, ApiType::Private).await?;
        self.convert_result::<JSONResponse<Withdrawal<PhoneWithdrawal>>>(&result)
    }
}
