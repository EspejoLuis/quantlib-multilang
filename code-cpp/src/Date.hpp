#pragma once // Include this header file only once, no matter how many times it's imported.
#include <string>

namespace QuantLibCpp {

    /*
    - To access values with Month::January, not just January
    - Start at 1
    - Allows more control
    */
    enum class Month{
        January= 1, February, March, April, May, June,
        July, August, September, October, November, December
    };

    class Date{
        public:
            //Default Constructor
            Date(); 

            // The main constructor: allows a user to create any valid date
            Date(int day, Month month, int year); 
            
            // These are getters for the private member variables.
            // const because they will not modify the object.
            // This is just the declaration of the method but not the definition.
            int day() const;
            Month month() const;
            int year() const;

            // Returns a string for date
            std::string toString() const; 

            /*
            The following will allow to compare dates
            if (d1 == d2) { ... }
            if (d1 < d2) { ... }
            Behind the scenes, this is whay is called:
            d1.operator==(d2);
            - bool --> return type
            - const Date& other: 
                - A reference to another Date object you're comparing to
                - Passed by const reference:
                    - const = telling compiler we are not to modify `other`.
                    - & = avoid copying but referecing 
            - const --> method itself as const --> not to modify the current object
            */
            bool operator==(const Date& other) const;
            bool operator<(const Date& other) const;

            /*
            Basic aritmetic implementation
            Date tomorrow = today + 1;
            Date yesterday = today - 1;
            */
            Date operator+(int days) const;
            Date operator-(int days) const;
        
        private:

            // These are the actual fields where data is stored.
            int day_;
            Month month_;
            int year_;
    };
}