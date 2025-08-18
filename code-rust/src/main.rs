// i.e. include date.rs file
mod date;

// So Date can be used directly
// instead of writing date::Date everytime
use date::Date;

fn main() {
    let d = Date::new(35, date::Month::August, 2025);
    println!("Date: {}-{}-{}", d.day(), d.month(), d.year());
}
