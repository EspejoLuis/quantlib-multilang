// Tell Catch2 to generate the main() function for you
#define CATCH_CONFIG_MAIN

#include "../lib/catch.hpp"
#include "Date.hpp"
#include <stdexcept>
#include <vector>

using namespace QuantLibCpp;

TEST_CASE("Default Date constructor sets correct values", "[Date]") {
    Date d;
    
    REQUIRE(d.day()==1);
    REQUIRE(d.month()==Month::January);
    REQUIRE(d.year()==1901);
}

TEST_CASE("Date constructor sets correct values", "[Date]") {
    Date d(15, Month::July, 1976);

    // An assertion: fails the test if the condition is false
    REQUIRE(d.day()==15);
    REQUIRE(d.month()==Month::July);
    REQUIRE(d.year()==1976);
}

TEST_CASE("Date formatting is correct", "[Date]") {
    Date d(23, Month::May, 1902);
    REQUIRE(d.toString() == "23-May-1902");
}

TEST_CASE("Equality operator works correctly", "[Date]") {
    Date d1(14, Month::May, 1989);
    Date d2(14, Month::May, 1989);
    Date d3(13, Month::May, 1989);
    Date d4(14, Month::June, 1989);
    Date d5(14, Month::May, 2010);

    REQUIRE(d1 == d2);
    REQUIRE_FALSE(d1 == d3);
    REQUIRE_FALSE(d1 == d4);
    REQUIRE_FALSE(d1 == d5);
}

TEST_CASE("Lower operator works correctly", "[Date]") {
    Date d1(14, Month::May, 1989);
    Date d2(15, Month::May, 1989);
    Date d3(14, Month::June, 1989);
    Date d4(14, Month::May, 2012);

    REQUIRE(d1 < d2);
    REQUIRE(d1 < d3);
    REQUIRE(d1 < d4);
    REQUIRE_FALSE(d3 < d1);
}

TEST_CASE("Adding operator works correctly - No Overflow", "[Date]") {
    Date d1(14, Month::May, 1989);
    Date result = d1 + 17;

    REQUIRE(result.day() == 14 + 17);
    REQUIRE(result.month() == Month::May);
    REQUIRE(result.year() == 1989);
}

TEST_CASE("Subtracting operator works correctly", "[Date]") {
    Date d1(14, Month::May, 1989);
    Date result = d1 - 11;

    REQUIRE(result.day() == 14 - 11);
    REQUIRE(result.month() == Month::May);
    REQUIRE(result.year() == 1989);
}

TEST_CASE("Leap year rule works correctly", "[Date]"){
    REQUIRE(Date::isLeap(1996));
    REQUIRE(Date::isLeap(2000));
    REQUIRE_FALSE(Date::isLeap(1900));
    REQUIRE_FALSE(Date::isLeap(1999));
    REQUIRE_FALSE(Date::isLeap(2100));
}

TEST_CASE("Number of days in a month works correctly", "[Date]"){
    
    SECTION("31-day months") {
    std::vector<Month> monthsWith31Days = {
        Month::January, Month::March, Month::May, Month::July,
        Month::August, Month::October, Month::December
    };
    
    /*
    Month m	--> Each loop iteration copies the element into m.
    Month& m --> Each loop iteration gives you a reference to the actual element (can modify original).
    const Month& m --> Reference, but read-only — avoids copies and prevents modification.
    auto m	Compiler --> deduces the type automatically.
    const auto& m --> Compiler deduces type, gives a read-only reference (common for big objects).
    */
    for (const auto& month: monthsWith31Days) {
        REQUIRE(Date::daysInMonth(month, 1989) == 31);
    }
    }

    SECTION("30-day months"){
    std::vector<Month> monthsWith30Days = {
        Month::April, Month::June, 
        Month::September, Month::November
    };

    for (const auto& month: monthsWith30Days){
        REQUIRE(Date::daysInMonth(month, 1978) == 30);
    }
    }

    SECTION("February leap"){
        REQUIRE(Date::daysInMonth(Month::February,1996) == 29);
        REQUIRE(Date::daysInMonth(Month::February,1999) == 28);
        REQUIRE(Date::daysInMonth(Month::February,2000) == 29);
        REQUIRE(Date::daysInMonth(Month::February,1900) == 28);
        REQUIRE(Date::daysInMonth(Month::February,2100) == 28);
    }

    SECTION("Invalid month throws"){
        REQUIRE_THROWS_AS(
            Date::daysInMonth(static_cast<Month>(13),2019),
            std::runtime_error);
        
        REQUIRE_THROWS_WITH(
            Date::daysInMonth(static_cast<Month>(13),2019),
            "Invalid Month passed");
    }
}

