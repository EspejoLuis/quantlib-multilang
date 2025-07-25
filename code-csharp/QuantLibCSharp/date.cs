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

public class Date
{
    /*
    In C++ style it would have been:
        private readonly int day_;
    
    and then
        public int Day() => day_; 
     */
    public int Day { get; }
    /*
    Equivalent to:

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
        Day = day;
        Month = month;
        Year = year;
    }

    /*
    Overriding the toString() from System.Object 
    In System.Object toString() is virtual:
        It can be overridden in a derived class to provide a different implementation!
    */
    public override string ToString()
    {
        // ISO format (different from the C++ implementation)
        // Casting Month to int i.e. the corresponding index of the month
        // D4 --> 4 Digits
        return $"{Year:D4}-{(int)Month:D2}-{Day:D2}";
    }

    // Nullable reference ? -> object could be null.
    // The check below defined what to do in case is null
    public override bool Equals(object? obj)
    {
        //If obj is not a Date, return false; otherwise, treat it as a Date and call it other
        if (obj is not Date other)
            return false;

        return Day == other.Day && Month == other.Month && Year == other.Year;
    }

    /*
    If Equals() is overriden, also GetHashCode() has to be overriden so that:
        if (a.Equals(b)) → then a.GetHashCode() == b.GetHashCode()
    Otherwise, your Date objects will behave inconsistently in hash collections.
    */
    public override int GetHashCode()
    {
        return HashCode.Combine(Day, Month, Year);
    }

    // In C#, all operator overloads must be static — by language design
    // They are not tied to an instance
    public static bool operator ==(Date left, Date right)
    {
        return left.Equals(right);
    }

    public static bool operator !=(Date left, Date right)
    {
        return !(left == right);
    }

    public static bool operator <(Date left, Date right)
    {
        if (left.Year != right.Year)
            return left.Year < right.Year;
        if (left.Month != right.Month)
            return left.Month < right.Month;
        return left.Day < right.Day;
    }

    public static bool operator >(Date left, Date right)
    {
        return !(left < right || left == right);
    }

    public static Date operator +(Date d, int days)
    {
        return new Date(d.Day + days, d.Month, d.Year);
    }

    public static Date operator -(Date d, int days)
    {
        return new Date(d.Day - days, d.Month, d.Year);
    }

}
