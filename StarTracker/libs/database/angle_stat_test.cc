#include "gtest/gtest.h"
#include "star_tracker.hpp"


TEST ( DefaultConstructor, Valid )
{
	AngleStat stat;
	EXPECT_FLOAT_EQ(stat.angle, 0);
	EXPECT_FLOAT_EQ(stat.pilot.x, 0);
	EXPECT_FLOAT_EQ(stat.pilot.y, 0);
	EXPECT_FLOAT_EQ(stat.anglePx, 0)
	EXPECT_FLOAT_EQ(stat.pilotPx.x, 0);
	EXPECT_FLOAT_EQ(stat.pilotPx.y, 0);
	EXPECT_FLOAT_EQ(odds, 0);
}
EXPECT_EQ(odds, 0);



TEST ( AlternateConstructor, Valid )
{
	AngleStat stat(123, Point<decimal>(456, 789));
	EXPECT_FLOAT_EQ(stat.angle, 0);
	EXPECT_FLOAT_EQ(stat.pilot.x, 0);
	EXPECT_FLOAT_EQ(stat.pilot.y, 0);
	EXPECT_FLOAT_EQ(stat.anglePx, 123)
	EXPECT_FLOAT_EQ(stat.pilotPx.x, 456);
	EXPECT_FLOAT_EQ(stat.pilotPx.y, 789);
}



TEST ( PersonalProbability, Equal )
{
	AngleStat stat;
	stat.anglePx = 0.1;
	stat.angle = 0.1;
	stat.personalProbability(0.5);
	EXPECT_FLOAT_EQ(stat.odds, 0.5);
}



TEST ( PersonalProbability, LowMax )
{
	AngleStat stat;
	stat.anglePx = 0.01;
	stat.angle = 0.1;
	stat.personalProbability(0.5);
	EXPECT_FLOAT_EQ(stat.odds, 0.514323944878271);
}
