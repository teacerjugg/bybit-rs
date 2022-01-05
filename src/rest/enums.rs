pub enum Interval {
    OneMin,
    ThreeMin,
    FiveMin,
    FifteenMin,
    ThirtyMin,
    OneHour,
    TwoHour,
    FourHour,
    SixHour,
    TwelveHour,
    Day,
    Week,
    Month,
}

impl ToString for Interval {
    fn to_string(&self) -> String {
        String::from(match self {
            Interval::OneMin => "1",
            Interval::ThreeMin => "3",
            Interval::FiveMin => "5",
            Interval::FifteenMin => "15",
            Interval::ThirtyMin => "30",
            Interval::OneHour => "60",
            Interval::TwoHour => "120",
            Interval::FourHour => "240",
            Interval::SixHour => "360",
            Interval::TwelveHour => "720",
            Interval::Day => "D",
            Interval::Week => "W",
            Interval::Month => "M",
        })
    }
}

impl Into<String> for Interval {
    fn into(self) -> String {
        String::from(match self {
            Interval::OneMin => "1",
            Interval::ThreeMin => "3",
            Interval::FiveMin => "5",
            Interval::FifteenMin => "15",
            Interval::ThirtyMin => "30",
            Interval::OneHour => "60",
            Interval::TwoHour => "120",
            Interval::FourHour => "240",
            Interval::SixHour => "360",
            Interval::TwelveHour => "720",
            Interval::Day => "D",
            Interval::Week => "W",
            Interval::Month => "M",
        })
    }
}

pub enum Period {
    FiveMin,
    FifteenMin,
    ThirtyMin,
    OneHour,
    FourHour,
    OneDay,
}

impl ToString for Period {
    fn to_string(&self) -> String {
        String::from(match self {
            Period::FiveMin => "5min",
            Period::FifteenMin => "15min",
            Period::ThirtyMin => "30min",
            Period::OneHour => "1h",
            Period::FourHour => "4h",
            Period::OneDay => "1d",
        })
    }
}

impl Into<String> for Period {
    fn into(self) -> String {
        String::from(match self {
            Period::FiveMin => "5min",
            Period::FifteenMin => "15min",
            Period::ThirtyMin => "30min",
            Period::OneHour => "1h",
            Period::FourHour => "4h",
            Period::OneDay => "1d",
        })
    }
}

pub enum OrderType {
    Limit,
    Market,
}

impl ToString for OrderType {
    fn to_string(&self) -> String {
        String::from(match self {
            OrderType::Limit => "Limit",
            OrderType::Market => "Market",
        })
    }
}

pub enum TimeInForce {
    GoodTillCancel,
    ImmediateOrCancel,
    FillOrKill,
    PostOnly,
}

impl ToString for TimeInForce {
    fn to_string(&self) -> String {
        String::from(match self {
            TimeInForce::GoodTillCancel => "GoodTillCancel",
            TimeInForce::ImmediateOrCancel => "ImmediateOrCancel",
            TimeInForce::FillOrKill => "FillOrKill",
            TimeInForce::PostOnly => "PostOnly",
        })
    }
}

pub enum OrderStatus {
    Filled,
    New,
}

impl ToString for OrderStatus {
    fn to_string(&self) -> String {
        String::from(match self {
            OrderStatus::Filled => "Filled",
            OrderStatus::New => "New",
        })
    }
}

#[allow(dead_code)]
pub enum ReturnCode {
    /// ret_code
    /// Example:
    /// RestResponse {
    ///     ret_code: 33004,
    ///     ret_msg: "api_key expire",
    ///     ext_code: "",
    ///     ext_info: "",
    ///     result: Null,
    ///     timestamp: 2022-01-04T10:41:11.629Z,
    ///     rate_limit_status: None,
    ///     rate_limit_reset_ms: None,
    ///     rate_limit: None,
    /// }
    OK, // 0
    APIKeyExpired, // 33004
}

// pub enum Direction {
//     Prev,
//     Next,
// }
