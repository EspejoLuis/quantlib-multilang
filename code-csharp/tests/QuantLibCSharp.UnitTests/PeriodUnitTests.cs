using QuantLibCSharp.Time;
using QuantLibCSharp.Time.IO;

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
    public void Test_Operator_Plus(
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
    public void Test_Operator_Plus_ThrowsOnInvalidCombinations(
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
    public void Test_Operator_Plus_ThrowsOnUnknownTimeUnits(
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
    public void Test_Unary_Operator_Minus(
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
    public void Test_Operator_Divide(
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
    public void Test_Operator_Divide_ThrowsInvalidOperationException(
        int length, TimeUnit unit, int divider)
    {
        var p = new Period(length, unit);
        var ex = Assert.Throws<InvalidOperationException>(() => { var _ = p / divider; });
        Assert.That(ex!.Message, Is.EqualTo($"{p} cannot be divided by {divider}"),
            $"Unexpected exception message for Period({length},{unit})/{divider}");
    }

    [Test]
    public void Test_Operator_Divide_ThrowsDivideByZeroException()
    {
        var p = new Period(5, TimeUnit.Years);
        var ex = Assert.Throws<DivideByZeroException>(() => { var _ = p / 0; });
        Assert.That(ex!.Message, Is.EqualTo("Period cannot be divided by zero"),
            $"Unexpected exception message for Period({p.Length()},{p.Units()})/0");
    }

    [Test]
    public void Test_Operator_Divide_ThrowsOutOfRangeException()
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
    public void Test_Operator_Minus(
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
    public void Test_Operator_Multiply_PeriodTimesMultiplier(
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
    public void Test_Operator_Multiply_MultiplierTimesPeriod(
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


    [Test]
    [TestCase(5, TimeUnit.Days, 5, 5)]
    [TestCase(3, TimeUnit.Weeks, 21, 21)]
    [TestCase(1, TimeUnit.Months, 28, 31)]
    [TestCase(2, TimeUnit.Years, 730, 732)]
    public void Test_DaysMinMax(int length, TimeUnit unit, int expectedMin, int expectedMax)
    {
        var period = new Period(length, unit);
        var (min, max) = Period.DaysMinMax(period);
        Assert.Multiple(() =>
        {
            Assert.That(min, Is.EqualTo(expectedMin),
             $"Period({length}, {unit}) : Expected min length {expectedMin}, but got {min}");
            Assert.That(max, Is.EqualTo(expectedMax),
             $"Period({length}, {unit}) : Expected max length {expectedMax}, but got {max}");
        });
    }

    [Test]
    public void Test_DaysMinMax_ThrowsOutOfRangeException()
    {
        var p = new Period(5, (TimeUnit)999);
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => { var _ = Period.DaysMinMax(p); });
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
    // Special Cases
    [TestCase(0, TimeUnit.Years, -2, TimeUnit.Years, false)]
    [TestCase(0, TimeUnit.Years, 2, TimeUnit.Years, true)]
    [TestCase(1, TimeUnit.Years, 0, TimeUnit.Years, false)]
    [TestCase(-1, TimeUnit.Years, 0, TimeUnit.Years, true)]
    // Exact Cases - Same unit
    [TestCase(-1, TimeUnit.Years, 1, TimeUnit.Years, true)]
    [TestCase(10, TimeUnit.Years, 9, TimeUnit.Years, false)]
    // Exact Cases - Months vs Years
    [TestCase(12, TimeUnit.Months, 2, TimeUnit.Years, true)]
    [TestCase(-36, TimeUnit.Months, -4, TimeUnit.Years, false)]
    [TestCase(-36, TimeUnit.Months, -3, TimeUnit.Years, false)] // Equality is false
    [TestCase(-36, TimeUnit.Months, -36, TimeUnit.Months, false)] // Equality is false
    // Exact Cases - Years vs Month
    [TestCase(3, TimeUnit.Years, 37, TimeUnit.Months, true)]
    [TestCase(-4, TimeUnit.Years, -56, TimeUnit.Months, false)]
    // Exact Cases - Weeks vs Days
    [TestCase(2, TimeUnit.Weeks, 15, TimeUnit.Days, true)]
    [TestCase(-3, TimeUnit.Weeks, -24, TimeUnit.Days, false)]
    [TestCase(-3, TimeUnit.Weeks, -21, TimeUnit.Days, false)] // Equality is false
    [TestCase(-3, TimeUnit.Weeks, -3, TimeUnit.Weeks, false)] // Equality is false
    // Exact Cases - Days vs Weeks
    [TestCase(23, TimeUnit.Days, 4, TimeUnit.Weeks, true)]
    [TestCase(-28, TimeUnit.Days, -5, TimeUnit.Weeks, false)]
    // Inexact comparison -> maxLhs < minRhs : minLhs=28,maxLhs=31 ; minRhs=40,maxRhs=40 : 31 < 40
    [TestCase(1, TimeUnit.Months, 40, TimeUnit.Days, true)]
    // Inexact comparison -> minLhs > maxRhs: minLhs=maxLhs=40 ; minRhs=28,maxRhs=31 : 40 > 31
    [TestCase(40, TimeUnit.Days, 1, TimeUnit.Months, false)]
    public void Test_Comparison_Lower(
        int lhsLength, TimeUnit lhsUnit,
        int rhsLength, TimeUnit rhsUnit,
        bool expectedResult)
    {
        Period lhs = new(lhsLength, lhsUnit);
        Period rhs = new(rhsLength, rhsUnit);
        bool result = lhs < rhs;
        Assert.That(result, Is.EqualTo(expectedResult),
        $"{lhs} expected to be lower than {rhs}");
    }

    [Test]
    public void Test_Comparison_Lower_ThrowsOnInvalidCombinations()
    {
        var lhs = new Period(1, TimeUnit.Months);
        var rhs = new Period(30, TimeUnit.Days);
        var ex = Assert.Throws<InvalidOperationException>(() => { var _ = lhs < rhs; });
        Assert.That(ex!.Message,
            Is.EqualTo($"Undecidable comparison between {lhs} and {rhs}"),
            $"Unexpected exception message for lhs={lhs}, rhs={rhs}");
    }

    [Test]
    // Special Cases
    [TestCase(0, TimeUnit.Years, -2, TimeUnit.Years, true)]
    [TestCase(0, TimeUnit.Years, 2, TimeUnit.Years, false)]
    [TestCase(1, TimeUnit.Years, 0, TimeUnit.Years, true)]
    [TestCase(-1, TimeUnit.Years, 0, TimeUnit.Years, false)]
    // Exact Cases - Same unit
    [TestCase(-1, TimeUnit.Years, 1, TimeUnit.Years, false)]
    [TestCase(10, TimeUnit.Years, 9, TimeUnit.Years, true)]
    // Exact Cases - Months vs Years
    [TestCase(12, TimeUnit.Months, 2, TimeUnit.Years, false)]
    [TestCase(-36, TimeUnit.Months, -4, TimeUnit.Years, true)]
    [TestCase(-36, TimeUnit.Months, -3, TimeUnit.Years, false)] // Equality is false
    [TestCase(-36, TimeUnit.Months, -36, TimeUnit.Months, false)] // Equality is false
    // Exact Cases - Years vs Month
    [TestCase(3, TimeUnit.Years, 37, TimeUnit.Months, false)]
    [TestCase(-4, TimeUnit.Years, -56, TimeUnit.Months, true)]
    // Exact Cases - Weeks vs Days
    [TestCase(2, TimeUnit.Weeks, 15, TimeUnit.Days, false)]
    [TestCase(-3, TimeUnit.Weeks, -24, TimeUnit.Days, true)]
    [TestCase(-3, TimeUnit.Weeks, -21, TimeUnit.Days, false)] // Equality is false
    [TestCase(-3, TimeUnit.Weeks, -3, TimeUnit.Weeks, false)] // Equality is false
    // Exact Cases - Days vs Weeks
    [TestCase(23, TimeUnit.Days, 4, TimeUnit.Weeks, false)]
    [TestCase(-28, TimeUnit.Days, -5, TimeUnit.Weeks, true)]
    // Inexact comparison -> maxLhs < minRhs : minLhs=28,maxLhs=31 ; minRhs=40,maxRhs=40 : 31 < 40
    [TestCase(1, TimeUnit.Months, 40, TimeUnit.Days, false)]
    // Inexact comparison -> minLhs > maxRhs: minLhs=maxLhs=40 ; minRhs=28,maxRhs=31 : 40 > 31
    [TestCase(40, TimeUnit.Days, 1, TimeUnit.Months, true)]
    public void Test_Comparison_Greater(
        int lhsLength, TimeUnit lhsUnit,
        int rhsLength, TimeUnit rhsUnit,
        bool expectedResult)
    {
        Period lhs = new(lhsLength, lhsUnit);
        Period rhs = new(rhsLength, rhsUnit);
        bool result = lhs > rhs;
        Assert.That(result, Is.EqualTo(expectedResult),
        $"{lhs} expected to be greater than {rhs}"); ;
    }

    [Test]
    public void Test_Comparison_Greater_ThrowsOnInvalidCombinations()
    {
        var lhs = new Period(1, TimeUnit.Months);
        var rhs = new Period(30, TimeUnit.Days);
        var ex = Assert.Throws<InvalidOperationException>(() => { var _ = lhs > rhs; });
        Assert.That(ex!.Message,
            Is.EqualTo($"Undecidable comparison between {rhs} and {lhs}"),
            $"Unexpected exception message for {rhs}, {lhs}");
    }

    [Test]
    [TestCase(-2, TimeUnit.Years, -2, TimeUnit.Years, true)]
    [TestCase(2, TimeUnit.Years, 2, TimeUnit.Years, true)]
    [TestCase(-2, TimeUnit.Years, -1, TimeUnit.Years, false)]
    [TestCase(2, TimeUnit.Years, 1, TimeUnit.Years, false)]
    public void Test_Comparison_Equality(
        int lhsLength, TimeUnit lhsUnit,
        int rhsLength, TimeUnit rhsUnit,
        bool expectedResult)
    {
        Period lhs = new(lhsLength, lhsUnit);
        Period rhs = new(rhsLength, rhsUnit);
        bool result = lhs == rhs;
        Assert.That(result, Is.EqualTo(expectedResult),
        $"{lhs} expected to be equal to {rhs}");
    }

    [Test]
    public void Test_Equality_BothNull()
    {
        Period lhs = null!;
        Period rhs = null!;
        bool result = lhs == rhs;
        Assert.That(result, Is.True);
    }

    [Test]
    public void Test_Equality_LhsNull_RhsNotNull()
    {
        Period lhs = null!;
        Period rhs = new(1, TimeUnit.Years);
        bool result = lhs == rhs;
        Assert.That(result, Is.False);
    }

    [Test]
    public void Test_Equality_LhsNotNull_RhsNull()
    {
        Period lhs = new(1, TimeUnit.Years);
        Period rhs = null!;
        bool result = lhs == rhs;
        Assert.That(result, Is.False);
    }

    [Test]
    [TestCase(-2, TimeUnit.Years, -2, TimeUnit.Years, false)]
    [TestCase(2, TimeUnit.Years, 2, TimeUnit.Years, false)]
    [TestCase(-2, TimeUnit.Years, -3, TimeUnit.Years, true)]
    [TestCase(2, TimeUnit.Years, 1, TimeUnit.Years, true)]
    public void Test_Comparison_Difference(
        int lhsLength, TimeUnit lhsUnit,
        int rhsLength, TimeUnit rhsUnit,
        bool expectedResult)
    {
        Period lhs = new(lhsLength, lhsUnit);
        Period rhs = new(rhsLength, rhsUnit);
        bool result = lhs != rhs;
        Assert.That(result, Is.EqualTo(expectedResult),
        $"{lhs} expected to be equal to {rhs}");
    }

    [Test]
    public void Test_Comparison_Difference_BothNull()
    {
        Period lhs = null!;
        Period rhs = null!;
        bool result = lhs != rhs;
        Assert.That(result, Is.False);
    }

    [Test]
    public void Test_Comparison_Difference_LhsNull_RhsNotNull()
    {
        Period lhs = null!;
        Period rhs = new(1, TimeUnit.Years);
        bool result = lhs != rhs;
        Assert.That(result, Is.True);
    }

    [Test]
    public void Test_Comparison_Difference_LhsNotNull_RhsNull()
    {
        Period lhs = new(1, TimeUnit.Years);
        Period rhs = null!;
        bool result = lhs != rhs;
        Assert.That(result, Is.True);
    }

    [Test]
    public void Test_Equals_ObjectWithSameFields_ReturnsTrue()
    {
        Period p1 = new(12, TimeUnit.Months);
        object p2 = new Period(12, TimeUnit.Months);
        var result = p1.Equals(p2);
        Assert.That(result, Is.True);
    }

    [Test]
    public void Test_Equals_ObjectWithDifferentFields_ReturnsTrue()
    {
        Period p1 = new(12, TimeUnit.Months);
        object p2 = new Period(1, TimeUnit.Years);
        var result = p1.Equals(p2);
        Assert.That(result, Is.True);
    }

    [Test]
    public void Test_Equals_ObjectIsNull_ReturnsFalse()
    {
        Period p1 = new(1, TimeUnit.Years);
        object? p2 = null;
        var result = p1.Equals(p2);
        Assert.That(result, Is.False);
    }

    [Test]
    public void Test_Equals_ObjectIsOtherType_ReturnsFalse()
    {
        Period p1 = new(1, TimeUnit.Years);
        object p2 = "not a Period";
        var result = p1.Equals(p2);
        Assert.That(result, Is.False);
    }

    [Test]
    public void Test_GetHashCode_EqualObjects_SameHashCode()
    {
        Period p1 = new(12, TimeUnit.Months);
        Period p2 = new(12, TimeUnit.Months);
        Assert.That(p1.GetHashCode(), Is.EqualTo(p2.GetHashCode()));
    }

    [Test]
    // Special Cases
    [TestCase(0, TimeUnit.Years, -2, TimeUnit.Years, true)]
    [TestCase(0, TimeUnit.Years, 2, TimeUnit.Years, false)]
    [TestCase(1, TimeUnit.Years, 0, TimeUnit.Years, true)]
    [TestCase(-1, TimeUnit.Years, 0, TimeUnit.Years, false)]
    // Exact Cases - Same unit
    [TestCase(-1, TimeUnit.Years, 1, TimeUnit.Years, false)]
    [TestCase(10, TimeUnit.Years, 9, TimeUnit.Years, true)]
    // Exact Cases - Months vs Years
    [TestCase(12, TimeUnit.Months, 2, TimeUnit.Years, false)]
    [TestCase(-36, TimeUnit.Months, -4, TimeUnit.Years, true)]
    [TestCase(-36, TimeUnit.Months, -3, TimeUnit.Years, true)] // NORMALIZED! Equality true
    [TestCase(-36, TimeUnit.Months, -36, TimeUnit.Months, true)] // Equality is true 
    // Exact Cases - Years vs Month
    [TestCase(3, TimeUnit.Years, 37, TimeUnit.Months, false)]
    [TestCase(-4, TimeUnit.Years, -56, TimeUnit.Months, true)]
    // Exact Cases - Weeks vs Days
    [TestCase(2, TimeUnit.Weeks, 15, TimeUnit.Days, false)]
    [TestCase(-3, TimeUnit.Weeks, -24, TimeUnit.Days, true)]
    [TestCase(-3, TimeUnit.Weeks, -21, TimeUnit.Days, true)] // NORMALIZED! Equality true
    [TestCase(-3, TimeUnit.Weeks, -3, TimeUnit.Weeks, true)] // Equality is true 
    // Exact Cases - Days vs Weeks
    [TestCase(23, TimeUnit.Days, 4, TimeUnit.Weeks, false)]
    [TestCase(-28, TimeUnit.Days, -5, TimeUnit.Weeks, true)]
    // Inexact comparison -> maxLhs < minRhs : minLhs=28,maxLhs=31 ; minRhs=40,maxRhs=40 : 31 < 40
    [TestCase(1, TimeUnit.Months, 40, TimeUnit.Days, false)]
    // Inexact comparison -> minLhs > maxRhs: minLhs=maxLhs=40 ; minRhs=28,maxRhs=31 : 40 > 31
    [TestCase(40, TimeUnit.Days, 1, TimeUnit.Months, true)]
    public void Test_Comparison_GreaterEqual(
        int lhsLength, TimeUnit lhsUnit,
        int rhsLength, TimeUnit rhsUnit,
        bool expectedResult)
    {
        Period lhs = new(lhsLength, lhsUnit);
        Period rhs = new(rhsLength, rhsUnit);
        bool result = lhs >= rhs;
        Assert.That(result, Is.EqualTo(expectedResult),
        $"{lhs} expected to be greater or equal than {rhs}"); ;
    }

    [Test]
    public void Test_Comparison_GreaterEqual_ThrowsOnInvalidCombinations()
    {
        var lhs = new Period(1, TimeUnit.Months);
        var rhs = new Period(30, TimeUnit.Days);
        var ex = Assert.Throws<InvalidOperationException>(() => { var _ = lhs >= rhs; });
        Assert.That(ex!.Message,
            Is.EqualTo($"Undecidable comparison between {lhs} and {rhs}"),
            $"Unexpected exception message for lhs={lhs}, rhs={rhs}");
    }
    [Test]
    // Special Cases
    [TestCase(0, TimeUnit.Years, -2, TimeUnit.Years, false)]
    [TestCase(0, TimeUnit.Years, 2, TimeUnit.Years, true)]
    [TestCase(1, TimeUnit.Years, 0, TimeUnit.Years, false)]
    [TestCase(-1, TimeUnit.Years, 0, TimeUnit.Years, true)]
    // Exact Cases - Same unit
    [TestCase(-1, TimeUnit.Years, 1, TimeUnit.Years, true)]
    [TestCase(10, TimeUnit.Years, 9, TimeUnit.Years, false)]
    // Exact Cases - Months vs Years
    [TestCase(12, TimeUnit.Months, 2, TimeUnit.Years, true)]
    [TestCase(-36, TimeUnit.Months, -4, TimeUnit.Years, false)]
    [TestCase(-36, TimeUnit.Months, -3, TimeUnit.Years, true)] /// NORMALIZED! Equality true
    [TestCase(-36, TimeUnit.Months, -36, TimeUnit.Months, true)] // Equality true
    // Exact Cases - Years vs Month
    [TestCase(3, TimeUnit.Years, 37, TimeUnit.Months, true)]
    [TestCase(-4, TimeUnit.Years, -56, TimeUnit.Months, false)]
    // Exact Cases - Weeks vs Days
    [TestCase(2, TimeUnit.Weeks, 15, TimeUnit.Days, true)]
    [TestCase(-3, TimeUnit.Weeks, -24, TimeUnit.Days, false)]
    [TestCase(-3, TimeUnit.Weeks, -21, TimeUnit.Days, true)] // NORMALIZED! Equality true
    [TestCase(-3, TimeUnit.Weeks, -3, TimeUnit.Weeks, true)] // Equality true
    // Exact Cases - Days vs Weeks
    [TestCase(23, TimeUnit.Days, 4, TimeUnit.Weeks, true)]
    [TestCase(-28, TimeUnit.Days, -5, TimeUnit.Weeks, false)]
    // Inexact comparison -> maxLhs < minRhs : minLhs=28,maxLhs=31 ; minRhs=40,maxRhs=40 : 31 < 40
    [TestCase(1, TimeUnit.Months, 40, TimeUnit.Days, true)]
    // Inexact comparison -> minLhs > maxRhs: minLhs=maxLhs=40 ; minRhs=28,maxRhs=31 : 40 > 31
    [TestCase(40, TimeUnit.Days, 1, TimeUnit.Months, false)]
    public void Test_Comparison_LowerEqual(
           int lhsLength, TimeUnit lhsUnit,
           int rhsLength, TimeUnit rhsUnit,
           bool expectedResult)
    {
        Period lhs = new(lhsLength, lhsUnit);
        Period rhs = new(rhsLength, rhsUnit);
        bool result = lhs <= rhs;
        Assert.That(result, Is.EqualTo(expectedResult),
        $"{lhs} expected to be lower than {rhs}");
    }

    [Test]
    public void Test_Comparison_LowerEqual_ThrowsOnInvalidCombinations()
    {
        var lhs = new Period(1, TimeUnit.Months);
        var rhs = new Period(30, TimeUnit.Days);
        var ex = Assert.Throws<InvalidOperationException>(() => { var _ = lhs <= rhs; });
        Assert.That(ex!.Message,
            Is.EqualTo($"Undecidable comparison between {rhs} and {lhs}"),
            $"Unexpected exception message for {lhs}, {rhs}");
    }

    [Test]
    [TestCase(1, TimeUnit.Days, "1D")]
    [TestCase(2, TimeUnit.Days, "2D")]
    [TestCase(1, TimeUnit.Weeks, "1W")]
    [TestCase(3, TimeUnit.Weeks, "3W")]
    [TestCase(1, TimeUnit.Months, "1M")]
    [TestCase(4, TimeUnit.Months, "4M")]
    [TestCase(1, TimeUnit.Years, "1Y")]
    [TestCase(5, TimeUnit.Years, "5Y")]
    public void Test_ToString(
                int length, TimeUnit unit,
                string expectedShort)
    {
        var p = new Period(length, unit);
        Assert.That(p.ToString(), Is.EqualTo(expectedShort));
    }

    [Test]
    [TestCase(1, TimeUnit.Days, "1D")]
    [TestCase(2, TimeUnit.Days, "2D")]
    [TestCase(1, TimeUnit.Weeks, "1W")]
    [TestCase(3, TimeUnit.Weeks, "3W")]
    [TestCase(1, TimeUnit.Months, "1M")]
    [TestCase(4, TimeUnit.Months, "4M")]
    [TestCase(1, TimeUnit.Years, "1Y")]
    [TestCase(5, TimeUnit.Years, "5Y")]
    public void Test_ShortFormat(
                int length, TimeUnit unit,
                string expectedShort)
    {
        var p = new Period(length, unit);
        Assert.That(PeriodIO.ShortFormat(p), Is.EqualTo(expectedShort));
    }

    [Test]
    [TestCase(1, TimeUnit.Days, "1 Day")]
    [TestCase(2, TimeUnit.Days, "2 Days")]
    [TestCase(1, TimeUnit.Weeks, "1 Week")]
    [TestCase(3, TimeUnit.Weeks, "3 Weeks")]
    [TestCase(1, TimeUnit.Months, "1 Month")]
    [TestCase(4, TimeUnit.Months, "4 Months")]
    [TestCase(1, TimeUnit.Years, "1 Year")]
    [TestCase(5, TimeUnit.Years, "5 Years")]
    public void Test_LongFormat(
                int length, TimeUnit unit,
               string expectedLong)
    {
        var p = new Period(length, unit);
        Assert.That(PeriodIO.LongFormat(p), Is.EqualTo(expectedLong));
    }

    [Test]
    public void Test_ShortFormat_ThrowsArgumentOutOfRangeException()
    {
        var p = new Period(1, (TimeUnit)1223);
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => { var _ = PeriodIO.LongFormat(p); });
        Assert.Multiple(() =>
        {
            Assert.That(ex!.ParamName, Is.EqualTo("period"),
                $"Unexpected ParamName for invalid TimeUnit in {p}");
            Assert.That(ex.ActualValue, Is.EqualTo((TimeUnit)1223),
                $"Unexpected ActualValue for invalid TimeUnit in {p}");
            Assert.That(ex.Message, Does.Contain("Unknown time units"),
                $"Unexpected exception message for invalid TimeUnit in {p}");
        });
    }

    [Test]
    public void Test_LongFormat_ThrowsArgumentOutOfRangeException()
    {
        var p = new Period(1, (TimeUnit)1223);
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => { var _ = PeriodIO.ShortFormat(p); });
        Assert.Multiple(() =>
        {
            Assert.That(ex!.ParamName, Is.EqualTo("period"),
                $"Unexpected ParamName for invalid TimeUnit in {p}");
            Assert.That(ex.ActualValue, Is.EqualTo((TimeUnit)1223),
                $"Unexpected ActualValue for invalid TimeUnit in {p}");
            Assert.That(ex.Message, Does.Contain("Unknown time units"),
                $"Unexpected exception message for invalid TimeUnit in {p}");
        });
    }
}