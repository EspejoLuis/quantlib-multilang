/*
This imports the type Date into your current scope. 
So now, instead of
    let d = code_rust::date::Date::new(…);
we can use
    let d = Date::new(…);
*/
use code_rust::date::Date;

#[test]
fn serial_conversion_works_correctly(){
    let expected_date = Date::new(14, 5, 1989);
    let serial_date = expected_date.to_serial();
    let derived_date = Date::from_serial(serial_date);

    assert_eq!(expected_date, derived_date);

}

#[test]
fn add_then_subtract_returns_original_date_correctly(){
    let expected_date = Date::new(14, 5, 1989);

    let date_add_days = expected_date + 40;
    let date_subtract_days = date_add_days - 40;

    assert_eq!(expected_date, date_subtract_days);
}