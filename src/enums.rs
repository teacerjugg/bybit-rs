#[derive(Clone)]
pub enum Endpoint {
    MAINNET,
    TESTNET,
}

#[derive(Debug)]
pub enum Order {
    Buy,
    Sell,
}
