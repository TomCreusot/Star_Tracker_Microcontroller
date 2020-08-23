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



TEST		( AlternateConstructor_LinkedListNode, Int )
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


TEST 	( AlternateConstructor_LinkedListNode, String )
{
	LinkedListNode<string> curr("0", NULL, NULL);

	EXPECT_TRUE(curr.next == NULL);
	EXPECT_TRUE(curr.prev == NULL);

	EXPECT_EQ(curr.value[0], '0');
}




TEST		( LinkedListNode_RemoveNode, Edge_Int )
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



TEST		( LinkedListNode_RemoveNode, Middle_Int )
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



TEST		( LinkedListNode_RemoveNode, Middle_String )
{
	LinkedListNode<string> curr("1", NULL, NULL);

	curr.RemoveNode();

	EXPECT_TRUE(curr.next == NULL);
	EXPECT_TRUE(curr.prev == NULL);
	EXPECT_EQ(curr.value, "1");
}





TEST		( LinkedListNode_InsertNode, Middle_Int )
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


TEST		( LinkedListNode_InsertNode, Middle_String )
{
	LinkedListNode<string> prev("0", NULL, NULL);
	LinkedListNode<string> curr("1", NULL, NULL);
	LinkedListNode<string> next("2", NULL, NULL);

	curr.InsertNode(&prev, &next);

	EXPECT_TRUE(curr.next == &next);
	EXPECT_TRUE(curr.prev == &prev);
	EXPECT_EQ(curr.value, "1");
	EXPECT_TRUE(prev.prev == NULL);
	EXPECT_TRUE(next.next == NULL);

	EXPECT_TRUE(prev.next == &curr);
	EXPECT_TRUE(next.prev == &curr);
}



/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|																	|
|					---- LinkedList ----							|
|																	|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/



TEST		( Empty_Full_Size_MaxSize, Int )
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


TEST		( Empty_Full_Size_MaxSize, String )
{
	util::LinkedList<string> linked;
	EXPECT_EQ(linked.Size(), 0);
	EXPECT_TRUE(linked.IsEmpty());
	EXPECT_FALSE(linked.IsFull());

	linked.PushBack("0");
	EXPECT_FALSE(linked.IsEmpty());
	EXPECT_FALSE(linked.IsFull());
	EXPECT_EQ(linked.Size(), 1);

	linked.PushBack("0");
	linked.PushBack("1");
	linked.PushBack("2");

	EXPECT_FALSE(linked.IsEmpty());
	EXPECT_FALSE(linked.IsFull());
	EXPECT_EQ(linked.Size(), 4);
}





/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|																	|
|						---- Push/Pop ----							|
|																	|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/



TEST		( PushBack,  WhenFull_Int )
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

TEST		( PushBack, String )
{
	util::LinkedList<string> linked;

	linked.PushBack("a");
	linked.PushBack("b");

	EXPECT_EQ(linked.PopBack()[0], 'b');
	EXPECT_EQ(linked.PopBack()[0], 'a');
}


TEST		( PushNodeBack, Int )
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


TEST		( PushNodeBack, String )
{
	util::LinkedList<string> linked;
	LinkedListNode<string>* n1 = new LinkedListNode<string>("a", NULL, NULL);
	LinkedListNode<string>* n2 = new LinkedListNode<string>("b", NULL, NULL);

	linked.PushNodeBack(n1);
	linked.PushNodeBack(n2);

	EXPECT_EQ(linked.PopBack()[0], 'b');
	EXPECT_EQ(linked.PopBack()[0], 'a');
}



TEST		( PopBack, All_Int )
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

	bool worked = false;
	try
	{
		linked.PopBack();
	}
	catch ( std::string )
	{
		worked = true;
	}
	EXPECT_TRUE(worked);
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

	bool worked = false;
	try
	{
		linked.PopFront();
	}
	catch ( std::string )
	{
		worked = true;
	}
	EXPECT_TRUE(worked);
	linked.PushBack(1);
	EXPECT_EQ(linked.PopFront(), 1);
	EXPECT_EQ(linked.Size(), 0);
}


TEST		( PopBack_PopFront, String )
{
	util::LinkedList<string> linked;
	linked.PushBack("a");
	linked.PushBack("b");
	linked.PushBack("c");

	EXPECT_EQ(linked.PopBack()[0], 'c');
	EXPECT_EQ(linked.PopFront()[0], 'a');
	EXPECT_EQ(linked.PopFront()[0], 'b');
}

/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|																	|
|							---- Other ----							|
|																	|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

TEST		( Operator, Int )
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

TEST		( Operator, String )
{
	util::LinkedList<string> linked;
	linked.PushBack("a");
	linked.PushBack("b");
	linked.PushBack("c");
	EXPECT_EQ(linked.Get(0)[0], 'a');
	EXPECT_EQ(linked.Get(1)[0], 'b');
	EXPECT_EQ(linked.Get(2)[0], 'c');
}



/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|																	|
|						---- Sorting ----							|
|																	|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

bool SortDecending ( int& a, int& b )
{
	return a >= b;
}


bool SortAscending ( string& a, string& b )
{
	return a[0] <= b[0];
}

TEST		( SortingMethods, JustCause )	// This is to stop lcov being annoying
{
	int a = 1, b = 2;
	EXPECT_TRUE(SortDecending(b, a));
	string c = "1", d = "2";
	EXPECT_TRUE(SortAscending(c, d));
}


TEST		( SortList, Int )
{
	util::LinkedList<int> input;
	input.PushBack(3);
	input.PushBack(1);
	input.PushBack(0);
	input.PushBack(5);
	input.PushBack(2);
	input.PushBack(4);
	input.PushBack(1);
	input.Sort(&SortDecending);

	EXPECT_EQ(input.Size(), 7);
	ASSERT_EQ(input.PopFront(), 5);
	ASSERT_EQ(input.PopFront(), 4);
	ASSERT_EQ(input.PopFront(), 3);
	ASSERT_EQ(input.PopFront(), 2);
	ASSERT_EQ(input.PopFront(), 1);

	ASSERT_EQ(input.PopBack(), 0);
	ASSERT_EQ(input.PopBack(), 1);
}




TEST		( SortList, String )
{
	string e1 = "a";
	string e2 = "b";
	string e3 = "c";
	string e4 = "d";
	string e5 = "e";
	string e6 = "f";

	util::LinkedList<string> input;
	input.PushBack(e4);
	input.PushBack(e1);
	input.PushBack(e5);
	input.PushBack(e3);
	input.PushBack(e6);
	input.PushBack(e2);
	input.Sort(&SortAscending);

	ASSERT_EQ(input.Size(), 6);
	ASSERT_EQ(input.PopFront()[0], 'a');
	ASSERT_EQ(input.PopFront()[0], 'b');
	ASSERT_EQ(input.PopFront()[0], 'c');
	ASSERT_EQ(input.PopFront()[0], 'd');
	ASSERT_EQ(input.PopFront()[0], 'e');
	ASSERT_EQ(input.PopFront()[0], 'f');
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
