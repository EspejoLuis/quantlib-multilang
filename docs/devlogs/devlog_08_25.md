# August

## 1 August 2025: Continue Implementation of Date class in Python

- Adding validators for month/years using `field_validator()`
- Adding validator for day using `model_validator(mode=after)`:
  - Leap years and checks on number of days

## 2 August 2025: Continue Implementation of Date class in Python

- Adding `__str__` and thought...isn't it better to move the month from `int` to enum `Month` ? This way will be allign with CPP which can be better in the long term. Actaully is by far better because then Pydantic automatically coerce!:
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
    - Remove any old build directory: `rm -rf build-coverage`
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

## 17 August 2025: Rust Review

- Review code
- Quantlib uses serial numbers not normalize functions (see beloew). Going to implement option B in Rust, then modify C++ to allign because as of now, approach A is used in C++
  - Approach A — Normalize loop (what you coded in C++ first)
    - You add/subtract days directly from (day, month, year).
    - Then run a normalize() loop to “fix” cases like:
    - Day = 32 → push into next month
    - Day = 0 → pull back into previous month
    - This is very manual, step-by-step adjustment.
  - Approach B — Serial numbers (what we’re now doing in Rust, and also what QuantLib does internally)
    - Convert (day, month, year) into a single integer (serial).
    - Add/subtract days directly on that integer.
    - Convert back into (day, month, year).
    - No loop, no manual adjustments — the modular arithmetic handles everything.
- ✅ `month_offset` created.
- ✅ `year` from serial number.

## 18 August 2025: Rust Date

- `imp Date` ok:
  - ✅ `month` from serial number.
  - ✅ `year` from serial number.
  - ✅ Operators: +/- - dates
  - ✅ Leap year awareness
- Used ChatGPT to create .sh scripts to run automatically coverage and tests.

## 19 August 2025: Rust Date:

- ✅ Add normalize()
- ✅ Overflow when adding days, normalize
- ✅ No utility like :
  - ✅ daysInMonth(month, year)
  - ✅ to string
  - ✅ from serial
  - ✅ to serial

## 20 August 2025: Rust Date:

- Normalize is actually not implemented in Quantlibe. Deleted.
- Implemented `days_in_month()` and `Month::fromi32()`
- Using [run_test_with_coverage.sh](/code-rust/scripts/run_test_with_coverage.sh) to check unit tests and add new unit tests accordingly.
- ✅ No conversion logic from overflowing days to next month/year.
- Thinking of borrowing (by ref) vs ownership (by value)

## 22 August 2025: Rust Date

- ✅ Add unit tests --> Used ChatGPT after a certain point.
- General refactoring
- Adding better loggin when panicking.
- Shuuld be serial number alligned with Excel ??? yes. At least this is what Quantlib does.

## 23 August 2025: Rust Date

- Relying on ChatGPT not always good. Date was an issue. Sometimes it was saying that Quantlib does implement a formula to define leap years, the classic (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0). Sometimes it says it uses arrays. Everytime it was switching from one to the other way. From now on is better to always ask for the direct reference in the github repo and check myself what is the implementation. And upload the corrispodning file in the folder.
- Refactor entire date class and unit tests
- ✅ Divide unit test when expecation is true or false
- ✅ Validation of inputs , validate day , validate year
- LCOV coverage doesn't allow to exclude inline tests (the ones in the function). For now it's ok. But maybe in the future is worth separating the unit test in another file.

## 24 August 2025: Rust Date

- ✅ Better panicking handling --> Asssert!
- ✅ Operators: > < != == . These are already implemented because of #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]: #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]:
  - PartialEq gives == and != logic
  - Eq: does not give anything more but confirms that == logic behaves mathematically sensibly. For example a == a can be false if a is Nan. By adding Eq that possibility is excluded a priori --> Rules out NaN cases!
  - PartialOrd --> Enables <, <=, >, >=.
  - Ord --> Enables full ordering (like sorting) --> Rules out NaN cases!
  - Using asser_eq!(d1, d1, "xxx") mean Rust will try to show the value when the test fails but to do that `Debug` is needed
  - Copy: Means values of this type can be copied bit-for-bit instead of moved. For Month, that’s fine: it’s just a tiny integer under the hood (the discriminant).
  ```
  Effect: you can do:
  let m1 = Month::March;
  let m2 = m1;      // this makes a *copy*, not a move
  let m3 = m1;      // ❌ still works, m1 is still valid
  If Month were not Copy, the assignment would “move” it, and m1 couldn’t be used anymore.
  ```
  - Clone: Gives you a .clone() method that makes an explicit copy. Normally, Clone can mean deep copies (like duplicating a vector). For a Copy type like Month, .clone() just does the same as assignment.
  ```
  Example:
  let m1 = Month::April;
  let m2 = m1.clone();  // same as m1
  ```
  - Started `weekdays.rs` file and updating `date.rs` with:
    - `iso_date`, `short_date`, `long_date`.
    - `is_end_of_month`,`end_of_month`.

## 25 August 2025: Rust Date

- From now onwards, the strategy will be to start from the cpp/hpp files in quantlib. Upload them in ChatGPT. Focus on just those function that can be implemented (i.e. dont need other files). Afterwards, start adding the other functions (cpp/hpp) that are needed.
- Start from CPP version and cascade to the other ones
- Added `todays_date()` to date
- Modified `Cargo.toml` to get chronos lib (for `todays_date()`)
- ✅ `weekday.rs`, `weekday()` in `date.rs`
- ❓ Thinking about having `enum month` in a proper `month.rs`
- ✅ Weekday-related:
  - ✅ `weekday()`
