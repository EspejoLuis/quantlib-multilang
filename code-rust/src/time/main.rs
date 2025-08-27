// i.e. include date.rs file
mod date;
mod dateformatter;
mod frequency;
mod period;
mod time_unit;
mod weekday;

// So Date can be used directly
// instead of writing date::Date everytime
use date::Date;

fn main() {
    let d = Date::new(35, date::Month::August, 2025);
    println!("Date: {}-{}-{}", d.day(), d.month(), d.year());

    let d = Date::new(0, date::Month::March, 2025);
    println!("Date: {}-{}-{}", d.day(), d.month(), d.year());
}
