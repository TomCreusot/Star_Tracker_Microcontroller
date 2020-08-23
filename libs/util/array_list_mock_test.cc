#include <cstring>

#include "gtest/gtest.h"
#include "array_list.h"
#include "array_list_mock.h"

using namespace util;
using namespace std;

TEST		( PushBack, Int )
{
	util::ArrayListMock<int, 10, 5> list;
	util::ArrayList<int, 10>* l = &list;
	l->PushBack(10);
	l->PushBack(2);
	l->PushBack(1);

	l->Get(0) = 3;

	EXPECT_EQ(list.push_list.Get(0), 10);
	EXPECT_EQ(list.push_list.Get(1), 2);
	EXPECT_EQ(list.push_list.Get(2), 1);
	EXPECT_EQ(list.Get(0), 3);
}



TEST		( PushBack, String )
{
	util::ArrayListMock<string, 6, 10> list;
	util::ArrayList<string, 6>* l = &list;
	l->PushBack("hello");
	l->PushBack("world");
	l->PushBack("!");

	l->Get(0) = "abcdef";

	EXPECT_EQ(list.push_list.Get(0), "hello");
	EXPECT_EQ(list.push_list.Get(1), "world");
	EXPECT_EQ(list.push_list.Get(2), "!");
	EXPECT_EQ(list.Get(0), "abcdef");
}














TEST		( PopBack, Int )
{
	util::ArrayListMock<int, 9, 10> list;
	util::ArrayList<int, 9>* l = &list;
	l->PushBack(10);
	l->PushBack(2);
	l->PushBack(23);

	l->PopBack();
	l->PopBack();
	l->PopBack();
	l->PopBack();

	l->Get(0) = 234;

	EXPECT_EQ(list.pop_list.Get(0), 23);
	EXPECT_EQ(list.pop_list.Get(1), 2);
	EXPECT_EQ(list.pop_list.Get(2), 10);
	EXPECT_EQ(list.pop_list.Get(3), 10);
	EXPECT_EQ(list.Get(0), 234);
}

TEST		( PopBack, String )
{
	util::ArrayListMock<string, 9, 10> list;
	util::ArrayList<string, 9>* l = &list;
	l->PushBack("hello");
	l->PushBack("world");
	l->PushBack("!");

	l->PopBack();
	l->PopBack();
	l->PopBack();
	l->PopBack();

	l->Get(0) = "abcdef";

	EXPECT_EQ(list.pop_list.Get(0), "!");
	EXPECT_EQ(list.pop_list.Get(1), "world");
	EXPECT_EQ(list.pop_list.Get(2), "hello");
	EXPECT_EQ(list.pop_list.Get(3), "hello");
	EXPECT_EQ(list.Get(0), "abcdef");
}









TEST		( Get, Int )
{
	util::ArrayListMock<int, 4, 10> list;
	util::ArrayList<int, 4>* l = &list;
	l->PushBack(10);
	l->PushBack(2);
	l->PushBack(23);

	l->Get(0) = 3;
	l->Get(2) = 3;
	l->Get(1) = 3;


	EXPECT_EQ(list.get_list.Get(0), 0);
	EXPECT_EQ(list.get_list.Get(1), 2);
	EXPECT_EQ(list.get_list.Get(2), 1);
}


TEST		( Get, String )
{
	util::ArrayListMock<string, 4, 10> list;
	util::ArrayList<string, 4>* l = &list;
	l->PushBack("hello");
	l->PushBack("world");
	l->PushBack("!");

	l->Get(0) = "hello";
	l->Get(2) = "world";
	l->Get(1) = "!";


	EXPECT_EQ(list.get_list.Get(0), 0);
	EXPECT_EQ(list.get_list.Get(1), 2);
	EXPECT_EQ(list.get_list.Get(2), 1);
}
