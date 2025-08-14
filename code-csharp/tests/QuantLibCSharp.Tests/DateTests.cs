
using NUnit.Framework;
using QuantLibCSharp;

namespace QuantLibCSharp.Tests;

public class DataTests
{
    /*
    In C++, you need to create a default constructor otherwise
    it will not compile. But not in C#
    */
    //[Test]
    //public void DefaultConstuctor_SetsCorrectValues()
    //{
    //Arrange and Act
    //    var d = new Date();

    //Assert
    //    Assert.That(d.Day, Is.EqualTo(1));
    //    Assert.That(d.Month, Is.EqualTo(Month.January));
    //}
    [Test]
    public void Constructor_SetsCorrectValues()
    {
        //Arrange and Act
        var d = new Date(14, Month.May, 1989);

        //Assert   
        Assert.That(d.Day, Is.EqualTo(14));
        Assert.That(d.Month, Is.EqualTo(Month.May));
        Assert.That(d.Year, Is.EqualTo(1989));
    }
    [Test]
    public void ToString_IsCorrect()
    {
        //Arrange 
        var d = new Date(14, Month.May, 1989);
        //Act
        var result = d.ToString();
        //Assert
        Assert.That(result, Is.EqualTo("14-May-1989"));
    }
    [Test]
    public void EqualityOperator_SameDate_IsTrue()
    {
        //Arrange 
        var d1 = new Date(14, Month.May, 1989);
        var d2 = new Date(14, Month.May, 1989);
       
        //Act
        var result1 = d1 == d2;
       
        //Assert
        Assert.That(result1, Is.True);
    }
    [Test]
    public void EqualityOperator_SameDate_IsFalse()
    {
        //Arrange 
        var d1 = new Date(14, Month.May, 1989);
        var d2 = new Date(15, Month.May, 1989);

        //Act
        var result2 = d1 == d2;

        //Assert
        Assert.That(result2, Is.False);
    }
    [Test]
    public void EqualityOperator_SameDateReference_IsTrue()
    {
        //Arrange 
        var d1 = new Date(14, Month.May, 1989);
        var d2 = d1;
        //Act
        var result1 = d1 == d2;
       
        //Assert
        Assert.That(result1, Is.True);
    }
    [Test]
    public void EqualityOperator_OneDateNull_IsFalse()
    {
        //Arrange 
        var d1 = new Date(14, Month.May, 1989);

        //Act
        var secondDateNull = d1 == null;
        var firstDateNull = null == d1;

        //Assert
        Assert.That(firstDateNull, Is.False);
        Assert.That(secondDateNull, Is.False);
    }
    [Test]
    public void Equals_Object_ListContains_IsTrue() {
        var d1 = new Date(1, Month.January, 2025);
        var d2 = new Date(1, Month.January, 2025);
        var list = new List<object> { d1 };
        Assert.That(list.Contains(d2), Is.True); 
    }
    [Test]
    public void Equals_Object_SameValue_IsTrue() {
        var d1 = new Date(1, Month.January, 2025);
        object object2 = new Date(1, Month.January, 2025);
        Assert.That(d1.Equals(object2), Is.True);
    }
    [Test]
    public void Equals_Object_DifferentValue_IsFalse() {
        var d1 = new Date(1, Month.January, 2025);
        object object2 = new Date(2, Month.January, 2025);
        Assert.That(d1.Equals(object2), Is.False);
    }
    [Test]
    public void Equals_Object_Null_IsFalse() {
        var d1 = new Date(1, Month.January, 2025);
        //d1.Equals((object?)null) forces the Equals(object?) override
        Assert.That(d1.Equals((object?)null), Is.False);
    }
    [Test]
     public void Equals_Date_Null_IsFalse() {
        var d1 = new Date(1, Month.January, 2025);
        Assert.That(d1.Equals(null), Is.False);
    }
    [Test]
    public void Equals_Object_NonDate_IsFalse() {
        var d1 = new Date(1, Month.January, 2025);
        object notADate = "hello";
        Assert.That(d1.Equals(notADate), Is.False);
    }
    [Test]
    public void LessThanOperator_Works()
    {
        //Arrange 
        var d1 = new Date(14, Month.May, 1989);
        var d2 = new Date(15, Month.May, 1989);
        var d3 = new Date(14, Month.June, 1989);
        var d4 = new Date(14, Month.May, 2011);

        //Act
        var result1 = d1 < d2;
        var result2 = d1 < d3;
        var result3 = d1 < d4;
        var result4 = d4 < d1;

        //Assert
        Assert.That(result1, Is.True);
        Assert.That(result2, Is.True);
        Assert.That(result3, Is.True);
        Assert.That(result4, Is.False);
    }
    [Test]
    public void NotEqualityOperator_Works()
    {
        //Arrange 
        var d1 = new Date(14, Month.May, 1989);
        var d2 = new Date(15, Month.May, 1989);
        var d3 = new Date(14, Month.May, 1989);

        //Act
        var result1 = d1 != d2;
        var result2 = d1 != d3;

        //Assert
        Assert.That(result1, Is.True);
        Assert.That(result2, Is.False);
    }
    [Test]
    public void MoreThanOperator_Works()
    {
        //Arrange 
        var d1 = new Date(14, Month.May, 1989);
        var d2 = new Date(15, Month.May, 1989);
        var d3 = new Date(14, Month.June, 1989);
        var d4 = new Date(14, Month.May, 2011);

        //Act
        var result1 = d2 > d1;
        var result2 = d3 > d1;
        var result3 = d4 > d1;
        var result4 = d1 > d4;

        //Assert
        Assert.That(result1, Is.True);
        Assert.That(result2, Is.True);
        Assert.That(result3, Is.True);
        Assert.That(result4, Is.False);
    }
    [Test]
    public void AdditionOperator_AddsDaysCorrectly()
    {
        //Arrange
        var d1 = new Date(14, Month.May, 1989);
        //Act
        var result = d1 + 13;
        //Assert
        Assert.That(result.Day, Is.EqualTo(14 + 13));
        Assert.That(result.Month, Is.EqualTo(Month.May));
        Assert.That(result.Year, Is.EqualTo(1989));
    }
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
    [TestCase(Month.January, 2023, 31)]
    [TestCase(Month.February, 2023, 28)]
    [TestCase(Month.February, 2024, 29)]
    [TestCase(Month.March, 2023, 31)]
    [TestCase(Month.April, 2023, 30)]
    [TestCase(Month.May, 2023, 31)]
    [TestCase(Month.June, 2023, 30)]
    [TestCase(Month.July, 2023, 31)]
    [TestCase(Month.August, 2023, 31)]
    [TestCase(Month.September, 2023, 30)]
    [TestCase(Month.October, 2023, 31)]
    [TestCase(Month.November, 2023, 30)]
    [TestCase(Month.December, 2023, 31)]
    [TestCase(Month.January, 1, 31)]
    [TestCase(Month.December, 9999, 31)]
    public void DaysInMonth_Works(Month month, int year, int expectedDays)
    {
        //Arrange
        var daysInMonth = Date.DaysInMonth(month, year);
        //Act
        //Assert
        Assert.That(daysInMonth, Is.EqualTo(expectedDays));
    }
    [TestCase(0)]
    [TestCase(10000)]
    public void DaysInMonth_YearOutOfRange_Throws(int year)
    {
        Assert.Throws<ArgumentOutOfRangeException>(
            () => Date.DaysInMonth(Month.January, year));
    }
    [TestCase((Month)0)]
    [TestCase((Month)13)]
    public void DaysInMonth_MonthOutOfRange_Throws(Month month)
    {
        Assert.Throws<ArgumentOutOfRangeException>(
            () => Date.DaysInMonth(month, 2024));
    }
}




