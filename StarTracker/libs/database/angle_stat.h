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
	decimal angle;
	Point<decimal> pilot;
	AngleStat* pixel;
	decimal odds;

	/** @brief Default Constructor */
	AngleStat ( );

	/**
	 * @brief Alternate Constructor
	 * @param angle_	The value of angle.
	 * @param pilot_	The value of the pilot.
	 */

	AngleStat ( decimal angle_, Point<decimal>& pilot_ );


	/**
	 * @brief Alternate Constructor
	 * @param px	The pixel AngleStat to copy and set.
	 * @details sets pixel and sets all the values as a copy constructor.
	 */

	AngleStat ( AngleStat& px );



	// AngleStat ( AngleStat)



	/**
	 * @brief	Generates probability from the stars angle and pixel angle.
	 * @details	The closer the angles, the lower the %.
	 */

	void personalProbability ( );




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
