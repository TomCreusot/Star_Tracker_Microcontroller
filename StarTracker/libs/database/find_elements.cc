#include "find_elements.h"

namespace database
{
void find_elements ( AngleStat& angle,
								decimal tolerance, ArrayList<AngleStat>& found )
{
	for ( int i = 0; i < database_size; i++ )
	{
		if ( fabs(angle.angle - database_array[i][0]) < tolerance )
		{
			Point<decimal> pilot(database_array[i][1], database_array[i][2]);
			Point<decimal> opposite(database_array[i][3], database_array[i][4]);
			AngleStat stat(database_array[i][0], pilot, opposite);
			stat.pixel = &angle;
			found.push_back(stat);
		}
	}
}
}
