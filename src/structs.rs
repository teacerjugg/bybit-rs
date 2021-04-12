use crate::enums::{Order, Tick};
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

#[derive(Clone, Debug)]
pub struct OrderBook {
    pub limits: HashMap<u64, Limit>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Deserialize, Clone, Debug)]
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

mod serde_tick {
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

mod serde_record_timestamp {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.to_rfc3339();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = i64::deserialize(deserializer)?;
        Ok(Utc.timestamp_millis(s))
    }
}

#[derive(Deserialize, Default)]
pub struct Instrument {
    pub symbol: String,
    #[serde(rename(deserialize = "last_price_e4"))]
    #[serde(deserialize_with = "deserialize_e4")]
    pub last_price: f32,
    #[serde(with = "serde_tick")]
    pub last_tick_direction: Tick,
    #[serde(rename(deserialize = "prev_price_24h_e4"))]
    #[serde(deserialize_with = "deserialize_e4")]
    pub prev_price_24h: f32,
    #[serde(rename(deserialize = "price_24h_pcnt_e6"))]
    #[serde(deserialize_with = "deserialize_e6")]
    pub price_24h_pcnt: f32,
    #[serde(rename(deserialize = "high_price_24h_e4"))]
    #[serde(deserialize_with = "deserialize_e4")]
    pub high_price_24h: f32,
    #[serde(rename(deserialize = "low_price_24h_e4"))]
    #[serde(deserialize_with = "deserialize_e4")]
    pub low_price_24h: f32,
    #[serde(rename(deserialize = "prev_price_1h_e4"))]
    #[serde(deserialize_with = "deserialize_e4")]
    pub prev_price_1h: f32,
    #[serde(rename(deserialize = "price_1h_pcnt_e6"))]
    #[serde(deserialize_with = "deserialize_e6")]
    pub price_1h_pcnt: f32,
    #[serde(rename(deserialize = "mark_price_e4"))]
    #[serde(deserialize_with = "deserialize_e4")]
    pub mark_price: f32,
    #[serde(rename(deserialize = "index_price_e4"))]
    #[serde(deserialize_with = "deserialize_e4")]
    pub index_price: f32,
    pub open_interest: usize,
    // #[serde(rename(deserialize = "open_value_e8"))]
    // open_value: usize, // open position value * 10^8
    #[serde(rename(deserialize = "total_turnover_e8"))]
    #[serde(deserialize_with = "deserialize_e8")]
    pub total_turnover: f64,
    #[serde(rename(deserialize = "turnover_24h_e8"))]
    #[serde(deserialize_with = "deserialize_e8")]
    pub turnover_24h: f64,
    pub total_volume: usize,
    pub volume_24h: usize,
    #[serde(rename(deserialize = "predicted_funding_rate_e6"))]
    #[serde(deserialize_with = "deserialize_e6")]
    pub predicted_funding_rate: f32,
    pub created_at: String,
    pub updated_at: String,
    pub next_funding_time: String,
    pub countdown_hour: u8,
}

fn deserialize_e4<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = i32::deserialize(deserializer)?;
    Ok(s as f32 / 10f32.powi(4))
}

fn deserialize_e6<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = i32::deserialize(deserializer)?;
    Ok(s as f32 / 10f32.powi(6))
}

fn deserialize_e8<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = i64::deserialize(deserializer)?;
    Ok(s as f64 / 10f64.powi(8))
}
