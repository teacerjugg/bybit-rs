use super::serde_side;
use crate::common::Order;
use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Deserializer};
use std::collections::HashMap;

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
