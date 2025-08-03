import pytest

from quantlib_python.date import Date


def test_str():
    test_date = Date(day=14, month=5, year=1989)
    assert(str(test_date) == "14-May-1989")

def test_add_within_one_month():
    date_created: Date = Date(day=13, month=2, year=1960) + 15
    date_expected = Date(day=28, month=2, year=1960)
    assert(date_created == date_expected)

def test_add_across_months():
    date_created: Date = Date(day=13, month=2, year=1989) + 17
    date_expected = Date(day=2, month=3, year=1989)
    assert(date_created == date_expected)

def test_add_leap_year():
    date_created: Date = Date(day=13, month=2, year=2024) + 16
    date_expected = Date(day=29, month=2, year=2024)
    assert(date_created == date_expected)

def test_add_zero_days():
    date_created: Date = Date(day=13, month=2, year=1972) + 0
    date_expected = Date(day=13, month=2, year=1972)
    assert(date_created == date_expected) 

def test_sub_days_within_one_month():
    date_created: Date = Date(day=28, month=2, year=1976) - 15
    date_expected = Date(day=13, month=2, year=1976)
    assert(date_created == date_expected)

def test_sub_days_across_months():
    date_created: Date = Date(day=2, month=3, year=1989) - 17
    date_expected = Date(day=13, month=2, year=1989)
    assert(date_created == date_expected)

def test_sub_days_leap_year():
    date_created: Date = Date(day=29, month=2, year=2024) - 16
    date_expected = Date(day=13, month=2, year=2024)
    assert(date_created == date_expected)

def test_sub_days_zero_days():
    date_created: Date = Date(day=13, month=2, year=2024) - 0
    date_expected = Date(day=13, month=2, year=2024)
    assert(date_created == date_expected) 

def test_sub_dates():
    days_created: Date = \
        Date(day=29, month=2, year=2024) - \
        Date(day=15, month=2, year=2024)
    
    days_expected = 29 - 15
    assert(days_created == days_expected)

def test_sub_invalid_types():
    with pytest.raises(TypeError) as exc_info:
        Date(day=29, month=2, year=2024) - "not a date or int"
    
    assert(str(exc_info.value) == \
           "Subtraction not implemented for type str")
    
def test_validate_range():
    with pytest.raises(ValueError) as exc_info:
        Date(day=45, month=1, year=1989)
    ## Pydantic wraps any error into a dictionary
    error = exc_info.value.errors()[0]
    assert error["msg"] == "Value error, day not valid: 45 is not between 1 and 31"
    assert error["type"] == "value_error"

