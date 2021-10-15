use bybit_rs::{
    prelude::{Endpoint, Symbol, Topic, WebSocket, WebSocketBuilder, API},
    websocket::store::take_orderbook,
};
use dotenv::dotenv;
use std::{env, error::Error, thread::sleep, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    pretty_env_logger::init();

    let api: API = API {
        key: env::var("TESTNET_API_KEY").unwrap(),
        secret: env::var("TESTNET_API_SECRET").unwrap(),
    };

    let mut ws: WebSocket = WebSocketBuilder::new()
        .endpoint(Endpoint::TESTNET)
        .api(api)
        .build()
        .await;

    ws.subscribe(vec![Topic::OrderBook200, Topic::Trade], Symbol::BTCUSD)
        .await?;
    ws.run_forever().await;

    (0..=5).for_each(|_| {
        sleep(Duration::from_secs(5));
        println!("{:#?}", take_orderbook());
    });

    Ok(())
}
