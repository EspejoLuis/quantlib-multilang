#define CATCH_CONFIG_MAIN
#include "../lib/catch.hpp"
#include "Date.hpp"

using namespace QuantLibCpp;

TEST_CASE("Date end to end add/sub", "[Date]"){
    Date d(14, Month::May, 1989);
    Date d1 = d + 11;
    Date dateCreated = d1 - 2;
    Date dateExpected = Date(23, Month::May, 1989);

    REQUIRE(dateCreated == dateExpected);
}

TEST_CASE("Range limit", "[Date]"){
    SECTION("Upper bound + toString"){
        Date d(31, Month::December, 2199);

        REQUIRE(d.toString() == "31-Dec-2199");
        REQUIRE_THROWS_AS(d + 1,
            std::out_of_range);
    }

    SECTION("Lower bound + toString"){
        Date d(01, Month::January, 1901);

        REQUIRE(d.toString() == "01-Jan-1901");
        REQUIRE_THROWS_AS(d - 1,
            std::out_of_range);
    }

    SECTION("Safe year rollover "){
        Date originalDate(30, Month::December, 2024);
        REQUIRE(originalDate.toString() == "30-Dec-2024");

        Date dateCreated = originalDate + 3;
        Date dateExpected(2, Month::January, 2025);
        REQUIRE(dateExpected.toString() == "02-Jan-2025");
       
        REQUIRE(originalDate < dateExpected);
        REQUIRE(dateCreated == dateExpected);

        Date newDate = dateExpected - 3;
        REQUIRE(newDate.toString() == "30-Dec-2024");
        REQUIRE(newDate == originalDate);

    }

}
