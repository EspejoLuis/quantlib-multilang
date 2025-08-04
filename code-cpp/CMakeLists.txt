# Specify the minimum version of CMake required to process this file
cmake_minimum_required(VERSION 3.10)

# Set the name of the project
project(QuantLibCpp)

# Set the C++ standard to use (C++17 here)
set(CMAKE_CXX_STANDARD 17)

# Define a library target named 'quantlib_cpp' made from your Date class
# This will compile src/QuantLibCpp/Date.cpp into a reusable static library
add_library(quantlib_cpp src/QuantLibCpp/Date.cpp)

# Specify the include directory for the quantlib_cpp library
# Any target that links to quantlib_cpp will be able to find headers here
target_include_directories(quantlib_cpp PUBLIC src/QuantLibCpp)

##### EXECUTE #####
# Define an executable target named 'date_tests'
# This is your Catch2 test runner
add_executable(date_tests tests/unit/DateTest_catch.cpp)

# Include directories needed to compile 'date_tests'
# - tests/lib: for catch.hpp
# - src/QuantLibCpp: for Date.hpp
target_include_directories(date_tests PRIVATE tests/lib src/QuantLibCpp)

# Link the quantlib_cpp library into the 'date_tests' executable
target_link_libraries(date_tests quantlib_cpp)

if(CMAKE_BUILD_TYPE STREQUAL "Coverage")
    message(STATUS "Building with coverage flags")
    
    # Set coverage flags for both targets
    target_compile_options(quantlib_cpp PRIVATE --coverage -g -O0)
    target_compile_options(date_tests PRIVATE --coverage -g -O0)
    
    # Add coverage linker flags
    target_link_options(quantlib_cpp PRIVATE --coverage)
    target_link_options(date_tests PRIVATE --coverage)
    
    # Ensure proper file generation
    set(CMAKE_CXX_OUTPUT_EXTENSION_REPLACE ON)
endif()