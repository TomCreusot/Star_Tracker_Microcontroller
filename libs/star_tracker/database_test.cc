#include "gtest/gtest.h"
#include "database.h"
#include "star_set.h"
#include "libs/util/array_list.h"
#include "libs/util/array_list_mock.h"
#include "libs/util/point.h"

using namespace std;
using namespace util;
using namespace star_tracker;

//////////////////////////////////////////////////////////////////////////////
//																			//
//					------	To Array String	------							//
//																			//
//////////////////////////////////////////////////////////////////////////////

TEST ( ToArrayString, Valid )	// This could fail if the database structure is altered.
{
	string str;
	Point<decimal> pos(10.01, 100.01);
	StarSet set(pos, -10.01, 0.01);

	Database::ToArrayString(set, &str);
	EXPECT_EQ(str, "-10.010000, 0.010000, 10.010000, 100.010000");
}


//////////////////////////////////////////////////////////////////////////////
//																			//
//						------	FindElements	------						//
//																			//
//////////////////////////////////////////////////////////////////////////////

vector<array<decimal, Database::kNumElements>> sample_database =
{	//area, 	moment, ra, 	dec
	{	-5,		-5,		0,		-5   },
	{	-4,		-4,		0,		-4   },
	{	-3,		-3,		0,		-3   },
	{	-2,		-2,		0,		-2   },
	{	-1,		-1,		0,		-1   },
	{	0,		0,		0,		0   },
	{	1,		1,		0,		1   },
	{	2,		2,		0,		2   },
	{	3,		3,		0,		3   },
	{	4,		4,		0,		4   }
};
/*

TEST ( FindElements, InvalidVotes )
{
	Database database (0, 0, &array);
	StarSet::database = &database;

}*/


TEST ( FindElements, OutsideTolerance )
{
	// Setup Database
	Database database(0, 0, &sample_database);

	// Array List Sizes
	const uint NI = 10, NO = 100;
	ArrayList<StarSet, NI> in;
	ArrayList<StarSet, NO> out;
	StarSet set(Point<decimal>(0), -5.1, 0);
	in.PushBack(set);

	database.FindElements<NI, NO>(in, 0.01, 1000, 2, &out);
	EXPECT_EQ(out.Size(), 0);

	set = StarSet(Point<decimal>(0), 0, 4.1);
	in.Get(0) = set;
	database.FindElements<NI, NO>(in, 1000, 0.01, 2, &out);
	EXPECT_EQ(out.Size(), 0);
}

TEST ( FindElements, InsideTolerance )
{
	// Setup Database
	Database database(0, 0, &sample_database);

	// Array List Sizes
	const uint NI = 10, NO = 100;
	ArrayList<StarSet, NI> in;
	ArrayList<StarSet, NO> out;
	StarSet set(Point<decimal>(0), -6, 12);
	in.PushBack(set);

	// TEST AREA
	database.FindElements<NI, NO>(in, 1, 1000, 2, &out);
	EXPECT_EQ(out.Size(), 1);
	EXPECT_EQ(out.Get(0).area, -5);

	// TEST MOMENT
	set = StarSet(Point<decimal>(0), 10, 5);
	in.Get(0) = set;
	database.FindElements<NI, NO>(in, 1000, 1, 1, &out);
	EXPECT_EQ(out.Size(), 2);
	EXPECT_EQ(out.Get(1).area, 4);
}

TEST ( FindElements, MultipleInputs )
{
	// Setup Database
	Database database(0, 0, &sample_database);

	// Array List Sizes
	const uint NI = 10, NO = 100;
	ArrayList<StarSet, NI> in;
	ArrayList<StarSet, NO> out;
	StarSet set1(Point<decimal>(0), -5, -5);
	StarSet set2(Point<decimal>(0), -3, -3);
	StarSet set3(Point<decimal>(0), -1, -1);
	StarSet set4(Point<decimal>(0), 4, 4);
	in.PushBack(set1);
	in.PushBack(set2);
	in.PushBack(set3);
	in.PushBack(set4);
	out = ArrayList<StarSet, NO>();
	database.FindElements<NI, NO>(in, 0.5, 0.5, 100, &out);

	EXPECT_EQ(out.Size(), 4);
	EXPECT_EQ(out.Get(0).area, -5);
	EXPECT_EQ(out.Get(1).area, -3);
	EXPECT_EQ(out.Get(2).area, -1);
	EXPECT_EQ(out.Get(3).area, 4);
}


TEST ( FindElements, MultipleOutputs )
{
	// Setup Database
	Database database(0, 0, &sample_database);

	// Array List Sizes
	const uint NI = 10, NO = 6;
	ArrayList<StarSet, NI> in;
	ArrayList<StarSet, NO> out;
	StarSet set1(Point<decimal>(0), -3.01, -3.01);
	StarSet set2(Point<decimal>(0), 2.01, 2.01);
	in.PushBack(set1);
	in.PushBack(set2);
	out = ArrayList<StarSet, NO>();
	database.FindElements<NI, NO>(in, 1.1, 1.1, 100, &out);

	EXPECT_EQ(out.Size(), 6);
	EXPECT_EQ(out.Get(0).area, -2);
	EXPECT_EQ(out.Get(1).area, -4);
	EXPECT_EQ(out.Get(2).area, -3);
	EXPECT_EQ(out.Get(3).area, 1);
	EXPECT_EQ(out.Get(4).area, 3);
	EXPECT_EQ(out.Get(5).area, 2);
}


TEST ( FindElements, Overflow )
{
	// Setup Database
	Database database(0, 0, &sample_database);

	// Array List Sizes
	const uint NI = 10, NO = 5;
	ArrayList<StarSet, NI> in;
	ArrayList<StarSet, NO> out;
	StarSet set1(Point<decimal>(0), -10, -10);
	StarSet set2(Point<decimal>(0), 10, 10);
	in.PushBack(set1);
	in.PushBack(set2);
	out = ArrayList<StarSet, NO>();
	database.FindElements<NI, NO>(in, 100, 100, 3, &out);

	EXPECT_EQ(out.Size(), 5);
	EXPECT_EQ(out.Get(0).area, -3);
	EXPECT_EQ(out.Get(1).area, -4);
	EXPECT_EQ(out.Get(2).area, -5);
	EXPECT_EQ(out.Get(3).area, 3);
	EXPECT_EQ(out.Get(4).area, 4);
}
