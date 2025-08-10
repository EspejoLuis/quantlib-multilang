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

    /*
TEST_CASE("Leap year behaviour around February", "[Date]"){
    Date notLeapYearDate(28, Month::February, 2023);
    Date leapYearDate1(28, Month::February, 2024);
    Date leapYearDate2(29, Month::February, 2024);
    Date notDivisibleBy400Date(28, Month::February, 1900);
    Date divisibleBy400Date(28, Month::February, 2000);
    

    Actions to perform (no code here, just what to do):

From each Feb‑28, add 1 day and check the next date.

From Feb‑29 (where it exists), add 1 day and check the next date.

From Mar‑01 in each case, subtract 1 day and check the previous date.

(Optional) Add 2–3 days across Feb‑28 to confirm multi‑day jumps land correctly.

Assertions you should write (you fill exact expectations):

Non‑leap Feb‑28 +1 → next day is March 1 of the same year.

Leap Feb‑28 +1 → next day is Feb‑29 of the same year.

Leap Feb‑29 +1 → next day is March 1 of the same year.

March 1 −1 → returns to Feb‑28 or Feb‑29 depending on leap status.

Year 1900 follows not leap; year 2000 is leap (400‑year rule).

Edge checks (pick at least one):

Add 365/366 days from Feb‑29 (leap year) and verify you land on the right date next year.

Subtract 365/366 days from Mar‑01 and verify symmetry.
*/
