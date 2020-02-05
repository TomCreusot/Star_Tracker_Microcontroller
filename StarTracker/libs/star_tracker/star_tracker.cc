#include "star_tracker.h"

namespace st
{
void deriveBrightest ( ArrayList<Blob>& list,
								ArrayList<Point<decimal>>& reduced, uint num )
{
	uint highest = -1;
	uint currentI = 0;
	Blob current = list[0];

	for ( uint ii = 0; ii < num && ii < list.size(); ii++ )
	{
		currentI = 0;
		for ( uint jj = 0; jj < list.size(); jj++ )
		{
			uint intensity = list[jj].getIntensity();
			if( intensity >= currentI && intensity < highest )
			{
				current = list[jj];
				currentI = current.getIntensity();
			}
		}
		reduced.push_back(
				Point<decimal>(current.getCentroidX(), current.getCentroidY()));
		highest = currentI;
	}
}



void clusterProbability ( ArrayList<AngleStat>& database,
											decimal w_separation, decimal fov )
{
	for ( uint ii = 0; ii < database.size(); ii++ )
		for ( uint jj = 0; jj < database.size(); jj++ )
		{
			const decimal dist = dAngles[ii].pilot.distance(database[jj]);
			if ( dist > fov )
			{
				database[ii].odds += w_separation / dist;
			}
		}
}




void findAngles ( ArrayList<Point<decimal>>& points, ArrayList<Combo>& combos,
												ArrayList<AngleStat>& angles )
{
	for ( uint i = 0; i < combos.size(); i++ )
	{
		angles.push_back(AngleStats(findAngle(points, combos[i]), points[combos[i].pilot]));
	}
}



void combinationsPilots ( uint numPilots, uint end, ArrayList<Combo>& combos )
{
	for ( uint i = 0; i < numPilots; i++ )
	{
		combinations(i, end, combos);
	}
}





void combinations ( uint start, uint end, ArrayList<Combo>& combos )
{
	for ( uint ii = start + 1; ii < end; ii++ )
		for ( uint jj = ii + 1; jj < end; jj++ )
			for ( uint kk = jj + 1; kk < end; kk++ )
			{
				Combo combo(start, ii, jj, kk);
				combos.push_back(Combo(start, ii, jj, kk));

			}
}



decimal findAngle ( ArrayList<Point<decimal>>& points, Combo& combo )
{
	Point<decimal> pilot = points[combo.pilot];
	Point<decimal> node1 = points[combo.s1];
	Point<decimal> node2 = points[combo.s2];
	Point<decimal> node3 = points[combo.s3];

	if ( !(node1.equal(node2) || node2.equal(node3) || node3.equal(node1)) )
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
			decimal temp = a;
			a = b;
			b = temp;
		}
		else if (opp > hyp)
		{ // if node3 is the futhest
			decimal temp = a;
			a = c;
			c = temp;
		}

		decimal angle = acos((b * b + c * c - a * a) / (2 * b * c));
		return angle;
	}
	else
	{
		return 1000;
	}

}








}
