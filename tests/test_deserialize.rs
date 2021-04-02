extern crate bybit_rs;
use log::debug;
use dotenv::dotenv;
use std::error::Error;
use bybit_rs::websocket::WebsocketResponse;

fn init() {
    dotenv().ok();
    // let _ = env_logger::builder().is_test(false).try_init();
    pretty_env_logger::init();
}

#[test]
fn deserialize_response() -> Result<(), Box<dyn Error>> {
    init();

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
    // let res: Value = serde_json::from_str(data).expect("Failed to deserialize JSON");

    debug!("{:#?}", res);

    Ok(())
}
