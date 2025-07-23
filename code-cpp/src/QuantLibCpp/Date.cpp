//Date.cpp contains the definitions 

#include "Date.hpp"
//Used to build strings using stream-style formatting
#include <sstream>   // for std::ostringstream
// 	Used to control number formatting like padding (e.g., 01)
#include <iomanip>   // for std::setw and std::setfill 

namespace QuantLibCpp {

    // Implements the default constructor of Date
    // This ensures the object is in a known state even if the user doesn't provide input.
    Date::Date()
        : day_(1), month_(Month::January), year_(1901) {}

    // Defines the main constructor used to create a specific date
    // Note : There is no validation yet 
    Date::Date(int day, Month month, int year)
        : day_(day), month_(month), year_(year) {}

    // Getters: They return a copy of the internal fields.
    // This is the definition of the methods declared in Date.hpp
    int Date::day() const {
        return day_;
    }

    Month Date::month() const {
        return month_;
    }

    int Date::year() const {
        return year_;
    }

    std::string Date::toString() const{

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
         static const char* monthNames[13] = {
            "",
            "Jan", "Feb", "Mar", "Apr", "May", "Jun",
            "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
        };

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
            << monthNames[static_cast<int>(month_)] << "-"
            << year_;
        
        // convert the stream into string!
        return oss.str();   
    }
    
    /*
    So this 
        - if (d1 == d2)
    It's tranlated in
        - d1.operator==(d2)
    */
    bool Date::operator==(const Date& other) const{
        return day_ == other.day_ &&
               month_ == other.month_ &&
               year_ == other.year_;
    }

    bool Date::operator<(const Date& other) const{
        if (year_ != other.year_) return year_ < other.year_;
        if (month_ != other.month_) return month_ < other.month_;
        return day_ < other.day_ ;
    }

    /*
    NOTE: Implementation doesn’t handle:
    - Day overflow into the next month
    - Leap years
    - Month/year changes
    */

    // Marked const because the current object is not modified
    // But simply a new object is return.
    Date Date::operator+(int days) const{
        return Date(day_ + days, month_, year_);
    }
    Date Date::operator-(int days) const{
        return Date(day_ - days, month_, year_);
    }

}