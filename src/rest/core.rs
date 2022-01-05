use super::{
    enums::{Interval, OrderStatus, OrderType, Period, TimeInForce},
    structs::RestResponse,
};
use crate::common::{Endpoint, Side, Symbol, API};
use hmac::{Hmac, Mac, NewMac};
use maplit::{btreemap, convert_args};
use reqwest::{Client, Result};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

pub struct Rest {
    pub endpoint: Endpoint,
    pub api_key: API,
    pub client: Client,
}

pub struct RestBuilder<EndpointType, ApiType, Client> {
    endpoint: EndpointType,
    api_key: ApiType,
    client: Client,
}

impl RestBuilder<Endpoint, API, Client> {
    pub fn build(self) -> Rest {
        Rest {
            endpoint: self.endpoint,
            api_key: self.api_key,
            client: self.client,
        }
    }
}

impl RestBuilder<(), (), Client> {
    pub fn new() -> Self {
        RestBuilder {
            endpoint: (),
            api_key: (),
            client: Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .unwrap(),
        }
    }
}

impl<EndpointType, ApiType, Client> RestBuilder<EndpointType, ApiType, Client> {
    pub fn endpoint(self, endpoint: Endpoint) -> RestBuilder<Endpoint, ApiType, Client> {
        RestBuilder {
            endpoint,
            api_key: self.api_key,
            client: self.client,
        }
    }

    pub fn api(self, api_key: API) -> RestBuilder<EndpointType, API, Client> {
        RestBuilder {
            endpoint: self.endpoint,
            api_key,
            client: self.client,
        }
    }
}

impl Rest {
    pub fn builder() -> RestBuilder<(), (), Client> {
        RestBuilder::new()
    }

    // pub async fn get(&self) -> Result<()> {
    //     let mut uri = self.endpoint.clone();
    //     uri.set_query(Some(&format!("api_key={}", self.api_key)));
    //     let resp = reqwest::get(uri).await?;
    //     println!("{:#?}", resp);
    //     Ok(())
    // }

    // pub async fn server_time(&self) -> Result<String> {
    //     let resp = reqwest::get(self.endpoint.join("/v2/public/time").unwrap())
    //         .await?
    //         .json()
    //         .await?;

    //     Ok(resp)
    // }

    fn sign(&self, query: &BTreeMap<String, String>) -> String {
        let query_str = query
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&");
        println!("{}", query_str);
        let mut mac = HmacSha256::new_varkey(self.api_key.secret.as_bytes()).unwrap();
        mac.update(query_str.as_bytes());
        format!("{:x}", mac.finalize().into_bytes())
    }

    fn construct_query<H>(&self, mut query: BTreeMap<H, H>) -> BTreeMap<String, String>
    where
        H: std::string::ToString + Ord,
    {
        let now = SystemTime::now();
        let unix_time = now.duration_since(UNIX_EPOCH).expect("back to the future");
        let timestamp = unix_time.as_secs() * 1000;

        let mut query = query
            .iter_mut()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<BTreeMap<String, String>>();
        query.insert("timestamp".to_string(), timestamp.to_string());
        query.insert("api_key".to_string(), self.api_key.key.clone());
        query.insert("sign".to_string(), self.sign(&query));

        // let mut uri = self.endpoint.to_uri();
        // query.iter().for_each(|(k, v)| {
        //     uri.query_pairs_mut().append_pair(k, v);
        // });

        // let uri = self.endpoint.to_uri_with_params(query);

        query
    }

    pub async fn public_orderbook_l2(&self, symbol: Symbol) -> Result<RestResponse> {
        const PATH: &str = "/v2/public/orderBook/L2";

        let mut uri = self.endpoint.to_uri();
        uri.set_query(Some(&format!("symbol={}", symbol.to_string())));
        uri.set_path(PATH);
        let resp = self.client.get(uri).send().await?.json().await?;

        Ok(resp)
    }

    pub async fn public_kline_list(
        &self,
        symbol: Symbol,
        interval: Interval,
        from: usize,
        limit: Option<usize>,
    ) -> Result<RestResponse> {
        const PATH: &str = "/v2/public/kline/list";

        let mut query = convert_args!(btreemap!(
            "symbol" => symbol,
            "interval" => interval,
            "from" => from.to_string(),
        ));
        if let Some(limit) = limit {
            query.insert(String::from("limit"), limit.to_string());
        }

        let mut uri = self.endpoint.to_uri_with_params(query);
        uri.set_path(PATH);
        let resp = self.client.get(uri).send().await?.json().await?;

        Ok(resp)
    }