TEST_CASE("Validate days works correctly - throws error", "[Date]"){
    SECTION("Upper Bound"){
        REQUIRE_THROWS_AS(
            Date(34, Month::January, 1989),
            std::out_of_range);

        REQUIRE_THROWS_WITH(
            Date(34, Month::January, 1989),
            "Day 34 not between 1 and 31");
    }
    SECTION("Lower Bound"){
        REQUIRE_THROWS_AS(
            Date(0, Month::January, 1989),
            std::out_of_range);

        REQUIRE_THROWS_WITH(
            Date(0, Month::January, 1989),
            "Day 0 not between 1 and 31");
    }
}

TEST_CASE("Validate year works correctly - throws erros", "[Date]"){
    SECTION("Lower Bound"){
        REQUIRE_THROWS_AS(
            Date(12,Month::December, 1200),
            std::out_of_range);

        REQUIRE_THROWS_WITH(
            Date(12,Month::December, 1200),
            "Year 1200 not between 1901 and 2199");
        }
    SECTION("Upper Bound"){
        REQUIRE_THROWS_AS(
            Date(12,Month::December, 3100),
            std::out_of_range);

        REQUIRE_THROWS_WITH(
            Date(12,Month::December, 3100),
            "Year 3100 not between 1901 and 2199");
        }
}

