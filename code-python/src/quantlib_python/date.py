
from pydantic import BaseModel, field_validator, model_validator
from enum import Enum
"""
 Under the hood, pydantic gives you:

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
    month: int
    year: int
     
    @field_validator('month')
    @classmethod
    def validate_month(cls, month: int) -> int:
        return cls.validate_range(month, "month", 1,12)
        
    @field_validator('year')
    @classmethod
    def validate_year(cls, year: int) -> int:
        return cls.validate_range(year, "year", 1950,2150)
        
    @staticmethod
    def validate_range(value: int, name: str,
                    low: int, high:int) -> int:
            if low <= value <= high:
                 return value
            raise ValueError(
                 f"{name} not valid: {value} is not between {low} and {high}")

    @model_validator(mode="after")
    @classmethod
    def validate_day(cls, data: "Date") -> "Date":
        
        month: int = data.month
        year: int = data.year
        day: int = data.day

        month_enum = Month(month)

        thirty_days : list[Month] = [
             Month.NOVEMBER,
             Month.APRIL,
             Month.JUNE,
             Month.SEPTEMBER
             ]

        thirty_one_days: list[Month] = [
             Month.JANUARY,
             Month.MARCH,
             Month.MAY,
             Month.JULY,
             Month.AUGUST,
             Month.OCTOBER,
             Month.DECEMBER
             ]

        if month_enum in thirty_days:
            cls.validate_range(day, "day", 1, 30)
        elif month_enum in thirty_one_days:
            cls.validate_range(day, "day", 1, 31)
        else:
             if cls.is_leap_year(year):
                 cls.validate_range(day, "day", 1, 29)
             else:
                 cls.validate_range(day, "day", 1, 28)
        return data
    
    @staticmethod
    def is_leap_year(year: int) -> bool:
         return (year % 4 ==0) and (year % 100 != 0 or year % 400 ==0)

   
if __name__ == '__main__':
    d =  Date(day="31",month="11", year="1989")
    print(d)
    