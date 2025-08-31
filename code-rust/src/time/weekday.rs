pub type WeekDayIndex = usize;
use std::fmt::{Display, Formatter, Result};
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
    pub fn from_index(number: WeekDayIndex) -> Self {
        if (1..=7).contains(&number) {
            /*
            transmute --> low-level cast that tells the compiler.
            transmute is unsafe because if you give it a value outside 1..=12,
            it would create an invalid enum value, which leads to UB (undefined behavior).
            unsafe --> "the contract necessary to call the operations inside the block has been
            checked by the programmer and is guaranteed to be respected"
            */
            unsafe { std::mem::transmute(number as u8) }
        } else {
            panic!("Weekday index {} out of range [1,7]", number);
        }
    }
}

// Traits
impl Display for Weekday {
    fn fmt(&self, formatter_buffer: &mut Formatter) -> Result {
        write!(formatter_buffer, "{}", io::long_weekday(*self))
    }
}
impl Add<WeekDayIndex> for Weekday {
    type Output = Weekday;
    fn add(self, right_hand_side: WeekDayIndex) -> Weekday {
        Weekday::from_index(self as WeekDayIndex + right_hand_side)
    }
}
impl Sub<WeekDayIndex> for Weekday {
    type Output = Weekday;

    fn sub(self, right_hand_side: WeekDayIndex) -> Weekday {
        Weekday::from_index(self as WeekDayIndex - right_hand_side)
    }
}

// Private
mod detail {
    use super::Weekday;
    use std::fmt;

    pub(crate) struct LongWeekday {
        pub(crate) weekday: Weekday,
    }
    pub(crate) struct ShortWeekday {
        pub(crate) weekday: Weekday,
    }
    pub(crate) struct ShortestWeekday {
        pub(crate) weekday: Weekday,
    }

    // impl Display is not a string, it’s just a wrapper that knows how to print itself.
    impl fmt::Display for LongWeekday {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let weekday: &Weekday = &self.weekday;
            let long_weekday: &'static str = match weekday {
                Weekday::Monday => "Monday",
                Weekday::Tuesday => "Tuesday",
                Weekday::Wednesday => "Wednesday",
                Weekday::Thursday => "Thursday",
                Weekday::Friday => "Friday",
                Weekday::Saturday => "Saturday",
                Weekday::Sunday => "Sunday",
            };
            write!(f, "{}", long_weekday)
        }
    }
    impl fmt::Display for ShortWeekday {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let weekday: &Weekday = &self.weekday;
            let short_weekday: &'static str = match weekday {
                Weekday::Monday => "Mon",
                Weekday::Tuesday => "Tue",
                Weekday::Wednesday => "Wed",
                Weekday::Thursday => "Thu",
                Weekday::Friday => "Fri",
                Weekday::Saturday => "Sat",
                Weekday::Sunday => "Sun",
            };
            write!(f, "{}", short_weekday)
        }
    }
    impl fmt::Display for ShortestWeekday {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let weekday: &Weekday = &self.weekday;
            let shortest_weekday: &'static str = match weekday {
                Weekday::Monday => "Mo",
                Weekday::Tuesday => "Tu",
                Weekday::Wednesday => "We",
                Weekday::Thursday => "Th",
                Weekday::Friday => "Fr",
                Weekday::Saturday => "Sa",
                Weekday::Sunday => "Su",
            };
            write!(f, "{}", shortest_weekday)
        }
    }
}

// Public API
pub(crate) mod io {
    use super::{Weekday, detail};

    pub fn long_weekday(wd: Weekday) -> impl std::fmt::Display {
        detail::LongWeekday { weekday: wd }
    }
    pub fn short_weekday(wd: Weekday) -> impl std::fmt::Display {
        detail::ShortWeekday { weekday: wd }
    }
    pub fn shortest_weekday(wd: Weekday) -> impl std::fmt::Display {
        detail::ShortestWeekday { weekday: wd }
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::io; // brings the io module into scope
    use super::*; // brings Weekday, WeekDayIndex, etc. into scope
    use std::panic;

    #[test]
    fn long_weekday_outputs_correct_names() {
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
            assert_eq!(
                format!("{}", io::long_weekday(weekday)),
                expected,
                "Failed for {:?}",
                weekday
            );
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
                format!("{}", io::short_weekday(weekday)),
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
                format!("{}", io::shortest_weekday(weekday)),
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

    #[test]
    fn from_index_valid_inputs() {
        let cases: [(WeekDayIndex, Weekday); 7] = [
            (1, Weekday::Sunday),
            (2, Weekday::Monday),
            (3, Weekday::Tuesday),
            (4, Weekday::Wednesday),
            (5, Weekday::Thursday),
            (6, Weekday::Friday),
            (7, Weekday::Saturday),
        ];

        for (num, expected) in cases {
            assert_eq!(Weekday::from_index(num), expected);
        }
    }

    #[test]
    fn from_index_invalid_inputs_panic() {
        let invalid_cases: [WeekDayIndex; 3] = [0, 8, 100];
        for num in invalid_cases {
            let result = panic::catch_unwind(|| {
                Weekday::from_index(num);
            });

            assert!(
                result.is_err(),
                "Expected panic for input {} but got Ok",
                num
            );
        }
    }

    #[test]
    fn add_weekday_within_bounds() {
        assert_eq!(Weekday::Sunday + 1, Weekday::Monday);
        assert_eq!(Weekday::Friday + 1, Weekday::Saturday);
    }

    #[test]
    fn sub_weekday_within_bounds() {
        assert_eq!(Weekday::Tuesday - 1, Weekday::Monday);
        assert_eq!(Weekday::Monday - 1, Weekday::Sunday);
    }

    #[test]
    fn add_weekday_out_of_bounds_panics() {
        let cases = [
            (Weekday::Saturday, 1), // 7 + 1 = 8 → invalid
            (Weekday::Friday, 3),   // 6 + 3 = 9 → invalid
        ];

        for (weekday, addend) in cases {
            let result = panic::catch_unwind(|| {
                let _ = weekday + addend;
            });

            assert!(
                result.is_err(),
                "Expected panic for {:?} + {} but got Ok",
                weekday,
                addend
            );
        }
    }

    #[test]
    fn sub_weekday_out_of_bounds_panics() {
        let cases = [
            (Weekday::Sunday, 1), // 1 - 1 = 0 → invalid
            (Weekday::Monday, 5), // 2 - 5 = -3 → invalid
        ];

        for (weekday, subtrahend) in cases {
            let result = panic::catch_unwind(|| {
                let _ = weekday - subtrahend;
            });

            assert!(
                result.is_err(),
                "Expected panic for {:?} - {} but got Ok",
                weekday,
                subtrahend
            );
        }
    }
}
