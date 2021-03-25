use async_tungstenite::{
    async_std::{connect_async, ConnectStream},
    tungstenite::{
        http::{Response, StatusCode},
        protocol::Message,
        Error, Result,
    },
    WebSocketStream,
};
use futures::{SinkExt, StreamExt};
use hmac::{Hmac, Mac, NewMac};
use log::{debug, error, info};
use serde::Serialize;
use serde_json::Value;
use sha2::Sha256;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

type HmacSha256 = Hmac<Sha256>;
type WSConnection = WebSocketStream<ConnectStream>;

#[derive(Serialize, Debug)]
struct WsArgs {
    op: String,
    args: Option<Vec<String>>,
}

impl WsArgs {
    fn into_msg(&self) -> Message {
        Message::text(serde_json::to_string(self).unwrap())
    }
}

#[derive(Debug)]
struct API {
    key: String,
    secret: String,
}

pub async fn connect() -> Result<WSConnection> {
    let url: Url = Url::parse("wss://stream.bybit.com/realtime").unwrap();
    // let url: Url = Url::parse("wss://stream.bytick.com/realtime").unwrap();
    // let url: Url = Url::parse("wss://stream-testnet.bybit.com/realtime").unwrap();
    let api = API {
        key: env::var("API_KEY").unwrap(),
        secret: env::var("API_SECRET").unwrap(),
    };

    let now = SystemTime::now();
    let unix_time = now.duration_since(UNIX_EPOCH).expect("back to the future");
    let expires = (unix_time.as_secs() + 10) * 1000;

    let mut mac = HmacSha256::new_varkey(api.secret.as_bytes()).unwrap();
    mac.update(b"GET/realtime");
    mac.update(expires.to_string().as_bytes());
    let signature = format!("{:x}", mac.finalize().into_bytes());

    let (mut ws_stream, _) = connect_async(url).await?;
    info!("Connected to websocket");

    let auth = WsArgs {
        op: "auth".to_owned(),
        args: Some([api.key, expires.to_string(), signature].to_vec()),
    };
    debug!("{}", serde_json::to_string(&auth).unwrap());

    ws_stream.send(auth.into_msg()).await?;
    info!("Sent authentication message");

    match ws_stream.next().await {
        Some(msg) => {
            let msg = msg?;
            let msg_json: Value = serde_json::from_str(&msg.into_text().unwrap()).unwrap();
            debug!("{:?}", &msg_json);

            match msg_json["success"] {
                Value::Bool(true) => {
                    info!("Authentication successful");
                    Ok(ws_stream)
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
        },
        None => Err(Error::Http(
            Response::builder()
                .status(StatusCode::NO_CONTENT)
                .body(Some("Nothing returned".to_owned()))
                .unwrap(),
        )),
    }
}

pub async fn subscribe(ws_stream: &mut WSConnection) -> Result<&mut WSConnection> {
    let subscribe = WsArgs {
        op: "subscribe".to_owned(),
        args: Some([
            "orderBook_200.100ms.BTCUSD".to_owned(),
            "trade.BTCUSD".to_owned(),
        ]
        .to_vec()),
    };

    ws_stream.send(subscribe.into_msg()).await?;
    info!("Sent subscribe message");

    match ws_stream.next().await {
        Some(msg) => {
            let msg = msg?;
            let msg_json: Value = serde_json::from_str(&msg.into_text().unwrap()).unwrap();
            debug!("{:?}", &msg_json);

            match msg_json["success"] {
                Value::Bool(true) => {
                    info!("Subscription successful");
                    Ok(ws_stream)
                }
                _ => {
                    error!("Subscription Failed: the subscribed topics may are invalid");
                    Err(Error::Http(
                        Response::builder()
                            .body(Some("Subscription Failed".to_owned()))
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

pub async fn ping(ws_stream: &mut WSConnection) -> Result<&mut WSConnection> {
    let ping = WsArgs {
        op: "ping".to_owned(),
        args: None,
    };

    ws_stream.send(ping.into_msg()).await?;
    debug!("Sent ping");

    match ws_stream.next().await {
        Some(msg) => {
            let msg = msg?;
            let msg_json: Value = serde_json::from_str(&msg.into_text().unwrap()).unwrap();
            debug!("{:?}", &msg_json);

            match msg_json["success"] {
                Value::Bool(true) => {
                    debug!("Ping successful");
                    Ok(ws_stream)
                }
                _ => {
                    error!("Ping Failed");
                    Err(Error::Http(
                        Response::builder()
                            .body(Some("Ping Failed".to_owned()))
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
