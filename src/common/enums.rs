pub enum Symbol {
    BTCUSD,
}

impl ToString for Symbol {
    fn to_string(&self) -> String {
        String::from(match self {
            Symbol::BTCUSD => "BTCUSD",
        })
    }
}

impl Into<String> for Symbol {
    fn into(self) -> String {
        String::from(match self {
            Symbol::BTCUSD => "BTCUSD",
        })
    }
}
