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
	odds	= 0;
}


AngleStat::AngleStat ( decimal angle_, Point<decimal>& pilot_ )
{
	angle	= angle_;
	pilot	= pilot_;
	pixel	= NULL;
	odds	= 0;
}


AngleStat::AngleStat ( AngleStat& px )
{
	angle = px.angle;
	pilot = px.pilot;
	odds = px.odds;
	pilot = px.pilot;
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
				// cout << database[ii].odds << "\t" << database[ii].angle << "\t\t" << database[jj].angle << "\t" << dist << endl;
				database[ii].odds /= dist * w_separation;
				// sleep_for(milliseconds(100));
			}
		}
}
}
