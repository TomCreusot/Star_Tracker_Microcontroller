/*
 *	This is for the settings of the runtime environment.
 *	This is a header file as the embedded software does not support
 */
#pragma once
#include "libs/util/util.h"
using namespace util;
namespace properties
{
class Properties
{
public:
	/// The resolution of the thresholding (1 - 255) as 255 being the most precise.
	static constexpr uint kHistogramBars			= $(histogram_bars);

	/// The agression of the threshold (ideal ~ 0.999)
	static constexpr decimal kThresholdTolerance	= $(threshold_tolerance);



	/// The maximum amount of stars to observe from the image.
	static constexpr uint kMaxPoints				= $(max_points);

	/// The maximum amount of unique triangles... to find matches from.
	static constexpr uint kMaxSets					= $(max_sets);

	/// The maximum amount of elements to have from the database.
	static constexpr uint kMaxMatches				= $(max_matches);

	/// The maximum number of matches to use per star.
	/// This is important as if there are lots of matches for a single star and it exceeds kMaxMatches,
	/// If the star is a false positive, it will throw all the results off.
	/// Lowering the max number of matches per star will reduce this error.
	static constexpr uint kMaxMatchesPerStar		= $(max_matches_per_star);



	/// When finding elements in the database, the difference from the actual value.
	/// The range of the database will be (0 - FOV).
	static constexpr uint kDistanceTolerance		= $(distance_tolerance);

	/// This must be >= the width of the image.
	static constexpr uint kImageWidth				= $(image_width);
	/// This must be >= the height of the image.
	static constexpr uint kImageHeight				= $(image_height);




	/// The allowed difference in area from the database to be valid.
	static constexpr decimal kToleranceArea			= $(tolerance_area);
	/// The allowed difference in moment from the database to be valid.
	static constexpr decimal kToleranceMoment		= $(tolerance_moment);
};
}
