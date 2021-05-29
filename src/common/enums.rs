pub enum Symbol {
    BTCUSD,
    ETHUSD,
    EOSUSD,
    XRPUSD,
    BTCUSDT,
    ETHUSDT,
    BCHUSDT,
    // LINKUSDT,
    // LTCUSDT,
    // XTZUSDT,
    // ADAUSDT,
    // DOTUSDT,
    // UNIUSDT,
    // AAVEUSDT,
    // SUSHIUSDT,
    // XRPUSDT,
    // XEMUSDT,
}

impl ToString for Symbol {
    fn to_string(&self) -> String {
        String::from(match self {
            Symbol::BTCUSD => "BTCUSD",
            Symbol::ETHUSD => "ETHUSD",
            Symbol::EOSUSD => "EOSUSD",
            Symbol::XRPUSD => "XRPUSD",
            Symbol::BTCUSDT => "BTCUSDT",
            Symbol::ETHUSDT => "ETHUSDT",
            Symbol::BCHUSDT => "BCHUSDT",
        })
    }
}

impl Into<String> for Symbol {
    fn into(self) -> String {
        String::from(match self {
            Symbol::BTCUSD => "BTCUSD",
            Symbol::ETHUSD => "ETHUSD",
            Symbol::EOSUSD => "EOSUSD",
            Symbol::XRPUSD => "XRPUSD",
            Symbol::BTCUSDT => "BTCUSDT",
            Symbol::ETHUSDT => "ETHUSDT",
            Symbol::BCHUSDT => "BCHUSDT",
        })
    }
}
