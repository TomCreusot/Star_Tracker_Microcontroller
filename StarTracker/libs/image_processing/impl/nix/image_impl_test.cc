#include "gtest/gtest.h"
#include "image_impl.hpp"




TEST ( DefaultConstructor, GetWidthHeight )
{
	ip::Image image;
	EXPECT_EQ(image.getWidth(), 1);
	EXPECT_EQ(image.getHeight(), 1);
}

TEST ( AlternateConstructor, GetWidthHeight )
{
	ip::Image image(10, 20);
	ip::Image image2(10, 20);
	EXPECT_EQ(image.getWidth(), 10);
	EXPECT_EQ(image.getHeight(), 20);
}


TEST ( CopyConstructor, FromDefault )
{
	ip::Image im1;
	im1.setPixel(0, 0, 1);
	ip::Image im2 (im1);
	EXPECT_EQ(im2.getWidth(), 1);
	EXPECT_EQ(im2.getHeight(), 1);
	EXPECT_EQ(im2.getPixel(0, 0), 1);
}


TEST ( CopyConstructor, FromAlternate )
{
	ip::Image im1(2, 2);
	im1.setPixel(0, 0, 1);
	im1.setPixel(0, 1, 2);
	im1.setPixel(1, 0, 3);
	im1.setPixel(1, 1, 4);
	ip::Image im2 (im1);
	EXPECT_EQ(im2.getWidth(), 2);
	EXPECT_EQ(im2.getHeight(), 2);
	EXPECT_EQ(im2.getPixel(0, 0), im1.getPixel(0, 0));
	EXPECT_EQ(im2.getPixel(0, 1), im1.getPixel(0, 1));
	EXPECT_EQ(im2.getPixel(1, 0), im1.getPixel(1, 0));
	EXPECT_EQ(im2.getPixel(1, 1), im1.getPixel(1, 1));
	EXPECT_TRUE(&im1 != &im2);
}
