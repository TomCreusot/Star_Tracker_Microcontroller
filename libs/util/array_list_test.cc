#include <cstring>

#include "gtest/gtest.h"
#include "array_list.h"

using namespace util;
using namespace std;

TEST		( DefaultConstructor, Int )
{
	util::ArrayList<int, 10> list;
	EXPECT_EQ(list.Size(), 0);
	EXPECT_EQ(list.MaxSize(), 10);
	EXPECT_TRUE(list.IsEmpty());
}


TEST		( DefaultConstructor, String )
{
	util::ArrayList<string, 0> list;
	EXPECT_EQ(list.Size(), 0);
	EXPECT_EQ(list.MaxSize(), 0);
	EXPECT_TRUE(list.IsEmpty());
}



TEST		( AlternateConstructor, Int )
{
	util::ArrayList<int, 4> list(3);
	EXPECT_EQ(list.Size(), 3);
	EXPECT_EQ(list.MaxSize(), 4);

	util::ArrayList<int, 3> listB(10);
	EXPECT_EQ(listB.Size(), 3);
	EXPECT_EQ(listB.MaxSize(), 3);
}


TEST		( AlternateConstructor, String )
{
	util::ArrayList<string, 10> list(3);
	EXPECT_EQ(list.Size(), 3);
	EXPECT_EQ(list.MaxSize(), 10);

	util::ArrayList<string, 1> listB(10);
	EXPECT_EQ(listB.Size(), 1);
	EXPECT_EQ(listB.MaxSize(), 1);
}





TEST		( CopyConstructor, Int )
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


TEST		( CopyConstructor, String )
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



TEST		( Empty_Full_Size_MaxSize, Int )
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


TEST		( Empty_Full_Size_MaxSize, String )
{
	util::ArrayList<string, 1> list;
	EXPECT_EQ(list.Size(), 0);
	EXPECT_TRUE(list.IsEmpty());
	EXPECT_FALSE(list.IsFull());

	list.PushBack("0");
	EXPECT_FALSE(list.IsEmpty());
	EXPECT_TRUE(list.IsFull());
	EXPECT_EQ(list.Size(), 1);
}




TEST 		( ReduceSize, WhenGreater_Int )
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


TEST 		( ReduceSize, WhenValid_Int )
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


TEST		( ReduceSize, String )
{
	util::ArrayList<string, 3> list;
	list.PushBack("1");
	list.PushBack("2");
	list.ReduceSize(1);
	EXPECT_EQ(list.Size(), 1);
	EXPECT_FALSE(list.IsEmpty());
}



/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|																	|
|						---- Push/Pop ----							|
|																	|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/



TEST		( PushBack,  Int )
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

TEST 	( PushBackValid, Int )
{
	util::ArrayList<int, 5> list;
	EXPECT_FALSE(list.PushBackValid(1, false));
	EXPECT_EQ(list.Size(), 0);
	EXPECT_TRUE(list.PushBackValid(2, true));
	EXPECT_EQ(list.Get(0), 2);
	EXPECT_TRUE(list.PushBackValid(2, true));
	EXPECT_TRUE(list.PushBackValid(2, true));
	EXPECT_TRUE(list.PushBackValid(2, true));
	EXPECT_TRUE(list.PushBackValid(2, true));
	EXPECT_FALSE(list.PushBackValid(1, true));
	EXPECT_EQ(list.Size(), 5);
}


TEST		( PushBack,  String )
{
	util::ArrayList<string, 100> list;

	for ( uint i = 0; i < 100; i++ )
	EXPECT_TRUE(list.PushBack("1"));

	EXPECT_TRUE(list.IsFull());
	EXPECT_FALSE(list.IsEmpty());
	EXPECT_EQ(list.Size(), 100);

	EXPECT_FALSE(list.PushBack("2"));

	EXPECT_TRUE(list.IsFull());
	EXPECT_FALSE(list.IsEmpty());
	EXPECT_EQ(list.Size(), 100);
}

