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
        Assert.That(d.SerialNumber(), Is.EqualTo(expectedSerial), "SerialNumber");
    }

    [Test]
    [TestCase(1, Month.January, 1901, 367)]          // min allowed
    [TestCase(31, Month.December, 2199, 109574)]     // max allowed
    [TestCase(29, Month.February, 2000, 36585)]      // leap year Feb 29
    [TestCase(28, Month.February, 2001, 36950)]      // non-leap Feb 28
    [TestCase(15, Month.July, 2024, 45488)]          // mid-year
    [TestCase(31, Month.December, 2000, 36891)]      // leap year end (fixed)
    [TestCase(31, Month.December, 2001, 37256)]      // non-leap year end (fixed)
    public void Test_Constructor_DayMonthYear(int day, Month month, int year, int expectedSerial)
    {
        var d = new Date(day, month, year);

        Assert.Multiple(() =>
        {
            Assert.That(d.SerialNumber(), Is.EqualTo(expectedSerial), "SerialNumber");
            Assert.That(d.Day(), Is.EqualTo(day), "Day");
            Assert.That(d.Month(), Is.EqualTo(month), "Month");
            Assert.That(d.Year(), Is.EqualTo(year), "Year");
        });
    }

    [Test]
    [TestCase(1, Month.January, 1900)]   // Year too low
    [TestCase(1, Month.January, 2200)]   // Year too high
    [TestCase(1, (Month)0, 2000)]        // Month too low
    [TestCase(1, (Month)13, 2000)]       // Month too high
    [TestCase(30, Month.February, 2001)] // Invalid day (non-leap Feb)
    [TestCase(32, Month.January, 2001)]  // Invalid day (too high)
    [TestCase(0, Month.January, 2001)]   // Invalid day (too low)
    public void Test_Constructor_DayMonthYear_ThrowsException(int day, Month month, int year)
    {
        Assert.Throws<ArgumentOutOfRangeException>(() => new Date(day, month, year));
    }

    [Test]
    [TestCase(367, 1, Month.January, 1901)]          // min serial
    [TestCase(109574, 31, Month.December, 2199)]     // max serial
    [TestCase(36585, 29, Month.February, 2000)]      // leap year Feb 29
    [TestCase(36950, 28, Month.February, 2001)]      // non-leap Feb 28
    [TestCase(36891, 31, Month.December, 2000)]      // leap year end (fixed)
    [TestCase(37256, 31, Month.December, 2001)]      // non-leap year end (fixed)
    public void Test_Constructor_SerialNumber(int serial, int expectedDay, Month expectedMonth, int expectedYear)
    {
        var d = new Date(serial);

        Assert.Multiple(() =>
        {
            Assert.That(d.SerialNumber(), Is.EqualTo(serial), "SerialNumber");
            Assert.That(d.Day(), Is.EqualTo(expectedDay), "Day");
            Assert.That(d.Month(), Is.EqualTo(expectedMonth), "Month");
            Assert.That(d.Year(), Is.EqualTo(expectedYear), "Year");
        });
    }

    [Test]
    [TestCase(366)]      // Serial too low
    [TestCase(109575)]   // Serial too high
    public void Test_Constructor_SerialNumber_ThrowsException(int serial)
    {
        Assert.Throws<ArgumentOutOfRangeException>(() => new Date(serial));
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
        Assert.That(Date.MonthOffSet(month, isLeap), Is.EqualTo(expectedOffset));
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
        Assert.That(Date.MonthLength(month, isLeap), Is.EqualTo(expectedLength));
    }

    [Test]
    [TestCase(2000, true)]  // leap
    [TestCase(2001, false)] // non-leap
    [TestCase(1900, true)]  // Excel bug (treated as leap)
    [TestCase(2100, false)] // non-leap
    public void Test_IsLeap(int year, bool expected)
    {
        Assert.That(Date.IsLeap(year), Is.EqualTo(expected));
    }

    [Test]
    [TestCase(1899)]
    [TestCase(2201)]
    public void Test_IsLeap_ThrowsException(int year)
    {
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => Date.IsLeap(year));
        Assert.That(ex!.Message, Does.Contain("Year out of range [1900,2200]"));
    }


    [Test]
    [TestCase(1900, 0)]
    [TestCase(1901, 366)]
    [TestCase(2000, 36525)]
    [TestCase(2199, 109209)]  // last year start
    public void Test_YearOffSet(int year, int expectedOffset)
    {
        Assert.That(Date.YearOffSet(year), Is.EqualTo(expectedOffset));
    }

    [Test]
    [TestCase(1899)]
    [TestCase(2201)]
    public void Test_YearOffSet_ThrowsException(int year)
    {
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => Date.YearOffSet(year));
        Assert.That(ex!.Message, Does.Contain("Year out of range [1900,2200]"));
    }

    [Test]
    [TestCase(2000, 366)] // leap
    [TestCase(2001, 365)] // non-leap
    [TestCase(1900, 366)] // Excel bug
    [TestCase(2100, 365)] // non-leap
    public void Test_YearLength(int year, int expectedLength)
    {
        Assert.That(Date.YearLength(year), Is.EqualTo(expectedLength));
    }

    [Test]
    [TestCase(367)]     // min
    [TestCase(109574)]  // max
    [TestCase(36585)]   // valid mid-range
    public void Test_CheckSerialNumber(int serial)
    {
        Assert.DoesNotThrow(() => Date.CheckSerialNumber(serial));
    }

    [Test]
    [TestCase(366)]
    [TestCase(109575)]
    public void Test_CheckSerialNumber_ThrowsException(int serial)
    {
        var ex = Assert.Throws<ArgumentOutOfRangeException>(() => Date.CheckSerialNumber(serial));
        Assert.That(ex!.Message, Does.Contain("Serial"));
    }



    // Static Helpers (Public)
    [Test]
    public void Test_MinDate()
    {
        Assert.Multiple(() =>
        {
            var min = Date.MinDate();
            Assert.That(min.SerialNumber(), Is.EqualTo(367), "MinDate SerialNumber");
            Assert.That(min.Day(), Is.EqualTo(1), "MinDate Day");
            Assert.That(min.Month(), Is.EqualTo(Month.January), "MinDate Month");
            Assert.That(min.Year(), Is.EqualTo(1901), "MinDate Year");

        });
    }

    [Test]
    public void Test_MaxDate()
    {
        Assert.Multiple(() =>
        {
            var max = Date.MaxDate();
            Assert.That(max.SerialNumber(), Is.EqualTo(109574), "MaxDate SerialNumber");
            Assert.That(max.Day(), Is.EqualTo(31), "MaxDate Day");
            Assert.That(max.Month(), Is.EqualTo(Month.December), "MaxDate Month");
            Assert.That(max.Year(), Is.EqualTo(2199), "MaxDate Year");
        });
    }

    [Test]
    public void Test_TodaysDate()
    {
        var todaySystem = DateTime.Today;
        var todayDate = Date.TodaysDate();

        Assert.Multiple(() =>
        {
            Assert.That(todayDate.Day(), Is.EqualTo(todaySystem.Day), "Day");
            Assert.That(todayDate.Month(), Is.EqualTo((Month)todaySystem.Month), "Month");
            Assert.That(todayDate.Year(), Is.EqualTo(todaySystem.Year), "Year");
        });
    }



    // Inspectors (Public) 
    [Test]
    [TestCase(367, 1)]           // 1901-01-01
    [TestCase(36585, 29)]        // 2000-02-29 (leap)
    [TestCase(36950, 28)]        // 2001-02-28 (non-leap)
    [TestCase(36891, 31)]        // 2000-12-31 (leap year end)
    [TestCase(37256, 31)]        // 2001-12-31 (non-leap year end)
    public void Test_DayOfMonth(int serial, int expectedDay)
    {
        var d = new Date(serial);
        Assert.That(d.DayOfMonth(), Is.EqualTo(expectedDay), $"Serial {serial}");
    }

    [Test]
    [TestCase(367, 1)]           // 1901-01-01
    [TestCase(36585, 60)]        // 2000-02-29 (leap → 60th day)
    [TestCase(36950, 59)]        // 2001-02-28 (non-leap → 59th day)
    [TestCase(36891, 366)]       // 2000-12-31 (leap year end)
    [TestCase(37256, 365)]       // 2001-12-31 (non-leap year end)
    public void Test_DayOfYear(int serial, int expectedDayOfYear)
    {
        var d = new Date(serial);
        Assert.That(d.DayOfYear(), Is.EqualTo(expectedDayOfYear), $"Serial {serial}");
    }

    [Test]
    [TestCase(367, Month.January)]      // 1901-01-01
    [TestCase(36585, Month.February)]   // 2000-02-29
    [TestCase(36950, Month.February)]   // 2001-02-28
    [TestCase(36891, Month.December)]   // 2000-12-31
    [TestCase(37256, Month.December)]   // 2001-12-31
    public void Test_Month(int serial, Month expectedMonth)
    {
        var d = new Date(serial);
        Assert.That(d.Month(), Is.EqualTo(expectedMonth), $"Serial {serial}");
    }

    [Test]
    [TestCase(367, 1901)]      // min
    [TestCase(36585, 2000)]    // leap year
    [TestCase(36950, 2001)]    // non-leap year
    [TestCase(36891, 2000)]    // leap year end
    [TestCase(37256, 2001)]    // non-leap year end
    public void Test_Year(int serial, int expectedYear)
    {
        var d = new Date(serial);
        Assert.That(d.Year(), Is.EqualTo(expectedYear), $"Serial {serial}");
    }

    [Test]
    [TestCase(367)]
    [TestCase(36585)]
    [TestCase(36950)]
    [TestCase(109574)]
    public void Test_SerialNumber(int serial)
    {
        var d = new Date(serial);
        Assert.That(d.SerialNumber(), Is.EqualTo(serial), $"Serial {serial}");
    }

    [Test]
    [TestCase(36585, true)]   // 2000-02-29 leap end
    [TestCase(36950, true)]   // 2001-02-28 non-leap end
    [TestCase(36584, false)]  // 2000-02-28 not end
    [TestCase(36891, true)]   // 2000-12-31
    [TestCase(36634, false)]  // 2000-07-15
    public void Test_IsEndOfMonth(int serial, bool expected)
    {
        var d = new Date(serial);
        Assert.That(d.IsEndOfMonth(), Is.EqualTo(expected), $"Serial {serial}");
    }

    [Test]
    [TestCase(36571, 36585)]  // 2000-02-15 → 2000-02-29
    [TestCase(36936, 36950)]  // 2001-02-15 → 2001-02-28
    [TestCase(36634, 36646)]  // 2000-07-15 → 2000-07-31
    [TestCase(36861, 36891)]  // 2000-12-01 → 2000-12-31
    public void Test_EndOfMonth(int serial, int expectedEndSerial)
    {
        var d = new Date(serial);
        var eom = d.EndOfMonth();

        Assert.Multiple(() =>
        {
            Assert.That(eom.SerialNumber(), Is.EqualTo(expectedEndSerial), "SerialNumber");
            Assert.That(eom.IsEndOfMonth(), Is.True, "Should be end of month");
        });
    }


}
