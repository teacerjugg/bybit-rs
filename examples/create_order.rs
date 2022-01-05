#![allow(unused)]
use bybit_rs::{
    prelude::{Endpoint, Side, Symbol, API},
    rest::{OrderType, Rest, RestBuilder, RestResponse, TimeInForce},
};
use dotenv::dotenv;
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    pretty_env_logger::init();

    let api: API = API {
        key: env::var("TESTNET_API_KEY").unwrap(),
        secret: env::var("TESTNET_API_SECRET").unwrap(),
    };

    let rest: Rest = RestBuilder::new()
        .endpoint(Endpoint::TESTNET)
        .api(api)
        .build();

    // submit order
    let order_response: RestResponse = rest
        .private_order_create(
            Side::Buy,
            Symbol::BTCUSD,
            OrderType::Limit,
            1usize,
            Some(46_600f32),
            TimeInForce::PostOnly,
            None,
            None,
            None,
            None,
        )
        .await?;
    dbg!(order_response);

    // get orders list
    let order_list: RestResponse = rest
        .private_order_list(Symbol::BTCUSD, None, None, None, None)
        .await?;
    dbg!(order_list);

    // cancell all orders
    let cancel_response: RestResponse = rest.private_cancel_all_orders(Symbol::BTCUSD).await?;
    dbg!(cancel_response);

    Ok(())
}
