#include "gtest/gtest.h"

#include "star_tracker.h"
#include "libs/image_processing/blob_mock.h"

//Random for find angle.
#include <cstdlib>
#include <time.h>

using namespace std;
using namespace star_tracker;
using namespace image_processing;
using namespace util;


/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|													|
|		------  DeriveBrightest	------				|
|													|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

TEST	( DeriveBrightest, Valid )
{
	util::ArrayList<Blob> l1;
	util::ArrayList<Point<decimal>> l2;
	l1.push_back(BlobMock(0, 0, 1));
	l1.push_back(BlobMock(5, 1, 2));
	l1.push_back(BlobMock(1, 2, 3));
	l1.push_back(BlobMock(2, 3, 4));

	star_tracker::deriveBrightest(l1, l2, 3);
	EXPECT_EQ(l2[0].x, 1);
	EXPECT_EQ(l2[0].y, 2);

	EXPECT_EQ(l2[1].x, 3);
	EXPECT_EQ(l2[1].y, 4);

	EXPECT_EQ(l2[2].x, 2);
	EXPECT_EQ(l2[2].y, 3);

	EXPECT_EQ(l2.size(), 3);
}


TEST	( DeriveBrightest, SameSize )
{
	util::ArrayList<Blob> l1;
	util::ArrayList<Point<decimal>> l2;
	l1.push_back(BlobMock(0, 1, 1));
	l1.push_back(BlobMock(4, 5, 5));
	l1.push_back(BlobMock(1, 2, 2));
	l1.push_back(BlobMock(2, 3, 3));
	l1.push_back(BlobMock(3, 4, 4));

	star_tracker::deriveBrightest(l1, l2, 5);
	EXPECT_EQ(l2[0].x, 5);
	EXPECT_EQ(l2[0].y, 5);

	EXPECT_EQ(l2[1].x, 4);
	EXPECT_EQ(l2[1].y, 4);

	EXPECT_EQ(l2[2].x, 3);
	EXPECT_EQ(l2[2].y, 3);

	EXPECT_EQ(l2[3].x, 2);
	EXPECT_EQ(l2[3].y, 2);

	EXPECT_EQ(l2[4].x, 1);
	EXPECT_EQ(l2[4].y, 1);

	EXPECT_EQ(l2.size(), 5);
}


TEST	( DeriveBrightest, RequiredLarger )
{
	util::ArrayList<Blob> l1;
	util::ArrayList<Point<decimal>> l2;
	l1.push_back(BlobMock(0, 0, 1));
	l1.push_back(BlobMock(5, 1, 2));
	l1.push_back(BlobMock(1, 2, 3));
	l1.push_back(BlobMock(2, 3, 4));
	l1.push_back(BlobMock(3, 4, 5));

	star_tracker::deriveBrightest(l1, l2, 10);

	EXPECT_EQ(l2.size(), 5);
}



/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|													|
|			------	FindAngles	------				|
|													|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

TEST ( FindAngles, SingleAngle )
{
	ArrayList<Point<decimal>> point;
	ArrayList<Combo> combos;
	ArrayList<AngleStat> angles;

	combos.push_back(Combo(0, 1, 2, 3));

	point.push_back(Point<decimal>(1, 2)); // pilot
	point.push_back(Point<decimal>(1, 3));
	point.push_back(Point<decimal>(2, 2));
	point.push_back(Point<decimal>(2, 3));

	findAngles(point, combos, angles);

	EXPECT_EQ(angles.size(), 1);
	EXPECT_FLOAT_EQ(angles[0].angle, 1.5707963267949); //90 deg
	EXPECT_FLOAT_EQ(angles[0].pilot.x, 1);
	EXPECT_FLOAT_EQ(angles[0].pilot.y, 2);
}





/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|													|
|			------ Combinations	------				|
|													|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/


TEST ( Combinations, OnePilot )
{
	ArrayList<star_tracker::Combo> combos;
	star_tracker::combinationsPilots(1, 6, combos);

	EXPECT_TRUE(combos.pop_back().equal(0, 3, 4, 5));
	EXPECT_TRUE(combos.pop_back().equal(0, 2, 4, 5));
	EXPECT_TRUE(combos.pop_back().equal(0, 2, 3, 5));
	EXPECT_TRUE(combos.pop_back().equal(0, 2, 3, 4));
	EXPECT_TRUE(combos.pop_back().equal(0, 1, 4, 5));
	EXPECT_TRUE(combos.pop_back().equal(0, 1, 3, 5));
	EXPECT_TRUE(combos.pop_back().equal(0, 1, 3, 4));
	EXPECT_TRUE(combos.pop_back().equal(0, 1, 2, 5));
	EXPECT_TRUE(combos.pop_back().equal(0, 1, 2, 4));
	EXPECT_TRUE(combos.pop_back().equal(0, 1, 2, 3));
}


