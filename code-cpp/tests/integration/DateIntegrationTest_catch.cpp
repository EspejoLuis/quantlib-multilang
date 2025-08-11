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

TEST_CASE("Range limit", "Date"){
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
}
