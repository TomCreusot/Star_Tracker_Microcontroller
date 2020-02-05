#include "gtest/gtest.h"
#include "image_impl.hpp"

using namespace ip;
TEST ( DefaultConstructor, Standard )
{
	ip::Image image;
	for ( int xx = 0; xx < IMAGE_WIDTH; xx++ )
		for ( int yy = 0; yy < IMAGE_HEIGHT; yy++ )
			EXPECT_EQ(image.getPixel(xx, yy), 0);
}

TEST ( CopyConstructor, Standard )
{
	ip::Image im1;
	for ( int xx = 0; xx < IMAGE_WIDTH; xx++ )
		for ( int yy = 0; yy < IMAGE_HEIGHT; yy++ )
			im1.setPixel(xx, yy, xx + yy);

	ip::Image im2 (im1);

	for ( int xx = 0; xx < IMAGE_WIDTH; xx++ )
		for ( int yy = 0; yy < IMAGE_HEIGHT; yy++ )
			EXPECT_EQ(im2.getPixel(xx, yy), im1.getPixel(xx, yy));
}


TEST ( GetSetPixel, Standard )
{
	ip::Image img;
	img.setPixel(IMAGE_WIDTH / 2, IMAGE_HEIGHT / 2, 231);
	EXPECT_EQ(img.getPixel(IMAGE_WIDTH / 2, IMAGE_HEIGHT / 2), 231);
}



TEST ( GetWidthHeight, Standard )
{
	ip::Image img;
	EXPECT_EQ(img.getWidth(), IMAGE_WIDTH);
	EXPECT_EQ(img.getHeight(), IMAGE_HEIGHT);
}




TEST ( ValidPixel, Inside )
{
	ip::Image image;
	EXPECT_TRUE(image.validPixel(0, 0));
	EXPECT_TRUE(image.validPixel(0, IMAGE_HEIGHT - 1));
	EXPECT_TRUE(image.validPixel(IMAGE_WIDTH - 1, 0));
	EXPECT_TRUE(image.validPixel(IMAGE_WIDTH - 1, IMAGE_HEIGHT - 1));
	EXPECT_TRUE(image.validPixel(IMAGE_WIDTH / 2, IMAGE_HEIGHT / 2));
}


TEST ( ValidPixel, Outside )
{
	ip::Image image;
	EXPECT_FALSE(image.validPixel(0, IMAGE_HEIGHT));
	EXPECT_FALSE(image.validPixel(IMAGE_WIDTH, 0));
	EXPECT_FALSE(image.validPixel(IMAGE_WIDTH, IMAGE_HEIGHT));
}
