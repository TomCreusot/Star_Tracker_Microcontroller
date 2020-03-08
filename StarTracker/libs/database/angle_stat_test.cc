#include "gtest/gtest.h"
#include "angle_stat.h"

using namespace database;

TEST ( DefaultConstructor, Valid )
{
	AngleStat stat;
	EXPECT_FLOAT_EQ(stat.angle, 0);
	EXPECT_FLOAT_EQ(stat.pilot.x, 0);
	EXPECT_FLOAT_EQ(stat.pilot.y, 0);
	EXPECT_FLOAT_EQ(stat.odds, 0);
	// EXPECT_EQ(stat.pixel, NULL);
}



TEST ( AlternateConstructor, Valid )
{
	Point<decimal> temp = Point<decimal>(456, 789);
	AngleStat stat(123, temp);
	EXPECT_FLOAT_EQ(stat.angle, 123);
	EXPECT_FLOAT_EQ(stat.pilot.x, 456);
	EXPECT_FLOAT_EQ(stat.pilot.y, 789);
	EXPECT_FLOAT_EQ(stat.odds, 0);
}

TEST ( CopyConstructor, Valid )
{
	Point<decimal> temp = Point<decimal>(456, 789);
	AngleStat stat(123, temp);
	stat.odds = 321;
	EXPECT_FLOAT_EQ(stat.pilot.x, 456);
	EXPECT_FLOAT_EQ(stat.pilot.y, 789);
	EXPECT_FLOAT_EQ(stat.angle, 123);
	EXPECT_FLOAT_EQ(stat.odds, 321);

	AngleStat stat2(stat);
	EXPECT_FLOAT_EQ(stat2.angle, 123);
	EXPECT_FLOAT_EQ(stat2.pilot.x, 456);
	EXPECT_FLOAT_EQ(stat2.pilot.y, 789);
	EXPECT_FLOAT_EQ(stat2.odds, 321);
	EXPECT_FLOAT_EQ(stat2.pixel->angle, 123);
	EXPECT_FLOAT_EQ(stat2.pixel->pilot.x, 456);
	EXPECT_FLOAT_EQ(stat2.pixel->pilot.y, 789);
	EXPECT_FLOAT_EQ(stat2.pixel->odds, 321);
}



TEST ( PersonalProbability, Equal )
{
	AngleStat statPx;
	AngleStat stat(statPx);
	stat.pixel->angle = 0.1;
	stat.angle = 0.1;
	stat.personalProbability();
	EXPECT_FLOAT_EQ(stat.odds, 1);
}


TEST ( PersonalProbability, NULL )
{
	AngleStat stat;
	stat.angle = 0.1;
	stat.personalProbability();
	EXPECT_FLOAT_EQ(stat.odds, 0);
}


TEST ( PersonalProbability, NotEqual )
{
	AngleStat statPx;
	AngleStat stat(statPx);
	stat.pixel->angle = 0;
	stat.angle = 0.1;
	stat.personalProbability();
	EXPECT_FLOAT_EQ(stat.odds, 1/1.1);
}


TEST ( PersonalProbability, VeryNotEqual )
{
	AngleStat statPx;
	AngleStat stat(statPx);
	stat.pixel->angle = -1000000000;
	stat.angle = 10000000000;
	stat.personalProbability();
	EXPECT_NEAR(stat.odds, 0, 0.00001);
}


TEST ( ClusterProbability, OneElement )
{
	Point<decimal> aa(0, 0);
	AngleStat a(0, aa);
	a.odds = 0.023;

	ArrayList<AngleStat> list;

	AngleStat::clusterProbability(list, 10, 10);
	EXPECT_FLOAT_EQ(a.odds, 0.023);
}



TEST( FindCenter, OnCenter )
{
	Point<decimal> px(50, 50);
	Point<decimal> center();
	AngleStat stat();
	AngleStat pxS;
	stat.pixel = *pxS;
	stat.pixel->opposite = px;
	clusterProbability(60, 50, 100, 100, center);

	EXPECT_FLOAT_EQ(center.x, 0);
	EXPECT_FLOAT_EQ(center.y, 0);
}



TEST( FindCenter, OnEdgeX )
{
	Point<decimal> px(0, 50);
	Point<decimal> center();
	AngleStat stat();
	AngleStat pxS;
	stat.pixel = *pxS;
	stat.pixel->opposite = px;
	clusterProbability(60, 50, 100, 100, center);

	EXPECT_FLOAT_EQ(center.x, -30);
	EXPECT_FLOAT_EQ(center.y, 0);
}
