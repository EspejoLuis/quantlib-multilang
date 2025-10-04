# July

## 20 July 2025 - Day One: setting environements and structure folder

- Setting Environments:

  - Quantlib 1-38 already installed in the past so we are good. It was a lengthy and somehow complex process but got it working.

- Using ChatGpt to help me with suggestiongs about structure of folders. We can have one single repo and three folders. One for each language. Then later if needed we can extract folders:

  - Python folder --> [`code-python`](../code-python)
  - C# folder--> [`code-csharp`](../code-csharp)
  - c++ folder --> [`code-cpp`](../code-ccp)

- Had to do some fixing:
  - git remote remove origin
  - then just using source control bar --> publish Branch

## 21 July 2025 - Day Two: setting python and C# environment

- Python:
  - `uv venv --python=/opt/homebrew/opt/python@3.13/bin/python3.13 .venv` to create the uv environemnt with python 3.11
  - Activate environment `source .venv/bin/activate`
  - `uv pip install -U pip setuptools wheel` to install some basic libraries
  - Create the pyproject.toml with polars version.
  - `uv pip install -r pyproject` to install the content in pyproject.
  - Added ruff extension to visual studio. Check in extensions.
  - Create ruff.toml setting file.
  - Create setting.json in .vscode for using ruff.
  - Select the python interpreter : cmd+shift+P --> .venv should appear. If not the add it manually using the address found in `which python`
- C#:
  - `dotnet new classlib --name QuantLibCSharp --output .` to set up the C# environment
  - Build it `dotnet build` from [`QuantLibCSharp`](../code-csharp/QuantLibCSharp/)

## 22 July 2025 - Day Three: setting C++ environment

- C++:

  - In [`code-cpp`](../code-cpp/) folder:
    - `mkdir src`
    - `touch src/main.cpp`
    - `touch CMakeLists.txt`
  - Add specs in CMakeLists.txt and example in main.ccp to test
  - Then:
    - `mkdir build`
    - `cd build`
    - `cmake ..` search for a CMakeLists file in the parent folder `code-ccp`, find the source files to compile `src/main.cpp`, set up Makefile in `build` folder.
    - `make` :
      - Compile `main.ccp` into `main.ccp.o`(object file in `CMakeFiles/quantlib_ccp.dir/src/`), `main.ccp.o.d` is created as well.
        - `main.cpp.o` -> compiled binary blob.
        - `main.cpp.o.d` -> dependency file for header tracking.
      - Link all objects into final binary `quantlib_ccp`(in `build/`).
    - `./quantlib_cpp` execute! (from `build/`)

- Implemenation class `Date`:
  - C++:
    - Defining `Date.hpp` and `Date.cpp`. To Continue
      - Construct valid dates
      - Print them
      - Compare them
      - Add/subtract days

## 23 July 2025 - Day #4: C++ Date Implementation

- In [`QuantLibCpp`](../code-cpp/src/QuantLibCpp/):
  - Implemenation class `Date`:
    - C++:
      - Continued defining `Date.hpp` and `Date.cpp`.
        - Defined comparison,add/subtract.
        - Comment methods to better understand what they are doing.
      - Compiled and created a pseudo test `DateExample.cpp`.
        - g++ -std=c++17 Date.cpp DateExample.cpp -o test_date
        - ./test_date

## 24 July 2025 - Day #5: Idea + C# Date Implementation

Had crazy idea to add also Rust. Maybe it is too much ? I'll do it anyway: - Install it: `curl https://sh.rustup.rs -sSf | sh` - Restart terminal and `source $HOME/.cargo/env` - Create folder: `cargo new code-rust --lib`

