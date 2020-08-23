/**
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
	static constexpr uint kHistogramBars			= 255;

	/// The agression of the threshold (ideal ~ 0.999)
	static constexpr decimal kThresholdTolerance	= 0.999;



	/// The maximum amount of stars to observe from the image.
	static constexpr uint kMaxPoints				= 20;

	/// The maximum amount of unique triangles... to find matches from.
	static constexpr uint kMaxSets					= 100;

	/// The maximum amount of elements to have from the database.
	static constexpr uint kMaxMatches				= 30;

	/// The maximum number of matches to use per star.
	/// This is important as if there are lots of matches for a single star and it exceeds kMaxMatches,
	/// If the star is a false positive, it will throw all the results off.
	/// Lowering the max number of matches per star will reduce this error.
	static constexpr uint kMaxMatchesPerStar		= 10;



	/// When finding elements in the database, the difference from the actual value.
	/// The range of the database will be (0 - FOV).
	static constexpr uint kDistanceTolerance		= 0.5;

	/// This must be >= the width of the image.
	static constexpr uint kImageWidth				= 2000;
	/// This must be >= the height of the image.
	static constexpr uint kImageHeight				= 1081;




	/// The allowed difference in area from the database to be valid.
	static constexpr decimal kToleranceArea			= 0.0001;
	/// The allowed difference in moment from the database to be valid.
	static constexpr decimal kToleranceMoment		= 0.000001;
};
}
