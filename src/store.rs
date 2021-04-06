use crate::websocket::WebsocketResponse;
use chrono::Utc;
use log::debug;
use once_cell::sync::Lazy;
// use rayon::prelude::*;
use crate::structs::{Limit, OrderBook, Record};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Mutex;

// pub static ORDERBOOK: Lazy<Mutex<HashMap<u64, Limit>>> = Lazy::new(|| {
//     let mut m = HashMap::new();
//     Mutex::new(m)
// });
pub static ORDERBOOK: Lazy<Mutex<OrderBook>> = Lazy::new(|| {
    Mutex::new(OrderBook {
        limits: HashMap::new(),
        timestamp: Utc::now(),
    })
});

pub static TRADING_RECORDS: Lazy<Mutex<Vec<Record>>> = Lazy::new(|| {
    let v = Vec::new();
    Mutex::new(v)
});

pub fn store_message(res: WebsocketResponse) {
    if res.topic.starts_with("orderBook") {
        let mut orderbook = ORDERBOOK.lock().expect("Failed to lock Mutex<HashMap>");
        match res.msg_type.as_str() {
            "snapshot" => {
                orderbook.timestamp = res.timestamp;
                // res.data.into_par_iter().for_each(|p| { // need parallel?
                if let Value::Array(data) = res.data {
                    data.into_iter().for_each(|p| {
                        orderbook.limits.insert(
                            p.get("id").unwrap().as_u64().unwrap(),
                            serde_json::from_value::<Limit>(p)
                                .expect("Failed to deserialize response data"),
                        );
                    });
                }
            }
            "delta" => {
                orderbook.timestamp = res.timestamp;
            }
            _ => panic!("Impossible message type"),
        }
        debug!("{:#?}", orderbook);
    } else if res.topic.starts_with("trade") {
        unimplemented!();
    }
}