- ✅ Weekday utilities
  - ✅ `next_weekday()`
  - ✅ `nth_weekday()`

## 26 August 2025: Rust time unit, frequency

- ✅ `time_unit.rs`.
- ✅ `frequency.rs`.
- Working on improving the detail,io structure for `date.rs`.

## 27 August 2025: Rust time unit, frequency

- Changed structure to allign with Quantlib.
- Add unit tests for `time_unit.rs`,`frequency.rs`,`dateformatter.rs`,`weekdays.rs`.

## 28 August 2025 - Rust

- Added `period.rs`.
- Added unit tests...heavily relying on ChatGPT. The prompt is based on having loops, one max two assertion per functions. When panic, tests using `panic::catch_unwind`.
- ❌ Period --> Checking coverage. One branch missing: done it was because of the two condition in here:

  - Version 1 (old):

  ```rust
  if 12 % abs_length == 0 && abs_length <= 12 { ... }
  ```

  B was never evaluated as false:

  | abs_length | Left (A = `12 % abs == 0`) | Right (B = `<= 12`) | Expression | Coverage note        |
  | ---------- | -------------------------- | ------------------- | ---------- | -------------------- |
  | 6          | true                       | true                | true       | ✅ A=true, B=true    |
  | 5          | false                      | (skipped)           | false      | ⚠️ B never evaluated |
  | 13         | false                      | (skipped)           | false      | ⚠️ B never evaluated |
  | 12         | true                       | true                | true       | ✅ same as 6         |

  - Version 2 (new):

  ```rust
  if abs_length <= 12 && 12 % abs_length == 0 { ... }
  ```

  | abs_length | Left (A = `<= 12`) | Right (B = `12 % abs == 0`) | Expression | Coverage note           |
  | ---------- | ------------------ | --------------------------- | ---------- | ----------------------- |
  | 6          | true               | true                        | true       | ✅ A=true, B=true       |
  | 5          | true               | false                       | false      | ✅ A=true, B=false      |
  | 13         | false              | (skipped)                   | false      | ✅ A=false case covered |

- Added `normalize()` and `normalized()`. C++ code uses the below in `period.hpp`
  ```cpp
  inline Period Period::normalized() const {
          Period p = *this;
          p.normalize();
          return p;
      }
  ```
  - Small function inline in the header means it doesn’t get compiled into its own symbol in the .o file.
  - Instead, its body is copied directly into every caller at compile time.

## 29 August 2025 - Rust

- Conversion methods `years()`, `months()`, `weeks()`, `days()` + tests.
  - QuantLib only allows conversions that are always exact:
    - ✅ Years - Months
    - ✅ Weeks - Days
  - And it rejects conversions that depend on a calendar or context:
    - ❌ Months - Days
    - ❌ Years - Days
    - ❌ Weeks - Months
- `+=` implementation with `AddAssign` trait ✅. Unit test as well.
  - ❌ The panic unknown time unit cases still to do.

## 31 August 2025 - Rust

- Delete timunits: seconds, microseconds, milliseconds, hours, minutes. Adjust unit tests.
- `-` and `-=` implementation with `SubAssign` and `Neg` traits:
  - We need `-a` not `a-b` so that's why use Neg instead of Sub!
- `*-` implemented with `MulAssign`
- `/=` implemented with `DivAssign`
- `+`, `/`,`-` implemented with `Add`,`Div`,`Sub`
- Unit test for all !!
- Note:
  - `+=` `/=` `\*=` `-=` always return the same object modified.
  - `-` `/` `*` `/` always return a new object.
- Operations always act on days < weeks < months < years.
- `days_min_max`: It takes a Period (length + unit) and returns a range of possible days (min_days, max_days).This is needed because some periods (like “1 month”) don’t map to a fixed number of days.
- Wanted to implement just `<` like C++ does. But cannot do the same in Rust. Have to implement all the `partila_ord` trait. Kinda Annoying because this means that i have to have one specific cases for Great/Less/Equal
- Note:
  ```md
  Once == and < exist, you can always write the others in terms of them:
  a > b → b < a
  a <= b → !(b < a)
  a >= b → !(a < b)
  So you don’t need >, <=, >= explicitly.
  ```

## 1 Sep 2025 - Rust:

- Checking coverage in period --> completed.
- ✅ Formatting (Display / long_period, short_period) with unit test!

### TODO:

- Should i call length and unit with .length or .length()
- Remove some partialOrd where not needed
- ❓ Thinking about having `enum month` in a proper `month.rs`
- What about using Size (usize) instead of MonthIndex or WeekDayIndex
- Date:

  - Rust:
    - ❌ Integration tests.
    - ❌ Check coverage. Some issue
    - ❌ Null cases
    - ❌ Parsing
    - ❌ parseISO(const std::string&) (takes "2024-07-23" and turns it into a Date)
    - ❌ d.toFormattedString("%d-%b-%Y");

- C++:

  - ❌ EVERYTHING!!!

- C#: review everything according to new strategy:

  - ❌ EVERYTHING!!!

- Python: review everything according to new strategy:

  - ❌ EVERYTHING!!!

- Python: Review:
  - We are using datetime + day,month,year. Is it correct ? should we store just datime so as to have one soruce of true ?

```

```
