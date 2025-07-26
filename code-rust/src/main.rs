// i.e. include date.rs file
mod date;

// So Date can be used directly 
// instead of writing date::Date everytime
use date::Date;


fn main(){
    let d = Date::new(14, 05, 2025);
    println!("Date: {}-{}-{}", d.day, d.month, d.year);

    // Add positive number
    let d1 = Date::new(14,05,1989);
    let d2 = d1 + 10;
    println!("d2: {}", d2);

    // Add a negative number
    let d1 = Date::new(14,05,1989);
    let d2 = d1 + (-10);
    println!("d2: {}", d2);

    // Sub a positive number
    let d1 = Date::new(14,05,1989);
    let d2 = d1 - 10;
    println!("d2: {}", d2);

    // Sub two dates
    let d1 = Date::new(14,05,1989);
    let d2 = Date::new(11,05,1989);
    let d3 = d1 - d2;
    println!("d3: {}", d3);
}