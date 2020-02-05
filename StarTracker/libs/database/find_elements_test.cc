#include "gtest/gtest.h"
#include "find_elements.h"

TEST ( FindElements, TenValid )
{
	AngleStat stat();
	stat.anglePx = 123;
	LinkedList<AngleStat> list = 0;
	find_elements(stat, 10, list);
}


TEST ( FindElements, OneValid )
{

}



TEST ( FindElements, NoneValid )
{

}



TEST ( FindElements, Negative )
{

}

TEST ( FindElements, NegativeTolerance )
{

}


TEST ( FindElements, Decimal )
{

}
