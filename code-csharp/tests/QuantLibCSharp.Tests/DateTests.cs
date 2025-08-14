
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
    public void Equality_Works()
    {
        //Arrange 
        var d1 = new Date(14, Month.May, 1989);
        var d2 = new Date(14, Month.May, 1989);
        var d3 = new Date(15, Month.May, 1989);

        //Act
        var result1 = d1 == d2;
        var result2 = d1 == d3;

        //Assert
        Assert.That(result1, Is.True);
        Assert.That(result2, Is.False);
    }
    [Test]
    public void LessThan_Works()
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
    public void NotEquality_Works()
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
    public void MoreThan_Works()
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
    [Test]
    public void SubtractionOperator_SubtractsDaysCorrectly()
    {
        //Arrange
        var d1 = new Date(14, Month.May, 1989);

        //Act
        var result = d1 - 11;
        //Assert
        Assert.That(result.Day, Is.EqualTo(14 - 11));
        Assert.That(result.Month, Is.EqualTo(Month.May));
        Assert.That(result.Year, Is.EqualTo(1989));
    }
}



