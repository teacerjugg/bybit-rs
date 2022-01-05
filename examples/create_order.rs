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

    // dbg!(rest.public_orderbook_l2(Symbol::BTCUSD).await?);

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

    let order_list: RestResponse = rest
        .private_order_list(Symbol::BTCUSD, None, None, None, None)
        .await?;
    dbg!(order_list);

    // let cancel_response: RestResponse = rest
    //     .private_cancel_order(Symbol::BTCUSD, "test", None)
    //     .await?;
    // dbg!(cancel_response);

    // (0..=5).for_each(|_| {
    //     sleep(Duration::from_secs(5));
    // });

    Ok(())
}
