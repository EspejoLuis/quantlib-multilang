using System.Runtime;
using NUnit.Framework;
using QuantLibCSharp.Time;

namespace QuantLibCSharp.UnitTests;

public class FrequencyUnitTests
{
    [Test]
    [TestCase(12, Frequency.Monthly)]
    [TestCase(6, Frequency.Bimonthly)]
    [TestCase(4, Frequency.Quarterly)]
    [TestCase(3, Frequency.EveryFourthMonth)]
    [TestCase(2, Frequency.Semiannual)]
    [TestCase(1, Frequency.Annual)]
    public void Test_FromNthTimesPerYear_Valid(int nthTimes, Frequency expected)
    {
        var freq = FrequencyUtils.FromNthTimesPerYear(nthTimes);
        Assert.That(freq, Is.EqualTo(expected), $"nThTimes={nthTimes}");
    }

    [Test]
    [TestCase(0)]
    [TestCase(5)]
    [TestCase(7)]
    [TestCase(8)]
    [TestCase(10)]
    [TestCase(100)]
    public void Test_FromNthTimesPerYear_Invalid(int nthTimes)
    {
        var freq = FrequencyUtils.FromNthTimesPerYear(nthTimes);
        Assert.That(freq, Is.EqualTo(Frequency.OtherFrequency), $"nThTimes={nthTimes}");
    }

}