mod instrument;
mod orderbook;
mod position;
mod record;

pub use instrument::Instrument;
pub use orderbook::{Limit, OrderBook};
pub use position::Position;
pub use record::Record;

use async_tungstenite::tungstenite::protocol::Message;
use serde::{self, Serialize};

#[derive(Serialize, Debug)]
pub(crate) struct WsArgs {
    pub op: String,
    pub args: Option<Vec<String>>,
}

impl WsArgs {
    pub fn into_msg(self) -> Message {
        Message::text(serde_json::to_string(&self).expect("Failed to serialize Args struct"))
    }
}

pub(crate) mod serde_side {
    use crate::common::enums::Order;
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

pub(crate) mod serde_tick {
    use crate::websocket::enums::Tick;
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
