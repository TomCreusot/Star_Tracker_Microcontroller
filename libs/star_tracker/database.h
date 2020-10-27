/*
 *	Stores the a database which can then be used for testing or for the actual database.
 *	@file	database.h
 *	@author	Tom Creusot
 */
#pragma once

#include <array>
#include <vector>
#include "libs/util/util.h"
#include "star_set.h"

using namespace std;
using namespace util;

namespace star_tracker
{
/**
 *	This is just to store details about a Database so it can be used for testing or to store the actual database.
 */

class Database
{
public:
	/// The index of the area of the star in the database.
	constexpr static uint kIndexArea		= 0;
	/// The index of the moment of the star in the database.
	constexpr static uint kIndexMoment 		= 1;
	/// The index of the opposite star ra/x in the database
	constexpr static uint kIndexRA 			= 2;
	/// The index of the opposite star dec/y in the database
	constexpr static uint kIndexDEC		 	= 3;

	/// The number of elements per set.
	constexpr static uint kNumElements 		= 4;



	decimal fov;			///<	The field of view chosen for the camera and database.
	decimal rad_per_pixel;	///<	The number of radians of fov in each pixel.
	vector<array<decimal, kNumElements>>* database = NULL;	///< The database to read from.


	/**
	 * @brief Alternate Constructor
	 * @param fov			[in]	The field of view (RADIANS) of the image.
	 * @param rad_per_pixel	[in]	The field of view per pixel on the image.
	 * @param database		[in]	The database of elements.
	 */
	Database (	decimal fov, decimal rad_per_pixel,
				vector<array<decimal, kNumElements>>* database )
	{
		this->database		= database;
		this->rad_per_pixel	= rad_per_pixel;
		this->fov			= fov;
	}


	/**
	 * @brief			Accessor of database.
	 * @param row		The index of the row.
	 * @param column	The index of the column.
	 * @return			The value at the position.
	 */

	decimal Get ( uint row, uint column )
	{
		return (*database)[row][column];
	}


	/**
	 * @brief			Accessor of database size.
	 * @return			The size of the database.
	 */
	uint Size ( )
	{
		return (uint)database->size();
	}

	/**
	 * @brief	This is to compile the database, a string is returned of one line of values describing the set.
	 * @param set [in]	The star set to generate a string of.
	 * @param str [out]	The string to append to.
	 */

	static void ToArrayString	( StarSet& set, string* str )
	{
		for ( uint i = 0; i < Database::kNumElements; i++ )
		{
			switch ( i )
			{
				case Database::kIndexArea:
				*str += to_string(set.area);
				break;
				case Database::kIndexMoment:
				*str += to_string(set.moment);
				break;
				case Database::kIndexRA:
				*str += to_string(set.position.x);
				break;
				case Database::kIndexDEC:
				*str += to_string(set.position.y);
				break;
			}
			if ( i != Database::kNumElements - 1 ) *str += ", ";
		}
	}





	/**
	 * @brief					Creates a star set from the database row.
	 * @param row				The index of the database.
	 * @param row_value	[out]	The value of the row.
	 */

	void DatabaseToStar ( uint row, StarSet* row_value )
	{
		Point<decimal> pilot (	Get(row, kIndexRA),
								Get(row, kIndexDEC) );
		decimal area = 			Get(row, kIndexArea);
		decimal moment = 		Get(row, kIndexDEC);
		row_value->area = area;
		row_value->moment = moment;
		row_value->position = pilot;
	}



	/**
	 * @brief	Finds any similar angles and appends it to the list.
	 * @param stars			[in]	The origional sets to search for and copy.
	 * @param t_a			[in]	The +- threshold for the area.
	 * @param t_m			[in]	The +- threshold for the moment.
	 * @param max_per_star	[in]	The maximum comparisons per star.
	 * @param found			[out]	The found points.

	 * @tparam NI				The size of the input list.
	 * @tparam NO				The size of the output list.
	 */

	template<const uint NI, const uint NO>
	void FindElements	(	ArrayList<StarSet, NI>& stars,
									decimal t_a, decimal t_m, uint max_per_star,
									ArrayList<StarSet, NO>* found	)
	{
		for ( uint ii = 0; ii < stars.Size(); ii++)
		{
			// Check every item in the input list.
			for ( uint jj = 0; jj < Size(); jj++ )
			{
				StarSet dbSet;
				DatabaseToStar(jj, &dbSet);
				dbSet.vote =
					StarSet::VoteSingle(stars.Get(ii).area, dbSet.area, stars.Get(ii).moment, dbSet.moment, t_a, t_m);


				if ( dbSet.vote > 0 )
				{
					dbSet.pixel = &stars.Get(ii);
					found->Slot(dbSet, &StarSet::SortByVoteDecending);
				}
			}
		}
	}
};

}