- In [`QuantLibCpp`](../code-cpp/src/QuantLibCpp/):

  - Implemenation proper test for class `Date`:
    - Going to use Catch2 as test framework
    - Create [`lib`](../code-cpp/src/QuantLibCpp/lib/)
    - Installed `curl -LO https://raw.githubusercontent.com/catchorg/Catch2/v2.x/single_include/catch2/catch.hpp` i.e. catch.hpp is t’s the entire Catch2 testing framework bundled into a single header file. From now on just need to use `#include "catch.hpp"` and start writing tests.
    - Create `DateTest_catch.cpp`
    - Manually compiling from [`QuantLibCpp`](../code-cpp/src/QuantLibCpp/):
      - Compile `g++ -std=c++17 Date.cpp DateTest_catch.cpp -o date_tests` --> Compile Date and DateTest_catch and link them into an executable date_tests
      - Run `./date_tests` --> All test passed 19 assertions in 7 test cases
    - Compiling using cmake from [`build`](../code-cpp/build/):
      - `cmake ..` meaning:
        - "Please read the CMakeLists.txt file in the parent directory (..) and generate all the build files needed (e.g., Makefile or Ninja files) here in the build/ directory"
        - After, in build there should be `Makefile`
      - `cmake --build .` uses `Makefile` and compiles

- In [`QuantLibCShapr`](../code-csharp/QuantLibCSharp/):
  - Implementation for `Date` class

## 25 July 2025 - Day #6: C# Date Test Implementation

- In [`code-csharp`](../code-csharp/):
  - Implementation of test for [`DateTests.cs`](../code-csharp/tests/QuantLibCSharp.Tests/DateTests.cs):
    - Create new test folder: `dotnet new nunit -n QuantlibCSharp.Tests`
    - Add "ProjectReference Include=" in csproj `dotnet add QuantLibCSharp.Tests/QuantLibCSharp.Tests.csproj reference QuantLibCSharp/QuantLibCSharp.csproj`
  - Creating solutions `dotnet new sln -n QuantLibCSharp`:
    - Register QuantLibCSharp with the solutuion: `dotnet sln add QuantLibCSharp/QuantLibCSharp.csproj`
    - Register QuantLibCSharp.Tests with the solution: `dotnet sln add QuantLibCSharp.Tests/QuantLibCSharp.Tests.csproj`
    - Test --> `dotnet test QuantLibCSharp.sln` from [`code-csharp`](../code-csharp/)

## 26 July 2025 - Day #7: Implementation of Date class in Rust

- In [`code-rust`](../code-rust/):
  - Started implementing Date class: [`date.rs`](../code-rust/src/date.rs)
    - Constructor
    - ToString
    - Date comparison:
      - Equal
      - Lower
    - Operators:
      - To use Add, to_serial and from_serial are needed
      - Add:
        - Add positive number
        - Add negative number
      - Subtract:
        - Subtract positive number
        - Subtract negative number
        - Subtract dates
  - Started implementing Date Test class: [`date.rs`](../code-rust/src/date.rs):
    - Unit tests in [`date.rs`](../code-rust/src/date.rs):
      - Constructor -> Done --> `cargo test` --> this compiles and runs all test targets

## 27 July 2025 - Day #8: Implementation of Test Date class in Rust

- In [`code-rust`](../code-rust/):
  - Started implementing Date Test class: [`date.rs`](../code-rust/src/date.rs):
    - Unit tests:
      - toString
      - Equality (==)
      - Comparison (> or <)
      - Operators:
        - fromSerial, toSerial
        - Add:
          - days
        - Sub:
          - days
          - Dates
      - Install `cargo install cargo-tarpaulin` to get tarpauling which helps to automatically check the coverage of tests and the `cargo tarpaulin --tests` will run tests, show which lines were executed and % coverage
    - In rust unit tests live next to the code, while integration tests have their own specific folder
    - Integration Test in folder [`tests`](../code-rust/tests/)
      - `cargo test --test date_integration` to run only integration tests
      - `cargo test` to test both unit and integration tests
      - serial_conversion_works_correctly()
      - add_then_subtract_returns_original_date_correctly():
        - This created some issues because I was using a variable that was already moved. Need to implement Add\<i32> for &Date instead of just for Date --> To support borrowing and gives me the possibility of reusing the variable.
      - Added also unit test:
        - subtract_days_by_reference_works_correctly()
        - subtract_days_value_works_correctly()
        - add_days_by_reference_works_correctly()
        - add_days_by_reference_works_correctly()

## 28 July 2025 - Day #9: Implementation of Date class in Python

- Manually select the .venv :
  - `/Users/apple/github_repos/quantlib-multilang/code-python/.venv/bin/python` using cmd + shift + p
  - This will create a .vscode at the root.
  - Added the version into the pyproject.toml `uv pip install -r pyproject.toml` after having used `uv pip list` to see the version of what I'm using.
