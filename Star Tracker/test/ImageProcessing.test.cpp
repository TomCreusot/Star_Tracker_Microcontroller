#include <iostream>
#include <gtest/gtest.h>

#include "../src/ImageProcessing/ImageProcessing.hpp"

#include "../src/EasyBMP/EasyBMP.h"
using namespace std;
using namespace ip;

int main( int argc, char** argv )
{
	testing::InitGoogleTest(&argc, argv);

	return RUN_ALL_TESTS();
}


/**
 * Tests the histograms general functionality
 *
 *
 */
bool GenerateHistogramTest_General ()
{
	bool valid = true;
	ip::byte img1[10] = {0,		1,	2,	3,	4,	5,	6,	7,	8,	9};
	ip::byte img2[10] = {10,	11,	12,	13,	14,	15,	16,	17,	17,	255};

	ip::byte** img;
	img[0] = img1;
	img[1] = img2;

	 uint* val = generateHistogram(img, 10, 2, 255);

	 // 255 should be 1
	 valid &= val[254] == 1;
	 // 17 should be 2
	 valid &= val[17] == 2;
	 // Should be 1.
	 for ( int i = 0; i < 17; i++ )
	 {
		 valid &= val[i] == 1;
	 }
	 // Should be 0.
	 for ( int i = 18; i < 255; i++ )
	 {
		 valid &= val[i] == 0;
	 }
}


/**
 * Tests generateHistogram().
 * Has a 2d array containing all values bellow 19 and a value of 255.
 * All values < 19 & > 254 should be 1 while everything else 0.
 *

TEST ( GenerateHistogramTest, GeneralFunctionality )
{
	bool valid = true;
	ip::byte img1[10] = {0,		1,	2,	3,	4,	5,	6,	7,	8,	9};
	ip::byte img2[10] = {10,	11,	12,	13,	14,	15,	16,	17,	18,	255};

	ip::byte** img;
	img[0] = img1;
	img[1] = img2;

	 uint* val = generateHistogram(img, 10, 2, 255);

	 // 255 should be 1
	 valid &= val[254] == 1;
	 // Should be 1.
	 for ( int i = 0; i < 19; i++ )
	 {
		 valid &= val[i] == 1;
	 }
	 // Should be 0.
	 for ( int i = 20; i < 255; i++ )
	 {
		 valid &= val[i] == 0;
	 }
	 EXPECT_TRUE(valid);
}
*/
