#include "angle_stat.h"

#include <iostream>
#include <thread>         // std::this_thread::sleep_for
#include <chrono>
using namespace std;
using namespace chrono;
using namespace this_thread;

namespace database
{
AngleStat::AngleStat ( )
{
	angle	= 0;
	pilot	= util::Point<decimal>(0, 0);
	pixel	= NULL;
	odds	= 1;
}


AngleStat::AngleStat ( decimal angle_,
	 						Point<decimal>& pilot_, Point<decimal>& opposite_ )
{
	angle		= angle_;
	pilot		= pilot_;
	opposite	 = opposite_;
	pixel		= NULL;
	odds		= 1;
}


AngleStat::AngleStat ( AngleStat& px )
{
	angle 		= px.angle;
	pilot 		= px.pilot;
	opposite	= px.opposite;
	odds 		= px.odds;
	pixel		= px.pixel;
}



void AngleStat::personalProbability ( )
{
	if ( pixel != NULL )
	{
		odds = 1 / (fabs(angle - pixel->angle) + 1);
	}
}



void AngleStat::clusterProbability ( ArrayList<AngleStat>& database,
											decimal w_separation, decimal fov )
{
	for ( uint ii = 0; ii < database.size(); ii++ )
		for ( uint jj = 0; jj < database.size(); jj++ )
		{
			const decimal dist =
							database[ii].pilot.distance(database[jj].pilot);
			if ( dist > fov )
			{
				database[ii].odds -= w_separation;
			}
		}
}
}
