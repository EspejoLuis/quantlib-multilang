use super::weekday::{WeekDayIndex, Weekday};
use chrono::{Datelike, Local, NaiveDate}; // Todays' date
use std::fmt::{Display, Formatter, Result}; // Display
use std::ops::{Add, Sub}; // Addition, Subtraction
use std::ops::{AddAssign, SubAssign};

type Day = i32;
type Year = i32;
type MonthIndex = usize;
type SerialType = i32;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)] // Ensures the compiler lays it out as 1–12. Without it, Rust doesn’t guarantee contiguous values.
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
    pub fn from_index(number: MonthIndex) -> Self {
        if (1..=12).contains(&number) {
            /*
            transmute --> low-level cast that tells the compiler.
            transmute is unsafe because if you give it a value outside 1..=12,
            it would create an invalid enum value, which leads to UB (undefined behavior).
            unsafe --> "the contract necessary to call the operations inside the block has been
            checked by the programmer and is guaranteed to be respected"
            */
            unsafe { std::mem::transmute(number as u8) }
        } else {
            panic!("Month index {} out of range [1,12]", number);
        }
    }
}

// Traits:
impl Display for Month {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // `&'static str` means a reference to a string literal that
        // lives for the entire program (string literals never expire).
        // Example: "January" is compiled into the binary and always safe.
        let name: &'static str = match self {
            Month::January => "January",
            Month::February => "February",
            Month::March => "March",
            Month::April => "April",
            Month::May => "May",
            Month::June => "June",
            Month::July => "July",
            Month::August => "August",
            Month::September => "September",
            Month::October => "October",
            Month::November => "November",
            Month::December => "December",
        };
        write!(f, "{}", name)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Date {
    // Defines a struct named Date, just like a class in C++ or C# with only data (no methods yet).
    // pub --> public so they can be access by other files like main.rs
    // unsigned 32-bit integer
    serial_number: SerialType, //private by default
                               // Order below impact for example how operator < works.
                               // First field to be check is the first one below.
                               //pub year: u32,
                               //pub month: Month,
                               //pub day: u32,
}
impl Date {
    const MIN_SERIAL: i32 = 367; // 1901-01-01
    const MAX_SERIAL: i32 = 109574; // 2199-12-31

    // Constructor
    pub fn new(day: Day, month: Month, year: Year) -> Date {
        // TODO: ADD how to handle wrong dates

        // 1. Year check
        assert!(
            (1901..=2199).contains(&year),
            "year {} out of bounds, must be [1901,2199]",
            year
        );
        // 2. Leap year
        let is_leap: bool = Date::is_leap(year);
        // 3. Month length & offset
        let month_length: i32 = Date::month_length(month as MonthIndex, is_leap);
        let month_offset: i32 = Date::month_offset(month as MonthIndex, is_leap);
        let year_offset: i32 = Date::year_offset(year);
        // 4. Day check
        assert!(
            day > 0 && day <= month_length,
            "day {} outside month ({}) day-range [1,{}]",
            day,
            month as MonthIndex,
            month_length
        );
        // 5. Serial number
        let serial_number: SerialType = day + month_offset + year_offset;
        // 6. Check serial number
        Date::check_serial_number(serial_number);
        Date { serial_number }
    }
    pub fn from_serial_number(serial_number: SerialType) -> Date {
        Date::check_serial_number(serial_number);
        Date {
            serial_number: serial_number,
        }
    }

