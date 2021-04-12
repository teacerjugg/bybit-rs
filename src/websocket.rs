pub use crate::enums::{Endpoint, Topic};
use crate::structs::WsArgs;
pub use crate::structs::API;

use async_tungstenite::{
    async_std::{connect_async, ConnectStream},
    tungstenite::{
        http::{Response, StatusCode},
        Error, Result,
    },
    WebSocketStream,
};

use crate::store;
use chrono::{DateTime, Utc};
use futures::{SinkExt, StreamExt};
use hmac::{Hmac, Mac, NewMac};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

type HmacSha256 = Hmac<Sha256>;
type WSConnection = WebSocketStream<ConnectStream>;

pub struct Websocket {
    endpoint: Endpoint,
    api: API,
    ws_stream: WSConnection,
}

pub struct WebsocketBuilder<EndpointType, ApiType, WsType> {
    endpoint: EndpointType,
    api: ApiType,
    ws_stream: WsType,
}

impl WebsocketBuilder<(), (), ()> {
    pub fn new() -> Self {
        WebsocketBuilder {
            endpoint: (),
            api: (),
            ws_stream: (),
        }
    }
}

impl<WsType> WebsocketBuilder<Endpoint, API, WsType> {
    pub async fn build(self) -> Websocket {
        Websocket {
            endpoint: self.endpoint.clone(),
            api: self.api.clone(),
            ws_stream: self
                .connect()
                .await
                .expect("Failed to connect Websocket API"),
        }
    }

    async fn connect(self) -> Result<WSConnection> {
        let url: Url = match self.endpoint {
            Endpoint::MAINNET => Url::parse("wss://stream.bybit.com/realtime").unwrap(),
            // Url::parse("wss://ws_stream.bytick.com/realtime").unwrap();
            Endpoint::TESTNET => Url::parse("wss://stream-testnet.bybit.com/realtime").unwrap(),
        };

        let (ws_stream, _) = connect_async(url).await?;
        info!("Connected to websocket API");

        Ok(ws_stream)
    }
}

impl<EndpointType, ApiType, WsType> WebsocketBuilder<EndpointType, ApiType, WsType> {
    pub fn endpoint(self, endpoint: Endpoint) -> WebsocketBuilder<Endpoint, ApiType, WsType> {
        WebsocketBuilder {
            endpoint: endpoint,
            api: self.api,
            ws_stream: self.ws_stream,
        }
    }

    pub fn api(self, api: API) -> WebsocketBuilder<EndpointType, API, WsType> {
        WebsocketBuilder {
            endpoint: self.endpoint,
            api: api,
            ws_stream: self.ws_stream,
        }
    }
}

impl Websocket {
    pub async fn connect(&mut self) -> Result<()> {
        let url: Url = match self.endpoint {
            Endpoint::MAINNET => Url::parse("wss://stream.bybit.com/realtime").unwrap(),
            // Url::parse("wss://ws_stream.bytick.com/realtime").unwrap();
            Endpoint::TESTNET => Url::parse("wss://stream-testnet.bybit.com/realtime").unwrap(),
        };

        let (ws_stream, _) = connect_async(url).await?;
        info!("Connected to websocket");

        self.ws_stream = ws_stream;

        Ok(())
    }

    pub async fn authenticate(&mut self) -> Result<()> {
        let now = SystemTime::now();
        let unix_time = now.duration_since(UNIX_EPOCH).expect("back to the future");
        let expires = (unix_time.as_secs() + 10) * 1000;

        let mut mac = HmacSha256::new_varkey(self.api.secret.as_bytes()).unwrap();
        mac.update(b"GET/realtime");
        mac.update(expires.to_string().as_bytes());
        let signature = format!("{:x}", mac.finalize().into_bytes());

        let auth = WsArgs {
            op: "auth".to_owned(),
            args: Some([self.api.key.clone(), expires.to_string(), signature].to_vec()),
        };
        debug!("{}", serde_json::to_string(&auth).unwrap());

        self.ws_stream.send(auth.into_msg()).await?;
        info!("Sent authentication message");

        match self.ws_stream.next().await {
            Some(msg) => {
                let msg = msg?;
                let msg_json: Value = serde_json::from_str(&msg.into_text().unwrap()).unwrap();
                debug!("{:?}", &msg_json);

                match msg_json["success"] {
                    Value::Bool(true) => {
                        info!("Authentication successful");
                        Ok(())
                    }
                    _ => {
                        error!("Websocket Authentication Failed");
                        Err(Error::Http(
                            Response::builder()
                                .status(StatusCode::UNAUTHORIZED)
                                .body(Some("Authentication Failed".to_owned()))
                                .unwrap(),
                        ))
                    }
                }
            }
            None => Err(Error::Http(
                Response::builder()
                    .status(StatusCode::NO_CONTENT)
                    .body(Some("Nothing returned".to_owned()))
                    .unwrap(),
            )),
        }
    }

