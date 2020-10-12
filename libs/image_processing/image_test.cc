#include <iostream>
#include "gtest/gtest.h"
#include "image.h"

#include "config/runtime.h"

using namespace std;
using namespace image_processing;

//////////////////////////////////////////////////////////////////////////////
//																			//
//							Constructors									//
//																			//
//////////////////////////////////////////////////////////////////////////////
// All the pixels should be set to zero on construction just in case the program is not setup properly.

TEST ( DefaultConstructor, Valid )
{
	image_processing::Image image;
	EXPECT_EQ(image.GetWidth(), 0);
	EXPECT_EQ(image.GetHeight(), 0);
}



TEST ( AlternateConstructor, Valid )
{
	image_processing::Image image(3, 3);
	EXPECT_EQ(image.GetWidth(), 3);
	EXPECT_EQ(image.GetHeight(), 3);

	EXPECT_EQ(image.GetPixel(0, 0), 0);
	EXPECT_EQ(image.GetPixel(0, 1), 0);
	EXPECT_EQ(image.GetPixel(0, 2), 0);

	EXPECT_EQ(image.GetPixel(1, 0), 0);
	EXPECT_EQ(image.GetPixel(1, 1), 0);
	EXPECT_EQ(image.GetPixel(1, 2), 0);

	EXPECT_EQ(image.GetPixel(2, 0), 0);
	EXPECT_EQ(image.GetPixel(2, 1), 0);
	EXPECT_EQ(image.GetPixel(2, 2), 0);
}



TEST ( CopyConstructor_SetPixel, Valid )
{
	image_processing::Image image(5, 10);
	image.SetPixel(2, 3, 23);
	image.SetPixel(3, 3, 33);
	image.SetPixel(3, 4, 34);
	image_processing::Image img (image);
	EXPECT_EQ(image.GetPixel(2, 3), img.GetPixel(2, 3));
	EXPECT_EQ(image.GetPixel(3, 3), img.GetPixel(3, 3));
	EXPECT_EQ(image.GetPixel(3, 4), img.GetPixel(3, 4));
	EXPECT_EQ(image.GetPixel(0, 0), img.GetPixel(0, 0));
}


//////////////////////////////////////////////////////////////////////////////
//																			//
//								Accessors									//
//																			//
//////////////////////////////////////////////////////////////////////////////

TEST ( GetMaxWidthHeight, Valid )
{
	EXPECT_TRUE(Image::MaxWidth() == config::image_width  );
	EXPECT_TRUE(Image::MaxHeight() == config::image_height );
}


TEST ( SetWidthHeight, Valid )
{
	image_processing::Image image(2, 2);
	image.SetWidthHeight(0, 10);
	EXPECT_EQ(image.GetWidth(), 0);
	EXPECT_EQ(image.GetHeight(), 10);

	image.SetWidthHeight(	image_processing::Image::MaxWidth(),
							image_processing::Image::MaxHeight()	);
	EXPECT_EQ(image.GetWidth(), image_processing::Image::MaxWidth());
	EXPECT_EQ(image.GetHeight(), image_processing::Image::MaxHeight());
}


TEST ( SetWidthHeight, Invalid )
{
	image_processing::Image image(2, 2);
	image.SetWidthHeight(0, image_processing::Image::MaxHeight() + 1);
	EXPECT_EQ(image.GetWidth(), 2);
	EXPECT_EQ(image.GetHeight(), 2);
	image.SetWidthHeight(image_processing::Image::MaxWidth() + 1, 0);
	EXPECT_EQ(image.GetWidth(), 2);
	EXPECT_EQ(image.GetHeight(), 2);
	image.SetWidthHeight(	image_processing::Image::MaxWidth() + 1,
							image_processing::Image::MaxHeight() + 1);
	EXPECT_EQ(image.GetWidth(), 2);
	EXPECT_EQ(image.GetHeight(), 2);
}





TEST ( ValidPixel, Inside )
{
	image_processing::Image img(10, 5);
	EXPECT_TRUE(img.ValidPixel(0, 					0));
	EXPECT_TRUE(img.ValidPixel(0, 					img.GetHeight()-1));
	EXPECT_TRUE(img.ValidPixel(img.GetWidth()-1, 	0));
	EXPECT_TRUE(img.ValidPixel(9, 					img.GetHeight()-1));
	EXPECT_TRUE(img.ValidPixel(img.GetWidth()/2,	img.GetHeight()/2));
}


