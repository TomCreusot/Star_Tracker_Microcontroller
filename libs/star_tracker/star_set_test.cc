#include "gtest/gtest.h"
#include "star_set.h"
#include "libs/util/array_list.h"
#include "libs/util/array_list_mock.h"
#include "libs/util/point.h"

using namespace std;
using namespace util;
using namespace star_tracker;


//////////////////////////////////////////////////////////////////////////////
//																			//
//						------	Constructors	------						//
//																			//
//////////////////////////////////////////////////////////////////////////////

TEST ( DefaultConstructor, Valid )
{
	StarSet set;
	EXPECT_FLOAT_EQ(set.area, 0);
	EXPECT_FLOAT_EQ(set.moment, 0);
	EXPECT_FLOAT_EQ(set.position.x, 0);
	EXPECT_FLOAT_EQ(set.position.y, 0);
	EXPECT_FLOAT_EQ(set.vote, 1);
}


TEST ( AlternateConstructor, Valid )
{
	Point<decimal> pos(3, 4);
	StarSet set(pos, 0, 1);
	EXPECT_FLOAT_EQ(set.area, 0);
	EXPECT_FLOAT_EQ(set.moment, 1);
	EXPECT_FLOAT_EQ(set.position.x, 3);
	EXPECT_FLOAT_EQ(set.position.y, 4);
	EXPECT_FLOAT_EQ(set.vote, 1);
}



TEST ( CopyConstructor, Valid )
{
	Point<decimal> pos(3, 4);
	StarSet set(pos, 0, 1);

	StarSet set2(set);
	EXPECT_FLOAT_EQ(set2.area,			set.area);
	EXPECT_FLOAT_EQ(set2.moment,		set.moment);
	EXPECT_FLOAT_EQ(set2.position.x,	set.position.x);
	EXPECT_FLOAT_EQ(set2.position.y,	set.position.y);
	EXPECT_FLOAT_EQ(set2.vote, 			set.vote);
}




//////////////////////////////////////////////////////////////////////////////
//																			//
//						------	Find Center	------							//
//																			//
//////////////////////////////////////////////////////////////////////////////
/*
TEST ( FindCenter, WIP )
{
	EXPECT_TRUE(false);
}
*/






//////////////////////////////////////////////////////////////////////////////
//																			//
//						------	GenerateSets	------						//
//																			//
//////////////////////////////////////////////////////////////////////////////


TEST ( GenerateSets, InvalidInsufficientElements )
{
	ArrayListMock<Point<decimal>, 5, 100> input;
	ArrayList<StarSet, 100> output;

	input.PushBack(Point<decimal>(0, 0));
	input.PushBack(Point<decimal>(0, 1));
	input.PushBack(Point<decimal>(1, 0));
	input.PushBack(Point<decimal>(1, 1));
	input.PushBack(Point<decimal>(2, 2));
	StarSet::GenerateSets<5, 100>(input, 0, 2, 0, &StarSet::CartesianAngle, &output);
	EXPECT_EQ(input.get_list.Size(), 0);

	StarSet::GenerateSets<5, 100>(input, 3, 5, 0, &StarSet::CartesianAngle, &output);
	EXPECT_EQ(input.get_list.Size(), 0);

	StarSet::GenerateSets<5, 100>(input, 2, 4, 0, &StarSet::CartesianAngle, &output);
	EXPECT_EQ(input.get_list.Size(), 0);

	StarSet::GenerateSets<5, 100>(input, 5, 1, 0, &StarSet::CartesianAngle, &output);
	EXPECT_EQ(input.get_list.Size(), 0);
}