    pub async fn public_tickers(&self, symbol: Option<Symbol>) -> Result<RestResponse> {
        const PATH: &str = "/v2/public/tickers";

        let mut uri = self.endpoint.to_uri();
        if let Some(symbol) = symbol {
            uri.set_query(Some(&format!("symbol={}", symbol.to_string())));
        }
        uri.set_path(PATH);
        let resp = self.client.get(uri).send().await?.json().await?;

        Ok(resp)
    }

    pub async fn public_trading_records(
        &self,
        symbol: Symbol,
        from: Option<usize>,
        limit: Option<usize>,
    ) -> Result<RestResponse> {
        const PATH: &str = "/v2/public/trading-records";

        let mut query = BTreeMap::new();
        query.insert(String::from("symbol"), symbol.to_string());
        if let Some(from) = from {
            query.insert(String::from("from"), from.to_string());
        }
        if let Some(limit) = limit {
            query.insert(String::from("limit"), limit.to_string());
        }

        let mut uri = self.endpoint.to_uri_with_params(query);
        uri.set_path(PATH);
        let resp = self.client.get(uri).send().await?.json().await?;

        Ok(resp)
    }

    pub async fn public_symbols(&self) -> Result<RestResponse> {
        const PATH: &str = "/v2/public/symbols";

        let uri = self.endpoint.to_uri().join(PATH).unwrap();
        let resp = self.client.get(uri).send().await?.json().await?;

        Ok(resp)
    }

    pub async fn public_liq_records(
        &self,
        symbol: Symbol,
        from: Option<usize>,
        limit: Option<usize>,
        start_time: Option<usize>,
        end_time: Option<usize>,
    ) -> Result<RestResponse> {
        const PATH: &str = "/v2/public/liq-records";

        let mut query = BTreeMap::new();
        query.insert(String::from("symbol"), symbol.to_string());
        if let Some(from) = from {
            query.insert(String::from("from"), from.to_string());
        }
        if let Some(limit) = limit {
            query.insert(String::from("limit"), limit.to_string());
        }
        if let Some(start_time) = start_time {
            query.insert(String::from("start_time"), start_time.to_string());
        }
        if let Some(end_time) = end_time {
            query.insert(String::from("end_time"), end_time.to_string());
        }

        let mut uri = self.endpoint.to_uri_with_params(query);
        uri.set_path(PATH);
        let resp = self.client.get(uri).send().await?.json().await?;

        Ok(resp)
    }

    pub async fn public_mark_price_kline(
        &self,
        symbol: Symbol,
        interval: Interval,
        from: usize,
        limit: Option<usize>,
    ) -> Result<RestResponse> {
        const PATH: &str = "/v2/public/mark-price-kline";

        let mut query = convert_args!(btreemap!(
            "symbol" => symbol,
            "interval" => interval,
            "from" => from.to_string(),
        ));
        if let Some(limit) = limit {
            query.insert(String::from("limit"), limit.to_string());
        }

        let mut uri = self.endpoint.to_uri_with_params(query);
        uri.set_path(PATH);
        let resp = self.client.get(uri).send().await?.json().await?;

        Ok(resp)
    }

    pub async fn public_index_price_kline(
        &self,
        symbol: Symbol,
        interval: Interval,
        from: usize,
        limit: Option<usize>,
    ) -> Result<RestResponse> {
        const PATH: &str = "/v2/public/index-price-kline";

        let mut query = convert_args!(btreemap!(
            "symbol" => symbol,
            "interval" => interval,
            "from" => from.to_string(),
        ));
        if let Some(limit) = limit {
            query.insert(String::from("limit"), limit.to_string());
        }

        let mut uri = self.endpoint.to_uri_with_params(query);
        uri.set_path(PATH);
        let resp = self.client.get(uri).send().await?.json().await?;

        Ok(resp)
    }

    pub async fn public_premium_price_kline(
        &self,
        symbol: Symbol,
        interval: Interval,
        from: usize,
        limit: Option<usize>,
    ) -> Result<RestResponse> {
        const PATH: &str = "/v2/public/premium-index-kline";

        let mut query = convert_args!(btreemap!(
            "symbol" => symbol,
            "interval" => interval,
            "from" => from.to_string(),
        ));
        if let Some(limit) = limit {
            query.insert(String::from("limit"), limit.to_string());
        }

        let mut uri = self.endpoint.to_uri_with_params(query);
        uri.set_path(PATH);
        let resp = self.client.get(uri).send().await?.json().await?;

        Ok(resp)
    }

    pub async fn public_open_interest(
        &self,
        symbol: Symbol,
        period: Period,
        limit: Option<usize>,
    ) -> Result<RestResponse> {
        const PATH: &str = "/v2/public/open-interest";

        let mut query = convert_args!(btreemap!(
            "symbol" => symbol,
            "period" => period,
        ));
        if let Some(limit) = limit {
            query.insert(String::from("limit"), limit.to_string());
        }

        let mut uri = self.endpoint.to_uri_with_params(query);
        uri.set_path(PATH);
        let resp = self.client.get(uri).send().await?.json().await?;

        Ok(resp)
    }

