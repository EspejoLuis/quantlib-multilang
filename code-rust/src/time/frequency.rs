use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Frequency {
    NoFrequency = -1,     // null frequency
    Once = 0,             // only once, e.g., a zero-coupon
    Annual = 1,           // once a year
    Semiannual = 2,       // twice a year
    EveryFourthMonth = 3, // every fourth month
    Quarterly = 4,        // every third month
    Bimonthly = 6,        // every second month
    Monthly = 12,         // once a month
    EveryFourthWeek = 13, // every fourth week
    Biweekly = 26,        // every second week
    Weekly = 52,          // once a week
    Daily = 365,          // once a day
    OtherFrequency = 999, // some other unknown frequency
}
impl Frequency {
    pub fn from_nth_times_per_year(nth_times: u32) -> Frequency {
        match nth_times {
            12 => Frequency::Monthly,
            6 => Frequency::Bimonthly,
            4 => Frequency::Quarterly,
            3 => Frequency::EveryFourthMonth,
            2 => Frequency::Semiannual,
            1 => Frequency::Annual,
            _ => Frequency::OtherFrequency,
        }
    }
}

// Traits
impl Display for Frequency {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let frequency: &'static str = match self {
            Frequency::NoFrequency => "No-Frequency",
            Frequency::Once => "Once",
            Frequency::Annual => "Annual",
            Frequency::Semiannual => "Semiannual",
            Frequency::EveryFourthMonth => "Every-Fourth-Month",
            Frequency::Quarterly => "Quarterly",
            Frequency::Bimonthly => "Bimonthly",
            Frequency::Monthly => "Monthly",
            Frequency::EveryFourthWeek => "Every-fourth-week",
            Frequency::Biweekly => "Biweekly",
            Frequency::Weekly => "Weekly",
            Frequency::Daily => "Daily",
            Frequency::OtherFrequency => "Unknown frequency",
        };
        write!(f, "{}", frequency)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_frequency_outputs_correct_format() {
        let cases: [(Frequency, &str); 13] = [
            (Frequency::NoFrequency, "No-Frequency"),
            (Frequency::Once, "Once"),
            (Frequency::Annual, "Annual"),
            (Frequency::Semiannual, "Semiannual"),
            (Frequency::EveryFourthMonth, "Every-Fourth-Month"),
            (Frequency::Quarterly, "Quarterly"),
            (Frequency::Bimonthly, "Bimonthly"),
            (Frequency::Monthly, "Monthly"),
            (Frequency::EveryFourthWeek, "Every-fourth-week"),
            (Frequency::Biweekly, "Biweekly"),
            (Frequency::Weekly, "Weekly"),
            (Frequency::Daily, "Daily"),
            (Frequency::OtherFrequency, "Unknown frequency"),
        ];

        for (frequency, expected) in cases {
            assert_eq!(
                format!("{}", frequency),
                expected,
                "Failed for frequency {:?}",
                frequency
            );
        }
    }

    #[test]
    fn from_nth_times_per_year_valid_inputs() {
        let cases: [(u32, Frequency); 6] = [
            (12, Frequency::Monthly),
            (6, Frequency::Bimonthly),
            (4, Frequency::Quarterly),
            (3, Frequency::EveryFourthMonth),
            (2, Frequency::Semiannual),
            (1, Frequency::Annual),
        ];

        for (input, expected) in cases {
            assert_eq!(
                Frequency::from_nth_times_per_year(input),
                expected,
                "Failed for input {}",
                input
            );
        }
    }

    #[test]
    fn from_nth_times_per_year_invalid_inputs() {
        let invalid_cases: [u32; 7] = [0, 5, 7, 8, 10, 13, 100];
        for input in invalid_cases {
            assert_eq!(
                Frequency::from_nth_times_per_year(input),
                Frequency::OtherFrequency,
                "Expected OtherFrequency for input {}",
                input
            );
        }
    }
}