TEST_CASE("Normalize method works correctly", "[Date]"){

    SECTION("Month - Overflow"){
        const auto [startDay, startMonth, startYear, delta, expectedDay, expectedMonth, expectedYear] =
            GENERATE_COPY(
                table<int, Month, int, int, int, Month, int>({
                    {1,  Month::January,  2024, 34,  4,  Month::February, 2024},
                    {1,  Month::May,      2023, 32,  2,  Month::June,     2023},
                    {31, Month::January,  2024, 29, 29,  Month::February, 2024}, // leap
                    {31, Month::January,  2023, 28, 28,  Month::February, 2023}, // non‑leap
                    {28, Month::February, 2023,  1,  1,  Month::March,    2023}, // non‑leap Feb → Mar
                    {29, Month::February, 2024,  1,  1,  Month::March,    2024}, // leap day → Mar 1
                    {30, Month::January,  2024, 30, 29,  Month::February, 2024}, // leap Feb end
                })
            );

        Date start(startDay, startMonth, startYear);
        Date result = start + delta;
        Date expected(expectedDay, expectedMonth, expectedYear);

        CAPTURE(startDay, static_cast<int>(startMonth), startYear,
                delta, expectedDay, static_cast<int>(expectedMonth), expectedYear,
                result.day(), result.month(), result.year());
        REQUIRE(result == expected);
    }

    SECTION("Month - Underflow"){
        const auto [startDay, startMonth, startYear, delta, expectedDay, expectedMonth, expectedYear] =
            GENERATE_COPY(
                table<int, Month, int, int, int, Month, int>({
                    {1,  Month::March,    2024, -1, 29, Month::February, 2024},
                    {1,  Month::March,    2023, -1, 28, Month::February, 2023},
                    {31, Month::March,    2024, -31,29, Month::February, 2024}, // leap
                    {30, Month::March,    2024, -30,29, Month::February, 2024}, // leap
                    {15, Month::March,    2024, -20,24, Month::February, 2024}, // leap
                })
            );

        Date start(startDay, startMonth, startYear);
        Date result = start + delta;
        Date expected(expectedDay, expectedMonth, expectedYear);

    CAPTURE(startDay, static_cast<int>(startMonth), startYear,
                delta, expectedDay, static_cast<int>(expectedMonth), expectedYear,
                result.day(), result.month(), result.year());
        REQUIRE(result == expected);
    }

    SECTION("Year - Overflow "){
        const auto [startDay, startMonth, startYear, delta, expectedDay, expectedMonth, expectedYear] =
            GENERATE_COPY(
                table<int, Month, int, int, int, Month, int>({
                    {31, Month::December, 2023,  1,  1, Month::January, 2024},
                    {25, Month::December, 2023, 10,  4, Month::January, 2024},
                    {15, Month::November, 2023, 60, 14, Month::January, 2024},
                    { 1, Month::December, 2023, 60, 30, Month::January, 2024},
                    {30, Month::December, 2023,  2,  1, Month::January, 2024},
                })
            );

        Date start(startDay, startMonth, startYear);
        Date result = start + delta;
        Date expected(expectedDay, expectedMonth, expectedYear);

    CAPTURE(startDay, static_cast<int>(startMonth), startYear,
                delta, expectedDay, static_cast<int>(expectedMonth), expectedYear,
                result.day(), result.month(), result.year());
        REQUIRE(result == expected);
    }

    SECTION("Year - Underflow "){
        const auto [startDay, startMonth, startYear, delta, expectedDay, expectedMonth, expectedYear] =
            GENERATE_COPY(
                table<int, Month, int, int, int, Month, int>({
                    {1,  Month::January, 2024, -1, 31, Month::December, 2023},
                    {5,  Month::January, 2024, -10,26, Month::December, 2023},
                    {15, Month::January, 2024, -60,16, Month::November, 2023},
                })
            );

        Date start(startDay, startMonth, startYear);
        Date result = start + delta;
        Date expected(expectedDay, expectedMonth, expectedYear);

    CAPTURE(startDay, static_cast<int>(startMonth), startYear,
                delta, expectedDay, static_cast<int>(expectedMonth), expectedYear,
                result.day(), result.month(), result.year());
        REQUIRE(result == expected);
    }

    SECTION("Multi Month - Overflow "){
        const auto [startDay, startMonth, startYear, delta, expectedDay, expectedMonth, expectedYear] =
            GENERATE_COPY(
                table<int, Month, int, int, int, Month, int>({
                    {1,  Month::January, 2023, 59,  1, Month::March,  2023}, // Jan→Feb→Mar (non‑leap)
                    {1,  Month::January, 2024, 60,  1, Month::March,  2024}, // Jan→Feb→Mar (leap)
                    {1,  Month::January, 2023, 60,  2, Month::March,  2023}, // Jan→Feb→Mar (non‑leap)
                    {30, Month::January, 2024, 31,  1, Month::March,  2024}, // via Feb (leap)
                    {1,  Month::March,   2024, 60, 30, Month::April,  2024}, // Mar→Apr via full month
                    {1,  Month::March,   2023, 60, 30, Month::April,  2023},
                    {31, Month::March,   2024, 31,  1, Month::May,    2024}, // Mar→Apr→May
                })
            );

        Date start(startDay, startMonth, startYear);
        Date result = start + delta;
        Date expected(expectedDay, expectedMonth, expectedYear);

    CAPTURE(startDay, static_cast<int>(startMonth), startYear,
                delta, expectedDay, static_cast<int>(expectedMonth), expectedYear,
                result.day(), result.month(), result.year());
        REQUIRE(result == expected);
    }

    SECTION("Multi Month - Underflow "){
        const auto [startDay, startMonth, startYear, delta, expectedDay, expectedMonth, expectedYear] =
            GENERATE_COPY(
                table<int, Month, int, int, int, Month, int>({
                    {10, Month::March, 2023, -40, 29, Month::January, 2023}, // Mar→Feb→Jan
                    { 1, Month::March, 2024, -31, 30, Month::January, 2024}, // via Feb (leap)
                    { 1, Month::March, 2024, -60,  1, Month::January, 2024}, // Mar→Jan (2 months)
                })
            );

        Date start(startDay, startMonth, startYear);
        Date result = start + delta;
        Date expected(expectedDay, expectedMonth, expectedYear);

    CAPTURE(startDay, static_cast<int>(startMonth), startYear,
                delta, expectedDay, static_cast<int>(expectedMonth), expectedYear,
                result.day(), result.month(), result.year());
        REQUIRE(result == expected);
    }

    SECTION("Multi year Overflows/Underflows") {
        const auto [startDay, startMonth, startYear, delta, expectedDay, expectedMonth, expectedYear] =
            GENERATE_COPY(
                table<int, Month, int, int, int, Month, int>({
                    {25, Month::August,   2021, +300, 21, Month::June,     2022}, // big forward jump
                    {3,  Month::July,     2013, -365,  3, Month::July,     2012}, // full year back
                    {29, Month::February, 2020, +366,  1, Month::March,    2021}, // leap +1 year
                    {1,  Month::January,  2000, -366, 31, Month::December, 1998}, // leap back
                })
            );

        Date start(startDay, startMonth, startYear);
        Date result = start + delta;
        Date expected(expectedDay, expectedMonth, expectedYear);

        CAPTURE(startDay, static_cast<int>(startMonth), startYear,
                delta, expectedDay, static_cast<int>(expectedMonth), expectedYear,
                result.day(), result.month(), result.year());
        REQUIRE(result == expected);
    }
}
