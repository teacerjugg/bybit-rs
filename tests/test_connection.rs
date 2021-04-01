extern crate bybit_rs;
use dotenv::dotenv;
// use log::info;
use std::env;
use bybit_rs::websocket::{WebsocketBuilder, Endpoint, API};


fn init() {
    dotenv().ok();
    let _ = env_logger::builder().is_test(false).try_init();
}

#[tokio::test]
#[ignore]
async fn connect() {
    init();
    let api: API = API {
        key: env::var("API_KEY").unwrap(),
        secret: env::var("API_SECRET").unwrap(),
    };

    let _ = WebsocketBuilder::new().endpoint(Endpoint::MAINNET).api(api).build();
}

#[tokio::test]
async fn subscribe() {
    init();
    let api: API = API {
        key: env::var("API_KEY").unwrap(),
        secret: env::var("API_SECRET").unwrap(),
    };

    // println!("{}", env::var("RUST_LOG").unwrap());

    let ws = WebsocketBuilder::new().endpoint(Endpoint::MAINNET).api(api).build().await;
    // info!("connected to API");
    let _ = ws.subscribe().await;
    // info!("Subscribed topics")
}

#[tokio::test]
#[ignore]
async fn ping() {
    init();
    let api: API = API {
        key: env::var("API_KEY").unwrap(),
        secret: env::var("API_SECRET").unwrap(),
    };

    let ws = WebsocketBuilder::new().endpoint(Endpoint::MAINNET).api(api).build().await;
    let _ = ws.ping().await;
}
