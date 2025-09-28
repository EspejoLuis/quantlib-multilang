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
    public void Test_Period_FromFrequency(
        Frequency frequency, int expectedLength, TimeUnit expectedUnits)
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
    [TestCase((Frequency)12345)]
    public void Test_Period_FromFrequency_ThrowsException(
       Frequency frequency
       )
    {
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => new Period(frequency));
        Assert.Multiple(() =>
        {
            Assert.That(ex!.ParamName, Is.EqualTo("frequency"),
                $"For frequency={frequency}, expected ParamName='frequency' but got '{ex.ParamName}'");

            Assert.That(ex.ActualValue, Is.EqualTo(frequency),
                $"For frequency={frequency}, expected ActualValue={frequency} but got {ex.ActualValue}");

            Assert.That(ex.Message, Does.Contain("Unknown frequency"),
                $"For frequency={frequency}, expected message to contain 'Unknown frequency' but got '{ex.Message}'");
        });
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
        var result = p.ToFrequency();
        Assert.That(result, Is.EqualTo(expected),
            $"Period({length}, {units}) : Expected Frequency={expected}, but got {result}");
    }

    [Test]
    public void Test_ToFrequency_ThrowsException()
    {
        var p = new Period(1, (TimeUnit)999);
        var ex = Assert.Throws<NotImplementedException>(() => p.ToFrequency());
        Assert.That(ex!.Message, Is.EqualTo("TimeUnit 999 not implemented"),
            $"Unexpected exception message for ToFrequency with invalid TimeUnit=999");
    }

    [Test]
    [TestCase(0, TimeUnit.Months, 0, TimeUnit.Days)]
    [TestCase(0, TimeUnit.Years, 0, TimeUnit.Days)]
    [TestCase(12, TimeUnit.Months, 1, TimeUnit.Years)]
    [TestCase(24, TimeUnit.Months, 2, TimeUnit.Years)]
    [TestCase(-36, TimeUnit.Months, -3, TimeUnit.Years)]
    [TestCase(14, TimeUnit.Months, 14, TimeUnit.Months)]
    [TestCase(25, TimeUnit.Months, 25, TimeUnit.Months)]
    [TestCase(7, TimeUnit.Days, 1, TimeUnit.Weeks)]
    [TestCase(14, TimeUnit.Days, 2, TimeUnit.Weeks)]
    [TestCase(-21, TimeUnit.Days, -3, TimeUnit.Weeks)]
    [TestCase(8, TimeUnit.Days, 8, TimeUnit.Days)]
    [TestCase(17, TimeUnit.Days, 17, TimeUnit.Days)]
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
            Assert.That((p.Length(), p.Units()), Is.EqualTo((length, units)),
                $"Expected original Period to remain unchanged, but got ({p.Length()},{p.Units()})");
            Assert.That((result.Length(), result.Units()), Is.EqualTo((expectedLength, expectedUnits)),
                $"Expected normalized Period=({expectedLength},{expectedUnits}), but got ({result.Length()},{result.Units()})");
        });
    }

    [Test]
    [TestCase(1, (TimeUnit)999)]
    [TestCase(1, (TimeUnit)9994)]
    public void Test_Normalzed_ThrowsException(int length, TimeUnit units)
    {
        var p = new Period(length, units);
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => p.Normalized());
        Assert.Multiple(() =>
        {
            Assert.That(ex!.ParamName, Is.EqualTo("_units"),
                $"Unexpected ParamName for invalid TimeUnit={units}");
            Assert.That(ex.ActualValue, Is.EqualTo(units),
                $"Unexpected ActualValue for invalid TimeUnit={units}");
            Assert.That(ex.Message, Does.Contain("Unknown time units"),
                $"Unexpected exception message for invalid TimeUnit={units}");
        });
    }

    [Test]
    [TestCase(-10, TimeUnit.Years, -10.0)]
    [TestCase(0, TimeUnit.Years, 0.0)]
    [TestCase(5, TimeUnit.Years, 5.0)]
    [TestCase(24, TimeUnit.Months, 2.0)]
    [TestCase(-24, TimeUnit.Months, -2.0)]
    [TestCase(12, TimeUnit.Months, 1.0)]
    [TestCase(18, TimeUnit.Months, 1.5)]
    [TestCase(1, TimeUnit.Months, 1.0 / 12.0)]
    [TestCase(0, TimeUnit.Months, 0.0)]
    public void Test_Years(int length, TimeUnit units, double expectedNumber)
    {
        var p = new Period(length, units);
        double result = p.Years();
        Assert.That(result, Is.EqualTo(expectedNumber).Within(1e-12),
            $"Period({length}, {units}) : Expected Years={expectedNumber}, but got {result}");
    }

    [Test]
    [TestCase(7, TimeUnit.Days, "Cannot convert Days into Years")]
    [TestCase(-7, TimeUnit.Days, "Cannot convert Days into Years")]
    [TestCase(2, TimeUnit.Weeks, "Cannot convert Weeks into Years")]
    [TestCase(-2, TimeUnit.Weeks, "Cannot convert Weeks into Years")]
    public void Test_Years_ThrowsNotImplementedException(
        int length, TimeUnit units, string expectedMessageFragment)
    {
        var p = new Period(length, units);
        var ex = Assert.Throws<NotImplementedException>(() => p.Years());
        Assert.That(ex!.Message, Is.EqualTo(expectedMessageFragment),
            $"Unexpected exception message for Period({length}, {units})");
    }

    [Test]
    [TestCase(1, (TimeUnit)999)]
    [TestCase(1, (TimeUnit)9994)]
    public void Test_Years_ThrowsOutOfRangeException(int length, TimeUnit units)
    {
        var p = new Period(length, units);
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => p.Years());
        Assert.Multiple(() =>
        {
            Assert.That(ex!.ParamName, Is.EqualTo("_units"),
                $"Unexpected ParamName for invalid TimeUnit={units}");
            Assert.That(ex.ActualValue, Is.EqualTo(units),
                $"Unexpected ActualValue for invalid TimeUnit={units}");
            Assert.That(ex.Message, Does.Contain("Unknown time units"),
                $"Unexpected exception message for invalid TimeUnit={units}");
        });
    }

    [Test]
    [TestCase(-10, TimeUnit.Months, -10.0)]
    [TestCase(0, TimeUnit.Months, 0.0)]
    [TestCase(5, TimeUnit.Months, 5.0)]
    [TestCase(2, TimeUnit.Years, 24.0)]
    [TestCase(-2, TimeUnit.Years, -24.0)]
    [TestCase(1, TimeUnit.Years, 12.0)]
    [TestCase(3, TimeUnit.Years, 36.0)]
    [TestCase(24, TimeUnit.Years, 288.0)]
    [TestCase(0, TimeUnit.Years, 0.0)]
    public void Test_Months(int length, TimeUnit units, double expectedNumber)
    {
        var p = new Period(length, units);
        double result = p.Months();
        Assert.That(result, Is.EqualTo(expectedNumber).Within(1e-12),
            $"Period({length}, {units}) : Expected Months={expectedNumber}, but got {result}");
    }

    [Test]
    [TestCase(127, TimeUnit.Days, typeof(NotImplementedException), "Cannot convert Days into Months")]
    [TestCase(-7312, TimeUnit.Days, typeof(NotImplementedException), "Cannot convert Days into Months")]
    [TestCase(21, TimeUnit.Weeks, typeof(NotImplementedException), "Cannot convert Weeks into Months")]
    [TestCase(-212, TimeUnit.Weeks, typeof(NotImplementedException), "Cannot convert Weeks into Months")]
    public void Test_Months_ThrowsException(
        int length, TimeUnit units,
        Type expectedException, string expectedMessageFragment)
    {
        var p = new Period(length, units);
        var ex = Assert.Throws(expectedException, () => p.Months());
        Assert.That(ex!.Message, Is.EqualTo(expectedMessageFragment),
            $"Unexpected exception message for Period({length}, {units})");
    }

    [Test]
    [TestCase(1, (TimeUnit)999)]
    [TestCase(1, (TimeUnit)9994)]
    public void Test_Months_ThrowsOutOfRangeException(int length, TimeUnit units)
    {
        var p = new Period(length, units);
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => p.Months());
        Assert.Multiple(() =>
        {
            Assert.That(ex!.ParamName, Is.EqualTo("_units"),
                $"Unexpected ParamName for invalid TimeUnit={units}");
            Assert.That(ex.ActualValue, Is.EqualTo(units),
                $"Unexpected ActualValue for invalid TimeUnit={units}");
            Assert.That(ex.Message, Does.Contain("Unknown time units"),
                $"Unexpected exception message for invalid TimeUnit={units}");
        });
    }

    [Test]
    [TestCase(-10, TimeUnit.Weeks, -10.0)]
    [TestCase(0, TimeUnit.Weeks, 0.0)]
    [TestCase(5, TimeUnit.Weeks, 5.0)]
    [TestCase(14, TimeUnit.Days, 2.0)]
    [TestCase(-14, TimeUnit.Days, -2.0)]
    [TestCase(7, TimeUnit.Days, 1.0)]
    [TestCase(19, TimeUnit.Days, 19.0 / 7.0)]
    [TestCase(1, TimeUnit.Days, 1.0 / 7.0)]
    [TestCase(0, TimeUnit.Days, 0.0)]
    public void Test_Weeks(int length, TimeUnit units, double expectedNumber)
    {
        var p = new Period(length, units);
        double result = p.Weeks();
        Assert.That(result, Is.EqualTo(expectedNumber).Within(1e-12),
            $"Period({length}, {units}) : Expected Weeks={expectedNumber}, but got {result}");
    }

    [Test]
    [TestCase(127, TimeUnit.Months, typeof(NotImplementedException), "Cannot convert Months into Weeks")]
    [TestCase(-7312, TimeUnit.Months, typeof(NotImplementedException), "Cannot convert Months into Weeks")]
    [TestCase(21, TimeUnit.Years, typeof(NotImplementedException), "Cannot convert Years into Weeks")]
    [TestCase(-212, TimeUnit.Years, typeof(NotImplementedException), "Cannot convert Years into Weeks")]
    public void Test_Weeks_ThrowsNotImplementedException(
        int length, TimeUnit units,
        Type expectedException, string expectedMessageFragment)
    {
        var p = new Period(length, units);
        var ex = Assert.Throws(expectedException, () => p.Weeks());
        Assert.That(ex!.Message, Is.EqualTo(expectedMessageFragment),
            $"Unexpected exception message for Period({length}, {units})");
    }

    [Test]
    [TestCase(1, (TimeUnit)999)]
    [TestCase(1, (TimeUnit)9994)]
    public void Test_Weeks_ThrowsOutOfRangeException(int length, TimeUnit units)
    {
        var p = new Period(length, units);
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => p.Weeks());
        Assert.Multiple(() =>
        {
            Assert.That(ex!.ParamName, Is.EqualTo("_units"),
                $"Unexpected ParamName for invalid TimeUnit={units}");
            Assert.That(ex.ActualValue, Is.EqualTo(units),
                $"Unexpected ActualValue for invalid TimeUnit={units}");
            Assert.That(ex.Message, Does.Contain("Unknown time units"),
                $"Unexpected exception message for invalid TimeUnit={units}");
        });
    }

    [Test]
    [TestCase(-10, TimeUnit.Days, -10.0)]
    [TestCase(0, TimeUnit.Days, 0.0)]
    [TestCase(5, TimeUnit.Days, 5.0)]
    [TestCase(2, TimeUnit.Weeks, 14.0)]
    [TestCase(-2, TimeUnit.Weeks, -14.0)]
    [TestCase(12, TimeUnit.Weeks, 84.0)]
    [TestCase(15, TimeUnit.Weeks, 105.0)]
    [TestCase(1, TimeUnit.Weeks, 7.0)]
    [TestCase(0, TimeUnit.Weeks, 0.0)]
    public void Test_Days(int length, TimeUnit units, double expectedNumber)
    {
        var p = new Period(length, units);
        double result = p.Days();
        Assert.That(result, Is.EqualTo(expectedNumber).Within(1e-12),
            $"Period({length}, {units}) : Expected Days={expectedNumber}, but got {result}");
    }

    [Test]
    [TestCase(127, TimeUnit.Months, typeof(NotImplementedException), "Cannot convert Months into Days")]
    [TestCase(-7312, TimeUnit.Months, typeof(NotImplementedException), "Cannot convert Months into Days")]
    [TestCase(21, TimeUnit.Years, typeof(NotImplementedException), "Cannot convert Years into Days")]
    [TestCase(-212, TimeUnit.Years, typeof(NotImplementedException), "Cannot convert Years into Days")]
    public void Test_Days_ThrowsException(
        int length, TimeUnit units,
        Type expectedException, string expectedMessageFragment)
    {
        var p = new Period(length, units);
        var ex = Assert.Throws(expectedException, () => p.Days());
        Assert.That(ex!.Message, Is.EqualTo(expectedMessageFragment),
            $"Unexpected exception message for Period({length}, {units})");
    }

    [Test]
    [TestCase(1, (TimeUnit)999)]
    [TestCase(1, (TimeUnit)9994)]
    public void Test_Days_ThrowsOutOfRangeException(int length, TimeUnit units)
    {
        var p = new Period(length, units);
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => p.Days());
        Assert.Multiple(() =>
        {
            Assert.That(ex!.ParamName, Is.EqualTo("_units"),
                $"Unexpected ParamName for invalid TimeUnit={units}");
            Assert.That(ex.ActualValue, Is.EqualTo(units),
                $"Unexpected ActualValue for invalid TimeUnit={units}");
            Assert.That(ex.Message, Does.Contain("Unknown time units"),
                $"Unexpected exception message for invalid TimeUnit={units}");
        });
    }

    [Test]
    [TestCase(0, TimeUnit.Years, 3, TimeUnit.Months, 3, TimeUnit.Months)]
    [TestCase(2, TimeUnit.Years, 3, TimeUnit.Years, 5, TimeUnit.Years)]
    [TestCase(2, TimeUnit.Years, 6, TimeUnit.Months, 30, TimeUnit.Months)]
    [TestCase(6, TimeUnit.Months, 2, TimeUnit.Years, 30, TimeUnit.Months)]
    [TestCase(2, TimeUnit.Weeks, 3, TimeUnit.Days, 17, TimeUnit.Days)]
    [TestCase(3, TimeUnit.Days, 2, TimeUnit.Weeks, 17, TimeUnit.Days)]
    [TestCase(5, TimeUnit.Days, 3, TimeUnit.Days, 8, TimeUnit.Days)]
    [TestCase(-2, TimeUnit.Years, 3, TimeUnit.Years, 1, TimeUnit.Years)]
    [TestCase(5, TimeUnit.Days, -3, TimeUnit.Days, 2, TimeUnit.Days)]
    [TestCase(-6, TimeUnit.Months, 12, TimeUnit.Months, 6, TimeUnit.Months)]
    [TestCase(2, TimeUnit.Weeks, -1, TimeUnit.Weeks, 1, TimeUnit.Weeks)]
    [TestCase(-1, TimeUnit.Years, -6, TimeUnit.Months, -18, TimeUnit.Months)]
    [TestCase(-2, TimeUnit.Weeks, -3, TimeUnit.Days, -17, TimeUnit.Days)]
    [TestCase(0, TimeUnit.Years, -4, TimeUnit.Months, -4, TimeUnit.Months)]
    [TestCase(0, TimeUnit.Days, -5, TimeUnit.Days, -5, TimeUnit.Days)]
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
                $"lhs={lhs}, rhs={rhs} : Expected length {expectedLen}, but got {result.Length()}");
            Assert.That(result.Units(), Is.EqualTo(expectedUnit),
                $"lhs={lhs}, rhs={rhs} : Expected units {expectedUnit}, but got {result.Units()}");
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
        var ex = Assert.Throws<InvalidOperationException>(() => { var _ = lhs + rhs; });
        Assert.That(ex!.Message,
            Is.EqualTo($"Impossible addition between {lhs} and {rhs}"),
            $"Unexpected exception message for lhs={lhs}, rhs={rhs}");
    }

    [Test]
    [TestCase(-1, (TimeUnit)999, 2, TimeUnit.Years)]
    [TestCase(1, (TimeUnit)999, -2, TimeUnit.Months)]
    [TestCase(2, TimeUnit.Months, 3, (TimeUnit)999)]
    [TestCase(2, TimeUnit.Years, 2, (TimeUnit)999)]
    [TestCase(-1, (TimeUnit)999, 2, TimeUnit.Weeks)]
    [TestCase(1, (TimeUnit)999, 2, TimeUnit.Days)]
    [TestCase(2, TimeUnit.Weeks, -3, (TimeUnit)999)]
    [TestCase(2, TimeUnit.Days, 2, (TimeUnit)999)]
    public void Test_OperatorPlus_ThrowsOnUnknownTimeUnits(
        int lhsLen, TimeUnit lhsUnit,
        int rhsLen, TimeUnit rhsUnit)
    {
        var lhs = new Period(lhsLen, lhsUnit);
        var rhs = new Period(rhsLen, rhsUnit);
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => { var _ = lhs + rhs; });
        Assert.Multiple(() =>
        {
            Assert.That(ex!.ParamName, Is.EqualTo("rhs"),
                $"Unexpected ParamName for lhs={lhs}, rhs={rhs}");
            Assert.That(ex.ActualValue, Is.EqualTo(rhsUnit),
                $"Unexpected ActualValue for lhs={lhs}, rhs={rhs}");
            Assert.That(ex.Message, Does.Contain("Unknown time units"),
                $"Unexpected exception message for lhs={lhs}, rhs={rhs}");
        });
    }

    [Test]
    [TestCase(1, TimeUnit.Years, -1, TimeUnit.Years)]
    [TestCase(-1, TimeUnit.Weeks, 1, TimeUnit.Weeks)]
    [TestCase(-1, TimeUnit.Months, 1, TimeUnit.Months)]
    [TestCase(45, TimeUnit.Days, -45, TimeUnit.Days)]
    public void Test_UnaryOperatorMinus(
        int length, TimeUnit units,
        int expectedLength, TimeUnit expectedUnits)
    {
        var period = new Period(length, units);
        var negativePeriod = -period;
        Assert.That((negativePeriod.Length(), negativePeriod.Units()),
            Is.EqualTo((expectedLength, expectedUnits)),
            $"Period({length}, {units}) : Expected negated ({expectedLength},{expectedUnits}), but got ({negativePeriod.Length()},{negativePeriod.Units()})");
    }

    [Test]
    [TestCase(12, TimeUnit.Months, 3, 4, TimeUnit.Months)]
    [TestCase(1, TimeUnit.Years, 2, 6, TimeUnit.Months)]
    [TestCase(-2, TimeUnit.Weeks, 2, -1, TimeUnit.Weeks)]
    [TestCase(-10, TimeUnit.Days, 2, -5, TimeUnit.Days)]
    [TestCase(6, TimeUnit.Months, 3, 2, TimeUnit.Months)]
    public void Test_OperatorDivide(
        int length, TimeUnit unit,
        int divider,
        int expectedLength, TimeUnit expectedUnit)
    {
        var p = new Period(length, unit);
        var result = p / divider;

        Assert.Multiple(() =>
        {
            Assert.That(result.Length(), Is.EqualTo(expectedLength),
                $"Period({length}, {unit})/{divider} : Expected length {expectedLength}, but got {result.Length()}");
            Assert.That(result.Units(), Is.EqualTo(expectedUnit),
                $"Period({length}, {unit})/{divider} : Expected units {expectedUnit}, but got {result.Units()}");
        });
    }

    [Test]
    [TestCase(3, TimeUnit.Years, 5)]
    [TestCase(5, TimeUnit.Months, 2)]
    [TestCase(-5, TimeUnit.Days, 2)]
    [TestCase(-5, TimeUnit.Weeks, 2)]
    public void Test_OperatorDivide_ThrowsInvalidOperationException(
        int length, TimeUnit unit, int divider)
    {
        var p = new Period(length, unit);
        var ex = Assert.Throws<InvalidOperationException>(() => { var _ = p / divider; });
        Assert.That(ex!.Message, Is.EqualTo($"{p} cannot be divided by {divider}"),
            $"Unexpected exception message for Period({length},{unit})/{divider}");
    }

    [Test]
    public void Test_OperatorDivide_ThrowsDivideByZeroException()
    {
        var p = new Period(5, TimeUnit.Years);
        var ex = Assert.Throws<DivideByZeroException>(() => { var _ = p / 0; });
        Assert.That(ex!.Message, Is.EqualTo("Period cannot be divided by zero"),
            $"Unexpected exception message for Period({p.Length()},{p.Units()})/0");
    }

    [Test]
    public void Test_OperatorDivide_ThrowsOutOfRangeException()
    {
        var p = new Period(5, (TimeUnit)999);
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => { var _ = p / 2; });
        Assert.Multiple(() =>
        {
            Assert.That(ex!.ParamName, Is.EqualTo("period"),
                $"Unexpected ParamName for invalid TimeUnit in {p}");
            Assert.That(ex.ActualValue, Is.EqualTo((TimeUnit)999),
                $"Unexpected ActualValue for invalid TimeUnit in {p}");
            Assert.That(ex.Message, Does.Contain("Unknown time units"),
                $"Unexpected exception message for invalid TimeUnit in {p}");
        });
    }

    [Test]
    [TestCase(5, TimeUnit.Years, 3, TimeUnit.Years, 2, TimeUnit.Years)]
    [TestCase(12, TimeUnit.Months, 6, TimeUnit.Months, 6, TimeUnit.Months)]
    [TestCase(10, TimeUnit.Days, 4, TimeUnit.Days, 6, TimeUnit.Days)]
    [TestCase(2, TimeUnit.Years, 6, TimeUnit.Months, 18, TimeUnit.Months)]
    [TestCase(6, TimeUnit.Months, 2, TimeUnit.Years, -18, TimeUnit.Months)]
    [TestCase(2, TimeUnit.Weeks, 3, TimeUnit.Days, 11, TimeUnit.Days)]
    [TestCase(3, TimeUnit.Days, 2, TimeUnit.Weeks, -11, TimeUnit.Days)]
    [TestCase(-2, TimeUnit.Years, -3, TimeUnit.Years, 1, TimeUnit.Years)]
    [TestCase(-5, TimeUnit.Days, 3, TimeUnit.Days, -8, TimeUnit.Days)]
    [TestCase(5, TimeUnit.Days, -3, TimeUnit.Days, 8, TimeUnit.Days)]
    public void Test_OperatorMinus(
             int lhsLen, TimeUnit lhsUnit,
             int rhsLen, TimeUnit rhsUnit,
             int expectedLen, TimeUnit expectedUnit)
    {
        var lhs = new Period(lhsLen, lhsUnit);
        var rhs = new Period(rhsLen, rhsUnit);

        var result = lhs - rhs;

        Assert.Multiple(() =>
        {
            Assert.That(result.Length(), Is.EqualTo(expectedLen),
                $"lhs={lhs}, rhs={rhs} : Expected length {expectedLen}, but got {result.Length()}");
            Assert.That(result.Units(), Is.EqualTo(expectedUnit),
                $"lhs={lhs}, rhs={rhs} : Expected units {expectedUnit}, but got {result.Units()}");
        });
    }


    [Test]
    [TestCase(12, TimeUnit.Months, 3, 36, TimeUnit.Months)]
    [TestCase(1, TimeUnit.Years, 2, 2, TimeUnit.Years)]
    [TestCase(-2, TimeUnit.Weeks, 2, -4, TimeUnit.Weeks)]
    [TestCase(-10, TimeUnit.Days, 2, -20, TimeUnit.Days)]
    [TestCase(6, TimeUnit.Months, 3, 18, TimeUnit.Months)]
    public void Test_OperatorMultiply_PeriodTimesMultiplier(
        int length, TimeUnit unit,
        int multiplier,
        int expectedLength, TimeUnit expectedUnit)
    {
        var p = new Period(length, unit);
        var result = p * multiplier;

        Assert.Multiple(() =>
        {
            Assert.That(result.Length(), Is.EqualTo(expectedLength),
                $"Period({length}, {unit})*{multiplier} : Expected length {expectedLength}, but got {result.Length()}");
            Assert.That(result.Units(), Is.EqualTo(expectedUnit),
                $"Period({length}, {unit})*{multiplier} : Expected units {expectedUnit}, but got {result.Units()}");
        });
    }

    [Test]
    [TestCase(12, TimeUnit.Months, 3, 36, TimeUnit.Months)]
    [TestCase(1, TimeUnit.Years, 2, 2, TimeUnit.Years)]
    [TestCase(-2, TimeUnit.Weeks, 2, -4, TimeUnit.Weeks)]
    [TestCase(-10, TimeUnit.Days, 2, -20, TimeUnit.Days)]
    [TestCase(6, TimeUnit.Months, 3, 18, TimeUnit.Months)]
    public void Test_OperatorMultiply_MultiplierTimesPeriod(
           int length, TimeUnit unit,
           int multiplier,
           int expectedLength, TimeUnit expectedUnit)
    {
        var p = new Period(length, unit);
        var result = multiplier * p;

        Assert.Multiple(() =>
        {
            Assert.That(result.Length(), Is.EqualTo(expectedLength),
                $"Period({length}, {unit})*{multiplier} : Expected length {expectedLength}, but got {result.Length()}");
            Assert.That(result.Units(), Is.EqualTo(expectedUnit),
                $"Period({length}, {unit})*{multiplier} : Expected units {expectedUnit}, but got {result.Units()}");
        });
    }
}