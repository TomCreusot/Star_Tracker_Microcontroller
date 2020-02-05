/*
 *	File: 		star_tracker.h
 *	Author:		Tom Creusot
 *  Purpose: Uses the Pyramid method to calculate an angle which will be unique.
 *
 * Header For: 	star_tracker.cc, combo.cc, angle_stat.cc.
 */

#pragma once

#include <vector>

#include "libs/util/util.h"
#include "libs/util/point.h"
#include "libs/image_processing/blob.h"

using namespace std;
using namespace image_processing;

namespace st
{



/**
* A container class which stores 3 variables.
* This is useful for storing the output of combinations.
*/

class Combo
{
public:
	uint pilot, s1, s2, s3;

	/**
	* @brief 	Default Constructor
	* @details	Sets all values to NULL.
	*/

	Combo	( );


	/**
	* @brief 		Alternate Constructor.
	* @param p		The value of the pilot.
	* @param sA		The value of s1
	* @param sB		The value of s2
	* @param sC		The value of s3
	*/

	Combo	( uint p, uint sA, uint sB, uint sC );


	/**
	* @brief 		Tests if the input and this object have the same value.
	* @param p		The value of the pilot.
	* @param sA		The element to test with.
	* @param sB		The other element to test with.
	* @param sC		The other other element to test with.
	* @return		If they are equal.
	* @details		If the nodes point to the same location, it is equal.
	*/

	bool equal	( uint , uint sA, uint sB, uint sC );
};




/**
 * @brief				Gets the brightest blobs and stores them in decending order.
 * @param list	  [in]	The list of all the blobs.
 * @param reduced [out]	The reduced list of stars.
 * @param num	  [in]	The number of stars to use.
 */

void deriveBrightest ( ArrayList<Blob>& list,
								ArrayList<Point<decimal>>& reduced, uint num );


/**
 *	@brief	Derives the probability of each node being the most accurate by testing how clustered they are (higher if clustered).
 *	@param database [in/out]	The valid elements of the database to compare and modify odds.
 *	@param wSeparation [in]		The weighting for the separation (0 - 1).
 *	@param fov					The feild of view which will not deduct probability.
 *	@details	If stars are within the FOV, they will not loose odds, if they are outside they start loosing at a weighting of "wSeparation".
 */

void clusterProbability ( ArrayList<AngleStat>& database, decimal wSeparation );




/**
 * @brief				Finds all the combinations of angles from the supplied stars.
 * @param points [in]	The points to calculate with.
 * @param combos [in]	The combos to use with points.
 * @param angles [out]	The pilot position and the corresponding angle.
 */

void findAngles ( ArrayList<Point<decimal>>& points, ArrayList<Combo>& combos,
												ArrayList<AngleStat>& angles );



/**
 * @brief					Gets every index conbination of multiple pilot stars in case of outlier.
 * @param numPilots	[in]	The number of pilots to sample.
 * @param end		[in]	The exclusive index to stop at.
 * @param combos	[out]	The combinations to append to.
 */

void combinationsPilots ( uint numPilots, uint end, ArrayList<Combo>& combos );




/**
* @brief				Creates a list of all the possible combinations of index's.
* @param start	[in]	The start index of the array (inclusive) usualy the pilot.
* @param end	[in]	The end index of the array (exclusive).
* @param combos	[out]	The combinations as an output.
*/

void combinations ( uint start, uint end, ArrayList<Combo>& combos );



/**
* @brief 				Finds angle between the stars and the futhest from the pilot.
* @param points	[in]	The points to read from.
* @param combo	[in]	The combinations index of the stars.
* @return 				The angle OR 1000 if invalid (points are the same).
*/

decimal findAngle ( ArrayList<Point<decimal>>& points, Combo& combo);
}
