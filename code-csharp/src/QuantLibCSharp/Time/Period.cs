namespace QuantLibCSharp.Time;

public class Period
{
    private readonly int _length;
    private readonly TimeUnit _units;


    // Constructor
    public Period(int length, TimeUnit units)
    {
        _units = units;
        _length = length;
    }
    public Period(Frequency frequency)
    {
        (_length, _units) = frequency switch
        {
            Frequency.NoFrequency => (0, TimeUnit.Days),
            Frequency.Once => (0, TimeUnit.Years),
            Frequency.Annual => (1, TimeUnit.Years),

            Frequency.Semiannual
            or Frequency.EveryFourthMonth
            or Frequency.Quarterly
            or Frequency.Bimonthly
            or Frequency.Monthly => (12 / (int)frequency, TimeUnit.Months),

            Frequency.EveryFourthWeek
            or Frequency.Biweekly
            or Frequency.Weekly => (52 / (int)frequency, TimeUnit.Weeks),

            Frequency.Daily => (1, TimeUnit.Days),
            Frequency.OtherFrequency => throw new ArgumentOutOfRangeException(
            nameof(frequency), frequency, "Unknown frequency"),

            _ => throw new ArgumentOutOfRangeException(
            nameof(frequency), frequency, "Unknown frequency")
        };
    }


    // Inspectors (Public) 
    public int Length() => _length;
    public TimeUnit Units() => _units;

}
