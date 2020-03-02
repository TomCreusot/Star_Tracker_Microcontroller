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
|		------	FindAnglesAllPilots	------			|
|													|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

TEST ( FindAnglesAllPilots, SingleAngle )
{
	ArrayList<Point<decimal>> point;
	ArrayList<AngleStat> angles;

	point.push_back(Point<decimal>(1, 2)); // pilot
	point.push_back(Point<decimal>(1, 3));
	point.push_back(Point<decimal>(2, 2));
	point.push_back(Point<decimal>(2, 3));

	findAnglesAllPilots(point, angles);

	EXPECT_EQ(angles.size(), 1);
	EXPECT_FLOAT_EQ(angles[0].angle, 1.5707963267949); //90 deg
	EXPECT_FLOAT_EQ(angles[0].pilot.x, 1);
	EXPECT_FLOAT_EQ(angles[0].pilot.y, 2);
}


TEST ( FindAnglesAllPilots, TwoPilots )
{
	ArrayList<Point<decimal>> point;
	ArrayList<AngleStat> angles;

	point.push_back(Point<decimal>(1, 2)); // pilot
	point.push_back(Point<decimal>(5, 5));
	point.push_back(Point<decimal>(2, 2));
	point.push_back(Point<decimal>(0, 0));
	point.push_back(Point<decimal>(1, 1));

	findAnglesAllPilots(point, angles);

	EXPECT_EQ(angles.size(), 5);
	EXPECT_FLOAT_EQ(angles[0].pilot.x, 1);
	EXPECT_FLOAT_EQ(angles[0].pilot.y, 2);
	EXPECT_FLOAT_EQ(angles[0].opposite.x, 5);
	EXPECT_FLOAT_EQ(angles[0].opposite.y, 5);

	// Next Pilot
	EXPECT_FLOAT_EQ(angles[4].pilot.x, 5);
	EXPECT_FLOAT_EQ(angles[4].pilot.y, 5);
	EXPECT_FLOAT_EQ(angles[4].opposite.x, 0);
	EXPECT_FLOAT_EQ(angles[4].opposite.y, 0);
}




/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|													|
|			------ DeriveFuthest ------				|
|													|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

TEST ( DeriveFuthest, AlreadySorted )
{
	Point<decimal> pilot;
	Point<decimal> s1(10, 10);
	Point<decimal> s2(9, 9);
	Point<decimal> s3(8, 8);

	deriveFuthest(&pilot, &s1, &s2, &s3);
	EXPECT_TRUE(s1.equal(10, 10));
	EXPECT_TRUE(s2.equal(9, 9));
	EXPECT_TRUE(s3.equal(8, 8));
}


TEST ( DeriveFuthest, SwappedParamOneAndTwo )
{
	Point<decimal> pilot;
	Point<decimal> s1(9, 10);
	Point<decimal> s2(10, 10);
	Point<decimal> s3(8, 8);


	deriveFuthest(&pilot, &s1, &s2, &s3);

	EXPECT_TRUE(s1.equal(10, 10));
	EXPECT_TRUE(s2.equal(9, 10));
	EXPECT_TRUE(s3.equal(8, 8));
}


TEST ( DeriveFuthest, SwappedParamOneAndThree )
{
	Point<decimal> pilot;
	Point<decimal> s1(9, 10);
	Point<decimal> s2(10, 10);
	Point<decimal> s3(800, 8);

	deriveFuthest(&pilot, &s1, &s2, &s3);
	EXPECT_TRUE(s1.equal(800, 8));
	EXPECT_TRUE(s2.equal(10, 10));
	EXPECT_TRUE(s3.equal(9, 10));
}





/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|													|
|		------ FindAngleSinglePilot	------			|
|													|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

/**
 * @brief To mock findAngle.
 * @return a.x * 1000 + b.x * 100 + c.x * 10 + d.x
 */

decimal func(	Point<decimal>& a, Point<decimal>& b,
				Point<decimal>& c, Point<decimal>& d	)
{
	return a.x * 1000 + b.x * 100 + c.x * 10 + d.x;
}


TEST ( FindAngleSinglePilot, OneCombo )
{
	Point<decimal> p;
	Point<decimal> s1(3);
	Point<decimal> s2(2);
	Point<decimal> s3(1);

	ArrayList<Point<decimal>> points;
	points.push_back( p );
	points.push_back( s1 );
	points.push_back( s2 );
	points.push_back( s3 );

	ArrayList<AngleStat> angles;

	findAnglesSinglePilot(0, 4, points, func, angles);

	EXPECT_FLOAT_EQ(angles[0].angle, 321);
}



