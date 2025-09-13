using System.Globalization;

namespace QuantLibCSharp;

using SerialType = Int32;
using Day = Int32;
using Year = Int32;

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
    private static readonly int[] YEAR_OFFSET = [
            // 1900–1909
            0, 366, 731, 1096, 1461, 1827, 2192, 2557, 2922, 3288,
            // 1910–1919
            3653, 4018, 4383, 4749, 5114, 5479, 5844, 6210, 6575, 6940,
            // 1920–1929
            7305, 7671, 8036, 8401, 8766, 9132, 9497, 9862, 10227, 10593,
            // 1930–1939
            10958, 11323, 11688, 12054, 12419, 12784, 13149, 13515, 13880, 14245,
            // 1940–1949
            14610, 14976, 15341, 15706, 16071, 16437, 16802, 17167, 17532, 17898,
            // 1950–1959
            18263, 18628, 18993, 19359, 19724, 20089, 20454, 20820, 21185, 21550,
            // 1960–1969
            21915, 22281, 22646, 23011, 23376, 23742, 24107, 24472, 24837, 25203,
            // 1970–1979
            25568, 25933, 26298, 26664, 27029, 27394, 27759, 28125, 28490, 28855,
            // 1980–1989
            29220, 29586, 29951, 30316, 30681, 31047, 31412, 31777, 32142, 32508,
            // 1990–1999
            32873, 33238, 33603, 33969, 34334, 34699, 35064, 35430, 35795, 36160,
            // 2000–2009
            36525, 36891, 37256, 37621, 37986, 38352, 38717, 39082, 39447, 39813,
            // 2010–2019
            40178, 40543, 40908, 41274, 41639, 42004, 42369, 42735, 43100, 43465,
            // 2020–2029
            43830, 44196, 44561, 44926, 45291, 45657, 46022, 46387, 46752, 47118,
            // 2030–2039
            47483, 47848, 48213, 48579, 48944, 49309, 49674, 50040, 50405, 50770,
            // 2040–2049
            51135, 51501, 51866, 52231, 52596, 52962, 53327, 53692, 54057, 54423,
            // 2050–2059
            54788, 55153, 55518, 55884, 56249, 56614, 56979, 57345, 57710, 58075,
            // 2060–2069
            58440, 58806, 59171, 59536, 59901, 60267, 60632, 60997, 61362, 61728,
            // 2070–2079
            62093, 62458, 62823, 63189, 63554, 63919, 64284, 64650, 65015, 65380,
            // 2080–2089
            65745, 66111, 66476, 66841, 67206, 67572, 67937, 68302, 68667, 69033,
            // 2090–2099
            69398, 69763, 70128, 70494, 70859, 71224, 71589, 71955, 72320, 72685,
            // 2100–2109
            73050, 73415, 73780, 74145, 74510, 74876, 75241, 75606, 75971, 76337,
            // 2110–2119
            76702, 77067, 77432, 77798, 78163, 78528, 78893, 79259, 79624, 79989,
            // 2120–2129
            80354, 80720, 81085, 81450, 81815, 82181, 82546, 82911, 83276, 83642,
            // 2130–2139
            84007, 84372, 84737, 85103, 85468, 85833, 86198, 86564, 86929, 87294,
            // 2140–2149
            87659, 88025, 88390, 88755, 89120, 89486, 89851, 90216, 90581, 90947,
            // 2150–2159
            91312, 91677, 92042, 92408, 92773, 93138, 93503, 93869, 94234, 94599,
            // 2160–2169
            94964, 95330, 95695, 96060, 96425, 96791, 97156, 97521, 97886, 98252,
            // 2170–2179
            98617, 98982, 99347, 99713, 100078, 100443, 100808, 101174, 101539, 101904,
            // 2180–2189
            102269, 102635, 103000, 103365, 103730, 104096, 104461, 104826, 105191, 105557,
            // 2190–2199
            105922, 106287, 106652, 107018, 107383, 107748, 108113, 108479, 108844, 109209,
            // 2200
            109574,
        ];
    private static readonly int[] MONTH_OFFSET = [
        0, 31, 59, 90, 120, 151, // Jan - Jun
        181, 212, 243, 273, 304, 334, // Jul - Dec
        365, // For fallback case
    ];
    private static readonly int[] MONTH_LEAP_OFFSET = [
        0, 31, 60, 91, 121, 152, // Jan - Jun
        182, 213, 244, 274, 305, 335, // Jul - Dec
        366, // For fallback case
    ];
    private static readonly int[] MONTH_LENGTH = [
        31, 28, 31, 30, 31, 30, // Jan - Jun
        31, 31, 30, 31, 30, 31 // Jul - Dec
    ];
    private static readonly int[] MONTH_LEAP_LENGTH = [
        31, 29, 31, 30, 31, 30, // Jan - Jun
        31, 31, 30, 31, 30, 31 // Jul - Dec
    ];
    private static readonly Date _minDate = new(MIN_SERIAL); //Equivalent to chashing
    private static readonly Date _maxDate = new(MAX_SERIAL); //Equivalent to chashing
    private const int MIN_SERIAL = 367; // 1901-01-01
    private const int MAX_SERIAL = 109574; // 2199-12-31


    // Contructor
    public Date() { _serialNumber = 0; }
    public Date(int day, Month month, int year)
    {
        // Year check
        if (year < 1901 || year > 2199)
            throw new ArgumentOutOfRangeException(nameof(year), "Year out of range [1901,2199]");

        // Month check
        if ((int)month < 1 || (int)month > 12)
            throw new ArgumentOutOfRangeException(nameof(month), "Month out of January-December range i.e. not in [1,12]");

        // Leap year
        bool isLeap = IsLeap(year);

        // Month length & offset
        int monthLength = MonthLength(month, isLeap);
        int monthOffset = MonthOffSet(month, isLeap);
        int yearOffset = YearOffSet(year);

        // Day check
        if (day < 1 || day > monthLength)
            throw new ArgumentOutOfRangeException(nameof(day), day,
            $"""
            Day {day} outside month {month}
            day range [1..{monthLength}]
            """
            );

        // Serial number
        SerialType serialNumber = day + monthOffset + yearOffset;

        // Check serial number
        CheckSerialNumber(serialNumber);
        _serialNumber = serialNumber;
    }
    public Date(SerialType serialNumber)
    {
        CheckSerialNumber(serialNumber);
        _serialNumber = serialNumber;
    }


    // Static Helpers (Private)
    static int MonthOffSet(Month month, bool isLeap)
    {
        // No check here coz month could be 13
        return isLeap ? MONTH_LEAP_OFFSET[(int)month - 1] : MONTH_OFFSET[(int)month - 1];
    }
    static int MonthLength(Month month, bool isLeap)
    {
        return isLeap ? MONTH_LEAP_LENGTH[(int)month - 1] : MONTH_LENGTH[(int)month - 1];
    }
    static bool IsLeap(int year)
    {
        if (year < 1900 || year > 2200)
            throw new ArgumentOutOfRangeException(nameof(year), "Year out of range [1900,2200]");
        return YEAR_IS_LEAP[year - 1900];
    }
    static int YearOffSet(int year)
    {
        if (year < 1900 || year > 2200)
            throw new ArgumentOutOfRangeException(nameof(year), "Year out of range [1900,2200]");

        return YEAR_OFFSET[year - 1900];
    }
    static int YearLength(int year)
    {
        return IsLeap(year) ? 366 : 365;
    }
    static void CheckSerialNumber(SerialType serialNumber)
    {
        if (serialNumber < MIN_SERIAL || serialNumber > MAX_SERIAL)
        {
            throw new ArgumentOutOfRangeException(nameof(serialNumber), serialNumber,
            $"""
            Serial {serialNumber} out of bounds,
            must be [{MIN_SERIAL}..{MAX_SERIAL}]
            or in dates [{MinDate()} - {MaxDate()}]
            """
            );
        }
    }


    // Static Helpers (Public)
    public static Date MinDate() => _minDate;
    public static Date MaxDate() => _maxDate;
    public static Date TodaysDate()
    {
        DateTime todayDate = DateTime.Today;
        return new Date(todayDate.Day, (Month)todayDate.Month, todayDate.Year);
    }


    // Inspectors (Public) 
    public Day DayOfMonth()
    {
        return _serialNumber - YearOffSet(Year()) - MonthOffSet(Month(), IsLeap(Year()));
    }
    public Day DayOfYear()
    {
        return _serialNumber - YearOffSet(Year());
    }
    public Month Month()
    {
        Day dayOfYear = DayOfYear();
        Year year = Year();
        bool isLeap = IsLeap(year);

        // Guess
        int month = dayOfYear / 30 + 1;

        while (dayOfYear <= MonthOffSet((Month)month, isLeap))
        {
            month -= 1;
        }
        while (dayOfYear > MonthOffSet((Month)(month + 1), isLeap))
        {
            month += 1;
        }

        return (Month)month;
    }
    public Year Year()
    {
        // Guess
        Year year = _serialNumber / 365 + 1900;

        // Adjust
        if (_serialNumber <= YearOffSet(year)) year -= 1;

        return year;
    }
    public Day Day()
    {
        return DayOfMonth();
    }
    public SerialType SerialNumber()
    {
        return _serialNumber;
    }
    public bool IsEndOfMonth()
    {
        return Day() == MonthLength(Month(), IsLeap(Year()));
    }
    public Date EndOfMonth()
    {
        return new Date(MonthLength(Month(), IsLeap(Year())), Month(), Year());
    }

}








/* 
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
*/