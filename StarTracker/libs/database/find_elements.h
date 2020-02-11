/**
 *	File: 		find_elements.h
 *	Author:		Tom Creusot
 *  Purpose:	Finds the required AngleStat from a database.
 */

#pragma once

#include "database_array.h"
#include "angle_stat.h"
#include "libs/util/array_list.h"
#include "libs/util/util.h"

using namespace util;

namespace database
{

/**
 * @brief					Finds any similar angles and appends it to the list.
 * @param angle		[in]	The origional angle to search for and copy.
 * @param tolerance	[in]	If the value is within the tolerance range, it is valid.
 * @param tolerance	[out]	All the valid angles found.
 */

void find_elements ( AngleStat& angle,
							decimal tolerance, ArrayList<AngleStat>& found );


}
