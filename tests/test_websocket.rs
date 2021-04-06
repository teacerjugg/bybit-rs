mod common;
extern crate bybit_rs;
use async_std::future::timeout;
use bybit_rs::websocket::{Endpoint, WebsocketBuilder, API};
use std::env;
use std::time::Duration;

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
async fn subscribe() -> common::BEResult {
    common::init();

    let api: API = API {
        key: env::var("API_KEY").unwrap(),
        secret: env::var("API_SECRET").unwrap(),
    };

    // println!("{}", env::var("RUST_LOG").unwrap());

    let mut ws = WebsocketBuilder::new()
        .endpoint(Endpoint::MAINNET)
        .api(api)
        .build()
        .await;
    let _ = ws.subscribe().await?;

    Ok(())
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
    let _ = ws.ping().await;
}

#[tokio::test]
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
    ws.subscribe().await.unwrap();

    assert!(timeout(Duration::from_secs(1), ws.on_message())
        .await
        .is_err());
}
