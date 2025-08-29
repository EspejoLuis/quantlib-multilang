use crate::time::frequency::Frequency;
use crate::time::time_unit::TimeUnit;

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
            _ => panic!("unknown time unit {:?}", units),
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
                _ => panic!("unknown time unit {:?}", units),
            }
        }
    }
    pub fn normalized(&self) -> Period {
        //If want a normalized copy without touching the original
        let mut period: Period = *self; // Create copy of the object
        period.normalize();
        period
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
            assert_eq!(p.length(), len, "Length mismatch for {:?}", unit);
            assert_eq!(p.units(), unit, "Unit mismatch for {:?}", unit);
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
    fn frequency_panics_on_not_implemented_timeunits() {
        let cases: [Period; 5] = [
            Period::new(1, TimeUnit::Milliseconds),
            Period::new(1, TimeUnit::Hours),
            Period::new(1, TimeUnit::Microseconds),
            Period::new(1, TimeUnit::Minutes),
            Period::new(1, TimeUnit::Seconds),
        ];

        for p in cases {
            let result = panic::catch_unwind(|| {
                let _ = p.frequency();
            });
            assert!(
                result.is_err(),
                "Expected panic for not implemented time units but got Ok"
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
            assert_eq!(result.length, expected.length);
            assert_eq!(result.units, expected.units);
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
                (expected.length, expected.units)
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
                (expected.length, expected.units)
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
                (expected.length, expected.units)
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
                (expected.length, expected.units)
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
            assert_eq!((result.length, result.units), (input.length, input.units));
        }
    }

    #[test]
    fn normalize_panics_on_not_implemented_timeunits() {
        let cases: [Period; 5] = [
            Period::new(1, TimeUnit::Milliseconds),
            Period::new(1, TimeUnit::Hours),
            Period::new(1, TimeUnit::Microseconds),
            Period::new(1, TimeUnit::Minutes),
            Period::new(1, TimeUnit::Seconds),
        ];

        for p in cases {
            let result = panic::catch_unwind(|| {
                let _ = p.normalized();
            });
            assert!(
                result.is_err(),
                "Expected panic for not implemented time units but got Ok"
            );
        }
    }
}
