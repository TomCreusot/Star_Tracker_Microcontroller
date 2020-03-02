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





void findAnglesAllPilots ( ArrayList<Point<decimal>>& points,
							ArrayList<AngleStat>& angles )
{
	FIND_ANGLE_PTR ptr = *findAngle;
	for ( uint i = 0; i < points.size() - 3; i++ )
	{
		findAnglesSinglePilot(i, points.size(), points, ptr, angles);
	}
}




void findAnglesSinglePilot ( uint start, uint end,
					ArrayList<Point<decimal>> points,
					FIND_ANGLE_PTR func_ptr,
					ArrayList<AngleStat>& angles )
{
	for ( uint ii = start + 1; ii < end; ii++ )
		for ( uint jj = ii + 1; jj < end; jj++ )
			for ( uint kk = jj + 1; kk < end; kk++ )
			{
				Point<decimal> p	= points[start];
				Point<decimal> s1	= points[ii];
				Point<decimal> s2	= points[jj];
				Point<decimal> s3	= points[kk];

				deriveFuthest(&p, &s1, &s2, &s3);


				const decimal angle = func_ptr(p, s1, s2, s3);
				AngleStat stat(angle, p, s1);

				angles.push_back(stat);
			}
}




void deriveFuthest ( Point<decimal>* pilot,
					Point<decimal>* s1, Point<decimal>* s2, Point<decimal>* s3 )
{
	// Finds the futhest star.
	const decimal a = s1->distance(*pilot);
	const decimal b = s2->distance(*pilot);
	const decimal c = s3->distance(*pilot);

	if ( b > a && b > c )
	{
		Point<decimal> temp = *s1;
		*s1 = *s2;
		*s2 = temp;
	}
	else if ( c > a )
	{
		Point<decimal> temp = *s1;
		*s1 = *s3;
		*s3 = temp;
	}
}




decimal findAngle ( Point<decimal>& pilot,
					Point<decimal>& s1, Point<decimal>& s2, Point<decimal>& s3 )
{
	if ( !(s1.equal(s2) || s2.equal(s3) || s3.equal(s1)) )
	{
		decimal a = s2.distance(s3);
		decimal b = s1.distance(s2);
		decimal c = s1.distance(s3);

		decimal angle = acos((b * b + c * c - a * a) / (2 * b * c));
		return angle;
	}
	else
	{
		return 1000;
	}

}








}
