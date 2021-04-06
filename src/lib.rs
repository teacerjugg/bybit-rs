pub mod enums;
pub mod store;
pub mod structs;
pub mod websocket;

// #[tokio::main]
// async fn main() {
//     dotenv().ok();
//     // env_logger::builder().format_timestamp(None).init();
//     env_logger::init();
//     let mut ws_stream = connect().await.unwrap();
//     ws_stream = subscribe(ws_stream).await.unwrap();
//     ws_stream.close(None).await.ok().unwrap();

// let (write, read) = ws_stream.split();

// let ws_to_stdout = {
//     read.for_each(|msg| async {
//         let data = msg.unwrap().into_data();
//         tokio::io::stdout().write_all(&data).await.unwrap();
//     })
// };

// pin_mut!(ws_to_stdout);
// future::select(ws_to_stdout).await;

// connect(url, |ws| {
//     ws.send(format!(
//         "{{'op':'auth', 'args':['{api_key}', '{expires}', '{signature}']}}",
//         api_key = api.key,
//         expires = expires,
//         signature = signature
//     )).unwrap();
//     // ws.send("{{'op':'subscribe', 'args':['trade.BTCUSD', 'orderBook_200.100ms.BTCUSD']}}")
//     //     .unwrap();

//     move |msg| {
//         println!("Got message: {:?}", msg);
//         ws.close(CloseCode::Normal)
//     }
// })
// .unwrap()

pub mod serde_timestamp {
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

pub mod serde_option_timestamp {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.unwrap().to_rfc3339();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = i64::deserialize(deserializer)?;
        Ok(Some(Utc.timestamp_nanos(s * 10_i64.pow(3))))
    }
}

pub mod serde_record_timestamp {
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
