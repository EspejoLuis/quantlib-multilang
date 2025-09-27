using System.ComponentModel.DataAnnotations;
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
    public void Test_Period_FromFrequency(Frequency frequency, int expectedLength, TimeUnit expectedUnits)
    {
        var p = new Period(frequency);
        Assert.Multiple(() =>
        {
            Assert.That(p.Length(), Is.EqualTo(expectedLength), $"Length mismatch for {frequency}");
            Assert.That(p.Units(), Is.EqualTo(expectedUnits), $"Units mismatch for {frequency}");
        });
    }

    [Test]
    [TestCase(Frequency.OtherFrequency, typeof(ArgumentOutOfRangeException), "Unknown frequency")]
    [TestCase((Frequency)12345, typeof(ArgumentOutOfRangeException), "Unknown frequency")]
    public void Test_Period_FromFrequency_ThrowsException(
       Frequency frequency,
       Type expectedException,
       string expectedMessageFragment)
    {
        var ex = Assert.Throws(expectedException, () => new Period(frequency));
        Assert.That(ex!.Message, Does.Contain(expectedMessageFragment));
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
    public void Test_ToFrequency_ThrowsException()
    {
        var p = new Period(1, (TimeUnit)999);
        var ex = Assert.Throws<NotImplementedException>(() => p.ToFrequency());
        Assert.That(ex!.Message, Does.Contain("TimeUnit 999 not implemented"));
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
        var p = new Period(length, units);
        Period result = p.Normalized();
        Assert.Multiple(() =>
        {
            // Check that original period p did not mutate after Normalized()
            Assert.That((p.Length(), p.Units()), Is.EqualTo((length, units)));
            // Checl that the new period is impacted by Normalized()
            Assert.That((result.Length(), result.Units()), Is.EqualTo((expectedLength, expectedUnits)));
        });

    }

    [Test]
    [TestCase(1, (TimeUnit)999)]
    [TestCase(1, (TimeUnit)9994)]
    public void Test_Normalzed_ThrowsException(
        int length,
        TimeUnit units)
    {
        var p = new Period(length, units);
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => p.Normalized());
        Assert.That(ex!.Message, Does.Contain("Unknown time units"));
    }

    [Test]
    [TestCase(-10, TimeUnit.Years, -10.0)]   // negative years
    [TestCase(0, TimeUnit.Years, 0.0)]   // zero years
    [TestCase(5, TimeUnit.Years, 5.0)]   // positive years

    // Convert Months into Years
    [TestCase(24, TimeUnit.Months, 2.0)]   // exact multiple of 12
    [TestCase(-24, TimeUnit.Months, -2.0)]   // negative multiple of 12
    [TestCase(12, TimeUnit.Months, 1.0)]   // exactly one year
    [TestCase(18, TimeUnit.Months, 1.5)]   // fractional year
    [TestCase(1, TimeUnit.Months, 1.0 / 12.0)] // smallest non-zero
    [TestCase(0, TimeUnit.Months, 0.0)]   // zero months
    public void Test_Years(
        int length, TimeUnit units,
        double expectedNumber)
    {
        var p = new Period(length, units);
        double result = p.Years();
        Assert.That(result, Is.EqualTo(expectedNumber).Within(1e-12));
    }

    [Test]
    [TestCase(7, TimeUnit.Days, typeof(NotImplementedException), "Cannot convert Days into Years")]
    [TestCase(-7, TimeUnit.Days, typeof(NotImplementedException), "Cannot convert Days into Years")]
    [TestCase(2, TimeUnit.Weeks, typeof(NotImplementedException), "Cannot convert Weeks into Years")]
    [TestCase(-2, TimeUnit.Weeks, typeof(NotImplementedException), "Cannot convert Weeks into Years")]
    [TestCase(1, (TimeUnit)999, typeof(ArgumentOutOfRangeException), "Unknown time units")]
    public void Test_Years_ThrowsException(
        int length,
        TimeUnit units,
        Type expectedException,
        string expectedMessageFragment)
    {
        var p = new Period(length, units);
        var ex = Assert.Throws(expectedException, () => p.Years());
        Assert.That(ex!.Message, Does.Contain(expectedMessageFragment));
    }

    [Test]
    [TestCase(-10, TimeUnit.Months, -10.0)]   // negative years
    [TestCase(0, TimeUnit.Months, 0.0)]   // zero years
    [TestCase(5, TimeUnit.Months, 5.0)]   // positive years

    // Convert Years into Months
    [TestCase(2, TimeUnit.Years, 24.0)]
    [TestCase(-2, TimeUnit.Years, -24.0)]
    [TestCase(1, TimeUnit.Years, 12.0)]
    [TestCase(3, TimeUnit.Years, 36.0)]
    [TestCase(24, TimeUnit.Years, 24 * 12.0)]
    [TestCase(0, TimeUnit.Years, 0.0)]
    public void Test_Months(
           int length, TimeUnit units,
           double expectedNumber)
    {
        var p = new Period(length, units);
        double result = p.Months();
        Assert.That(result, Is.EqualTo(expectedNumber).Within(1e-12));
    }

    [Test]
    [TestCase(127, TimeUnit.Days, typeof(NotImplementedException), "Cannot convert Days into Months")]
    [TestCase(-7312, TimeUnit.Days, typeof(NotImplementedException), "Cannot convert Days into Months")]
    [TestCase(21, TimeUnit.Weeks, typeof(NotImplementedException), "Cannot convert Weeks into Months")]
    [TestCase(-212, TimeUnit.Weeks, typeof(NotImplementedException), "Cannot convert Weeks into Months")]
    [TestCase(1, (TimeUnit)726, typeof(ArgumentOutOfRangeException), "Unknown time units")]
    public void Test_Months_ThrowsException(
        int length,
        TimeUnit units,
        Type expectedException,
        string expectedMessageFragment)
    {
        var p = new Period(length, units);
        var ex = Assert.Throws(expectedException, () => p.Months());
        Assert.That(ex!.Message, Does.Contain(expectedMessageFragment));
    }

    [Test]
    [TestCase(-10, TimeUnit.Weeks, -10.0)]
    [TestCase(0, TimeUnit.Weeks, 0.0)]
    [TestCase(5, TimeUnit.Weeks, 5.0)]

    // Convert Days into Weeks
    [TestCase(14, TimeUnit.Days, 2.0)]
    [TestCase(-14, TimeUnit.Days, -2.0)]
    [TestCase(7, TimeUnit.Days, 1.0)]
    [TestCase(19, TimeUnit.Days, 19.0 / 7.0)]
    [TestCase(1, TimeUnit.Days, 1.0 / 7.0)]
    [TestCase(0, TimeUnit.Days, 0.0)]
    public void Test_Weeks(
        int length, TimeUnit units,
        double expectedNumber)
    {
        var p = new Period(length, units);
        double result = p.Weeks();
        Assert.That(result, Is.EqualTo(expectedNumber).Within(1e-12));
    }

    [Test]
    [TestCase(127, TimeUnit.Months, typeof(NotImplementedException), "Cannot convert Months into Weeks")]
    [TestCase(-7312, TimeUnit.Months, typeof(NotImplementedException), "Cannot convert Months into Weeks")]
    [TestCase(21, TimeUnit.Years, typeof(NotImplementedException), "Cannot convert Years into Weeks")]
    [TestCase(-212, TimeUnit.Years, typeof(NotImplementedException), "Cannot convert Years into Weeks")]
    [TestCase(1, (TimeUnit)123, typeof(ArgumentOutOfRangeException), "Unknown time units")]
    public void Test_Weeks_ThrowsException(
       int length,
       TimeUnit units,
       Type expectedException,
       string expectedMessageFragment)
    {
        var p = new Period(length, units);
        var ex = Assert.Throws(expectedException, () => p.Weeks());
        Assert.That(ex!.Message, Does.Contain(expectedMessageFragment));
    }

    [Test]
    [TestCase(-10, TimeUnit.Days, -10.0)]
    [TestCase(0, TimeUnit.Days, 0.0)]
    [TestCase(5, TimeUnit.Days, 5.0)]

    // Convert Weeks into Days
    [TestCase(2, TimeUnit.Weeks, 14.0)]
    [TestCase(-2, TimeUnit.Weeks, -14.0)]
    [TestCase(12, TimeUnit.Weeks, 12.0 * 7.0)]
    [TestCase(15, TimeUnit.Weeks, 15 * 7.0)]
    [TestCase(1, TimeUnit.Weeks, 7.0)]
    [TestCase(0, TimeUnit.Weeks, 0.0)]
    public void Test_Days(
        int length, TimeUnit units,
        double expectedNumber)
    {
        var p = new Period(length, units);
        double result = p.Days();
        Assert.That(result, Is.EqualTo(expectedNumber).Within(1e-12));
    }

    [Test]
    [TestCase(127, TimeUnit.Months, typeof(NotImplementedException), "Cannot convert Months into Days")]
    [TestCase(-7312, TimeUnit.Months, typeof(NotImplementedException), "Cannot convert Months into Days")]
    [TestCase(21, TimeUnit.Years, typeof(NotImplementedException), "Cannot convert Years into Days")]
    [TestCase(-212, TimeUnit.Years, typeof(NotImplementedException), "Cannot convert Years into Days")]
    [TestCase(1, (TimeUnit)9199, typeof(ArgumentOutOfRangeException), "Unknown time units")]
    public void Test_Days_ThrowsException(
        int length,
        TimeUnit units,
        Type expectedException,
        string expectedMessageFragment)
    {
        var p = new Period(length, units);
        var ex = Assert.Throws(expectedException, () => p.Days());
        Assert.That(ex!.Message, Does.Contain(expectedMessageFragment));
    }

    [Test]
    [TestCase(0, TimeUnit.Years, 3, TimeUnit.Months, 3, TimeUnit.Months)]
    [TestCase(2, TimeUnit.Years, 3, TimeUnit.Years, 5, TimeUnit.Years)]
    [TestCase(2, TimeUnit.Years, 6, TimeUnit.Months, 30, TimeUnit.Months)] // 24M + 6M
    [TestCase(6, TimeUnit.Months, 2, TimeUnit.Years, 30, TimeUnit.Months)] // 6M + 24M
    [TestCase(2, TimeUnit.Weeks, 3, TimeUnit.Days, 17, TimeUnit.Days)]     // 14D + 3D
    [TestCase(3, TimeUnit.Days, 2, TimeUnit.Weeks, 17, TimeUnit.Days)]     // 3D + 14D
    [TestCase(5, TimeUnit.Days, 3, TimeUnit.Days, 8, TimeUnit.Days)]
    public void Test_OperatorPlus(
             int lhsLen, TimeUnit lhsUnit,
             int rhsLen, TimeUnit rhsUnit,
             int expectedLen, TimeUnit expectedUnit)
    {
        var lhs = new Period(lhsLen, lhsUnit);
        var rhs = new Period(rhsLen, rhsUnit);
        var result = lhs + rhs;
        Assert.Multiple(() =>
        {
            Assert.That(result.Length(), Is.EqualTo(expectedLen),
                $"Expected length {expectedLen}, got {result.Length()}");
            Assert.That(result.Units(), Is.EqualTo(expectedUnit),
                $"Expected units {expectedUnit}, got {result.Units()}");
        });
    }

    [Test]
    [TestCase(1, TimeUnit.Years, 5, TimeUnit.Days)]
    [TestCase(1, TimeUnit.Years, 2, TimeUnit.Weeks)]
    [TestCase(1, TimeUnit.Months, 5, TimeUnit.Days)]
    [TestCase(1, TimeUnit.Months, 2, TimeUnit.Weeks)]
    [TestCase(1, TimeUnit.Weeks, 1, TimeUnit.Years)]
    [TestCase(1, TimeUnit.Weeks, 2, TimeUnit.Months)]
    [TestCase(1, TimeUnit.Days, 1, TimeUnit.Years)]
    [TestCase(1, TimeUnit.Days, 2, TimeUnit.Months)]
    public void Test_OperatorPlus_ThrowsOnInvalidCombinations(
        int lhsLen, TimeUnit lhsUnit,
        int rhsLen, TimeUnit rhsUnit)
    {
        var lhs = new Period(lhsLen, lhsUnit);
        var rhs = new Period(rhsLen, rhsUnit);
        var ex = Assert.Throws<ArgumentException>(() => { var _ = lhs + rhs; });
        Assert.That(ex!.Message,
            Is.EqualTo($"Impossible addition between {lhs} and {rhs}"));
    }

    [Test]
    [TestCase(1, (TimeUnit)999, 2, TimeUnit.Years)]
    [TestCase(1, (TimeUnit)999, 2, TimeUnit.Months)]
    [TestCase(2, TimeUnit.Months, 3, (TimeUnit)999)]
    [TestCase(2, TimeUnit.Years, 2, (TimeUnit)999)]
    [TestCase(1, (TimeUnit)999, 2, TimeUnit.Weeks)]
    [TestCase(1, (TimeUnit)999, 2, TimeUnit.Days)]
    [TestCase(2, TimeUnit.Weeks, 3, (TimeUnit)999)]
    [TestCase(2, TimeUnit.Days, 2, (TimeUnit)999)]
    public void Test_OperatorPlus_ThrowsOnUnknownTimeUnits(
            int lhsLen, TimeUnit lhsUnit,
            int rhsLen, TimeUnit rhsUnit)
    {
        var lhs = new Period(lhsLen, lhsUnit);
        var rhs = new Period(rhsLen, rhsUnit);
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => { var _ = lhs + rhs; });
        Assert.That(ex!.Message, Does.Contain("Unknown time units"));
    }

    [Test]
    [TestCase(1, TimeUnit.Years, -1, TimeUnit.Years)]
    [TestCase(-1, TimeUnit.Weeks, 1, TimeUnit.Weeks)]
    [TestCase(-1, TimeUnit.Months, 1, TimeUnit.Months)]
    [TestCase(45, TimeUnit.Days, -45, TimeUnit.Days)]
    public void Test_UnaryOperatorMinus(int length, TimeUnit units, int expectedLength, TimeUnit expectedUnits)
    {
        var period = new Period(length, units);
        var negativePeriod = -period;
        Assert.That((negativePeriod.Length(), negativePeriod.Units()), Is.EqualTo((expectedLength, expectedUnits)));
    }



}