TEST ( GenerateSets, ValidFiveElements )
{
	ArrayListMock<Point<decimal>, 100, 100> input;
	ArrayList<StarSet, 100> output;

	input.PushBack(Point<decimal>(-1, 0));
	input.PushBack(Point<decimal>(1, 0));
	input.PushBack(Point<decimal>(2, 2));
	input.PushBack(Point<decimal>(3, 3));
	input.PushBack(Point<decimal>(4, 4));
	input.PushBack(Point<decimal>(5, 6));

	StarSet::GenerateSets<100, 100>(input, 1, 5, 0, &StarSet::CartesianAngle, &output);

	EXPECT_EQ(input.get_list.Get(0), 1);
	EXPECT_EQ(input.get_list.Get(1), 2);
	EXPECT_EQ(input.get_list.Get(2), 3);

	EXPECT_EQ(input.get_list.Get(3), 1);
	EXPECT_EQ(input.get_list.Get(4), 2);
	EXPECT_EQ(input.get_list.Get(5), 4);

	EXPECT_EQ(input.get_list.Get(6), 1);
	EXPECT_EQ(input.get_list.Get(7), 3);
	EXPECT_EQ(input.get_list.Get(8), 4);

	EXPECT_EQ(input.get_list.Get(9), 2);
	EXPECT_EQ(input.get_list.Get(10), 3);
	EXPECT_EQ(input.get_list.Get(11), 4);
}


//////////////////////////////////////////////////////////////////////////////
//																			//
//							------	Vote	------							//
//																			//
//////////////////////////////////////////////////////////////////////////////


TEST ( Vote, SingleElement )
{
	const int kSize = 10;
	ArrayList<StarSet, kSize> list;
	StarSet d1 ( Point<decimal>(0, 0), 0, 0 );
	StarSet p1 ( Point<decimal>(0, 0), 0, 0 );
	list.PushBack(d1);
	StarSet::Vote<kSize>(&list);

	EXPECT_FLOAT_EQ(list.Get(0).vote, 1);
}

TEST ( Vote, SameDistance )
{
	const int kSize = 10;
	ArrayList<StarSet, kSize> list;
	StarSet p1 ( Point<decimal>(M_PI / 2, 0), 0, 0 );
	StarSet d1 ( Point<decimal>(M_PI / 2, 0), 0, 0 );
	d1.pixel = &p1;

	StarSet p2 ( 0, 0, 0 );
	StarSet d2 ( Point<decimal>(0, 0), 0, 0 );
	d2.pixel = &p2;

	list.PushBack(d1);
	list.PushBack(d2);

	StarSet::Vote<kSize>(&list);

	EXPECT_FLOAT_EQ(list.Get(0).vote, 1);
	EXPECT_FLOAT_EQ(list.Get(1).vote, 1);
}


TEST ( Vote, InAccuracy )
{
	const int kSize = 10;
	ArrayList<StarSet, kSize> list;
	StarSet p1 ( Point<decimal>(1, 1), 0, 0 );
	StarSet d1 ( Point<decimal>(2, 2), 0, 0 );
	d1.pixel = &p1;

	StarSet p2 ( Point<decimal>(0.01, 0.01), 0, 0 );
	StarSet d2 ( Point<decimal>(0.6, 0.5), 0, 0 );
	d2.pixel = &p2;

	StarSet p3 ( Point<decimal>(0, 0), 0, 0 );
	StarSet d3 ( Point<decimal>(3, 1), 0, 0 );
	d3.pixel = &p3;
	list.PushBack(d1);
	list.PushBack(d2);
	list.PushBack(d3);
	StarSet::Vote<kSize>(&list);

	EXPECT_TRUE(list.Get(0).vote > list.Get(2).vote);
	EXPECT_TRUE(list.Get(0).vote > list.Get(1).vote);
	EXPECT_TRUE(list.Get(1).vote > list.Get(2).vote);
}














































//////////////////////////////////////////////////////////////////////////////
//																			//
//						------	VoteSingle	------							//
//																			//
//////////////////////////////////////////////////////////////////////////////

