
from pydantic import BaseModel, field_validator

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

@field_validator 
class Date(BaseModel):
    day: int
    month: int
    year: int



if __name__ == '__main__':
    d =  Date(day="1",month="2", year="sjoj")
    print(d)