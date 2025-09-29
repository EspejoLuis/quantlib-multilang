using QuantLibCSharp.Time;
using QuantLibCSharp.Time.IO;

namespace QuantLibCSharp.UnitTests;

public class WeekdayUnitTests
{

    [Test]
    [TestCase(Weekday.Monday, "Monday")]
    [TestCase(Weekday.Tuesday, "Tuesday")]
    [TestCase(Weekday.Wednesday, "Wednesday")]
    [TestCase(Weekday.Thursday, "Thursday")]
    [TestCase(Weekday.Friday, "Friday")]
    [TestCase(Weekday.Saturday, "Saturday")]
    [TestCase(Weekday.Sunday, "Sunday")]
    public void Test_LongFormat(
                Weekday w,
                string expectedLong)
    {
        Assert.That(WeekdayIO.LongFormat(w),
            Is.EqualTo(expectedLong),
            $"{WeekdayIO.LongFormat(w)} expected to be equal to {expectedLong}");
    }

    [Test]
    public void Test_LongFormat_ThrowsArgumentOutOfRangeException()
    {
        var w = (Weekday)9;
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => { var _ = WeekdayIO.LongFormat(w); });
        Assert.Multiple(() =>
        {
            Assert.That(ex!.ParamName, Is.EqualTo("weekday"),
                $"Unexpected ParamName for invalid weekday in {w}");
            Assert.That(ex.ActualValue, Is.EqualTo((Weekday)9),
                $"Unexpected ActualValue for invalid weekday in {w}");
            Assert.That(ex.Message, Does.Contain("Unknown weekday"),
                $"Unexpected exception message for invalid weekday in {w}");
        });
    }

    [Test]
    [TestCase(Weekday.Monday, "Mon")]
    [TestCase(Weekday.Tuesday, "Tue")]
    [TestCase(Weekday.Wednesday, "Wed")]
    [TestCase(Weekday.Thursday, "Thu")]
    [TestCase(Weekday.Friday, "Fri")]
    [TestCase(Weekday.Saturday, "Sat")]
    [TestCase(Weekday.Sunday, "Sun")]
    public void Test_ShortFormat(
                    Weekday w,
                    string expectedShort)
    {
        Assert.That(WeekdayIO.ShortFormat(w),
            Is.EqualTo(expectedShort),
            $"{WeekdayIO.ShortFormat(w)} expected to be equal to {expectedShort}");
    }

    [Test]
    public void Test_ShortFormat_ThrowsArgumentOutOfRangeException()
    {
        var w = (Weekday)9;
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => { var _ = WeekdayIO.ShortFormat(w); });
        Assert.Multiple(() =>
        {
            Assert.That(ex!.ParamName, Is.EqualTo("weekday"),
                $"Unexpected ParamName for invalid weekday in {w}");
            Assert.That(ex.ActualValue, Is.EqualTo((Weekday)9),
                $"Unexpected ActualValue for invalid weekday in {w}");
            Assert.That(ex.Message, Does.Contain("Unknown weekday"),
                $"Unexpected exception message for invalid weekday in {w}");
        });
    }


    [Test]
    [TestCase(Weekday.Monday, "Mo")]
    [TestCase(Weekday.Tuesday, "Tu")]
    [TestCase(Weekday.Wednesday, "We")]
    [TestCase(Weekday.Thursday, "Th")]
    [TestCase(Weekday.Friday, "Fr")]
    [TestCase(Weekday.Saturday, "Sa")]
    [TestCase(Weekday.Sunday, "Su")]
    public void Test_ShortestFormat(
                    Weekday w,
                    string expectedShortest)
    {
        Assert.That(WeekdayIO.ShortestFormat(w),
            Is.EqualTo(expectedShortest),
            $"{WeekdayIO.ShortestFormat(w)} expected to be equal to {expectedShortest}");
    }

    [Test]
    public void Test_ShortestFormat_ThrowsArgumentOutOfRangeException()
    {
        var w = (Weekday)9;
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => { var _ = WeekdayIO.ShortestFormat(w); });
        Assert.Multiple(() =>
        {
            Assert.That(ex!.ParamName, Is.EqualTo("weekday"),
                $"Unexpected ParamName for invalid weekday in {w}");
            Assert.That(ex.ActualValue, Is.EqualTo((Weekday)9),
                $"Unexpected ActualValue for invalid weekday in {w}");
            Assert.That(ex.Message, Does.Contain("Unknown weekday"),
                $"Unexpected exception message for invalid weekday in {w}");
        });
    }


}