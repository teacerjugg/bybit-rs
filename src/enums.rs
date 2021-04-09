#[derive(Clone)]
pub enum Endpoint {
    MAINNET,
    TESTNET,
}

#[derive(Clone, Debug)]
pub enum Order {
    Buy,
    Sell,
}

#[derive(Debug)]
pub enum Tick {
    MinusTick,
    ZeroMinusTick,
    PlusTick,
    ZeroPlusTick,
}
