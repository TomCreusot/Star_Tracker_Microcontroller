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
 *		star_tracker::StarSet::set_fov(100);	// Sets the field of view to 100 for all calculations.
 *
 *
 *		const util::uint input_size = 3;
 *		const util::uint output_size = 5;
 *		util::ArrayList<util::Point<util::decimal>, input_size> input;
 *		util::ArrayList<star_tracker::StarSet, output_size> output;
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
 *		star_tracker::StarSet::GenerateSetsPilots<input_size, output_size>(input, 0, 5, &output);
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
	decimal vote;				///< The likelyhood of this being the match.
	Point<decimal> position;	///< The star farthest from the pilot.



	///	When finding Vote, divide by this if not within range.
	constexpr static decimal kSeparationDiv = 1.000001;

//////////////////////////////////////////////////////////////////////////////
//																			//
//						------	Constructors	------						//
//																			//
//////////////////////////////////////////////////////////////////////////////

	/** @brief Default Constructor */

	StarSet ( );


	/**
	 * @brief		Alternate Constructor. Creates a star set triangle method.
	 * @param pos		[in]	The position of the pilot star.
	 * @param area		[in]	The area of the triangle.
	 * @param moment	[in]	The moment of the triangle shape.
	 */

	StarSet ( Point<decimal> pos, decimal area, decimal moment );



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
	 * @param size		[in]	The size of the image.
	 * @return center 			The center of the image.
	 *

	Point<decimal> FindCenter ( Point<int>& size );
*/







//////////////////////////////////////////////////////////////////////////////
//																			//
//							------	Static	------							//
//																			//
//////////////////////////////////////////////////////////////////////////////




	/**
	 * @brief		This is to generate StarSets with a range of Points IN SORTED ORDER.
	 * @param list			[in]	The list of points to read from.
	 * @param start			[in]	The index to start from (inclusive).
	 * @param end			[in]	The index to end on (exclusive).
	 * @param rad_per_pixel	[in]	The angle resolution in the camera.
	 * @param func_dist		[in]	The function to generate the distance (Cartesian/Equatorial Angle).
	 * @param sets			[out]	The constructed StarSets.
	 */

	template<unsigned int N1, unsigned int N2>
	static void GenerateSets (
				ArrayList<Point<decimal>, N1>& list, uint start, uint end,
				decimal rad_per_pixel,
				decimal (*func_dist)(Point<decimal>&, Point<decimal>&, decimal),
				ArrayList<StarSet, N2>* sets )
	{
		for ( uint ii = start; ii < end; ii++ )
			for ( uint jj = ii + 1; jj < end; jj++ )
				for ( uint kk = jj + 1; kk < end; kk++ )
				{
					Point<decimal> s0 = list.Get(ii);
					Point<decimal> s1 = list.Get(jj);
					Point<decimal> s2 = list.Get(kk);


					decimal a1 = func_dist(s0, s1, rad_per_pixel);
					decimal a2 = func_dist(s0, s2, rad_per_pixel);
					decimal a3 = func_dist(s1, s2, rad_per_pixel);

					decimal area = StarSet::CalcArea(a1, a2, a3);
					decimal moment = StarSet::CalcMoment(area, a1, a2, a3);

					StarSet set(s0, area, moment);
					sets->PushBack(set);
				}
	}



	/**
	 *	@brief	Derives the probability of each node being the most accurate by testing how clustered they are (higher if clustered).
	 *	@param fov	[in]		The field of view which is valid.
	 *	@param sets	[in/out]	The valid elements of the database to compare and modify odds.
	 *
	 *	@tparam N1	The max size of the input set.
	 *	@details	If stars are within the FOV, they will not loose odds, if they are outside they start loosing at a weighting of "kSeparationDiv".
	 */

	template < uint N1 >
	static void Vote ( decimal fov, ArrayList<StarSet, N1>* sets )
	{
		for ( uint ii = 0; ii < sets->Size(); ii++ )
			for ( uint jj = ii + 1; jj < sets->Size(); jj++ ) // No repeates
			{
				decimal dist =
					sets->Get(ii).position.RadialDistance(sets->Get(jj).position);
				std::cout << dist << " > " << fov << " " << (dist > fov) << "\t" << ii << ", " << jj << std::endl;
				if ( dist > fov )
				{
					sets->Get(ii).vote /= StarSet::kSeparationDiv;
					sets->Get(jj).vote /= StarSet::kSeparationDiv;
				}
			}
	}



	/**
	 * @brief	Finds the odds of the specified triangle compared to a database element.
	 * @param area1				The image triangle area.
	 * @param area2				The database triangle area.
	 * @param moment1			The image triangle moment.
	 * @param moment2			The database triangle moment.
	 * @param toleranceArea		Increase this and the odds increase with area, ( > 0).
	 * @param toleranceMoment	Increase this and the odds increase with moment ( > 0).
	 * @return	The likelyhood based on the closeness to the database.
	 */

	static decimal VoteSingle (	decimal area1, decimal area2,
								decimal moment1, decimal moment2,
								decimal toleranceArea, decimal toleranceMoment);











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
	 * @brief		Finds the angle between the points on a cartesian coordinate system.
	 * @param p1 				[in]	The first star.
	 * @param p2 				[in]	The second star.
	 * @param rad_per_pixel 	[in]	The angular resolution per pixel.
	 * @details				It is scalar so it should not matter which order.
	 */

	static decimal CartesianAngle (
							Cartesian<decimal>& p1, Cartesian<decimal>& p2,
							decimal rad_per_pixel );


	/**
	* @brief		Finds the angle between the points with spherical coordinates.
	* @param p1 	[in]	The first star.
	* @param p2 	[in]	The second star.
	* @param rad_per_pixel 	[in]	Satisfies a function pointer, literaly useless.
	* @details				It is scalar so it should not matter which order.
	*/

	static decimal EquatorialAngle (
							Equatorial<decimal>& p1, Equatorial<decimal>& p2,
							decimal rad_per_pixel	);



	/**
	 * @brief Used for util::ArrayList<StarSet>.sort() to sort in decending order.
	 * @param larger	The element that should be larger than the other.
	 * @param smaller	The element that should be smaller than the other.
	 * @return			True if in order.
	 */

	static bool SortByVoteDecending		( StarSet& larger, StarSet& smaller );

	/**
	 * @brief Used for util::ArrayList<StarSet>.sort() to sort in decending order.
	 * @param larger	The element that should be larger than the other.
	 * @param smaller	The element that should be smaller than the other.
	 * @return			True if in order.
	 */

	static bool SortByVoteAscending			( StarSet& larger, StarSet& smaller );
};
}
