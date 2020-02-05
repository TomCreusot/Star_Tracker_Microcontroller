#include <cstring>

#include "gtest/gtest.h"
#include "array_list.h"

using namespace util;
using namespace std;

TEST		( DefaultConstructor, WithPrimative )
{
	util::ArrayList<int> list;
	EXPECT_EQ(list.size(), 0);
	EXPECT_EQ(list.max_size(), ARRAY_LIST_SIZE);
	EXPECT_TRUE(list.empty());
}


TEST		( DefaultConstructor, WithObject )
{
	util::ArrayList<string> list;
	EXPECT_EQ(list.max_size(), ARRAY_LIST_SIZE);
	EXPECT_EQ(list.size(), 0);
	EXPECT_TRUE(list.empty());
}




/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|																	|
|					---- Dimentions ----							|
|																	|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/



TEST		( Empty_Full_Size_MaxSize, Valid )
{
	util::ArrayList<int> list;
	EXPECT_EQ(list.size(), 0);
	EXPECT_TRUE(list.empty());
	EXPECT_FALSE(list.full());

	EXPECT_TRUE(list.empty());
	EXPECT_FALSE(list.full());

	list.push_back(0);
	EXPECT_FALSE(list.empty());
	EXPECT_FALSE(list.full());
	EXPECT_EQ(list.size(), 1);

	for ( uint i = 1; i < list.max_size(); i++ ) list.push_back(i);

	EXPECT_FALSE(list.empty());
	EXPECT_TRUE(list.full());
	EXPECT_EQ(list.size(), list.max_size());
}




/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|																	|
|						---- Push/Pop ----							|
|																	|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/



TEST		( PushBack,  Standard_WhenFull )
{
	util::ArrayList<int> list;

	for ( uint i = 0; i < list.max_size(); i++ )
	EXPECT_TRUE(list.push_back(1));

	EXPECT_TRUE(list.full());
	EXPECT_FALSE(list.empty());
	EXPECT_EQ(list.size(), list.max_size());

	EXPECT_FALSE(list.push_back(2));

	EXPECT_TRUE(list.full());
	EXPECT_FALSE(list.empty());
	EXPECT_EQ(list.size(), list.max_size());
}


TEST		( PopBack, Valid )
{
	util::ArrayList<int> list;
	list.push_back(1);
	list.push_back(2);
	list.push_back(3);
	list.push_back(4);
	EXPECT_EQ(list.pop_back(), 4);
	EXPECT_EQ(list.pop_back(), 3);
	EXPECT_EQ(list.pop_back(), 2);
	EXPECT_EQ(list.pop_back(), 1);
}


TEST		( PopBack, WhenEmpty )
{
	util::ArrayList<int> list;

	list.push_back(1);
	list.push_back(2);
	EXPECT_EQ(list.pop_back(), 2);
	EXPECT_EQ(list.pop_back(), 1);
	EXPECT_EQ(list.pop_back(), 1);
}


TEST		( PopBack, WhenFull )
{
	util::ArrayList<int> list;

	for ( uint i = 0; i < list.max_size(); i++ ) list.push_back(0);
	list.push_back(1);

	EXPECT_EQ(list.pop_back(), 0);
	EXPECT_EQ(list.pop_back(), 0);
}


/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|																	|
|							---- Other ----							|
|																	|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

TEST		( Operator, Valid )
{
	util::ArrayList<int> list;
	for ( uint i = 0; i < list.max_size(); i++ )	list.push_back(i + 2);

	for ( uint i = 0; i < list.size(); i++ )
	{
		int e = list[i] - 2;
		EXPECT_EQ(e, i);
	}
}