TEST ( FindAngleSinglePilot, MultiCombo )
{
	Point<decimal> p;
	Point<decimal> s1(5);
	Point<decimal> s2(4);
	Point<decimal> s3(3);
	Point<decimal> s4(2);
	Point<decimal> s5(1);

	ArrayList<Point<decimal>> points;
	points.push_back( p );
	points.push_back( s1 );
	points.push_back( s2 );
	points.push_back( s3 );
	points.push_back( s4 );
	points.push_back( s5 );

	ArrayList<AngleStat> angles;

	findAnglesSinglePilot(0, 6, points, func, angles);

	EXPECT_FLOAT_EQ(angles[0].angle, 543);
	EXPECT_FLOAT_EQ(angles[1].angle, 542);
	EXPECT_FLOAT_EQ(angles[2].angle, 541);
	EXPECT_FLOAT_EQ(angles[3].angle, 532);
	EXPECT_FLOAT_EQ(angles[4].angle, 531);
	EXPECT_FLOAT_EQ(angles[5].angle, 521);
	EXPECT_FLOAT_EQ(angles[6].angle, 432);
	EXPECT_FLOAT_EQ(angles[7].angle, 431);
	EXPECT_FLOAT_EQ(angles[8].angle, 421);
	EXPECT_FLOAT_EQ(angles[9].angle, 321);
}





TEST ( FindAngleSinglePilot, MultiComboUnsorted )
{

	Point<decimal> p;
	Point<decimal> s1(1);
	Point<decimal> s2(2);
	Point<decimal> s3(3);
	Point<decimal> s4(4);
	Point<decimal> s5(5);


	ArrayList<Point<decimal>> points;
	points.push_back( p );
	points.push_back( s1 );
	points.push_back( s2 );
	points.push_back( s3 );
	points.push_back( s4 );
	points.push_back( s5 );

	ArrayList<AngleStat> angles;

	findAnglesSinglePilot(0, 6, points, func, angles);

	EXPECT_FLOAT_EQ(angles[0].angle, 321);	// 1 2 3
	EXPECT_FLOAT_EQ(angles[1].angle, 421);	// 1 2 4
	EXPECT_FLOAT_EQ(angles[2].angle, 521);	// 1 2 5
	EXPECT_FLOAT_EQ(angles[3].angle, 431);	// 1 3 4
	EXPECT_FLOAT_EQ(angles[4].angle, 531);	// 1 3 5
	EXPECT_FLOAT_EQ(angles[5].angle, 541);	// 1 4 5
	EXPECT_FLOAT_EQ(angles[6].angle, 432);	// 2 3 4
	EXPECT_FLOAT_EQ(angles[7].angle, 532);	// 2 3 5
	EXPECT_FLOAT_EQ(angles[8].angle, 542);	// 2 4 5
	EXPECT_FLOAT_EQ(angles[9].angle, 543);	// 3 4 5
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

	decimal angle = star_tracker::findAngle(pilot, hypot, adjac, oppos);
	EXPECT_FLOAT_EQ(angle, 1.8545904360032242d);
}


TEST ( FindAngle, EqualSideLengths )
{
	Point<decimal> pilot(10, -5);
	Point<decimal> hypot(10, -10);
	Point<decimal> adjac(9, -5);
	Point<decimal> oppos(11, -5);

	decimal angle = star_tracker::findAngle(pilot, hypot, adjac, oppos);
	EXPECT_FLOAT_EQ(angle, 0.3947911197);
}


TEST ( FindAngle, PointsAwayFromPilot )
{
	Point<decimal> pilot(0, 0);
	Point<decimal> hypot(5, 5);
	Point<decimal> adjac(0, 5);
	Point<decimal> oppos(1, 6);

	decimal angle = star_tracker::findAngle(pilot, hypot, adjac, oppos);
	EXPECT_FLOAT_EQ(angle, 0.2449786631);
}



TEST ( FindAngle, RealExample )
{
	// Orions belt from hyg database
	Point<decimal> pilot(5.603559,	-1.20192);	// Alnilam, mag: 1.69
	Point<decimal> hypot(5.645769,	-2.600069);	// 48 ori, mag: 3.77
	Point<decimal> adjac(5.679313, -1.942572);	// Alnitak, mag: 1.74
	Point<decimal> oppos(5.533445,	-0.299092);	// Mintaka, mag: 2.25


	decimal angle = star_tracker::findAngle(pilot, hypot, adjac, oppos);
	EXPECT_FLOAT_EQ(angle, 0.099750593); //, 0.099753); (Close enough?)
}






// When things break.
TEST ( FindAngle, NanAllEqual )
{
	Point<decimal> pilot(0, 0);
	Point<decimal> oppos(0, 0);
	Point<decimal> adjac(0, 0);
	Point<decimal> hypot(0, 0);

	decimal angle = star_tracker::findAngle(pilot, oppos, adjac, hypot);
	EXPECT_FLOAT_EQ(angle, 1000);
}



// Tests that it can never output nan.
TEST ( FindAngle, RandomTestNAN )
{
	srand(time(NULL));

	for ( unsigned long i = 1; i < 1000000;  i+= 1000 )
	{
		Point<decimal> pilot(rand() / i / 100000, rand() / i / 100000 - 100);
		Point<decimal> oppos(rand() / i / 100000, rand() / i / 100000 - 100);
		Point<decimal> adjac(rand() / i / 100000 - 100, rand() / i / 100000);
		Point<decimal> hypot(rand() / i / 100000 - 100, rand() / i / 100000);

		deriveFuthest(&pilot, &oppos, &adjac, &hypot);

		decimal angle = star_tracker::findAngle(pilot, oppos, adjac, hypot);
		bool valid = !isnan(angle) && !isinf(angle);

		EXPECT_TRUE(valid);
	}

}
