/*
 *	File: 		StarTracker.hpp
 *	Author:		Tom Creusot
 *  Purpose:
 *				Uses the Pyramid method to calculate an angle which will be unique.
 *
 *	Reference:
 *
 * Header For: 	StarTracker.cpp, ImageProcessing.hpp.
 */


#ifndef STAR_TRACKER_H
#define STAR_TRACKER_H

#include <../ImageProcessing/ImageProcessing.hpp>
#include <math.h>

using namespace ip;
using namespace std;

namespace st
{
	// Refer to StarTracker.cpp
	std::vector<float>& findAngles(const int num, const Blobs* set);
	float findAngle(const KeyPoint& pilot, const KeyPoint& node1, const KeyPoint& node2, const KeyPoint& node3);
}

#endif
