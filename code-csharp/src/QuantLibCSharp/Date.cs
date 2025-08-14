using System.Globalization;

//Not needed anymore to do namespace QuantLibCSharp{} !
namespace QuantLibCSharp;

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
    private readonly DateOnly _dateonly;
    public int Day { get; }
    /*
    public int Day { get; }

    In C++ it woudl have been:
    private readonly int _day;

    public int Day
    {
        get { return _day; }
    }
    */
    public Month Month { get; }
    public int Year { get; }
    // Contructor
    public Date(int day, Month month, int year)
    {
        // DateOnly will validate and handle leap years
        // yyyy-MM-dd (ISO 8601 date).
        _dateonly = new DateOnly(year, (int)month, day);
        Day = _dateonly.Day;
        Month = (Month)_dateonly.Month;
        Year = _dateonly.Year;
    }
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
        // Can be used in collection for example
        // Nullable reference ? -> object could be null.
        // The check below defined what to do in case is null
        // If obj is not a Date, return false; otherwise, treat it as a Date and call it other
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
        // : IEquatable<Date> is needed to declare this.
        // Collections will used this method over Equals(object? obj)
        // Polymorphic/object APIs call your overridden Equals(object?) like object.ReferenceEquals(left, right)
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