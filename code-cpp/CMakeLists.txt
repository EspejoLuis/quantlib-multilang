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


# ------------------------------------------------------------------------------------
# Only apply this configuration if the build type is explicitly set to "Coverage"
# Usage:
#   cmake -S . -B build-coverage -DCMAKE_BUILD_TYPE=Coverage
# ------------------------------------------------------------------------------------
if(CMAKE_BUILD_TYPE STREQUAL "Coverage")
    message(STATUS "Building with LLVM coverage instrumentation")

    # --- 1) Add compiler flags for coverage instrumentation ---
    # -fprofile-instr-generate : instruments code to generate profiling counters (.profraw file)
    # -fcoverage-mapping       : generates mapping info so llvm-cov can map counters to source lines
    # -O0                      : disable optimisations (ensures accurate mapping)
    # -g                       : include debug symbols for better coverage reports
    target_compile_options(quantlib_cpp PRIVATE -fprofile-instr-generate -fcoverage-mapping -O0 -g)
    target_compile_options(date_tests   PRIVATE -fprofile-instr-generate -fcoverage-mapping -O0 -g)

    # --- 2) Add linker flags for coverage ---
    # This ensures the generated binary is linked with the profiling runtime
    target_link_options(quantlib_cpp PRIVATE -fprofile-instr-generate)
    target_link_options(date_tests   PRIVATE -fprofile-instr-generate)

    # --- 3) Define a custom target "coverage" to:
    #   a) Build the tests
    #   b) Run them with LLVM_PROFILE_FILE set so output goes to date.profraw
    #   c) Merge the raw profiling data into date.profdata
    #   d) Generate a terminal coverage report
    #   e) Generate an HTML coverage report in coverage_html/
    add_custom_target(
        coverage
        # Run tests with LLVM_PROFILE_FILE set to write profiling data
        COMMAND ${CMAKE_COMMAND} -E env LLVM_PROFILE_FILE=date.profraw
                $<TARGET_FILE:date_tests>

        # Merge the raw profiling data (.profraw) into an indexed format (.profdata)
        COMMAND xcrun llvm-profdata merge -sparse date.profraw -o date.profdata

        # Generate a terminal coverage summary report
        COMMAND xcrun llvm-cov report $<TARGET_FILE:date_tests>
                -instr-profile=date.profdata

        # Create the HTML output directory
        COMMAND ${CMAKE_COMMAND} -E make_directory coverage_html

        # Generate a detailed HTML coverage report
        COMMAND xcrun llvm-cov show $<TARGET_FILE:date_tests>
                -instr-profile=date.profdata
                -format=html
                -output-dir=coverage_html

        # Run everything from the build directory so output paths are clean
        WORKING_DIRECTORY ${CMAKE_BINARY_DIR}

        # Ensure the tests are built before running coverage
        DEPENDS date_tests

        COMMENT "Running tests with LLVM coverage and generating terminal + HTML reports"
    )
endif()