TEST ( Combinations, MultiplePilots )
{
	ArrayList<star_tracker::Combo> combos;
	star_tracker::combinationsPilots(2, 6, combos);

	EXPECT_TRUE(combos.pop_back().equal(1, 3, 4, 5));
	EXPECT_TRUE(combos.pop_back().equal(1, 2, 4, 5));
	EXPECT_TRUE(combos.pop_back().equal(1, 2, 3, 5));
	EXPECT_TRUE(combos.pop_back().equal(1, 2, 3, 4));
	EXPECT_TRUE(combos.pop_back().equal(0, 3, 4, 5));
	EXPECT_TRUE(combos.pop_back().equal(0, 2, 4, 5));
	EXPECT_TRUE(combos.pop_back().equal(0, 2, 3, 5));
	EXPECT_TRUE(combos.pop_back().equal(0, 2, 3, 4));
	EXPECT_TRUE(combos.pop_back().equal(0, 1, 4, 5));
	EXPECT_TRUE(combos.pop_back().equal(0, 1, 3, 5));
	EXPECT_TRUE(combos.pop_back().equal(0, 1, 3, 4));
	EXPECT_TRUE(combos.pop_back().equal(0, 1, 2, 5));
	EXPECT_TRUE(combos.pop_back().equal(0, 1, 2, 4));
	EXPECT_TRUE(combos.pop_back().equal(0, 1, 2, 3));
}



TEST ( Combinations, MorePilotsElements )
{
	ArrayList<star_tracker::Combo> combos;
	star_tracker::combinationsPilots(10, 6, combos);
	EXPECT_TRUE(combos.pop_back().equal(2, 3, 4, 5));
	EXPECT_TRUE(combos.pop_back().equal(1, 3, 4, 5));
	EXPECT_TRUE(combos.pop_back().equal(1, 2, 4, 5));
	EXPECT_TRUE(combos.pop_back().equal(1, 2, 3, 5));
	EXPECT_TRUE(combos.pop_back().equal(1, 2, 3, 4));
}




/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|													|
|			------ Combinations	------				|
|													|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

TEST ( Combinations, Valid )
{
	ArrayList<star_tracker::Combo> combos;
	star_tracker::combinations(1, 7, combos);

	EXPECT_TRUE(combos.pop_back().equal(1, 4, 5, 6));

	EXPECT_TRUE(combos.pop_back().equal(1, 3, 5, 6));
	EXPECT_TRUE(combos.pop_back().equal(1, 3, 4, 6));
	EXPECT_TRUE(combos.pop_back().equal(1, 3, 4, 5));

	EXPECT_TRUE(combos.pop_back().equal(1, 2, 5, 6));
	EXPECT_TRUE(combos.pop_back().equal(1, 2, 4, 6));
	EXPECT_TRUE(combos.pop_back().equal(1, 2, 4, 5));
	EXPECT_TRUE(combos.pop_back().equal(1, 2, 3, 6));
	EXPECT_TRUE(combos.pop_back().equal(1, 2, 3, 5));
	EXPECT_TRUE(combos.pop_back().equal(1, 2, 3, 4));


}



TEST ( Combinations, InValid )
{
	ArrayList<star_tracker::Combo> combos;
	star_tracker::combinations(1, 3, combos);

	EXPECT_EQ(combos.size(), 0);

	star_tracker::combinations(3, 0, combos);

	EXPECT_EQ(combos.size(), 0);
}




/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|													|
|			------	FindAngle	------				|
|													|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

TEST ( FindAngle, Valid )
{
	Point<decimal> pilot(0, 0);
	Point<decimal> hypot(0, 10);
	Point<decimal> adjac(-4, 7);
	Point<decimal> oppos(8, 4);
	ArrayList<Point<decimal>> a;
	a.push_back(pilot);
	a.push_back(hypot);
	a.push_back(adjac);
	a.push_back(oppos);


	Combo combo; // suppress lcov for Combo
	combo.pilot = 0; combo.s1 = 1; combo.s2 = 2; combo.s3 = 3;

	decimal angle = star_tracker::findAngle(a, combo);
	EXPECT_DOUBLE_EQ(angle, 1.8545904360032242d);
}



