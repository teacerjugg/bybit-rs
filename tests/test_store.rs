mod common;

extern crate bybit_rs;
use bybit_rs::prelude::{Endpoint, WebsocketBuilder, WebsocketResponse, API};
use bybit_rs::store;
use log::{debug, info};
use std::env;
use tokio::time::{sleep, Duration};

#[test]
fn store_message_snapshot() -> common::BEResult {
    common::init();

    let data = r#"
        {
            "cross_seq": 5567735855,
            "data": [{
                "id": 543800000,
                "price": "54380.00",
                "side": "Buy",
                "size": 3928,
                "symbol": "BTCUSD"
              }, {
                "id": 545925000,
                "price": "54592.50",
                "side": "Sell",
                "size": 18,
                "symbol": "BTCUSD"
            }],
            "timestamp_e6": 1616855405687214,
            "topic": "orderBook_200.100ms.BTCUSD",
            "type": "snapshot"
        }"#;

    let res: WebsocketResponse = serde_json::from_str(data)?;
    store::store_message(res);

    Ok(())
}

#[test]
fn store_message_delta() -> common::BEResult {
    common::init();

    let data = r#"
        {
            "topic": "orderBook_200.100ms.BTCUSD",
            "type": "delta",
            "data": {
                "delete": [{
                    "price": "58892.50",
                    "symbol": "BTCUSD",
                    "id": 588925000,
                    "side": "Sell"
                }],
                "update": [{
                    "price": "58818.50",
                    "symbol": "BTCUSD",
                    "id": 588185000,
                    "side": "Sell",
                    "size": 290181
                }],
                "insert": [{
                    "price": "58785.50",
                    "symbol": "BTCUSD",
                    "id": 587855000,
                    "side": "Sell",
                    "size": 11552
                }],
                "transactTimeE6": 0
            },
            "cross_seq": 5739038410,
            "timestamp_e6": 1617702691138576
        }"#;

    let res: WebsocketResponse = serde_json::from_str(data)?;
    store::store_message(res);

    Ok(())
}

#[test]
fn store_message_record() -> common::BEResult {
    common::init();

    let data = r#"
        {
            "topic": "trade.BTCUSD",
            "data": [{
                "trade_time_ms": 1617705958803,
                "timestamp": "2021-04-06T10:45:58.000Z",
                "symbol": "BTCUSD",
                "side": "Sell",
                "size": 9460,
                "price": 58617,
                "tick_direction": "ZeroMinusTick",
                "trade_id": "930ba5dd-67d3-5067-b253-117eb1aeeb7b",
                "cross_seq": 5739533035
            }]
        }"#;

    let res: WebsocketResponse = serde_json::from_str(data)?;
    store::store_message(res);

    Ok(())
}

#[tokio::test]
async fn get_data() -> common::BEResult {
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
    ws.subscribe().await?;

    let _handle = tokio::spawn(async move {
        ws.on_message().await.unwrap();
    });
    info!("Spawned on_message function");
    sleep(Duration::from_secs(1)).await;
    // let _ = handle.await.unwrap();

    debug!("{:#?}", bybit_rs::store::take_trading_records());
    debug!("{:#?}", bybit_rs::store::take_orderbook());

    sleep(Duration::from_secs(1)).await;

    debug!("{:#?}", bybit_rs::store::take_trading_records());
    debug!("{:#?}", bybit_rs::store::take_orderbook());

    Ok(())
}
