use crate::enums::{Order, Tick};
use crate::serde_record_timestamp;
use async_tungstenite::tungstenite::protocol::Message;
use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct WsArgs {
    pub op: String,
    pub args: Option<Vec<String>>,
}

impl WsArgs {
    pub fn into_msg(self) -> Message {
        Message::text(serde_json::to_string(&self).expect("Failed to serialize Args struct"))
    }
}

#[derive(Debug, Clone)]
pub struct API {
    pub key: String,
    pub secret: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Record {
    pub trade_id: String,
    pub price: f32,
    #[serde(with = "serde_side")]
    pub side: Order,
    pub size: u32,
    #[serde(rename(deserialize = "trade_time_ms"))]
    #[serde(with = "serde_record_timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(with = "serde_tick")]
    pub tick_direction: Tick,
}

#[derive(Debug)]
pub struct OrderBook {
    pub limits: HashMap<u64, Limit>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct Limit {
    #[serde(deserialize_with = "deserialize_price")]
    pub price: f32,
    #[serde(with = "serde_side")]
    pub side: Order,
    pub size: u32,
}

fn deserialize_price<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.parse::<f32>().unwrap())
}

mod serde_side {
    use super::Order;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(side: &Order, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match side {
            Order::Buy => "Buy",
            Order::Sell => "Sell",
        };
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Order, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Buy" => Ok(Order::Buy),
            "Sell" => Ok(Order::Sell),
            _ => panic!("Impossible order side"),
        }
    }
}

pub mod serde_tick {
    use super::Tick;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(tick: &Tick, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match tick {
            Tick::MinusTick => "MinusTick",
            Tick::ZeroMinusTick => "ZeroMinusTick",
            Tick::PlusTick => "PlusTick",
            Tick::ZeroPlusTick => "ZeroPlusTick",
        };
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Tick, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "MinusTick" => Ok(Tick::MinusTick),
            "ZeroMinusTick" => Ok(Tick::ZeroMinusTick),
            "PlusTick" => Ok(Tick::PlusTick),
            "ZeroPlusTick" => Ok(Tick::ZeroPlusTick),
            _ => panic!("Impossible tick direction"),
        }
    }
}