TEST ( FindAngle, HypotIsAdjac )
{
	Point<decimal> pilot(0, 0);
	Point<decimal> adjac(0, 10);
	Point<decimal> hypot(-4, 7);
	Point<decimal> oppos(8, 4);
	ArrayList<Point<decimal>> a;
	a.push_back(pilot);
	a.push_back(hypot);
	a.push_back(adjac);
	a.push_back(oppos);

	Combo combo(0, 1, 2, 3);

	decimal angle = star_tracker::findAngle(a, combo);
	EXPECT_DOUBLE_EQ(angle, 1.8545904360032242d);
}



TEST ( FindAngle, HypotIsOppos )
{
	Point<decimal> pilot(0, 0);
	Point<decimal> oppos(0, 10);
	Point<decimal> adjac(-4, 7);
	Point<decimal> hypot(8, 4);
	ArrayList<Point<decimal>> a;
	a.push_back(pilot);
	a.push_back(hypot);
	a.push_back(adjac);
	a.push_back(oppos);

	Combo combo(0, 1, 2, 3);

	decimal angle = star_tracker::findAngle(a, combo);
	EXPECT_DOUBLE_EQ(angle, 1.8545904360032242d);
}



TEST ( FindAngle, EqualSideLengths )
{
	Point<decimal> pilot(10, -5);
	Point<decimal> hypot(10, -10);
	Point<decimal> adjac(9, -5);
	Point<decimal> oppos(11, -5);
	ArrayList<Point<decimal>> a;
	a.push_back(pilot);
	a.push_back(hypot);
	a.push_back(adjac);
	a.push_back(oppos);

	Combo combo(0, 1, 2, 3);

	decimal angle = (decimal) star_tracker::findAngle(a, combo);
	EXPECT_FLOAT_EQ(angle, 0.3947911197);
}


TEST ( FindAngle, PointsAwayFromPilot )
{
	Point<decimal> pilot(0, 0);
	Point<decimal> hypot(5, 5);
	Point<decimal> adjac(0, 5);
	Point<decimal> oppos(1, 6);
	ArrayList<Point<decimal>> a;
	a.push_back(pilot);
	a.push_back(hypot);
	a.push_back(adjac);
	a.push_back(oppos);

	Combo combo(0, 1, 2, 3);

	decimal angle = (decimal) star_tracker::findAngle(a, combo);
	EXPECT_FLOAT_EQ(angle, 0.2449786631);
}


TEST ( FindAngle, RealExample )
{
	// Orions belt from hyg database
	Point<decimal> pilot(5.603559,	-1.20192);	// Alnilam, mag: 1.69
	Point<decimal> hypot(5.533445,	-0.299092);	// Mintaka, mag: 2.25
	Point<decimal> adjac(5.679313, -1.942572);	// Alnitak, mag: 1.74
	Point<decimal> oppos(5.645769,	-2.600069);	// 48 ori, mag: 3.77
	ArrayList<Point<decimal>> a;
	a.push_back(pilot);
	a.push_back(hypot);
	a.push_back(adjac);
	a.push_back(oppos);

	Combo combo(0, 1, 2, 3);



	decimal angle = (decimal) star_tracker::findAngle(a, combo);
	EXPECT_FLOAT_EQ(angle, 0.099750593); //, 0.099753); (Close enough?)
}






// When things break.
TEST ( FindAngle, NanAllEqual )
{
	Point<decimal> pilot(0, 0);
	Point<decimal> oppos(0, 0);
	Point<decimal> adjac(0, 0);
	Point<decimal> hypot(0, 0);
	ArrayList<Point<decimal>> a;
	a.push_back(pilot);
	a.push_back(hypot);
	a.push_back(adjac);
	a.push_back(oppos);

	Combo combo(0, 1, 2, 3);

	decimal angle = (decimal) star_tracker::findAngle(a, combo);
	EXPECT_FLOAT_EQ(angle, 1000);
}




TEST ( FindAngle, RandomTestNAN )
{
	srand(time(NULL));

	for ( unsigned long i = 1; i < 1000000;  i+= 1000 )
	{
		Point<decimal> pilot(rand() / i / 100000, rand() / i / 100000 - 100);
		Point<decimal> oppos(rand() / i / 100000, rand() / i / 100000 - 100);
		Point<decimal> adjac(rand() / i / 100000 - 100, rand() / i / 100000);
		Point<decimal> hypot(rand() / i / 100000 - 100, rand() / i / 100000);
		ArrayList<Point<decimal>> a;
		a.push_back(pilot);
		a.push_back(hypot);
		a.push_back(adjac);
		a.push_back(oppos);

		Combo combo(0, 1, 2, 3);

		decimal angle = (decimal) star_tracker::findAngle(a, combo);

		bool valid = !isnan(angle) && !isinf(angle);

		EXPECT_TRUE(valid);
	}

}
