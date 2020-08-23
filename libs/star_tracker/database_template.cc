/**
 *  @brief	A 2d array containing the database.
 *	This is constructed from template.cc with "make database".
 *	The configured field of view was $(fov) rad.
 *	@file	$(file)
 */

#pragma once

#include <vector>
#include <array>
#include "libs/util/util.h"
#include "database.h"
using namespace std;
using namespace util;

/// @namespace star_tracker
namespace star_tracker
{
/**
 * @brief this class contains the database and the field of view used to construct it.
 */

namespace database_array
{



/// The field of view of the database constructed (radians).
static constexpr util::decimal fov = $(fov);

/// The star database.
static vector<array<decimal, Database::kNumElements>> array =
{
$(array_elements)
};






}
}
