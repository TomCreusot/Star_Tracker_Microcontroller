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
#include "libs/nix/properties.h"

using namespace util;
using namespace image_processing;
using namespace star_tracker;
using namespace std;
using namespace nix;


int main ( int argc, char** argv )
{


	if ( argc == 2 )
	{
		// Gets the properties of the image, camera and thresholds.
		nix::Properties p;
		p.ReadFile(argv[1]);
		string IMAGE_FILE 				= p.GetString("image_file");
		string IMAGE_OUT_FILE		 	= p.GetString("image_out_file");

		int HISTOGRAM_BARS 					= p.GetInteger("histogram_bars");
		util::decimal THRESHOLD_TOLERANCE	= p.GetDecimal("threshold_tolerance");

		util::uint MAX_POINTS 				= p.GetInteger("max_points");
		util::uint MAX_SETS					= p.GetInteger("max_sets");
		util::uint MAX_DISPLAY				= p.GetInteger("max_display");

		util::decimal FIND_ANGLE_TOLERANCE 	= p.GetDecimal("find_angle_tolerance");
		util::decimal FIND_DISTANCE_TOLERANCE= p.GetDecimal("find_distance_tolerance");

		util::decimal FOV 					= p.GetDecimal("fov");

		/**********************************************************************************************************
		 * Purpose: 	Reads an image from a file (PLATFORM DEPENDANCE)
		 * Namespace: 	get_image
		 * The image must be obtainted.
		 */

		nix::GetImage get(IMAGE_FILE);
		image_processing::Image img;				// An image object contains the pixel data of an image and is dynamicaly resizable.
		get.GetBMP(&img);							// Copies bitmap to img.

		// Some useful details
		cout 	<< IMAGE_FILE << endl
				<< "Image size: " << img.GetWidth() << "\t" << img.GetHeight()
				<< ", pixels: " << img.GetWidth() * img.GetHeight() << endl;

		// Shows the effects of increaseContrast and adaptiveThreshold.
		// int threshold_area = 10;
		// img.AdaptiveThreshold(threshold_area, threshold_tolerance)
		// get.CopyToBMP(img);




		/**********************************************************************************************************
		 * Purpose:		Generates a threshold.
		 * Namespace: 	image_processing
		 */

		// It takes a number from 1-255 being the number of sample brightnesses.
		// The larger the number, the more memory it uses but more accurate.
		util::ArrayList<util::uint, 255> histogram(HISTOGRAM_BARS);
		img.GenerateHistogram<255>(&histogram);

		// The percentage threshold removes all values bellow a specific percentage depending on their brightness.
		// It returns the first valid pixel brightness above 0.
		util::uint threshold =  img.PercentThreshold<255>(THRESHOLD_TOLERANCE, histogram);


		/**********************************************************************************************************
		 * Purpose:		Finds Points.
		 * Namespace: 	image_processing
		 * The points must be found for all the stars and their size.
		 * This uses a "grassfire blob detection method" where it consumes the image.
		 */

		const util::uint max_stars = 1000;		// The max amount of stars in the image.
		const util::uint max_star_size = 1000; 	// The max number of pixels.

		// This finds the "BLOBS" from the image.
		// This object contains a brightness and position of the center.
		util::ArrayList<image_processing::Blob, max_stars> blobs;
		image_processing::Blob::FindBlobs<max_stars, max_star_size>(threshold, &img, &blobs);

		// This sorts the blobs in order of intencity.
		// Blob::sortByIntensity is a function pointer to sort blobs in this way.
		blobs.Sort(Blob::SortByIntensity);

		// It is no longer useful to store the details of the blobs.
		// This converts them to points only.
		util::ArrayList<util::Point<util::decimal>, max_stars> points;
		Blob::ToPointList(blobs, &points);




		/**********************************************************************************************************
		 * You may want to reduce the number of points to view or for computation time.
		 */

		util::uint size = (points.Size() > MAX_POINTS ? MAX_POINTS : points.Size());
		cout << "Found: " << points.Size() << " points, Using: " << size << endl;
		points.ReduceSize(size);


		/**********************************************************************************************************
		 * Purpose: 	Finds unique identifiers from that section of the sky.
		 *				This includes a modified 'pyramid method'
		 *				The results are put into an AngleStat object per set.
		 * Namespace:	star_tracker
		 */

		const util::uint max_input_sets = 1000;
		util::ArrayList<star_tracker::StarSet, max_input_sets> angles;
		// This allows you to get unique details about points in this part of the sky.
		// Use this to compare with a database.
		star_tracker::StarSet::GenerateSetsPilots<max_stars, max_input_sets>(points, 0, size, &angles);



		/**********************************************************************************************************
		 * You may want to reduce the number of points to view or for computation time.
		 */

		size = (angles.Size() > MAX_SETS ? MAX_SETS : angles.Size());
		cout << "Found: " << angles.Size() << " angles, Using: " << size << endl;
		angles.ReduceSize(size);



		/**********************************************************************************************************
		 * Purpose:		Compares the results from star_tracker and finds the most likely match.
		 * Namespace: 	database
		 */

		const util::uint max_output_sets = 1000;

		star_tracker::StarSet::set_pixel_resolution(hypot(img.GetHeight(), img.GetWidth()));
		star_tracker::StarSet::set_fov(FOV);

		// The elements will be found, these are all the elements within the threshold.
		util::ArrayList<star_tracker::StarSet, max_output_sets> database_angles;
		star_tracker::StarSet::FindElements<max_input_sets, max_output_sets>(star_tracker::database_array, star_tracker::database_size, angles, FIND_ANGLE_TOLERANCE, FIND_DISTANCE_TOLERANCE, &database_angles);

		// The elements are then compared with each other.
		// If the distance between the stars are > fov, their likelyhood drops.
		star_tracker::StarSet::ClusterProbability<max_output_sets>(&database_angles);

		// They are then sorted in order of probability.
		database_angles.Sort(star_tracker::StarSet::SortByOdds);










		//////////////////////////////////////
		//	\/		POST PROCESSING		\/	//
		//////////////////////////////////////

		// Draws where the blobs were detected.
		get.DrawPoints<max_stars>(points, 255, 0, 0);
		get.WriteImage(IMAGE_OUT_FILE);

		// Output
		cout << database_angles.Size() << " matches:" << endl;
		for ( uint i = 0; i < MAX_DISPLAY && i < database_angles.Size(); i++ )
		{
			star_tracker::StarSet set = database_angles.Get(i);
			cout 	<< "\t Opposite: " << set.opposite.x << ", " << set.opposite.y
					<< "\t\t Odds:" << set.odds << endl;
		}

	}
	else
	{
		cout << "ERROR, Please Enter the name of the properties file" << endl;
	}
}
