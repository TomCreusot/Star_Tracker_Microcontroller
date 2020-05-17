#include "gtest/gtest.h"
#include "star_set.h"
#include "libs/util/array_list.h"
#include "libs/util/array_list_mock.h"
#include "libs/util/point.h"
#include <iostream>

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
	star_tracker::StarSet set;
	EXPECT_FLOAT_EQ(set.angle, 0);
	EXPECT_FLOAT_EQ(set.opposite.x, 0);
	EXPECT_FLOAT_EQ(set.opposite.y, 0);
	EXPECT_FLOAT_EQ(set.distance, 0);
	EXPECT_FLOAT_EQ(set.odds, 1);
	EXPECT_FLOAT_EQ(set.orientation, 0);
}


TEST ( AlternateConstructor, WIP )
{
	EXPECT_TRUE(false);
}





TEST ( CopyConstructor, Valid )
{
	Point<decimal> oppos = Point<decimal>(22, 23);
	star_tracker::StarSet set;
	set.angle = 123;
	set.opposite = oppos;
	set.distance = 124;
	set.odds = 321;
	set.pixel = NULL;
	set.orientation = 12;

	star_tracker::StarSet set2(set);
	EXPECT_FLOAT_EQ(set2.angle, 123);
	EXPECT_FLOAT_EQ(set2.opposite.x, 22);
	EXPECT_FLOAT_EQ(set2.opposite.y, 23);
	EXPECT_FLOAT_EQ(set2.distance, 124);
	EXPECT_FLOAT_EQ(set2.odds, 321);
	EXPECT_FLOAT_EQ(set2.orientation, 12);
}



//////////////////////////////////////////////////////////////////////////////
//																			//
//					------	GenerateSetsPilots	------						//
//																			//
//////////////////////////////////////////////////////////////////////////////

TEST ( GenerateSetsPilots, InvalidInsufficientElements )
{
	util::ArrayListMock<util::Point<util::decimal>, 5, 100> input;
	util::ArrayList<star_tracker::StarSet, 100> output;

	input.PushBack(util::Point<util::decimal>(0, 0));
	input.PushBack(util::Point<util::decimal>(0, 1));
	input.PushBack(util::Point<util::decimal>(1, 0));
	input.PushBack(util::Point<util::decimal>(1, 1));
	input.PushBack(util::Point<util::decimal>(2, 2));

	star_tracker::StarSet::GenerateSetsPilots<5, 100>(input, 0, 2, &output);
	EXPECT_EQ(input.get_list.Size(), 0);

	star_tracker::StarSet::GenerateSetsPilots<5, 100>(input, 2, 5, &output);
	EXPECT_EQ(input.get_list.Size(), 0);

	star_tracker::StarSet::GenerateSetsPilots<5, 100>(input, 1, 4, &output);
	EXPECT_EQ(input.get_list.Size(), 0);

	star_tracker::StarSet::GenerateSetsPilots<5, 100>(input, 5, 1, &output);
	EXPECT_EQ(input.get_list.Size(), 0);
}




TEST ( GenerateSetsPilots, ValidFiveElements )
{
	util::ArrayListMock<util::Point<util::decimal>, 5, 100> input;
	util::ArrayList<star_tracker::StarSet, 100> output;

	input.PushBack(util::Point<util::decimal>(0, 0));
	input.PushBack(util::Point<util::decimal>(0, 1));
	input.PushBack(util::Point<util::decimal>(1, 0));
	input.PushBack(util::Point<util::decimal>(1, 1));
	input.PushBack(util::Point<util::decimal>(2, 2));

	star_tracker::StarSet::GenerateSetsPilots<5, 100>(input, 0, 5, &output);

	EXPECT_EQ(input.get_list.Get(0), 0);
	EXPECT_EQ(input.get_list.Get(1), 1);
	EXPECT_EQ(input.get_list.Get(2), 2);
	EXPECT_EQ(input.get_list.Get(3), 3);

	EXPECT_EQ(input.get_list.Get(4), 0);
	EXPECT_EQ(input.get_list.Get(5), 1);
	EXPECT_EQ(input.get_list.Get(6), 2);
	EXPECT_EQ(input.get_list.Get(7), 4);

	EXPECT_EQ(input.get_list.Get(8), 0);
	EXPECT_EQ(input.get_list.Get(9), 1);
	EXPECT_EQ(input.get_list.Get(10), 3);
	EXPECT_EQ(input.get_list.Get(11), 4);

	EXPECT_EQ(input.get_list.Get(12), 0);
	EXPECT_EQ(input.get_list.Get(13), 2);
	EXPECT_EQ(input.get_list.Get(14), 3);
	EXPECT_EQ(input.get_list.Get(15), 4);

	EXPECT_EQ(input.get_list.Get(16), 1);
	EXPECT_EQ(input.get_list.Get(17), 2);
	EXPECT_EQ(input.get_list.Get(18), 3);
	EXPECT_EQ(input.get_list.Get(19), 4);

	EXPECT_EQ(input.get_list.Size(), 20);
}







