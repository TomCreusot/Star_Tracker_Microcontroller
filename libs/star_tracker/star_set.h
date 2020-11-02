/*
 *  Stores data on a set of stars to make a unique set.
 *	@file	star_set.h
 *	@author	Tom Creusot
 */

#pragma once
#include <string>

#include "libs/util/util.h"
#include "libs/util/array_list.h"
#include "libs/util/point.h"

using namespace util;
using namespace std;

/// @namespace star_tracker	Finds unique features about the provided stars.
namespace star_tracker
{
/**
 *  Stores data on a set of stars to make a unique set.
 *	@details
 *		- The unique features are the position of the opposite (angle) star.
 *		- The angle between the stars and the opposite.
 *		- The likelyhood of it being the correct match.
 *		- The orientation of the opposite star to its farthest not pilot.
 *		- A StarSet of the image.
 *
 *	@example
 *		const util::uint in_size  = 3;
 *		const util::uint out_size = 5;
 *		util::ArrayList<util::Point<util::decimal>, in> input;
 *		util::ArrayList<star_tracker::StarSet, out> output;
 *		util::Point<util::decimal> pilot(0, 0);
 *		util::Point<util::decimal> s1(0, 1);
 *		util::Point<util::decimal> s2(1, 0);
 *		util::Point<util::decimal> s3(1, 1);
 *		util::Point<util::decimal> s4(1, 2);
 *
 *		// The stars must be in order of magnitude:
 *		input.PushBack(pilot);
 *		input.PushBack(s1);
 *		input.PushBack(s2);
 *		input.PushBack(s3);
 *		input.PushBack(s4);
 *
 *		// Finds all the combinations assuming they are ordered in apperant magnitude (brightest first).
 *		// Tries: (0, 1, 2, 3), (0, 1, 2, 4), (0, 1, 3, 4), (0, 2, 3, 4), (1, 2, 3, 4).
 *		star_tracker::StarSet::GenerateSets<in_size, out_size>(input, &output);
 *
 *		cout << output.Get(0).angle << endl; // angle between (s1, s3, s2) as s3 is farthest the opposite (pi/4, (90 degrees)).
 *
 *		// Cluster probability will favour close matching points.
 *		// If multiple matches are close to the same place, the likelyhood of this being the correct match is higher.
 *		// The more within the fov, the higher the odds.
 *		decimal fov = 1;
 *		star_tracker::StarSetClusterProbability<output_size>(fov, &output);
 *		output.Get(0).odds;
 *
 *		// For constructing the database
 *		string str;
 *		for ( int i = 0; i < output.Size(); i++ )
 *		{
 *			output.Get(i).ToArrayString(&str);
 *		}
 * @author Tom Creusot
 */

class StarSet
{

public:

	decimal area;				///< The area of the triangle.
	decimal moment;				///< The shape?
	StarSet* pixel;				///< The placement on the image.
	decimal vote;					///< The likelyhood of this being the match.
	Equatorial<decimal> position;	///< The star farthest from the pilot.


//////////////////////////////////////////////////////////////////////////////
//																			//
//						------	Constructors	------						//
//																			//
//////////////////////////////////////////////////////////////////////////////

	/** @brief Default Constructor */

	StarSet ( );


	/**
	 *	@brief	Alternate Constructor
	 *	@param pos		The position of the triangle.
	 *	@param area		The area of the triangle.
	 *	@param moment	The moment of the triangle.
	 */

	StarSet ( Point<decimal> pos, decimal area, decimal moment );




	/**
	 * @brief		Creates a star set triangle.
	 * @param s1	The pilot star.
	 * @param s2	Another star.
	 * @param s3	Another other star.
	 */

	StarSet ( Point<decimal> s1, Point<decimal> s2, Point<decimal> s3 );



	/**
	 * @brief		Copy Constructor.
	 * @param px	The pixel StarSet to copy.
	 * @details 	Sets all the values as a copy constructor.
	 */

	StarSet ( const StarSet& px );




//////////////////////////////////////////////////////////////////////////////
//																			//
//					------	Member Functions	------						//
//																			//
//////////////////////////////////////////////////////////////////////////////

