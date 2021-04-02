#![allow(unused)]
use async_tungstenite::tungstenite::protocol::Message;
use chrono::{DateTime, TimeZone, Utc};
// use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::fmt;

// enum DataType {
//     OrderBook(Vec<Limit>),
//     TradingRecord(Vec<Record>),
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct WebsocketResponse {
    // cross_seq: u64,
    pub topic: String,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub msg_type: String,
    #[serde(rename(deserialize = "timestamp_e6"))]
    #[serde(with = "serde_timestamp")]
    pub timestamp: DateTime<Utc>,
    pub data: Vec<Value>,
}

mod serde_timestamp {
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
        Ok(Utc.timestamp_nanos(s * 10_i64.pow(3)))
    }
}

// impl<'de> Deserialize<'de> for WebsocketResponse {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         enum Field {
//             Topic,
//             MsgType,
//             Timestamp,
//             Data,
//             Unknown,
//         }

//         impl<'de> Deserialize<'de> for Field {
//             fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
//             where
//                 D: Deserializer<'de>,
//             {
//                 struct FieldVisitor;

//                 impl<'de> Visitor<'de> for FieldVisitor {
//                     type Value = Field;

//                     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                         formatter.write_str("`topic`, `msgtype`, `timestamp` or `data`")
//                     }

//                     fn visit_str<E>(self, value: &str) -> Result<Field, E>
//                     where
//                         E: de::Error,
//                     {
//                         match value {
//                             "topic" => Ok(Field::Topic),
//                             "type" => Ok(Field::MsgType),
//                             "timestamp_e6" => Ok(Field::Timestamp),
//                             "data" => Ok(Field::Data),
//                             _ => Ok(Field::Unknown),
//                             // _ => Err(de::Error::unknown_field(value, FIELDS)),
//                         }
//                     }
//                 }

//                 deserializer.deserialize_identifier(FieldVisitor)
//             }
//         }

//         struct WebsocketResponseVisitor;

//         impl<'de> Visitor<'de> for WebsocketResponseVisitor {
//             type Value = WebsocketResponse;

//             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                 formatter.write_str("struct WebsocketResponse")
//             }

//             fn visit_map<V>(self, mut map: V) -> Result<WebsocketResponse, V::Error>
//             where
//                 V: MapAccess<'de>,
//             {
//                 let mut topic = None;
//                 let mut msg_type = None;
//                 let mut timestamp = None;
//                 let mut data = None;

//                 while let Some(key) = map.next_key::<Field>()? {
//                     match key {
//                         Field::Topic => {
//                             if topic.is_some() {
//                                 return Err(de::Error::duplicate_field("topic"));
//                             }
//                             topic = Some(map.next_value::<String>()?);
//                         }
//                         Field::MsgType => {
//                             if msg_type.is_some() {
//                                 return Err(de::Error::duplicate_field("msg_type"));
//                             }
//                             msg_type = Some(map.next_value::<String>()?);
//                         }
//                         Field::Timestamp => {
//                             if timestamp.is_some() {
//                                 return Err(de::Error::duplicate_field("timestamp"));
//                             }
//                             timestamp =
//                                 Some(Utc.timestamp_nanos(map.next_value::<i64>()? * 10_i64.pow(3)));
//                         }
//                         Field::Data => {
//                             if data.is_some() {
//                                 return Err(de::Error::duplicate_field("data"));
//                             }
//                             data = Some(map.next_value::<Vec<Value>>()?);
//                         }
//                         Field::Unknown => (),
//                     }
//                 }

//                 let topic = topic.ok_or_else(|| de::Error::missing_field("topic"))?;
//                 let msg_type = msg_type.ok_or_else(|| de::Error::missing_field("msg_type"))?;
//                 let timestamp = timestamp.ok_or_else(|| de::Error::missing_field("timestamp"))?;
//                 let data = data.ok_or_else(|| de::Error::missing_field("data"))?;

//                 Ok(WebsocketResponse {
//                     topic,
//                     msg_type,
//                     timestamp,
//                     data,
//                 })
//             }
//         }

//         const FIELDS: &'static [&'static str] = &["topic", "msg_type", "timestamp", "data"];
//         deserializer.deserialize_struct("WebsocketResponse", FIELDS, WebsocketResponseVisitor)
//     }
// }

enum Order {
    Buy,
    Sell,
}

struct Limit {
    price: f32,
    side: String,
    size: u64,
}

struct OrderBook {
    limits: Vec<Limit>,
    timestamp: DateTime<Utc>,
}

struct Record {
    price: f32,
    side: Order,
    size: u64,
    timestamp: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct WsArgs {
    pub op: String,
    pub args: Option<Vec<String>>,
}

impl WsArgs {
    pub fn into_msg(&self) -> Message {
        Message::text(serde_json::to_string(self).expect("Failed to serialize Args struct"))
    }
}

#[derive(Debug, Clone)]
pub struct API {
    pub key: String,
    pub secret: String,
}

#[derive(Clone)]
pub enum Endpoint {
    MAINNET,
    TESTNET,
}