//////////////////////////////////////////////////////////////////////////////
//																			//
//					------	ClusterProbability	------						//
//																			//
//////////////////////////////////////////////////////////////////////////////

TEST ( ClusterProbability, SingleElement )
{
	const util::uint input_size = 1;
	star_tracker::StarSet set1;
	util::ArrayList<star_tracker::StarSet, input_size> sets;
	sets.PushBack(set1);

	star_tracker::StarSet::set_fov(0);
	star_tracker::StarSet::ClusterProbability<input_size>(&sets);

	EXPECT_FLOAT_EQ(sets.PopBack().odds, 1);
}


TEST ( ClusterProbability, TwoElements )
{
	const util::uint input_size = 2;

	star_tracker::StarSet set1;
	star_tracker::StarSet set2;

	set1.opposite = util::Point<util::decimal>(0, 0);
	set2.opposite = util::Point<util::decimal>(-3, 4);

	util::ArrayList<star_tracker::StarSet, input_size> sets;
	sets.PushBack(set1);
	sets.PushBack(set2);

	StarSet::set_fov(5.001);
	star_tracker::StarSet::ClusterProbability<input_size>(&sets);

	EXPECT_FLOAT_EQ(sets.Get(0).odds, 1);
	EXPECT_FLOAT_EQ(sets.Get(1).odds, 1);


	StarSet::set_fov(4.999);
	star_tracker::StarSet::ClusterProbability<input_size>(&sets);

	EXPECT_FLOAT_EQ(sets.Get(0).odds, 1 / star_tracker::StarSet::kSeparationDiv);
	EXPECT_FLOAT_EQ(sets.Get(1).odds, 1 / star_tracker::StarSet::kSeparationDiv);
}


TEST ( ClusterProbability, ThreeElements )
{
	const util::uint input_size = 3;

	star_tracker::StarSet set1;
	star_tracker::StarSet set2;
	star_tracker::StarSet set3;

	set1.opposite = util::Point<util::decimal>(0, 0);
	set2.opposite = util::Point<util::decimal>(-3, 4);
	set3.opposite = util::Point<util::decimal>(10, 10);

	util::ArrayList<star_tracker::StarSet, input_size> sets;
	sets.PushBack(set1);
	sets.PushBack(set2);
	sets.PushBack(set3);


	StarSet::set_fov(5.001);
	star_tracker::StarSet::ClusterProbability<input_size>(&sets);

	EXPECT_FLOAT_EQ(sets.Get(0).odds, 1 / star_tracker::StarSet::kSeparationDiv);
	EXPECT_FLOAT_EQ(sets.Get(1).odds, 1 / star_tracker::StarSet::kSeparationDiv);
	EXPECT_FLOAT_EQ(sets.Get(2).odds, 1 / star_tracker::StarSet::kSeparationDiv / star_tracker::StarSet::kSeparationDiv);


	sets.Get(0).odds = 1;
	sets.Get(1).odds = 1;
	sets.Get(2).odds = 1;
	StarSet::set_fov(4.999);
	star_tracker::StarSet::ClusterProbability<input_size>(&sets);
	EXPECT_FLOAT_EQ(sets.Get(0).odds, 1 / star_tracker::StarSet::kSeparationDiv / star_tracker::StarSet::kSeparationDiv);
	EXPECT_FLOAT_EQ(sets.Get(1).odds, 1 / star_tracker::StarSet::kSeparationDiv / star_tracker::StarSet::kSeparationDiv);
	EXPECT_FLOAT_EQ(sets.Get(2).odds, 1 / star_tracker::StarSet::kSeparationDiv / star_tracker::StarSet::kSeparationDiv);
}



//////////////////////////////////////////////////////////////////////////////
//																			//
//						------	Find Center	------							//
//																			//
//////////////////////////////////////////////////////////////////////////////

TEST ( FindCenter, WIP )
{
	EXPECT_TRUE(false);
}




//////////////////////////////////////////////////////////////////////////////
//																			//
//						------	Find Center	------							//
//																			//
//////////////////////////////////////////////////////////////////////////////

TEST ( ToArrayString, WIP )
{
	string str;
	star_tracker::StarSet set;
	set.angle = -10.01;
	set.distance = 0.01;
	set.orientation = 1.01;
	set.opposite.x = 10.01;
	set.opposite.y = 100.01;

	set.ToArrayString(&str);
	EXPECT_EQ(str, "-10.010000, 0.010000, 1.010000, 10.010000, 100.010000");
}





//////////////////////////////////////////////////////////////////////////////
//																			//
//						------	SortDistance	------						//
//																			//
//////////////////////////////////////////////////////////////////////////////

TEST ( SortDistance, AlreadySorted )
{
	util::Point<util::decimal> p(10, 10);
	util::Point<util::decimal> s1(11, 1000);
	util::Point<util::decimal> s2(12, 1000);

	star_tracker::StarSet::SortDistance(p, &s1, &s2);

	EXPECT_FLOAT_EQ(s1.x, 11);
	EXPECT_FLOAT_EQ(s2.x, 12);
}


