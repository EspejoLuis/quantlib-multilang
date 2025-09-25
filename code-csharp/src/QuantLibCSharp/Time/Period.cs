using System;
using System.Reflection.Metadata.Ecma335;
namespace QuantLibCSharp.Time;

public class Period
{
    private int _length;
    private TimeUnit _units;


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
    public Frequency ToFrequency()
    {
        // Period -> Frequency
        int abs_length = Math.Abs(_length);
        TimeUnit units = _units;

        if (abs_length == 0)
        {
            return (units == TimeUnit.Years)
            ? Frequency.Once
            : Frequency.NoFrequency;
        }

        return units switch
        {
            TimeUnit.Years => abs_length == 1
                ? Frequency.Annual
                : Frequency.OtherFrequency,
            TimeUnit.Months => (abs_length <= 12 && 12 % abs_length == 0)
                ? FrequencyUtils.FromNthTimesPerYear(12 / abs_length)
                : Frequency.OtherFrequency,
            TimeUnit.Weeks => abs_length switch
            {
                1 => Frequency.Weekly,
                2 => Frequency.Biweekly,
                4 => Frequency.EveryFourthWeek,
                _ => Frequency.OtherFrequency,
            },
            TimeUnit.Days => abs_length == 1
                ? Frequency.Daily
                : Frequency.OtherFrequency,
            _ => throw new NotImplementedException($"TimeUnit {units} not implemented"),
        };
    }

    public void Normalize()
    {
        if (_length == 0) { _units = TimeUnit.Days; }
        else
        {
            switch (_units)
            {
                case TimeUnit.Months:
                    if ((_length % 12) == 0)
                    {
                        _length /= 12;
                        _units = TimeUnit.Years;
                    }
                    break;
                case TimeUnit.Days:
                    if ((_length % 7) == 0)
                    {
                        _length /= 7;
                        _units = TimeUnit.Weeks;
                    }
                    break;
                case TimeUnit.Weeks or TimeUnit.Years:
                    break;
            }
        }
    }

    public Period Normalized()
    {
        Period CopyPeriod = new(_length, _units); ;
        CopyPeriod.Normalize();
        return CopyPeriod;
    }


}


