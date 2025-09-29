using QuantLibCSharp.Time.Detail;

namespace QuantLibCSharp.Time.IO;

public static class PeriodIO
{
    public static string LongFormat(Period p) => PeriodFormatter.LongFormatter(p);
    public static string ShortFormat(Period p) => PeriodFormatter.ShortFormatter(p);
}
