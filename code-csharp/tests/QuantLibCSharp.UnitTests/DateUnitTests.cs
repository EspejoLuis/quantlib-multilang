using System.Runtime;
using NUnit.Framework;
using QuantLibCSharp.Time;

namespace QuantLibCSharp.UnitTests;

public class DateUnitTests
{
    [Test]
    [TestCase(0)]
    public void Test_DefaultConstructor(int expectedSerial)
    {
        var d = new Date();
        Assert.That(d.SerialNumber(), Is.EqualTo(expectedSerial), "SerialNumber");
    }

    [Test]
    [TestCase(1, Month.January, 1901, 367)]
    [TestCase(31, Month.December, 2199, 109574)]
    [TestCase(29, Month.February, 2000, 36585)]   // leap year
    [TestCase(28, Month.February, 2001, 36950)]   // non-leap year
    [TestCase(15, Month.July, 2024, 45488)]   // mid-year
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
    [TestCase(367, 1, Month.January, 1901)]
    [TestCase(109574, 31, Month.December, 2199)]
    [TestCase(36584, 28, Month.February, 2000)]
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

}
