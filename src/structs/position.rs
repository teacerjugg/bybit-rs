use super::serde_side;
use crate::enums::{Order, Status, Trigger};
use serde::{self, Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Position {
    pub user_id: usize,
    pub symbol: String,
    #[serde(with = "serde_side")]
    pub side: Order,
    pub size: usize,
    #[serde(deserialize_with = "deserialize_string_to_f32")]
    pub position_value: f32,
    #[serde(deserialize_with = "deserialize_string_to_f32")]
    pub entry_price: f32,
    #[serde(deserialize_with = "deserialize_string_to_f32")]
    pub liq_price: f32,
    #[serde(deserialize_with = "deserialize_string_to_f32")]
    pub bust_price: f32,
    #[serde(deserialize_with = "deserialize_string_to_u8")]
    pub leverage: u8,
    #[serde(deserialize_with = "deserialize_string_to_u8")]
    pub order_margin: u8,
    #[serde(deserialize_with = "deserialize_string_to_u8")]
    pub position_margin: u8,
    #[serde(deserialize_with = "deserialize_string_to_f32")]
    pub available_balance: f32,
    #[serde(deserialize_with = "deserialize_string_to_f32")]
    pub take_profit: f32,
    #[serde(with = "serde_trigger")]
    pub tp_trigger_by: Trigger,
    #[serde(deserialize_with = "deserialize_string_to_f32")]
    pub stop_loss: f32,
    #[serde(with = "serde_trigger")]
    pub sl_trigger_by: Trigger,
    #[serde(deserialize_with = "deserialize_string_to_f32")]
    pub realised_pnl: f32,
    #[serde(deserialize_with = "deserialize_string_to_f32")]
    pub trailing_stop: f32,
    #[serde(deserialize_with = "deserialize_string_to_f32")]
    pub trailing_active: f32,
    #[serde(deserialize_with = "deserialize_string_to_f32")]
    pub wallet_balance: f32,
    pub risk_id: usize,
    pub is_isolated: bool,
    #[serde(deserialize_with = "deserialize_string_to_f32")]
    pub occ_closing_fee: f32,
    #[serde(deserialize_with = "deserialize_string_to_f32")]
    pub occ_funding_fee: f32,
    pub auto_add_margin: u8,
    #[serde(deserialize_with = "deserialize_string_to_f32")]
    pub cum_realised_pnl: f32,
    #[serde(with = "serde_status")]
    pub position_status: Status,
    // pub position_seq: usize,
}

fn deserialize_string_to_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.parse::<f32>().unwrap())
}

fn deserialize_string_to_u8<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.parse::<u8>().unwrap())
}

mod serde_trigger {
    use crate::enums::Trigger;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(side: &Trigger, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match side {
            Trigger::LastPrice => "LastPrice",
            Trigger::IndexPrice => "IndexPrice",
            Trigger::None => "None",
        };
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Trigger, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "LastPrice" => Ok(Trigger::LastPrice),
            "IndexPrice" => Ok(Trigger::IndexPrice),
            "" => Ok(Trigger::None),
            _ => panic!("Impossible order side"),
        }
    }
}

mod serde_status {
    use crate::enums::Status;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(side: &Status, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match side {
            Status::Normal => "Normal",
            Status::Liquidation => "Liquidation",
            Status::AutoDeleveraging => "Auto-Deleveraging",
        };
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Status, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Normal" => Ok(Status::Normal),
            "Liq" => Ok(Status::Liquidation),
            "Adl" => Ok(Status::AutoDeleveraging),
            _ => panic!("Impossible order side"),
        }
    }
}