    pub async fn public_big_deal(
        &self,
        symbol: Symbol,
        limit: Option<usize>,
    ) -> Result<RestResponse> {
        const PATH: &str = "/v2/public/big-deal";

        let mut query = convert_args!(btreemap!(
            "symbol" => symbol,
        ));
        if let Some(limit) = limit {
            query.insert(String::from("limit"), limit.to_string());
        }

        let mut uri = self.endpoint.to_uri_with_params(query);
        uri.set_path(PATH);
        let resp = self.client.get(uri).send().await?.json().await?;

        Ok(resp)
    }

    pub async fn public_account_ratio(
        &self,
        symbol: Symbol,
        period: Period,
        limit: Option<usize>,
    ) -> Result<RestResponse> {
        const PATH: &str = "/v2/public/account-ratio";

        let mut query = convert_args!(btreemap!(
            "symbol" => symbol,
            "period" => period,
        ));
        if let Some(limit) = limit {
            query.insert(String::from("limit"), limit.to_string());
        }

        let mut uri = self.endpoint.to_uri_with_params(query);
        uri.set_path(PATH);
        let resp = self.client.get(uri).send().await?.json().await?;

        Ok(resp)
    }

    pub async fn public_server_time(&self) -> Result<RestResponse> {
        const PATH: &str = "/v2/public/time";

        let uri = self.endpoint.to_uri().join(PATH).unwrap();
        let resp = self.client.get(uri).send().await?.json().await?;

        Ok(resp)
    }

    pub async fn private_order_create(
        &self,
        side: Side,
        symbol: Symbol,
        order_type: OrderType,
        qty: usize,
        price: Option<f32>,
        time_in_force: TimeInForce,
        take_profit: Option<f32>,
        stop_loss: Option<f32>,
        reduce_only: Option<bool>,
        close_on_trigger: Option<bool>,
    ) -> Result<RestResponse> {
        const PATH: &str = "/v2/private/order/create";

        let mut query = convert_args!(btreemap!(
            "side" => side.to_string(),
            "symbol" => symbol,
            "order_type" => order_type.to_string(),
            "qty" => qty.to_string(),
            "time_in_force" => time_in_force.to_string(),
        ));
        if let Some(price) = price {
            query.insert(String::from("price"), price.to_string());
        }
        if let Some(take_profit) = take_profit {
            query.insert(String::from("take_profit"), take_profit.to_string());
        }
        if let Some(stop_loss) = stop_loss {
            query.insert(String::from("stop_loss"), stop_loss.to_string());
        }
        if let Some(reduce_only) = reduce_only {
            query.insert(String::from("reduce_only"), reduce_only.to_string());
        }
        if let Some(close_on_trigger) = close_on_trigger {
            query.insert(
                String::from("close_on_trigger"),
                close_on_trigger.to_string(),
            );
        }

        let mut uri = self
            .endpoint
            .to_uri_with_params(self.construct_query(query));
        uri.set_path(PATH);

        let resp = self
            .client
            .post(uri)
            //.header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::CONTENT_LENGTH, 0)
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }

    pub async fn private_order_list(
        &self,
        symbol: Symbol,
        order_status: Option<OrderStatus>,
        direction: Option<String>,
        limit: Option<usize>,
        cursor: Option<String>,
    ) -> Result<RestResponse> {
        const PATH: &str = "/v2/private/order/list";

        let mut query = BTreeMap::new();
        query.insert(String::from("symbol"), symbol.to_string());
        if let Some(order_status) = order_status {
            query.insert(String::from("order_status"), order_status.to_string());
        }
        if let Some(direction) = direction {
            query.insert(String::from("direction"), direction);
        }
        if let Some(limit) = limit {
            query.insert(String::from("limit"), limit.to_string());
        }
        if let Some(cursor) = cursor {
            query.insert(String::from("cursor"), cursor);
        }

        let mut uri = self
            .endpoint
            .to_uri_with_params(self.construct_query(query));
        uri.set_path(PATH);
        let resp = self.client.get(uri).send().await?.json().await?;

        Ok(resp)
    }

    pub async fn private_cancel_order<T: ToString>(
        &self,
        symbol: Symbol,
        order_id: T,
        order_link_id: Option<String>,
    ) -> Result<RestResponse> {
        const PATH: &str = "/v2/private/order/cancel";

        let mut query = BTreeMap::new();
        query.insert(String::from("symbol"), symbol.to_string());
        query.insert(String::from("order_id"), order_id.to_string());
        if let Some(order_link_id) = order_link_id {
            query.insert(String::from("order_link_id"), order_link_id);
        }

        let mut uri = self
            .endpoint
            .to_uri_with_params(self.construct_query(query));
        uri.set_path(PATH);
        let resp = self
            .client
            .post(uri)
            .header(reqwest::header::CONTENT_LENGTH, 0)
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }

