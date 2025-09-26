use crate::time::frequency::Frequency;
use crate::time::time_unit::TimeUnit;
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::fmt::{Display, Formatter, Result};
use std::ops::Neg;
use std::ops::{Add, AddAssign, Div, DivAssign, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Period {
    length: i32, // Can be negative...basically saying to go back 3 months
    units: TimeUnit,
}
impl Period {
    // Constructor
    pub fn new(length: i32, units: TimeUnit) -> Period {
        Period { length, units }
    }
    pub fn from_frequency(frequency: Frequency) -> Period {
        // Frequency -> Period
        match frequency {
            Frequency::NoFrequency => Period::new(0, TimeUnit::Days),
            Frequency::Once => Period::new(0, TimeUnit::Years),
            Frequency::Annual => Period::new(1, TimeUnit::Years),

            Frequency::Semiannual
            | Frequency::EveryFourthMonth
            | Frequency::Quarterly
            | Frequency::Bimonthly
            | Frequency::Monthly => Period::new(12 / (frequency as i32), TimeUnit::Months),

            Frequency::EveryFourthWeek | Frequency::Biweekly | Frequency::Weekly => {
                Period::new(52 / (frequency as i32), TimeUnit::Weeks)
            }

            Frequency::Daily => Period::new(1, TimeUnit::Days),
            Frequency::OtherFrequency => panic!("unknown frequency"),
        }
    }

    // Inspectors public
    pub fn length(&self) -> i32 {
        self.length
    }
    pub fn units(&self) -> TimeUnit {
        self.units
    }
    pub fn frequency(&self) -> Frequency {
        // Period -> Frequency
        let abs_length: u32 = self.length.abs() as u32; // abs because length can be negative
        let units: TimeUnit = self.units;

        if abs_length == 0 {
            return if units == TimeUnit::Years {
                Frequency::Once
            } else {
                Frequency::NoFrequency
            };
        }

        match units {
            TimeUnit::Years => {
                if abs_length == 1 {
                    Frequency::Annual
                } else {
                    Frequency::OtherFrequency
                }
            }
            TimeUnit::Months => {
                if abs_length <= 12 && 12 % abs_length == 0 {
                    Frequency::from_nth_times_per_year(12 / abs_length)
                } else {
                    Frequency::OtherFrequency
                }
            }
            TimeUnit::Weeks => match abs_length {
                1 => Frequency::Weekly,
                2 => Frequency::Biweekly,
                4 => Frequency::EveryFourthWeek,
                _ => Frequency::OtherFrequency,
            },
            TimeUnit::Days => {
                if abs_length == 1 {
                    Frequency::Daily
                } else {
                    Frequency::OtherFrequency
                }
            }
        }
    }
    pub fn normalize(&mut self) {
        // If object is owned and want to mutate
        let units: TimeUnit = self.units;

        if self.length == 0 {
            self.units = TimeUnit::Days;
        } else {
            match units {
                TimeUnit::Months => {
                    if (self.length % 12) == 0 {
                        self.length /= 12;
                        self.units = TimeUnit::Years;
                    }
                }
                TimeUnit::Days => {
                    if (self.length % 7) == 0 {
                        self.length /= 7;
                        self.units = TimeUnit::Weeks;
                    }
                }
                TimeUnit::Weeks | TimeUnit::Years => {}
            }
        }
    }
    pub fn normalized(&self) -> Period {
        // Normalized copy without touching the original
        let mut period: Period = *self; // Create copy of the object
        period.normalize();
        period
    }
    pub fn years(&self) -> f64 {
        // Convert into years
        if self.length == 0 {
            return 0.0;
        }

        match self.units {
            TimeUnit::Days => panic!("cannot convert Days into Years"),
            TimeUnit::Weeks => panic!("cannot convert Weeks into Years"),
            TimeUnit::Months => self.length as f64 / 12.0,
            TimeUnit::Years => self.length as f64,
        }
    }
    pub fn months(&self) -> f64 {
        // Convert into months
        if self.length == 0 {
            return 0.0;
        }

        match self.units {
            TimeUnit::Days => panic!("cannot convert Days into Months"),
            TimeUnit::Weeks => panic!("cannot convert Weeks into Months"),
            TimeUnit::Months => self.length as f64,
            TimeUnit::Years => self.length as f64 * 12.0,
        }
    }
    pub fn weeks(&self) -> f64 {
        // Convert into weeks
        if self.length == 0 {
            return 0.0;
        }

        match self.units {
            TimeUnit::Days => self.length as f64 / 7.0,
            TimeUnit::Weeks => self.length as f64,
            TimeUnit::Months => panic!("cannot convert Months into Weeks"),
            TimeUnit::Years => panic!("cannot convert Years into Weeks"),
        }
    }
    pub fn days(&self) -> f64 {
        // Convert into days
        if self.length == 0 {
            return 0.0;
        }

        match self.units {
            TimeUnit::Days => self.length as f64,
            TimeUnit::Weeks => self.length as f64 * 7.0,
            TimeUnit::Months => panic!("cannot convert Months into Days"),
            TimeUnit::Years => panic!("cannot convert Years into Days"),
        }
    }

    // Private Static
    fn days_min_max(period: &Period) -> (i32, i32) {
        /*
        It takes a Period (length + unit) and returns a range of possible days (min_days, max_days).
        This is needed because some periods (like “1 month”) don’t map to a fixed number of days.
        */
        match period.units {
            // Min and Max are the same in days
            TimeUnit::Days => (period.length(), period.length()),
            // Min and Max are the the same in days
            TimeUnit::Weeks => (period.length() * 7, period.length() * 7),
            // Min and Max could be different in days according to the month
            TimeUnit::Months => (period.length() * 28, period.length() * 31),
            // Min and Max could be different in days according to the year
            TimeUnit::Years => (period.length() * 365, period.length() * 366),
        }
    }
}

// Traits
impl PartialOrd for Period {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.length() == 0 {
            if other.length() > 0 {
                return Some(Ordering::Less);
            } else if other.length() < 0 {
                return Some(Ordering::Greater);
            } else {
                return Some(Ordering::Equal);
            }
        }

        if other.length() == 0 {
            if self.length() < 0 {
                return Some(Ordering::Less);
            } else
            //self.length() > 0
            {
                return Some(Ordering::Greater);
            }
        }

        // --- Exact comparisons ---
        if self.units() == other.units() {
            return if self.length() < other.length() {
                Some(Ordering::Less)
            } else if self.length() > other.length() {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Equal)
            };
        }

        // --- Convertible units (Months <-> Years, Days <-> Weeks) ---
        if self.units() == TimeUnit::Months && other.units() == TimeUnit::Years {
            return if self.length() < 12 * other.length() {
                Some(Ordering::Less)
            } else if self.length() > 12 * other.length() {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Equal)
            };
        }

        if self.units() == TimeUnit::Years && other.units() == TimeUnit::Months {
            return if 12 * self.length() < other.length() {
                Some(Ordering::Less)
            } else if 12 * self.length() > other.length() {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Equal)
            };
        }

        if self.units() == TimeUnit::Days && other.units() == TimeUnit::Weeks {
            return if self.length() < 7 * other.length() {
                Some(Ordering::Less)
            } else if self.length() > 7 * other.length() {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Equal)
            };
        }

        if self.units() == TimeUnit::Weeks && other.units() == TimeUnit::Days {
            return if 7 * self.length() < other.length() {
                Some(Ordering::Less)
            } else if 7 * self.length() > other.length() {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Equal)
            };
        }

        // --- Inexact comparisons: fallback to days_min_max ---
        let (self_min, self_max) = Period::days_min_max(self);
        let (other_min, other_max) = Period::days_min_max(other);

        if self_max < other_min {
            // The largest possible value of self is still less than the smallest possible value of other.
            return Some(Ordering::Less);
        } else if self_min > other_max {
            // The smallest possible value of self is still greater than the largest possible value of other
            return Some(Ordering::Greater);
        } else {
            panic!("Undecidable comparison between {:?} and {:?}", self, other);
        }
    }
}
impl Display for Period {
    fn fmt(&self, formatter_buffer: &mut Formatter) -> Result {
        write!(formatter_buffer, "{}", io::short_period(self))
    }
}
impl AddAssign<Period> for Period {
    // AddAssign -> right hand side
    // for Period -> left hand side
    // No Output, no new Date returned. SAME Period modified!
    fn add_assign(&mut self, rhs: Period) -> () {
        if self.length == 0 {
            self.length = rhs.length();
            self.units = rhs.units();
            // if same units
        } else {
            match self.units {
                TimeUnit::Years => match rhs.units() {
                    TimeUnit::Years => self.length += rhs.length(),
                    TimeUnit::Months => {
                        // years*12 + month
                        self.units = TimeUnit::Months;
                        self.length = self.length * 12 + rhs.length();
                    }
                    TimeUnit::Weeks | TimeUnit::Days => {
                        panic!("Impossible addition between {:?} and {:?}", self, rhs)
                    }
                },
                TimeUnit::Months => match rhs.units() {
                    TimeUnit::Months => self.length += rhs.length(),
                    TimeUnit::Years => {
                        // months + years*12
                        self.length += 12 * rhs.length();
                    }
                    TimeUnit::Weeks | TimeUnit::Days => {
                        panic!("Impossible addition between {:?} and {:?}", self, rhs)
                    }
                },
                TimeUnit::Weeks => match rhs.units() {
                    TimeUnit::Weeks => self.length += rhs.length(),
                    TimeUnit::Days => {
                        // weeks*7 + days
                        self.units = TimeUnit::Days;
                        self.length = self.length * 7 + rhs.length();
                    }
                    TimeUnit::Years | TimeUnit::Months => {
                        panic!("Impossible addition between {:?} and {:?}", self, rhs)
                    }
                },
                TimeUnit::Days => match rhs.units() {
                    TimeUnit::Days => self.length += rhs.length(),
                    TimeUnit::Weeks => {
                        // days + weeks*7
                        self.length += 7 * rhs.length();
                    }
                    TimeUnit::Years | TimeUnit::Months => {
                        panic!("Impossible addition between {:?} and {:?}", self, rhs)
                    }
                },
            }
        }
    }
}
impl Add<Period> for Period {
    type Output = Period;
    fn add(mut self, rhs: Period) -> Period {
        self += rhs;
        self
    }
}
impl Neg for Period {
    // We need -a not a-b so that's why use Neg instead of Sub
    type Output = Period;
    fn neg(self) -> Period {
        // New period as output
        Period::new(-self.length, self.units)
    }
}
impl SubAssign<Period> for Period {
    // No Output, no new Date returned. SAME Period modified!
    fn sub_assign(&mut self, rhs: Period) -> () {
        *self += -rhs
    }
}
impl Sub<Period> for Period {
    type Output = Period;
    fn sub(mut self, rhs: Period) -> Period {
        self += -rhs;
        self
    }
}

