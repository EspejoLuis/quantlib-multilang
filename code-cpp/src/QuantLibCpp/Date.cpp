// Date.cpp contains the definitions

#include "Date.hpp"
// Used to build strings using stream-style formatting
#include <sstream> // for std::ostringstream
// 	Used to control number formatting like padding (e.g., 01)
#include <iomanip> // for std::setw and std::setfill
#include <stdexcept>
namespace QuantLibCpp {

    Date::Date()
    // Implements the default constructor of Date
    // This ensures the object is in a known state even if the user doesn't provide input.
    : day_(1), month_(Month::January), year_(1901) {}

    Date::Date(int day, Month month, int year) {
        // Defines the main constructor used to create a specific date
        validateYearRange(year);
        validateDayInMonth(day, month, year);
        day_ = day;
        month_ = month;
        year_ = year;
    }

    int Date::day() const {
        // Getters: They return a copy of the internal fields.
        // This is the definition of the methods declared in Date.hpp
        return day_;
    }

    Month Date::month() const {
        return month_;
    }

    int Date::year() const {
        return year_;
    }

    std::string Date::toString() const {

        /*
        char* :
            - A pointer to a null-terminated string of characters
            - ['J', 'a', 'n', '\0'] --> pointed to by a char*
        const :
            - A pointer to a string that cannot be modified
            - Example: const char* s = "Jan";
                       s[0] = 'F';  --> error: cannot modify a const string
        static :
            - Create this array only once and reuse it every time toString() is called.
            - Lives for the entire program's lifetime (not just during the function call)

        Why 13 and not 12 ? Because in this way index it's starting at 1 for January
        */
        static const char* monthNames[13] = {"",    "Jan", "Feb", "Mar", "Apr", "May", "Jun",
                                             "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"};

        /*
        It creates an empty stream object. It is not a string!
        In this context, it is better than using string because allows
        to use better manipulation (setw or setfill)
        */
        std::ostringstream oss;

        /*
        std::setw(2):
            - Set the minimum width for the next number to 2 characters.
            - Example: 3 becomes " 3" (with a space in front).
        std::setfill('0'):
            - This sets the fill character to '0' instead of a space.
            - So now 3 becomes "03" — perfect for date formatting.
        All this for day_.

        static_cast<int>(month_):
            - month_ is of type Month, an enum class
            - enum class values don’t implicitly convert to integers (which is good for type safety)
            - So it explicitly converts to integer i.e. the index for month_

        monthNames[...]: will return the string corresponding to the index
        */
        oss << std::setw(2) << std::setfill('0') << day_ << "-"
            << monthNames[static_cast<int>(month_)] << "-" << year_;

        // convert the stream into string!
        return oss.str();
    }

    bool Date::operator==(const Date& other) const {
        /*
        So this
            - if (d1 == d2)
        It's tranlated in
            - d1.operator==(d2)
        */
        return day_ == other.day_ && month_ == other.month_ && year_ == other.year_;
    }

    bool Date::operator<(const Date& other) const {
        if (year_ != other.year_)
            return year_ < other.year_;
        if (month_ != other.month_)
            return month_ < other.month_;
        return day_ < other.day_;
    }

    Date Date::operator+(int days) const {
        // Marked const because the current object is not modified
        // But simply a new object is return.
        // *this means dereference the pointer to get the actual object (Date)
        // It's making a COPY
        /*
        Expression --> Type -->	Copy happens? --> Meaning
        this --> Date* -->  ❌ No --> Pointer to current object
        *this --> Date&	--> ❌ No --> Reference to current object
        Date tmp = *this --> —	--> ✅ Yes --> Makes a copy via copy ctor
        return *this; --> Date& or const Date& --> ❌ No --> Return current object by reference
        */
        Date tmpDate = *this;
        tmpDate.addDaysToCurrentDate(days);
        return tmpDate;
    }

    Date Date::operator-(int days) const {
        return Date::operator+(-days);
    }

    bool Date::isLeap(int year) {
        // Leap Year
        // Divisible by 4 And (not divisible by 100 or divisible by 400) --> LEAP (2000)
        // NDivisible by 4 and (divisible by 100 and not divisible by 400) --> NOT LEAP (2100)
        return ((year % 4 == 0) && (year % 100 != 0 || year % 400 == 0));
    }

    int Date::daysInMonth(Month month, int year) {
        switch (month) {
            case Month::February:
                return isLeap(year) ? 29 : 28;

            case Month::April:
            case Month::June:
            case Month::September:
            case Month::November:
                return 30;
            case Month::January:
            case Month::March:
            case Month::May:
            case Month::July:
            case Month::August:
            case Month::October:
            case Month::December:
                return 31;
            default:
                throw std::runtime_error("Invalid Month passed");
        }
    }

    void Date::validateYearRange(int year) {
        // Why void and not boolean ? void is intentional because this helper’s purpose is not
        // to tell if the year is valid — it’s to enforce the rule.
        if (year < 1901 || year > 2199) {
            throw std::out_of_range("Year " + std::to_string(year) + " not between 1901 and 2199");
        };
    }

    void Date::validateDayInMonth(int day, Month month, int year) {
        int daysInMonth = Date::daysInMonth(month, year);
        if (day < 1 || day > daysInMonth) {
            throw std::out_of_range("Day " + std::to_string(day) + " not between 1 and " +
                                    std::to_string(daysInMonth));
        }
    }

    void Date::normalize() {

        int daysInCurrentMonth = Date::daysInMonth(month_, year_);

        while (day_ > daysInCurrentMonth) {
            day_ = day_ - daysInCurrentMonth;
            int monthNumber = static_cast<int>(month_);
            if (monthNumber == 12) {
                int nextYear = year_ + 1;
                Date::validateYearRange(nextYear);
                year_ = nextYear;
                month_ = static_cast<Month>(1);
            } else {
                month_ = static_cast<Month>(monthNumber + 1);
            }
            daysInCurrentMonth = Date::daysInMonth(month_, year_);
        }

        while (day_ < 1) {
            int monthNumber = static_cast<int>(month_);
            if (monthNumber == 1) {
                int previousYear = year_ - 1;
                Date::validateYearRange(previousYear);
                year_ = previousYear;
                month_ = static_cast<Month>(12);
            } else {
                month_ = static_cast<Month>(monthNumber - 1);
            }

            int daysInPreviousMonth = Date::daysInMonth(month_, year_);
            day_ += daysInPreviousMonth;
        }
    }

    void Date::addDaysToCurrentDate(int days) {
        day_ = day_ + days;
        normalize();
    }


}
