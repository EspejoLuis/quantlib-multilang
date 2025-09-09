using System.Globalization;
using Microsoft.VisualBasic;

namespace QuantLibCSharp;

using SerialType = System.Int32;

public enum Month
{
    January = 1,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December
}

public class Date : IEquatable<Date>
{
    // ReadOnly so it can be assigned only in the constructor
    private readonly SerialType _serialNumber;
    private static readonly bool[] YEAR_IS_LEAP = [
            // 1900 is leap in agreement with Excel's bug
            // 1900–1909
            true, false, false, false, true, false, false, false, true, false,
            // 1910–1919
            false, false, true, false, false, false, true, false, false, false,
            // 1920–1929
            true, false, false, false, true, false, false, false, true, false,
            // 1930–1939
            false, false, true, false, false, false, true, false, false, false,
            // 1940–1949
            true, false, false, false, true, false, false, false, true, false,
            // 1950–1959
            false, false, true, false, false, false, true, false, false, false,
            // 1960–1969
            true, false, false, false, true, false, false, false, true, false,
            // 1970–1979
            false, false, true, false, false, false, true, false, false, false,
            // 1980–1989
            true, false, false, false, true, false, false, false, true, false,
            // 1990–1999
            false, false, true, false, false, false, true, false, false, false,
            // 2000–2009
            true, false, false, false, true, false, false, false, true, false,
            // 2010–2019
            false, false, true, false, false, false, true, false, false, false,
            // 2020–2029
            true, false, false, false, true, false, false, false, true, false,
            // 2030–2039
            false, false, true, false, false, false, true, false, false, false,
            // 2040–2049
            true, false, false, false, true, false, false, false, true, false,
            // 2050–2059
            false, false, true, false, false, false, true, false, false, false,
            // 2060–2069
            true, false, false, false, true, false, false, false, true, false,
            // 2070–2079
            false, false, true, false, false, false, true, false, false, false,
            // 2080–2089
            true, false, false, false, true, false, false, false, true, false,
            // 2090–2099
            false, false, true, false, false, false, true, false, false, false,
            // 2100–2109
            false, false, true, false, false, false, true, false, false, false,
            // 2110–2119
            false, false, true, false, false, false, true, false, false, false,
            // 2120–2129
            true, false, false, false, true, false, false, false, true, false,
            // 2130–2139
            false, false, true, false, false, false, true, false, false, false,
            // 2140–2149
            true, false, false, false, true, false, false, false, true, false,
            // 2150–2159
            false, false, true, false, false, false, true, false, false, false,
            // 2160–2169
            true, false, false, false, true, false, false, false, true, false,
            // 2170–2179
            false, false, true, false, false, false, true, false, false, false,
            // 2180–2189
            true, false, false, false, true, false, false, false, true, false,
            // 2190–2199
            false, false, true, false, false, false, true, false, false, false, // 2200
            false];
    private static readonly int[] MONTH_OFFSET = [
            0, 0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365,
        ];
    private static readonly int[] MONTH_LEAP_OFFSET = [
        0, 0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366,
    ];

    public Date() { _serialNumber = 0; }
    // Contructor
    public Date(int day, Month month, int year)
    {
        // Converts to serial
    }

    public Date(SerialType serial)
    {
        // Date from serial number
    }

    // Helpers
    static bool IsLeap(int year)
    {
        if (year < 1901 || year > 2199)
            throw new ArgumentOutOfRangeException(nameof(year), "Year out of range [1901,2199]");
        return YEAR_IS_LEAP[year - 1900];
    }
    static int MonthOffSet(Month month, bool isLeap) { }
    static int MonthLength(Month month, bool isLeap) { }

    // Operators
    public static Date operator +(Date date, int days)
    {
        // AddDays(...) returns a new DateOnly,it doesn’t modify the original.
        // _dateonly field stays the same, the local created is just a copy with the shifted date.
        // _dateonly field is readonly, so it cannot be reassigned.
        DateOnly newDate = date._dateonly.AddDays(days);
        return new Date(newDate.Day, (Month)newDate.Month, newDate.Year);
    }
    public static Date operator -(Date date, int days)
    {
        // No new needed here.
        return date + (-days);
    }
    public static int operator -(Date left, Date right)
    {
        return left._dateonly.DayNumber - right._dateonly.DayNumber;
    }
    public override string ToString()
    {
        /*
        Overriding the toString() from System.Object 
        In System.Object toString() is virtual:
            It can be overridden in a derived class to provide a different implementation!
        */
        // If not InvariantCulture, it could print
        // if culture is (it-IT), dd-MMM-yyyy could print 01-gen-2025.
        return _dateonly.ToString("dd-MMM-yyyy", CultureInfo.InvariantCulture);
    }
    public static int DaysInMonth(Month month, int year)
    {
        // Can be used for EndOfMonth/IsEndOfMonth   
        return DateTime.DaysInMonth(year, (int)month);
    }
    public static bool operator ==(Date? left, Date? right)
    {
        // In C#, all operator overloads must be static — by language design
        // They are not tied to an instance
        //“Are these two variables pointing to the exact same object instance?”
        if (object.ReferenceEquals(left, right))
        {
            return true;
        }
        ;

        if (left is null || right is null)
            return false;

        return left.Equals(right);
    }
    public static bool operator !=(Date? left, Date? right)
    {
        return !(left == right);
    }
    public override bool Equals(object? obj)
    {
        /*
        -------------------------------------------------------------------------------------------------
        Can be used in collection for example.
        Polymorphic/object APIs call overridden Equals(object?) like object.ReferenceEquals(left, right)
        -------------------------------------------------------------------------------------------------
        Nullable reference ? -> object could be null
        The check below defined what to do in case is null
        If obj is not a Date, return false; otherwise, treat it as a Date and call it other
        */
        if (obj is Date other)
        {
            return _dateonly.Equals(other._dateonly);
        }
        else
        {
            return false;
        }
    }
    public bool Equals(Date? date)
    {
        /*
        --------------------------------------------------------------------------------------------- 
        Without This:
        Two Dates with the same serial number but created separately would not be equal by default.
        ---------------------------------------------------------------------------------------------
        - IEquatable<Date> is needed to declare this.
        - Collections will used this method over Equals(object? obj)
        - By implementing IEquatable<Date>, you give generic collections like 
          List<Date>, HashSet<Date>, Dictionary<Date, …> a strongly-typed equality method
        - This will make such generic collections faster because they will not need to cast from
          object from Equals(object? obj)
        */
        if (date is null)
            return false;
        return _dateonly.DayNumber == date._dateonly.DayNumber;
    }
    public override int GetHashCode()
    {
        /*
        If Equals() is overriden, also GetHashCode() has to be overriden so that:
            if (a.Equals(b)) → then a.GetHashCode() == b.GetHashCode()
        Otherwise, your Date objects will behave inconsistently in hash collections.
        */
        return _dateonly.GetHashCode();
    }
    public static bool operator <(Date left, Date right)
    {
        return left._dateonly.DayNumber < right._dateonly.DayNumber;
    }
    public static bool operator >(Date left, Date right)
    {
        return !(left < right || left == right);
    }
}