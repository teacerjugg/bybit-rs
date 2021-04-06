mod common;

extern crate bybit_rs;
use bybit_rs::store;
use bybit_rs::websocket::WebsocketResponse;

#[test]
fn store_message() -> common::BEResult {
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
