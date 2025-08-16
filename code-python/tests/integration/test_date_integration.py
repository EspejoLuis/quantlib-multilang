import pytest

from quantlib_python.date import Date


def test_add_sub() -> None:
    date_created: Date = Date(day=14, month=5, year=1989) + 15 - 13
    date_expected: Date = Date(day=16, month=5, year=1989)
    assert date_created == date_expected


def test_chain_operations() -> None:
    starting_date: Date = Date(day=14, month=5, year=1989)
    date_step_1: Date = starting_date + 17
    date_step_2: Date = date_step_1 - 13
    date_created: Date = date_step_2 + 3
    date_expected: Date = Date(day=21, month=5, year=1989)
    assert date_created == date_expected


def test_delta_days() -> None:
    d1: Date = Date(day=14, month=5, year=1989)
    d2: Date = Date(day=29, month=5, year=1989)
    delta_days: int = d2 - d1
    date_created: Date = d1 + delta_days
    assert date_created == d2


def test_leap_years() -> None:
    d1: Date = Date(day=15, month=2, year=2024) + 14
    d2: Date = Date(day=15, month=2, year=2025) + 14
    date_created_leap_year: Date = Date(day=29, month=2, year=2024)
    date_created_no_leap_year: Date = Date(day=1, month=3, year=2025)

    assert d1 == date_created_leap_year
    assert d2 == date_created_no_leap_year


def test_crossing_years() -> None:
    d1: Date = Date(day=31, month=12, year=2024)
    d2: Date = d1 + 2
    d3: Date = d2 - 1
    date_expected: Date = Date(day=1, month=1, year=2025)

    assert d3 == date_expected
    assert str(object=d3) == "01-Jan-2025"


def test_integration_error() -> None:
    d1: Date = Date(day=29, month=2, year=2024) + 365
    with pytest.raises(expected_exception=TypeError) as exc_info:
        d1 + "Not Integer"

    assert str(object=exc_info.value) == "Addition not implemented for type str"
