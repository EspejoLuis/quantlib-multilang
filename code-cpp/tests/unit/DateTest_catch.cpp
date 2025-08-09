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
    Date d(23, Month::May, 1787);
    REQUIRE(d.toString() == "23-May-1787");
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

TEST_CASE("Adding operator works correctly", "[Date]") {
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