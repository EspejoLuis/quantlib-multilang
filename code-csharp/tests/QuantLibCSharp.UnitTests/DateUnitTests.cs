using System.Runtime;
using NUnit.Framework;
using QuantLibCSharp;

namespace QuantLibCSharp.UnitTests;

public class DateUnitTests
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

        Assert.Multiple(() =>
        {
            //Assert   
            Assert.That(d.Day, Is.EqualTo(14));
            Assert.That(d.Month, Is.EqualTo(Month.May));
            Assert.That(d.Year, Is.EqualTo(1989));
        });
    }

    [Test]
    public void ToString_IsCorrect()
    {
        //Arrange 
        var d = new Date(14, Month.May, 1989);
        //Act1q
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

        Assert.Multiple(()=> {
        //Assert
        Assert.That(firstDateNull, Is.False);
        Assert.That(secondDateNull, Is.False);
        });
    }

    [Test]
    public void Equals_Object_ListContains_IsTrue()
    {
        var d1 = new Date(1, Month.January, 2025);
        var d2 = new Date(1, Month.January, 2025);
        var list = new List<object> { d1 };
        Assert.That(list.Contains(d2), Is.True);
    }

    [Test]
    public void Equals_Object_SameValue_IsTrue()
    {
        var d1 = new Date(1, Month.January, 2025);
        object object2 = new Date(1, Month.January, 2025);
        Assert.That(d1.Equals(object2),Is.True);
    }

    [Test]
    public void Equals_Object_DifferentValue_IsFalse()
    {
        var d1 = new Date(1, Month.January, 2025);
        object object2 = new Date(2, Month.January, 2025);
        Assert.That(d1.Equals(object2), Is.False);
    }

    [Test]
    public void Equals_Object_Null_IsFalse()
    {
        var d1 = new Date(1, Month.January, 2025);
        //d1.Equals((object?)null) forces the Equals(object?) override
        Assert.That(d1.Equals((object?)null), Is.False);
    }

    [Test]
    public void Equals_Date_Null_IsFalse()
    {
        var d1 = new Date(1, Month.January, 2025);
        Assert.That(d1.Equals(null), Is.False);
    }

    [Test]
    public void Equals_Object_NonDate_IsFalse()
    {
        var d1 = new Date(1, Month.January, 2025);
        object notADate = "hello";
        Assert.That(d1, Is.Not.EqualTo(notADate));
    }

    [Test]
    public void LessThanOperator_IsTrue()
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

        Assert.Multiple(() =>
        {
            //Assert
            Assert.That(result1, Is.True);
            Assert.That(result2, Is.True);
            Assert.That(result3, Is.True);
        });
    }

    [Test]
    public void LessThanOperator_IsFalse()
    {
        //Arrange 
        var d1 = new Date(14, Month.May, 1989);
        var d4 = new Date(14, Month.May, 2011);

        //Act
        var result4 = d4 < d1;

        //Assert
        Assert.That(result4, Is.False);
    }

    [Test]
    public void NotEqualityOperator_IsTrue()
    {
        //Arrange 
        var d1 = new Date(14, Month.May, 1989);
        var d2 = new Date(15, Month.May, 1989);

        //Act
        var result1 = d1 != d2;

        //Assert
        Assert.That(result1, Is.True);
    }

    [Test]
    public void NotEqualityOperator_IsFalse()
    {
        //Arrange 
        var d1 = new Date(14, Month.May, 1989);
        var d3 = new Date(14, Month.May, 1989);

        //Act
        var result2 = d1 != d3;

        //Assert
        Assert.That(result2, Is.False);
    }

    [Test]
    public void MoreThanOperator_IsTrue()
    {
        //Arrange 
        var d1 = new Date(14, Month.May, 1989);
        var d2 = new Date(15, Month.May, 1989);
        var d3 = new Date(14, Month.June, 1989);

        //Act
        var result1 = d2 > d1;
        var result2 = d3 > d1;

        Assert.Multiple(() =>
        {
            //Assert
            Assert.That(result1, Is.True);
            Assert.That(result2, Is.True);
        });

    }

    [Test]
    public void MoreThanOperator_IsFalse()
    {
        //Arrange 
        var d1 = new Date(14, Month.May, 1989);
        var d4 = new Date(14, Month.May, 2011);

        //Act
        var result4 = d1 > d4;

        //Assert
        Assert.That(result4, Is.False);
    }

    [Test]
    public void AdditionOperator_AddsDaysCorrectly()
    {
        //Arrange
        var d1 = new Date(14, Month.May, 1989);
        //Act
        var result = d1 + 13;
        Assert.Multiple(() =>
        {
            //Assert
            Assert.That(result.Day, Is.EqualTo(14 + 13));
            Assert.That(result.Month, Is.EqualTo(Month.May));
            Assert.That(result.Year, Is.EqualTo(1989));
        });

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

    [Test]
    public void GetHashCode_EqualDates_SameHash()
    {
        var d1 = new Date(1, Month.January, 2025);
        var d2 = new Date(1, Month.January, 2025);

        Assert.That(d1, Is.EqualTo(d2));
        Assert.That(d1.GetHashCode(), Is.EqualTo(d2.GetHashCode()));
    }
}