TEST ( VoteSingle, ZeroBounds )
{
	// a1, a2, m1, m2, ta, tm
	decimal result = star_tracker::StarSet::VoteSingle(1, 0, 1, 0, 1, 1);
	EXPECT_FLOAT_EQ(result, 0);
	result = StarSet::VoteSingle(-1000, 0, 1000, 0, 1000, 1000);
	EXPECT_FLOAT_EQ(result, 0);
	result = StarSet::VoteSingle(1000, -1000, 0, 0, 1000, 1000);
	EXPECT_FLOAT_EQ(result, 0);
}

TEST ( VoteSingle, MaxBounds )
{
	// a1, a2, m1, m2, ta, tm
	decimal result = StarSet::VoteSingle(0, 0, 0, 0, 0.01, 0.001);
	EXPECT_FLOAT_EQ(result, 1);
	result = StarSet::VoteSingle(-100, -99, 9, 10, 1, 1);
	EXPECT_FLOAT_EQ(result, 0);
}


TEST ( VoteSingle, HalfBounds )
{
	// a1, a2, m1, m2, ta, tm
	decimal result =	StarSet::VoteSingle(1, 0, 1, 0, 4, 4);
	EXPECT_FLOAT_EQ(result, 0.75);
	result = 			StarSet::VoteSingle(-1, 0, 0, 0, 2, 1);
	EXPECT_FLOAT_EQ(result, 0.75);
	result = 			StarSet::VoteSingle(1, 0, -1, 0, 2, 2);
	EXPECT_FLOAT_EQ(result, 0.5);
}





//////////////////////////////////////////////////////////////////////////////
//																			//
//						------	Unique	------								//
//																			//
//////////////////////////////////////////////////////////////////////////////

TEST ( CalcArea, Valid )
{

	decimal a = 3;
	decimal b = 4;
	decimal c = 5;
	decimal s = (a + b + c) / 2;
	decimal area = std::sqrt(s * (s - a) * (s - b) * (s - c));
	EXPECT_FLOAT_EQ(area, StarSet::CalcArea(a, b, c));
}


TEST ( CalcMoment, Valid )
{
	decimal area = 6;
	decimal a = 3;
	decimal b = 4;
	decimal c = 5;
	decimal moment = area * ( a * a + b * b + c * c ) / 36;
	EXPECT_FLOAT_EQ(moment, StarSet::CalcMoment(area, a, b, c));
}


//////////////////////////////////////////////////////////////////////////////
//																			//
//						------	Angle	------								//
//																			//
//////////////////////////////////////////////////////////////////////////////

TEST ( CartesianAngle, Valid )
{
	Point<decimal> s1(10, 10);
	Point<decimal> s2(0.5, 0.5);
	decimal angle = StarSet::CartesianAngle( s1, s2, 0.5 );
	EXPECT_FLOAT_EQ(angle, 6.717514421);
}


TEST ( EquatorialAngle, Valid )
{
	Equatorial<decimal> p1(1, 1);
	Equatorial<decimal> p2(2, 2);
	EXPECT_FLOAT_EQ(p1.RadialDistance(p2), StarSet::EquatorialAngle(p1, p2, 0));
}



//////////////////////////////////////////////////////////////////////////////
//																			//
//						------	Sorting	------								//
//																			//
//////////////////////////////////////////////////////////////////////////////

TEST ( SortByVoteAscending, Valid )
{
	StarSet a;
	StarSet b;
	a.vote = 10;
	b.vote = 10.01;
	EXPECT_TRUE(StarSet::SortByVoteAscending(a, b));
	EXPECT_FALSE(StarSet::SortByVoteAscending(b, a));
}

TEST ( SortByVoteDecending, Valid )
{
	StarSet a;
	StarSet b;
	a.vote = 10;
	b.vote = 10.01;
	EXPECT_FALSE(StarSet::SortByVoteDecending(a, b));
	EXPECT_TRUE(StarSet::SortByVoteDecending(b, a));
}
