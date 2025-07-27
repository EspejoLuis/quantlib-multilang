/*
Since the struct contains types that implement equality
(u32) then Rust automatically generates == logic
i.e. each field is compared in ORDER (day, month, year)
if all fiels are equal then true

PartialEq vs Eq:
    - PartialEq gives == and != logic
    - Eq: does not give anything more but 
        confirms that == logic behaves mathematically sensibly
        For example a == a can be false if a is Nan. By saying Eq
        that possibility is excluded a priori. 
*/

/*
With PartialOrd and Ord:
    - PartialOrd --> Enables <, <=, >, >=
    - Ord --> Enables full ordering (like sorting)
*/
/*
Using asser_eq!(d1, d1, "xxx") mean Rust will try to show the 
value when the test fails but to do that `Debug` is needed
*/

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
/*
This can be done!
    if date1 == date2 {
        println!("Dates are equal!");
    }
*/
// defines a struct named Date, just like a class in C++ or C# with only data (no methods yet).
pub struct Date {
    // pub --> public so they can be access by other files like main.rs
    // unsigned 32-bit integer 

    // Order below impact for example how operator < works.
    // First field to be check is the first one below.
    pub year: u32,
    pub month: u32,
    pub day: u32,

}

// Implementation block i.e. to have a constructor
impl Date {
    pub fn new(day: u32, month: u32, year: u32) -> Date {
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
        */        
        Date {day, month, year}
    }

    /*
    Rust does not have built-in calendar logic in std 
        - like Python’s datetime 
        - or C#’s DateTime.
    To make operator + work, two methods need to be implemented:
        - A method to_serial() that gives an integer day count.
        - A method from_serial(n: u32) -> Date that builds a date from that count.
    */

    pub fn to_serial(&self) -> u32{
        // Assum each month has 30 days and each year has 360 days.
        // This avoids needing leap year or real calendar logic for now.
        self.year * 360 + self.month * 30 + self.day
        /* Alternatively:
        let serial = self.year * 360 + self.month * 30 + self.day
        serial
        */
    }

    pub fn from_serial(n: u32) -> Date {
        // Assum each month has 30 days and each year has 360 days.
        let year = n / 360;
        let month = (n % 360) / 30;
        let day = (n % 360) % 30;

        Date::new(day, month, year)
    }



}

use std::fmt;

/*
- impl: we are implementing something.
- fmt::Display: this is a trait:
    - like an interface in C# or abstract base class in C++.
- for Date:
    - the Display trait is implemented for Date struct.
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}-{}", self.day, self.month, self.year)
    }
}

use std::ops::Add;

/*
Implementing the behavior of the + operator where:
  - the left-hand side is Date
  - the right-hand side is i32

This (impl Add<i32> for Date) is similar to:
    - C++: Date operator-(int n) const;
    - C#: public static Date operator -(Date d, int n)
*/

// Add days to date
impl Add<i32> for Date{
    
    type Output = Date;
    // i32 means it can be NEGATIVE!
    fn add(self, rhs: i32) -> Date {
        let serial_i32 = self.to_serial() as i32;
        // rhs and serial cannot be added 
        // i32 vs u32
        let new_serial = serial_i32 + rhs;
        // Put Check
        assert!(new_serial >= 0, "New date is before base date");

        Date::from_serial(new_serial as u32)
    }
}

use std::ops::Sub;

// Subtract days from date
impl Sub<i32> for Date{
    type Output = Date;
    // i32 means it can be NEGATIVE!
    fn sub(self, rhs: i32) -> Date {
        let serial_i32 = self.to_serial() as i32;
        // rhs and serial cannot be added 
        // i32 vs u32
        let new_serial = serial_i32 - rhs;
        // Put Check
        assert!(new_serial >= 0, "New date is before base date");

        Date::from_serial(new_serial as u32)
    }
}

// Subtract dates
impl Sub<Date> for Date{
    type Output = i32;

    fn sub(self, rhs: Date) -> i32 {
        let rhs_i32 = rhs.to_serial() as i32;
        let serial_i32 = self.to_serial() as i32;

        serial_i32 - rhs_i32
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
        let d1 = Date::new(1, 05, 1989);

        assert_eq!(d1.day, 1);
        assert_eq!(d1.month, 05);
        assert_eq!(d1.year, 1989);
    }

    #[test]
    fn display_date_correctly(){
        let d1 = Date::new(14, 5, 1989);
        let result = format!("{}", d1);

        assert_eq!(result, "14-5-1989");
    }

    #[test]
    fn equality_works_when_fields_match(){
        let d1 = Date::new(14, 5, 1989);
        let d2 = Date::new(14, 5, 1989);

        assert_eq!(d1, d2, "Dates should be equal");

        let d3 = Date::new(15, 5, 1989);
        
        assert_ne!(d1, d3, "Dates should not be equal");
        
    }
    
    #[test]
    fn date_comparison_works(){
        let d1 = Date::new(14, 5, 1989);
        let d2 = Date::new(17, 5, 1989);

        assert!(d1 < d2);

        let d3 = Date::new(13, 8, 1989);

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
    fn add_days_works_correctly(){
        let d1 = Date::new(1, 5, 1989);
        let derived_date = d1 + 40;

        // We made assumption 30 days per month
        let expected_date = Date::new(11, 6, 1989);

        assert_eq!(derived_date, expected_date);

    }

    #[test]
    fn subtract_days_works_correctly(){
        let d1 = Date::new(15, 5, 1989);
        // Subtracting 15 days will result in 0 May
        // Subtracitng 16 days will result in 29 May
        // so when is 30 May ?
        let derived_date = d1 - 16;

        // We made assumption 30 days per month
        let expected_date = Date::new(29, 4, 1989);

        assert_eq!(derived_date, expected_date);

    }

    #[test]
    fn subtract_dates_works_correctly(){
        let d1 = Date::new(14, 5, 1989);
        let d2 = Date::new(15, 5, 1989);

        let derived_days = d2 - d1;
        let expected_days = 1;

        assert_eq!(derived_days, expected_days);
    }

    #[test]
    fn to_serial_works_correctly(){
        let d = Date::new(14, 5, 1989);

        let derived_serial = d.to_serial();
        let expected_serial = 14 + 5 * 30 + 1989 * 360;

        assert_eq!(derived_serial, expected_serial);
    }

    #[test]
    fn from_serial_works_correctly(){
        let serial = 11 + 5 * 30 + 1989 * 360;
        let derived_date = Date::from_serial(serial);
        let expected_date = Date::new(11, 5, 1989);
        
        assert_eq!(derived_date, expected_date);
    }
}