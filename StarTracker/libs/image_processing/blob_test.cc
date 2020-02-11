#include "gtest/gtest.h"
#include "blob.h"

using namespace image_processing;

/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|													|
|			------ Constructors	------				|
|													|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/


TEST ( DefaultConstructor, Standard )
{
	image_processing::Blob blob;
	EXPECT_FLOAT_EQ 	( blob.getCentroidX(), 0 );
	EXPECT_FLOAT_EQ 	( blob.getCentroidY(), 0 );
	EXPECT_EQ 			( blob.getPixels(), 0 );
	EXPECT_EQ 			( blob.getIntensity(), 0 );
	EXPECT_EQ 			( blob.getOriginX(), 0 );
	EXPECT_EQ 			( blob.getOriginY(), 0 );
}


TEST ( AlternateConstructor, Standard )
{
	image_processing::Blob blob ( 1, 2 );
	EXPECT_FLOAT_EQ 	( blob.getCentroidX(), 1 );
	EXPECT_FLOAT_EQ 	( blob.getCentroidY(), 2 );
	EXPECT_EQ 			( blob.getPixels(), 0 );
	EXPECT_EQ 			( blob.getIntensity(), 0 );
	EXPECT_EQ 			( blob.getOriginX(), 1 );
	EXPECT_EQ 			( blob.getOriginY(), 2 );
}





/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|													|
|		------- SpreadGrassFire	-------				|
|													|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/


TEST ( SpreadGrassFire, StandardNix )
{
	//		|	10	|	11	|	12	|
	//		-------------------------
	//	10	|		|		|	1	|
	//	11	|		|		|	2	|
	//	12	|	6	|	5	|	3	|
	//	13	|		|		|	4	|
	//	14	|		|	7	|		|

	image_processing::Blob a(12, 10);
	Image img(100, 100);
	img.setPixel(12, 10, 1);
	img.setPixel(12, 11, 2);
	img.setPixel(12, 12, 3);
	img.setPixel(12, 13, 4);
	img.setPixel(11, 12, 5);
	img.setPixel(10, 12, 6);
	img.setPixel(11, 14, 7);

	a.spreadGrassFire(img, 0);

	EXPECT_EQ(a.getPixels(), 7);
	EXPECT_EQ(a.getIntensity(), 28);
	EXPECT_EQ(a.getWidth(), 3);
	EXPECT_EQ(a.getHeight(), 5);
	EXPECT_FLOAT_EQ(a.getCentroidX(), 11.14285714);
	EXPECT_FLOAT_EQ(a.getCentroidY(), 12.5);
}




TEST ( SpreadGrassFire, OnePixelNix)
{
	image_processing::Blob a(12, 20);
	Image img(200, 200);
	img.setPixel(12, 20, 2);
	a.spreadGrassFire(img, 1);
	EXPECT_EQ(a.getPixels(), 1);
	EXPECT_EQ(a.getIntensity(), 2);
	EXPECT_EQ(a.getWidth(), 1);
	EXPECT_EQ(a.getHeight(), 1);
	EXPECT_FLOAT_EQ(a.getCentroidX(), 12);
	EXPECT_FLOAT_EQ(a.getCentroidY(), 20);
}


TEST ( SpreadGrassFire, IntensityVariation )
{
	//		|	10	|	11	|	12	|
	//		-------------------------
	//	10	|		|	7	|	~1	|
	//	11	|		|		|	2	|
	//	12	|	6	|	5	|	3	|
	//	13	|		|		|	4	|
	image_processing::Blob a(11, 10);
	Image img(100, 100);
	img.setPixel(12, 10, 1);
	img.setPixel(12, 11, 2);
	img.setPixel(12, 12, 3);
	img.setPixel(12, 13, 4);
	img.setPixel(11, 12, 5);
	img.setPixel(10, 12, 6);
	img.setPixel(11, 10, 7);

	a.spreadGrassFire(img, 1);

	EXPECT_EQ(a.getPixels(), 6);
	EXPECT_EQ(a.getIntensity(), 27);
	EXPECT_EQ(a.getWidth(), 3);
	EXPECT_EQ(a.getHeight(), 4);
	EXPECT_FLOAT_EQ(a.getCentroidX(), 11.111111111);
	EXPECT_FLOAT_EQ(a.getCentroidY(), 11.555555556);

}



TEST ( SpreadGrassFire, InvalidIntensity )
{
	image_processing::Blob a(0, 0);
	Image img(10, 10);

	a.spreadGrassFire(img, 10);
	EXPECT_NEAR(a.getCentroidX(), 0, 0.01);
	EXPECT_NEAR(a.getCentroidY(), 0, 0.01);
	EXPECT_EQ(a.getWidth(), 0);
	EXPECT_EQ(a.getHeight(), 0);
	EXPECT_EQ(a.getPixels(), 0);
	EXPECT_EQ(a.getIntensity(), 0);
}


TEST ( FindCentroid, Standard )
{
	// Value:		5 	6	0	2	1	1	2	0	6
	// Position:	0	7	8	9	10	11	12	13	14
	image_processing::Blob a(0, 0);
	EXPECT_FLOAT_EQ(a.findCentroid(10, 1, 11, 1), 10.5);
	EXPECT_FLOAT_EQ(a.findCentroid(10.5, 2, 9, 2), 9.75);
	EXPECT_FLOAT_EQ(a.findCentroid(9.75, 4, 12, 2), 10.5);
	EXPECT_FLOAT_EQ(a.findCentroid(10.5, 6, 7, 6), 8.75);
	EXPECT_FLOAT_EQ(a.findCentroid(8.75, 12, 14, 6), 10.5);
	EXPECT_FLOAT_EQ(a.findCentroid(10.5, 18, 0, 5), 8.217391304);
}






/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|													|
|			------- FindBlobs	-------				|
|													|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

TEST ( FindBlobs, Valid )
{
	image_processing::Image img(100, 100);
	ArrayList<Blob> blobs;
	Blob::findBlobs(img, 0, blobs);
	EXPECT_EQ(blobs.size(), 0);

	img.setPixel(0, 0, 1);
	blobs = ArrayList<Blob>();
	Blob::findBlobs(img, 0, blobs);
	EXPECT_EQ(blobs.size(), 1);

	img.setPixel(99, 99, 2);
	img.setPixel(99, 2, 2);
	blobs = ArrayList<Blob>();
	Blob::findBlobs(img, 0, blobs);
	EXPECT_EQ(blobs.size(), 2);

	blobs = ArrayList<Blob>();
	Blob::findBlobs(img, 1, blobs);
	EXPECT_EQ(blobs.size(), 0);
}



TEST ( FindBlobs, MaxThreshold)
{
	image_processing::Image img(100, 100);
	ArrayList<Blob> blobs;

	img.setPixel(0, 0, 255);
	Blob::findBlobs(img, 255, blobs);
	EXPECT_EQ(blobs.size(), 0);
}



TEST ( FindBlobs, Bounds )
{
	image_processing::Image img(100, 200);
	ArrayList<Blob> blobs;
	img.setPixel(0, 0, 255);
	img.setPixel(0, 199, 255);
	img.setPixel(99, 0, 255);
	img.setPixel(99, 199, 255);

	Blob::findBlobs(img, 254, blobs);
	EXPECT_EQ(blobs.size(), 4);
}
