# August

## 1 August 2025: Continue Implementation of Date class in Python
- Adding validators for month/years using `field_validator()`
- Adding validator for day using `model_validator(mode=after)`:
    - Leap years and checks on number of days

## 2 August 2025: Continue Implementation of Date class in Python
-  Adding `__str__` and thought...isn't it better to move the month from `int` to enum `Month` ? This way will be allign with CPP which can be better in the long term. Actaully is by far better because then Pydantic automatically coerce!:
    - `Date(day=1, month=1, year=2025)` -> ✅ auto: Month.JANUARY.
    - `Date(day=1, month="JANUARY", year=2025)` -> # ✅ auto: Month.JANUARY.
    - `Date(day=1, month=Month.JANUARY, year=2025)` -> # ✅ already correct.
- Using: 
    - Adding `to_datetime()`.
    - Adding `__add__`.
    - Adding `__sub__`, implemented both the one for `int` and `Date`.
- To install project + dependencies and dev dependencies: `uv pip install -e ".[dev]"`

## 3 August 2025: Implementation of Date class tests in Python
- Import `calendar` and `monthrange` in `validate_day` to simplify structure.
- Check coverage using `coverage run --source=src -m pytest`
- Creating [`unit test folder`](/code-python/tests/unit/):
    - `coverage html` to create coverage report.
    - `open htmlcov/index.html` to open it in the browser. On Mac.
- Coverage 100% for units tests.
- Added integration tests.

## 4 August 2025: Implementation of coverage tests for Date class in C++
- ALWAY RUN CMAAKE COMMAND FROM THE `build` directory.
- Trying to get the coverage of tests in C++ using catch2:
    - Install `brew install lcov`
    - Add in the `CMakeLists.txt:`
    ```cmake
        if(CMAKE_BUILD_TYPE STREQUAL "Coverage")
        message(STATUS "Building with coverage flags")
        add_compile_options(--coverage)
        add_link_options(--coverage)
        endif()
    ```
    - Configure CMake for coverage: `cmake -DCMAKE_BUILD_TYPE=Coverage ..`
    - Build: `cmake build .`
    - Run: `./date_tests`
    - Collect coverage data but ignore errors from system/test headers: `lcov --capture --directory . --output-file coverage.info --ignore-errors inconsistent,unsupported`
    - Exclude system headers and test frameworks and test files: `lcov --remove coverage.info '*/tests/*' '*/lib/*' '/usr/*' '/opt/*' '/Library/*' --output-file coverage.filtered.info`
    - Create HTML report: `genhtml coverage.filtered.info --output-directory coverage_report`
    - Open report: `open coverage_report/index.html`

## 7 August 2025: Implementation of edge cases for Date class in C++
- Added leap years: `isLeap`.
- Added number of days in the month: `daysInMonth`.

## 8 August 2025: Implementation of edge cases for Date class in C++
- Added unit tests for `isLeap` and `daysInMonth`
- Creating two builds. One normal build for debug and release and one instrumented build for coverage. QuantLib itself does this.
    - Create new build with no coverage:
        - `cd code-ccp`
        - `rm -rf build`
        - Configure with build type: `cmake -S . -B build -DCMAKE_BUILD_TYPE=Debug`
        - Build your tests: `cmake --build build --target date_tests`
        - Runt test from build fodler: `./date_tests`
    - Create separate build with coverage:
        - `cd code-ccp`
        - Remove any old  build directory: `rm -rf build-coverage`
        - Configure with coverage build type: `cmake -S . -B build-coverage -DCMAKE_BUILD_TYPE=Coverage`
        - Build your tests: `cmake --build build-coverage --target date_tests`
        - From build-coverage folder, run test: `./date_tests`
    - Modified `CMakeLists` file.
    - From build-coverage:
        - `cd build-coverage` 
        - Run `LLVM_PROFILE_FILE="date.profraw" ./date_tests`. The last commnand will create the file `date.profraw`.
        - Terminal output: `xcrun llvm-profdata merge -sparse date.profraw -o date.profdata`
        - Terminal report: `xcrun llvm-cov report ./date_tests -instr-profile=date.profdata`
        - HTML report: `xcrun llvm-cov show ./date_tests -instr-profile=date.profdata -format=html -output-dir=coverage_html`
        - Open HTML `open coverage_html/index.html` 
    - Update the `CmakeLists` to do this:
        - From `code-ccp`
            - `cmake -S . -B build-coverage -DCMAKE_BUILD_TYPE=Coverage`
            - `cmake --build build-coverage`
            - `cmake --build build-coverage --target coverage`
            - From build-coverage folder: `./date_tests`
            - `open build-coverage/coverage_html/index.html `
        - From `code-ccp`
            - `cmake -S . -B build -DCMAKE_BUILD_TYPE=Debug`
            - `cmake --build build --target date_tests`
            - From build folder`./date_tests`

## 9 August 2025: Coverage test and integration tests for Date class in C++
- Want to run the coverage test report automatically, without too much effort. Created a run_test_with_coverage.sh file in [`scripts`](/code-cpp/scripts/). Now just do:
    - `bash code-cpp/scripts/run_test_with_coverage.sh`
- Will do the smae for the test with no coverage (debug):
    - `bash code-cpp/scripts/run_test.sh`
- Difference between the two is that:
    - Coverage = Configure (Coverage) → Build binaries → Run coverage target. This is why for coverage, there is one more line to run i.e. `cmake --build build-coverage`.
    - Debug = Configure (Debug) → Build only what you need.
- Integration test :
    - Created empty file. Adding it to both build and build-coverage.
    - Created new scripts and modified old ones. TOOK LONG JEEEZ! 
    - Adding integration tests for February.....need first to have a method in Date that allows to move from one month to the next:
        - Creating `normalize` method.

## 10 August 2025: Validation methods for Date class in C++
- Created a private days validator method.
- Created the `normalize` private method:
    - Adding unit test for normalize. A LOT!  
    - Check coverage tests. All 100% !

## TO DO
- C++:
    ❌ Check coverage. Some issue 
    ✅ Validation of inputs (e.g. no check for 30 Feb, etc. or for adding 11 to 30)
    ✅ Leap year awareness
    ✅ Overflow when adding days
    ❌ Integration tests (we only have unit test DateTest_catch.cpp)
    ✅  No utility like daysInMonth(month, year)
    ✅ No conversion logic from overflowing days to next month/year

- C++:  - Or manually: `g++ --coverage -o test_date DateTest_catch.cpp` -> To check if it's correct.
- Unit Tests:
    - Date validation to avoid 30 february for C++/C#/Rust. In python should be easier
    - Function for adding/subtracting month,years not just days:
        - What if days are more than 30/31
        - What if days are negative ?
        - Same for months ?
        - What if subtracting Dates instead of just days
        - Need to implement calendar logic
    - Take into account for starting a new month/year
    - Can we use DateTime in C# ? for operations with dates instead of creating our own ?
    - Rust:
        Assumption: for now that the input is always non-negative, and that self.to_serial() + n will never underflow (negative dates will be handled later)
- Integration tests:
    - Rust
    - C#
    - C++