TEST 	( PushBackValid, String )
{
	util::ArrayList<string, 5> list;
	EXPECT_FALSE(list.PushBackValid("abc", false));
	EXPECT_EQ(list.Size(), 0);
	EXPECT_TRUE(list.PushBackValid("def", true));
	EXPECT_EQ(list.Get(0)[0], 'd');
	EXPECT_TRUE(list.PushBackValid("def", true));
	EXPECT_TRUE(list.PushBackValid("def", true));
	EXPECT_TRUE(list.PushBackValid("def", true));
	EXPECT_TRUE(list.PushBackValid("def", true));
	EXPECT_FALSE(list.PushBackValid("fed", true));
	EXPECT_EQ(list.Size(), 5);
}


TEST		( PopBack, Int )
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


TEST		( PopBack, Int_WhenEmpty )
{
	const int size = 5;
	util::ArrayList<int, size> list;

	list.PushBack(1);
	list.PushBack(2);
	EXPECT_EQ(list.PopBack(), 2);
	EXPECT_EQ(list.PopBack(), 1);
	EXPECT_EQ(list.PopBack(), 1);
}


TEST		( PopBack, Int_WhenFull )
{
	const int size = 5;
	util::ArrayList<int, size> list;

	for ( uint i = 0; i < size; i++ ) list.PushBack(0);
	list.PushBack(1);

	EXPECT_EQ(list.PopBack(), 0);
	EXPECT_EQ(list.PopBack(), 0);
}


TEST		( PopBack,  String )
{
	util::ArrayList<string, 100> list;
	list.PushBack("1");

	EXPECT_EQ(list.PopBack()[0], '1');
	EXPECT_EQ(list.Size(), 0);
}


/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|																	|
|							---- Other ----							|
|																	|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

TEST		( Operator, Int )
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

TEST		( Operator, String )
{
	const int size = 100;
	util::ArrayList<string, size> list;
	list.PushBack("1");
	EXPECT_EQ(list.Get(0)[0], '1');
}



// CallAll
void AddTen	( int* val )
{
	*val += 10;
}

void SwapFrontCharacters ( string* val )
{
	char temp = (*val)[0];
	(*val)[0] = (*val)[1];
	(*val)[1] = temp;
}

TEST 		( CallALL, Int )
{
	const int size = 10;
	util::ArrayList<int, size> list;
	list.PushBack(1);
	list.PushBack(2);
	list.PushBack(3);
	list.PushBack(4);
	list.PopBack();

	list.CallAll(&AddTen);

	EXPECT_EQ(list.Get(0), 11);
	EXPECT_EQ(list.Get(1), 12);
	EXPECT_EQ(list.Get(2), 13);
	EXPECT_EQ(list.Get(3), 4);
}

TEST 		( CallALL, String )
{
	const int size = 10;

	string e1 = "hello";
	string e2 = "world";

	util::ArrayList<string, size> list;
	list.PushBack(e1);
	list.PushBack(e2);
	list.PushBack(e2);
	list.PopBack();

	list.CallAll(&SwapFrontCharacters);

	EXPECT_EQ(list.Get(0)[0], 'e');
	EXPECT_EQ(list.Get(0)[1], 'h');
	EXPECT_EQ(list.Get(1)[0], 'o');
	EXPECT_EQ(list.Get(1)[1], 'w');
	EXPECT_EQ(list.Get(2)[0], 'w');
	EXPECT_EQ(list.Get(2)[1], 'o');
}



// Sort
bool SortDecending ( float& a, float& b )
{
	return a >= b;
}


bool SortAscending ( string& a, string& b )
{
	return a[0] <= b[0];
}

bool SortAscending ( float& a, float& b )
{
	return a <= b;
}


TEST		( SortList, Int )
{
	const int size = 10;
	util::ArrayList<float, size> input;
	input.PushBack(3);
	input.PushBack(1);
	input.PushBack(0);
	input.PushBack(5);
	input.PushBack(2);
	input.PushBack(4);
	input.PushBack(1);
	input.Sort(&SortDecending);

	EXPECT_FLOAT_EQ(input.Get(0), 5);
	EXPECT_FLOAT_EQ(input.Get(1), 4);
	EXPECT_FLOAT_EQ(input.Get(2), 3);
	EXPECT_FLOAT_EQ(input.Get(3), 2);
	EXPECT_FLOAT_EQ(input.Get(4), 1);
	EXPECT_FLOAT_EQ(input.Get(5), 1);
	EXPECT_FLOAT_EQ(input.Get(6), 0);
	EXPECT_FLOAT_EQ(input.Size(), 7);
}




