#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)] // makes it laid out exactly as 1–7
pub enum Weekday {
    Monday = 1,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

use std::fmt;

impl fmt::Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // `&'static str` means a reference to a string literal that
        // lives for the entire program (string literals never expire).
        // Example: "Jan" is compiled into the binary and always safe.
        write!(f, "{}", self.long_weekday())
    }
}
impl Weekday {
    fn long_weekday(&self) -> &'static str {
        match self {
            Weekday::Monday => "Monday",
            Weekday::Tuesday => "Tuesday",
            Weekday::Wednesday => "Wednesday",
            Weekday::Thursday => "Thursday",
            Weekday::Friday => "Friday",
            Weekday::Saturday => "Saturday",
            Weekday::Sunday => "Sunday",
        }
    }

    fn short_weekday(&self) -> &'static str {
        match self {
            Weekday::Monday => "Mon",
            Weekday::Tuesday => "Tue",
            Weekday::Wednesday => "Wed",
            Weekday::Thursday => "Thu",
            Weekday::Friday => "Fri",
            Weekday::Saturday => "Sat",
            Weekday::Sunday => "Sun",
        }
    }

    fn shortest_weekday(&self) -> &'static str {
        match self {
            Weekday::Monday => "Mo",
            Weekday::Tuesday => "Tu",
            Weekday::Wednesday => "We",
            Weekday::Thursday => "Th",
            Weekday::Friday => "Fr",
            Weekday::Saturday => "Sa",
            Weekday::Sunday => "Su",
        }
    }
}
