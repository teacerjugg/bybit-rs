use super::serde_tick;
use crate::enums::Tick;
use serde::{self, Deserialize, Deserializer};

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
