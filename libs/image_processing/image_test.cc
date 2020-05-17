#include <iostream>
#include "gtest/gtest.h"
#include "image.h"

using namespace std;
using namespace image_processing;


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




TEST ( SetWidthHeight, Valid )
{
	image_processing::Image image(2, 2);
	image.SetWidthHeight(0, 10);
	EXPECT_EQ(image.GetWidth(), 0);
	EXPECT_EQ(image.GetHeight(), 10);

	image.SetWidthHeight(image_processing::kImageWidthMax,
											image_processing::kImageHeightMax);
	EXPECT_EQ(image.GetWidth(), image_processing::kImageWidthMax);
	EXPECT_EQ(image.GetHeight(), image_processing::kImageHeightMax);
}


TEST ( SetWidthHeight, Invalid )
{
	image_processing::Image image(2, 2);
	image.SetWidthHeight(0, image_processing::kImageHeightMax + 1);
	EXPECT_EQ(image.GetWidth(), 2);
	EXPECT_EQ(image.GetHeight(), 2);
	image.SetWidthHeight(image_processing::kImageWidthMax + 1, 0);
	EXPECT_EQ(image.GetWidth(), 2);
	EXPECT_EQ(image.GetHeight(), 2);
	image.SetWidthHeight(image_processing::kImageWidthMax + 1,
									image_processing::kImageHeightMax + 1);
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



TEST ( FindMinMax, SampleAboveBounds )
{
	image_processing::Image img;
	byte min, max;
	img.SetPixel(0, 0, 22);
	img.FindMinMax(0, 0, img.GetWidth() + img.GetHeight(), &min, &max);
	EXPECT_EQ(min, 22);
	EXPECT_EQ(max, 22);
}



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


//
TEST ( PercentThreshold, TwoElementHistogram )
{
	const int array_size = 255;
	Image img(3, 3);

	util::ArrayList<util::uint, array_size> v(array_size);

	for ( uint i = 0; i < v.Size(); i++ ) v.Get(i) = 0;

	v.Get(0) = img.GetWidth() * img.GetHeight() / 2;
	v.Get(100) = img.GetWidth() * img.GetHeight() - v.Get(0);

	EXPECT_EQ(img.PercentThreshold<array_size>(0.0, v), 0);
	EXPECT_EQ(img.PercentThreshold<array_size>(0.5, v), 0);
	EXPECT_EQ(img.PercentThreshold<array_size>(1.0, v), 100);
}


TEST ( PercentThreshold, MaxStarIncluded )
{
	const int array_size = 255;
	Image img(2, 2);

	util::ArrayList<util::uint, array_size> v(array_size);
	for ( uint i = 0; i < v.Size(); i++ ) v.Get(i) = 0;

	v.Get(0) = 1;
	v.Get(127) = 1;
	v.Get(253) = 1;
	v.Get(254) = 1;

	EXPECT_EQ(img.PercentThreshold<array_size>(0.0, v), 0);
	EXPECT_EQ(img.PercentThreshold<array_size>(0.5, v), 127);
	EXPECT_EQ(img.PercentThreshold<array_size>(1.0, v), 254);
}







TEST ( GenerateHistogram, OneElement )
{
	Image img(100, 100);
	img.SetPixel(0, 0, 10);
	img.SetPixel(0, 50, 30);
	img.SetPixel(0, 50, 66);
	util::ArrayList<uint, 1> v(1);

	img.GenerateHistogram<1>(&v);

	EXPECT_EQ(v.Get(0), img.GetHeight() * img.GetWidth());
}

TEST ( GenerateHistogram, TwoElements )
{
	Image img(100, 100);
	img.SetPixel(0, 0, 129);
	img.SetPixel(0, 50, 128);
	img.SetPixel(0, 50, 127);
	util::ArrayList<uint, 2> v(2);

	img.GenerateHistogram<2>(&v);

	EXPECT_EQ(v.Get(0), img.GetHeight() * img.GetWidth() - 1);
	EXPECT_EQ(v.Get(1), 1);
}

TEST ( GenerateHistogram, AllElements )
{
	Image img(100, 100);
	img.SetPixel(0, 0, 255);
	img.SetPixel(0, 6, 254);
	img.SetPixel(0, 60, 2);
	img.SetPixel(0, 50, 1);
	util::ArrayList<uint, 256> v(256);

	img.GenerateHistogram<256>(&v);

	EXPECT_EQ(v.Get(0), img.GetHeight() * img.GetWidth() - 4);
	EXPECT_EQ(v.Get(1), 1);
	EXPECT_EQ(v.Get(2), 1);
	EXPECT_EQ(v.Get(254), 1);
	EXPECT_EQ(v.Get(255), 1);
	EXPECT_EQ(v.Get(253), 0);
}