    // Helpers
    fn year_offset(year: Year) -> i32 {
        /*
        Returns the offset in days from 31 Dec 1900 (serial 0)
        up to 1 Jan `year` using the closed-form formula.

        "Number of days before the year begins"

        Examples:
        - year_offset(1901) = 0    → (1-Jan-1901 is serial 1)
        - year_offset(1902) = 365
        - year_offset(1903) = 731
         */
        assert!(
            (1900..=2200).contains(&year),
            "year {} outside valid range [1900,2200]",
            year
        );

        const YEAR_OFFSET: [i32; 301] = [
            // 1900–1909
            0, 366, 731, 1096, 1461, 1827, 2192, 2557, 2922, 3288, // 1910–1919
            3653, 4018, 4383, 4749, 5114, 5479, 5844, 6210, 6575, 6940, // 1920–1929
            7305, 7671, 8036, 8401, 8766, 9132, 9497, 9862, 10227, 10593, // 1930–1939
            10958, 11323, 11688, 12054, 12419, 12784, 13149, 13515, 13880, 14245,
            // 1940–1949
            14610, 14976, 15341, 15706, 16071, 16437, 16802, 17167, 17532, 17898,
            // 1950–1959
            18263, 18628, 18993, 19359, 19724, 20089, 20454, 20820, 21185, 21550,
            // 1960–1969
            21915, 22281, 22646, 23011, 23376, 23742, 24107, 24472, 24837, 25203,
            // 1970–1979
            25568, 25933, 26298, 26664, 27029, 27394, 27759, 28125, 28490, 28855,
            // 1980–1989
            29220, 29586, 29951, 30316, 30681, 31047, 31412, 31777, 32142, 32508,
            // 1990–1999
            32873, 33238, 33603, 33969, 34334, 34699, 35064, 35430, 35795, 36160,
            // 2000–2009
            36525, 36891, 37256, 37621, 37986, 38352, 38717, 39082, 39447, 39813,
            // 2010–2019
            40178, 40543, 40908, 41274, 41639, 42004, 42369, 42735, 43100, 43465,
            // 2020–2029
            43830, 44196, 44561, 44926, 45291, 45657, 46022, 46387, 46752, 47118,
            // 2030–2039
            47483, 47848, 48213, 48579, 48944, 49309, 49674, 50040, 50405, 50770,
            // 2040–2049
            51135, 51501, 51866, 52231, 52596, 52962, 53327, 53692, 54057, 54423,
            // 2050–2059
            54788, 55153, 55518, 55884, 56249, 56614, 56979, 57345, 57710, 58075,
            // 2060–2069
            58440, 58806, 59171, 59536, 59901, 60267, 60632, 60997, 61362, 61728,
            // 2070–2079
            62093, 62458, 62823, 63189, 63554, 63919, 64284, 64650, 65015, 65380,
            // 2080–2089
            65745, 66111, 66476, 66841, 67206, 67572, 67937, 68302, 68667, 69033,
            // 2090–2099
            69398, 69763, 70128, 70494, 70859, 71224, 71589, 71955, 72320, 72685,
            // 2100–2109
            73050, 73415, 73780, 74145, 74510, 74876, 75241, 75606, 75971, 76337,
            // 2110–2119
            76702, 77067, 77432, 77798, 78163, 78528, 78893, 79259, 79624, 79989,
            // 2120–2129
            80354, 80720, 81085, 81450, 81815, 82181, 82546, 82911, 83276, 83642,
            // 2130–2139
            84007, 84372, 84737, 85103, 85468, 85833, 86198, 86564, 86929, 87294,
            // 2140–2149
            87659, 88025, 88390, 88755, 89120, 89486, 89851, 90216, 90581, 90947,
            // 2150–2159
            91312, 91677, 92042, 92408, 92773, 93138, 93503, 93869, 94234, 94599,
            // 2160–2169
            94964, 95330, 95695, 96060, 96425, 96791, 97156, 97521, 97886, 98252,
            // 2170–2179
            98617, 98982, 99347, 99713, 100078, 100443, 100808, 101174, 101539, 101904,
            // 2180–2189
            102269, 102635, 103000, 103365, 103730, 104096, 104461, 104826, 105191, 105557,
            // 2190–2199
            105922, 106287, 106652, 107018, 107383, 107748, 108113, 108479, 108844, 109209,
            // 2200
            109574,
        ];

        YEAR_OFFSET[(year - 1900) as usize]
    }
    fn year_lenght(year: Year) -> i32 {
        if Date::is_leap(year) { 366 } else { 365 }
    }
    fn month_offset(month_index: MonthIndex, is_leap: bool) -> i32 {
        /*
        Returns the offset in days from 1-Jan of `year`
        up to (but not including) the 1st day of the given `month`.
        */

        // month_index could be 13 when called from month()
        assert!(
            (1..=13).contains(&month_index),
            "Month index {} out of range [1,13]",
            month_index
        );

        const MONTH_OFFSET: [i32; 14] = [
            0, 0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365,
        ];
        const MONTH_LEAP_OFFSET: [i32; 14] = [
            0, 0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366,
        ];

        if is_leap {
            MONTH_LEAP_OFFSET[month_index as usize]
        } else {
            MONTH_OFFSET[month_index as usize]
        }
    }
    fn month_length(month_index: MonthIndex, is_leap: bool) -> i32 {
        // Days from 1-Jan to start of each month
        const MONTH_LENGTH: [i32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        const MONTH_LEAP_LENGTH: [i32; 13] = [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        match is_leap {
            true => MONTH_LEAP_LENGTH[month_index as usize],
            false => MONTH_LENGTH[month_index as usize],
        }
    }
    fn is_leap(year: Year) -> bool {
        assert!(
            (1900..=2200).contains(&year),
            "year {} outside valid range [1900, 2200]",
            year
        );

        const YEAR_IS_LEAP: [bool; 301] = [
            // 1900 is leap in agreement with Excel's bug
            // 1900–1909
            true, false, false, false, true, false, false, false, true, false,
            // 1910–1919
            false, false, true, false, false, false, true, false, false, false,
            // 1920–1929
            true, false, false, false, true, false, false, false, true, false,
            // 1930–1939
            false, false, true, false, false, false, true, false, false, false,
            // 1940–1949
            true, false, false, false, true, false, false, false, true, false,
            // 1950–1959
            false, false, true, false, false, false, true, false, false, false,
            // 1960–1969
            true, false, false, false, true, false, false, false, true, false,
            // 1970–1979
            false, false, true, false, false, false, true, false, false, false,
            // 1980–1989
            true, false, false, false, true, false, false, false, true, false,
            // 1990–1999
            false, false, true, false, false, false, true, false, false, false,
            // 2000–2009
            true, false, false, false, true, false, false, false, true, false,
            // 2010–2019
            false, false, true, false, false, false, true, false, false, false,
            // 2020–2029
            true, false, false, false, true, false, false, false, true, false,
            // 2030–2039
            false, false, true, false, false, false, true, false, false, false,
            // 2040–2049
            true, false, false, false, true, false, false, false, true, false,
            // 2050–2059
            false, false, true, false, false, false, true, false, false, false,
            // 2060–2069
            true, false, false, false, true, false, false, false, true, false,
            // 2070–2079
            false, false, true, false, false, false, true, false, false, false,
            // 2080–2089
            true, false, false, false, true, false, false, false, true, false,
            // 2090–2099
            false, false, true, false, false, false, true, false, false, false,
            // 2100–2109
            false, false, true, false, false, false, true, false, false, false,
            // 2110–2119
            false, false, true, false, false, false, true, false, false, false,
            // 2120–2129
            true, false, false, false, true, false, false, false, true, false,
            // 2130–2139
            false, false, true, false, false, false, true, false, false, false,
            // 2140–2149
            true, false, false, false, true, false, false, false, true, false,
            // 2150–2159
            false, false, true, false, false, false, true, false, false, false,
            // 2160–2169
            true, false, false, false, true, false, false, false, true, false,
            // 2170–2179
            false, false, true, false, false, false, true, false, false, false,
            // 2180–2189
            true, false, false, false, true, false, false, false, true, false,
            // 2190–2199
            false, false, true, false, false, false, true, false, false, false, // 2200
            false,
        ];

        YEAR_IS_LEAP[(year - 1900) as usize]
    }
    fn check_serial_number(serial_number: SerialType) {
        assert!(
            (Date::MIN_SERIAL..=Date::MAX_SERIAL).contains(&serial_number),
            "Serial number {} out of bounds, must be [{}..={}] or in dates [{} - {}]",
            serial_number,
            Date::MIN_SERIAL,
            Date::MAX_SERIAL,
            Date::max_date(),
            Date::min_date()
        );
    }
    // Inspectors private

    // Inspectors public
    pub fn day_of_month(&self) -> Day {
        let year: Year = self.year();
        let leap: bool = Date::is_leap(year);
        let month_index: MonthIndex = self.month() as MonthIndex;

        self.serial_number - Date::year_offset(year) - Date::month_offset(month_index, leap)
    }
    pub fn day_of_year(&self) -> Day {
        self.serial_number - Date::year_offset(self.year())
    }
    pub fn year(&self) -> Year {
        // Initial guess
        let mut year: Year = (self.serial_number / 365) + 1900;

        // Correction step
        if self.serial_number <= Date::year_offset(year) {
            year -= 1;
        }

        year
    }
    pub fn month(&self) -> Month {
        let day_of_year: Day = self.day_of_year();
        let year: Year = self.year();
        let leap: bool = Date::is_leap(year);

        // Rough guess
        let mut month_index: MonthIndex = (day_of_year / 30 + 1) as MonthIndex;

        // Adjust down if too far
        while day_of_year <= Date::month_offset(month_index, leap) {
            month_index -= 1;
        }

        // Adjust up if still inside next month
        while day_of_year > Date::month_offset(month_index + 1, leap) {
            month_index += 1;
        }

        // month_index cannot be 13
        Month::from_index(month_index)
    }
    pub fn day(&self) -> Day {
        self.day_of_month()
    }
    pub fn to_serial_number(&self) -> SerialType {
        self.serial_number
    }
    pub fn min_date() -> Date {
        Date::from_serial_number(Date::MIN_SERIAL)
    }
    pub fn max_date() -> Date {
        Date::from_serial_number(Date::MAX_SERIAL)
    }
    pub fn is_end_of_month(&self) -> bool {
        self.day() == Date::month_length(self.month() as MonthIndex, Date::is_leap(self.year()))
    }
    pub fn end_of_month(&self) -> Date {
        let month: Month = self.month();
        let year: Year = self.year();
        Date::new(
            Date::month_length(month as MonthIndex, Date::is_leap(year)),
            month,
            year,
        )
    }
    pub fn todays_date() -> Date {
        let today: NaiveDate = Local::now().date_naive();
        let year: Year = today.year();
        let month: Month = Month::from_index(today.month() as MonthIndex);
        let day: Day = today.day() as Day;
        Date::new(day, month, year)
    }
    pub fn weekday(&self) -> Weekday {
        /*
        If remainder is zero then it's a Saturday

        When you divide by 7, the remainder cycles every 7 days:
        … remainder 1 → Sunday
        … remainder 2 → Monday
        … remainder 3 → Tuesday
        … remainder 4 → Wednesday
        … remainder 5 → Thursday
        … remainder 6 → Friday
        … remainder 0 → Saturday
        */
        let day_mask: SerialType = self.serial_number % 7;
        let day_of_week: WeekDayIndex = if day_mask == 0 { 7 } else { day_mask as usize };
        Weekday::from_index(day_of_week)
    }
    pub fn next_weekday(&self, target_day_of_week: Weekday) -> Date {
        /*
        // starting from a given date, move forward until you hit the requested Weekday.
        // New Date

        Suppose today is Wednesday 21-Aug-2024 (weekday = 3).
            - You ask for the next Friday (weekday = 5).
            - current = 3, target = 5
            - current > target? → No → so diff = 0 - 3 + 5 = 2
            - Return = Wednesday + 2 days = Friday 23-Aug-2024

        Suppose today is Friday 23-Aug-2024 (weekday = 5).
            - You ask for the next Wednesday (weekday = 3).
            - current = 5, target = 3
            - current > target? → Yes → so diff = 7 - 5 + 3 = 5
            - Return = Friday + 5 days = Wednesday 28-Aug-2024

        If next is the same of the starting date then it should not give a new date i.e.
        if next Friday is asked and the current date is already a Friday, then the result
        will be the same date.
        */
        let current_day_of_week: Day = self.weekday() as Day;
        let target_day_of_week: Day = target_day_of_week as Day;
        let diff: Day = if current_day_of_week > target_day_of_week {
            7
        } else {
            0
        } - current_day_of_week
            + target_day_of_week;

        *self + diff
    }
    pub fn nth_weekday(nth: usize, day_of_week: Weekday, month: Month, year: Year) -> Date {
        /*
        Find the nth occurrence of a specific weekday in a month/year.

        Example 1:
            - Find the 3rd Friday of September 2024.
            - First day of September 2024 is Sunday.
            - target weekday = Friday.
            - Formula finds: 20-Sep-2024.

        Example 2:
            - Find the 1st Monday of May 2025.
            - First day of May 2025 is Thursday.
            - First Monday = 5-May-2025.

        */
        assert!(nth > 0, "The zeroth day of the week is not defined");
        assert!(nth < 6, "No more the 5 weekday in a given month");

        let day_of_week: Day = day_of_week as Day;
        let first_day_of_week: Day = Date::new(1, month, year).weekday() as Day;

        let skip: Day = nth as Day
            - if day_of_week >= first_day_of_week {
                1
            } else {
                0
            };

        Date::new(1 + day_of_week + skip * 7 - first_day_of_week, month, year)
    }
    pub fn increment(&mut self) -> () {
        let serial_number: SerialType = self.serial_number + 1;
        Date::check_serial_number(serial_number);
        self.serial_number = serial_number
    }
    pub fn decrement(&mut self) -> () {
        let serial_number: SerialType = self.serial_number - 1;
        Date::check_serial_number(serial_number);
        self.serial_number = serial_number
    }
}

// Traits:
impl Display for Date {
    /*
    - impl: we are implementing something.
    - fmt::Display: this is a TRAIT:
        - like an interface in C# or abstract base class in C++.
    - for Date:
        - the Display TRAIT is implemented for Date struct.
    So basically implementing how Date should be printed using the {} format

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
    fn fmt(&self, formatter_buffer: &mut Formatter) -> Result {
        write!(formatter_buffer, "{}", io::long_date(self))
    }
}
impl Add<Day> for Date {
    /*
    This would be the trait:

    pub trait Add<Rhs = Self> {
        type Output;
        fn add(self, rhs: Rhs) -> Self::Output;
    }

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

    Implementing the behavior of the + operator where:
        - the left-hand side is Date
        - the right-hand side is i32

    This (impl Add<i32> for Date) is similar to:
        - C++: Date operator-(int n) const;
        - C#: public static Date operator -(Date d, int n)

    Add days to date (by REFERENCE):
    creates a new Date too, but can keep the original around
    &d1 + 5 → borrows d1, returns a new Date, and still keep d1.
    */
    type Output = Date;
    // i32 means it can be NEGATIVE!
    // New Date as Output
    fn add(self, right_hand_side: Day) -> Date {
        Date::from_serial_number(self.serial_number + right_hand_side)
    }
}
impl AddAssign<Day> for Date {
    // No Output, no new Date returned. SAME DATE modified!
    fn add_assign(&mut self, right_hand_side: Day) -> () {
        let serial_number: SerialType = self.serial_number + right_hand_side;
        Date::check_serial_number(serial_number);
        self.serial_number += right_hand_side
    }
}
impl Sub<Day> for Date {
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
    // New Date as Output
    fn sub(self, right_hand_side: Day) -> Date {
        Date::from_serial_number(self.serial_number - right_hand_side)
    }
}
impl SubAssign<Day> for Date {
    // No Output, no new Date returned. SAME DATE modified!
    fn sub_assign(&mut self, right_hand_side: Day) -> () {
        let serial_number: SerialType = self.serial_number - right_hand_side;
        Date::check_serial_number(serial_number);
        self.serial_number -= right_hand_side
    }
}
impl Sub<Date> for Date {
    type Output = SerialType;

    fn sub(self, right_hand_side: Date) -> SerialType {
        // Subtract dates
        self.to_serial_number() - right_hand_side.to_serial_number()
    }
}
impl Default for Date {
    fn default() -> Self {
        Date { serial_number: 0 }
    }
}
// Private
mod detail {
    use super::Date;
    use super::MonthIndex;
    use crate::io;
    use chrono::NaiveDate;
    use std::fmt::{Display, Formatter, Result};

    pub(crate) struct LongDate<'a> {
        pub(crate) date: &'a Date,
    }
    pub(crate) struct ShortDate<'a> {
        pub(crate) date: &'a Date,
    }
    pub(crate) struct IsoDate<'a> {
        pub(crate) date: &'a Date,
    }
    pub(crate) struct FormattedDate<'a> {
        pub(crate) date: &'a Date,
        pub(crate) format: &'a str,
    }

    impl<'a> Display for LongDate<'a> {
        fn fmt(&self, f: &mut Formatter) -> Result {
            // Example: "July 23, 2024"
            write!(
                f,
                "{} {}, {}",
                self.date.month(),
                io::ordinal(self.date.day() as usize),
                self.date.year()
            )
        }
    }
    impl<'a> Display for ShortDate<'a> {
        fn fmt(&self, f: &mut Formatter) -> Result {
            // Example: "05-31-2025"
            write!(
                f,
                "{:02}/{:02}/{}",
                self.date.month() as MonthIndex,
                self.date.day(),
                self.date.year()
            )
        }
    }
    impl<'a> Display for IsoDate<'a> {
        fn fmt(&self, f: &mut Formatter) -> Result {
            // Example: "2025-05-12"
            write!(
                f,
                "{:04}-{:02}-{:02}",
                self.date.year(),
                self.date.month() as MonthIndex,
                self.date.day()
            )
        }
    }
    impl<'a> Display for FormattedDate<'a> {
        fn fmt(&self, f: &mut Formatter) -> Result {
            if self.date.serial_number == 0 {
                return write!(f, "null date");
            }

            match NaiveDate::from_ymd_opt(
                self.date.year(),
                self.date.month() as u32,
                self.date.day() as u32,
            ) {
                Some(naive) => write!(f, "{}", naive.format(self.format)),
                None => panic!("Invalid date"),
            }
        }
    }
}

