use crate::structs::{Limit, OrderBook, Record};
use crate::websocket::WebsocketResponse;
use chrono::Utc;
use once_cell::sync::Lazy;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Mutex;

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
        match res.msg_type.unwrap().as_str() {
            "snapshot" => {
                orderbook.timestamp = res.timestamp.unwrap();
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
                orderbook.timestamp = res.timestamp.unwrap();
                if let Value::Object(data) = res.data {
                    data.get("delete")
                        .unwrap()
                        .as_array()
                        .unwrap()
                        .into_iter()
                        .for_each(|p| {
                            orderbook
                                .limits
                                .remove(&p.get("id").unwrap().as_u64().unwrap());
                        });
                    data.get("update")
                        .unwrap()
                        .as_array()
                        .unwrap()
                        .into_iter()
                        .for_each(|p| {
                            orderbook.limits.insert(
                                p.get("id").unwrap().as_u64().unwrap(),
                                serde_json::from_value::<Limit>(p.clone())
                                    .expect("Failed to deserialize response data"),
                            );
                        });
                    data.get("insert")
                        .unwrap()
                        .as_array()
                        .unwrap()
                        .into_iter()
                        .for_each(|p| {
                            orderbook.limits.insert(
                                p.get("id").unwrap().as_u64().unwrap(),
                                serde_json::from_value::<Limit>(p.clone())
                                    .expect("Failed to deserialize response data"),
                            );
                        });
                }
            }
            _ => panic!("Impossible message type"),
        }
    } else if res.topic.starts_with("trade") {
        let mut records = TRADING_RECORDS
            .lock()
            .expect("Failed to lock Mutex<Vec<Record>>");

        if let Value::Array(data) = res.data {
            data.into_iter().for_each(|r| {
                records.push(
                    serde_json::from_value::<Record>(r).expect("Failed to deserialize record"),
                );
            })
        }
    }
}

pub fn take_orderbook() -> OrderBook {
    ORDERBOOK.lock().unwrap().clone()
}

pub fn take_trading_records() -> Vec<Record> {
    TRADING_RECORDS.lock().unwrap().drain(..).collect()
}
