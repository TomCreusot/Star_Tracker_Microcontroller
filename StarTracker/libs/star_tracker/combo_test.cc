#include "gtest/gtest.h"
#include "star_tracker.h"

using namespace star_tracker;


TEST ( DefaultConstructor, Standard )
{
	star_tracker::Combo c;
	EXPECT_TRUE(c.equal(0, 0, 0, 0));
}


TEST ( AlternateConstructor, Standard )
{
	star_tracker::Combo combo(0, -1, 0, 32);
	EXPECT_TRUE(combo.equal(0, -1, 0, 32));
}


TEST ( Equal, IsEqual )
{
	star_tracker::Combo c ( 0, -1, 2, 3 );
	EXPECT_TRUE (c.equal(0, -1, 2, 3));
}

TEST ( Equal, IsNotEqual )
{
	star_tracker::Combo c (10, 1, 2, 2 );
	EXPECT_FALSE (c.equal(10, 1, 2, 3));
}
