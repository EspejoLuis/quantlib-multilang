using System.Runtime;
using NUnit.Framework;
using QuantLibCSharp.Time;

namespace QuantLibCSharp.UnitTests;

public class DateUnitTests
{

    // Constructors
    [Test]
    [TestCase(0)]
    public void Test_DefaultConstructor(int expectedSerial)
    {
        var d = new Date();
        Assert.That(d.SerialNumber(), Is.EqualTo(expectedSerial),
            $"Default constructor : Expected Serial={expectedSerial}, but got {d.SerialNumber()}");
    }

    [Test]
    [TestCase(1, Month.January, 1901, 367)]
    [TestCase(31, Month.December, 2199, 109574)]
    [TestCase(29, Month.February, 2000, 36585)]
    [TestCase(28, Month.February, 2001, 36950)]
    [TestCase(15, Month.July, 2024, 45488)]
    [TestCase(31, Month.December, 2000, 36891)]
    [TestCase(31, Month.December, 2001, 37256)]
    public void Test_Constructor_DayMonthYear(int day, Month month, int year, int expectedSerial)
    {
        var d = new Date(day, month, year);

        Assert.Multiple(() =>
        {
            Assert.That(d.SerialNumber(), Is.EqualTo(expectedSerial),
                $"Date({day},{month},{year}) : Expected Serial={expectedSerial}, but got {d.SerialNumber()}");
            Assert.That(d.Day(), Is.EqualTo(day),
                $"Date({day},{month},{year}) : Expected Day={day}, but got {d.Day()}");
            Assert.That(d.Month(), Is.EqualTo(month),
                $"Date({day},{month},{year}) : Expected Month={month}, but got {d.Month()}");
            Assert.That(d.Year(), Is.EqualTo(year),
                $"Date({day},{month},{year}) : Expected Year={year}, but got {d.Year()}");
        });
    }

    [Test]
    [TestCase(1, Month.January, 1900,
        "year", 1900, "Year out of range [1901,2199]")]
    [TestCase(1, Month.January, 2200,
        "year", 2200, "Year out of range [1901,2199]")]

    [TestCase(1, (Month)0, 2000,
        "month", (Month)0, "Month out of January-December range i.e. not in [1,12]")]
    [TestCase(1, (Month)13, 2000,
        "month", (Month)13, "Month out of January-December range i.e. not in [1,12]")]

    [TestCase(30, Month.February, 2001,
        "day", 30, "Day 30 outside month February")]
    [TestCase(32, Month.January, 2001,
        "day", 32, "Day 32 outside month January")]
    [TestCase(0, Month.January, 2001,
        "day", 0, "Day 0 outside month January")]
    public void Test_Constructor_DayMonthYear_ThrowsException(
        int day, Month month, int year,
        string expectedParamName, object expectedValue, string expectedMessageFragment)
    {
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => new Date(day, month, year));

