#include <cstring>

#include "gtest/gtest.h"
#include "libs/util/array_list.h"
#include "libs/util/point.h"
#include "linked_list.h"

using namespace util;
using namespace std;

/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|																	|
|					---- LinkedListNode ----						|
|																	|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/



TEST		( LinkedListNode, AlternateConstructor )
{
	LinkedListNode<int> prev(0, NULL, NULL);
	LinkedListNode<int> curr(1, &prev, NULL);
	LinkedListNode<int> next(2, &curr, NULL);

	EXPECT_EQ(prev.next, &curr);
	EXPECT_EQ(curr.next, &next);
	EXPECT_TRUE(next.next == NULL);

	EXPECT_TRUE(prev.prev == NULL);
	EXPECT_EQ(curr.prev, &prev);
	EXPECT_EQ(next.prev, &curr);

	EXPECT_EQ(prev.value, 0);
	EXPECT_EQ(curr.value, 1);
	EXPECT_EQ(next.value, 2);
}



TEST		( LinkedListNode, RemoveNodeEdge )
{
	LinkedListNode<int> prev(0, NULL, NULL);
	LinkedListNode<int> curr(1, &prev, NULL);
	LinkedListNode<int> next(2, &curr, NULL);

	prev.RemoveNode();

	EXPECT_TRUE(prev.next == NULL);
	EXPECT_EQ(curr.next, &next);
	EXPECT_TRUE(next.next == NULL);

	EXPECT_TRUE(prev.prev == NULL);
	EXPECT_TRUE(curr.prev == NULL);
	EXPECT_EQ(next.prev, &curr);

	EXPECT_EQ(prev.value, 0);
	EXPECT_EQ(curr.value, 1);
	EXPECT_EQ(next.value, 2);
}



TEST		( LinkedListNode, RemoveNodeMiddle )
{
	LinkedListNode<int> prev(0, NULL, NULL);
	LinkedListNode<int> curr(1, &prev, NULL);
	LinkedListNode<int> next(2, &curr, NULL);

	curr.RemoveNode();

	EXPECT_EQ(prev.next, &next);
	EXPECT_TRUE(curr.next == NULL);
	EXPECT_TRUE(next.next == NULL);

	EXPECT_TRUE(prev.prev == NULL);
	EXPECT_TRUE(curr.prev == NULL);
	EXPECT_EQ(next.prev, &prev);

	EXPECT_EQ(prev.value, 0);
	EXPECT_EQ(curr.value, 1);
	EXPECT_EQ(next.value, 2);
}



TEST		( LinkedListNode, InsertMiddle )
{
	LinkedListNode<int> prev(0, NULL, NULL);
	LinkedListNode<int> curr(1, NULL, NULL);
	LinkedListNode<int> next(2, NULL, NULL);

	curr.InsertNode(&prev, &next);
	EXPECT_EQ(prev.next, &curr);
	EXPECT_EQ(curr.next, &next);
	EXPECT_TRUE(next.next == NULL);

	EXPECT_TRUE(prev.prev == NULL);
	EXPECT_EQ(curr.prev, &prev);
	EXPECT_EQ(next.prev, &curr);


	EXPECT_EQ(prev.value, 0);
	EXPECT_EQ(curr.value, 1);
	EXPECT_EQ(next.value, 2);
}




/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|																	|
|					---- Dimentions ----							|
|																	|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/



TEST		( Empty_Full_Size_MaxSize, Valid )
{
	util::LinkedList<int> linked;
	EXPECT_EQ(linked.Size(), 0);
	EXPECT_TRUE(linked.IsEmpty());
	EXPECT_FALSE(linked.IsFull());

	linked.PushBack(0);
	EXPECT_FALSE(linked.IsEmpty());
	EXPECT_FALSE(linked.IsFull());
	EXPECT_EQ(linked.Size(), 1);

	linked.PushBack(0);
	linked.PushBack(1);
	linked.PushBack(2);

	EXPECT_FALSE(linked.IsEmpty());
	EXPECT_FALSE(linked.IsFull());
	EXPECT_EQ(linked.Size(), 4);
}





/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|																	|
|						---- Push/Pop ----							|
|																	|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/



TEST		( PushBack,  Standard_WhenFull )
{
	util::LinkedList<int> linked;

	for ( uint i = 0; i < 100; i++ )
		EXPECT_TRUE(linked.PushBack(1));

	EXPECT_FALSE(linked.IsEmpty());
	EXPECT_EQ(linked.Size(), 100);

	EXPECT_TRUE(linked.PushBack(2));

	EXPECT_FALSE(linked.IsFull());
	EXPECT_FALSE(linked.IsEmpty());
	EXPECT_EQ(linked.Size(), 101);
}


