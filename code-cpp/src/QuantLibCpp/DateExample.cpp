/*
So even though you're including just Date.hpp, the compiler will:
    - Use the declarations from Date.hpp to understand how to call functions
    - Use the compiled Date.cpp to actually run the code you've implemented
*/
#include <iostream>
#include "Date.hpp"

using namespace QuantLibCpp;

int main() {

    Date d1(13, Month::March, 1993);
    Date d2(14, Month::May, 1989);
    Date d3(30,Month::January, 2024);
    Date d4; //Default Constructor

    std::cout << "d1 = " << d1.toString() << std::endl;
    std::cout << "d2 = " << d2.toString() << std::endl;
    std::cout << "d3 = " << d3.toString() << std::endl;

    std::cout << "d1 = d2 ? " << (d1 == d2 ? "true" : "false") << std::endl;
    std::cout << "d1 < d2 ? " << (d1 < d2 ? "true" : "false") << std::endl;

    Date d5 = d1 + 2; 
    std::cout << "d4 + 2 days = " << d5.toString() << std::endl;

    Date d6 = d1 - 1; 
    std::cout << "d5 - 1 day = " << d6.toString() << std::endl;

    Date d7 = d3 + 2; 
    // Doesn't take into account for month ends
    std::cout << "d3 + 2 day = " << d7.toString() << std::endl;

    std::cout << "default date constructor " << d4.toString() << std::endl;

    return 0;
}
