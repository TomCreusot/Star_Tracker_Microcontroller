/*
 *	File: 		StarTracker.cpp
 *	Author:		Tom Creusot
 *  Purpose:
 *			Uses the Pyramid method to calculate an angle which will be unique.
 *
 *	Reference:
 *
 * Ideal Calls: findAngles().
 *
 *
 * cpp For: 	StarTracker.hpp
 */

#include "StarTracker.hpp"

namespace st
{

/**
 * @brief 			Returns a vector of every angle from a combination
 											"numPilots" pilots and the children.
 *
 * @param num		The number of elements in the array,
 												MUST BE >= THAN "numPilots" + 3.
 * @param set		The array of nodes, the first node must be the pilot star.
 * @param numPilots	The number of pilots to check.
 * @return 			All the angle combinations from the array.
 */

std::list<decimal> pilotAngles ( int num, ip::Blob* set, int numPilots )
{
	std::list<decimal> angles;
	for ( int i = 0; i < numPilots; i++ )
	{
		std::list<decimal>* fAngles = findAngles(i, num, set);
		angles.insert(angles.end(), fAngles->begin(), fAngles->end());
	}
	return angles;
}


/**
 * @brief 		Returns a vector of every angle from a combination of nodes.
 				Requires the array to have the first element the pilot star.
 * @param startPos	Ignore everything before this point in the array.
 * @param num		The number of elements in the array.
 * @param set		The array of nodes, the first node must be the pilot star.
 * @return 			All the angle combinations from the array.
 *
 * DISPOSE OF THIS POINTER!!!
 */

std::list<decimal>* findAngles ( int startPos, int num, Blob* set )
{
	std::list<decimal>* angles = new std::list<decimal>();
	for (int ii = startPos; ii < num; ii++)
		for (int jj = ii + 1; jj < num; jj++)
			for (int kk = jj + 1; kk < num; kk++)
				angles -> push_back(findAngle(set[startPos].centroid,
						set[ii].centroid, set[jj].centroid, set[kk].centroid));
	return angles;
}





/**
 * @brief		Finds the angle between the 2 closest nodes to the pilot.
 *
 *	If the value is invalid, (multiple points are the same or impossible angle):
 *	1001 will be returned.
 *
 *
 * @param pilot The brightest star.
 * @param node1 A non pilot star.
 * @param node2 An other non pilot star.
 * @param node3 An other other non pilot star.
 * @return
 *		The angle at the node farthest from the pilot node or 1001 if invalid.
 */

decimal findAngle ( ip::Point<decimal>& pilot, ip::Point<decimal>& node1,
						ip::Point<decimal>& node2, ip::Point<decimal>& node3 )
{
	//cosine rule: A = acos((b^2 + c^2 - a^2) / 2bc)
	//a is farthest node from pilot.
	decimal hyp = node1.distance(pilot);
	decimal adj = node2.distance(pilot);
	decimal opp = node3.distance(pilot);

	// node1 is hyp
	decimal a = node2.distance(node3);
	// node2 is adj
	decimal b = node1.distance(node3);
	// node3 is opp
	decimal c = node1.distance(node2);

	//If our assumption is wrong, swap.
	if (adj > hyp && adj > opp)
	{ // if node2 is the futhest
		float temp = a;
		a = b;
		b = temp;
	}
	else if (opp > hyp)
	{ // if node3 is the futhest
		float temp = a;
		a = c;
		c = temp;
	}

	decimal angle = acos((b * b + c * c - a * a) / (2 * b * c));
	if ( node1.equal(node2) || node1.equal(node3) || node2.equal(node3)
												|| isnan(angle) || isinf(angle))
	{
		return 1001;
	}
	return angle;
}
}
