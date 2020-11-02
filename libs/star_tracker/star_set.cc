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



StarSet::StarSet ( Point<decimal> pos, decimal area, decimal moment )
{
	this->area		= area;
	this->moment 	= moment;
	this->position	= pos;
	this->pixel		= NULL;
	this->vote		= 1;
}



StarSet::StarSet ( Point<decimal> s1, Point<decimal> s2, Point<decimal> s3 )
{
	decimal a = s1.RadialDistance(s2);
	decimal b = s1.RadialDistance(s3);
	decimal c = s2.RadialDistance(s3);

	this->area   = StarSet::CalcArea(a, b, c);
	this->moment = StarSet::CalcMoment(area, a, b, c);
	this->position	= s1;
	this->pixel 	= NULL;
	this->vote 		= 1;
}



StarSet::StarSet ( const StarSet& px )
{
	area		= px.area;
	moment		= px.moment;
	position	= px.position;
	pixel		= px.pixel;
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
	return sqrt(s * (s - a) * (s - b) * (s - c));
}

decimal StarSet::CalcMoment	( decimal area, decimal a, decimal b, decimal c )
{
	return area * ( a * a + b * b + c * c ) / 36.0;
}

decimal StarSet::VoteSingle (	decimal area1, decimal area2,
							decimal moment1, decimal moment2,
							decimal tolerance_area, decimal tolerance_moment )
{
	decimal areaVote 	= 1 - fabs(area1 - area2) / tolerance_area;
	decimal momentVote	= 1 - fabs(moment1 - moment2) / tolerance_moment;
	return (areaVote + momentVote) / 2;
}




bool StarSet::SortByVoteDecending ( StarSet& left, StarSet& right )
{
	return left.vote > right.vote;
}

bool StarSet::SortByVoteAscending ( StarSet& left, StarSet& right )
{
	return left.vote < right.vote;
}

}
