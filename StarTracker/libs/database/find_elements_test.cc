/**
 * IDK how to make a test harness for this!?!?
 *
 */

#include "gtest/gtest.h"
#include "find_elements.h"

decimal database[22][3] =
{
	{0, 1, 1},
	{1, 2, 2},
	{2, 3, 3},
	{3, 4, 4},
	{4, 5, 5},
	{5, 6, 6},
	{6, 7, 7},
	{7, 8, 8},
	{8, 9, 9},
	{9, 10, 10},
	{10, 11, 11},
	{11, 12, 12},
	{102.4, 1024, 1024},
	{102.3, 1023, 1023}
};


TEST ( FindElements, TenValid )
{
	AngleStat stat();
	stat.angle = 1;
	LinkedList<AngleStat> list;
	find_elements((decimal**)database stat, 9, list);
	EXPECT_EQ(list.size(), 10);
	EXPECT_EQ(list.pop_back().pilot.x, 10);
	EXPECT_EQ(list.pop_back().pilot.x, 9);
	EXPECT_EQ(list.pop_back().pilot.x, 8);
	EXPECT_EQ(list.pop_back().pilot.x, 7);
	EXPECT_EQ(list.pop_back().pilot.x, 6);
	EXPECT_EQ(list.pop_back().pilot.x, 5);
	EXPECT_EQ(list.pop_back().pilot.x, 4);
	EXPECT_EQ(list.pop_back().pilot.x, 3);
	EXPECT_EQ(list.pop_back().pilot.x, 2);
	EXPECT_EQ(list.pop_back().pilot.x, 1);
}


TEST ( FindElements, OneValid )
{
	AngleStat stat();
	stat.angle = 2;
	LinkedList<AngleStat> list;
	find_elements((decimal**)database stat, 0, list);
	EXPECT_EQ(list.size(), 1);
	EXPECT_EQ(list.pop_back().pilot.x, 2);
}

TEST ( FindElements, NoneValid )
{
	AngleStat stat();
	stat.angle = 12;
	LinkedList<AngleStat> list;
	find_elements((decimal**)database stat, 0, list);
	EXPECT_EQ(list.size(), 0);
}



TEST ( FindElements, Negative )
{
	AngleStat stat();
	stat.angle = -1;
	LinkedList<AngleStat> list;
	find_elements((decimal**)database stat, 2, list);
	EXPECT_EQ(list.size(), 2);
	EXPECT_EQ(list.pop_back().pilot.x, 1);
	EXPECT_EQ(list.pop_back().pilot.x, 0);
}


TEST ( FindElements, Decimal )
{
	AngleStat stat();
	stat.angle = 100.22;
	LinkedList<AngleStat> list;
	find_elements((decimal**)database stat, 0.015, list);
	EXPECT_EQ(list.size(), 1);
	EXPECT_EQ(list.pop_back().pilot.x, 1023);
}
