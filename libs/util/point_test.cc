#include "gtest/gtest.h"

#include "util.h"
#include "point.h"

using namespace std;
using namespace util;



TEST ( DefaultConstructor, Valid )
{
	util::Point<int> point;
	EXPECT_EQ(point.x, 0);
	EXPECT_EQ(point.y, 0);
}

TEST ( AlternateConstructor, SingleValue )
{
	util::Point<int> point(-1);
	EXPECT_EQ(point.x, -1);
	EXPECT_EQ(point.y, -1);
}

TEST ( AlternateConstructor, XandY )
{
	util::Point<int> point(-1, 1000);
	EXPECT_EQ(point.x, -1);
	EXPECT_EQ(point.y, 1000);
}


TEST ( CopyConstructor, Valid )
{
	util::Point<int> point(1, -3);
	util::Point<int> p(point);
	EXPECT_EQ(p.x, point.x);
	EXPECT_EQ(p.y, point.y);
}


TEST ( Accessors, Ra_Dec )
{
	Equatorial<decimal> point(M_PI * 2, 2.3);
	EXPECT_FLOAT_EQ(M_PI * 2, point.x);
	EXPECT_FLOAT_EQ(2.3, point.y);
	EXPECT_FLOAT_EQ(point.x, point.Ra());
	EXPECT_FLOAT_EQ(point.RaHour(), 24);
	EXPECT_FLOAT_EQ(point.DecDeg(), 131.780292880089);
	EXPECT_FLOAT_EQ(point.y, point.Dec());

	point.Ra(321);
	point.Dec(123);
	EXPECT_FLOAT_EQ(point.x, 321);
	EXPECT_FLOAT_EQ(point.y, 123);
	EXPECT_FLOAT_EQ(point.DecDeg(), 7047.38088010913);

	point.RaHour(12.0);
	point.DecDeg(90);
	EXPECT_EQ(point.x, M_PI);
	EXPECT_EQ(point.y, M_PI / 2);
}


TEST ( Set, XandY )
{
	util::Point<int> point;
	EXPECT_EQ(point.x, 0);
	EXPECT_EQ(point.y, 0);
	point.Set(-1, 2323);
	EXPECT_EQ(point.x, -1);
	EXPECT_EQ(point.y, 2323);
}



TEST ( Distance, Valid )
{
	Cartesian<decimal> p1(0, 1);	// Cartesian is typedef for point
	Cartesian<decimal> p2(2, 3);
	EXPECT_FLOAT_EQ(p1.Distance(p2), 2.828427125);

	p2 = Cartesian<decimal>(-100, -200);
	EXPECT_FLOAT_EQ(p1.Distance(p2), 224.5016704);
}



TEST ( RadialDistance, CartesianValid )
{
	Point<decimal> p1(0, -M_PI);
	Point<decimal> p2(0, -M_PI);
	decimal out = p1.RadialDistance(10, p2);	// 0 Distance
	EXPECT_FLOAT_EQ(out, 0);
	p1.Set(3, 0);
	p2.Set(0, 4);
	out = p1.RadialDistance(10, p2);			// 30,40,50 triangle
	EXPECT_FLOAT_EQ(out, 50);
}

TEST ( RadialDistance, EquatorialZeroAngle )
{
	Equatorial<decimal> p1(0, -M_PI);
	Equatorial<decimal> p2(0, -M_PI);
	decimal out = p1.RadialDistance(p2);	// 0 Distance
	EXPECT_FLOAT_EQ(out, 0);
	p1.Set(0, 0);
	p2.Set(2*M_PI, 0);
	out = p1.RadialDistance(p2);			//
	EXPECT_FLOAT_EQ(out, 0);
}

TEST ( RadialDistance, EquatorialSinglePlane )
{
	Equatorial<decimal> p1(0, -M_PI / 2.0);
	Equatorial<decimal> p2(0, +M_PI / 2.0);
	decimal out = p1.RadialDistance(p2);	// 180 deg Distance
	EXPECT_FLOAT_EQ(out, M_PI);
	p1.Set(M_PI, 0);
	p2.Set(2*M_PI, 0);
	out = p1.RadialDistance(p2);			//
	EXPECT_FLOAT_EQ(out, M_PI);
}


TEST ( RadialDistance, EquatorialRandomPoints )
{
	Equatorial<decimal> p1(4.15, 1.2);
	Equatorial<decimal> p2(0.1, -0.3);
	decimal out = p1.RadialDistance(p2);	// 180 deg Distance
	EXPECT_FLOAT_EQ(out, 2.08097590023879);
	p1.Set(0.1, 1.23);
	p2.Set(1000, -0.31);
	out = p1.RadialDistance(p2);			//
	EXPECT_FLOAT_EQ(out, 1.65401625934163);
}


TEST ( Magnitude, Valid )
{
	Point<decimal> val(1.2, 2.1);
	EXPECT_FLOAT_EQ(val.Magnitude(), 2.418677324);
}


TEST ( Equal, IsTrue )
{
	Point<decimal> p1(1000, -22);
	Point<decimal> p2(1000, -22);
	EXPECT_TRUE(p1.Equal(p1));
	EXPECT_TRUE(p1.Equal(p2));

	EXPECT_TRUE(p1.Equal(1000, -22));
}



TEST ( Equal, IsFalse )
{
	Point<decimal> p1(22, 1000);
	Point<decimal> p2(1000, 22);
	EXPECT_FALSE(p1.Equal(p2));

	p2 = util::Point<util::decimal>(22, -1000);
	EXPECT_FALSE(p1.Equal(p2));

	EXPECT_FALSE(p1.Equal(1000, 22));
}
