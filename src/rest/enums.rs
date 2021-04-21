use url::Url;

pub enum Endpoint {
    MAINNET,
    TESTNET,
}

impl Endpoint {
    pub fn to_uri(&self) -> Url {
        match self {
            Endpoint::MAINNET => Url::parse("https:/api.bybit.com").unwrap(),
            Endpoint::TESTNET => Url::parse("https://api-testnet.bybit.com").unwrap(),
        }
    }
}
