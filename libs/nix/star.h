/**
 * @brief	This is for storing the position and magnitude of a star.
 *			it is essentialy a util point which is sortable.
 *
 * @file star.h
 * @author Tom Creusot
 */


#pragma once

#include <list>
#include <vector>
#include <string>
#include <fstream>

#include "libs/util/util.h"
#include "libs/util/array_list.h"
#include "libs/util/point.h"


using namespace std;
using namespace util;


namespace star_tracker
{

/**
 * @brief	This is for storing the position and magnitude of a star.
 *			it is essentialy a util point which is sortable.
 * @example
 *			const uint max_num_stars = 1000;
 *			const uint index_ra = 10, index_dec = 12, index_mag = 100;
 *
 *			ArrayList<Star, max_num_stars> stars;
 *			StarsFromCSV("file.csv", index_ra, index_dec, index_mag, &stars);
 *			stars.Sort(star_tracker::Star::SortByMagnitude);
 *
 * @author Tom Creusot
 */

class Star
{
public:
	/// The location x: ra, y: dec.
	Equatorial<decimal> position;
	/// The apparant brightness of the star.
	decimal magnitude;

	/**
	 *	@brief		Default Constructor.
	 */

	Star (  )
	{
		position.x = 0;
		position.y = 0;
		magnitude = 0;
	}

	/**
	*	@brief		Alternate Constructor.
	*	@param pos	x: ra, y: dec
	*	@param mag	The apparant magnitude.
	*/

	Star ( Point<decimal> pos, decimal mag  )
	{
		position = pos;
		magnitude = mag;
	}

	/**
	 *	@brief		Alternate Constructor.
	 *	@param ra	The right ascention.
	 *	@param dec	The declination.
	 *	@param mag	The apparant magnitude.
	 */

	Star ( decimal ra, decimal dec, decimal mag  )
	{
		position.x = ra;
		position.y = dec;
		magnitude = mag;
	}


	/**
	 *	@brief	Appends all the stars in the csv file to the list.
	 *	@param file			[in]	The name of the file.
	 *	@param cutoff_mag	[in]	Does not include anything above.
	 *	@param ra			[in]	The index of the ra (2000).
	 *	@param dec			[in]	The index of the dec (2000).
	 *	@param mag			[in]	The index of the apparant magnitude.
	 *	@param star_list	[out]	The list to append to.
	 *	@tparam N					The max size of the array list.
	 *	@details
	 *			The input must be in decimal ra hms and decimal dec degms
	 */
	template<const uint N>
	static void StarsFromCSV ( 	string file, const decimal cutoff_mag,
								const uint ra, const uint dec, const uint mag,
								ArrayList<Star, N>* star_list	)
	{
		ifstream strm;
		strm.open(file, std::ifstream::in);

		string line;
		std::vector<string> columns;

		while ( getline(strm, line) )
		{
			columns = std::vector<string>();
			Split(',', line, &columns);

			try
			{
				util::decimal x		= std::stod(columns[ra]);
				util::decimal y		= std::stod(columns[dec]);
				util::decimal m		= std::stod(columns[mag]);
				if ( m < cutoff_mag )
				{
					// util::Point<util::decimal> pt(x, 0, 0, y, 0, 0); // Converts to 90/90 degrees from 24/90 degrees
					// Star s(pt, m);
					Equatorial<decimal> point;
					point.RaHour(x);
					point.DecDeg(y);
					Star s(point, m);//s(x, y, m);
					columns = std::vector<string>();
					star_list->PushBack(s);
				}
			}
			catch ( invalid_argument ) {}//If line is not a proper line, skip.
		}

		strm.close();
	}



	/**
	 * @brief	Splits a string along each token.
	 * @param token		[in]	The character to split on.
	 * @param in 		[in]	The input string.
	 * @param out		[out]	The split string.
	 */

	static void Split ( char token, string in, vector<string>* out )
	{
		string str;

		for ( uint i = 0; i < in.size(); i++ )
		{
			if ( in[i] == token )
			{
				out->push_back(str);
				str = string();
			}
			else	str.push_back(in[i]);
		}

		out->push_back(str);
	}



	/**
	 *	@brief Finds the brightest stars within the specified field of view of the pilot.
	 *	@param start	[in]	The pilot location.
	 *	@param num		[in]	The number of stars to add to the list.
	 *	@param dist		[in]	The max distance stars can be appart (fov).
	 *	@param in		[in]	The input database.
	 *	@param out		[out]	The stars close to the pilot.
	 *
	 *	@tparam NI				The size of the input list.
	 *	@tparam NO				The size of the output list.
	 *	@note
	 *			Requires a sorted list.
	 */

	template<const uint NI, const uint NO>
	static void FindCloseStars (	uint start, uint num, decimal dist,
									ArrayList<Star, NI>& in,
									ArrayList<Equatorial<decimal>, NO>* out)
	{
		for ( uint i = start; i < in.Size() && out->Size() < num; i++ )
		{
			// The start will always be added because the distance to itself is 0.
			decimal distance = in.Get(start).position.RadialDistance(in.Get(i).position);

			if ( distance < dist )
			{
				out->PushBack(in.Get(i).position);
			}
		}
	}



	/**
	 * @brief	Checks if the elements are in sorted order of magnitude (smallest first).
	 * @param smaller	[in]	The smaller element.
	 * @param larger	[in]	The larger element.
	 * @return True if in order.
	 */

	inline static bool SortByMagnitude ( Star & smaller, Star & larger )
	{
		return smaller.magnitude < larger.magnitude;
	}



	/**
	 * @brief				Greater operator (magnitude).
	 * @param rhs  [in]		The value of the right hand side.
	 * @return True if this is greater than rhs.
	 */

	inline bool operator > ( const Star& rhs ) const
	{
		return magnitude > rhs.magnitude;
	}

};






}