// Traits - i32 for Period
impl DivAssign<i32> for Period {
    fn div_assign(&mut self, divider: i32) {
        if divider == 0 {
            panic!("cannot be divided by zero");
        }
        // Assumption:
        if self.length % divider == 0 {
            // clean division, keep units
            self.length /= divider;
        } else {
            let mut new_units: TimeUnit = self.units;
            let mut new_length: i32 = self.length;

            match self.units {
                TimeUnit::Years => {
                    new_length *= 12;
                    new_units = TimeUnit::Months;
                }
                TimeUnit::Weeks => {
                    new_length *= 7;
                    new_units = TimeUnit::Days;
                }
                _ => { /* Days, Months — no conversion attempted */ }
            }

            if new_length % divider != 0 {
                panic!("{:?} cannot be divided by {}", self, divider);
            }

            self.length = new_length / divider;
            self.units = new_units;
        }
    }
}
impl Div<i32> for Period {
    type Output = Period;
    fn div(mut self, divider: i32) -> Period {
        // += /= *= -= always return the same object modified
        // + / * / always return a new object
        self /= divider;
        self
    }
}
impl MulAssign<i32> for Period {
    // No Output, no new Date returned. SAME Period modified!
    fn mul_assign(&mut self, multiplier: i32) -> () {
        // Scale the length
        self.length *= multiplier
    }
}

// Private
mod detail {
    use super::Period;
    use super::*;
    use std::fmt::{Display, Formatter, Result};

