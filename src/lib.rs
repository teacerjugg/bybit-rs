// #![allow(unused_imports)]
// #![allow(dead_code)]
// #![feature(once_cell)]
// use dotenv::dotenv;
// use hex_literal::hex;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use std::env;
// use std::lazy::Lazy;
use std::time::{SystemTime, UNIX_EPOCH};
// use ws::{connect, CloseCode};
use futures::{SinkExt, StreamExt};
use log::{debug, error, info};
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio_tungstenite::{
//     connect_async,
//     tungstenite::protocol::Message,
//     tungstenite::{Error, Result},
// };
// use async_std::io;
// use async_std::prelude::*;
// use async_std::task;
use async_tungstenite::{
    async_std::{connect_async, ConnectStream},
    tungstenite::{
        http::{Response, StatusCode},
        protocol::Message,
        Error, Result,
    },
    WebSocketStream,
};
use serde::Serialize;
use serde_json::Value;
use url::Url;

// #[macro_use]
// extern crate once_cell;
// use once_cell::sync::Lazy;

type HmacSha256 = Hmac<Sha256>;
type WSConnection = WebSocketStream<ConnectStream>;

#[derive(Serialize, Debug)]
struct WsArgs {
    op: String,
    args: Vec<String>,
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

// static API: Lazy<API> = Lazy::new(|| {
//     dotenv().ok();
//     API {
//         key: env::var("API_KEY").unwrap(),
//         secret: env::var("API_SECRET").unwrap(),
//     }
// });

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
    let signature = format!(
        "{:x}",
        mac.finalize().into_bytes() // Sha256::new()
                                    //     .chain(api.secret.as_bytes())
                                    //     .chain(b"GET/realtime")
                                    //     .chain(expires.to_owned())
                                    //     .finalize()
    );

    let (mut ws_stream, _) = connect_async(url).await?;
    info!("Connected to websocket");

    let auth = WsArgs {
        op: "auth".to_owned(),
        args: [api.key, expires.to_string(), signature].to_vec(),
    };
    debug!("{}", serde_json::to_string(&auth).unwrap());

    ws_stream.send(auth.into_msg()).await?;
    info!("Sent authentication message");

    // while let Some(msg) = ws_stream.next().await {
    //     let msg = msg?;
    //     println!("{:?}", msg.into_text().unwrap()); //.parse().unwrap());
    // }

    if let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        let msg_json: Value = serde_json::from_str(&msg.into_text().unwrap()).unwrap();
        debug!("{:?}", &msg_json);

        match msg_json["success"] {
            Value::Bool(true) => info!("Authentication successful"),
            _ => {
                error!("Websocket Authentication Failed");
                return Err(Error::Http(
                    Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(Some("Authentication Failed".to_owned()))
                        .unwrap(),
                ));
            }
        }
    }

    Ok(ws_stream)
}

pub async fn subscribe(mut ws_stream: WSConnection) -> Result<WSConnection> {
    let subscribe = WsArgs {
        op: "subscribe".to_owned(),
        args: [
            "orderBookL2_25.BTCUSD".to_owned(),
            "trade.BTCUSD".to_owned(),
        ]
        .to_vec(),
    };

    ws_stream.send(subscribe.into_msg()).await?;
    info!("Sent subscribe message");

    if let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        let msg_json: Value = serde_json::from_str(&msg.into_text().unwrap()).unwrap();
        debug!("{:?}", msg_json);
        // if msg_json["success"] == "true" {
        //     info!("Authentication successful")
        // } else {
        //     return Err(Error::Http(
        //         Response::builder()
        //             .status(StatusCode::UNAUTHORIZED)
        //             .body(Some("Authentication Failed".to_owned()))
        //             .unwrap(),
        //     ));
        // }
    }

    Ok(ws_stream)
}

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
