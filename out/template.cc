/**
 *  @brief	A 2d array containing the database.
 *	@file	$(file)
 *	@author	Tom Creusot
 */

#pragma once

#include "libs/util/util.h"
using namespace util;

/// @namespace star_tracker
namespace star_tracker
{
/// The number of elements in the database.
const util::uint database_size = $(num_elements);

/// The star database.
const util::decimal $(array_name) [$(num_elements)][6] =
{
$(array_elements)
};

}
