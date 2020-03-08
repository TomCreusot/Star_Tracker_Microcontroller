/*
 *	File: 		angle_stat.h
 *	Author:		Tom Creusot
 *  Purpose:	Stores details about an angle and a position in real and px.
 *				Also calculates the odds of the angles to be the same.
 *
 * Header For: 	angle_stat.cc.
 */

#pragma once

#include "libs/util/util.h"
#include "libs/util/array_list.h"
#include "libs/util/point.h"

using namespace util;

namespace database
{
class AngleStat
{
public:
	decimal angle;				///< The angle of the opposite star.
	Point<decimal> pilot;		///< The brightest star.
	Point<decimal> opposite;	///< The star futhest from the pilot.
	AngleStat* pixel;			///< The placement on the image.
	decimal odds;				///< The likelyhood of this being the match.

	/** @brief Default Constructor */
	AngleStat ( );

	/**
	 * @brief Alternate Constructor
	 * @param angle_	The value of angle.
	 * @param pilot_	The value of the pilot.
	 * @param opposite_	The value of the star futhest from the pilot.
	 */

	AngleStat ( decimal angle_,
					Point<decimal>& pilot_, Point<decimal>& opposite_ );


	/**
	 * @brief Alternate Constructor
	 * @param px	The pixel AngleStat to copy and set.
	 * @details 	Sets pixel and sets all the values as a copy constructor.
	 */

	AngleStat ( AngleStat& px );



	/**
	 * @brief	Generates probability from the stars angle and pixel angle.
	 * @details	The closer the angles, the lower the %.
	 */

	void personalProbability ( );


	/**
	 * @brief					Finds where the center pixel is in real world.
	 * @param fov  		[in]	The diagonal feild of view of the camera.
	 * @param wodth		[in]	The width of the image.
	 * @param height	[in]	The height of the image.
	 * @param center 	[out]	The position to assign.
	 */

	void findCenter ( decimal fovX, decimal fovY, decimal width, decimal height,
									Point<decimal>& center );





	/**
	 *	@brief	Derives the probability of each node being the most accurate by testing how clustered they are (higher if clustered).
	 *	@param database [in/out]	The valid elements of the database to compare and modify odds.
	 *	@param wSeparation [in]		The weighting for the separation (0 - 1).
	 *	@param fov					The feild of view which will not deduct probability.
	 *	@details	If stars are within the FOV, they will not loose odds, if they are outside they start loosing at a weighting of "wSeparation".
	 */

	static void clusterProbability ( ArrayList<AngleStat>& database,
											decimal wSeparation, decimal fov );


};
}
