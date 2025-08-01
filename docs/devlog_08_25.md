# August

## 1 August 2025: Continuation Implementation of Date class in Python
- Adding validators for month/years using `field_validator()`
- Adding validator for day using `model_validator(mode=after)`:
    - Leap years and checks on number of days




# TO DO 
## Python:
    - Add __str__ and to datetime 
    - Add .to_datetime() method (if using datetime.date)
    - Implement arithmetic like __add__(self, days: int), __sub__.- Replace the thirty_*_days lists with a cleaner lookup table or use a calendar module — optional.



- Date validation to avoid 30 february for c++/c#/rust. In python should be easier
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
- Integration tests