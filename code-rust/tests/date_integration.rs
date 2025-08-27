/*
This imports the type Date into your current scope.
So now, instead of
    let d = code_rust::date::Date::new(…);
we can use
    let d = Date::new(…);
*/
use code_rust::time::date::Date;
use code_rust::time::date::Month;

#[test]
fn serial_conversion_works_correctly() {
    let expected_date: Date = Date::new(14, Month::July, 1989);
    let serial_date: i32 = expected_date.to_serial_number();
    let derived_date: Date = Date::from_serial_number(serial_date);

    assert_eq!(expected_date, derived_date);
}

#[test]
fn add_then_subtract_returns_original_date_correctly() {
    let expected_date: Date = Date::new(12, Month::November, 1989);
    /*
    Had to implement by reference add function (&)
    by value was not enough.

        let a = Date::new(1, 1, 2020);
        let c = &a + 10;  // borrow `a`

        let b = Date::new(1, 1, 2020);
        let a = b;  // ownership of `b` moved into `a`
    */
    let date_add_days: Date = expected_date + 40;
    let date_subtract_days: Date = date_add_days - 40;

    assert_eq!(expected_date, date_subtract_days);
}