TEST		( SortList, String )
{
	const int size = 10;

	string e1 = "1";
	string e2 = "2";
	string e3 = "3";
	string e4 = "4";
	string e5 = "5";


	util::ArrayList<string, size> input;
	input.PushBack(e4);
	input.PushBack(e1);
	input.PushBack(e5);
	input.PushBack(e3);
	input.PushBack(e2);
	input.Sort(&SortAscending);

	ASSERT_EQ(input.Get(0)[0], '1');
	ASSERT_EQ(input.Get(1)[0], '2');
	ASSERT_EQ(input.Get(2)[0], '3');
	ASSERT_EQ(input.Get(3)[0], '4');
	ASSERT_EQ(input.Get(4)[0], '5');
	ASSERT_EQ(input.Size(), 5);
}

/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|																	|
|							---- Slot ----							|
|																	|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

TEST ( Slot, Ascending )
{
	// SlotFilling should always return true as it will fill.
	// SlotFull if always sent smaller values will return false.
	const uint size = 6;
	util::ArrayList<float, size> input;

	bool (*in_order)(float&, float&) = &SortAscending;
	float to_slot = 0;
	EXPECT_TRUE(input.Slot(to_slot, in_order));
	to_slot = 1;
	EXPECT_TRUE(input.Slot(to_slot, in_order));
	to_slot = 2;
	EXPECT_TRUE(input.Slot(to_slot, in_order));
	to_slot = 3;
	EXPECT_TRUE(input.Slot(to_slot, in_order));
	to_slot = 4;
	EXPECT_TRUE(input.Slot(to_slot, in_order));
	to_slot = 5;
	EXPECT_TRUE(input.Slot(to_slot, in_order));

	EXPECT_EQ(input.Size(), size);
	// Full, 0, 1, 2, 3, 4, 5
	to_slot = -1;
	EXPECT_TRUE(input.Slot(to_slot, in_order));//-1, 0, 1, 2, 3, 4
	to_slot = -2;
	EXPECT_TRUE(input.Slot(to_slot, in_order));//-2, -1, 0, 1, 2, 3
	to_slot = 1;
	EXPECT_TRUE(input.Slot(to_slot, in_order));//-2, -1, 0, 1, 1, 2
	to_slot = 10;
	EXPECT_FALSE(input.Slot(to_slot, in_order));//-2, -1, 0, 1, 1, 2
}


TEST ( Slot, Decending )
{
	// SlotFilling should always return true as it will fill.
	// SlotFull if always sent smaller values will return false.
	const uint size = 6;
	util::ArrayList<float, size> input;

	bool (*in_order)(float&, float&) = &SortDecending;
	float to_slot = 5;
	EXPECT_TRUE(input.Slot(to_slot, in_order));
	to_slot = 4;
	EXPECT_TRUE(input.Slot(to_slot, in_order));
	to_slot = 3;
	EXPECT_TRUE(input.Slot(to_slot, in_order));
	to_slot = 2;
	EXPECT_TRUE(input.Slot(to_slot, in_order));
	to_slot = 1;
	EXPECT_TRUE(input.Slot(to_slot, in_order));
	to_slot = 0;
	EXPECT_TRUE(input.Slot(to_slot, in_order));

	EXPECT_EQ(input.Size(), size);
	// Full, 5, 4, 3, 2, 1
	to_slot = -1;
	EXPECT_FALSE(input.Slot(to_slot, in_order));//5, 4, 3, 2, 1, 0
	to_slot = -2;
	EXPECT_FALSE(input.Slot(to_slot, in_order));//5, 4, 3, 2, 1, 0
	to_slot = 1;
	EXPECT_TRUE(input.Slot(to_slot, in_order));//5, 4, 3, 2, 1, 1
	to_slot = 10;
	EXPECT_TRUE(input.Slot(to_slot, in_order));//10, 5, 4, 3, 2, 1
	to_slot = 100;
	EXPECT_TRUE(input.Slot(to_slot, in_order)); // 100, 10, 5, 4, 3, 2
	to_slot = 10;
	EXPECT_TRUE(input.Slot(to_slot, in_order)); // 100, 10, 10, 5, 4, 3
}