TEST		( PushNodeBack, Standard )
{
	util::LinkedList<int> linked;

	LinkedListNode<int>* n1 = new LinkedListNode<int>(0, NULL, NULL);
	LinkedListNode<int>* n2 = new LinkedListNode<int>(1, NULL, NULL);
	LinkedListNode<int>* n3 = new LinkedListNode<int>(2, NULL, NULL);
	LinkedListNode<int>* n4 = new LinkedListNode<int>(3, NULL, NULL);


	linked.PushNodeBack(n1);
	linked.PushNodeBack(n2);
	linked.PushNodeBack(n3);
	linked.PushNodeBack(n4);

	EXPECT_FALSE(linked.IsEmpty());
	EXPECT_EQ(linked.Size(), 4);

	EXPECT_FALSE(linked.IsFull());

	EXPECT_EQ(linked.PopBack(), 3);
	EXPECT_EQ(linked.PopBack(), 2);
	EXPECT_EQ(linked.PopFront(), 0);
	EXPECT_EQ(linked.PopFront(), 1);
	EXPECT_EQ(linked.Size(), 0);
	EXPECT_TRUE(linked.head == NULL);
	EXPECT_TRUE(linked.tail == NULL);
}




TEST		( PopBack, All )
{
	util::LinkedList<int> linked;
	linked.PushBack(1);
	linked.PushBack(2);
	linked.PushBack(3);
	linked.PushBack(4);
	EXPECT_EQ(linked.PopBack(), 4);
	EXPECT_EQ(linked.PopBack(), 3);
	EXPECT_EQ(linked.PopBack(), 2);
	EXPECT_EQ(linked.PopBack(), 1);
	try
	{
		linked.PopBack();
		EXPECT_TRUE(false);
	}
	catch ( std::string )
	{
		EXPECT_TRUE(true);
	}
	linked.PushBack(1);
	EXPECT_EQ(linked.PopBack(), 1);
	EXPECT_EQ(linked.Size(), 0);
}


TEST 		( PopFront, All )
{
	util::LinkedList<int> linked;
	linked.PushBack(1);
	linked.PushBack(2);
	linked.PushBack(3);
	linked.PushBack(4);
	EXPECT_EQ(linked.PopFront(), 1);
	EXPECT_EQ(linked.PopFront(), 2);
	EXPECT_EQ(linked.PopFront(), 3);
	EXPECT_EQ(linked.PopFront(), 4);
	try
	{
		linked.PopFront();
		EXPECT_TRUE(false);
	}
	catch ( std::string )
	{
		EXPECT_TRUE(true);
	}
	linked.PushBack(1);
	EXPECT_EQ(linked.PopFront(), 1);
	EXPECT_EQ(linked.Size(), 0);
}


/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|																	|
|							---- Other ----							|
|																	|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

TEST		( Operator, Valid )
{
	const int size = 100;
	util::LinkedList<int> linked;
	for ( uint i = 0; i < size; i++ )	linked.PushBack(i + 2);

	for ( uint i = 0; i < linked.Size(); i++ )
	{
		int e = linked.Get(i) - 2;
		EXPECT_EQ(e, i);
	}
}



/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|																	|
|						---- Sorting ----							|
|																	|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

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
	util::LinkedList<int> input;
	input.PushBack(3);
	input.PushBack(1);
	input.PushBack(0);
	input.PushBack(5);
	input.PushBack(2);
	input.PushBack(4);
	input.PushBack(1);
	input.Sort(&sortDecending);

	EXPECT_EQ(input.Size(), 7);
	ASSERT_EQ(input.PopFront(), 5);
	ASSERT_EQ(input.PopFront(), 4);
	ASSERT_EQ(input.PopFront(), 3);
	ASSERT_EQ(input.PopFront(), 2);
	ASSERT_EQ(input.PopFront(), 1);

	ASSERT_EQ(input.PopBack(), 0);
	ASSERT_EQ(input.PopBack(), 1);
}




TEST		( SortList, WithObjects )
{
	Point<int> e1(0, 0);
	Point<int> e2(1, 0);
	Point<int> e3(2, 0);
	Point<int> e4(3, 0);
	Point<int> e5(5, 0);
	Point<int> e6(5, 0);


	util::LinkedList<Point<int>> input;
	input.PushBack(e4);
	input.PushBack(e1);
	input.PushBack(e5);
	input.PushBack(e3);
	input.PushBack(e6);
	input.PushBack(e2);
	input.Sort(&sortAscending);

	ASSERT_EQ(input.Size(), 6);
	ASSERT_EQ(input.PopFront().x, 0);
	ASSERT_EQ(input.PopFront().x, 1);
	ASSERT_EQ(input.PopFront().x, 2);
	ASSERT_EQ(input.PopFront().x, 3);
	ASSERT_EQ(input.PopFront().x, 5);
	ASSERT_EQ(input.PopFront().x, 5);
}



/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|																	|
|					---- ListToString ----							|
|																	|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

TEST 		( ListToString, Empty )
{
	LinkedList<string> list;
	string* str = list.ListToString(list);
	EXPECT_EQ(*str, "");
	delete str;
}



TEST 		( ListToString, Filled )
{
	LinkedList<string> list;
	string s = "1234";
	list.PushBack(s);
	list.PushBack("5678");
	list.PushBack("\n90");
	string* str = list.ListToString(list);
	EXPECT_EQ(*str, "12345678\n90");
	delete str;
}
