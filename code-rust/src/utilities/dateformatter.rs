// Private
mod detail {
    use std::fmt;

    pub(crate) struct Ordinal {
        pub(crate) number: usize,
    }

    impl fmt::Display for Ordinal {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let number: &usize = &self.number;
            let suffix: &'static str = if (11..=13).contains(&(number % 100)) {
                "th"
            } else {
                match number % 10 {
                    1 => "st",
                    2 => "nd",
                    3 => "rd",
                    _ => "th",
                }
            };
            write!(f, "{}{}", number, suffix)
        }
    }
}

// Public API
pub(crate) mod io {
    use super::detail;

    pub fn ordinal(number: usize) -> impl std::fmt::Display {
        detail::Ordinal { number: number }
    }
}
