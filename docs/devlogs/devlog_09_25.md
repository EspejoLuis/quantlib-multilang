# September

## 1 Sep 2025 - Rust:

- Checking coverage in period --> completed.
- ✅ Formatting (Display / long_period, short_period) with unit test!

## 2 Sep 2025 - Date - Rust:

- Asked ChatGPT to list the differences. Added the list below.
- `check_serial_number` added. Have to add it in ++d and --d but dont have them so have to create them.
- Increments. We are going to implement only pre-increments (they are used in `calendar.cpp` and `calendar.hpp`).

  - Pre-increment (++d):
    - Increment first, then return reference to the updated object
    - Example: int x=5; int y=++x; // x=6, y=6
    - Reference to updated object
  - Post-increment (d++):
    - Copy current value, then increment, return the old copy
    - Example: int x=5; int y=x++; // x=6, y=5
    - Copy of old object

- Important. For operators, C++ always returns a reference to the object itself for `Date&` or `Period&`. In Rust, we should do the same i.e. returning a mutable reference to the same object.
  Instead of having:

  ```rust
  sub_assign(&mut self, right_hand_side: Day) -> ()
  ```

  we should have

  ```rust
  sub_assign(&mut self, right_hand_side: Day) -> &mut self
  ```

  This could potentially allow to do a += 4 += 5 (like C++).

- Implemented `formatted_date()`. Using `String`:
  - String (owned string)
    - You own the text.
    - Stored on the heap (memory has to be manually allocated)
    - You can keep it as long as you like, change it, grow it.
    - Example --> let s: String = String::from("Hello");
    - Analogy: you bought the book → it’s yours, you can write notes, rip pages, keep it forever.
  - &str (string slice):
    - You borrow the text.
    - It’s just a view into some existing string (or literal).
    - You can’t change it, and it only lives as long as the original string.
    - Example --> let s: &str = "Hello"; // literal is &'static str
    - Analogy: you borrowed the book from a library → you can read it, but can’t keep it, and it must go back.
- If an iterator is used in the future, need to implement this :

  ```rust
  for d in start.iter_to(end) {
  // body
  }
  ```

- Implemented `default()` as trait for `Default`

## 3 Sep 2025 - Date - Rust:

- Change to lifetime `'a`:
  - In C++, const Date& means “borrow this Date, don’t copy it.”
  - In Rust, this is expressed with &'a Date: a reference tied to a lifetime 'a.
  - The impl<'a> tells Rust: this implementation works for any lifetime 'a:
  ```rust
  impl<'a> Display for IsoDate<'a> {
      fn fmt(&self, f: &mut Formatter) -> Result { ... }
  }
  ```
  - The public API must also carry the lifetime forward, ensuring the returned object cannot outlive the borrowed date:
  ```rust
  pub fn iso_date<'a>(d: &'a Date) -> impl Display + 'a {
      detail::IsoDate { date: d }
  }
  ```
  - `impl<'a> Display for IsoDate<'a>` -> implement Display for borrowed dates with lifetime 'a.
  - `impl Display + 'a ` -> the returned object implements Display and is valid only as long as the borrowed Date lives.
    This makes Rust’s API faithful to QuantLib: borrowed views of a Date instead of copying.

## 4 Sep 2025 - Thursday - Date - Rust:

- Updated `period.rs` with lifetime `'a`.
- Update `weekday.rs` with lifetime `'a`:
  ```rust
  pub(crate) struct LongWeekday<'a> {
      pub(crate) weekday: &'a Weekday,
  }
  ```
  'a does not exist at runtime. It’s not a variable, not memory, not data. It’s purely for the compiler to enforce borrowing rules. Example with a car:
  Imagine 'a is a parking permit:
  - You borrow a car (&Weekday) but the permit says you can only keep it until 'a expires.
  - LongWeekday<'a> is the parking lot that stores the borrowed car.
  - When 'a ends, you must return the car — you can’t keep using it.

### TODO:

- Should MonthIndex and WeekdayIndex go ? simply use Month and WeekDay and then cast as usize where needed ?
- Should i call length and unit with .length or .length()
- Remove some partialOrd where not needed
- ❓ Thinking about having `enum month` in a proper `month.rs`
- What about using Size (usize) instead of MonthIndex or WeekDayIndex
  Date:

  - Rust: [from ChatGPT]

    - Integration with Period and TimeUnit
      - In QuantLib, Date supports advancing by a Period (+= Period, -= Period, + Period, - Period).
      - This requires calling advance(self, n, TimeUnit) which handles days, weeks, months, and years.
      - Currently, your Rust Date only supports +/- Day and Date - Date. No handling of Period.
    - Advance Function

      - A key part of QuantLib: moves a date forward/backward by n units (Days, Weeks, Months, Years).
      - Your Rust code doesn’t yet have an equivalent.

    - Null Date Handling
      - QuantLib defines Date() as a null date (serial = 0).
      - Your Rust Date always has a valid serial number — no concept of null.
      - If you want to stay consistent with QuantLib, you should add pub fn null_date() -> Date returning serial_number = 0, and handle it in Display as "null date".
    - Hashing

      - C++ provides hash_value(const Date&).
      - In Rust, you can implement impl std::hash::Hash for Date to support use in HashMap.

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
