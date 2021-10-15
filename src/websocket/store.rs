use super::core::WebSocketResponse;
use super::structs::{Instrument, Limit, OrderBook, Position, Record};
use chrono::Utc;
use once_cell::sync::Lazy;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Mutex;

static ORDERBOOK: Lazy<Mutex<OrderBook>> = Lazy::new(|| {
    Mutex::new(OrderBook {
        limits: HashMap::new(),
        timestamp: Utc::now(),
    })
});

static TRADING_RECORDS: Lazy<Mutex<Vec<Record>>> = Lazy::new(|| {
    let v = Vec::new();
    Mutex::new(v)
});

static INSTRUMENT: Lazy<Mutex<Instrument>> = Lazy::new(|| Mutex::new(Default::default()));
static POSITION: Lazy<Mutex<Position>> = Lazy::new(|| Mutex::new(Default::default()));

fn orderbook(res: WebSocketResponse) {
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
}

fn records(res: WebSocketResponse) {
    let mut records = TRADING_RECORDS
        .lock()
        .expect("Failed to lock Mutex<Vec<Record>>");

    if let Value::Array(data) = res.data {
        data.into_iter().for_each(|r| {
            records
                .push(serde_json::from_value::<Record>(r).expect("Failed to deserialize record"));
        })
    }
}

fn instrument(res: WebSocketResponse) {
    let mut instrument = INSTRUMENT.lock().expect("Failed to lock Mutex<Instrument>");
    match res.msg_type.unwrap().as_str() {
        "snapshot" => {
            *instrument = serde_json::from_value::<Instrument>(res.data)
                .expect("Failed to deserialize snapshot of instrument_info");
        }
        "delta" => {
            if let Value::Object(data) = res.data {
                *instrument =
                    serde_json::from_value::<Instrument>(data.get("update").unwrap().clone())
                        .expect("Failed to deserialize delta of instrument_info");
            }
        }
        _ => panic!("Impossible message type"),
    }
}

fn position(res: WebSocketResponse) {
    let mut position = POSITION.lock().expect("Failed to lock Mutex<Position>");
    *position =
        serde_json::from_value::<Position>(res.data).expect("Failed to deserialize position");
}

pub(crate) fn store_message(res: WebSocketResponse) {
    match res.topic.chars().next() {
        Some('o') if res.topic.chars().nth(5).is_some() => orderbook(res), // orderbook
        Some('t') => records(res),                                         // trade
        Some('i') if res.topic.chars().nth(10).is_none() => todo!(),       // insurance
        Some('i') => instrument(res),                                      // instrument_info
        Some('k') => todo!(),                                              // kline
        Some('p') => position(res),                                        // position
        Some('e') => todo!(),                                              // execution
        Some('o') => todo!(),                                              // order
        Some('s') => todo!(),                                              // stop_order
        _ => unreachable!(),
    }

    // if res.topic.starts_with("orderBook") {
    //     store_orderbook(res);
    // } else if res.topic.starts_with("trade") {
    //     store_records(res);
    // } else if res.topic.starts_with("instrument_info") {
    //     todo!();
    // } else if res.topic.starts_with("kline") {
    //     todo!();
    // } else if res.topic.starts_with("position") {
    //     todo!();
    // } else if res.topic.starts_with("execution") {
    //     todo!();
    // } else if res.topic.starts_with("order") {
    //     todo!();
    // } else if res.topic.starts_with("stop_order") {
    //     todo!();
    // }
}

pub fn take_orderbook() -> OrderBook {
    ORDERBOOK.lock().unwrap().clone()
}

pub fn take_trading_records() -> Vec<Record> {
    TRADING_RECORDS.lock().unwrap().drain(..).collect()
}