    pub(crate) struct LongPeriod<'a> {
        pub(crate) period: &'a Period,
    }
    pub(crate) struct ShortPeriod<'a> {
        pub(crate) period: &'a Period,
    }

    // impl Display is not a string, it’s just a wrapper that knows how to print itself.
    impl<'a> Display for LongPeriod<'a> {
        fn fmt(&self, f: &mut Formatter) -> Result {
            let length: i32 = self.period.length();
            let long_length: &'static str = match &self.period.units() {
                TimeUnit::Days => {
                    if length == 1 {
                        "Day"
                    } else {
                        "Days"
                    }
                }
                TimeUnit::Weeks => {
                    if length == 1 {
                        "Week"
                    } else {
                        "Weeks"
                    }
                }
                TimeUnit::Months => {
                    if length == 1 {
                        "Month"
                    } else {
                        "Months"
                    }
                }
                TimeUnit::Years => {
                    if length == 1 {
                        "Year"
                    } else {
                        "Years"
                    }
                }
            };
            write!(f, "{} {}", length, long_length)
        }
    }
    impl<'a> Display for ShortPeriod<'a> {
        fn fmt(&self, f: &mut Formatter) -> Result {
            let length: i32 = self.period.length();
            let short_length: &'static str = match &self.period.units() {
                TimeUnit::Days => "D",
                TimeUnit::Weeks => "W",
                TimeUnit::Months => "M",
                TimeUnit::Years => "Y",
            };
            write!(f, "{}{}", length, short_length)
        }
    }
}

// Public API
pub(crate) mod io {
    use super::{Period, detail};
    use std::fmt::Display;