TEST ( ValidPixel, Outside )
{
	image_processing::Image img(10, 5);
	EXPECT_FALSE(img.ValidPixel(0, img.GetHeight()));
	EXPECT_FALSE(img.ValidPixel(img.GetWidth(), 0));
	EXPECT_FALSE(img.ValidPixel(0, img.GetHeight()));
	EXPECT_FALSE(img.ValidPixel(img.GetWidth(), img.GetHeight()));
}


//////////////////////////////////////////////////////////////////////////////
//																			//
//								FindMinMax									//
//																			//
//////////////////////////////////////////////////////////////////////////////
// Finds the brightest and dullest pixels in the specified area.
// This is required for adaptive thresholding.

// Checks the general functionality.
TEST ( FindMinMax, Valid )
{
	image_processing::Image img(3, 2);
	byte min, max;
	img.SetPixel(0, 0, 1);
	img.FindMinMax(1, 1, 1, &min, &max);
	EXPECT_EQ(min, 0);
	EXPECT_EQ(max, 1);

	img.SetPixel(1, 0, 2);
	img.FindMinMax(1, 1, 1, &min, &max);
	EXPECT_EQ(min, 0);
	EXPECT_EQ(max, 2);

	img.SetPixel(2, 0, 3);
	img.FindMinMax(1, 1, 1, &min, &max);
	EXPECT_EQ(min, 0);
	EXPECT_EQ(max, 3);

	img.SetPixel(0, 1, 4);
	img.FindMinMax(1, 1, 1, &min, &max);
	EXPECT_EQ(min, 0);
	EXPECT_EQ(max, 4);

	img.SetPixel(1, 1, 5);
	img.FindMinMax(1, 1, 1, &min, &max);
	EXPECT_EQ(min, 0);
	EXPECT_EQ(max, 5);

	img.SetPixel(2, 1, 6);	// The whole image
	img.FindMinMax(1, 1, 1, &min, &max);
	EXPECT_EQ(min, 1);
	EXPECT_EQ(max, 6);

	img.SetPixel(0, 0, 7);	// Overriding the min
	img.FindMinMax(1, 1, 1, &min, &max);
	EXPECT_EQ(min, 2);
	EXPECT_EQ(max, 7);
}


// Creates an area larger than the bounds, the image should not read outside the bounds.
TEST ( FindMinMax, SampleAboveBounds )
{
	image_processing::Image img;
	byte min, max;
	img.SetPixel(0, 0, 22);
	img.FindMinMax(0, 0, img.GetWidth() + img.GetHeight(), &min, &max);
	EXPECT_EQ(min, 22);
	EXPECT_EQ(max, 22);
}


// Checks if the function does not exceed the edges of the image and reads the edges.
TEST ( FindMinMax, Bounds )
{
	image_processing::Image im1(1, 2);
	byte min, max;

	im1.SetPixel(0, 0, 100);
	im1.SetPixel(0, 1, 200);
	im1.FindMinMax(0, 0, 5, &min, &max);
	EXPECT_EQ(min, 100);
	EXPECT_EQ(max, 200);

	image_processing::Image im2(7, 8);
	for ( uint x = 0; x < im2.GetWidth(); x++ )
		for ( uint y = 0; y < im2.GetHeight(); y++ )
			im2.SetPixel(x, y, 20);

	im2.SetPixel(0, 0, 9);
	im2.SetPixel(0, 1, 8);
	im2.SetPixel(1, 0, 7);
	im2.SetPixel(6, 6, 6);
	im2.SetPixel(6, 0, 5);
	im2.SetPixel(0, 6, 4);

	im2.SetPixel(2, 5, 10);
	im2.SetPixel(2, 1, 100);
	im2.FindMinMax(3, 3, 2, &min, &max);
	EXPECT_EQ(min, 10);
	EXPECT_EQ(max, 100);


}

