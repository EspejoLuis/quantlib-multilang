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

##### Unit Test EXECUTABLE #####
# Define an executable target named 'date_tests_unit'
# This is your Catch2 test runner
add_executable(date_tests_unit tests/unit/DateTest_catch.cpp)
# Include directories needed to compile 'date_tests_unit'
# - tests/lib: for catch.hpp
# - src/QuantLibCpp: for Date.hpp
target_include_directories(date_tests_unit PRIVATE tests/lib src/QuantLibCpp)
# Link the quantlib_cpp library into the 'date_tests_unit' executable
target_link_libraries(date_tests_unit quantlib_cpp)

##### Integration Test EXECUTABLE #####
add_executable(date_tests_integration tests/integration/DateIntegrationTest_catch.cpp)
target_include_directories(date_tests_integration PRIVATE tests/lib src/QuantLibCpp)
target_link_libraries(date_tests_integration quantlib_cpp)

# ------------------------------------------------------------------------------------
# Only apply this configuration if the build type is explicitly set to "Coverage"
# Usage:
#   cmake -S . -B build-coverage -DCMAKE_BUILD_TYPE=Coverage
# ------------------------------------------------------------------------------------
if(CMAKE_BUILD_TYPE STREQUAL "Coverage")
    message(STATUS "Building with LLVM coverage instrumentation")

    # Apply to everything in this build (belt-and-braces)
    add_compile_options(-fprofile-instr-generate -fcoverage-mapping -O0 -g)
    add_link_options(-fprofile-instr-generate)

    # --- 1) Add compiler flags for coverage instrumentation ---
    # -fprofile-instr-generate : instruments code to generate profiling counters (.profraw file)
    # -fcoverage-mapping       : generates mapping info so llvm-cov can map counters to source lines
    # -O0                      : disable optimisations (ensures accurate mapping)
    # -g                       : include debug symbols for better coverage reports
    target_compile_options(quantlib_cpp           PRIVATE -fprofile-instr-generate -fcoverage-mapping -O0 -g)
    target_compile_options(date_tests_unit        PRIVATE -fprofile-instr-generate -fcoverage-mapping -O0 -g)
    target_compile_options(date_tests_integration PRIVATE -fprofile-instr-generate -fcoverage-mapping -O0 -g)

    # --- 2) Add linker flags for coverage ---
    target_link_options(quantlib_cpp           PRIVATE -fprofile-instr-generate)
    target_link_options(date_tests_unit        PRIVATE -fprofile-instr-generate)
    target_link_options(date_tests_integration PRIVATE -fprofile-instr-generate)

    # Paths/patterns to ignore from reports (tests + Catch header)
    set(ignore_regex_tests ".*/tests/")
    set(ignore_regex_catch "catch\\.hpp$")

    # --- 3) Define a custom target "coverage" to:
    #   a) Build the tests (unit + integration)
    #   b) Run them with LLVM_PROFILE_FILE set so output goes to separate .profraw files
    #   c) Merge the raw profiling data into a single .profdata
    #   d) Generate a terminal coverage report (for both binaries)
    #   e) Generate an HTML coverage report in coverage_html/ (for both binaries)
    add_custom_target(
        coverage
        # Start clean: remove previous coverage artifacts so paths map cleanly
        COMMAND ${CMAKE_COMMAND} -E remove -f unit.profraw integ.profraw merged.profdata
        COMMAND ${CMAKE_COMMAND} -E remove_directory coverage_html

        # Run UNIT tests -> unit.profraw
        COMMAND ${CMAKE_COMMAND} -E env LLVM_PROFILE_FILE=unit.profraw
                $<TARGET_FILE:date_tests_unit>

        # Run INTEGRATION tests -> integ.profraw
        COMMAND ${CMAKE_COMMAND} -E env LLVM_PROFILE_FILE=integ.profraw
                $<TARGET_FILE:date_tests_integration>

        # Merge profiles
        COMMAND xcrun llvm-profdata merge -sparse unit.profraw integ.profraw -o merged.profdata

        # Generate a terminal coverage summary report for BOTH binaries
        COMMAND xcrun llvm-cov report
                $<TARGET_FILE:date_tests_unit>
                -object=$<TARGET_FILE:date_tests_integration>
                -instr-profile=merged.profdata
                -ignore-filename-regex=${ignore_regex_tests}
                -ignore-filename-regex=${ignore_regex_catch}
                -path-equivalence=${CMAKE_SOURCE_DIR},.

        # Create the HTML output directory
        COMMAND ${CMAKE_COMMAND} -E make_directory coverage_html

        # Generate a detailed HTML coverage report for BOTH executables
        # NOTE: we chdir to SOURCE dir so 'src/…' paths resolve and files render.
        COMMAND ${CMAKE_COMMAND} -E chdir ${CMAKE_SOURCE_DIR} xcrun llvm-cov show
                $<TARGET_FILE:date_tests_unit>
                -object=$<TARGET_FILE:date_tests_integration>
                -instr-profile=${CMAKE_BINARY_DIR}/merged.profdata
                -format=html
                -output-dir=${CMAKE_BINARY_DIR}/coverage_html
                -ignore-filename-regex=${ignore_regex_tests}
                -ignore-filename-regex=${ignore_regex_catch}
                --show-line-counts-or-regions
                --show-branches=count
                --show-expansions
                -path-equivalence=${CMAKE_SOURCE_DIR},.
                ${CMAKE_SOURCE_DIR}/src/QuantLibCpp

        # Optional (macOS): auto-open the report
        COMMAND open coverage_html/index.html

        # Run everything from the build directory so output paths are clean
        WORKING_DIRECTORY ${CMAKE_BINARY_DIR}

        # Ensure BOTH tests are built before running coverage
        DEPENDS date_tests_unit date_tests_integration

        COMMENT "Running unit + integration with LLVM coverage and generating merged terminal + HTML reports (with branch details)"
    )
endif()