// Public String API
pub(crate) mod io {
    /*
    println!("{}", io::long_date(d));   // default-style
    println!("{}", io::short_date(d));  // 07/23/2024
    println!("{}", io::iso_date(d));    // 2024-07-23
    */
    use super::{Date, detail};
    use std::fmt::Display;

    pub fn long_date<'a>(d: &'a Date) -> impl Display + 'a {
        detail::LongDate { date: d }
    }
    pub fn short_date<'a>(d: &'a Date) -> impl Display + 'a {
        detail::ShortDate { date: d }
    }
    pub fn iso_date<'a>(d: &'a Date) -> impl Display + 'a {
        detail::IsoDate { date: d }
    }
    pub fn formatted_date<'a>(d: &'a Date, format: &'a str) -> impl Display + 'a {
        detail::FormattedDate {
            date: d,
            format: format,
        }
    }
}

// Tests
#[cfg(test)]
mod tests {

    use super::*; // Bring everything from the outer scope (Date, its methods, etc.)
    use std::panic;

    #[test]
    fn new_sets_day_month_year_correctly() {
        let cases: [(Date, Day, Month, Year, &str); 6] = [
            (
                Date::new(1, Month::October, 1989),
                1,
                Month::October,
                1989,
                "normal date",
            ),
            (
                Date::new(31, Month::December, 2099),
                31,
                Month::December,
                2099,
                "last day of 2099",
            ),
            (
                Date::new(1, Month::January, 1901),
                1,
                Month::January,
                1901,
                "epoch start (QuantLib)",
            ),
            (
                Date::new(1, Month::October, 1989),
                1,
                Month::October,
                1989,
                "normal date",
            ),
            (
                Date::new(29, Month::February, 2000), // leap year
                29,
                Month::February,
                2000,
                "leap year Feb 29",
            ),
            (
                Date::new(31, Month::December, 2199), // max supported date
                31,
                Month::December,
                2199,
                "maximum supported date",
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
    fn new_panics_invalid_dates() {
        let cases: [(Day, Month, Year, &str); 5] = [
            (29, Month::February, 1901, "Feb 29 non-leap year"),
            (1, Month::January, 1800, "year before min"),
            (1, Month::January, 2200, "year after max"),
            (400, Month::October, 1989, "day overflow"),
            (-1, Month::October, 1989, "day underflow"),
        ];

        for (day, month, year, label) in cases {
            let result = panic::catch_unwind(|| {
                Date::new(day, month, year);
            });

            assert!(
                result.is_err(),
                "Expected panic but got Ok for case: {} ({}-{}-{})",
                label,
                day,
                month as MonthIndex,
                year
            );
        }
    }

    #[test]
    fn year_offset_panics_invalid_years() {
        let invalid_years: [Year; 2] = [1899, 2201];

        for year in invalid_years {
            let result = panic::catch_unwind(|| {
                Date::year_offset(year);
            });

            assert!(
                result.is_err(),
                "Expected panic for invalid year {} but got Ok",
                year
            );
        }
    }

    #[test]
    fn month_offset_panics_invalida_months() {
        let invalid_month_indices: [MonthIndex; 3] = [15, 0, 17];

        for month_index in invalid_month_indices {
            let result = std::panic::catch_unwind(|| {
                Date::month_offset(month_index, true);
            });

            assert!(
                result.is_err(),
                "Expected panic for invalid month {} but got Ok",
                month_index
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
            // Epoch check (QuantLib epoch)
            (
                Date::new(1, Month::January, 1901),
                Date::new(1, Month::January, 1901),
                true,
                "Epoch date (QuantLib)",
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
            let result: std::cmp::Ordering = d1.cmp(&d2);
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
    fn add_days_works() {
        let cases: [(Date, Day, Date, &str); 6] = [
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
            let result: Date = start + delta;
            assert_eq!(
                result, expected,
                "Failed {}: {} + {} days",
                label, start, delta
            );
        }
    }

    #[test]
    fn subtract_days_works() {
        let cases: [(Day, Month, Year, Day, Day, Month, Year); 5] = [
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
            let start: Date = Date::new(start_day, start_month, start_year);
            let derived: Date = start - delta;
            let expected: Date = Date::new(expected_day, expected_month, expected_year);

            assert_eq!(derived, expected, "Failed case: {start:?} - {delta}");
        }
    }

    #[test]
    fn subtract_dates_works() {
        let cases: [(Day, Month, Year, Day, Month, Year, Day); 5] = [
            (14, Month::February, 1989, 15, Month::February, 1989, 1),
            (28, Month::February, 1989, 1, Month::March, 1989, 1),
            (31, Month::December, 1989, 1, Month::January, 1990, 1),
            (28, Month::February, 2024, 1, Month::March, 2024, 2),
            (1, Month::January, 1989, 1, Month::January, 1990, 365),
        ];

        for (d1_day, d1_month, d1_year, d2_day, d2_month, d2_year, expected_days) in cases {
            let d1: Date = Date::new(d1_day, d1_month, d1_year);
            let d2: Date = Date::new(d2_day, d2_month, d2_year);

            let derived_days: Day = d2 - d1;
            assert_eq!(derived_days, expected_days, "Failed case: {d2:?} - {d1:?}");
        }
    }

    #[test]
    fn to_serial_number_works() {
        let cases: [(Day, Month, Year); 6] = [
            (1, Month::January, 1901),   // epoch start (first usable date)
            (29, Month::February, 1904), // leap day
            (31, Month::December, 1989), // year end
            (14, Month::June, 1989),     // mid example
            (1, Month::January, 2000),   // cross century
            (31, Month::December, 2099), // far future
        ];

        for (day, month, year) in cases {
            let date: Date = Date::new(day, month, year);

            let is_leap: bool = Date::is_leap(year);
            let derived: SerialType = date.to_serial_number();
            let expected: SerialType =
                day + Date::month_offset(month as MonthIndex, is_leap) + Date::year_offset(year);

            assert_eq!(derived, expected, "Failed case: {date:?}");
        }
    }

    #[test]
    fn from_serial_number_works() {
        let cases: [Date; 4] = [
            Date::new(1, Month::January, 1956),
            Date::new(1, Month::January, 2100),
            Date::new(31, Month::December, 2099),
            Date::new(1, Month::January, 1901), // fixed epoch
        ];

        for date in cases {
            let serial_number: SerialType = date.to_serial_number();
            let derived_date: Date = Date::from_serial_number(serial_number);

            assert_eq!(derived_date, date);
        }
    }

    #[test]
    fn to_serial_number_vs_quantlib() {
        let cases: [(Day, Month, Year, SerialType); 7] = [
            // serial 0
            (1, Month::January, 1901, 367),     //
            (31, Month::December, 1901, 731),   // non-leap year end
            (1, Month::January, 1902, 732),     // start of next year
            (29, Month::February, 1904, 1521),  // leap day
            (31, Month::December, 1989, 32873), // checked against QuantLib
            (1, Month::January, 2000, 36526),   // millennium
            (31, Month::December, 2099, 73050), // far future
        ];

        for (day, month, year, expected_serial) in cases {
            let date: Date = Date::new(day, month, year);
            let derived: SerialType = date.to_serial_number();

            assert_eq!(derived, expected_serial, "Failed case: {date:?}");
        }
    }

    #[test]
    fn from_index_gives_correct_month() {
        let cases: [(MonthIndex, Month); 12] = [
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
            let derived: Month = Month::from_index(num);
            assert_eq!(derived, expected, "Failed for number {}", num);
        }
    }

    #[test]
    fn from_index_invalid_panics() {
        let invalid_cases: [MonthIndex; 3] = [0, 13, 99];
        for num in invalid_cases {
            let result = panic::catch_unwind(|| {
                Month::from_index(num);
            });
            assert!(
                result.is_err(),
                "Expected panic for invalid month {} but got Ok",
                num
            );
        }
    }

    #[test]
    fn is_leap_true() {
        let leap_years: [Year; 4] = [2000, 1928, 1956, 1900]; //1900
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
        let not_leap_years: [Year; 3] = [1945, 1999, 1901]; //1900 
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
    fn is_leap_invalid_panics() {
        let invalid_years: [Year; 2] = [1700, 2500];

        for year in invalid_years {
            let result = panic::catch_unwind(|| {
                Date::is_leap(year);
            });

            assert!(
                result.is_err(),
                "Expected panic for invalid year {} but got Ok",
                year
            );
        }
    }

    #[test]
    fn month_length_not_leap_year() {
        let cases: [(Day, Month); 12] = [
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
                Date::month_length(month as MonthIndex, false),
                days,
                "Month {} for year {} does not have right number of days {}",
                month,
                2001,
                days
            );
        }
    }

    #[test]
    fn month_length_leap_year() {
        let cases: [(Day, Month); 12] = [
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
                Date::month_length(month as MonthIndex, true),
                days,
                "Month {} for year {} does not have right number of days {}",
                month,
                2000,
                days
            );
        }
    }

    #[test]
    fn from_serial_number_invalid_panics() {
        let invalid_serials: [SerialType; 3] = [Date::MIN_SERIAL - 1, 0, Date::MAX_SERIAL + 1];

        for n in invalid_serials {
            let result = panic::catch_unwind(|| {
                Date::from_serial_number(n);
            });

            assert!(
                result.is_err(),
                "Expected panic for invalid serial {} but got Ok",
                n
            );
        }
    }

    #[test]
    fn min_date_returns_expected() {
        let d: Date = Date::min_date();
        assert_eq!(d.to_serial_number(), Date::MIN_SERIAL);
        assert_eq!(d, Date::new(1, Month::January, 1901));
    }

    #[test]
    fn max_date_returns_expected() {
        let d: Date = Date::max_date();
        assert_eq!(d.to_serial_number(), Date::MAX_SERIAL);
        assert_eq!(d, Date::new(31, Month::December, 2199));
    }

    #[test]
    fn display_month_outputs_correct_abbreviation() {
        let cases: [(Month, &str); 12] = [
            (Month::January, "January"),
            (Month::February, "February"),
            (Month::March, "March"),
            (Month::April, "April"),
            (Month::May, "May"),
            (Month::June, "June"),
            (Month::July, "July"),
            (Month::August, "August"),
            (Month::September, "September"),
            (Month::October, "October"),
            (Month::November, "November"),
            (Month::December, "December"),
        ];

        for (month, expected) in cases {
            assert_eq!(
                format!("{}", month),
                expected,
                "Failed for month {:?}",
                month
            );
        }
    }

    #[test]
    fn display_date_outputs_correct_format() {
        let cases: [(Date, &str); 11] = [
            (Date::new(1, Month::January, 1901), "January 1st, 1901"), // epoch
            (Date::new(15, Month::May, 1989), "May 15th, 1989"),       // mid example
            (Date::new(29, Month::February, 2000), "February 29th, 2000"), // leap year
            (Date::new(31, Month::December, 2199), "December 31st, 2199"), // max supported
            (Date::new(1, Month::January, 2024), "January 1st, 2024"),
            (Date::new(2, Month::January, 2024), "January 2nd, 2024"),
            (Date::new(3, Month::January, 2024), "January 3rd, 2024"),
            (Date::new(4, Month::January, 2024), "January 4th, 2024"),
            (Date::new(11, Month::January, 2024), "January 11th, 2024"),
            (Date::new(12, Month::January, 2024), "January 12th, 2024"),
            (Date::new(13, Month::January, 2024), "January 13th, 2024"),
        ];

        for (date, expected) in cases {
            assert_eq!(format!("{}", date), expected, "Failed for date {:?}", date);
        }
    }

    #[test]
    fn long_date_outputs_correct_format() {
        let cases: [(Date, &str); 11] = [
            (Date::new(1, Month::January, 1901), "January 1st, 1901"), // epoch
            (Date::new(15, Month::May, 1989), "May 15th, 1989"),       // mid example
            (Date::new(29, Month::February, 2000), "February 29th, 2000"), // leap year
            (Date::new(31, Month::December, 2199), "December 31st, 2199"), // max supported
            (Date::new(1, Month::January, 2024), "January 1st, 2024"),
            (Date::new(2, Month::January, 2024), "January 2nd, 2024"),
            (Date::new(3, Month::January, 2024), "January 3rd, 2024"),
            (Date::new(4, Month::January, 2024), "January 4th, 2024"),
            (Date::new(11, Month::January, 2024), "January 11th, 2024"),
            (Date::new(12, Month::January, 2024), "January 12th, 2024"),
            (Date::new(13, Month::January, 2024), "January 13th, 2024"),
        ];

        for (date, expected) in cases {
            assert_eq!(
                format!("{}", io::long_date(&date)),
                expected,
                "Failed for date {:?}",
                date
            );
        }
    }
    #[test]
    fn short_date_outputs_correct_format() {
        let cases: [(Date, &str); 4] = [
            (Date::new(1, Month::January, 1901), "01/01/1901"), // epoch
            (Date::new(15, Month::May, 1989), "05/15/1989"),    // mid example
            (Date::new(29, Month::February, 2000), "02/29/2000"), // leap year
            (Date::new(31, Month::December, 2199), "12/31/2199"), // max supported
        ];

        for (date, expected) in cases {
            assert_eq!(
                format!("{}", io::short_date(&date)),
                expected,
                "Failed for date {:?}",
                date
            );
        }
    }

    #[test]
    fn iso_date_outputs_correct_format() {
        let cases: [(Date, &str); 4] = [
            (Date::new(1, Month::January, 1901), "1901-01-01"), // epoch
            (Date::new(15, Month::May, 1989), "1989-05-15"),    // mid example
            (Date::new(29, Month::February, 2000), "2000-02-29"), // leap year
            (Date::new(31, Month::December, 2199), "2199-12-31"), // max supported
        ];

        for (date, expected) in cases {
            assert_eq!(
                format!("{}", io::iso_date(&date)),
                expected,
                "Failed for date {:?}",
                date
            );
        }
    }

    #[test]
    fn year_length_works() {
        let cases: [(Year, i32); 4] = [
            (1900, 366), // Fake leap year
            (1901, 365), // No leap
            (2000, 366), // Leap year
            (2100, 365), // No Leap
        ];

        for (year, expected) in cases {
            assert_eq!(
                Date::year_lenght(year),
                expected,
                "Failed for year {:?}",
                year
            );
        }
    }
    #[test]
    fn is_end_of_month_works() {
        let cases: [(Date, bool); 6] = [
            // True cases
            (Date::new(31, Month::January, 2024), true), // 31-day month
            (Date::new(30, Month::April, 2024), true),   // 30-day month
            (Date::new(29, Month::February, 2020), true), // leap-year Feb
            // False cases
            (Date::new(30, Month::January, 2024), false), // not last day
            (Date::new(28, Month::February, 2020), false), // leap year, not last day
            (Date::new(27, Month::February, 2021), false), // non-leap year
        ];

        for (date, expected) in cases {
            assert_eq!(
                date.is_end_of_month(),
                expected,
                "Failed for date {:?}",
                date
            );
        }
    }

    #[test]
    fn end_of_month_returns_correct_date() {
        let cases: [(Date, Date); 3] = [
            (
                Date::new(15, Month::January, 2024),
                Date::new(31, Month::January, 2024),
            ),
            (
                Date::new(10, Month::February, 2020), // leap year
                Date::new(29, Month::February, 2020),
            ),
            (
                Date::new(10, Month::February, 2021), // non-leap year
                Date::new(28, Month::February, 2021),
            ),
        ];

        for (input, expected) in cases {
            let result = input.end_of_month();
            assert_eq!(result, expected, "Failed for date {:?}", input);
        }
    }

    #[test]
    fn todays_date_matches_system_time() {
        // Get today using chrono
        let chrono_today: NaiveDate = Local::now().date_naive();

        let expected = Date::new(
            chrono_today.day() as Day,
            Month::from_index(chrono_today.month() as MonthIndex),
            chrono_today.year(),
        );

        let derived: Date = Date::todays_date();

        // 1. Check equality with chrono-based date
        assert_eq!(
            derived, expected,
            "todays_date() mismatch: expected {:?}, got {:?}",
            expected, derived
        );

        // 2. Check range boundaries
        assert!(
            derived >= Date::min_date() && derived <= Date::max_date(),
            "todays_date() out of valid QuantLib range: {:?}",
            derived
        );

        // 3. Check round-trip serial number consistency
        let round_trip = Date::from_serial_number(derived.to_serial_number());
        assert_eq!(
            round_trip, derived,
            "Round-trip serial_number failed for todays_date()"
        );
    }

    #[test]
    fn add_assign_days_works() {
        let cases: [(Date, Day, Date); 3] = [
            // Within same month
            (
                Date::new(1, Month::January, 2024),
                10,
                Date::new(11, Month::January, 2024),
            ),
            // Across month boundary
            (
                Date::new(25, Month::January, 2024),
                10,
                Date::new(4, Month::February, 2024),
            ),
            // Across year boundary
            (
                Date::new(31, Month::December, 2023),
                1,
                Date::new(1, Month::January, 2024),
            ),
        ];

        for (mut start, delta, expected) in cases {
            start += delta;
            assert_eq!(
                start, expected,
                "Failed case: start +={} {:?}",
                delta, start
            );
        }
    }

    #[test]
    fn sub_assign_days_works() {
        let cases: [(Date, Day, Date); 3] = [
            // Within same month
            (
                Date::new(15, Month::January, 2024),
                5,
                Date::new(10, Month::January, 2024),
            ),
            // Across month boundary
            (
                Date::new(5, Month::February, 2024),
                10,
                Date::new(26, Month::January, 2024),
            ),
            // Across year boundary
            (
                Date::new(1, Month::January, 2024),
                1,
                Date::new(31, Month::December, 2023),
            ),
        ];

        for (mut start, delta, expected) in cases {
            start -= delta;
            assert_eq!(
                start, expected,
                "Failed case: start -={} {:?}",
                delta, start
            );
        }
    }

    #[test]
    fn test_next_weekday_edge_cases() {
        let cases: [(Date, Weekday, Date); 3] = [
            // (start_date, target_weekday, expected_date)
            (
                Date::new(21, Month::August, 2024),
                Weekday::Friday,
                Date::new(23, Month::August, 2024),
            ), // Wed → Fri
            (
                Date::new(23, Month::August, 2024),
                Weekday::Wednesday,
                Date::new(28, Month::August, 2024),
            ), // Fri → Wed next week
            (
                Date::new(23, Month::August, 2024),
                Weekday::Friday,
                Date::new(23, Month::August, 2024),
            ), // Fri → Fri (same date)
        ];

        for (start, target, expected) in cases {
            let result: Date = start.next_weekday(target);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_nth_weekday_edge_cases() {
        let cases: [(usize, Weekday, Month, Year, Date); 3] = [
            // (nth, weekday, month, year, expected_date)
            (
                1,
                Weekday::Monday,
                Month::May,
                2025,
                Date::new(5, Month::May, 2025),
            ), // 1st Monday of May 2025
            (
                3,
                Weekday::Friday,
                Month::September,
                2024,
                Date::new(20, Month::September, 2024),
            ), // 3rd Friday of Sep 2024
            (
                5,
                Weekday::Friday,
                Month::March,
                2024,
                Date::new(29, Month::March, 2024),
            ), // 5th Friday in March 2024 (leap year)
        ];

        for (nth, wd, month, year, expected) in cases {
            let result: Date = Date::nth_weekday(nth, wd, month, year);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn nth_weekday_invalid_panics() {
        let cases: [(usize, Weekday, Month, i32); 3] = [
            (0, Weekday::Monday, Month::January, 2025), // nth = 0 → invalid
            (6, Weekday::Monday, Month::January, 2025), // nth = 6 → invalid
            (5, Weekday::Monday, Month::February, 2025), // Feb 2025 has only 4 Mondays
        ];

        for (nth, wd, month, year) in cases {
            let result = panic::catch_unwind(|| {
                Date::nth_weekday(nth, wd, month, year);
            });

            assert!(
                result.is_err(),
                "Expected panic for nth={nth}, wd={wd:?}, month={month:?}, year={year} but got Ok"
            );
        }
    }

    #[test]
    fn test_increment_normal_cases() {
        let cases: [(Date, Date); 3] = [
            (
                Date::new(14, Month::February, 1989),
                Date::new(15, Month::February, 1989),
            ), // simple
            (
                Date::new(31, Month::January, 1989),
                Date::new(1, Month::February, 1989),
            ), // month boundary
            (
                Date::new(31, Month::December, 1989),
                Date::new(1, Month::January, 1990),
            ), // year boundary
        ];

        for (input, expected) in cases {
            let mut d = input;
            d.increment();
            assert_eq!(d, expected, "Increment failed: start={:?}", input);
        }
    }

    #[test]
    fn test_increment_panic_cases() {
        let cases: [Date; 1] = [
            Date::new(31, Month::December, 2199), // beyond max date
        ];

        for input in cases {
            let result = panic::catch_unwind(|| {
                let mut d = input;
                d.increment();
            });
            assert!(
                result.is_err(),
                "Increment panic failed: start={:?} did not panic",
                input
            );
        }
    }

    #[test]
    fn test_decrement_normal_cases() {
        let cases: [(Date, Date); 4] = [
            (
                Date::new(14, Month::February, 1989),
                Date::new(13, Month::February, 1989),
            ), // simple
            (
                Date::new(1, Month::January, 1990),
                Date::new(31, Month::December, 1989),
            ), // year boundary
            (
                Date::new(1, Month::March, 1989),
                Date::new(28, Month::February, 1989),
            ), // non-leap year
            (
                Date::new(1, Month::March, 1988),
                Date::new(29, Month::February, 1988),
            ), // leap year
        ];

        for (input, expected) in cases {
            let mut d = input;
            d.decrement();
            assert_eq!(d, expected, "Decrement failed: start={:?}", input);
        }
    }

    #[test]
    fn test_decrement_panic_cases() {
        let cases: [Date; 1] = [
            Date::new(1, Month::January, 1901), // before min date
        ];

        for input in cases {
            let result = panic::catch_unwind(|| {
                let mut d = input;
                d.decrement();
            });
            assert!(
                result.is_err(),
                "Decrement panic failed: start={:?} did not panic",
                input
            );
        }
    }

    #[test]
    fn test_formatted_date_normal_cases() {
        let cases: [(Date, &str, &str); 3] = [
            (
                Date::new(14, Month::February, 1989),
                "%Y-%m-%d",
                "1989-02-14",
            ),
            (
                Date::new(14, Month::February, 1989),
                "%m/%d/%Y",
                "02/14/1989",
            ),
            (
                Date::new(14, Month::February, 1989),
                "%B %d, %Y",
                "February 14, 1989",
            ),
        ];

        for (input, format, expected) in cases {
            let result = format!("{}", io::formatted_date(&input, format));
            assert_eq!(
                result, expected,
                "Formatted date failed: date={:?}, format={}",
                input, format
            );
        }
    }

    #[test]
    fn test_formatted_date_null_cases() {
        let cases: [(Date, &str, &str); 1] = [(Date::default(), "%Y-%m-%d", "null date")];

        for (input, format, expected) in cases {
            let result = format!("{}", io::formatted_date(&input, format));
            assert_eq!(
                result, expected,
                "Formatted date null failed: date={:?}, format={}",
                input, format
            );
        }
    }
    #[test]
    fn test_formatted_date_invalid_date_panics() {
        let cases: [Date; 1] = [
            Date { serial_number: -1 }, // bypass Date::new()
        ];

        for input in cases {
            let result = panic::catch_unwind(|| {
                let _ = format!("{}", io::formatted_date(&input, "%Y-%m-%d"));
            });
            assert!(
                result.is_err(),
                "Formatted date invalid case failed: date={:?} did not panic",
                input
            );
        }
    }
}
