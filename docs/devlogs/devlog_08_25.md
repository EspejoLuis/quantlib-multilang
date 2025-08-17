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
        - Runt test from build folder: `./date_tests`
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

## 11 August 2025: Integration tests for Date class in C++
- Added 3 integration tests for date class:
    - Testing range limit i.e. `validateYearRange()` and `toString()`.
    - Testing year roll over.
- Done all the following points:
    - ✅ Check coverage. Some issue 
    - ✅ Validation of inputs (e.g. no check for 30 Feb, etc. or for adding 11 to 30)
    - ✅ Leap year awareness
    - ✅ Overflow when adding days
    - ✅ Integration tests
    - ✅ No utility like daysInMonth(month, year)
    - ✅ No conversion logic from overflowing days to next month/year
- Reviewing C#

## 12 August 2025: C# Unit Test

- Instead of using System.DateTime, going to use System.DateOnly. Not aiming at doing intraday valuation so time is not really needed just Date:
    - Modified the code accordingly

## 14 August 2025: C# Unit Test

- Checked test: `dotnet test QuantLibCSharp.sln`.
- Adding a coverage tool also for C#:
    ```
    cd code-csharp/QuantLibCSharp.Tests
    dotnet add package coverlet.collector
    dotnet build
    ```
- From test folder:
    ```
    cd code-csharp/QuantLibCSharp.Tests
    dotnet test --collect:"XPlat Code Coverage" --results-directory ./TestResults
    ```
- From c# root:
    ```
    cd code-csharp
    dotnet new tool-manifest
    dotnet tool install dotnet-reportgenerator-globaltool
    ```
- Run `dotnet tool run reportgenerator -reports:"tests/QuantLibCSharp.Tests/TestResults/**/coverage.cobertura.xml"  -targetdir:"CoverageReport" -reporttypes:"Html;TextSummary"`
- Created a bash file in [scripts](/code-csharp/scripts/)
- Added units tests! Many. Only `GetHashCode` missing.

## 15 August 2025: C# Unit Test
- Completed 100% unit tests.

## 16 August 2025: Structure and C# Integration tests
- Structure:
    - Create new workspace. This contains the multilang and the 4 languages folders. The 4 folders are simply added to the workspace so the `settings.json` are correctly read by vscode from each folder.
    - Settings are in `.vscode/setting.json` for each language and in the root multilang folder.
    - Set tests in Testing window in VSCode for 3 languages:
        - ✅ C# --> C# Dev Kit with coverage
        - ✅ C++ --> TestMate C++ with coverage
        - ✅ Python --> Pytest with coverage
        - ❌ Rust --> Rust-Analyzer without coverate. Still doesn't support coverage so have to do it manually.
    - Coverage reports and test can be anyway run also manually.
    - Also debugging is working. Nice.
- C#:
    - Created UniteTests and IntegrationTests projects.
    - Updated the scripts to manually run coverage reports
    - Integration Tests:
        - In Unit Tests there were tests that should have been put in integration.
    - Review:
        - ✅ Validation of inputs --> is it in DateOnly
        - ✅ Leap year awareness
        - ✅ Overflow when adding days --> DateOnly
        - ✅ Add unit tests
        - ✅ Divide unit test when expecation is true or false
        - ✅ Integration tests.
        - ✅ No utility like daysInMonth(month, year)
        - ✅ No conversion logic from overflowing days to next month/year --> DateOnly
        - ✅ Check coverage. Some issue 
        - ✅ Null cases???
- Python:
    - Structured better the tests (integration and unit)
    - Created scripts files to run tests automatically with coverage. Testing covers already but wanted to do it manually
- Rust:
    - Reviewing code written before to refresh memory
- C++:
    - Installled `clang-format` as extension to have formatting when saving the file.

# 16 August 2025: Rust Review
- Review code

# TODO:
- Date:
    - Rust:
        - ❌  Validation of inputs --> is it in DateOnly
        - ❌  Leap year awareness
        - ❌  Overflow when adding days --> DateOnly
        - ❌  Add unit tests
        - ❌  Divide unit test when expecation is true or false
        - ❌  Integration tests.
        - ❌  No utility like daysInMonth(month, year)
        - ❌  No conversion logic from overflowing days to next month/year --> DateOnly
        - ❌  Check coverage. Some issue 
        - ❌  Null cases
    - C++:
        - ❌ operator > 
        - ❌ opeartor - for two dates
        - ❌ Enf of month/IsEndOfMonth
        - ❌ Different
    - Python: Review:
        - We are using datetime + day,month,year. Is it correct ? should we store just datime so as to have one soruce of true ?
    - Date Parser:
        - Given string create Date
