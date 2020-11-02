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
//						------	To Array	------							//
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


TEST ( ToArray, Valid )	// This could fail if the database structure is altered.
{
	array<decimal, Database::kNumElements> a;
	Point<decimal> pos(10.01, 100.01);
	StarSet set(pos, -10.01, 0.01);

	Database::ToArray(set, &a);
	EXPECT_FLOAT_EQ(a[0], -10.010000);
	EXPECT_FLOAT_EQ(a[1], 0.010000);
	EXPECT_FLOAT_EQ(a[2], 10.010000);
	EXPECT_FLOAT_EQ(a[3], 100.010000);
}














TEST ( DatabaseToStar, Valid )	// This could fail if the database structure is altered.
{
	vector<array<decimal, Database::kNumElements>> a(1);
	a[0][Database::kIndexRA] = 1;
	a[0][Database::kIndexDEC] = 2;
	a[0][Database::kIndexArea] = 3;
	a[0][Database::kIndexMoment] = 4;
	Database db(0, &a);

	StarSet set;
	db.DatabaseToStar(0, &set);

	EXPECT_FLOAT_EQ(set.position.Ra(), 1);
	EXPECT_FLOAT_EQ(set.position.Dec(), 2);
	EXPECT_FLOAT_EQ(set.area, 3);
	EXPECT_FLOAT_EQ(set.moment, 4);
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
	Database database(0, &sample_database);

	// Array List Sizes
	const uint NI = 10, NO = 100;
	ArrayList<StarSet, NI> in;
	ArrayList<StarSet, NO> out;
	StarSet set(Point<decimal>(0), -5.1, 0);
	in.PushBack(set);

	database.FindElements<NI, NO>(in, 0.01, 1000, &out);
	EXPECT_EQ(out.Size(), 0);

	set = StarSet(Point<decimal>(0), 0, 4.1);
	in.Get(0) = set;
	database.FindElements<NI, NO>(in, 1000, 0.01, &out);
	EXPECT_EQ(out.Size(), 0);
}

TEST ( FindElements, InsideTolerance )
{
	// Setup Database
	Database database(0, &sample_database);

	// Array List Sizes
	const uint NI = 10, NO = 100;
	ArrayList<StarSet, NI> in;
	ArrayList<StarSet, NO> out;
	StarSet set(Point<decimal>(0), -6, 12);
	in.PushBack(set);

	// TEST AREA
	database.FindElements<NI, NO>(in, 1, 1000, &out);
	EXPECT_EQ(out.Size(), 1);
	EXPECT_EQ(out.Get(0).area, -5);

	// TEST MOMENT
	out = ArrayList<StarSet, NO>();
	set = StarSet(Point<decimal>(0), 13, 5);
	in.Get(0) = set;
	database.FindElements<NI, NO>(in, 1000, 1, &out);
	EXPECT_EQ(out.Get(0).area, 4);



}

TEST ( FindElements, MultipleInputs )
{
	// Setup Database
	Database database(0, &sample_database);

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
	database.FindElements<NI, NO>(in, 0.5, 0.5, &out);

	EXPECT_EQ(out.Size(), 4);
	EXPECT_EQ(out.Get(0).area, -5);
	EXPECT_EQ(out.Get(1).area, -3);
	EXPECT_EQ(out.Get(2).area, -1);
	EXPECT_EQ(out.Get(3).area, 4);
}


TEST ( FindElements, MultipleOutputs )
{
	// Setup Database
	Database database(0, &sample_database);

	// Array List Sizes
	const uint NI = 10, NO = 6;
	ArrayList<StarSet, NI> in;
	ArrayList<StarSet, NO> out;
	StarSet set1(Point<decimal>(0), -3.01, -3.01);
	StarSet set2(Point<decimal>(0), 2.01, 2.01);
	in.PushBack(set1);
	in.PushBack(set2);
	out = ArrayList<StarSet, NO>();
	database.FindElements<NI, NO>(in, 1.1, 1.1, &out);

	EXPECT_EQ(out.Size(), 6);
	EXPECT_EQ(out.Get(0).area, -3);
	EXPECT_EQ(out.Get(1).area, 2);
	EXPECT_EQ(out.Get(2).area, -4);
	EXPECT_EQ(out.Get(3).area, 3);
	EXPECT_EQ(out.Get(4).area, -2);
	EXPECT_EQ(out.Get(5).area, 1);
}


TEST ( FindElements, Overflow )
{
	// Setup Database
	Database database(0, &sample_database);

	// Array List Sizes
	const uint NI = 10, NO = 4;
	ArrayList<StarSet, NI> in;
	ArrayList<StarSet, NO> out;
	// The moment and area are on the outsides of the list.
	StarSet set1(Point<decimal>(0), -10, -10);
	StarSet set2(Point<decimal>(0), 10, 10);
	in.PushBack(set1);
	in.PushBack(set2);
	out = ArrayList<StarSet, NO>();
	database.FindElements<NI, NO>(in, 100, 100, &out);

	EXPECT_EQ(out.Size(), 4);
	EXPECT_EQ(out.Get(0).area, -5); // Will search set1 first.
	EXPECT_EQ(out.Get(1).area, -4);
	EXPECT_EQ(out.Get(2).area, 4);
	EXPECT_EQ(out.Get(3).area, -3);
}