//////////////////////////////////////////////////////////////////////////////
//																			//
//							AdaptiveThreshold								//
//																			//
//////////////////////////////////////////////////////////////////////////////
// Adaptive Thresholding is where an area of the image is compared and if the pixel observed is in that range.
// If the pixel is < average, it should be removed as background.
// This will consume the image.

// With full aggression, no pixels should be valid.
TEST ( AdaptiveThreshold, Aggression_100_Percent )
{
	image_processing::Image img(5, 5);
	img.SetPixel(0, 0, 255);
	img.SetPixel(0, 1, 254);
	img.SetPixel(4, 4, 1);
	img.SetPixel(4, 3, 2);
	img.AdaptiveThreshold(1, 1);
	EXPECT_EQ(img.GetPixel(0, 0), 255);
	EXPECT_EQ(img.GetPixel(0, 1), 0);
	EXPECT_EQ(img.GetPixel(4, 3), 2);
	EXPECT_EQ(img.GetPixel(4, 4), 0);
}



// With 50% aggression, only pixels above the average will be valid.
TEST ( AdaptiveThreshold, Aggression_50_Percent )
{
	image_processing::Image img(2, 2);
	img.SetPixel(0, 0, 255);
	img.SetPixel(1, 0, 100);
	img.SetPixel(0, 1, 50);
	img.SetPixel(1, 1, 255);
	img.AdaptiveThreshold(2, 0.5);	// avg = 165, anything bellow is invalid.
	EXPECT_EQ(img.GetPixel(0, 0), 255);
	EXPECT_EQ(img.GetPixel(1, 0), 0);
	EXPECT_EQ(img.GetPixel(0, 1), 0);
	EXPECT_EQ(img.GetPixel(1, 1), 255);

	img.SetPixel(1, 0, 187);
	img.SetPixel(0, 1, 50);
	img.AdaptiveThreshold(2, 0.5);	// avg = 186, anything <= is invalid.
	EXPECT_EQ(img.GetPixel(0, 0), 255);
	EXPECT_EQ(img.GetPixel(1, 0), 187);
	EXPECT_EQ(img.GetPixel(0, 1), 0);
	EXPECT_EQ(img.GetPixel(1, 1), 255);
}


// With 0% aggression, all pixels will be valid.
TEST ( AdaptiveThreshold, Aggression_0_Percent )
{
	image_processing::Image img(2, 1);
	img.SetPixel(0, 0, 255);
	img.SetPixel(1, 0, 1);
	img.AdaptiveThreshold(1, 0);
	EXPECT_EQ(img.GetPixel(0, 0), 255);
	EXPECT_EQ(img.GetPixel(1, 0), 1);
}



// If the sample radius > image size, the sample should just be smaller.
TEST ( AdaptiveThreshold, SampleRadius_Greater_ImageSize )
{
	image_processing::Image img(1, 1);
	img.SetPixel(0, 0, 255);
	img.AdaptiveThreshold(img.GetWidth() + img.GetHeight(), 0);
	EXPECT_EQ(img.GetPixel(0, 0), 255);
}





//////////////////////////////////////////////////////////////////////////////
//																			//
//							PercentThreshold								//
//																			//
//////////////////////////////////////////////////////////////////////////////
//
// This should return a number representing the boundary of background and foreground noise.
// This uses a histogram of pixels with an intensity.
// For each bar, there is a percentage of the sum of all pixels darker then it.
// Using a percentage, you should be able to get a value of the cut off point between background and foreground.
// The returned number will be on the background side of the boundary.

// This tests when there is a smaller array ( < 255 ) if it will perform as expected.
TEST ( PercentThreshold, SmallArray )
{
	const int ARRAY_SIZE = 5;
	Image img(260, 1);

	util::ArrayList<util::uint, ARRAY_SIZE> v(ARRAY_SIZE);

					//	| Return Range	|	Sumed	|	% Input Range	|	Output	|
	v.Get(0) = 100;	//	| 0 - 51		|	100 	|	0 - 38			|	0		|
	v.Get(1) = 75;	//	| 51 - 102		|	175 	|	39 - 67 		|	51		|
	v.Get(2) = 50;	//	| 102 - 153		|	225 	|	68 - 86.5		|	102		|
	v.Get(3) = 25;	//	| 153 - 204		|	250 	|	87 - 96			|	153		|
	v.Get(4) = 10;	//	| 204 - 255		|	260 	|	97 - 100		|	204		|

	EXPECT_EQ(img.PercentThreshold<ARRAY_SIZE>(0.0,		v), 0);
	EXPECT_EQ(img.PercentThreshold<ARRAY_SIZE>(0.39,	v), 51);
	EXPECT_EQ(img.PercentThreshold<ARRAY_SIZE>(0.68,	v), 102);
	EXPECT_EQ(img.PercentThreshold<ARRAY_SIZE>(0.87,	v), 153);
	EXPECT_EQ(img.PercentThreshold<ARRAY_SIZE>(0.97,	v), 204);
}