        Assert.Multiple(() =>
        {
            Assert.That(ex!.ParamName, Is.EqualTo(expectedParamName),
                $"For Date({day},{month},{year}), expected ParamName='{expectedParamName}' but got '{ex.ParamName}'");

            Assert.That(ex.ActualValue, Is.EqualTo(expectedValue),
                $"For Date({day},{month},{year}), expected ActualValue={expectedValue} but got {ex.ActualValue}");

            Assert.That(ex.Message, Does.Contain(expectedMessageFragment),
                $"For Date({day},{month},{year}), expected message to contain '{expectedMessageFragment}' but got '{ex.Message}'");
        });
    }

    [Test]
    [TestCase(367, 1, Month.January, 1901)]
    [TestCase(109574, 31, Month.December, 2199)]
    [TestCase(36585, 29, Month.February, 2000)]
    [TestCase(36950, 28, Month.February, 2001)]
    [TestCase(36891, 31, Month.December, 2000)]
    [TestCase(37256, 31, Month.December, 2001)]
    public void Test_Constructor_SerialNumber(int serial, int expectedDay, Month expectedMonth, int expectedYear)
    {
        var d = new Date(serial);

        Assert.Multiple(() =>
        {
            Assert.That(d.SerialNumber(), Is.EqualTo(serial),
                $"Serial {serial} : Expected SerialNumber={serial}, but got {d.SerialNumber()}");
            Assert.That(d.Day(), Is.EqualTo(expectedDay),
                $"Serial {serial} : Expected Day={expectedDay}, but got {d.Day()}");
            Assert.That(d.Month(), Is.EqualTo(expectedMonth),
                $"Serial {serial} : Expected Month={expectedMonth}, but got {d.Month()}");
            Assert.That(d.Year(), Is.EqualTo(expectedYear),
                $"Serial {serial} : Expected Year={expectedYear}, but got {d.Year()}");
        });
    }

    [Test]
    [TestCase(366)]
    [TestCase(109575)]
    public void Test_Constructor_SerialNumber_ThrowsException(int serial)
    {
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => new Date(serial));
        Assert.That(ex!.Message, Does.Contain("Serial"),
            $"Expected exception message for invalid Serial={serial}");
    }

    // Internal
    [Test]
    [TestCase(Month.January, false, 0)]
    [TestCase(Month.February, false, 31)]
    [TestCase(Month.March, false, 59)]
    [TestCase(Month.December, false, 334)]
    [TestCase(Month.January, true, 0)]
    [TestCase(Month.February, true, 31)]
    [TestCase(Month.March, true, 60)]
    [TestCase(Month.December, true, 335)]
    public void Test_MonthOffSet(Month month, bool isLeap, int expectedOffset)
    {
        var result = Date.MonthOffSet(month, isLeap);
        Assert.That(result, Is.EqualTo(expectedOffset),
            $"MonthOffSet({month}, {isLeap}) : Expected {expectedOffset}, but got {result}");
    }

    [Test]
    [TestCase(Month.January, false, 31)]
    [TestCase(Month.February, false, 28)]
    [TestCase(Month.March, false, 31)]
    [TestCase(Month.April, false, 30)]
    [TestCase(Month.December, false, 31)]
    [TestCase(Month.February, true, 29)]
    public void Test_MonthLength(Month month, bool isLeap, int expectedLength)
    {
        var result = Date.MonthLength(month, isLeap);
        Assert.That(result, Is.EqualTo(expectedLength),
            $"MonthLength({month},{isLeap}) : Expected {expectedLength}, but got {result}");
    }

    [Test]
    [TestCase(2000, true)]
    [TestCase(2001, false)]
    [TestCase(1900, true)]
    [TestCase(2100, false)]
    public void Test_IsLeap(int year, bool expected)
    {
        var result = Date.IsLeap(year);
        Assert.That(result, Is.EqualTo(expected),
            $"IsLeap({year}) : Expected {expected}, but got {result}");
    }

    [Test]
    [TestCase(1899)]
    [TestCase(2201)]
    public void Test_IsLeap_ThrowsException(int year)
    {
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => Date.IsLeap(year));
        Assert.That(ex!.Message, Does.Contain("Year out of range"),
            $"Expected exception message for IsLeap({year})");
    }

    [Test]
    [TestCase(1900, 0)]
    [TestCase(1901, 366)]
    [TestCase(2000, 36525)]
    [TestCase(2199, 109209)]
    public void Test_YearOffSet(int year, int expectedOffset)
    {
        var result = Date.YearOffSet(year);
        Assert.That(result, Is.EqualTo(expectedOffset),
            $"YearOffSet({year}) : Expected {expectedOffset}, but got {result}");
    }

    [Test]
    [TestCase(1899)]
    [TestCase(2201)]
    public void Test_YearOffSet_ThrowsException(int year)
    {
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => Date.YearOffSet(year));
        Assert.That(ex!.Message, Does.Contain("Year out of range"),
            $"Expected exception message for YearOffSet({year})");
    }

    [Test]
    [TestCase(2000, 366)]
    [TestCase(2001, 365)]
    [TestCase(1900, 366)]
    [TestCase(2100, 365)]
    public void Test_YearLength(int year, int expectedLength)
    {
        var result = Date.YearLength(year);
        Assert.That(result, Is.EqualTo(expectedLength),
            $"YearLength({year}) : Expected {expectedLength}, but got {result}");
    }

    [Test]
    [TestCase(367)]
    [TestCase(109574)]
    [TestCase(36585)]
    public void Test_CheckSerialNumber(int serial)
    {
        Assert.DoesNotThrow(() => Date.CheckSerialNumber(serial),
            $"CheckSerialNumber({serial}) unexpectedly threw an exception");
    }

    [Test]
    [TestCase(366)]
    [TestCase(109575)]
    public void Test_CheckSerialNumber_ThrowsException(int serial)
    {
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => Date.CheckSerialNumber(serial));
        Assert.That(ex!.Message, Does.Contain("Serial"),
            $"Expected exception message for CheckSerialNumber({serial})");
    }

    // Static Helpers (Public)
    [Test]
    public void Test_MinDate()
    {
        var min = Date.MinDate();
        Assert.Multiple(() =>
        {
            Assert.That(min.SerialNumber(), Is.EqualTo(367), "MinDate SerialNumber mismatch");
            Assert.That(min.Day(), Is.EqualTo(1), "MinDate Day mismatch");
            Assert.That(min.Month(), Is.EqualTo(Month.January), "MinDate Month mismatch");
            Assert.That(min.Year(), Is.EqualTo(1901), "MinDate Year mismatch");
        });
    }

    [Test]
    public void Test_MaxDate()
    {
        var max = Date.MaxDate();
        Assert.Multiple(() =>
        {
            Assert.That(max.SerialNumber(), Is.EqualTo(109574), "MaxDate SerialNumber mismatch");
            Assert.That(max.Day(), Is.EqualTo(31), "MaxDate Day mismatch");
            Assert.That(max.Month(), Is.EqualTo(Month.December), "MaxDate Month mismatch");
            Assert.That(max.Year(), Is.EqualTo(2199), "MaxDate Year mismatch");
        });
    }

    [Test]
    public void Test_TodaysDate()
    {
        var todaySystem = DateTime.Today;
        var todayDate = Date.TodaysDate();

        Assert.Multiple(() =>
        {
            Assert.That(todayDate.Day(), Is.EqualTo(todaySystem.Day),
                $"TodaysDate : Expected Day={todaySystem.Day}, but got {todayDate.Day()}");
            Assert.That(todayDate.Month(), Is.EqualTo((Month)todaySystem.Month),
                $"TodaysDate : Expected Month={(Month)todaySystem.Month}, but got {todayDate.Month()}");
            Assert.That(todayDate.Year(), Is.EqualTo(todaySystem.Year),
                $"TodaysDate : Expected Year={todaySystem.Year}, but got {todayDate.Year()}");
        });
    }

    // Inspectors (Public) 
    [Test]
    [TestCase(367, 1)]
    [TestCase(36585, 29)]
    [TestCase(36950, 28)]
    [TestCase(36891, 31)]
    [TestCase(37256, 31)]
    public void Test_DayOfMonth(int serial, int expectedDay)
    {
        var d = new Date(serial);
        Assert.That(d.DayOfMonth(), Is.EqualTo(expectedDay),
            $"Serial {serial} : Expected DayOfMonth={expectedDay}, but got {d.DayOfMonth()}");
    }

    [Test]
    [TestCase(367, 1)]
    [TestCase(36585, 60)]
    [TestCase(36950, 59)]
    [TestCase(36891, 366)]
    [TestCase(37256, 365)]
    public void Test_DayOfYear(int serial, int expectedDayOfYear)
    {
        var d = new Date(serial);
        Assert.That(d.DayOfYear(), Is.EqualTo(expectedDayOfYear),
            $"Serial {serial} : Expected DayOfYear={expectedDayOfYear}, but got {d.DayOfYear()}");
    }

    [Test]
    [TestCase(367, Month.January)]
    [TestCase(36585, Month.February)]
    [TestCase(36950, Month.February)]
    [TestCase(36891, Month.December)]
    [TestCase(37256, Month.December)]
    public void Test_Month(int serial, Month expectedMonth)
    {
        var d = new Date(serial);
        Assert.That(d.Month(), Is.EqualTo(expectedMonth),
            $"Serial {serial} : Expected Month={expectedMonth}, but got {d.Month()}");
    }

    [Test]
    [TestCase(367, 1901)]
    [TestCase(36585, 2000)]
    [TestCase(36950, 2001)]
    [TestCase(36891, 2000)]
    [TestCase(37256, 2001)]
    public void Test_Year(int serial, int expectedYear)
    {
        var d = new Date(serial);
        Assert.That(d.Year(), Is.EqualTo(expectedYear),
            $"Serial {serial} : Expected Year={expectedYear}, but got {d.Year()}");
    }

    [Test]
    [TestCase(367)]
    [TestCase(36585)]
    [TestCase(36950)]
    [TestCase(109574)]
    public void Test_SerialNumber(int serial)
    {
        var d = new Date(serial);
        Assert.That(d.SerialNumber(), Is.EqualTo(serial),
            $"Expected Serial={serial}, but got {d.SerialNumber()}");
    }

    [Test]
    [TestCase(36585, true)]
    [TestCase(36950, true)]
    [TestCase(36584, false)]
    [TestCase(36891, true)]
    [TestCase(36634, false)]
    public void Test_IsEndOfMonth(int serial, bool expected)
    {
        var d = new Date(serial);
        Assert.That(d.IsEndOfMonth(), Is.EqualTo(expected),
            $"Serial {serial} : Expected IsEndOfMonth={expected}, but got {d.IsEndOfMonth()}");
    }

    [Test]
    [TestCase(36571, 36585)]
    [TestCase(36936, 36950)]
    [TestCase(36634, 36646)]
    [TestCase(36861, 36891)]
    public void Test_EndOfMonth(int serial, int expectedEndSerial)
    {
        var d = new Date(serial);
        var eom = d.EndOfMonth();

        Assert.Multiple(() =>
        {
            Assert.That(eom.SerialNumber(), Is.EqualTo(expectedEndSerial),
                $"Serial {serial} : Expected EndOfMonth Serial={expectedEndSerial}, but got {eom.SerialNumber()}");
            Assert.That(eom.IsEndOfMonth(), Is.True,
                $"Serial {serial} : Expected EndOfMonth to be true, but got false");
        });
    }
}
