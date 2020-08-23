/*
 *	File: 		demo.cc
 *	Author:		Tom Creusot
 *  Purpose:
 *				To visualy show the workings of the software.
 */

#include <iostream>
#include <chrono> // Get Time

#include "libs/image_processing/image.h"
#include "libs/image_processing/blob.h"
#include "libs/star_tracker/star_set.h"
#include "libs/star_tracker/database_array.h"
#include "libs/util/util.h"
#include "libs/util/point.h"
#include "libs/util/array_list.h"
#include "libs/nix/get_image.h"
#include "libs/nix/config.h"

#include "libs/properties/properties.h"

using namespace chrono;

using namespace util;
using namespace image_processing;
using namespace star_tracker;
using namespace std;
using namespace nix;

/**
 * @brief 	Gets the timestamp since epoch for comparisons.
 * @return	The time.
 */
inline milliseconds GetTimeStamp ( )
{
	return duration_cast<milliseconds>(system_clock::now().time_since_epoch());
}


int main ( int argc, char** argv )
{
	if ( argc == 2 )
	{
	/**********************************************************************************************************
	* Purpose: 	Reads a properties file describing the image (PLATFORM DEPENDANCE, unix).
	* Namespace: 	nix
	*/
		// Gets the properties of the image, camera and thresholds.
		nix::Config config;
		config.ReadFile(argv[1]);
		string IMAGE_FILE 				= config.GetString("image_file");
		string IMAGE_OUT_FILE		 	= config.GetString("image_out_file");
		util::uint MAX_DISPLAY			= config.GetInteger("max_display");

		// Get Duration
		milliseconds start_time = GetTimeStamp();


	/**********************************************************************************************************
	* Purpose: 	Reads an image from a file (PLATFORM DEPENDANCE, unix).
	* Namespace: 	nix
	* The image must be obtainted.
	*/

		image_processing::Image img;		// This is the image class which works in the program.
		nix::GetImage get(IMAGE_FILE);		// This is an external library which reads bitmaps.
		get.GetBMP(&img);					// Copies bitmap to img so the program can read it.
		// Scope, this stuff is only relevant here.
		{
			cout 	<< IMAGE_FILE << endl
					<< "size: (" << img.GetWidth() << " x " << img.GetHeight()<<")"
					<< endl << "pixels: " << img.GetWidth()*img.GetHeight() << endl;
		}


	/**********************************************************************************************************
	 * Purpose:		Generates a threshold.
	 * Namespace: 	image_processing
	 */
	 	// The brighness of the first valid pixel.
		util::uint threshold;

		// Scope, this stuff is only relevant here.
		{
			// Required Properties
			constexpr util::uint NUM_BARS 	= Properties::kHistogramBars;
			constexpr util::uint THRESHOLD 	= Properties::kThresholdTolerance;

			// It takes a number from 1-255 being the number of sample brightnesses.
			// The larger the number, the more memory it uses but more accurate.
			util::ArrayList<util::uint, NUM_BARS> histogram;
			img.GenerateHistogram<NUM_BARS>(&histogram);

			// The percentage threshold removes all values bellow a specific percentage depending on their brightness.
			// It returns the first valid pixel brightness above 0.
			threshold = img.PercentThreshold<NUM_BARS>(THRESHOLD, histogram);
		}



	/**********************************************************************************************************
	 * Purpose:		Finds Points.
	 * Namespace: 	image_processing
	 * The points must be found for all the stars and their size.
	 * This uses a "grassfire blob detection method" where it consumes the image.
	 * It then is converted to a point array list to save space.
	 */

		// The maximum number of stars to observe in the image.
		const util::uint MAX_STARS		= Properties::kMaxPoints;

		// It is no longer useful to store the details of the blobs.
		// This converts them to points only.
		util::ArrayList<util::Point<util::decimal>, MAX_STARS> points;

		// Scope, this stuff is only relevant here.
		{
			// The maximum number of pixels a star can occupy in the image.
			const util::uint MAX_STAR_SIZE	= 20;

			// This finds the "BLOBS" from the image.
			// This object contains a brightness and position of the center.
			util::ArrayList<image_processing::Blob, MAX_STARS> blobs;
			image_processing::Blob::FindBlobs<MAX_STARS, MAX_STAR_SIZE>
													(threshold, &img, &blobs);

			// This sorts the blobs in order of intencity.
			// Blob::sortByIntensity is a function pointer to sort blobs in this way.
			blobs.Sort(Blob::SortByIntensity);

			// Inserts the points from the blobs into the array list points.
			Blob::ToPointList(blobs, &points);
		}




	/**********************************************************************************************************
	 * Purpose: 	Finds unique identifiers from that section of the sky.
	 *				This includes a modified 'pyramid method'
	 *				The results are put into an AngleStat object per set.
	 * Namespace:	star_tracker
	 */

		// The maximum number of sets of stars to use.
		const util::uint MAX_SETS = properties::Properties::kMaxSets;

		// Setup the database.
		// This is important for getting the field of view, pixel resolution and the database to compare the stars to.
		decimal imgHyp = std::hypot(img.GetWidth(), img.GetHeight());
		decimal fov = star_tracker::database_array::fov;
		decimal rad_per_pixel = fov / imgHyp;
		star_tracker::Database database(fov, rad_per_pixel,
										&star_tracker::database_array::array );

		// All the triangles found from the image.
		util::ArrayList<star_tracker::StarSet, MAX_SETS> triangles;

		// This allows you to get unique details about points in this part of the sky.
		// Use this to compare with a database.
		star_tracker::StarSet::GenerateSets<MAX_STARS, MAX_SETS>
				(	points,
					0, points.Size(), rad_per_pixel,
					&star_tracker::StarSet::CartesianAngle, &triangles	);


	/**********************************************************************************************************
	 * Purpose:		Compares the results from star_tracker and finds the most likely match.
	 * Namespace: 	star_tracker
	 */

		const uint MAX_MATCHES		= properties::Properties::kMaxMatches;
		const uint MAX_MATCHES_STAR =properties::Properties::kMaxMatchesPerStar;
		const uint TOLERANCE_AREA	= properties::Properties::kToleranceArea;
		const uint TOLERANCE_MOMENT = properties::Properties::kToleranceMoment;

		// The elements will be found, these are all the elements within the threshold.
		util::ArrayList<star_tracker::StarSet, MAX_MATCHES> database_angles;
		database.FindElements<MAX_SETS, MAX_MATCHES>
				(	triangles, TOLERANCE_AREA, TOLERANCE_MOMENT,
					MAX_MATCHES_STAR, &database_angles);

		// The elements are then compared with each other.
		// If the distance between the stars are > fov, their likelyhood drops.
		star_tracker::StarSet::Vote<MAX_MATCHES> ( fov, &database_angles );

		// They are then sorted in order of probability.
		database_angles.Sort(star_tracker::StarSet::SortByVoteDecending);










		//////////////////////////////////////
		//	\/		POST PROCESSING		\/	//
		//////////////////////////////////////


		// Output
		cout << database_angles.Size() << " matches:" << endl;
		for ( uint i = 0; i < MAX_DISPLAY && i < database_angles.Size(); i++ )
		{
			star_tracker::StarSet set = database_angles.Get(i);
			cout<< "\t Opposite: " << set.position.x << ", " << set.position.y
				<< "\t\t Odds:" << set.vote << endl;
		}
		cout << (GetTimeStamp().count() - start_time.count())
												<< " ms to execute." << endl;

		// Draws where the blobs were detected.
		get.DrawPoints<MAX_STARS>(points, 255, 0, 0);
		get.WriteImage(IMAGE_OUT_FILE);
	}
	else
	{
		cout << "ERROR, Please Enter the name of the properties file" << endl;
	}
}
