using QuantLibCSharp.Time.Detail;

namespace QuantLibCSharp.Time.IO;

public static class PeriodIO
{
    public static string LongFormat(Period p) => PeriodFormatter.LongFormatter(p);
    public static string ShortFormat(Period p) => PeriodFormatter.ShortFormatter(p);
}

public static class WeekdayIO
{
    public static string LongFormat(Weekday w) => WeekdayFormatter.LongFormatter(w);
    public static string ShortFormat(Weekday w) => WeekdayFormatter.ShortFormatter(w);
    public static string ShortestFormat(Weekday w) => WeekdayFormatter.ShortestFormatter(w);
}