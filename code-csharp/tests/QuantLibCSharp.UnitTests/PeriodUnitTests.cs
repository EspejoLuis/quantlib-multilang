using System.Runtime;
using NUnit.Framework;
using QuantLibCSharp.Time;

namespace QuantLibCSharp.UnitTests;

public class PeriodUnitTests
{
    [Test]
    [TestCase(5, TimeUnit.Days)]
    [TestCase(10, TimeUnit.Weeks)]
    [TestCase(12, TimeUnit.Months)]
    [TestCase(2, TimeUnit.Years)]
    [TestCase(-5, TimeUnit.Days)]    // negative case
    [TestCase(-2, TimeUnit.Years)]   // negative case
    [TestCase(0, TimeUnit.Days)]     // zero case
    [TestCase(0, TimeUnit.Months)]   // zero case
    public void Test_Period(int length, TimeUnit units)
    {
        var p = new Period(length, units);

        Assert.Multiple(() =>
        {
            Assert.That(p.Length(), Is.EqualTo(length), "Length mismatch");
            Assert.That(p.Units(), Is.EqualTo(units), "Units mismatch");
        });
    }

    [Test]
    [TestCase(Frequency.NoFrequency, 0, TimeUnit.Days)]
    [TestCase(Frequency.Once, 0, TimeUnit.Years)]
    [TestCase(Frequency.Annual, 1, TimeUnit.Years)]
    [TestCase(Frequency.Semiannual, 6, TimeUnit.Months)]
    [TestCase(Frequency.EveryFourthMonth, 4, TimeUnit.Months)]
    [TestCase(Frequency.Quarterly, 3, TimeUnit.Months)]
    [TestCase(Frequency.Bimonthly, 2, TimeUnit.Months)]
    [TestCase(Frequency.Monthly, 1, TimeUnit.Months)]
    [TestCase(Frequency.EveryFourthWeek, 4, TimeUnit.Weeks)]
    [TestCase(Frequency.Biweekly, 2, TimeUnit.Weeks)]
    [TestCase(Frequency.Weekly, 1, TimeUnit.Weeks)]
    [TestCase(Frequency.Daily, 1, TimeUnit.Days)]
    public void Test_Period_FromFrequency_Valid(Frequency frequency, int expectedLength, TimeUnit expectedUnits)
    {
        var p = new Period(frequency);

        Assert.Multiple(() =>
        {
            Assert.That(p.Length(), Is.EqualTo(expectedLength), $"Length mismatch for {frequency}");
            Assert.That(p.Units(), Is.EqualTo(expectedUnits), $"Units mismatch for {frequency}");
        });
    }

    [Test]
    [TestCase(Frequency.OtherFrequency)]
    [TestCase((Frequency)12345)] // unmapped frequency
    public void Test_Period_FromFrequency_Invalid(Frequency frequency)
    {
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => new Period(frequency));
        Assert.That(ex!.Message, Does.Contain("Unknown frequency"));
    }


}