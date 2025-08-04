// Tell Catch2 to generate the main() function for you
#define CATCH_CONFIG_MAIN

#include "../lib/catch.hpp"
#include "Date.hpp"

using namespace QuantLibCpp;

TEST_CASE("Deafult Date constructor sets correct values", "[Date]") {
    Date d;
    
    REQUIRE(d.day()==1);
    REQUIRE(d.month()==Month::January);
    REQUIRE(d.year()==1901);
}

//Defines a test case (can contain multiple assertions)
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

    REQUIRE(d1 == d2);
    REQUIRE_FALSE(d1 == d3);
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


//TODO Add testing for month end dates
TEST_CASE("Adding operator works correctly", "[Date]") {
    Date d1(14, Month::May, 1989);
    Date result = d1 + 17;

    REQUIRE(result.day() == 14 + 17);
    REQUIRE(result.month() == Month::May);
    REQUIRE(result.year() == 1989);
}


//TODO Add testing for month end dates
TEST_CASE("Subtracting operator works correctly", "[Date]") {
    Date d1(14, Month::May, 1989);
    Date result = d1 - 11;

    REQUIRE(result.day() == 14 - 11);
    REQUIRE(result.month() == Month::May);
    REQUIRE(result.year() == 1989);
}