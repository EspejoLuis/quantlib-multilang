using System;
namespace QuantLibCSharp.Time;

public class Period
{
    // Variables
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
                case TimeUnit.Weeks:
                case TimeUnit.Years:
                    break;
                default:
                    throw new ArgumentOutOfRangeException(nameof(_units), _units, "Unknown time units");
            }
        }
    }
    public Period Normalized()
    {
        Period CopyPeriod = new(_length, _units); ;
        CopyPeriod.Normalize();
        return CopyPeriod;
    }
    public double Years()
    {
        if (_length == 0) { return 0.0; }

        return _units switch
        {
            TimeUnit.Days => throw new NotImplementedException($"Cannot convert {_units} into Years"),
            TimeUnit.Weeks => throw new NotImplementedException($"Cannot convert {_units} into Years"),
            TimeUnit.Months => _length / 12.0,
            TimeUnit.Years => _length,
            _ => throw new ArgumentOutOfRangeException(nameof(_units), _units, "Unknown time units"),
        };
    }
    public double Months()
    {
        if (_length == 0) { return 0.0; }

        return _units switch
        {
            TimeUnit.Days => throw new NotImplementedException($"Cannot convert {_units} into Months"),
            TimeUnit.Weeks => throw new NotImplementedException($"Cannot convert {_units} into Months"),
            TimeUnit.Months => _length,
            TimeUnit.Years => _length * 12.0,
            _ => throw new ArgumentOutOfRangeException(nameof(_units), _units, "Unknown time units"),
        };
    }
    public double Weeks()
    {
        if (_length == 0) { return 0.0; }

        return _units switch
        {
            TimeUnit.Days => _length / 7.0,
            TimeUnit.Weeks => _length,
            TimeUnit.Months => throw new NotImplementedException($"Cannot convert {_units} into Weeks"),
            TimeUnit.Years => throw new NotImplementedException($"Cannot convert {_units} into Weeks"),
            _ => throw new ArgumentOutOfRangeException(nameof(_units), _units, "Unknown time units"),
        };
    }
    public double Days()
    {
        if (_length == 0) { return 0.0; }

        return _units switch
        {
            TimeUnit.Days => _length,
            TimeUnit.Weeks => _length * 7.0,
            TimeUnit.Months => throw new NotImplementedException($"Cannot convert {_units} into Days"),
            TimeUnit.Years => throw new NotImplementedException($"Cannot convert {_units} into Days"),
            _ => throw new ArgumentOutOfRangeException(nameof(_units), _units, "Unknown time units"),
        };
    }


    // Operators
    // Arithmetic 
    public static Period operator +(Period lhs, Period rhs)
    {
        // Asssumption: use rhs as base i.e. if different time
        // units, rhs units is used as reference
        int length = lhs._length;
        TimeUnit units = lhs._units;

        if (length == 0)
        {
            // If zero, then the length is determine by rhs
            // We dont care about lhs units because
            // zero weeks,days,months,years are just zero
            length = rhs._length;
            units = rhs._units;
        }
        else if (units == rhs._units)
        {
            // Same units
            length += rhs._length;
        }
        else
        {
            switch (units)
            {
                case TimeUnit.Years:
                    switch (rhs._units)
                    {
                        case TimeUnit.Months:
                            units = rhs._units;
                            length = length * 12 + rhs._length;
                            break;
                        case TimeUnit.Weeks:
                        case TimeUnit.Days:
                            throw new ArgumentException(
                                        $"Impossible addition between {lhs} and {rhs}");
                        default:
                            throw new ArgumentOutOfRangeException(
                                nameof(rhs), rhs._units, "Unknown time units");
                    }
                    break;
                case TimeUnit.Months:
                    switch (rhs._units)
                    {
                        case TimeUnit.Years:
                            units = rhs._units;
                            length += rhs._length * 12;
                            break;
                        case TimeUnit.Weeks:
                        case TimeUnit.Days:
                            throw new ArgumentException(
                                        $"Impossible addition between {lhs} and {rhs}");
                        default:
                            throw new ArgumentOutOfRangeException(
                                nameof(rhs), rhs._units, "Unknown time units");
                    }
                    break;
                case TimeUnit.Weeks:
                    switch (rhs._units)
                    {
                        case TimeUnit.Days:
                            units = rhs._units;
                            length = length * 7 + rhs._length;
                            break;
                        case TimeUnit.Years:
                        case TimeUnit.Months:
                            throw new ArgumentException(
                                        $"Impossible addition between {lhs} and {rhs}");
                        default:
                            throw new ArgumentOutOfRangeException(
                                nameof(rhs), rhs._units, "Unknown time units");
                    }
                    break;
                case TimeUnit.Days:
                    switch (rhs._units)
                    {
                        case TimeUnit.Weeks:
                            units = rhs._units;
                            length += rhs._length * 7;
                            break;
                        case TimeUnit.Years:
                        case TimeUnit.Months:
                            throw new ArgumentException(
                                        $"Impossible addition between {lhs} and {rhs}");
                        default:
                            throw new ArgumentOutOfRangeException(
                                nameof(rhs), rhs._units, "Unknown time units");
                    }
                    break;
                default:
                    throw new ArgumentOutOfRangeException(
                                    nameof(rhs), rhs._units, "Unknown time units");
            }
        }
        return new Period(length, units);
    }
    public static Period operator -(Period period)
    {
        return new Period(-period._length, period._units);
    }
    // Comparison

}


