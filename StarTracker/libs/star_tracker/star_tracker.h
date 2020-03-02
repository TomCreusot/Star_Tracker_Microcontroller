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
#include "libs/database/angle_stat.h"


using namespace image_processing;
using namespace database;


namespace star_tracker
{

 typedef decimal (*FIND_ANGLE_PTR)	(	Point<decimal>&,
									Point<decimal>&,
									Point<decimal>&,
									Point<decimal>&);


/**
 * @brief				Gets the brightest blobs and stores them in decending order.
 * @param list	  [in]	The list of all the blobs.
 * @param reduced [out]	The reduced list of stars.
 * @param num	  [in]	The number of stars to use.
 */

void deriveBrightest ( ArrayList<Blob>& list,
								ArrayList<Point<decimal>>& reduced, uint num );


/**
 * @brief				Finds all the combinations of angles from the supplied stars.
 * @param points [in]	The points to calculate with.
 * @param angles [out]	The pilot position and the corresponding angle.
 */

 void findAnglesAllPilots ( ArrayList<Point<decimal>>& points,
 							ArrayList<AngleStat>& angles );


/**
 * @brief				Makes "s1" the futhest star from the pilot.
 * @param pilot [in]	To allow to find distance.
 * @param s1 [in/out]	The variable to be the futhest star.
 * @param s2 [in/out]	The variable to possibly swap with.
 * @param s3 [in/out]	The variabel to possibly swap with.
 */

void deriveFuthest ( Point<decimal>* pilot,
				Point<decimal>* s1, Point<decimal>* s2, Point<decimal>* s3 );


/**
* @brief					Appends all angles with one pilot.
* @param start		[in]	The start index of the array (inclusive) usualy the pilot.
* @param end		[in]	The end index of the array (exclusive).
* @param points 	[in]	The points to use.
* @param func_ptr	[in]	The function to find an individual angle.
* @param angles		[out]	The angles to append to.
*/

void findAnglesSinglePilot ( uint start, uint end,
							ArrayList<Point<decimal>> points,
							FIND_ANGLE_PTR func_ptr,
							ArrayList<AngleStat>& angles );



/**
* @brief 				Finds angle between the stars and the futhest from the pilot.
* @param pilot			The pilot star.
* @param s1				The futhest star.
* @param s2				The other star.
* @param s3				The another other star.
* @return 				The angle OR 1000 if invalid (points are the same).
*/

decimal findAngle (Point<decimal>& pilot,
				   Point<decimal>& s1, Point<decimal>& s2, Point<decimal>& s3 );
}
