#include "gtest/gtest.h"
#include "image_base.hpp"
//#include "impl/nix/image_impl.hpp"
#include "image_impl.hpp"

using namespace ip;


TEST ( CopyConstructor, Standard )
{
	ip::Image image(5, 10);
	image.setPixel(2, 3, 23);
	image.setPixel(3, 3, 33);
	image.setPixel(3, 4, 34);
	ip::Image img (image);
	EXPECT_EQ(image.getPixel(2, 3), img.getPixel(2, 3));
	EXPECT_EQ(image.getPixel(3, 3), img.getPixel(3, 3));
	EXPECT_EQ(image.getPixel(3, 4), img.getPixel(3, 4));
	EXPECT_EQ(image.getPixel(0, 0), img.getPixel(0, 0));
}




TEST ( ValidPixel, NixInside )
{
	ip::Image img(10, 5);
	EXPECT_TRUE(img.validPixel(0, 					0));
	EXPECT_TRUE(img.validPixel(0, 					img.getHeight()-1));
	EXPECT_TRUE(img.validPixel(img.getWidth()-1, 	0));
	EXPECT_TRUE(img.validPixel(9, 					img.getHeight()-1));
	EXPECT_TRUE(img.validPixel(img.getWidth()/2,	img.getHeight()/2));
}


TEST ( ValidPixel, NixOutside )
{
	ip::Image img(10, 5);
	EXPECT_FALSE(img.validPixel(0, 				img.getHeight()));
	EXPECT_FALSE(img.validPixel(img.getWidth(), 0));
	EXPECT_FALSE(img.validPixel(img.getWidth(), img.getHeight()));
}





TEST ( FindMinMax, Standard )
{
	ip::ImageNix img(3, 2);
	byte min, max;
	img.setPixel(0, 0, 1);
	img.findMinMax(1, 1, 1, min, max);
	EXPECT_EQ(min, 0);
	EXPECT_EQ(max, 1);

	img.setPixel(1, 0, 2);
	img.findMinMax(1, 1, 1, min, max);
	EXPECT_EQ(min, 0);
	EXPECT_EQ(max, 2);

	img.setPixel(2, 0, 3);
	img.findMinMax(1, 1, 1, min, max);
	EXPECT_EQ(min, 0);
	EXPECT_EQ(max, 3);

	img.setPixel(0, 1, 4);
	img.findMinMax(1, 1, 1, min, max);
	EXPECT_EQ(min, 0);
	EXPECT_EQ(max, 4);

	img.setPixel(1, 1, 5);
	img.findMinMax(1, 1, 1, min, max);
	EXPECT_EQ(min, 0);
	EXPECT_EQ(max, 5);

	img.setPixel(2, 1, 6);	// The whole image
	img.findMinMax(1, 1, 1, min, max);
	EXPECT_EQ(min, 1);
	EXPECT_EQ(max, 6);

	img.setPixel(0, 0, 7);	// Overriding the min
	img.findMinMax(1, 1, 1, min, max);
	EXPECT_EQ(min, 2);
	EXPECT_EQ(max, 7);
}



TEST ( FindMinMax, SampleAboveBounds )
{
	ip::Image img;
	byte min, max;
	img.setPixel(0, 0, 22);
	img.findMinMax(0, 0, img.getWidth() + img.getHeight(), min, max);
	EXPECT_EQ(min, 22);
	EXPECT_EQ(max, 22);
}



TEST ( FindMinMax, Bounds )
{
	ip::Image im1(1, 2);
	byte min, max;

	im1.setPixel(0, 0, 100);
	im1.setPixel(0, 1, 200);
	im1.findMinMax(0, 0, 5, min, max);
	EXPECT_EQ(min, 100);
	EXPECT_EQ(max, 200);

	ip::Image im2(7, 8);
	for ( uint x = 0; x < im2.getWidth(); x++ )
		for ( uint y = 0; y < im2.getHeight(); y++ )
			im2.setPixel(x, y, 20);

	im2.setPixel(0, 0, 9);
	im2.setPixel(0, 1, 8);
	im2.setPixel(1, 0, 7);
	im2.setPixel(6, 6, 6);
	im2.setPixel(6, 0, 5);
	im2.setPixel(0, 6, 4);

	im2.setPixel(2, 5, 10);
	im2.setPixel(2, 1, 100);
	im2.findMinMax(3, 3, 2, min, max);
	EXPECT_EQ(min, 10);
	EXPECT_EQ(max, 100);


}




// With full aggression, no pixels should be valid.
TEST ( AdaptiveThreshold, Aggression_100_Percent )
{
	ip::Image img(5, 5);
	img.setPixel(0, 0, 255);
	img.setPixel(0, 1, 254);
	img.setPixel(4, 4, 1);
	img.setPixel(4, 3, 2);
	img.adaptiveThreshold(1, 1);
	EXPECT_EQ(img.getPixel(0, 0), 255);
	EXPECT_EQ(img.getPixel(0, 1), 0);
	EXPECT_EQ(img.getPixel(4, 3), 2);
	EXPECT_EQ(img.getPixel(4, 4), 0);
}



// With 50% aggression, only pixels above the average will be valid.
TEST ( AdaptiveThreshold, Aggression_50_Percent )
{
	ip::Image img(2, 2);
	img.setPixel(0, 0, 255);
	img.setPixel(1, 0, 100);
	img.setPixel(0, 1, 50);
	img.setPixel(1, 1, 255);
	img.adaptiveThreshold(2, 0.5);	// avg = 165, anything bellow is invalid.
	EXPECT_EQ(img.getPixel(0, 0), 255);
	EXPECT_EQ(img.getPixel(1, 0), 0);
	EXPECT_EQ(img.getPixel(0, 1), 0);
	EXPECT_EQ(img.getPixel(1, 1), 255);

	img.setPixel(1, 0, 187);
	img.setPixel(0, 1, 50);
	img.adaptiveThreshold(2, 0.5);	// avg = 186, anything <= is invalid.
	EXPECT_EQ(img.getPixel(0, 0), 255);
	EXPECT_EQ(img.getPixel(1, 0), 187);
	EXPECT_EQ(img.getPixel(0, 1), 0);
	EXPECT_EQ(img.getPixel(1, 1), 255);
}


// With 0% aggression, all pixels will be valid.
TEST ( AdaptiveThreshold, Aggression_0_Percent )
{
	ip::Image img(2, 1);
	img.setPixel(0, 0, 255);
	img.setPixel(1, 0, 1);
	img.adaptiveThreshold(1, 0);
	EXPECT_EQ(img.getPixel(0, 0), 255);
	EXPECT_EQ(img.getPixel(1, 0), 1);
}



// If the sample radius > image size, the sample should just be smaller.
TEST ( AdaptiveThreshold, SampleRadius_Greater_ImageSize )
{
	ip::Image img(1, 1);
	img.setPixel(0, 0, 255);
	img.adaptiveThreshold(img.getWidth() + img.getHeight(), 0);
	EXPECT_EQ(img.getPixel(0, 0), 255);
}
