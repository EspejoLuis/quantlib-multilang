use std::fmt;
pub type WeekDayIndex = i32;
use std::ops::{Add, Sub}; // Addition, Subtraction

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)] // makes it laid out exactly as 1–7
pub enum Weekday {
    Sunday = 1,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
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
    pub fn from_i32(number: WeekDayIndex) -> Option<Self> {
        if (1..=7).contains(&number) {
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

// Traits
impl fmt::Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // `&'static str` means a reference to a string literal that
        // lives for the entire program (string literals never expire).
        // Example: "Jan" is compiled into the binary and always safe.
        write!(f, "{}", self.long_weekday())
    }
}
impl Add<WeekDayIndex> for Weekday {
    type Output = Weekday;
    fn add(self, right_hand_side: WeekDayIndex) -> Weekday {
        Weekday::from_i32(self as WeekDayIndex + right_hand_side).unwrap()
    }
}
impl Sub<WeekDayIndex> for Weekday {
    type Output = Weekday;

    fn sub(self, right_hand_side: WeekDayIndex) -> Weekday {
        Weekday::from_i32(self as WeekDayIndex - right_hand_side).unwrap()
    }
}
// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_weekday_outputs_correct_names() {
        let cases: [(Weekday, &str); 7] = [
            (super::Weekday::Sunday, "Sunday"),
            (Weekday::Monday, "Monday"),
            (Weekday::Tuesday, "Tuesday"),
            (Weekday::Wednesday, "Wednesday"),
            (Weekday::Thursday, "Thursday"),
            (Weekday::Friday, "Friday"),
            (Weekday::Saturday, "Saturday"),
        ];

        for (weekday, expected) in cases {
            assert_eq!(weekday.long_weekday(), expected, "Failed for {:?}", weekday);
        }
    }

    #[test]
    fn short_weekday_outputs_correct_abbreviations() {
        let cases: [(Weekday, &str); 7] = [
            (Weekday::Sunday, "Sun"),
            (Weekday::Monday, "Mon"),
            (Weekday::Tuesday, "Tue"),
            (Weekday::Wednesday, "Wed"),
            (Weekday::Thursday, "Thu"),
            (Weekday::Friday, "Fri"),
            (Weekday::Saturday, "Sat"),
        ];

        for (weekday, expected) in cases {
            assert_eq!(
                weekday.short_weekday(),
                expected,
                "Failed for {:?}",
                weekday
            );
        }
    }

    #[test]
    fn shortest_weekday_outputs_correct_abbreviations() {
        let cases: [(Weekday, &str); 7] = [
            (Weekday::Sunday, "Su"),
            (Weekday::Monday, "Mo"),
            (Weekday::Tuesday, "Tu"),
            (Weekday::Wednesday, "We"),
            (Weekday::Thursday, "Th"),
            (Weekday::Friday, "Fr"),
            (Weekday::Saturday, "Sa"),
        ];

        for (weekday, expected) in cases {
            assert_eq!(
                weekday.shortest_weekday(),
                expected,
                "Failed for {:?}",
                weekday
            );
        }
    }

    #[test]
    fn display_trait_outputs_long_weekday() {
        let cases: [(Weekday, &str); 7] = [
            (Weekday::Sunday, "Sunday"),
            (Weekday::Monday, "Monday"),
            (Weekday::Tuesday, "Tuesday"),
            (Weekday::Wednesday, "Wednesday"),
            (Weekday::Thursday, "Thursday"),
            (Weekday::Friday, "Friday"),
            (Weekday::Saturday, "Saturday"),
        ];

        for (weekday, expected) in cases {
            assert_eq!(format!("{}", weekday), expected, "Failed for {:?}", weekday);
        }
    }
}
