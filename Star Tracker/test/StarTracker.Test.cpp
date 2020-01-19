#include "UnitTest.hpp"
using namespace st;
using namespace ip;
using namespace db;


namespace stT
{


bool run ( )
{
	printHeader("StarTracker.cpp", "StarTracker.Test.cpp");
	bool valid = true;

	valid &= printPass("findAngle: Valid", testFindAngleValid());
	valid &= printPass("findAngle: All the same value", testFindAngleAllSame());
	valid &= printPass("findAngle: All points are in a line", testFindAngleAllInLine());
	return valid;
}


/**
 * @brief Tests findAngle when a value is supposed to be returned(runs 3 tests).
 * @return True if all tests passed.
 */

bool testFindAngleValid ( )
{
	bool valid = true;
	// Side lengths are rational.
	ip::Point<decimal> pilot(0, 0);
	ip::Point<decimal> hypot(0, 10);
	ip::Point<decimal> adjac(-4, 7);
	ip::Point<decimal> oppos(8, 4);

	decimal angle = st::findAngle(pilot, hypot, adjac, oppos);
	valid &= equal(angle, 1.8545904360032242d);


	// 2 side angles are equal.
	pilot.set(10, -5);
	hypot.set(10, -10);
	adjac.set(9, -5);
	oppos.set(11, -5);

	angle = st::findAngle(pilot, hypot, adjac, oppos);
	valid &= equal(angle, 0.3947911197d);


	// Angle points away from pilot.
	pilot.set(0, 0);
	hypot.set(5, 5);
	adjac.set(0, 5);
	oppos.set(1, 6);

	angle = st::findAngle(pilot, hypot, adjac, oppos);
	valid &= equal(angle, 0.2449786631d);

	// Orions belt from hyg database
	pilot.set(5.603559,	-1.20192);	// Alnilam, mag: 1.69
	hypot.set(5.533445,	-0.299092);	// Mintaka, mag: 2.25
	adjac.set(5.679313, -1.942572);	// Alnitak, mag: 1.74
	oppos.set(5.645769,	-2.600069);	// 48 ori, mag: 3.77
	angle = st::findAngle(pilot, hypot, adjac, oppos);
	valid &= equal(angle, 0.099753);
	
	return valid;
}




/**
* @brief Tests findAngle when all the points are the same.
* @return True if all tests passed.
*/

bool testFindAngleAllSame ( )
{
	bool valid = true;

	ip::Point<decimal> pilot(0, 0);
	ip::Point<decimal> hypot(0, 0);
	ip::Point<decimal> adjac(0, 0);
	ip::Point<decimal> oppos(0, 0);

	decimal angle = st::findAngle(pilot, hypot, adjac, oppos);
	valid &= 1000 < angle; // error value is 1001
	return valid;
}



/**
* @brief Tests findAngle when all the points inline with each other.
* @return True if all tests passed.
*/

bool testFindAngleAllInLine ( )
{
	bool valid = true;

	ip::Point<decimal> pilot(0, -10);
	ip::Point<decimal> hypot(0, 0);
	ip::Point<decimal> adjac(0, 1);
	ip::Point<decimal> oppos(0, 2);

	decimal angle = st::findAngle(pilot, hypot, adjac, oppos);
	valid &= equal(0, angle);
	return valid;
}







bool testFindAngles ( )
{
	return true;
}

}
