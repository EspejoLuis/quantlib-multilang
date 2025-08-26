use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum TimeUnit {
    Days,
    Weeks,
    Months,
    Years,
    Hours,
    Minutes,
    Seconds,
    Milliseconds,
    Microseconds,
}

impl fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let time_unit: &'static str = match self {
            TimeUnit::Days => "Days",
            TimeUnit::Weeks => "Weeks",
            TimeUnit::Months => "Months",
            TimeUnit::Years => "Years",
            TimeUnit::Hours => "Hours",
            TimeUnit::Minutes => "Minutes",
            TimeUnit::Seconds => "Seconds",
            TimeUnit::Milliseconds => "Milliseconds",
            TimeUnit::Microseconds => "Microseconds",
        };
        write!(f, "{}", time_unit)
    }
}
