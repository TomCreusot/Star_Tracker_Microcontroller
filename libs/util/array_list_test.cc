#include <cstring>

#include "gtest/gtest.h"
#include "array_list.h"
#include "point.h"

using namespace util;
using namespace std;

TEST		( DefaultConstructor, WithPrimative )
{
	util::ArrayList<int, 10> list;
	EXPECT_EQ(list.Size(), 0);
	EXPECT_EQ(list.MaxSize(), 10);
	EXPECT_TRUE(list.IsEmpty());
}


TEST		( DefaultConstructor, WithObject )
{
	util::ArrayList<string, 0> list;
	EXPECT_EQ(list.Size(), 0);
	EXPECT_EQ(list.MaxSize(), 0);
	EXPECT_TRUE(list.IsEmpty());
}



TEST		( AlternateConstructor, WithPrimative )
{
	util::ArrayList<int, 4> list(3);
	EXPECT_EQ(list.Size(), 3);
	EXPECT_EQ(list.MaxSize(), 4);

	util::ArrayList<int, 3> listB(10);
	EXPECT_EQ(listB.Size(), 3);
	EXPECT_EQ(listB.MaxSize(), 3);
}


TEST		( AlternateConstructor, WithObject )
{
	util::ArrayList<string, 10> list(3);
	EXPECT_EQ(list.Size(), 3);
	EXPECT_EQ(list.MaxSize(), 10);

	util::ArrayList<string, 1> listB(10);
	EXPECT_EQ(listB.Size(), 1);
	EXPECT_EQ(listB.MaxSize(), 1);
}





TEST		( CopyConstructor, WithPrimative )
{
	util::ArrayList<int, 10> list;
	list.PushBack(1);
	list.PushBack(2);
	list.PushBack(3);
	list.PushBack(4);
	util::ArrayList<int, 10> copy(list);
	EXPECT_EQ(list.Get(0), copy.Get(0));
	EXPECT_EQ(list.Get(1), copy.Get(1));
	EXPECT_EQ(list.Get(2), copy.Get(2));
	EXPECT_EQ(list.Get(3), copy.Get(3));
	EXPECT_EQ(list.Size(), copy.Size());
}


TEST		( CopyConstructor, WithObject )
{
	util::ArrayList<string, 10> list;
	list.PushBack("a");
	list.PushBack("b");
	list.PushBack("c");
	list.PushBack("d");
	util::ArrayList<string, 10> copy(list);
	EXPECT_EQ(list.Get(0), copy.Get(0));
	EXPECT_EQ(list.Get(1), copy.Get(1));
	EXPECT_EQ(list.Get(2), copy.Get(2));
	EXPECT_EQ(list.Get(3), copy.Get(3));
	EXPECT_EQ(list.Size(), copy.Size());
}




/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|																	|
|					---- Dimentions ----							|
|																	|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/



TEST		( Empty_Full_Size_MaxSize, Valid )
{
	util::ArrayList<int, 3> list;
	EXPECT_EQ(list.Size(), 0);
	EXPECT_TRUE(list.IsEmpty());
	EXPECT_FALSE(list.IsFull());

	EXPECT_TRUE(list.IsEmpty());
	EXPECT_FALSE(list.IsFull());

	list.PushBack(0);
	EXPECT_FALSE(list.IsEmpty());
	EXPECT_FALSE(list.IsFull());
	EXPECT_EQ(list.Size(), 1);

	list.PushBack(0);
	list.PushBack(1);
	list.PushBack(2);

	EXPECT_FALSE(list.IsEmpty());
	EXPECT_TRUE(list.IsFull());
	EXPECT_EQ(list.Size(), 3);
}



TEST 		( ReduceSize, WhenGreater)
{
	util::ArrayList<int, 5> list;
	list.PushBack(1);
	list.PushBack(2);
	list.PushBack(3);
	list.ReduceSize(4);
	EXPECT_EQ(list.Size(), 3);
	EXPECT_FALSE(list.IsEmpty());

	list.ReduceSize(100);
	EXPECT_EQ(list.Size(), 3);

	list.ReduceSize(3);
	EXPECT_EQ(list.Size(), 3);
}


