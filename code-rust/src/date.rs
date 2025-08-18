/*
1. Copy
Means values of this type can be copied bit-for-bit instead of moved.
For Month, that’s fine: it’s just a tiny integer under the hood (the discriminant).

Effect: you can do:
let m1 = Month::March;
let m2 = m1;      // this makes a *copy*, not a move
let m3 = m1;      // ❌ still works, m1 is still valid
If Month were not Copy, the assignment would “move” it, and m1 couldn’t be used anymore.

2. Clone
Gives you a .clone() method that makes an explicit copy.
Normally, Clone can mean deep copies (like duplicating a vector).
For a Copy type like Month, .clone() just does the same as assignment.

Example:
let m1 = Month::April;
let m2 = m1.clone();  // same as m1

*/
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Month {
    January = 1,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

/*
Since the struct contains types that implement equality
(u32) then Rust automatically generates == logic
i.e. each field is compared in ORDER (day, month, year)
if all fiels are equal then true
*/

/*
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]:
    - PartialEq gives == and != logic
    - Eq: does not give anything more but
        confirms that == logic behaves mathematically sensibly
        For example a == a can be false if a is Nan. By adding Eq
        that possibility is excluded a priori.
    - PartialOrd --> Enables <, <=, >, >=
    - Ord --> Enables full ordering (like sorting)
    - Using asser_eq!(d1, d1, "xxx") mean Rust will try to show the
    value when the test fails but to do that `Debug` is needed
*/

impl Month {
    // Because not every u32 is valid (e.g., 0 or 13), the function should return an Option:
    // If n is between 1 and 12 → return Some(Month::X)
    // Otherwise → return None
    pub fn from_i32(number: i32) -> Option<Month> {
        match number {
            1 => Some(Month::January),
            2 => Some(Month::February),
            3 => Some(Month::March),
            4 => Some(Month::April),
            5 => Some(Month::May),
            6 => Some(Month::June),
            7 => Some(Month::July),
            8 => Some(Month::August),
            9 => Some(Month::September),
            10 => Some(Month::October),
            11 => Some(Month::November),
            12 => Some(Month::December),
            _ => None,
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
/*
This can be done!
    if date1 == date2 {
        println!("Dates are equal!");
    }
*/
pub struct Date {
    // Defines a struct named Date, just like a class in C++ or C# with only data (no methods yet).
    // pub --> public so they can be access by other files like main.rs
    // unsigned 32-bit integer
    serial_number: i32, //private by default
                        // Order below impact for example how operator < works.
                        // First field to be check is the first one below.
                        //pub year: u32,
                        //pub month: Month,
                        //pub day: u32,
}

impl Date {
    // Days from 1-Jan to start of each month
    pub const MONTH_OFFSET: [i32; 13] =
        [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365];
    pub const MONTH_LEAP_OFFSET: [i32; 13] =
        [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366];

    pub fn new(day: i32, month: Month, year: i32) -> Date {
        // Implementation block i.e. to have a constructor

        /*
        This uses Rust's field init shorthand:
        since the parameter names (defined in new)
        match the field names (define in Date)
        no need to write:

            Date {
                day: day,
                month: month,
                year: year,
            }
            Date { day, month, year }
        */
        Date::from_serial_number(day + Date::month_offset(year, month) + Date::year_offset(year))
    }

    /*
    Rust does not have built-in calendar logic in std
        - like Python’s datetime
        - or C#’s DateTime.
    To make operator + work, two methods need to be implemented:
        - A method to_serial_number() that gives an integer day count.
        - A method from_serial_number(n: i32) -> Date that builds a date from that count.
    */

    pub fn from_serial_number(n: i32) -> Date {
        Date { serial_number: n }
    }

    pub fn to_serial_number(&self) -> i32 {
        self.serial_number
    }

    fn year_offset(year: i32) -> i32 {
        /*
        Returns the offset in days from 1900-01-01 up
        to year-01-01 using the closed-form formula.

        Example:
        - year_offset(1900) = 0
        - year_offset(1901) = 365
        - year_offset(1903) = 365 + 365 + 366 = 1096
        */
        let y: i32 = (year as i32) - 1900;
        // JUST INTEGERS!
        //y * 365 base days assuming all years normal.
        //+ (y+3)/4 add leap days (every 4 years).
        //- (y+99)/100 remove century years that aren not leap (every century)
        //+ (y+399)/400 add back 400-multiples (leap again) (every 400 years)
        y * 365 + (y + 3) / 4 - (y + 99) / 100 + (y + 399) / 400
    }

    fn month_offset(year: i32, month: Month) -> i32 {
        /*
        Returns the offset in days from the start of `year` (i.e. Jan-01)
        up to, but not including, `year`-`month`-01

        Examples:
        - month_offset(1901, January) = 0
        - month_offset(2017, February) = 31
        - month_offset(1904, March) = 31 + 29 = 60
        */
        let days: i32 = if Date::is_leap(year) {
            Date::MONTH_LEAP_OFFSET[month as usize - 1]
        } else {
            Date::MONTH_OFFSET[month as usize - 1]
        };

        days
    }

    pub fn is_leap(year: i32) -> bool {
        (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
    }

    pub fn days_in_month(month: Month, year: i32) -> i32 {
        match month {
            Month::January => 31,
            Month::February => {
                if Date::is_leap(year) {
                    29
                } else {
                    28
                }
            }
            Month::March => 31,
            Month::April => 30,
            Month::May => 31,
            Month::June => 30,
            Month::July => 31,
            Month::August => 31,
            Month::September => 30,
            Month::October => 31,
            Month::November => 30,
            Month::December => 31,
        }
    }

    pub fn days_in_year(year: i32) -> i32 {
        if Date::is_leap(year) { 366 } else { 365 }
    }

    //Getters
    pub fn day(&self) -> i32 {
        /*
        Decode `day` from `serial_number` using QuantLib-style year_offset formula.
        1. Decode the year from serial_number using Date::year().
        2. Decode the month from serial_number using Date::month().
        3. Compute day_of_year = serial_number - year_offset(year).
        4. Compute day_of_month = day_of_year - month_offset(year, month).

        Example:
        - serial_number = 36 (1900-02-05)
            → year = 1900
            → month = February
            → day_of_year = 36 - 0 = 36
            → month_offset(1900, Feb) = 31
            → day = 36 - 31 = 5 ✅
        */
        let year: i32 = Date::year(&self);
        let month: Month = Date::month(&self);
        let days_of_month: i32 =
            self.serial_number - Date::month_offset(year, month) - Date::year_offset(year);
        days_of_month
    }

    pub fn month(&self) -> Month {
        /*
        Decode `month` from `serial_number` using QuantLib-style year_offset formula.
        1. Find the year with Date::year().
        2. Compute day_of_year = serial_number - year_offset(year).
           Example: if serial_number = 36 in 1900 → day_of_year = 36 - 0 = 36.
        3. Select the correct offset table:
           - MONTH_OFFSET for normal years
           - MONTH_LEAP_OFFSET for leap years
        4. Iterate over the table:
           - Find largest month m where offset[m] <= day_of_year
           - That m is the month
           Example: day_of_year = 36, not leap → February (since offset[2]=31 ≤ 36 < 59).

        If serial_number = 36 (i.e. 1900-02-05):
        year = 1900
        day_of_year = 36
        offsets = [0,31,59,…] (non-leap)
        loop runs until candidate_month = 3 (March overshoot), then -1 gives February ✅
        Compute on Demand !
        */
        let year: i32 = Date::year(&self);
        let days_from_start: i32 = self.serial_number as i32 - Date::year_offset(year); //Number of days from start year
        let mut candidate_month: i32 = 1;
        while (Date::month_offset(
            year,
            Month::from_i32(candidate_month).expect("Internal logic error: invalid month index"),
        ) as i32)
            <= days_from_start
        {
            candidate_month += 1;
        }

        Month::from_i32(candidate_month - 1).unwrap()
    }

    pub fn year(&self) -> i32 {
        /*
        Decode `year` from `serial_number` using QuantLib-style year_offset formula.
        Algorithm:
        1. Make a first guess: 1900 + serial_number/365.
        2. Adjust upward if year_offset(candidate+1) <= serial_number.
        3. Adjust downward if year_offset(candidate) > serial_number.

        Example:
        - serial_number = 73049
            → initial guess = 1900 + 73049/365 = 2100
            → year_offset(2101) = 73400 > 73049 -> No `while`
            → year_offset(2100) = 73035 ≤ 73049 -> No `while`
        → result = 2100.

        Computed on Demand!
        */
        let mut candidate_year: i32 = 1900 + self.serial_number / 365;

        // If Guess correct both 'while' will not be entered
        while Date::year_offset(candidate_year + 1) <= self.serial_number {
            candidate_year += 1;
        }

        while Date::year_offset(candidate_year) > self.serial_number {
            candidate_year -= 1;
        }
        candidate_year as i32
    }
}

use std::fmt;

static MONTH_NAMES: [&str; 13] = [
    "", "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

impl fmt::Display for Month {
    // You need to dereference (*self) because self is a reference i.e. pointer i.e. address of a value (&Month).
    // Only the value (Month) can be cast to its numeric discriminant.
    // A reference (&Month) can’t be directly cast to a number
    // — otherwise you’d be casting the pointer address, not the enum value
    fn fmt(&self, formatter_buffer: &mut fmt::Formatter) -> fmt::Result {
        let index: usize = *self as usize; // convert enum discriminant to usize
        write!(formatter_buffer, "{}", MONTH_NAMES[index])
    }
}
/*
- impl: we are implementing something.
- fmt::Display: this is a TRAIT:
    - like an interface in C# or abstract base class in C++.
- for Date:
    - the Display TRAIT is implemented for Date struct.
So basically implementing how Date should be printed using the {} format
*/
impl fmt::Display for Date {
    /*
    - fn fmt(..):
        - The fmt method is called automatically when {} is used with Date.
    - &self:
        - Borrowing the Date (just like this in C#/C++), but immutably (& means read-only).
    - f: &mut fmt::Formatter:
        - f is just the name of the variable
            - It is a Formatter object (a string buffer similar to ostringstrem in C++).
        - &mut is a mutable reference, i.e. f it's mutable and it can be written on.
            - It could be &T or &mut T. Like in C++ there is const T& and T&
    - -> fmt::Result:
        - Function must return a result:
            - It could be ok(()) for sucess
            - Err(..) for formatting error
    So basically:
        - &self -> fields of the struct can be looked at, but cannot be changed
        - &mut fmt::Formatter -> allowed to write into this formatter buffer
    */
    fn fmt(&self, formatter_buffer: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter_buffer,
            "{:02}-{}-{}",
            self.day(),
            MONTH_NAMES[self.month() as usize],
            self.year()
        )
    }
}

use std::ops::Add;

/*
This would be the trait:

pub trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

*/

/*
Implementing the behavior of the + operator where:
  - the left-hand side is Date
  - the right-hand side is i32

This (impl Add<i32> for Date) is similar to:
    - C++: Date operator-(int n) const;
    - C#: public static Date operator -(Date d, int n)
*/

impl Add<i32> for Date {
    // Add days to date (by REFERENCE):
    // creates a new Date too, but can keep the original around
    // &d1 + 5 → borrows d1, returns a new Date, and still keep d1.
    type Output = Date;
    // i32 means it can be NEGATIVE!
    fn add(self, right_hand_side: i32) -> Date {
        Date::from_serial_number(self.serial_number + right_hand_side)
    }
}

/*
It will be implemented automatically by adding #derive(clone,copy)
impl Add<i32> for Date {
    // Add days to date (by VALUE):
    // indeed consumes the original Date and creates a new one.
    // You can do d1 + 5, but after that d1 is moved (not usable anymore).
    // d1 + 5 → consumes d1, returns a new Date.
    type Output = Date;
    // i32 means it can be NEGATIVE!
    fn add(self, right_hand_side: i32) -> Date {
        Date::from_serial_number(self.serial_number + right_hand_side)
    }
}
*/
use std::ops::Sub;

impl Sub<i32> for Date {
    /*
    by VALUE
    This implementations consumes the value:
    When a function takes an argument by value (not by reference),
    it moves ownership of that argument into the function,
    meaning the caller can no longer use it afterward unless it's Copy.

    This is what's meant by "consuming" a value:
        it's no longer usable after that operation.
    */
    type Output = Date;
    // i32 means it can be NEGATIVE!
    fn sub(self, right_hand_side: i32) -> Date {
        Date::from_serial_number(self.serial_number - right_hand_side)
    }
}

impl Sub<Date> for Date {
    type Output = i32;

    fn sub(self, right_hand_side: Date) -> i32 {
        // Subtract dates
        self.to_serial_number() - right_hand_side.to_serial_number()
    }
}

// This block will be compiled only when running cargo test
#[cfg(test)]
// Defines a nested test module
mod tests {
    // Bring everything from the outer scope (Date, its methods, etc.)
    use super::*;
    #[test]
    fn creates_date_correctly() {
        let d1: Date = Date::new(1, Month::April, 1989);

        assert_eq!(d1.day(), 1);
        assert_eq!(d1.month(), Month::April);
        assert_eq!(d1.year(), 1989);
    }

    #[test]
    fn display_date_iso_format() {
        let d1: Date = Date::new(2, Month::May, 1989);
        let result: String = format!("{}", d1);

        assert_eq!(result, "02-May-1989");
    }

    #[test]
    fn equality_same_date_true() {
        let d1: Date = Date::new(14, Month::May, 1989);
        let d2: Date = Date::new(14, Month::May, 1989);

        assert_eq!(d1, d2, "Dates should be equal");
    }

    #[test]
    fn equality_same_date_false() {
        let d1: Date = Date::new(14, Month::May, 1989);
        let d3: Date = Date::new(15, Month::May, 1989);

        assert_ne!(d1, d3, "Dates should not be equal");
    }

    #[test]
    fn date_comparison_works() {
        let d1: Date = Date::new(14, Month::May, 1989);
        let d2: Date = Date::new(17, Month::May, 1989);

        assert!(d1 < d2);

        let d3: Date = Date::new(13, Month::July, 1989);

        /*
        The first time it run, got an error here. Why ?
        Because the the order of comparison was:
            day -> month -> year.
        The order is defined in the constructor.
        It should be:
            year -> month -> day

        */
        assert!(d3 > d2);
    }

    #[test]
    fn add_days_works_correctly() {
        let d1: Date = Date::new(1, Month::May, 1989);
        let derived_date: Date = d1 + 40;

        // We made assumption 30 days per month
        let expected_date: Date = Date::new(10, Month::June, 1989);

        assert_eq!(derived_date, expected_date);
    }

    /*
     Not needed anymore because #derive(clone, copy):
     Doing d1 + 40 with copy is the same as doing &d1 + 40
    #[test]
     fn add_days_by_reference_works_correctly() {
         // `d1` owns a Date instance
         let d1 = Date::new(1, Month::August, 1989);
         // `&d1` borrows d1 immutably
         // meaning d1 can be borrowed but not changed
         let derived_date = &d1 + 40;

         // We made assumption 30 days per month
         let expected_date = Date::new(11, Month::September, 1989);

         assert_eq!(derived_date, expected_date);
     }
     */
    #[test]
    fn subtract_days_works_correctly() {
        let d1: Date = Date::new(15, Month::May, 1989);
        let derived_date: Date = d1 - 15;

        let expected_date: Date = Date::new(30, Month::April, 1989);

        assert_eq!(derived_date, expected_date);
    }

    #[test]
    fn subtract_dates_works_correctly() {
        let d1: Date = Date::new(14, Month::February, 1989);
        let d2: Date = Date::new(15, Month::February, 1989);

        let derived_days: i32 = d2 - d1;
        let expected_days: i32 = 1;

        assert_eq!(derived_days, expected_days);
    }

    #[test]
    fn to_serial_number_works_correctly() {
        let d: Date = Date::new(14, Month::October, 1989);

        let derived_serial_number: i32 = d.to_serial_number();
        let expected_serial_number: i32 =
            14 + Date::month_offset(1989, Month::October) + Date::year_offset(1989);

        assert_eq!(derived_serial_number, expected_serial_number);
    }

    #[test]
    fn from_serial_number_works_correctly() {
        let expected_date: Date = Date::new(11, Month::October, 2012);

        let serial_number: i32 = expected_date.to_serial_number();
        let derived_date: Date = Date::from_serial_number(serial_number);

        assert_eq!(derived_date, expected_date);
    }
}
