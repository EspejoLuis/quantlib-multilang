// This line tells the compiler to make the date module public
// so integration tests can say: use code_rust::date::Date;

pub mod time {
    pub mod date;
    pub mod frequency;
    pub mod period;
    pub mod time_unit;
    pub mod weekday;
}
mod utilities {
    pub mod dateformatter;
}

pub mod io {
    // date formatting
    pub use crate::time::date::io::{iso_date, long_date, short_date};
    // weekday formatting
    pub use crate::time::weekday::io::{long_weekday, short_weekday, shortest_weekday};
    // dataformatters (ordinal, percent, etc.)
    pub use crate::utilities::dateformatter::io::ordinal;
}
