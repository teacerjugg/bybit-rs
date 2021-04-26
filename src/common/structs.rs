use reqwest::Url;
use std::collections::BTreeMap;

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

impl Endpoint {
    pub fn to_uri_with_params(&self, params: BTreeMap<String, String>) -> Url {
        match self {
            Endpoint::MAINNET => Url::parse_with_params("https://api.bybit.com", params).unwrap(),
            Endpoint::TESTNET => {
                Url::parse_with_params("https://api-testnet.bybit.com", params).unwrap()
            }
        }
    }

    pub fn to_uri(&self) -> Url {
        match self {
            Endpoint::MAINNET => Url::parse("https://api.bybit.com").unwrap(),
            Endpoint::TESTNET => Url::parse("https://api-testnet.bybit.com").unwrap(),
        }
    }
}
