namespace QuantLibCSharp.Time.Detail;

internal static class PeriodFormatter
{
    internal static string LongFormatter(Period period)
    {
        int length = period.Length();
        return period.Units() switch
        {
            TimeUnit.Days => length == 1 ? $"{length} Day" : $"{length} Days",
            TimeUnit.Weeks => length == 1 ? $"{length} Week" : $"{length} Weeks",
            TimeUnit.Months => length == 1 ? $"{length} Month" : $"{length} Months",
            TimeUnit.Years => length == 1 ? $"{length} Year" : $"{length} Years",
            _ => throw new ArgumentOutOfRangeException(
                nameof(period),
                period.Units(),
                "Unknown time units")
        };
    }
    internal static string ShortFormatter(Period period)
    {
        int length = period.Length();
        return period.Units() switch
        {
            TimeUnit.Days => $"{length}D",
            TimeUnit.Weeks => $"{length}W",
            TimeUnit.Months => $"{length}M",
            TimeUnit.Years => $"{length}Y",
            _ => throw new ArgumentOutOfRangeException(
                nameof(period),
                period.Units(),
                "Unknown time units")
        };
    }
}

internal static class WeekdayFormatter
{
    internal static string LongFormatter(Weekday weekday)
    {
        return weekday switch
        {
            Weekday.Monday => "Monday",
            Weekday.Tuesday => "Tuesday",
            Weekday.Wednesday => "Wednesday",
            Weekday.Thursday => "Thursday",
            Weekday.Friday => "Friday",
            Weekday.Saturday => "Saturday",
            Weekday.Sunday => "Sunday",
            _ => throw new ArgumentOutOfRangeException(
                            nameof(weekday),
                            weekday,
                            "Unknown weekday")

        };
    }

    internal static string ShortFormatter(Weekday weekday)
    {
        return weekday switch
        {
            Weekday.Monday => "Mon",
            Weekday.Tuesday => "Tue",
            Weekday.Wednesday => "Wed",
            Weekday.Thursday => "Thu",
            Weekday.Friday => "Fri",
            Weekday.Saturday => "Sat",
            Weekday.Sunday => "Sun",
            _ => throw new ArgumentOutOfRangeException(
                            nameof(weekday),
                            weekday,
                            "Unknown weekday")

        };
    }

    internal static string ShortestFormatter(Weekday weekday)
    {
        return weekday switch
        {
            Weekday.Monday => "Mo",
            Weekday.Tuesday => "Tu",
            Weekday.Wednesday => "We",
            Weekday.Thursday => "Th",
            Weekday.Friday => "Fr",
            Weekday.Saturday => "Sa",
            Weekday.Sunday => "Su",
            _ => throw new ArgumentOutOfRangeException(
                            nameof(weekday),
                            weekday,
                            "Unknown weekday")

        };
    }
}





