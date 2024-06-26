use chrono::{DateTime, TimeZone, Utc};
use serde::{self, Deserialize, Deserializer};
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct RestResponse {
    pub ret_code: usize,
    pub ret_msg: String,
    pub ext_code: String,
    pub ext_info: String,
    pub result: Value,
    #[serde(rename(deserialize = "time_now"))]
    #[serde(deserialize_with = "deserialize_time_now")]
    pub timestamp: DateTime<Utc>,
    pub rate_limit_status: Option<usize>,
    pub rate_limit_reset_ms: Option<usize>,
    pub rate_limit: Option<usize>,
}

fn deserialize_time_now<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let timestamp = s.parse::<f64>().unwrap();
    Ok(Utc.timestamp_millis((timestamp * 10f64.powi(3)) as i64))
}
