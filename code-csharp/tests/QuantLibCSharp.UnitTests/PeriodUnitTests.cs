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

    [Test]
    [TestCase(0, TimeUnit.Years, Frequency.Once)]
    [TestCase(0, TimeUnit.Months, Frequency.NoFrequency)]
    [TestCase(1, TimeUnit.Years, Frequency.Annual)]
    [TestCase(2, TimeUnit.Years, Frequency.OtherFrequency)]
    [TestCase(1, TimeUnit.Months, Frequency.Monthly)]
    [TestCase(2, TimeUnit.Months, Frequency.Bimonthly)]
    [TestCase(3, TimeUnit.Months, Frequency.Quarterly)]
    [TestCase(4, TimeUnit.Months, Frequency.EveryFourthMonth)]
    [TestCase(6, TimeUnit.Months, Frequency.Semiannual)]
    [TestCase(12, TimeUnit.Months, Frequency.Annual)]
    [TestCase(5, TimeUnit.Months, Frequency.OtherFrequency)]
    [TestCase(1, TimeUnit.Weeks, Frequency.Weekly)]
    [TestCase(2, TimeUnit.Weeks, Frequency.Biweekly)]
    [TestCase(4, TimeUnit.Weeks, Frequency.EveryFourthWeek)]
    [TestCase(3, TimeUnit.Weeks, Frequency.OtherFrequency)]
    [TestCase(1, TimeUnit.Days, Frequency.Daily)]
    [TestCase(2, TimeUnit.Days, Frequency.OtherFrequency)]
    public void Test_ToFrequency(int length, TimeUnit units, Frequency expected)
    {
        var p = new Period(length, units);
        Assert.That(p.ToFrequency(), Is.EqualTo(expected));
    }

    [Test]
    public void Test_ToFrequency_NotImplemented()
    {
        var p = new Period(1, (TimeUnit)999);
        Assert.Throws<NotImplementedException>(() => p.ToFrequency());
    }

    [Test]
    // _ZeroLength_BecomesDays
    [TestCase(0, TimeUnit.Months, 0, TimeUnit.Days)]
    [TestCase(0, TimeUnit.Years, 0, TimeUnit.Days)]

    //Months_Multiple_of_12_Becomes_Years
    [TestCase(12, TimeUnit.Months, 1, TimeUnit.Years)]
    [TestCase(24, TimeUnit.Months, 2, TimeUnit.Years)]
    [TestCase(-36, TimeUnit.Months, -3, TimeUnit.Years)]

    // Months_NotMultipleOf12_NotChanged
    [TestCase(14, TimeUnit.Months, 14, TimeUnit.Months)]
    [TestCase(25, TimeUnit.Months, 25, TimeUnit.Months)]

    // Days_MultipleOf7_Becomes_Weeks
    [TestCase(7, TimeUnit.Days, 1, TimeUnit.Weeks)]
    [TestCase(14, TimeUnit.Days, 2, TimeUnit.Weeks)]
    [TestCase(-21, TimeUnit.Days, -3, TimeUnit.Weeks)]

    // Days_NotMultipleOf7_NotChanged
    [TestCase(8, TimeUnit.Days, 8, TimeUnit.Days)]
    [TestCase(17, TimeUnit.Days, 17, TimeUnit.Days)]

    // Weeks_And_Years_Remain_Unchanged
    [TestCase(5, TimeUnit.Weeks, 5, TimeUnit.Weeks)]
    [TestCase(-3, TimeUnit.Years, -3, TimeUnit.Years)]

    public void Test_Normalized(
        int length, TimeUnit units,
        int expectedLength, TimeUnit expectedUnits)
    {
        // Arrange
        var p = new Period(length, units);

        // Act
        Period result = p.Normalized();

        // Assert
        Assert.Multiple(() =>
        {
            // Check that original period p did not mutate after Normalized()
            Assert.That((p.Length(), p.Units()), Is.EqualTo((length, units)));
            // Checl that the new period is impacted by Normalized()
            Assert.That((result.Length(), result.Units()), Is.EqualTo((expectedLength, expectedUnits)));
        });

    }


}