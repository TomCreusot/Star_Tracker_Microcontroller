/*
 *  Stores data on a set of stars to make a unique set.
 *	@file	StarSet.h
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
private:
	/// The field of view of the camera (diagonal).
	static util::decimal fov;
	/// The number of pixels diagonaly on the camera.
	static util::decimal pixel_resolution;


public:
	/// The index of the angle in the database
	constexpr static util::uint kDatabaseIndexAngle 		= 0;
	/// The index of the distance in the database
	constexpr static util::uint kDatabaseIndexDist 			= 1;
	/// The index of the orientation in the database
	constexpr static util::uint kDatabaseIndexOrientation 	= 2;
	/// The index of the opposite star ra/x in the database
	constexpr static util::uint kDatabaseIndexOppositeX 	= 3;
	/// The index of the opposite star dec/y in the database
	constexpr static util::uint kDatabaseIndexOppositeY 	= 4;

	/// The number of elements per set.
	constexpr static util::uint kDatabaseNumElements 		= 5;



	/// If the angle could not be found, this is what it is equal to.
	constexpr static util::decimal kInvalidAngle = NAN;
	///	When finding ClusterProbability, divide by this if not within range.
	constexpr static util::decimal kSeparationDiv = 1.000001;






	decimal angle;				///< The angle of the opposite star.
	decimal distance;			///< The farthest distance from the opposite to the farthest other star.
	StarSet* pixel;				///< The placement on the image.
	decimal odds;				///< The likelyhood of this being the match.
	decimal orientation;		///< The angle from up (0, 1).
	Point<decimal> opposite;	///< The star farthest from the pilot.

	/** @brief Default Constructor */
	StarSet ( );




	/**
	* @brief			Alternate Constructor. Creates a star set from a set of stars finding the angle...etc.
	* @param p	[in]	The pilot star (the brightest).
	* @param s1	[in]	Another star.
	* @param s2	[in]	Another other star.
	* @param s3	[in]	Another other, another star.
	*/

	StarSet ( 	Point<decimal>& p,
				Point<decimal>& s1, Point<decimal>& s2, Point<decimal>& s3 );




	/**
	 * @brief		Copy Constructor.
	 * @param px	The pixel StarSet to copy.
	 * @details 	Sets all the values as a copy constructor.
	 */

	StarSet ( StarSet& px );




	/**
	 * @brief					Finds where the center pixel is in real world.
	 * @param fovPP		[in]	The degrees per pixel field of view.
	 * @param size		[in]	The size of the image.
	 * @param center 	[out]	The position to assign.
	 */

	void FindCenter (	decimal fovPP,
						Point<int>& size,
						Point<decimal>* center );




	/**
	 * @brief	This is to compile the database, a string is returned of one line of values describing the set.
	 * @param str [out]	The string to append to.
	 */

	void ToArrayString ( string* str );







	/**
	 * @brief				Takes in a list of stars sorted in apperant magnitude and finds all the sets.
	 * @param list   [in]	A sorted list of star locations, brightest first.
	 * @param start  [in]	The first element to start in the list (inclusive).
	 * @param end    [in]	The element to end at (exclusive).
	 * @param sets	 [out]	The output sets.
	 *
	 * @tparam N1			The max number of input points.
	 * @tparam N2			The max number of output sets.
	 */

	template<unsigned int N1, unsigned int N2>
	static void GenerateSetsPilots ( 	ArrayList<Point<decimal>, N1>& list,
										uint start, uint end,
										ArrayList<StarSet, N2>* sets		)
	{
		for ( uint ii = start; ii < end; ii++ )
			for ( uint jj = ii + 1; jj < end; jj++ )
				for ( uint kk = jj + 1; kk < end; kk++ )
					for ( uint ll = kk + 1; ll < end; ll++ )
					{
						Point<decimal>* pilot 	= &list.Get(ii);
						Point<decimal>* s1 		= &list.Get(jj);
						Point<decimal>* s2 		= &list.Get(kk);
						Point<decimal>* s3 		= &list.Get(ll);

						StarSet set(*pilot, *s1, *s2, *s3);
						if ( set.angle != kInvalidAngle ) sets->PushBack(set);
					}
	}






	/**
	 *	@brief	Derives the probability of each node being the most accurate by testing how clustered they are (higher if clustered).
	 *	@param database 	[in/out]	The valid elements of the database to compare and modify odds.
	 *
	 *	@tparam N1	The max size of the input database.
	 *	@details	If stars are within the FOV, they will not loose odds, if they are outside they start loosing at a weighting of "kSeparationDiv".
	 */

	template < uint N1 >
	static void ClusterProbability ( ArrayList<StarSet, N1>* database )
	{
		for ( uint ii = 0; ii < database->Size(); ii++ )
			for ( uint jj = 0; jj < database->Size(); jj++ )
			{
				if ( ii != jj )
				{
					decimal dist =
							database->Get(ii).opposite.Distance(database->Get(jj).opposite);
					if ( dist > fov )
					{
						database->Get(ii).odds /= kSeparationDiv;
					}
				}
			}
	}




	/**
	 * @brief orders s1/s2 in order of distance from pilot closest first.
	 * @param pilot	[in]		The point to compare to.
	 * @param s1	[in/out]	Will be the closest.
	 * @param s2	[in/out]	Will be the farthest.
	 */

	static void SortDistance	( 	Point<decimal>& pilot,
									Point<decimal>* s1, Point<decimal>* s2	);




	/**
	* @brief 				Finds angle between 3 points.
	* @param center	[in]	The star in the middle.
	* @param left	[in]	The other star.
	* @param right	[in]	The another other star.
	* @return 				The angle OR 1000 if invalid (points are the same).
	* @details				If any are the same position, it would cause a problem, an invalid value is equal kInvalidAngle.
	*/

	static decimal FindAngle ( 	Point<decimal>& center,
								Point<decimal>& left, Point<decimal>& right );


	/**
	 * @brief					Input pixel positions, output real distance between the two.
	 * @param opposite 	[in]	The pixel position of the found opposite.
	 * @param other		[in]	The pixel position of the found farthest from the opposite.
	 * @return					The distance in real coordinates.
	 */

	static inline decimal FindRealDistance ( 	Point<decimal>& opposite,
												Point<decimal>& other )
	{
		return opposite.Distance(other) * fov / pixel_resolution;
	}




	/**
	 * @brief Used for util::ArrayList<StarSet>.sort() to sort in decending order.
	 * @param larger	The element that should be larger than the other.
	 * @param smaller	The element that should be smaller than the other.
	 * @return			True if in order.
	 */

	static bool SortByOdds			( StarSet& larger, StarSet& smaller );








	/**
	 * @brief	Finds any similar angles and appends it to the list.
	 * @param database		[in]	The database to read from.
	 * @param database_size	[in]	The number of elements in the database.
	 * @param angles		[in]	The origional angles to search for and copy.
	 * @param tA			[in]	The +- threshold for the angle.
	 * @param tD			[in]	The +- threshold for the distance from the opposite.
	 * @param found			[out]	The found points.

	 * @tparam NI				The size of the input list.
	 * @tparam NO				The size of the output list.
	 */

	template<const uint NI, const uint NO>
	static void FindElements (
							const decimal database[][6], uint database_size,
							ArrayList<star_tracker::StarSet, NI>& angles,
							decimal tA, decimal tD,
							ArrayList<star_tracker::StarSet, NO>* found
							)
	{
		for ( uint ii = 0; ii < angles.Size(); ii++)
		{
			for ( uint jj = 0; jj < database_size; jj++ )
			{
				decimal dAngle = database[jj][kDatabaseIndexAngle];
				decimal dDist = database[jj][kDatabaseIndexDist];

				if ( 	fabs(dAngle - angles.Get(ii).angle) < tA)// &&
					//	fabs(dDist - angles.Get(ii).distance) < tD	)
				{
					Point<decimal> opposite(
								database[jj][kDatabaseIndexOppositeX],
								database[jj][kDatabaseIndexOppositeY]);
					star_tracker::StarSet set;
					set.angle = dAngle;
					set.distance = dDist;
					set.opposite = opposite;
					set.orientation = database[jj][kDatabaseIndexOrientation];

					set.pixel = &angles.Get(ii);
					found->PushBack(set);
				}
			}
		}
	}






















	/**
	 * @brief	Sets the DIAGONAL field of view of the camera for all future calculations.
	 * @param f	The field of view.
	 * @details	This is static as it requires less memory storage, it is assumed that there will be only one fov.
	 */
	static inline void set_fov				( decimal f ) {		fov = f;	}

	/**
	* @brief	Sets the DIAGNONAL resolution of the camera for all future calculations.
	* @param p	The diagonal resolution.
	* @details	This is static as it requires less memory storage, it is assumed that there will be only one resolution.
	*/
	static inline void set_pixel_resolution ( decimal p ){pixel_resolution = p;}
};
}
