#pragma once

#include "libs/util/util.h"

using namespace util;

/// @namespace config all the variables to setup for the code to function.
namespace config
{

/**
* These settings are required for the runtime program, both for embedded and nix.
* Change the variables to best fit the context of the circumstance.
*/


/// The allowed difference in area from the database to be valid.
const decimal	tolerance_area		= 0.1;
/// The allowed difference in moment from the database to be valid.
const decimal	tolerance_moment	= 1;



/// The resolution of the thresholding (1 - 255) as 255 being the most precise.
/// This is also the number of bytes to use to generate the histogram.
const uint 		histogram_bars		= 255;

/// The agression of the threshold (ideal ~ 0.999).
const decimal	threshold_tolerance	= 0.9991;


/// The maximum amount of stars to observe from the image.
/// The higher the value, the more storage required.
const uint 		max_points			= 20;

/// The maximum amount of unique triangles... to find matches from.
/// The higher the value, the more storage required.
const uint		max_sets			= 100;

/// The number of matches from the database (the higher the more memory).
const uint		max_matches			= 30;

/// The maximum number of matches to use per star.
/// This is important as if there are lots of matches for a single star and it exceeds kMaxMatches,
/// If the star is a false positive, it will throw all the results off.
/// Lowering the max number of matches per star will reduce this error.
const uint		max_matches_per_star= 10;



/// When finding elements in the database, the difference from the actual value.
/// The range of the database will be (0 - FOV).
const decimal	distance_tolerance	= 0.5;


/// This must be >= the width of the image.
const uint		image_width			= 2000;
/// This must be >= the height of the image.
const uint		image_height		= 1081;
}