TEST 		( ReduceSize, WhenValid)
{
	util::ArrayList<int, 10> list;
	list.PushBack(1);
	list.PushBack(2);
	list.PushBack(3);
	list.PushBack(4);
	list.PushBack(5);
	list.ReduceSize(4);
	EXPECT_EQ(list.Size(), 4);
	EXPECT_FALSE(list.IsEmpty());

	list.ReduceSize(3);
	EXPECT_EQ(list.Size(), 3);
}






/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|																	|
|						---- Push/Pop ----							|
|																	|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/



TEST		( PushBack,  Standard_WhenFull )
{
	util::ArrayList<int, 100> list;

	for ( uint i = 0; i < 100; i++ )
	EXPECT_TRUE(list.PushBack(1));

	EXPECT_TRUE(list.IsFull());
	EXPECT_FALSE(list.IsEmpty());
	EXPECT_EQ(list.Size(), 100);

	EXPECT_FALSE(list.PushBack(2));

	EXPECT_TRUE(list.IsFull());
	EXPECT_FALSE(list.IsEmpty());
	EXPECT_EQ(list.Size(), 100);
}


TEST		( PopBack, Valid )
{
	const int size = 5;
	util::ArrayList<int, size> list;
	list.PushBack(1);
	list.PushBack(2);
	list.PushBack(3);
	list.PushBack(4);
	EXPECT_EQ(list.PopBack(), 4);
	EXPECT_EQ(list.PopBack(), 3);
	EXPECT_EQ(list.PopBack(), 2);
	EXPECT_EQ(list.PopBack(), 1);
}


TEST		( PopBack, WhenEmpty )
{
	const int size = 5;
	util::ArrayList<int, size> list;

	list.PushBack(1);
	list.PushBack(2);
	EXPECT_EQ(list.PopBack(), 2);
	EXPECT_EQ(list.PopBack(), 1);
	EXPECT_EQ(list.PopBack(), 1);
}


TEST		( PopBack, WhenFull )
{
	const int size = 5;
	util::ArrayList<int, size> list;

	for ( uint i = 0; i < size; i++ ) list.PushBack(0);
	list.PushBack(1);

	EXPECT_EQ(list.PopBack(), 0);
	EXPECT_EQ(list.PopBack(), 0);
}


/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|																	|
|							---- Other ----							|
|																	|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

TEST		( Operator, Valid )
{
	const int size = 100;
	util::ArrayList<int, size> list;
	for ( uint i = 0; i < size; i++ )	list.PushBack(i + 2);

	for ( uint i = 0; i < list.Size(); i++ )
	{
		int e = list.Get(i) - 2;
		EXPECT_EQ(e, i);
	}
}










// Sort
bool sortDecending ( int& a, int& b )
{
	return a >= b;
}


bool sortAscending ( Point<int>& a, Point<int>& b )
{
	return a.x <= b.x;
}


TEST		( SortList, WithPrimative )
{
	const int size = 10;
	util::ArrayList<int, size> input;
	input.PushBack(3);
	input.PushBack(1);
	input.PushBack(0);
	input.PushBack(5);
	input.PushBack(2);
	input.PushBack(4);
	input.PushBack(1);
	input.Sort(&sortDecending);

	EXPECT_EQ(input.Get(0), 5);
	EXPECT_EQ(input.Get(1), 4);
	EXPECT_EQ(input.Get(2), 3);
	EXPECT_EQ(input.Get(3), 2);
	EXPECT_EQ(input.Get(4), 1);
	EXPECT_EQ(input.Get(5), 1);
	EXPECT_EQ(input.Get(6), 0);
	EXPECT_EQ(input.Size(), 7);
}











TEST		( SortList, WithObjects )
{
	const int size = 10;

	Point<int> e1(0, 0);
	Point<int> e2(1, 0);
	Point<int> e3(2, 0);
	Point<int> e4(3, 0);
	Point<int> e5(5, 0);


	util::ArrayList<Point<int>, size> input;
	input.PushBack(e4);
	input.PushBack(e1);
	input.PushBack(e5);
	input.PushBack(e3);
	input.PushBack(e2);
	input.Sort(&sortAscending);

	ASSERT_EQ(input.Get(0).x, 0);
	ASSERT_EQ(input.Get(1).x, 1);
	ASSERT_EQ(input.Get(2).x, 2);
	ASSERT_EQ(input.Get(3).x, 3);
	ASSERT_EQ(input.Get(4).x, 5);
	ASSERT_EQ(input.Size(), 5);
}
