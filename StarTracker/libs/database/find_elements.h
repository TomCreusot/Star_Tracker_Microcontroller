/**
 *	File: 		find_elements.h
 *	Author:		Tom Creusot
 *  Purpose:	Finds the required AngleStat from a database.
 */

#pragma once

#include "database_array.hpp"
#include "libs/utils/util.hpp"
#include "libs/star_tracker/star_tracker.hpp"

using namespace util;
using namespace star_tracker;

namespace db
{

/**
 * @brief					Finds any similar angles and appends it to the list.
 * @param angle		[in]	The origional angle to search for and copy.
 * @param tolerance	[in]	If the value is within the tolerance range, it is valid.
 * @param tolerance	[out]	All the valid angles found.
 */

void find_elements ( AngleStat angle, decimal tolerance, ArrayList<AngleStat>& found );


}
