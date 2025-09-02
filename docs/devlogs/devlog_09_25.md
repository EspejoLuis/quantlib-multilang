# September

## 1 Sep 2025 - Rust:

- Checking coverage in period --> completed.
- ✅ Formatting (Display / long_period, short_period) with unit test!

## 2 Sep 2025 - Date - Rust:

- Asked ChatGPT to list the differences. Added the list below
- `check_serial_number` added. Have to add it in ++d and --d but dont have them so have to create them.

### TODO:

- Should i call length and unit with .length or .length()
- Remove some partialOrd where not needed
- ❓ Thinking about having `enum month` in a proper `month.rs`
- What about using Size (usize) instead of MonthIndex or WeekDayIndex
- Date:

  - Rust:

    - Integration with Period and TimeUnit
      - In QuantLib, Date supports advancing by a Period (+= Period, -= Period, + Period, - Period).
      - This requires calling advance(self, n, TimeUnit) which handles days, weeks, months, and years.
      - Currently, your Rust Date only supports +/- Day and Date - Date. No handling of Period.
    - Increment / Decrement Operators
      - C++ implements both pre/post increment (++d, d++) and pre/post decrement (--d, d--) on dates.
      - Rust can’t overload ++, but you should still expose equivalents:
      - pub fn increment(&mut self) → like ++d.
      - pub fn decrement(&mut self) → like --d.
    - advance Function

      - A key part of QuantLib: moves a date forward/backward by n units (Days, Weeks, Months, Years).
      - Your Rust code doesn’t yet have an equivalent.

    - Null Date Handling
      - QuantLib defines Date() as a null date (serial = 0).
      - Your Rust Date always has a valid serial number — no concept of null.
      - If you want to stay consistent with QuantLib, you should add pub fn null_date() -> Date returning serial_number = 0, and handle it in Display as "null date".
    - Comparison Traits Beyond <
      - In QuantLib, Date has ==, !=, <, <=, >, >=.
      - Rust derives PartialEq, Eq, PartialOrd, Ord already, so this part is fine.
    - Hashing
      - C++ provides hash_value(const Date&).
      - In Rust, you can implement impl std::hash::Hash for Date to support use in HashMap.
    - Formatting Helpers

      - You already added io::long_date, io::short_date, io::iso_date.
      - QuantLib also has io::formatted_date (custom pattern) and, in high-resolution mode, iso_datetime.
      - You may skip high-resolution for now, but formatted_date is still missing.

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
