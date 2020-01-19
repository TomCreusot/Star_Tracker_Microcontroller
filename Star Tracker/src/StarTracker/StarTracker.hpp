/*
 *	File: 		StarTracker.hpp
 *	Author:		Tom Creusot
 *  Purpose:
 *				Uses the Pyramid method to calculate an angle which will be unique.
 *
 *	Reference:
 *
 * Header For: 	StarTracker.cpp.
 */


#ifndef STAR_TRACKER_H
#define STAR_TRACKER_H

#include "../ImageProcessing/ImageProcessing.hpp" // decimal, blob
#include "../ImageProcessing/Point.cpp"
#include "../Database/Database.hpp" // To retreive the position from angle
#include <math.h>

using namespace ip;
using namespace db;
using namespace std;


namespace st
{
	// Refer to StarTracker.cpp
	std::list<decimal> pilotAngles ( int num, ip::Blob* set, int numPilots );
	std::list<decimal>* findAngles ( int startPos, int num, ip::Blob* set );
	decimal findAngle ( ip::Point<decimal>& pilot, ip::Point<decimal>& node1,
						ip::Point<decimal>& node2, ip::Point<decimal>& node3 );
}

#endif
