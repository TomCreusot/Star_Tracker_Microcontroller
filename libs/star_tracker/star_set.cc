#include "star_set.h"
namespace star_tracker
{
util::decimal StarSet::fov = 10;
util::decimal StarSet::pixel_resolution = 800; // 480x640

StarSet::StarSet ( )
{
	angle	= 0;
	opposite	= util::Point<decimal>(0, 0);
	pixel	= NULL;
	odds	= 1;
	distance = 0;
}

StarSet::StarSet ( StarSet& px )
{
	angle 		= px.angle;
	opposite	= px.opposite;
	odds 		= px.odds;
	distance 	= px.distance;
	orientation = px.orientation;
}



StarSet::StarSet (	Point<decimal>& p,
					Point<decimal>& s1, Point<decimal>& s2, Point<decimal>& s3 )
{
	//Point<decimal> pilot = p;
	Point<decimal> o1 = s1;
	Point<decimal> other = s2;
	Point<decimal> opposite = s3;

	// Makes opposite the opposite.
	SortDistance(p, &o1, &opposite);
	SortDistance(p, &other, &opposite);

	// Makes other the farthest from opposite.
	SortDistance(opposite, &o1, &other);

	angle = FindAngle(o1, other, opposite);
	distance = opposite.Distance(other);

	this->opposite = opposite;
	pixel	= NULL;
	odds	= 1;
}





void StarSet::FindCenter	 (	decimal fovPP,
								Point<int>& size,
								Point<decimal>* center )
{
	//decimal angle = orientation - pixel->orientation;
}





void StarSet::ToArrayString	( string* str )
{
	for ( uint i = 0; i < kDatabaseNumElements; i++ )
	{
		switch ( i )
		{
			case kDatabaseIndexAngle:
				*str += to_string(angle);
				break;
			case kDatabaseIndexDist:
				*str += to_string(distance);
				break;
			case kDatabaseIndexOrientation:
				*str += to_string(orientation);
				break;
			case kDatabaseIndexOppositeX:
				*str += to_string(opposite.x);
				break;
			case kDatabaseIndexOppositeY:
				*str += to_string(opposite.y);
				break;
		}
		if ( i != kDatabaseNumElements - 1 ) *str += ", ";
	}
}



void StarSet::SortDistance	( 		Point<decimal>& pilot,
									Point<decimal>* s1, Point<decimal>* s2	)
{
	if ( s1->Distance(pilot) > s2->Distance(pilot) )
	{
		Point<decimal> temp;
		temp = *s1;
		*s1 = *s2;
		*s2 = temp;
	}
}



decimal StarSet::FindAngle ( 	Point<decimal>& center,
								Point<decimal>& left, Point<decimal>& right )
{
	if ( !(center.Equal(left)| center.Equal(right) || left.Equal(right)) )
	{
		decimal a = left.Distance(right);
		decimal b = left.Distance(center);
		decimal c = right.Distance(center);

		decimal angle = acos((b * b + c * c - a * a) / (2 * b * c));
		return angle;
	}
	else
	{
		return StarSet::kInvalidAngle;
	}
}

bool StarSet::SortByOdds ( StarSet& larger, StarSet& smaller )
{
	return larger.odds >= smaller.odds;
}



}
