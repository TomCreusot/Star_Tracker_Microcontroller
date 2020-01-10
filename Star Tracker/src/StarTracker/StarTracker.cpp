/*
 *	File: 		StarTracker.cpp
 *	Author:		Tom Creusot
 *  Purpose:
 *				Uses the Pyramid method to calculate an angle which will be unique.
 *
 *	Reference:
 *
 * Ideal Calls: findAngles().
 *
 *
 * Header For: 	StarTracker.cpp, ImageProcessing.hpp.
 */

#include "StarTracker.h"

namespace st
{
	/**
	 * @brief 		Returns a vector of every angle from a combination of nodes.
	 *		  		This calls findAngle to find the angle and uses the first element in the array as the pilot.
	 * @param num	The number of elements in the array.
	 * @param set	The array of nodes, the first node must be the pilot star.
	 * @return 		All the angle combinations from the array.
	 */

	std::vector<float>& st::findAngles(const int num, Blob* set)
	{
		vector<float> angles;
		for (int ii = 1; ii < num; ii++)
			for (int jj = ii + 1; jj < num; jj++)
				for (int kk = jj + 1; kk < num; kk++)
					angles.push_back(findAngle(set[0], set[ii], set[jj], set[kk]));
		return angles;
	}





	/**
	 * @brief		Finds the angle between the 2 closest nodes to the pilot.
	 * @param pilot The brightest star.
	 * @param node1 A non pilot star.
	 * @param node2 An other non pilot star.
	 * @param node3 An other other non pilot star.
	 * @return 		The angle at the node farthest from the pilot node.
	 */

	float st::findAngle(const Blob& pilot, const Blob& node1, const Blob& node2, const Blob& node3)
	{
		//cosine rule: A = acos((b^2 + c^2 - a^2) / 2bc)
		//a is farthest node from pilot.
		float hyp = hypotf(node1.centroid.x - pilot.centroid.x, node1.centroid.y - pilot.centroid.y);
		float adj = hypotf(node2.centroid.x - pilot.centroid.x, node2.centroid.y - pilot.centroid.y);
		float opp = hypotf(node3.centroid.x - pilot.centroid.x, node3.centroid.y - pilot.centroid.y);

		//Assume node1 is a, node2 is b, node3 is c.
		float a = hypotf(node2.centroid.x - node3.centroid.x, node2.centroid.y - node3.centroid.y);
		float b = hypotf(node1.centroid.x - node2.centroid.x, node1.centroid.y - node2.centroid.y);
		float c = hypotf(node1.centroid.x - node3.centroid.x, node1.centroid.y - node3.centroid.y);

		//If our assumption is wrong, swap.
		if (adj > hyp && adj > opp)
		{
			float temp = a;
			a = b;
			b = temp;
		}
		else if (opp > hyp)
		{
			float temp = a;
			a = c;
			c = temp;
		}

		return acos((b*b + c * c - a * a) / 2 * b * c);
	}
}
