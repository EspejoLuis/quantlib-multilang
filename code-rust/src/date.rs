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
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)] // Ensures the compiler really lays it out as 1–12.
// Without it, Rust doesn’t guarantee contiguous values.
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
    /*
    Because not every u32 is valid, the function should return an Option:
    If n is between 1 and 12 → return Some(Month::X)
    Otherwise → return None

    Option<T> in Rust is an enum with two variants:
    enum Option<T> {
    None,
    Some(T),
    }
    */
    pub fn from_i32(number: i32) -> Option<Self> {
        if (1..=12).contains(&number) {
            /*
            transmute --> low-level cast that tells the compiler.
            transmute is unsafe because if you give it a value outside 1..=12,
            it would create an invalid enum value, which leads to UB (undefined behavior).
            unsafe --> "the contract necessary to call the operations inside the block has been
            checked by the programmer and is guaranteed to be respected"
            */
            Some(unsafe { std::mem::transmute(number as u8) })
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
/*
Because of #[derive(..)] dont need to create specific functions for
date. ><==!

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
    const MONTH_OFFSET: [i32; 13] = [0, 0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    const MONTH_LEAP_OFFSET: [i32; 13] = [0, 0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];
    const MONTH_LENGTH: [i32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    const MONTH_LEAP_LENGTH: [i32; 13] = [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

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
        assert!(
            n >= 0,
            "Invalid date: QuantLib epoch is 1899-12-31 (serial=0). Got serial={}",
            n
        );
        Date { serial_number: n }
    }

    pub fn to_serial_number(&self) -> i32 {
        self.serial_number
    }

    fn year_offset(year: i32) -> i32 {
        /*
        Returns the offset in days from 1901-01-01 up
        to year-01-01 using the closed-form formula.

        Examples:
        - year_offset(1901) = 0
        - year_offset(1902) = 365
        - year_offset(1903) = 730
        - year_offset(1904) = 1095  (leap added at 1904)
        */

        // The serial number is the number of days since 1899-12-31 (serial 0).
        //
        // 1900 is handled as a special case:
        //  - 1900-01-01 has serial 1
        //  - 1900-02-28 has serial 59
        //  - 1900-03-01 has serial 61
        //
        // i.e. February 29, 1900 is skipped (Excel-compatible).
        if year == 1900 {
            return 0;
        }; // special case

        let y: i32 = year - 1900;

        y * 365 + (y + 4) / 4 - (y + 100) / 100 + (y + 300) / 400
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
        match Date::is_leap(year) {
            true => Date::MONTH_LEAP_OFFSET[month as usize],
            false => Date::MONTH_OFFSET[month as usize],
        }
    }

    pub fn is_leap(year: i32) -> bool {
        (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
    }

    pub fn days_in_month(month: Month, year: i32) -> i32 {
        match Date::is_leap(year) {
            true => Date::MONTH_LEAP_LENGTH[month as usize],
            false => Date::MONTH_LENGTH[month as usize],
        }
    }

    // Getters
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
        let year: i32 = Date::year(&self); //self.year()
        let month: Month = Date::month(&self);

        self.serial_number - Date::month_offset(year, month) - Date::year_offset(year)
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
        let year: i32 = self.year();
        let day_of_year: i32 = self.serial_number - Date::year_offset(year);
        let mut candidate_month: i32 = 1;

        while candidate_month <= 12
            && Date::month_offset(
                year,
                Month::from_i32(candidate_month).unwrap_or_else(|| {
                    panic!(
                "Month decoding failed: candidate_month={} day_of_year={} serial={} year={}",
                candidate_month ,
                day_of_year,
                self.serial_number,
                year
            )
                }),
            ) < day_of_year
        // self.serial_number as i32 - Date::year_offset(year) is Number of days from start year
        {
            candidate_month += 1;
        }

        Month::from_i32(candidate_month - 1).unwrap_or_else(|| {
            panic!(
                "Month decoding failed: candidate_month={} day_of_year={} serial={} year={}",
                candidate_month - 1,
                day_of_year,
                self.serial_number,
                year
            )
        })
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

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // `&'static str` means a reference to a string literal that
        // lives for the entire program (string literals never expire).
        // Example: "Jan" is compiled into the binary and always safe.
        let name: &'static str = match self {
            Month::January => "Jan",
            Month::February => "Feb",
            Month::March => "Mar",
            Month::April => "Apr",
            Month::May => "May",
            Month::June => "Jun",
            Month::July => "Jul",
            Month::August => "Aug",
            Month::September => "Sep",
            Month::October => "Oct",
            Month::November => "Nov",
            Month::December => "Dec",
        };
        write!(f, "{}", name)
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
            self.month(),
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
    // add should panic cases

    #[test]
    fn creates_date_correctly() {
        let cases: [(Date, i32, Month, i32, &str); 8] = [
            // --- Normal date ---
            (
                Date::new(1, Month::October, 1989),
                1,
                Month::October,
                1989,
                "normal date",
            ),
            // --- Overflow into February ---
            (
                Date::new(32, Month::January, 2100),
                1,
                Month::February,
                2100,
                "overflow into next month",
            ),
            // --- Edge of supported range ---
            (
                Date::new(31, Month::December, 2099),
                31,
                Month::December,
                2099,
                "last day of 2099",
            ),
            // --- Epoch ---
            (
                Date::new(1, Month::January, 1900),
                1,
                Month::January,
                1900,
                "epoch start",
            ),
            // --- Big overflow: 78 Jan-days → March 19 ---
            (
                Date::new(78, Month::January, 1989),
                19,
                Month::March,
                1989,
                "big overflow across months",
            ),
            // --- Cross into next year ---
            (
                Date::new(370, Month::January, 1989),
                5,
                Month::January,
                1990,
                "overflow into next year",
            ),
            // --- Negative overflow: -10 Jan-days → Dec 22, 1889 ---
            (
                Date::new(-10, Month::January, 1901),
                21,
                Month::December,
                1900,
                "negative overflow into previous year",
            ),
            // --- Negative overflow: -40 Feb-days → Dec 22, 1899 ---
            (
                Date::new(-40, Month::February, 1901),
                22,
                Month::December,
                1900,
                "negative overflow crossing multiple months",
            ),
        ];

        for (date, exp_day, exp_month, exp_year, label) in cases {
            assert_eq!(
                date.day(),
                exp_day,
                "Day mismatch ({}) for {:?}",
                label,
                date
            );
            assert_eq!(
                date.month(),
                exp_month,
                "Month mismatch ({}) for {:?}",
                label,
                date
            );
            assert_eq!(
                date.year(),
                exp_year,
                "Year mismatch ({}) for {:?}",
                label,
                date
            );
        }
    }

    #[test]
    fn equality_cases() {
        let cases: [(Date, Date, bool, &str); 7] = [
            // Equal
            (
                Date::new(14, Month::May, 1989),
                Date::new(14, Month::May, 1989),
                true,
                "Same date",
            ),
            // Not equal - different day
            (
                Date::new(14, Month::May, 1989),
                Date::new(15, Month::May, 1989),
                false,
                "Different day",
            ),
            // Not equal - different month
            (
                Date::new(14, Month::May, 1989),
                Date::new(14, Month::June, 1989),
                false,
                "Different month",
            ),
            // Not equal - different year
            (
                Date::new(14, Month::May, 1989),
                Date::new(14, Month::May, 1990),
                false,
                "Different year",
            ),
            // Epoch check
            (
                Date::new(1, Month::January, 1900),
                Date::new(1, Month::January, 1900),
                true,
                "Epoch date",
            ),
            // Far boundary check
            (
                Date::new(31, Month::December, 2199),
                Date::new(31, Month::December, 2199),
                true,
                "Far boundary",
            ),
            // Leap year vs non-leap
            (
                Date::new(29, Month::February, 2000),
                Date::new(28, Month::February, 2001),
                false,
                "Leap vs non-leap",
            ),
        ];

        for (d1, d2, expected, msg) in cases {
            if expected {
                assert_eq!(d1, d2, "Expected equality failed: {}", msg);
            } else {
                assert_ne!(d1, d2, "Expected inequality failed: {}", msg);
            }
        }
    }

    #[test]
    fn date_comparison_all_ops() {
        let cases: [(Date, Date, std::cmp::Ordering, &'static str); 7] = [
            // --- Day comparisons ---
            (
                Date::new(14, Month::May, 1989),
                Date::new(17, Month::May, 1989),
                std::cmp::Ordering::Less,
                "day <",
            ),
            (
                Date::new(17, Month::May, 1989),
                Date::new(14, Month::May, 1989),
                std::cmp::Ordering::Greater,
                "day >",
            ),
            // --- Month comparisons ---
            (
                Date::new(17, Month::May, 1989),
                Date::new(13, Month::July, 1989),
                std::cmp::Ordering::Less,
                "month <",
            ),
            (
                Date::new(13, Month::July, 1989),
                Date::new(17, Month::May, 1989),
                std::cmp::Ordering::Greater,
                "month >",
            ),
            // --- Year comparisons ---
            (
                Date::new(13, Month::July, 1989),
                Date::new(1, Month::January, 1990),
                std::cmp::Ordering::Less,
                "year <",
            ),
            (
                Date::new(1, Month::January, 1990),
                Date::new(13, Month::July, 1989),
                std::cmp::Ordering::Greater,
                "year >",
            ),
            // --- Equality ---
            (
                Date::new(1, Month::January, 1990),
                Date::new(1, Month::January, 1990),
                std::cmp::Ordering::Equal,
                "equal",
            ),
        ];

        for (d1, d2, expected, label) in cases {
            let result = d1.cmp(&d2);
            assert_eq!(result, expected, "Failed cmp for {}", label);

            match expected {
                std::cmp::Ordering::Less => {
                    assert!(d1 < d2, "Failed < for {}", label);
                    assert!(d1 != d2, "Failed != for {}", label);
                }
                std::cmp::Ordering::Greater => {
                    assert!(d1 > d2, "Failed > for {}", label);
                    assert!(d1 != d2, "Failed != for {}", label);
                }
                std::cmp::Ordering::Equal => {
                    assert!(d1 == d2, "Failed == for {}", label);
                    assert!(!(d1 != d2), "Failed != negation for {}", label);
                }
            }
        }
    }

    #[test]
    fn add_days_works_correctly() {
        let cases: [(Date, i32, Date, &str); 6] = [
            // --- Within same month ---
            (
                Date::new(1, Month::May, 1989),
                10,
                Date::new(11, Month::May, 1989),
                "add within same month",
            ),
            // --- Next month ---
            (
                Date::new(1, Month::May, 1989),
                40,
                Date::new(10, Month::June, 1989),
                "add across one month",
            ),
            // --- Multiple months ---
            (
                Date::new(15, Month::May, 1989),
                60,
                Date::new(14, Month::July, 1989),
                "add across multiple months",
            ),
            // --- Next year ---
            (
                Date::new(20, Month::December, 1989),
                15,
                Date::new(4, Month::January, 1990),
                "cross into next year",
            ),
            // --- Leap year Feb ---
            (
                Date::new(25, Month::February, 2020), // leap year
                10,
                Date::new(6, Month::March, 2020),
                "cross leap-year February",
            ),
            // --- Negative days ---
            (
                Date::new(5, Month::March, 1989),
                -10,
                Date::new(23, Month::February, 1989),
                "subtract days (negative add)",
            ),
        ];

        for (start, delta, expected, label) in cases {
            let result = start + delta;
            assert_eq!(
                result, expected,
                "Failed {}: {} + {} days",
                label, start, delta
            );
        }
    }

    /*
     Not needed anymore because #derive(clone, copy):
     Doing d1 + 40 with copy is the same as doing &d1 + 40
    #[test]
     fn add_days_by_reference_works_correctly() {
         // `d1` owns a Date instance
         let d1 = Date::new(1, Month::January, 1989);
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
        let cases: [(i32, Month, i32, i32, i32, Month, i32); 5] = [
            // (start_day, start_month, start_year, delta, expected_day, expected_month, expected_year)
            (20, Month::May, 1989, 5, 15, Month::May, 1989), // within same month
            (15, Month::May, 1989, 15, 30, Month::April, 1989), // cross month
            (1, Month::January, 1990, 1, 31, Month::December, 1989), // cross year
            (1, Month::March, 2024, 1, 29, Month::February, 2024), // leap year
            (1, Month::March, 2023, 1, 28, Month::February, 2023), // non-leap year
        ];

        for (
            start_day,
            start_month,
            start_year,
            delta,
            expected_day,
            expected_month,
            expected_year,
        ) in cases
        {
            let start = Date::new(start_day, start_month, start_year);
            let derived = start - delta;
            let expected = Date::new(expected_day, expected_month, expected_year);

            assert_eq!(derived, expected, "Failed case: {start:?} - {delta}");
        }
    }

    #[test]
    fn subtract_dates_works_correctly() {
        let cases: [(i32, Month, i32, i32, Month, i32, i32); 5] = [
            // (d1_day, d1_month, d1_year, d2_day, d2_month, d2_year, expected_days)
            (14, Month::February, 1989, 15, Month::February, 1989, 1), // same month
            (28, Month::February, 1989, 1, Month::March, 1989, 1),     // cross month
            (31, Month::December, 1989, 1, Month::January, 1990, 1),   // cross year
            (28, Month::February, 2024, 1, Month::March, 2024, 2),     // leap year
            (1, Month::January, 1989, 1, Month::January, 1990, 365),   // full year
        ];

        for (d1_day, d1_month, d1_year, d2_day, d2_month, d2_year, expected_days) in cases {
            let d1 = Date::new(d1_day, d1_month, d1_year);
            let d2 = Date::new(d2_day, d2_month, d2_year);

            let derived_days: i32 = d2 - d1;
            assert_eq!(derived_days, expected_days, "Failed case: {d2:?} - {d1:?}");
        }
    }

    #[test]
    fn to_serial_number_works_correctly() {
        let cases: [(i32, Month, i32); 6] = [
            (1, Month::January, 1901),   // epoch start
            (29, Month::February, 1904), // leap day
            (31, Month::December, 1989), // year end
            (14, Month::June, 1989),     // mid example
            (1, Month::January, 2000),   // cross century
            (31, Month::December, 2099), // far future
        ];

        for (day, month, year) in cases {
            let d = Date::new(day, month, year);

            let derived = d.to_serial_number();
            let expected = day + Date::month_offset(year, month) + Date::year_offset(year);

            assert_eq!(derived, expected, "Failed case: {d:?}");
        }
    }

    #[test]
    fn from_serial_number_works_correctly() {
        let cases: [Date; 4] = [
            Date::new(01, Month::January, 1956),
            Date::new(01, Month::January, 2100),
            Date::new(31, Month::December, 2099),
            Date::new(01, Month::January, 1900),
        ];

        for date in cases {
            let serial_number: i32 = date.to_serial_number();
            let derived_date: Date = Date::from_serial_number(serial_number);

            assert_eq!(derived_date, date);
        }
    }

    #[test]
    fn to_serial_number_vs_quantlib() {
        let cases: [(i32, Month, i32, i32); 7] = [
            (1, Month::January, 1901, 1),       // epoch start
            (31, Month::December, 1901, 365),   // non-leap year end
            (1, Month::January, 1902, 366),     // start of next year
            (29, Month::February, 1904, 1520),  // leap day
            (31, Month::December, 1989, 32417), // checked against QuantLib
            (1, Month::January, 2000, 36524),   // millennium
            (31, Month::December, 2099, 73049), // far future
        ];

        for (day, month, year, expected_serial) in cases {
            let d = Date::new(day, month, year);
            let derived = d.to_serial_number();

            assert_eq!(derived, expected_serial, "Failed case: {d:?}");
        }
    }

    #[test]
    fn from_month_i32_gives_correct_month() {
        let cases: [(i32, Month); 12] = [
            (1, Month::January),
            (2, Month::February),
            (3, Month::March),
            (4, Month::April),
            (5, Month::May),
            (6, Month::June),
            (7, Month::July),
            (8, Month::August),
            (9, Month::September),
            (10, Month::October),
            (11, Month::November),
            (12, Month::December),
        ];

        for (num, expected) in cases {
            let derived: Option<Month> = Month::from_i32(num);
            assert_eq!(derived.unwrap(), expected, "Failed for number {}", num);
        }
    }

    #[test]
    fn from_month_i32_invalid_gives_none() {
        let invalid_cases: [i32; 4] = [0, 13, 99, -5];

        for num in invalid_cases {
            let derived: Option<Month> = Month::from_i32(num);
            assert!(
                derived.is_none(),
                // It comes from the Debug trait (automatically derived in Month enum
                // since #[derive(Debug)] was declared.
                // It prints a developer-friendly representation of a value
                "Expected None for invalid month {}, but got {:?}",
                num,
                derived
            );
        }
    }

    #[test]
    fn is_leap_true() {
        let leap_years: [i32; 3] = [2000, 1928, 1956];
        for year in leap_years {
            assert_eq!(
                Date::is_leap(year),
                true,
                "Year {} was incorrectly detected as leap",
                year
            );
        }
    }

    #[test]
    fn is_leap_false() {
        let not_leap_years: [i32; 3] = [1945, 1999, 1900];
        for year in not_leap_years {
            assert_eq!(
                Date::is_leap(year),
                false,
                "Year {} was incorrectly detected as not leap",
                year
            );
        }
    }

    #[test]
    fn days_in_month_not_leap_year() {
        /*
        If your function takes T (by value), then the function owns the argument.
        That means:
            - The caller loses ownership.
            - The callee can do whatever it wants (mutate, move, drop).
            - The caller cannot use the value afterward unless it was Copy.

        If your function takes &mut T, then the function borrows mutably:
            - The caller keeps ownership.
            - The callee can mutate the data, but only while it holds the borrow.
            - After the borrow ends, the caller can use the value again.
        */
        let cases: [(i32, Month); 12] = [
            (31, Month::January),
            (28, Month::February),
            (31, Month::March),
            (30, Month::April),
            (31, Month::May),
            (30, Month::June),
            (31, Month::July),
            (31, Month::August),
            (30, Month::September),
            (31, Month::October),
            (30, Month::November),
            (31, Month::December),
        ];
        for (days, month) in cases {
            assert_eq!(
                Date::days_in_month(month, 2001),
                days,
                "Month {} for year {} does not have right number of days {}",
                month,
                2001,
                days
            );
        }
    }

    #[test]
    fn days_in_month_leap_year() {
        let cases: [(i32, Month); 12] = [
            (31, Month::January),
            (29, Month::February),
            (31, Month::March),
            (30, Month::April),
            (31, Month::May),
            (30, Month::June),
            (31, Month::July),
            (31, Month::August),
            (30, Month::September),
            (31, Month::October),
            (30, Month::November),
            (31, Month::December),
        ];
        for (days, month) in cases {
            assert_eq!(
                Date::days_in_month(month, 2000),
                days,
                "Month {} for year {} does not have right number of days {}",
                month,
                2000,
                days
            );
        }
    }
}
