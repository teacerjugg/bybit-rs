extern crate bybit_rs;
use dotenv::dotenv;
// use log::info;
// use std::env;

fn init() {
    dotenv().ok();
    let _ = env_logger::builder().is_test(true).try_init();
}


#[tokio::test]
#[ignore]
async fn test_connect_to_bybit_api() {
    init();

    // println!("{}", env::var("RUST_LOG").unwrap());

    let mut ws = bybit_rs::websocket::connect().await.unwrap();
    // info!("connected to API");
    let _ = bybit_rs::websocket::subscribe(&mut ws).await.unwrap();
    // info!("Subscribed topics")
}

#[tokio::test]
async fn test_ping() {
    init();

    let mut ws = bybit_rs::websocket::connect().await.unwrap();
    let _ = bybit_rs::websocket::ping(&mut ws).await.unwrap();
}
