#include <iostream>
#include "gtest/gtest.h"

#include "libs/util/util.h"
#include "libs/util/array_list.h"
#include "libs/util/point.h"
#include "image.h"
#include "blob.h"

using namespace util;
using namespace image_processing;

// For this namespace to function as expected, these functions should be tested in order:
//		- Generate Histogram
//		- PercentThreshold
//		- FindBlobs

//	It is assumed that kImageWidth and kImageHeight is bigger than 10x10.

/// @return	A completely black image.
Image CreateImageBlank ( )
{
	Image img(10, 10);
	return img;
}

/// @return	An image with 50 intensity corners.
Image CreateImageCorners ( )
{
	Image img(10, 10);
	img.SetPixel(0, 0, 50);
	img.SetPixel(9, 0, 50);
	img.SetPixel(0, 9, 50);
	img.SetPixel(9, 9, 50);
	return img;
}

/// @return	A 10 intensity image with a teselating varying intensity pixel.
Image CreateImageDots ( )
{
	Image img(10, 10);

	for ( uint y = 1; y < img.GetHeight(); y += 2 )
		for ( uint x = 1; x < img.GetWidth(); x += 2 )
		{
			img.SetPixel(x, y, x * 20 + y * 3 );	// y is odd to stop overlapping.
		}

	return img;
}


/// @return	An image with 2 shapes with an off center intensity.
///	The background is 10 intensity.
///	They should have an origin of and (0.75, 0.75), (2.75, 2.75)
///	 	0	1	2	3	4
///		====================
///	 0 | -	50	-	-	-	|
///	 1 | 50	100	-	50	-	|
///	 2 | -	-	-	50	-	|
///	 3 | -	50	50	100	50	|
///	 4 | -	-	-	50	-	|
///		====================
Image CreateImageIntensityVariation ( )
{
	Image img(5, 5);

	for ( uint x = 0; x < img.GetWidth(); x++ )
		for ( uint y = 0; y < img.GetHeight(); y++ )
			img.SetPixel(x, y, 10);

	// Top Shape
	img.SetPixel(1, 0, 50);
	img.SetPixel(0, 1, 50);
	img.SetPixel(1, 1, 100);

	// Bottom Shape
	img.SetPixel(3, 1, 50);
	img.SetPixel(3, 2, 50);
	img.SetPixel(1, 3, 50);
	img.SetPixel(2, 3, 50);
	img.SetPixel(3, 3, 100);
	img.SetPixel(4, 3, 50);
	img.SetPixel(3, 4, 50);
	return img;
}


/// @brief Prints out an image.
/// @param [in] img	The image to print.
void PrintImage ( Image& img )
{
	cout << endl << "\t";
	for ( uint x = 0; x < img.GetWidth(); x++ )
		cout << x << ",\t";

	cout << endl << "  ";

	for ( uint x = 0; x < img.GetWidth(); x++ )
		cout << "________";

	cout << endl;

	for ( uint y = 0; y < img.GetHeight(); y++ )
	{
		cout << y << " |\t";
		for ( uint x = 0; x < img.GetWidth(); x++ )
		{
			if ( img.GetPixel(x, y) > 0 )
				cout << (int)img.GetPixel(x, y) << ",\t";
			else
				cout << " \t";
		}
		cout << endl;
	}

	cout << endl;
}


























///	Tests an empty image, checks if an image with no stars will not register.
///	The histogram will be only 0.
/// The threshold will be 0.
///	# of points = 0.
TEST ( ImageProcessing, Blank )
{
	Image img = CreateImageBlank();
	PrintImage(img);
	// Histogram Testing.
	const uint HISTOGRAM_SIZE = 255;
	ArrayList<uint, HISTOGRAM_SIZE> histogram(HISTOGRAM_SIZE);
	img.GenerateHistogram<HISTOGRAM_SIZE>(&histogram);
	EXPECT_EQ(histogram.Get(0), 100);

	// Percent Threshold Testing.
	decimal threshold_aggression = 0;
	byte threshold =
		img.PercentThreshold<HISTOGRAM_SIZE>(threshold_aggression, histogram);
	EXPECT_EQ(threshold, 0);

	// Blob Detection Testing.
	const uint LIST_SIZE = 10;
	const uint BLOB_SIZE = 110;
	ArrayList<Blob, LIST_SIZE> blobs;
	Blob::FindBlobs<LIST_SIZE, BLOB_SIZE>(threshold, &img, &blobs);
	EXPECT_EQ(blobs.Size(), 0);
}