// This tests an array of size 255, the expected size.
TEST ( PercentThreshold, ByteSizedArray )
{
	const int ARRAY_SIZE = 255;
	// A sequence of 255 down to 0 sums up to 32385
	// This cannot be stored with a standard image size, ()
	Image img(255, 1);

	util::ArrayList<util::uint, ARRAY_SIZE> v(ARRAY_SIZE);


	for ( uint i = 0; i < v.Size(); i++ )
		v.Get(i) = 1;


	EXPECT_EQ(img.PercentThreshold<ARRAY_SIZE>(0.0,		v), 0);
	EXPECT_EQ(img.PercentThreshold<ARRAY_SIZE>(0.5,		v), 127);
	EXPECT_EQ(img.PercentThreshold<ARRAY_SIZE>(1,		v), 254);
}



//////////////////////////////////////////////////////////////////////////////
//																			//
//							GenerateHistogram								//
//																			//
//////////////////////////////////////////////////////////////////////////////
// This generates a histogram of the number of pixels in each intensity.
// At index 0, the intensity should be 0.
// At index 255, the intensity should be 255 (MAX).
// If the image is black, at [0], the value should be image.GetWidth() * image.GetHeight().


// This is to test when there is only a single slot in the array, everything should fit in it.
TEST ( GenerateHistogram, OneElement )
{
	Image img(100, 100);
	img.SetPixel(0, 0, 10);
	img.SetPixel(0, 50, 30);
	img.SetPixel(0, 50, 66);
	const uint HISTOGRAM_SIZE = 1;
	util::ArrayList<uint, HISTOGRAM_SIZE> v(HISTOGRAM_SIZE);

	img.GenerateHistogram<HISTOGRAM_SIZE>(&v);

	EXPECT_EQ(v.Get(0), img.GetHeight() * img.GetWidth());
}

// This has a [ < 50%] and a [ > 50% ] slot, the pixels > 50% should be in [1].
TEST ( GenerateHistogram, TwoElements )
{
	Image img(100, 100);
	img.SetPixel(0, 0, 129);
	img.SetPixel(0, 50, 128);
	img.SetPixel(0, 50, 127);
	const uint HISTOGRAM_SIZE = 2;
	util::ArrayList<uint, HISTOGRAM_SIZE> v(HISTOGRAM_SIZE);

	img.GenerateHistogram<HISTOGRAM_SIZE>(&v);

	EXPECT_EQ(v.Get(0), img.GetHeight() * img.GetWidth() - 1);
	EXPECT_EQ(v.Get(1), 1);
}

// This tests an array of
TEST ( GenerateHistogram, AllElements )
{
	Image img(100, 100);
	img.SetPixel(0, 0, 255);
	img.SetPixel(0, 6, 254);
	img.SetPixel(0, 60, 2);
	img.SetPixel(0, 50, 1);
	const uint HISTOGRAM_SIZE = 256;
	util::ArrayList<uint, HISTOGRAM_SIZE> v(HISTOGRAM_SIZE);

	img.GenerateHistogram<HISTOGRAM_SIZE>(&v);


	EXPECT_EQ(v.Get(0), img.GetHeight() * img.GetWidth() - 4);
	EXPECT_EQ(v.Get(1), 1);
	EXPECT_EQ(v.Get(2), 1);
	EXPECT_EQ(v.Get(254), 1);
	EXPECT_EQ(v.Get(255), 1);
	EXPECT_EQ(v.Get(253), 0);
}
