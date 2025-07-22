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

        // A pointer to a string that cannot be modified
        static const char* monthNames[13] = {
            "", "Jan", "Feb", "Mar", "Apr", "May", "Jun",
            "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
        };

        std::ostringstream oss;
        oss << std::setw(2) << std::setfill('0') << day_ << "-"
            << monthNames[static_cast<int>(month_)] << "-"
            << year_;
        
        return oss.str();

    }


}