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


TEST ( AlternateConstructor, DegMinSec )
{
	util::Point<util::decimal> point(10, 20, 30,  -11, 0, 30);
	EXPECT_FLOAT_EQ(point.x, 37.84167);
	EXPECT_FLOAT_EQ(point.y, -11.00833);
}


TEST ( CopyConstructor, Valid )
{
	util::Point<int> point(1, -3);
	util::Point<int> p(point);
	EXPECT_EQ(p.x, point.x);
	EXPECT_EQ(p.y, point.y);
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


TEST ( Set, DegMinSec )
{
	util::Point<util::decimal> point;
	EXPECT_DOUBLE_EQ(point.x, 0);
	EXPECT_DOUBLE_EQ(point.y, 0);
	point.Set(10, 20, 30,  -11, 0, 30);
	EXPECT_FLOAT_EQ(point.x, 10.34167);
	EXPECT_FLOAT_EQ(point.y, -11.00833);
}


TEST ( Distance, Valid )
{
	util::Point<util::decimal> p1(0, 1);
	util::Point<util::decimal> p2(2, 3);
	EXPECT_FLOAT_EQ(p1.Distance(p2), 2.828427125);

	p2 = util::Point<util::decimal>(-100, -200);
	EXPECT_FLOAT_EQ(p1.Distance(p2), 224.5016704);
}



TEST ( Equal, IsTrue )
{
	util::Point<util::decimal> p1(1000, -22);
	util::Point<util::decimal> p2(1000, -22);
	EXPECT_TRUE(p1.Equal(p1));
	EXPECT_TRUE(p1.Equal(p2));

	EXPECT_TRUE(p1.Equal(1000, -22));
}



TEST ( Equal, IsFalse )
{
	util::Point<util::decimal> p1(22, 1000);
	util::Point<util::decimal> p2(1000, 22);
	EXPECT_FALSE(p1.Equal(p2));

	p2 = util::Point<util::decimal>(22, -1000);
	EXPECT_FALSE(p1.Equal(p2));

	EXPECT_FALSE(p1.Equal(1000, 22));
}
