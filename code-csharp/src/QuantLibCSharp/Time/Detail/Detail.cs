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





