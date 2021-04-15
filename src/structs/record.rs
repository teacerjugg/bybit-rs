use super::{serde_side, serde_tick};
use crate::enums::{Order, Tick};
use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Serialize};

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
