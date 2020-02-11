#include "star_tracker.h"


namespace star_tracker
{
void deriveBrightest ( ArrayList<Blob>& list,
								ArrayList<Point<decimal>>& reduced, uint num )
{
	for ( uint ii = 0; ii < list.size(); ii++ )
	{
		uint jj = ii;
		Blob temp = list[jj];
		while(jj > 0 && list[jj-1].getIntensity() < temp.getIntensity())
		{
			list[jj] = list[jj - 1];
			jj--;
		}
		list[jj] = temp;
	}


	for ( uint i = 0; i < list.size() && i < num; i++ )
	{
		Point<decimal> point(list[i].getCentroidX(), list[i].getCentroidY());
		reduced.push_back(point);
	}
}



void findAngles ( ArrayList<Point<decimal>>& points, ArrayList<Combo>& combos,
												ArrayList<AngleStat>& angles )
{
	for ( uint i = 0; i < combos.size(); i++ )
	{
		AngleStat angle(findAngle(points, combos[i]), points[combos[i].pilot]);
		angles.push_back(angle);
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