    pub async fn subscribe(&mut self, topics: Vec<Topic>) -> Result<()> {
        let subscribe = WsArgs {
            op: "subscribe".to_owned(),
            args: Some(topics.into_iter().map(|t| t.into_string()).collect()),
        };

        self.ws_stream.send(subscribe.into_msg()).await?;
        info!("Sent subscribe message");

        Ok(())
    }

    pub async fn ping(&mut self) -> Result<()> {
        let ping = WsArgs {
            op: "ping".to_owned(),
            args: None,
        };

        self.ws_stream.send(ping.into_msg()).await?;
        debug!("Sent ping");

        Ok(())

        // match self.ws_stream.next().await {
        //     Some(msg) => {
        //         let msg = msg?;
        //         let msg_json: Value = serde_json::from_str(&msg.into_text().unwrap()).unwrap();
        //         debug!("{:?}", &msg_json);

        //         match msg_json["success"] {
        //             Value::Bool(true) => {
        //                 debug!("Ping successful");
        //                 Ok(())
        //             }
        //             _ => {
        //                 error!("Ping Failed");
        //                 Err(Error::Http(
        //                     Response::builder()
        //                         .body(Some("Ping Failed".to_owned()))
        //                         .unwrap(),
        //                 ))
        //             }
        //         }
        //     }
        //     None => Err(Error::Http(
        //         Response::builder()
        //             .status(StatusCode::NO_CONTENT)
        //             .body(Some("Nothing returned".to_owned()))
        //             .unwrap(),
        //     )),
        // }
    }

    pub async fn on_message(&mut self) -> Result<()> {
        if let Some(msg) = self.ws_stream.next().await {
            let msg = msg?;

            let msg_json: WebsocketResponse =
                match serde_json::from_str::<WebsocketResponse>(msg.to_text().unwrap()) {
                    Ok(res) => res,
                    Err(_) => {
                        if let Ok(res) = serde_json::from_str::<Value>(msg.to_text().unwrap()) {
                            if res["success"] == Value::Bool(true) {
                                if res["ret_msg"] == "pong" {
                                    debug!("Ping successful");
                                } else {
                                    info!("Subscription successful");
                                }
                            }
                            return Ok(());
                        } else {
                            error!("Subscription Failed: the subscribed topics may are invalid");
                            return Err(Error::Http(
                                Response::builder()
                                    .body(Some("Subscription Failed".to_owned()))
                                    .unwrap(),
                            ));
                        }
                    }
                };
            store::store_message(msg_json);
        }

        Ok(())
    }

    pub async fn run_forever(mut self) {
        tokio::spawn(async move {
            self.ping().await.expect("Ping Failed");
            let mut now = SystemTime::now();

            loop {
                if let Ok(elapsed) = now.elapsed() {
                    if elapsed.as_secs() >= 60 {
                        self.ping().await.expect("Ping Failed");
                        now = SystemTime::now();
                    }
                }

                self.on_message().await.unwrap();
            }
        });
        // .await
        // .unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebsocketResponse {
    pub topic: String,
    #[serde(default)]
    #[serde(rename(deserialize = "type", serialize = "type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<String>,
    #[serde(default)]
    #[serde(rename(deserialize = "timestamp_e6"))]
    #[serde(with = "serde_option_timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
    pub data: Value,
}

mod serde_option_timestamp {
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
