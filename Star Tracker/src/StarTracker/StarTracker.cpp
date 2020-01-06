#include "StarTracker.h"

namespace st
{
	/**
	 * @brief Returns a vector of every angle from a combination of nodes.
	 *		  This calls findAngle to find the angle and uses the first element in the array as the pilot.
	 * @param num The number of elements in the array.
	 * @param set The array of nodes, the first node must be the pilot star.
	 * @return All the angle combinations from the array.
	 */

	vector<float>& st::findAngles(const int num, const KeyPoint* set)
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

	float st::findAngle(const KeyPoint& pilot, const KeyPoint& node1, const KeyPoint& node2, const KeyPoint& node3)
	{
		//cosine rule: A = acos((b^2 + c^2 - a^2) / 2bc)
		//a is farthest node from pilot.
		float hyp = hypotf(node1.pt.x - pilot.pt.x, node1.pt.y - pilot.pt.y);
		float adj = hypotf(node2.pt.x - pilot.pt.x, node2.pt.y - pilot.pt.y);
		float opp = hypotf(node3.pt.x - pilot.pt.x, node3.pt.y - pilot.pt.y);

		//Assume node1 is a, node2 is b, node3 is c.
		float a = hypotf(node2.pt.x - node3.pt.x, node2.pt.y - node3.pt.y);
		float b = hypotf(node1.pt.x - node2.pt.x, node1.pt.y - node2.pt.y);
		float c = hypotf(node1.pt.x - node3.pt.x, node1.pt.y - node3.pt.y);

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
