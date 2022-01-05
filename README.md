# Disclaimer

USE OF THIS LIBRARY IS AT YOUR OWN RISK.
YOU UNDERSTAND AND AGREE THAT YOU MAY LOSE YOUR MONEY AND YOU ACCEPT FULL RESPONSIBILITY FOR ANY SUCH LOSS.

本ライブラリの使用は自己責任となります．
金銭損失の可能性があり，すべての責任が自身にあることを理解・同意してください．

# Description

<!-- Bybit API library for Rust.   -->
<!-- This library is based on [pybybit](https://github.com/MtkN1/pybybit).   -->

Before using this, you may need to read:
- [Bybit API Document](https://bybit-exchange.github.io/docs/inverse/#t-introduction)

<!-- # Example -->

<!-- ```rust -->
<!-- use bybit_rs::prelude::*; -->
<!-- use bybit_rs::store; -->
<!-- use std::env; -->

<!-- #[tokio::main] -->
<!-- async fn main() { -->
<!--     let api: API = API { -->
<!--         key: env::var("API_KEY").unwrap(), -->
<!--         secret: env::var("API_SECRET").unwrap(), -->
<!--     }; -->

<!--     let mut ws = WebsocketBuilder::new() -->
<!--         .endpoint(Endpoint::MAINNET) -->
<!--         .api(api) -->
<!--         .build() -->
<!--         .await; -->
<!--     ws.subscribe().await?; -->

<!--     let _handle = tokio::spawn(async move { -->
<!--         ws.on_message().await.unwrap(); -->
<!--     }); -->
<!-- } -->
<!-- ``` -->

# ToDo
- [] すべてのREST APIに対応
- [] WebSocketとRESTをまとめたモジュール
- [] ReturnCodeによるErrorトレイト
- [] Orderの管理用struct
