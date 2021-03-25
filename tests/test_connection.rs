extern crate bybit_rs;
use dotenv::dotenv;
use log::info;
use std::env;

fn init() {
    dotenv().ok();
    let _ = env_logger::builder().is_test(true).try_init();
}


#[tokio::test]
async fn test_connect_to_bybit_api() {
    init();

    println!("{}", env::var("RUST_LOG").unwrap());

    let ws = bybit_rs::connect().await.unwrap();
    info!("connected to API");
    let _ = bybit_rs::subscribe(ws).await.unwrap();
    info!("Subscribed topics")
}
