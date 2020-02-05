#include "star_tracker.h"

AngleStat::AngleStat ( )
{
	angle	= 0;
	pilot	= util::Point<decimal>(0, 0);
	anglePx	= 0;
	pilotPx	= util::Point<decimal>(0, 0);
	odds	= 0;
}


AngleStat::AngleStat ( decimal angleP, Point<decimal> pilotP )
{
	angle	= 0;
	pilot	= util::Point<decimal>(0, 0);
	anglePx	= 0;
	pilotPx	= pilotP;
	odds	= angleP;
}


void AngleStat::personalProbability ( decimal maxValue )
{
	odds = maxValue - abs(angle - anglePx) / ;
}
