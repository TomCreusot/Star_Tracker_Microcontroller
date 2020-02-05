/*
 *	File: 		angle_stat.h
 *	Author:		Tom Creusot
 *  Purpose:	Stores details about an angle and a position in real and px.
 *				Also calculates the odds of the angles to be the same.
 *
 * Header For: 	angle_stat.cc.
 */



typedef struct AngleStat
{
	decimal angle;
	decimal angle_px;
	Point<decimal> pilot_px;
	Point<decimal> pilot;
	decimal odds;

	/** @brief Default Constructor */
	AngleStats ( );

	/**
	 * @brief Alternate Constructor
	 * @param angle_	The value of angle.
	 * @param pilot_	The value of the pilot.
	 */

	AngleStats ( decimal angle_, Point<decimal>& pilot_ )

	/**
	 * @brief			Generates probability from the stars angle and angle_px.
	 * @param maxValue	The maximam possibility it can have.
	 */

	void personalProbability ( decimal maxValue );
} AngleStats;