    pub async fn private_cancel_all_orders(&self, symbol: Symbol) -> Result<RestResponse> {
        const PATH: &str = "/v2/private/order/cancelAll";

        let mut query = BTreeMap::new();
        query.insert(String::from("symbol"), symbol.to_string());

        let mut uri = self
            .endpoint
            .to_uri_with_params(self.construct_query(query));
        uri.set_path(PATH);
        let resp = self
            .client
            .post(uri)
            .header(reqwest::header::CONTENT_LENGTH, 0)
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }

    pub async fn private_replace_order(
        &self,
        _symbol: Symbol,
        _order_id: String,
        _order_link_id: Option<String>,
        _p_r_qty: Option<usize>,
        _p_r_price: Option<f32>,
        _take_profit: Option<f32>,
        _stop_loss: Option<f32>,
        _tp_trigger_by: Option<String>,
        _sl_trigger_by: Option<String>,
    ) -> Result<RestResponse> {
        unimplemented!();
    }

    pub async fn private_query_order(
        &self,
        _symbol: Symbol,
        _order_id: Option<String>,
        _order_link_id: Option<String>,
    ) -> Result<RestResponse> {
        unimplemented!();
    }

    pub async fn private_position_list(&self, symbol: Option<Symbol>) -> Result<RestResponse> {
        const PATH: &str = "/v2/private/position/list";

        let mut query = BTreeMap::new();
        if let Some(symbol) = symbol {
            query.insert(String::from("symbol"), symbol.to_string());
        }

        let mut uri = self.endpoint.to_uri_with_params(query);
        uri.set_path(PATH);
        let resp = self.client.get(uri).send().await?.json().await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{Endpoint, Symbol, API};
    use crate::rest::enums::Interval;
    use chrono::Utc;
    use log::debug;
    use std::collections::BTreeMap;
    use std::env;

    fn init() {
        dotenv::dotenv().ok();
        // let _ = pretty_env_logger::formatted_builder().try_init();
        pretty_env_logger::init();
    }

    #[test]
    fn test_construct_request() {
        init();

        let mut hash = BTreeMap::new();
        hash.insert("key", "value");

        let rest = super::RestBuilder::new()
            .api(API {
                key: String::from("this-is-key"),
                secret: String::from("this-is-secret"),
            })
            .endpoint(Endpoint::TESTNET)
            .build();

        debug!("{:#?}", rest.construct_query(hash));
    }

    #[tokio::test]
    async fn test_public_orderbook_l2() {
        init();

        let rest = super::RestBuilder::new()
            .api(API {
                key: env::var("TESTNET_API_KEY").unwrap(),
                secret: env::var("TESTNET_API_SECRET").unwrap(),
            })
            .endpoint(Endpoint::TESTNET)
            .build();

        let resp = rest.public_orderbook_l2(Symbol::BTCUSD).await;
        debug!("{:#?}", resp);

        assert!(resp.is_ok());
    }

    #[tokio::test]
    async fn test_public_server_time() {
        init();

        let rest = super::RestBuilder::new()
            .api(API {
                key: env::var("TESTNET_API_KEY").unwrap(),
                secret: env::var("TESTNET_API_SECRET").unwrap(),
            })
            .endpoint(Endpoint::TESTNET)
            .build();

        let resp = rest.public_server_time().await;
        debug!("{:#?}", resp);

        assert!(resp.is_ok());
    }

    #[tokio::test]
    async fn test_public_kline_list() {
        init();

        let rest = super::RestBuilder::new()
            .api(API {
                key: env::var("TESTNET_API_KEY").unwrap(),
                secret: env::var("TESTNET_API_SECRET").unwrap(),
            })
            .endpoint(Endpoint::TESTNET)
            .build();

        let resp = rest
            .public_kline_list(
                Symbol::BTCUSD,
                Interval::FiveMin,
                Utc::now().timestamp() as usize - 300,
                None,
            )
            .await;
        debug!("{:#?}", resp);

        assert!(resp.is_ok());
    }

    #[test]
    fn test_url() {
        init();
        let mut uri = reqwest::Url::parse("https://api-testnet.bybit.com").unwrap();

        debug!("{:#?}", uri);
        uri.set_query(Some("key=val"));
        debug!("{:#?}", uri);
        uri.set_path("/v2/orderBook/L2/");
        debug!("{:#?}", uri);
    }
}
