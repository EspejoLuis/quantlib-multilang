using System.Collections.Generic;
using NUnit.Framework;
using QuantLibCSharp;

namespace QuantLibCSharp.IntegrationTests;

public class DateIntegrationTests
{
    [TestCase(14, Month.May, 1989, 11, 3, Month.May, 1989)] // Same-month simple back
    [TestCase(1, Month.March, 2023, 1, 28, Month.February, 2023)] // Month boundary back (non-leap)
    [TestCase(5, Month.March, 2023, 7, 26, Month.February, 2023)] // Month boundary back (non-leap)
    [TestCase(1, Month.March, 2024, 1, 29, Month.February, 2024)] // Month boundary back (leap)
    [TestCase(15, Month.March, 2024, 45, 30, Month.January, 2024)] // Month boundary back (leap)
    [TestCase(1, Month.March, 2023, 30, 30, Month.January, 2023)] // Two-month spans (non-leap vs leap contrast)
    [TestCase(1, Month.March, 2024, 30, 31, Month.January, 2024)] // Two-month spans (non-leap vs leap contrast)
    [TestCase(5, Month.January, 2025, 10, 26, Month.December, 2024)] // Year boundary back
    [TestCase(1, Month.January, 2000, 1, 31, Month.December, 1999)] // Year boundary back
    [TestCase(1, Month.January, 2024, 365, 1, Month.January, 2023)] // Larger jump across year boundary
    [TestCase(31, Month.March, 2024, 31, 29, Month.February, 2024)] // leap Feb
    [TestCase(31, Month.March, 2023, 31, 28, Month.February, 2023)] // non-leap Feb
    public void SubtractionOperator_SubtractsDaysCorrectly(
        int startDay, Month startMonth, int startYear,
        int daysToSubtract,
        int expDay, Month expMonth, int expYear)
    {
        //Arrange
        var d1 = new Date(startDay, startMonth, startYear);

        //Act
        var result = d1 - daysToSubtract;

        //Assert
        Assert.That(result.Day, Is.EqualTo(expDay));
        Assert.That(result.Month, Is.EqualTo(expMonth));
        Assert.That(result.Year, Is.EqualTo(expYear));
    }

    [TestCase(1, Month.January, 2025, 1, Month.January, 2025, 0)]// Same date
    [TestCase(19, Month.May, 1989, 14, Month.May, 1989, 5)]// Within month
    [TestCase(14, Month.May, 1989, 19, Month.May, 1989, -5)]// Within month
    [TestCase(1, Month.March, 2023, 28, Month.February, 2023, 1)]// Month boundary (non-leap)
    [TestCase(28, Month.February, 2023, 1, Month.March, 2023, -1)]// Month boundary (non-leap)
    [TestCase(1, Month.March, 2024, 29, Month.February, 2024, 1)]// Month boundary (leap)
    [TestCase(1, Month.March, 2024, 28, Month.February, 2024, 2)]// Month boundary (leap)
    [TestCase(29, Month.February, 2024, 1, Month.March, 2024, -1)]// Month boundary (leap)
    [TestCase(1, Month.January, 2000, 1, Month.January, 2000, 0)]// Year boundary
    [TestCase(1, Month.January, 2025, 31, Month.December, 2024, 1)]// Year boundary
    [TestCase(31, Month.December, 2024, 1, Month.January, 2025, -1)]// Year boundary
    [TestCase(1, Month.January, 2024, 1, Month.January, 2023, 365)]// 2023 non-leap
    [TestCase(1, Month.January, 2025, 1, Month.January, 2024, 366)]// 2024 leap
    [TestCase(15, Month.March, 2024, 30, Month.January, 2024, 45)]// 1(Jan31) + 29(Feb) + 15 = 45
    [TestCase(15, Month.March, 2023, 30, Month.January, 2023, 44)]// non-leap Feb=28
    [TestCase(30, Month.January, 2024, 15, Month.March, 2024, -45)]
    [TestCase(31, Month.March, 2024, 29, Month.February, 2024, 31)]// End-of-month checks
    [TestCase(31, Month.March, 2023, 28, Month.February, 2023, 31)]// End-of-month checks
    [TestCase(1, Month.March, 2024, 1, Month.March, 2023, 366)]// Same day across years (leap effect)
    public void SubtractionOperator_SubtractsDatesCorrectly(
        int lDay, Month lMonth, int lYear,
        int rDay, Month rMonth, int rYear,
        int expectedDelta)
    {
        //Arrange
        var left = new Date(lDay, lMonth, lYear);
        var right = new Date(rDay, rMonth, rYear);

        //Act
        var result = left - right;

        //Assert
        Assert.That(result, Is.EqualTo(expectedDelta));
    }

    [Test]
    public void HashSet_Contains_UsesValueEqualityAndHash()
    {
        var d1 = new Date(29, Month.February, 2024);
        var d2 = new Date(29, Month.February, 2024);

        var set = new HashSet<Date> { d1 };
        Assert.That(set.Contains(d2), Is.True); 
    }
    /*
    In UNIT TEST:
    List<object>.Contains(d2) uses EqualityComparer<object>.Default, 
    which ends up calling virtual object.Equals(object?) on the contained element.
    This will lead to overridden Equals(object?) in Date.
    
    → This verifies your override works and that boxing/upcasting still yields value equality.

    In INTEGRATION TEST:
    HashSet<Date> / Dictionary<Date, …> are different: those generic collections consult 
    EqualityComparer<Date>.Default, which prefers IEquatable<Date>.Equals(Date?) 
    and also requires a consistent GetHashCode().

    → That’s cross-component behavior (type + generic collections + hashing), so it belongs in integration.

    A List<object> doesn’t use hashing and doesn’t engage the IEquatable<Date> fast path. 
    */
    [Test]
    public void Dictionary_KeyLookup_UsesValueEqualityAndHash()
    {
        var key1 = new Date(31, Month.December, 2024);
        var key2 = new Date(31, Month.December, 2024); // different instance, same value

        var dict = new Dictionary<Date, string> { [key1] = "year-end" };

        Assert.Multiple(() =>
        {
            Assert.That(dict.ContainsKey(key2), Is.True, "ContainsKey should use value equality");
            Assert.That(dict.TryGetValue(key2, out var value), Is.True, "TryGetValue should succeed");
            Assert.That(value, Is.EqualTo("year-end"));
        });
    }
}
