/*
This imports the type Date into your current scope.
So now, instead of
    let d = code_rust::date::Date::new(…);
we can use
    let d = Date::new(…);
*/
use code_rust::time::date::Date;
use code_rust::time::date::Month;
use code_rust::time::period::Period;
use code_rust::time::time_unit::TimeUnit;

#[test]
fn add_then_subtract_returns_original_date_correctly() {
    /*
    Had to implement by reference add function (&)
    by value was not enough.

        let a = Date::new(1, 1, 2020);
        let c = &a + 10;  // borrow `a`

        let b = Date::new(1, 1, 2020);
        let a = b;  // ownership of `b` moved into `a`
    */
    let cases: [(Date, i32, &str); 6] = [
        // --- Simple mid-range ---
        (Date::new(12, Month::November, 1989), 40, "mid example"),
        // --- Month boundary ---
        (Date::new(25, Month::January, 2024), 10, "cross month"),
        // --- Year boundary ---
        (Date::new(31, Month::December, 2023), 5, "cross new year"),
        // --- Leap year February ---
        (Date::new(29, Month::February, 2020), 10, "leap Feb 29"),
        // --- Non-leap February ---
        (Date::new(28, Month::February, 2021), 10, "non-leap Feb 28"),
        // --- Negative offset ---
        (
            Date::new(10, Month::March, 2022),
            -15,
            "subtract first, then add",
        ),
    ];

    for (start, delta, label) in cases {
        let forward: Date = start + delta;
        let round_trip: Date = forward - delta;

        assert_eq!(
            round_trip, start,
            "Round-trip failed: {} | start={:?}, delta={}",
            label, start, delta
        );
    }
}

#[test]
fn date_and_period_month_ends() {
    let cases: [(Date, Period, Date, &str); 9] = [
        // --- Month rollovers ---
        (
            Date::new(31, Month::January, 2024),
            Period::new(1, TimeUnit::Months),
            Date::new(29, Month::February, 2024),
            "Jan 31 → Feb end (leap year)",
        ),
        (
            Date::new(31, Month::January, 2023),
            Period::new(2, TimeUnit::Months),
            Date::new(31, Month::March, 2023),
            "Jan 31 → Mar 31 (non-leap)",
        ),
        (
            Date::new(30, Month::November, 2023),
            Period::new(1, TimeUnit::Months),
            Date::new(30, Month::December, 2023),
            "Nov 30 → Dec 30",
        ),
        // --- Week rollovers ---
        (
            Date::new(25, Month::January, 2024),
            Period::new(1, TimeUnit::Weeks),
            Date::new(1, Month::February, 2024),
            "Jan 25 + 1 Week → Feb 1",
        ),
        (
            Date::new(27, Month::December, 2023),
            Period::new(1, TimeUnit::Weeks),
            Date::new(3, Month::January, 2024),
            "Dec 27 + 1 Week → Jan 3 (next year)",
        ),
        (
            Date::new(29, Month::December, 2024),
            Period::new(2, TimeUnit::Weeks),
            Date::new(12, Month::January, 2025),
            "Dec 29 + 2 Weeks → Jan 12 (crossing year boundary, leap start)",
        ),
        // --- Day rollovers ---
        (
            Date::new(31, Month::December, 2023),
            Period::new(1, TimeUnit::Days),
            Date::new(1, Month::January, 2024),
            "Dec 31 + 1 Day → Jan 1 (next year)",
        ),
        (
            Date::new(1, Month::January, 2024),
            Period::new(-1, TimeUnit::Days),
            Date::new(31, Month::December, 2023),
            "Jan 1 - 1 Day → Dec 31 (previous year)",
        ),
        // --- Year rollovers ---
        (
            Date::new(29, Month::February, 2020),
            Period::new(1, TimeUnit::Years),
            Date::new(28, Month::February, 2021),
            "Leap Feb 29 + 1 Year → Feb 28 (non-leap)",
        ),
    ];

    for (start, period, expected, label) in cases {
        let result: Date = start + period;
        assert_eq!(
            result, expected,
            "Failed case: {} | start={:?} + period={:?}",
            label, start, period
        );
    }
}

#[test]
fn period_normalization_and_date_advance_consistency() {
    let cases: [(Date, Period, Period, Date, &str); 8] = [
        // --- Days ↔ Weeks ---
        (
            Date::new(1, Month::March, 2024),
            Period::new(7, TimeUnit::Days),
            Period::new(1, TimeUnit::Weeks),
            Date::new(8, Month::March, 2024),
            "7 Days == 1 Week",
        ),
        (
            Date::new(10, Month::March, 2024),
            Period::new(14, TimeUnit::Days),
            Period::new(2, TimeUnit::Weeks),
            Date::new(24, Month::March, 2024),
            "14 Days == 2 Weeks",
        ),
        (
            Date::new(1, Month::February, 2024),
            Period::new(28, TimeUnit::Days),
            Period::new(4, TimeUnit::Weeks),
            Date::new(29, Month::February, 2024), // leap year
            "28 Days == 4 Weeks (leap year Feb)",
        ),
        // --- Months ↔ Years ---
        (
            Date::new(15, Month::January, 2020),
            Period::new(12, TimeUnit::Months),
            Period::new(1, TimeUnit::Years),
            Date::new(15, Month::January, 2021),
            "12 Months == 1 Year",
        ),
        (
            Date::new(7, Month::February, 2024),
            Period::new(24, TimeUnit::Months),
            Period::new(2, TimeUnit::Years),
            Date::new(7, Month::February, 2026),
            "24 Months == 2 Years",
        ),
        (
            Date::new(1, Month::March, 2020),
            Period::new(36, TimeUnit::Months),
            Period::new(3, TimeUnit::Years),
            Date::new(1, Month::March, 2023),
            "36 Months == 3 Years",
        ),
        (
            Date::new(10, Month::June, 2020),
            Period::new(60, TimeUnit::Months),
            Period::new(5, TimeUnit::Years),
            Date::new(10, Month::June, 2025),
            "60 Months == 5 Years",
        ),
        (
            Date::new(15, Month::January, 2020),
            Period::new(-12, TimeUnit::Months),
            Period::new(-1, TimeUnit::Years),
            Date::new(15, Month::January, 2019),
            "-12 Months == -1 Year",
        ),
    ];

    for (start, period_days_weeks, period_equivalent, expected, label) in cases {
        let result_a: Date = start + period_days_weeks.normalized();
        let result_b: Date = start + period_equivalent.normalized();

        assert_eq!(
            (result_a, result_b),
            (expected, expected),
            "Failed case: {} | start={:?}, a={:?}, b={:?}",
            label,
            start,
            period_days_weeks,
            period_equivalent
        );
    }
}