	/**
	 * @brief					Finds where the center pixel is in real world.
	 */

//	Point<decimal> FindAttitude (  );








//////////////////////////////////////////////////////////////////////////////
//																			//
//							------	Static	------							//
//																			//
//////////////////////////////////////////////////////////////////////////////




	/**
	 * @brief		This is to generate StarSets with a range of Points IN SORTED ORDER.
	 * @param list			[in]	The list of points to read from.
	 * @param sets			[out]	The constructed StarSets.
	 *
	 * @tparam NI	The size of the list to read from.
	 * @tparam NO	The size of the list to write to.
	 */

	template<unsigned int NI, unsigned int NO>
	static void GenerateSets (	ArrayList<Point<decimal>, NI>& list,
								ArrayList<StarSet, NO>* sets 			)
	{
		for ( uint ii = 0; ii < list.Size(); ii++ )
			for ( uint jj = ii + 1; jj < list.Size(); jj++ )
				for ( uint kk = jj + 1; kk < list.Size(); kk++ )
				{
					Point<decimal> s0 = list.Get(ii);
					Point<decimal> s1 = list.Get(jj);
					Point<decimal> s2 = list.Get(kk);

					StarSet set(s0, s1, s2);
					sets->PushBack(set);
				}
	}



	/**
	 *	@brief	Derives the probability of each node being the most accurate by testing how clustered they are (higher if clustered).
	 *	@param sets	[in/out]	The valid elements of the database to compare and modify odds.
	 *
	 *	@tparam N1	The max size of the input set.
	 */

	template < uint N1 >
	static void Vote ( ArrayList<StarSet, N1>* sets )
	{
		for ( uint ii = 0; ii < sets->Size(); ii++ )
			for ( uint jj = ii + 1; jj < sets->Size(); jj++ ) // No repeates
			{
				decimal dist_data =
					sets->Get(ii).position.RadialDistance(sets->Get(jj).position);

				decimal dist_pixel =
					sets->Get(ii).pixel->position.RadialDistance(sets->Get(jj).pixel->position);

				decimal error = fabs(dist_data - dist_pixel) + 1;

				sets->Get(ii).vote /= error;
				sets->Get(jj).vote /= error;
			}
	}



	/**
	 * @brief	Finds the odds of the specified triangle compared to a database element.
	 * @param area1				The image triangle area.
	 * @param area2				The database triangle area.
	 * @param moment1			The image triangle moment.
	 * @param moment2			The database triangle moment.
	 * @param tolerance_area	The maximum area.
	 * @param tolerance_moment	The maximum moment.
	 * @return	The likelyhood based on the closeness to the database.
	 */

	static decimal VoteSingle (	decimal area1, decimal area2,
							decimal moment1, decimal moment2,
							decimal tolerance_area, decimal tolerance_moment);











	/**
	 * @brief	Finds the area variable for the triangle method.
	 * @param a	[in]	The first distance.
	 * @param b	[in]	The second distance.
	 * @param c	[in]	The third distance.
	 * @return			The area of the triangle or NAN if invalid triangle.
	 */

	static decimal CalcArea ( decimal a, decimal b, decimal c );


	/**
	 * @brief	Finds the moment variable for the triangle method.
	 * @param area	[in]	The previously calculated area.
	 * @param a		[in]	The first distance.
	 * @param b		[in]	The second distance.
	 * @param c		[in]	The third distance.
	 * @return				The moment of the triangle.
	 */

	static decimal CalcMoment ( decimal area, decimal a, decimal b, decimal c );



	/**
	 * @brief Used for util::ArrayList<StarSet>.sort() to sort in decending order.
	 * @param right	The element that should be larger than the other.
	 * @param left	The element that should be smaller than the other.
	 * @return			True if in order.
	 */

	static bool SortByVoteDecending		( StarSet& left, StarSet& right );

	/**
	 * @brief Used for util::ArrayList<StarSet>.sort() to sort in decending order.
	 * @param left	The element that should be smaller than the other.
	 * @param right	The element that should be larger than the other.
	 * @return			True if in order.
	 */

	static bool SortByVoteAscending		( StarSet& left, StarSet& right );
};
}
