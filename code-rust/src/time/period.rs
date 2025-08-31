use crate::time::frequency::Frequency;
use crate::time::time_unit::TimeUnit;
use std::ops::Neg;
use std::ops::{AddAssign, SubAssign};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

    // Inspectors public
    pub fn length(&self) -> i32 {
        self.length
    }
    pub fn units(&self) -> TimeUnit {
        self.units
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
        //If want a normalized copy without touching the original
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
}

impl Neg for Period {
    // We need -a not a-b so that's why use Neg instead of Sub
    type Output = Period;
    fn neg(self) -> Period {
        // New period as output
        Period::new(-self.length, self.units)
    }
}

impl AddAssign<Period> for Period {
    // AddAssign -> right hand side
    // for Period -> left hand side
    // No Output, no new Date returned. SAME Period modified!
    fn add_assign(&mut self, period_to_add: Period) -> () {
        if self.length == 0 {
            self.length = period_to_add.length();
            self.units = period_to_add.units();
            // if same units
        } else {
            match self.units {
                TimeUnit::Years => match period_to_add.units() {
                    TimeUnit::Years => self.length += period_to_add.length(),
                    TimeUnit::Months => {
                        // years*12 + month
                        self.units = TimeUnit::Months;
                        self.length = self.length * 12 + period_to_add.length();
                    }
                    TimeUnit::Weeks | TimeUnit::Days => {
                        panic!(
                            "Impossible addition between {:?} and {:?}",
                            self, period_to_add
                        )
                    }
                },
                TimeUnit::Months => match period_to_add.units() {
                    TimeUnit::Months => self.length += period_to_add.length(),
                    TimeUnit::Years => {
                        // months + years*12
                        self.length += 12 * period_to_add.length();
                    }
                    TimeUnit::Weeks | TimeUnit::Days => {
                        panic!(
                            "Impossible addition between {:?} and {:?}",
                            self, period_to_add
                        )
                    }
                },
                TimeUnit::Weeks => match period_to_add.units() {
                    TimeUnit::Weeks => self.length += period_to_add.length(),
                    TimeUnit::Days => {
                        // weeks*7 + days
                        self.units = TimeUnit::Days;
                        self.length = self.length * 7 + period_to_add.length();
                    }
                    TimeUnit::Years | TimeUnit::Months => {
                        panic!(
                            "Impossible addition between {:?} and {:?}",
                            self, period_to_add
                        )
                    }
                },
                TimeUnit::Days => match period_to_add.units() {
                    TimeUnit::Days => self.length += period_to_add.length(),
                    TimeUnit::Weeks => {
                        // days + weeks*7
                        self.length += 7 * period_to_add.length();
                    }
                    TimeUnit::Years | TimeUnit::Months => {
                        panic!(
                            "Impossible addition between {:?} and {:?}",
                            self, period_to_add
                        )
                    }
                },
            }
        }
    }
}

impl SubAssign<Period> for Period {
    // No Output, no new Date returned. SAME Period modified!
    fn sub_assign(&mut self, period_to_add: Period) -> () {
        *self += -period_to_add
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

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
                    length: 25,
                    units: TimeUnit::Months,
                },
                Period {
                    length: 25,
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
                    length: 17,
                    units: TimeUnit::Days,
                },
                Period {
                    length: 17,
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

    // ---- years ----
    #[test]
    fn years_works() {
        let cases = vec![
            (0, TimeUnit::Years, 0.0),
            (1, TimeUnit::Years, 1.0),
            (24, TimeUnit::Months, 2.0),
        ];

        for (len, unit, expected) in cases {
            let p = Period::new(len, unit);
            let result = p.years();
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
        let cases = vec![(10, TimeUnit::Days), (5, TimeUnit::Weeks)];

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

    // ---- months ----
    #[test]
    fn months_works() {
        let cases = vec![
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
        let cases = vec![(7, TimeUnit::Days), (3, TimeUnit::Weeks)];

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

    // ---- weeks ----
    #[test]
    fn weeks_works() {
        let cases = vec![
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

    // ---- days ----
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
        let cases: [(Period, Period, (i32, TimeUnit)); 6] = [
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
}
