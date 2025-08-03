
from calendar import monthrange
from datetime import date, timedelta
from enum import Enum
from typing import Union

from pydantic import BaseModel, field_validator, model_validator

"""
 Pydantic gives:

 - Type enforcement:
 If someone tries to pass "hello" as the month, 
 you get a nice error.

 - Automatic parsing/coercion:
 It turns "2" into 2, "2024" into 2024, etc

 - Built-in functions:
 You can convert to dict or JSON effortlessly
 great for logging, debugging, API responses.

 So much better that this
 class Date:
    def __init__(self, day, month, year):
        self.day = day
        self.month = month
        self.year = year
"""

class Month(Enum):
    JANUARY = 1
    FEBRUARY = 2
    MARCH = 3
    APRIL = 4
    MAY = 5
    JUNE = 6
    JULY = 7
    AUGUST = 8
    SEPTEMBER = 9
    OCTOBER = 10
    NOVEMBER = 11
    DECEMBER = 12

class Date(BaseModel):
    day: int
    # Using Month in padantic is cool, automatically it does
    # the autocercion
    # Date(day=1, month=1, year=2025)                # ✅ auto: Month.JANUARY
    # Date(day=1, month="JANUARY", year=2025)        # ✅ auto: Month.JANUARY
    # Date(day=1, month=Month.JANUARY, year=2025)    # ✅ already correct
    month: Month
    year: int
           
    @field_validator('year')
    @classmethod
    def validate_year(cls, year: int) -> int:
        return cls.validate_range(year, "year", 1950,2150)
        
    @staticmethod
    def validate_range(value: int, 
                       name: str,
                       low: int, 
                       high:int) -> int:
            
            if low <= value <= high:
                 return value
            raise ValueError(
                 f"{name} not valid: {value} is not between {low} and {high}")

    @model_validator(mode="after")
    @classmethod
    def validate_day(cls, data: "Date") -> "Date":

        _, days_in_month = \
            monthrange(year=data.year, month=data.month.value)
        cls.validate_range(data.day, "day", 1, days_in_month)
        return data
        
    def to_datetime(self) -> date:
        return date(year=self.year,
                    month=self.month.value,
                    day=self.day)

    def __str__(self) -> str:
         return self.to_datetime().strftime("%d-%b-%Y")
    
    def __add__(self, other:int) -> "Date":
        new_date: date = self.to_datetime() + timedelta(days=other)
        return Date(day=new_date.day, 
                    month=new_date.month,
                    year=new_date.year)

    def __sub__(self, other:Union[int,"Date"]) -> Union["Date", int]:
        if isinstance(other, Date):
            delta_days: timedelta = self.to_datetime() - other.to_datetime()
            return delta_days.days
        elif isinstance(other,int):
            new_date: date = self.to_datetime() - timedelta(days=other)
            return Date(day=new_date.day,
                        month=new_date.month,
                        year=new_date.year)
        else:
             raise TypeError(f"Subtraction not implemented for type {type(other).__name__}")
    

    