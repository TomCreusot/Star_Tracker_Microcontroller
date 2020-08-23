#include "star_set.h"
namespace star_tracker
{

//////////////////////////////////////////////////////////////////////////////
//																			//
//						------	Constructors	------						//
//																			//
//////////////////////////////////////////////////////////////////////////////

StarSet::StarSet ( )
{
	area		= 0;
	moment 		= 0;
	position	= util::Point<decimal>(0, 0);
	pixel		= NULL;
	vote		= 1;
}


// Distance only
StarSet::StarSet ( Point<decimal> pos, decimal area, decimal moment )
{
	this->area		= area;
	this->moment	= moment;
	this->position	= pos;
	this->pixel 	= NULL;
	this->vote 		= 1;
}



StarSet::StarSet ( const StarSet& px )
{
	area		= px.area;
	moment		= px.moment;
	position	= px.position;
	pixel		= NULL;
	vote		= px.vote;
}






//////////////////////////////////////////////////////////////////////////////
//																			//
//						------	Core Functions	------						//
//																			//
//////////////////////////////////////////////////////////////////////////////
/*
Point<decimal> StarSet::FindCenter	 ( Point<int>& size )
{
	return Point<decimal>(0);
	//decimal angle = orientation - pixel->orientation;
}*/








//////////////////////////////////////////////////////////////////////////////
//																			//
//					------	Function Pointers	------						//
//																			//
//////////////////////////////////////////////////////////////////////////////



decimal StarSet::CalcArea	( decimal a, decimal b, decimal c )
{
	decimal s = (a + b + c) / 2;
	return std::sqrt(s * (s - a) * (s - b) * (s - c));
}

decimal StarSet::CalcMoment	( decimal area, decimal a, decimal b, decimal c )
{
	return area * ( a * a + b * b + c * c ) / 36;
}

decimal StarSet::VoteSingle (	decimal area1, decimal area2,
								decimal moment1, decimal moment2,
								decimal toleranceArea, decimal toleranceMoment )
{
	decimal areaVote 	= 1 - fabs(area1 - area2) / toleranceArea;
	decimal momentVote	= 1 - fabs(moment1 - moment2) / toleranceMoment;
	return (areaVote + momentVote) / 2;
}


decimal StarSet::CartesianAngle (
							Cartesian<decimal>& p1, Cartesian<decimal>& p2,
							decimal rad_per_pixel )
{
	return p1.Distance(p2) * rad_per_pixel;
}


decimal StarSet::EquatorialAngle (
							Equatorial<decimal>& p1, Equatorial<decimal>& p2,
							decimal rad_per_pixel	)
{
	return p1.RadialDistance(p2);
}





bool StarSet::SortByVoteDecending ( StarSet& larger, StarSet& smaller )
{
	return larger.vote >= smaller.vote;
}

bool StarSet::SortByVoteAscending ( StarSet& larger, StarSet& smaller )
{
	return larger.vote <= smaller.vote;
}

}
