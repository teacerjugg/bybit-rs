mod common;
extern crate bybit_rs;
// use async_std::future::timeout;
use bybit_rs::websocket::{Endpoint, Topic, WebsocketBuilder, API};
use std::env;
// use std::time::Duration;
use log::debug;
use tokio::time::{sleep, Duration};

#[tokio::test]
#[ignore]
async fn connect() {
    common::init();

    let api: API = API {
        key: env::var("API_KEY").unwrap(),
        secret: env::var("API_SECRET").unwrap(),
    };

    let _ = WebsocketBuilder::new()
        .endpoint(Endpoint::MAINNET)
        .api(api)
        .build();
}

#[tokio::test]
#[ignore]
async fn subscribe() {
    common::init();

    let api: API = API {
        key: env::var("API_KEY").unwrap(),
        secret: env::var("API_SECRET").unwrap(),
    };

    let mut ws = WebsocketBuilder::new()
        .endpoint(Endpoint::MAINNET)
        .api(api)
        .build()
        .await;

    assert!(ws
        .subscribe(vec![Topic::OrderBook200, Topic::Trade])
        .await
        .is_ok());
}

#[tokio::test]
#[ignore]
async fn ping() {
    common::init();

    let api: API = API {
        key: env::var("API_KEY").unwrap(),
        secret: env::var("API_SECRET").unwrap(),
    };

    let mut ws = WebsocketBuilder::new()
        .endpoint(Endpoint::MAINNET)
        .api(api)
        .build()
        .await;

    assert!(ws.ping().await.is_ok());
}

#[tokio::test]
#[ignore]
async fn on_message() {
    common::init();

    let api: API = API {
        key: env::var("API_KEY").unwrap(),
        secret: env::var("API_SECRET").unwrap(),
    };

    let mut ws = WebsocketBuilder::new()
        .endpoint(Endpoint::MAINNET)
        .api(api)
        .build()
        .await;
    ws.subscribe(vec![Topic::OrderBook200, Topic::Trade])
        .await
        .unwrap();

    assert!(ws.on_message().await.is_ok());
}

#[tokio::test]
async fn run_forever() -> common::BEResult {
    common::init();

    let api: API = API {
        key: env::var("API_KEY").unwrap(),
        secret: env::var("API_SECRET").unwrap(),
    };

    let mut ws = WebsocketBuilder::new()
        .endpoint(Endpoint::MAINNET)
        .api(api)
        .build()
        .await;
    ws.subscribe(vec![Topic::OrderBook200, Topic::Trade])
        .await?;

    // assert!(timeout(Duration::from_secs(10), ws.run_forever())
    //     .await
    //     .is_ok());
    let _ = ws.run_forever().await;

    sleep(Duration::from_secs(5)).await;

    debug!("{:#?}", bybit_rs::store::take_trading_records());
    debug!("{:#?}", bybit_rs::store::take_orderbook());

    sleep(Duration::from_secs(2)).await;

    debug!("{:#?}", bybit_rs::store::take_trading_records());
    debug!("{:#?}", bybit_rs::store::take_orderbook());

    Ok(())
}