///	Tests an image with stars on the exact corner.
///	The histogram will have 96 [0], 4 [50].
/// The threshold will be 50 - 1
///	# of points = 4.
TEST ( ImageProcessing, Corners )
{
	Image img = CreateImageCorners();
	PrintImage(img);
	// Histogram Testing.
	const uint HISTOGRAM_SIZE = 255;
	ArrayList<uint, HISTOGRAM_SIZE> histogram(HISTOGRAM_SIZE);
	img.GenerateHistogram<HISTOGRAM_SIZE>(&histogram);
	EXPECT_EQ(histogram.Get(0), 96);
	EXPECT_EQ(histogram.Get(49), 4);

	// Percent Threshold Testing.
	decimal threshold_aggression = 1.0001;
	byte threshold =
		img.PercentThreshold<HISTOGRAM_SIZE>(threshold_aggression, histogram);
	EXPECT_EQ((int)threshold, 49);

	threshold_aggression = 0.99;
	threshold =
		img.PercentThreshold<HISTOGRAM_SIZE>(threshold_aggression, histogram);
	EXPECT_EQ((int)threshold, 49);

	// Blob Detection Testing.
	const uint LIST_SIZE = 100;
	const uint BLOB_SIZE = 110;
	ArrayList<Blob, LIST_SIZE> blobs;
	Blob::FindBlobs<LIST_SIZE, BLOB_SIZE>(threshold, &img, &blobs);
	EXPECT_EQ(blobs.Size(), 4);
}



///	Tests an image with too many stars, it should only choose the brightest.
///	The histogram will have 96 [0], 4 [50].
/// The threshold will be 50 - 1
///	# of points = 4.
TEST ( ImageProcessing, Dots )
{
	Image img = CreateImageDots();
	PrintImage(img);
	// Histogram Testing.
	const uint HISTOGRAM_SIZE = 255;
	ArrayList<uint, HISTOGRAM_SIZE> histogram(HISTOGRAM_SIZE);
	img.GenerateHistogram<HISTOGRAM_SIZE>(&histogram);
	EXPECT_EQ(histogram.Get(0), 75); // the rest are stars

	// Percent Threshold Testing.
	decimal threshold_aggression = 1;
	byte threshold =
		img.PercentThreshold<HISTOGRAM_SIZE>(threshold_aggression, histogram);
	EXPECT_EQ((int)threshold, 206); // the bar bellow the brightest pixel.

	threshold_aggression = 0;
	threshold =
		img.PercentThreshold<HISTOGRAM_SIZE>(threshold_aggression, histogram);
	EXPECT_EQ((int)threshold, 0);

	// Blob Detection Testing.
	const uint LIST_SIZE = 20; // Limit the size
	const uint BLOB_SIZE = 110;
	ArrayList<Blob, LIST_SIZE> blobs;
	Blob::FindBlobs<LIST_SIZE, BLOB_SIZE>(threshold, &img, &blobs);
	EXPECT_EQ(blobs.Size(), LIST_SIZE);


	blobs.Sort(&Blob::SortByIntensity);
	uint prev = blobs.Get(0).intensity;
	for ( uint i = 1; i < blobs.Size(); i++ )
	{
		uint intense = blobs.Get(i).intensity;
		EXPECT_TRUE( intense < prev );
	}
	// The brightest is 9, 9
	EXPECT_FLOAT_EQ(blobs.Get(0).boundsMin.x, 9);
	EXPECT_FLOAT_EQ(blobs.Get(0).boundsMin.y, 9);
}




///	Tests an image with too many stars, it should only choose the brightest.
///	The histogram will have 96 [0], 4 [50].
/// The threshold will be 50 - 1
///	# of points = 4.
TEST ( ImageProcessing, IntensityVariation )
{
	Image img = CreateImageIntensityVariation();
	PrintImage(img);
	// Histogram Testing.
	const uint HISTOGRAM_SIZE = 255;
	ArrayList<uint, HISTOGRAM_SIZE> histogram(HISTOGRAM_SIZE);
	img.GenerateHistogram<HISTOGRAM_SIZE>(&histogram);
	EXPECT_EQ(histogram.Get(9), 15); // the rest are stars

	// Percent Threshold Testing.
	decimal threshold_aggression = 1;
	byte threshold =
		img.PercentThreshold<HISTOGRAM_SIZE>(threshold_aggression, histogram);
	EXPECT_EQ((int)threshold, 99); // the brightest pixel - 1

	threshold_aggression = 0.0;
	threshold =
		img.PercentThreshold<HISTOGRAM_SIZE>(threshold_aggression, histogram);
	EXPECT_EQ((int)threshold, 0);	// The dullest pixel

	threshold_aggression = 0.70;
	threshold =
		img.PercentThreshold<HISTOGRAM_SIZE>(threshold_aggression, histogram);
	EXPECT_EQ((int)threshold, 49);	// The foreground is 50 and 100

	// Blob Detection Testing.
	const uint LIST_SIZE = 20; // Limit the size
	const uint BLOB_SIZE = 110;
	ArrayList<Blob, LIST_SIZE> blobs;
	Blob::FindBlobs<LIST_SIZE, BLOB_SIZE>(threshold, &img, &blobs);

	// Bottom Right (more intensity)
	EXPECT_FLOAT_EQ(blobs.Get(0).centroid.x, 2.75);
	EXPECT_FLOAT_EQ(blobs.Get(0).centroid.y, 2.75);
	// Top Left (Less intensity)
	EXPECT_FLOAT_EQ(blobs.Get(1).centroid.x, 0.75);
	EXPECT_FLOAT_EQ(blobs.Get(1).centroid.y, 0.75);
}
