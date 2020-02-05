#include "gtest/gtest.h"
#include "star_tracker.h"

using namespace st;


TEST ( DefaultConstructor, Standard )
{
	st::Combo c;
	EXPECT_TRUE(c.equal(0, 0, 0, 0));
}


TEST ( AlternateConstructor, Standard )
{
	st::Combo combo(0, -1, 0, 32);
	EXPECT_TRUE(combo.equal(0, -1, 0, 32));
}


TEST ( Equal, IsEqual )
{
	st::Combo c ( 0, -1, 2, 3 );
	EXPECT_TRUE (c.equal(0, -1, 2, 3));
}

TEST ( Equal, IsNotEqual )
{
	st::Combo c (10, 1, 2, 2 );
	EXPECT_FALSE (c.equal(10, 1, 2, 3));
}
