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

// pub enum OrderStatus {
//     Filled,
//     New,
// }

// pub enum Direction {
//     Prev,
//     Next,
// }