TEST ( SortDistance, Unsorted )
{
	util::Point<util::decimal> p(10, 10);
	util::Point<util::decimal> s1(-1, 1000);
	util::Point<util::decimal> s2(102, 1000);

	star_tracker::StarSet::SortDistance(p, &s2, &s1);

	EXPECT_FLOAT_EQ(s1.x, 102);
	EXPECT_FLOAT_EQ(s2.x, -1);
}


TEST ( SortDistance, SameDistance )
{
	util::Point<util::decimal> p(0, 0);
	util::Point<util::decimal> s1(10, -1000);
	util::Point<util::decimal> s2(-10, 1000);

	star_tracker::StarSet::SortDistance(p, &s1, &s2);

	EXPECT_FLOAT_EQ(s1.x, 10);
	EXPECT_FLOAT_EQ(s2.x, -10);
}








//////////////////////////////////////////////////////////////////////////////
//																			//
//						------	FindAngle	------							//
//																			//
//////////////////////////////////////////////////////////////////////////////

TEST ( FindAngle, Valid )
{
	Point<decimal> hypot(0, 10);
	Point<decimal> adjac(-4, 7);
	Point<decimal> oppos(8, 4);

	decimal angle = star_tracker::StarSet::FindAngle(hypot, adjac, oppos);
	EXPECT_FLOAT_EQ(angle, 1.8545904360032242d);
}


TEST ( FindAngle, EqualSideLengths )
{
	Point<decimal> pilot(10, -5);
	Point<decimal> hypot(10, -10);
	Point<decimal> adjac(9, -5);
	Point<decimal> oppos(11, -5);

	decimal angle = star_tracker::StarSet::FindAngle(hypot, adjac, oppos);
	EXPECT_FLOAT_EQ(angle, 0.3947911197);
}


TEST ( FindAngle, PointsAwayFromPilot )
{
	Point<decimal> pilot(0, 0);
	Point<decimal> hypot(5, 5);
	Point<decimal> adjac(0, 5);
	Point<decimal> oppos(1, 6);

	decimal angle = star_tracker::StarSet::FindAngle(hypot, adjac, oppos);
	EXPECT_FLOAT_EQ(angle, 0.2449786631);
}



TEST ( FindAngle, RealExample )
{
	// Orions belt from hyg database
	Point<decimal> pilot(5.603559,	-1.20192);	// Alnilam, mag: 1.69
	Point<decimal> hypot(5.645769,	-2.600069);	// 48 ori, mag: 3.77
	Point<decimal> adjac(5.679313, -1.942572);	// Alnitak, mag: 1.74
	Point<decimal> oppos(5.533445,	-0.299092);	// Mintaka, mag: 2.25


	decimal angle = star_tracker::StarSet::FindAngle(hypot, adjac, oppos);
	EXPECT_FLOAT_EQ(angle, 0.099750593); //, 0.099753); (Close enough?)
}



// When things break.
TEST ( FindAngle, ValuesEqual )
{
	Point<decimal> oppos(0, 0);
	Point<decimal> adjac(0, 0);
	Point<decimal> hypot(0, 0);

	decimal angle = star_tracker::StarSet::FindAngle(oppos, adjac, hypot);
	EXPECT_TRUE(isnan(angle));

	oppos.Set(0, 1);
	adjac.Set(0, 0);
	hypot.Set(0, 0);

	angle = star_tracker::StarSet::FindAngle(oppos, adjac, hypot);
	EXPECT_TRUE(isnan(angle));

	oppos.Set(0, 1);
	adjac.Set(0, 1);
	hypot.Set(0, 0);

	angle = star_tracker::StarSet::FindAngle(oppos, adjac, hypot);
	EXPECT_TRUE(isnan(angle));
}



//////////////////////////////////////////////////////////////////////////////
//																			//
//					------	FindRealDistance	------						//
//																			//
//////////////////////////////////////////////////////////////////////////////

TEST ( FindRealDistance, Valid )
{
	Point<decimal> oppos(1, 0);
	Point<decimal> other(-1, -1);

	star_tracker::StarSet::set_fov(10);
	star_tracker::StarSet::set_pixel_resolution(4);
	decimal angle = star_tracker::StarSet::FindRealDistance(oppos, other);
	// Pixel Distance = 2.23606797749979.
	// Real Distance = 5.59016994374947.
	EXPECT_FLOAT_EQ(angle, 5.59016994374947);
}






//////////////////////////////////////////////////////////////////////////////
//																			//
//						------	FindElements	------						//
//																			//
//////////////////////////////////////////////////////////////////////////////

TEST (  )
{

}





//////////////////////////////////////////////////////////////////////////////
//																			//
//						------ 	Presets		------							//
//																			//
//////////////////////////////////////////////////////////////////////////////


TEST ( Set_FOV_SET_PIXEL_RESOLUTION, SUPPRESSION )
{
	star_tracker::StarSet::set_fov(10);
	star_tracker::StarSet::set_pixel_resolution(10);
}
