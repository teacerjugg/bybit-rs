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

impl Default for Tick {
    fn default() -> Self {
        Tick::MinusTick
    }
}

pub enum Topic {
    OrderBook25,
    OrderBook200,
    Trade,
    // Insurance,
    Instrument,
    KLine,
    Position,
    Execution,
    Order,
    StopOrder,
}

impl Topic {
    pub fn into_string(self) -> String {
        String::from(match self {
            Topic::OrderBook25 => "orderBookL2_25.BTCUSD",
            Topic::OrderBook200 => "orderBook_200.100ms.BTCUSD",
            Topic::Trade => "trade.BTCUSD",
            Topic::Instrument => "instrument_info.100ms.BTCUSD",
            Topic::KLine => "klineV2.1.BTCUSD",
            Topic::Position => "position",
            Topic::Execution => "execution",
            Topic::Order => "order",
            Topic::StopOrder => "stop_order",
        })
    }
}
