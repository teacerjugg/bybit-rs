extern crate bybit_rs;
use async_std::future::timeout;
use bybit_rs::websocket::{Endpoint, WebsocketBuilder, API};
use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::time::Duration;

type BEResult = Result<(), Box<dyn Error>>;

fn init() {
    dotenv().ok();
    // let _ = env_logger::builder().is_test(false).try_init();
    // let _ = pretty_env_logger::formatted_builder().is_test(true).try_init();
    pretty_env_logger::init();
}

#[tokio::test]
#[ignore]
async fn connect() {
    init();

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
async fn subscribe() -> BEResult {
    init();
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
    init();

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
    init();

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

    assert!(timeout(Duration::from_secs(10), ws.on_message()).await.is_err());
}