    pub fn long_period<'a>(p: &'a Period) -> impl Display + 'a {
        detail::LongPeriod { period: p }
    }
    pub fn short_period<'a>(p: &'a Period) -> impl Display + 'a {
        detail::ShortPeriod { period: p }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;
    use std::panic;
    use std::result::Result;

    #[test]
    fn new_sets_length_and_units() {
        let cases: [(i32, TimeUnit); 3] = [
            (3, TimeUnit::Months),
            (1, TimeUnit::Years),
            (7, TimeUnit::Days),
        ];

        for (len, unit) in cases {
            let p: Period = Period::new(len, unit);
            assert_eq!(
                p.length(),
                len,
                "Length mismatch for unit {:?}: got {}, expected {}",
                unit,
                p.length(),
                len
            );
            assert_eq!(
                p.units(),
                unit,
                "Unit mismatch for length {}: got {:?}, expected {:?}",
                len,
                p.units(),
                unit
            );
        }
    }

    #[test]
    fn from_frequency_constructs_expected_periods() {
        let cases: [(Frequency, (i32, TimeUnit)); 12] = [
            (Frequency::Annual, (1, TimeUnit::Years)),
            (Frequency::Semiannual, (6, TimeUnit::Months)),
            (Frequency::Quarterly, (3, TimeUnit::Months)),
            (Frequency::Bimonthly, (2, TimeUnit::Months)),
            (Frequency::Monthly, (1, TimeUnit::Months)),
            (Frequency::EveryFourthMonth, (4, TimeUnit::Months)),
            (Frequency::Weekly, (1, TimeUnit::Weeks)),
            (Frequency::Biweekly, (2, TimeUnit::Weeks)),
            (Frequency::EveryFourthWeek, (4, TimeUnit::Weeks)),
            (Frequency::Daily, (1, TimeUnit::Days)),
            (Frequency::NoFrequency, (0, TimeUnit::Days)),
            (Frequency::Once, (0, TimeUnit::Years)),
        ];

        for (freq, (len, unit)) in cases {
            let p: Period = Period::from_frequency(freq);
            assert_eq!(
                (p.length(), p.units()),
                (len, unit),
                "Mismatch for {:?}",
                freq
            );
        }
    }

    #[test]
    fn from_frequency_panics_on_otherfrequency() {
        let result = panic::catch_unwind(|| {
            let _ = Period::from_frequency(Frequency::OtherFrequency);
        });
        assert!(
            result.is_err(),
            "Expected panic on OtherFrequency but got Ok"
        );
    }

    #[test]
    fn frequency_returns_expected_values() {
        let cases: [(Period, Frequency); 10] = [
            (Period::new(1, TimeUnit::Years), Frequency::Annual),
            (Period::new(6, TimeUnit::Months), Frequency::Semiannual),
            (Period::new(3, TimeUnit::Months), Frequency::Quarterly),
            (Period::new(2, TimeUnit::Months), Frequency::Bimonthly),
            (Period::new(1, TimeUnit::Months), Frequency::Monthly),
            (
                Period::new(4, TimeUnit::Months),
                Frequency::EveryFourthMonth,
            ),
            (Period::new(1, TimeUnit::Weeks), Frequency::Weekly),
            (Period::new(2, TimeUnit::Weeks), Frequency::Biweekly),
            (Period::new(4, TimeUnit::Weeks), Frequency::EveryFourthWeek),
            (Period::new(1, TimeUnit::Days), Frequency::Daily),
        ];

        for (p, expected_freq) in cases {
            assert_eq!(p.frequency(), expected_freq, "Mismatch for {:?}", p);
        }
    }

    #[test]
    fn frequency_negative_returns_expected_values() {
        let cases: [(Period, Frequency); 10] = [
            (Period::new(-1, TimeUnit::Years), Frequency::Annual),
            (Period::new(-6, TimeUnit::Months), Frequency::Semiannual),
            (Period::new(-3, TimeUnit::Months), Frequency::Quarterly),
            (Period::new(-2, TimeUnit::Months), Frequency::Bimonthly),
            (Period::new(-1, TimeUnit::Months), Frequency::Monthly),
            (
                Period::new(-4, TimeUnit::Months),
                Frequency::EveryFourthMonth,
            ),
            (Period::new(-1, TimeUnit::Weeks), Frequency::Weekly),
            (Period::new(-2, TimeUnit::Weeks), Frequency::Biweekly),
            (Period::new(-4, TimeUnit::Weeks), Frequency::EveryFourthWeek),
            (Period::new(-1, TimeUnit::Days), Frequency::Daily),
        ];

        for (p, expected_freq) in cases {
            assert_eq!(p.frequency(), expected_freq, "Mismatch for {:?}", p);
        }
    }

    #[test]
    fn frequency_zero_length_special_cases() {
        let cases: [(Period, Frequency); 4] = [
            (Period::new(0, TimeUnit::Years), Frequency::Once),
            (Period::new(0, TimeUnit::Days), Frequency::NoFrequency),
            (Period::new(0, TimeUnit::Months), Frequency::NoFrequency),
            (Period::new(0, TimeUnit::Weeks), Frequency::NoFrequency),
        ];

        for (p, expected_freq) in cases {
            assert_eq!(
                p.frequency(),
                expected_freq,
                "Mismatch for zero-length {:?}",
                p
            );
        }
    }

    #[test]
    fn frequency_returns_otherfrequency_for_invalid_cases() {
        let cases: [Period; 5] = [
            Period::new(5, TimeUnit::Months), // not a divisor of 12
            Period::new(7, TimeUnit::Weeks),  // no 7-week freq
            Period::new(10, TimeUnit::Days),  // no 10-day freq
            Period::new(2, TimeUnit::Years),  // no 2-year freq
            Period::new(13, TimeUnit::Months),
        ];

        for p in cases {
            assert_eq!(
                p.frequency(),
                Frequency::OtherFrequency,
                "Expected OtherFrequency for {:?}",
                p
            );
        }
    }

    #[test]
    fn normalized_zero_length_becomes_days() {
        let cases: [(Period, Period); 2] = [
            (
                Period {
                    length: 0,
                    units: TimeUnit::Months,
                },
                Period {
                    length: 0,
                    units: TimeUnit::Days,
                },
            ),
            (
                Period {
                    length: 0,
                    units: TimeUnit::Years,
                },
                Period {
                    length: 0,
                    units: TimeUnit::Days,
                },
            ),
        ];

        for (input, expected) in cases {
            let result: Period = input.normalized();
            assert_eq!(
                (result.length, result.units),
                (expected.length, expected.units),
                "normalized({:?}) failed: got ({}, {:?}), expected ({}, {:?})",
                input,
                result.length,
                result.units,
                expected.length,
                expected.units
            );
        }
    }

    #[test]
    fn normalized_months_multiple_of_12_becomes_years() {
        let cases: [(Period, Period); 3] = [
            (
                Period {
                    length: 12,
                    units: TimeUnit::Months,
                },
                Period {
                    length: 1,
                    units: TimeUnit::Years,
                },
            ),
            (
                Period {
                    length: 24,
                    units: TimeUnit::Months,
                },
                Period {
                    length: 2,
                    units: TimeUnit::Years,
                },
            ),
            (
                Period {
                    length: 36,
                    units: TimeUnit::Months,
                },
                Period {
                    length: 3,
                    units: TimeUnit::Years,
                },
            ),
        ];

        for (input, expected) in cases {
            let result: Period = input.normalized();
            assert_eq!(
                (result.length, result.units),
                (expected.length, expected.units),
                "normalized({:?}) failed: got ({}, {:?}), expected ({}, {:?})",
                input,
                result.length,
                result.units,
                expected.length,
                expected.units
            );
        }
    }

    #[test]
    fn normalized_months_not_multiple_of_12_not_changed() {
        let cases: [(Period, Period); 2] = [
            (
                Period {
                    length: 14,
                    units: TimeUnit::Months,
                },
                Period {
                    length: 14,
                    units: TimeUnit::Months,
                },
            ),
            (
                Period {
                    length: -25,
                    units: TimeUnit::Months,
                },
                Period {
                    length: -25,
                    units: TimeUnit::Months,
                },
            ),
        ];

        for (input, expected) in cases {
            let result: Period = input.normalized();
            assert_eq!(
                (result.length, result.units),
                (expected.length, expected.units),
                "normalized({:?}) failed: got ({}, {:?}), expected ({}, {:?})",
                input,
                result.length,
                result.units,
                expected.length,
                expected.units
            );
        }
    }

    #[test]
    fn normalized_days_multiple_of_7_becomes_weeks() {
        let cases: [(Period, Period); 3] = [
            (
                Period {
                    length: 7,
                    units: TimeUnit::Days,
                },
                Period {
                    length: 1,
                    units: TimeUnit::Weeks,
                },
            ),
            (
                Period {
                    length: 14,
                    units: TimeUnit::Days,
                },
                Period {
                    length: 2,
                    units: TimeUnit::Weeks,
                },
            ),
            (
                Period {
                    length: 21,
                    units: TimeUnit::Days,
                },
                Period {
                    length: 3,
                    units: TimeUnit::Weeks,
                },
            ),
        ];

        for (input, expected) in cases {
            let result: Period = input.normalized();
            assert_eq!(
                (result.length, result.units),
                (expected.length, expected.units),
                "normalized({:?}) failed: got ({}, {:?}), expected ({}, {:?})",
                input,
                result.length,
                result.units,
                expected.length,
                expected.units
            );
        }
    }

    #[test]
    fn normalized_days_not_multiple_of_7_not_changed() {
        let cases: [(Period, Period); 2] = [
            (
                Period {
                    length: 8,
                    units: TimeUnit::Days,
                },
                Period {
                    length: 8,
                    units: TimeUnit::Days,
                },
            ),
            (
                Period {
                    length: -17,
                    units: TimeUnit::Days,
                },
                Period {
                    length: -17,
                    units: TimeUnit::Days,
                },
            ),
        ];

        for (input, expected) in cases {
            let result: Period = input.normalized();
            assert_eq!(
                (result.length, result.units),
                (expected.length, expected.units),
                "normalized({:?}) failed: got ({}, {:?}), expected ({}, {:?})",
                input,
                result.length,
                result.units,
                expected.length,
                expected.units
            );
        }
    }

    #[test]
    fn normalized_weeks_and_years_remain_unchanged() {
        let cases: [Period; 2] = [
            Period {
                length: 5,
                units: TimeUnit::Weeks,
            },
            Period {
                length: 3,
                units: TimeUnit::Years,
            },
        ];

        for input in cases {
            let result: Period = input.normalized();
            assert_eq!(
                (result.length, result.units),
                (input.length, input.units),
                "normalized({:?}) failed: got ({}, {:?}), expected ({}, {:?})",
                input,
                result.length,
                result.units,
                input.length,
                input.units
            );
        }
    }

    #[test]
    fn years_works() {
        let cases: [(i32, TimeUnit, f64); 3] = [
            (0, TimeUnit::Years, 0.0),
            (1, TimeUnit::Years, 1.0),
            (24, TimeUnit::Months, 2.0),
        ];

        for (len, unit, expected) in cases {
            let p: Period = Period::new(len, unit);
            let result: f64 = p.years();
            assert!(
                (result - expected).abs() < 1e-12,
                "years(): failed for {:?} {:?}, got {}, expected {}",
                len,
                unit,
                result,
                expected
            );
        }
    }

    #[test]
    fn years_panics_timeunit_implemented() {
        let cases: [(i32, TimeUnit); 2] = [(10, TimeUnit::Days), (5, TimeUnit::Weeks)];

        for (len, unit) in cases {
            let p = Period::new(len, unit);
            let result = panic::catch_unwind(|| p.years());
            assert!(
                result.is_err(),
                "years(): expected panic for {:?} {:?}, but got Ok",
                len,
                unit
            );
        }
    }

    #[test]
    fn months_works() {
        let cases: [(i32, TimeUnit, f64); 3] = [
            (0, TimeUnit::Months, 0.0),
            (12, TimeUnit::Months, 12.0),
            (2, TimeUnit::Years, 24.0),
        ];

        for (len, unit, expected) in cases {
            let p = Period::new(len, unit);
            let result = p.months();
            assert!(
                (result - expected).abs() < 1e-12,
                "months(): failed for {:?} {:?}, got {}, expected {}",
                len,
                unit,
                result,
                expected
            );
        }
    }

    #[test]
    fn months_panic_timeunit_implemented() {
        let cases: [(i32, TimeUnit); 2] = [(7, TimeUnit::Days), (3, TimeUnit::Weeks)];

        for (len, unit) in cases {
            let p = Period::new(len, unit);
            let result = panic::catch_unwind(|| p.months());
            assert!(
                result.is_err(),
                "months(): expected panic for {:?} {:?}, but got Ok",
                len,
                unit
            );
        }
    }

    #[test]
    fn weeks_works() {
        let cases: [(i32, TimeUnit, f64); 3] = [
            (0, TimeUnit::Weeks, 0.0),
            (2, TimeUnit::Weeks, 2.0),
            (14, TimeUnit::Days, 2.0),
        ];

        for (len, unit, expected) in cases {
            let p = Period::new(len, unit);
            let result = p.weeks();
            assert!(
                (result - expected).abs() < 1e-12,
                "weeks(): failed for {:?} {:?}, got {}, expected {}",
                len,
                unit,
                result,
                expected
            );
        }
    }

    #[test]
    fn weeks_panics_timeunit_implemented() {
        let cases: [(i32, TimeUnit); 2] = [(1, TimeUnit::Months), (1, TimeUnit::Years)];

        for (len, unit) in cases {
            let p: Period = Period::new(len, unit);
            let result = panic::catch_unwind(|| p.weeks());
            assert!(
                result.is_err(),
                "weeks(): expected panic for {:?} {:?}, but got Ok",
                len,
                unit
            );
        }
    }

    #[test]
    fn days_works() {
        let cases: [(i32, TimeUnit, f64); 3] = [
            (0, TimeUnit::Days, 0.0),
            (7, TimeUnit::Days, 7.0),
            (3, TimeUnit::Weeks, 21.0),
        ];

        for (len, unit, expected) in cases {
            let p: Period = Period::new(len, unit);
            let result: f64 = p.days();
            assert!(
                (result - expected).abs() < 1e-12,
                "days(): failed for {:?} {:?}, got {}, expected {}",
                len,
                unit,
                result,
                expected
            );
        }
    }

    #[test]
    fn days_panics_timeunit_implemented() {
        let cases: [(i32, TimeUnit); 2] = [(1, TimeUnit::Months), (1, TimeUnit::Years)];

        for (len, unit) in cases {
            let p: Period = Period::new(len, unit);
            let result = panic::catch_unwind(|| p.days());
            assert!(
                result.is_err(),
                "days(): expected panic for {:?} {:?}, but got Ok",
                len,
                unit
            );
        }
    }

    #[test]
    fn add_assign_works() {
        let cases: [(Period, Period, (i32, TimeUnit)); 7] = [
            // length == 0
            (
                Period::new(0, TimeUnit::Years),
                Period::new(3, TimeUnit::Months),
                (3, TimeUnit::Months),
            ),
            // same units
            (
                Period::new(2, TimeUnit::Years),
                Period::new(3, TimeUnit::Years),
                (5, TimeUnit::Years),
            ),
            // Years += Months
            (
                Period::new(2, TimeUnit::Years),
                Period::new(6, TimeUnit::Months),
                (30, TimeUnit::Months),
            ), // 2Y=24M + 6M
            // Months += Years
            (
                Period::new(6, TimeUnit::Months),
                Period::new(2, TimeUnit::Years),
                (30, TimeUnit::Months),
            ), // 6M + 24M
            // Weeks += Days
            (
                Period::new(2, TimeUnit::Weeks),
                Period::new(3, TimeUnit::Days),
                (17, TimeUnit::Days),
            ), // 2W=14D + 3D
            // Days += Weeks
            (
                Period::new(3, TimeUnit::Days),
                Period::new(2, TimeUnit::Weeks),
                (17, TimeUnit::Days),
            ), // 3D + 14D
            // Days += Days
            (
                Period::new(5, TimeUnit::Days),
                Period::new(3, TimeUnit::Days),
                (8, TimeUnit::Days),
            ),
        ];

        for (mut lhs, rhs, (expected_len, expected_unit)) in cases {
            lhs += rhs;
            assert_eq!(
                (lhs.length(), lhs.units()),
                (expected_len, expected_unit),
                "add_assign OK failed: initial lhs={:?}, rhs={:?}, got=({:?},{:?}), expected=({:?},{:?})",
                lhs,
                rhs,
                lhs.length(),
                lhs.units(),
                expected_len,
                expected_unit
            );
        }
    }

    #[test]
    fn add_assign_panics() {
        let cases: [(Period, Period); 8] = [
            // Years += Days
            (
                Period::new(1, TimeUnit::Years),
                Period::new(5, TimeUnit::Days),
            ),
            // Years += Weeks
            (
                Period::new(1, TimeUnit::Years),
                Period::new(2, TimeUnit::Weeks),
            ),
            // Months += Days
            (
                Period::new(1, TimeUnit::Months),
                Period::new(5, TimeUnit::Days),
            ),
            // Months += Weeks
            (
                Period::new(1, TimeUnit::Months),
                Period::new(2, TimeUnit::Weeks),
            ),
            // Weeks += Years
            (
                Period::new(1, TimeUnit::Weeks),
                Period::new(1, TimeUnit::Years),
            ),
            // Weeks += Months
            (
                Period::new(1, TimeUnit::Weeks),
                Period::new(2, TimeUnit::Months),
            ),
            // Days += Years
            (
                Period::new(1, TimeUnit::Days),
                Period::new(1, TimeUnit::Years),
            ),
            // Days += Months
            (
                Period::new(1, TimeUnit::Days),
                Period::new(2, TimeUnit::Months),
            ),
        ];

        for (mut lhs, rhs) in cases {
            let result = panic::catch_unwind(move || {
                lhs += rhs;
            });
            assert!(
                result.is_err(),
                "add_assign PANIC expected but got Ok: lhs={:?}, rhs={:?}",
                lhs,
                rhs
            );
        }
    }

    #[test]
    fn add_works() {
        let cases: [(Period, Period, (i32, TimeUnit)); 6] = [
            // Years + Years
            (
                Period::new(2, TimeUnit::Years),
                Period::new(3, TimeUnit::Years),
                (5, TimeUnit::Years),
            ),
            // Years + Months (converted to Months)
            (
                Period::new(1, TimeUnit::Years),
                Period::new(6, TimeUnit::Months),
                (18, TimeUnit::Months),
            ), // 1Y=12M + 6M
            // Months + Years (converted to Months)
            (
                Period::new(6, TimeUnit::Months),
                Period::new(2, TimeUnit::Years),
                (30, TimeUnit::Months),
            ), // 6M + 24M
            // Weeks + Weeks
            (
                Period::new(2, TimeUnit::Weeks),
                Period::new(3, TimeUnit::Weeks),
                (5, TimeUnit::Weeks),
            ),
            // Weeks + Days (converted to Days)
            (
                Period::new(2, TimeUnit::Weeks),
                Period::new(3, TimeUnit::Days),
                (17, TimeUnit::Days),
            ), // 2W=14D + 3D
            // Days + Weeks (converted to Days)
            (
                Period::new(3, TimeUnit::Days),
                Period::new(2, TimeUnit::Weeks),
                (17, TimeUnit::Days),
            ), // 3D + 14D
        ];

        for (lhs, rhs, (expected_len, expected_unit)) in cases {
            let result = lhs + rhs;
            assert_eq!(
                (result.length(), result.units()),
                (expected_len, expected_unit),
                "add OK failed: lhs={:?}, rhs={:?}, got=({:?},{:?}), expected=({:?},{:?})",
                lhs,
                rhs,
                result.length(),
                result.units(),
                expected_len,
                expected_unit
            );
        }
    }

    #[test]
    fn sub_assign_works() {
        let cases: [(Period, Period, (i32, TimeUnit)); 3] = [
            // Years -= Years
            (
                Period::new(5, TimeUnit::Years),
                Period::new(2, TimeUnit::Years),
                (3, TimeUnit::Years),
            ),
            // Months -= Months
            (
                Period::new(12, TimeUnit::Months),
                Period::new(6, TimeUnit::Months),
                (6, TimeUnit::Months),
            ),
            // Weeks -= Weeks
            (
                Period::new(4, TimeUnit::Weeks),
                Period::new(1, TimeUnit::Weeks),
                (3, TimeUnit::Weeks),
            ),
        ];

        for (mut lhs, rhs, (expected_len, expected_unit)) in cases {
            lhs -= rhs;
            assert_eq!(
                (lhs.length(), lhs.units()),
                (expected_len, expected_unit),
                "sub_assign OK failed: lhs={:?}, rhs={:?}, got=({:?},{:?}), expected=({:?},{:?})",
                lhs,
                rhs,
                lhs.length(),
                lhs.units(),
                expected_len,
                expected_unit
            );
        }
    }

    #[test]
    fn sub_works() {
        let cases: [(Period, Period, (i32, TimeUnit)); 3] = [
            // Years - Years
            (
                Period::new(5, TimeUnit::Years),
                Period::new(2, TimeUnit::Years),
                (3, TimeUnit::Years),
            ),
            // Months - Years (converted to Months)
            (
                Period::new(24, TimeUnit::Months),
                Period::new(1, TimeUnit::Years),
                (12, TimeUnit::Months),
            ), // 24M - 12M
            // Weeks - Days (converted to Days)
            (
                Period::new(2, TimeUnit::Weeks),
                Period::new(7, TimeUnit::Days),
                (7, TimeUnit::Days),
            ), // 14D - 7D
        ];

        for (lhs, rhs, (expected_len, expected_unit)) in cases {
            let result = lhs - rhs;
            assert_eq!(
                (result.length(), result.units()),
                (expected_len, expected_unit),
                "sub OK failed: lhs={:?}, rhs={:?}, got=({:?},{:?}), expected=({:?},{:?})",
                lhs,
                rhs,
                result.length(),
                result.units(),
                expected_len,
                expected_unit
            );
        }
    }

    #[test]
    fn mul_assign_works() {
        let cases: [(Period, i32, (i32, TimeUnit)); 3] = [
            // Years *= positive
            (Period::new(2, TimeUnit::Years), 3, (6, TimeUnit::Years)),
            // Months *= negative
            (Period::new(4, TimeUnit::Months), -2, (-8, TimeUnit::Months)),
            // Days *= zero
            (Period::new(5, TimeUnit::Days), 0, (0, TimeUnit::Days)),
        ];

        for (mut lhs, factor, (expected_len, expected_unit)) in cases {
            lhs *= factor;
            assert_eq!(
                (lhs.length(), lhs.units()),
                (expected_len, expected_unit),
                "mul_assign OK failed: lhs={:?}, factor={}, got=({:?},{:?}), expected=({:?},{:?})",
                lhs,
                factor,
                lhs.length(),
                lhs.units(),
                expected_len,
                expected_unit
            );
        }
    }

    #[test]
    fn div_assign_works() {
        let cases: [(Period, i32, (i32, TimeUnit)); 5] = [
            // Months /= exact
            (Period::new(12, TimeUnit::Months), 3, (4, TimeUnit::Months)),
            // Years /= not exact -> convert to Months
            (Period::new(1, TimeUnit::Years), 2, (6, TimeUnit::Months)),
            // Weeks /= exact
            (Period::new(2, TimeUnit::Weeks), 2, (1, TimeUnit::Weeks)),
            // Days /= exact
            (Period::new(10, TimeUnit::Days), 2, (5, TimeUnit::Days)),
            // Months /= exact
            (Period::new(6, TimeUnit::Months), 3, (2, TimeUnit::Months)),
        ];

        for (mut lhs, divider, (expected_len, expected_unit)) in cases {
            lhs /= divider;
            assert_eq!(
                (lhs.length(), lhs.units()),
                (expected_len, expected_unit),
                "div_assign OK failed: lhs={:?}, divider={}, got=({:?},{:?}), expected=({:?},{:?})",
                lhs,
                divider,
                lhs.length(),
                lhs.units(),
                expected_len,
                expected_unit
            );
        }
    }

    #[test]
    fn div_assign_panics() {
        let cases: [(Period, i32); 5] = [
            // divide by zero
            (Period::new(1, TimeUnit::Years), 0),
            // Years not divisible even after conversion
            (Period::new(1, TimeUnit::Years), 5),
            // Weeks not divisible
            (Period::new(1, TimeUnit::Weeks), 4),
            // Months not divisible (no conversion attempted)
            (Period::new(5, TimeUnit::Months), 2),
            // Days not divisible (no conversion attempted)
            (Period::new(5, TimeUnit::Days), 3),
        ];

        for (mut lhs, divider) in cases {
            let result = panic::catch_unwind(move || {
                lhs /= divider;
            });
            assert!(
                result.is_err(),
                "div_assign PANIC expected but got Ok: lhs={:?}, divider={}",
                lhs,
                divider
            );
        }
    }

    #[test]
    fn div_operator_works() {
        let cases: [(Period, i32, (i32, TimeUnit)); 2] = [
            // Months / exact
            (Period::new(12, TimeUnit::Months), 6, (2, TimeUnit::Months)),
            // Years / not exact -> convert to Months
            (Period::new(1, TimeUnit::Years), 2, (6, TimeUnit::Months)),
        ];

        for (lhs, divider, (expected_len, expected_unit)) in cases {
            let result = lhs / divider;
            assert_eq!(
                (result.length(), result.units()),
                (expected_len, expected_unit),
                "div operator OK failed: lhs={:?}, divider={}, got=({:?},{:?}), expected=({:?},{:?})",
                lhs,
                divider,
                result.length(),
                result.units(),
                expected_len,
                expected_unit
            );
        }
    }

    #[test]
    fn partial_cmp_all_cases() {
        // Result<Ordering, ()>
        // Ok(Ordering) -> the comparison succeeded with a definite result.
        // Err(()) -> represents a panic case (we use () here as a placeholder type
        // when we don’t care about extra info).

        // Each tuple: (left, right, expected_result, description)
        let cases: [(Period, Period, Option<Result<Ordering, ()>>, &str); 29] = [
            // --- Zero length vs positive/negative/zero
            (
                Period::new(0, TimeUnit::Days),
                Period::new(5, TimeUnit::Days),
                Some(Ok(Ordering::Less)),
                "0D < 5D - Days",
            ),
            (
                Period::new(0, TimeUnit::Days),
                Period::new(-5, TimeUnit::Days),
                Some(Ok(Ordering::Greater)),
                "0D > -5D - Days",
            ),
            (
                Period::new(0, TimeUnit::Days),
                Period::new(0, TimeUnit::Days),
                Some(Ok(Ordering::Equal)),
                "0D == 0D - Days",
            ),
            // --- Positive/negative vs zero length
            (
                Period::new(5, TimeUnit::Days),
                Period::new(0, TimeUnit::Days),
                Some(Ok(Ordering::Greater)),
                "5D > 0D - Days",
            ),
            (
                Period::new(-5, TimeUnit::Days),
                Period::new(0, TimeUnit::Days),
                Some(Ok(Ordering::Less)),
                "-5D < 0D - Days",
            ),
            // --- Exact comparisons (same units)
            (
                Period::new(3, TimeUnit::Years),
                Period::new(5, TimeUnit::Years),
                Some(Ok(Ordering::Less)),
                "3Y < 5Y - Years",
            ),
            (
                Period::new(5, TimeUnit::Years),
                Period::new(3, TimeUnit::Years),
                Some(Ok(Ordering::Greater)),
                "5Y > 3Y - Years",
            ),
            (
                Period::new(3, TimeUnit::Years),
                Period::new(3, TimeUnit::Years),
                Some(Ok(Ordering::Equal)),
                "3Y == 3Y - Years",
            ),
            // --- Convertible Months <-> Years
            (
                Period::new(12, TimeUnit::Months),
                Period::new(1, TimeUnit::Years),
                Some(Ok(Ordering::Equal)),
                "12M == 1Y",
            ),
            (
                Period::new(24, TimeUnit::Months),
                Period::new(1, TimeUnit::Years),
                Some(Ok(Ordering::Greater)),
                "24M > 1Y",
            ),
            (
                Period::new(6, TimeUnit::Months),
                Period::new(1, TimeUnit::Years),
                Some(Ok(Ordering::Less)),
                "6M < 1Y",
            ),
            (
                Period::new(1, TimeUnit::Years),
                Period::new(24, TimeUnit::Months),
                Some(Ok(Ordering::Less)),
                "1Y < 24M",
            ),
            (
                Period::new(1, TimeUnit::Years),
                Period::new(12, TimeUnit::Months),
                Some(Ok(Ordering::Equal)),
                "1Y == 12M",
            ),
            // --- Convertible Days <-> Weeks
            (
                Period::new(7, TimeUnit::Days),
                Period::new(1, TimeUnit::Weeks),
                Some(Ok(Ordering::Equal)),
                "7D == 1W",
            ),
            (
                Period::new(14, TimeUnit::Days),
                Period::new(1, TimeUnit::Weeks),
                Some(Ok(Ordering::Greater)),
                "14D > 1W",
            ),
            (
                Period::new(3, TimeUnit::Days),
                Period::new(1, TimeUnit::Weeks),
                Some(Ok(Ordering::Less)),
                "3D < 1W",
            ),
            (
                Period::new(15, TimeUnit::Days),
                Period::new(2, TimeUnit::Weeks),
                Some(Ok(Ordering::Greater)),
                "15D > 2W",
            ),
            (
                Period::new(1, TimeUnit::Days),
                Period::new(1, TimeUnit::Months),
                Some(Ok(Ordering::Less)),
                "1D < 1M (covers Days && not Weeks RHS-false branch)",
            ),
            (
                Period::new(2, TimeUnit::Weeks),
                Period::new(14, TimeUnit::Days),
                Some(Ok(Ordering::Equal)),
                "2W == 14D",
            ),
            (
                Period::new(3, TimeUnit::Weeks),
                Period::new(20, TimeUnit::Days),
                Some(Ok(Ordering::Greater)),
                "3W > 20D",
            ),
            (
                Period::new(1, TimeUnit::Weeks),
                Period::new(7, TimeUnit::Days),
                Some(Ok(Ordering::Equal)),
                "1W == 7D",
            ),
            (
                Period::new(1, TimeUnit::Weeks),
                Period::new(1, TimeUnit::Months),
                Some(Ok(Ordering::Less)),
                "1W < 1M (covers Weels && not Days RHS-false branch)",
            ),
            // --- Fallback days_min_max (cover Weeks + Years inside days_min_max)
            (
                Period::new(2, TimeUnit::Weeks),
                Period::new(20, TimeUnit::Days),
                Some(Ok(Ordering::Less)),
                "2W < 20D (days_min_max: Weeks)",
            ),
            (
                Period::new(1, TimeUnit::Years),
                Period::new(400, TimeUnit::Days),
                Some(Ok(Ordering::Less)),
                "1Y < 400D (days_min_max: Years)",
            ),
            // --- Fallback clear inequality
            (
                Period::new(1, TimeUnit::Months),
                Period::new(60, TimeUnit::Days),
                Some(Ok(Ordering::Less)),
                "1M < 60D (min=28, max=31)",
            ),
            (
                Period::new(2, TimeUnit::Years),
                Period::new(10, TimeUnit::Months),
                Some(Ok(Ordering::Greater)),
                "2Y > 10M",
            ),
            (
                Period::new(2, TimeUnit::Years),
                Period::new(400, TimeUnit::Days),
                Some(Ok(Ordering::Greater)),
                "2Y > 400D (fallback self_min > other_max)",
            ),
            (
                Period::new(24, TimeUnit::Months),
                Period::new(1, TimeUnit::Years),
                Some(Ok(Ordering::Greater)),
                "24M > 1Y (min>max branch)",
            ),
            // --- Panic case: undecidable overlap
            (
                Period::new(1, TimeUnit::Months),
                Period::new(30, TimeUnit::Days),
                Some(Err(())),
                "1M vs 30D undecidable",
            ),
        ];

        for (left, right, expected, description) in cases {
            let result = panic::catch_unwind(|| left.partial_cmp(&right));

            let mapped: Option<Result<Ordering, ()>> = match result {
                Ok(v) => v.map(|ord: Ordering| Ok(ord)), // wrap Ordering
                Err(_) => Some(Err(())),                 // panic -> Err(())
            };

            assert_eq!(
                mapped, expected,
                "Failed case: {} (left={:?}, right={:?})",
                description, left, right
            );
        }
    }

    #[test]
    fn days_min_max_all_units() {
        // Each tuple: (Period, expected_min, expected_max, description)
        let cases: [(Period, i32, i32, &str); 6] = [
            (Period::new(5, TimeUnit::Days), 5, 5, "5D"),
            (Period::new(2, TimeUnit::Weeks), 14, 14, "2W"),
            (Period::new(1, TimeUnit::Months), 28, 31, "1M"),
            (Period::new(2, TimeUnit::Months), 56, 62, "2M"),
            (Period::new(1, TimeUnit::Years), 365, 366, "1Y"),
            (Period::new(2, TimeUnit::Years), 730, 732, "2Y"),
        ];

        for (period, expected_min, expected_max, description) in cases {
            let (min_days, max_days) = Period::days_min_max(&period);
            assert_eq!(
                (min_days, max_days),
                (expected_min, expected_max),
                "Failed case: {} (period={:?})",
                description,
                period
            );
        }
    }

    #[test]
    fn long_period_displays_correctly() {
        let cases: [((i32, TimeUnit), &str); 8] = [
            ((1, TimeUnit::Days), "1 Day"),
            ((2, TimeUnit::Days), "2 Days"),
            ((1, TimeUnit::Weeks), "1 Week"),
            ((3, TimeUnit::Weeks), "3 Weeks"),
            ((1, TimeUnit::Months), "1 Month"),
            ((3, TimeUnit::Months), "3 Months"),
            ((1, TimeUnit::Years), "1 Year"),
            ((10, TimeUnit::Years), "10 Years"),
        ];

        for ((len, unit), expected) in cases {
            let p = Period::new(len, unit);
            let result = format!("{}", io::long_period(&p));
            assert_eq!(result, expected, "Failed case: {} {:?}", expected, p);
        }
    }

    #[test]
    fn short_period_displays_correctly() {
        let cases: [((i32, TimeUnit), &str); 4] = [
            ((1, TimeUnit::Days), "1D"),
            ((2, TimeUnit::Weeks), "2W"),
            ((3, TimeUnit::Months), "3M"),
            ((4, TimeUnit::Years), "4Y"),
        ];

        for ((len, unit), expected) in cases {
            let p: Period = Period::new(len, unit);
            let result: String = format!("{}", io::short_period(&p));
            assert_eq!(result, expected, "Failed case: {} {:?}", expected, p);
        }
    }

    #[test]
    fn period_display_outputs_correctly() {
        let cases: [((i32, TimeUnit), &str); 4] = [
            ((1, TimeUnit::Days), "1D"),
            ((2, TimeUnit::Weeks), "2W"),
            ((3, TimeUnit::Months), "3M"),
            ((4, TimeUnit::Years), "4Y"),
        ];

        for ((len, unit), expected) in cases {
            let p: Period = Period::new(len, unit);
            let result: String = format!("{}", p);
            assert_eq!(result, expected, "Failed case: {} {:?}", expected, p);
        }
    }
}
