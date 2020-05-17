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
	EXPECT_FLOAT_EQ 	( blob.centroid.x, 0);
	EXPECT_FLOAT_EQ 	( blob.centroid.y, 0);
	EXPECT_EQ 			( blob.pixels, 0);
	EXPECT_EQ 			( blob.intensity, 0);
	EXPECT_EQ 			( blob.origin.x, 0);
	EXPECT_EQ 			( blob.origin.y, 0);
}


TEST ( AlternateConstructor, Standard )
{
	image_processing::Blob blob ( 1, 2 );

	EXPECT_FLOAT_EQ 	( blob.centroid.x, 1);
	EXPECT_FLOAT_EQ 	( blob.centroid.y, 2);
	EXPECT_EQ 			( blob.pixels, 0);
	EXPECT_EQ 			( blob.intensity, 0);
	EXPECT_EQ 			( blob.origin.x, 1);
	EXPECT_EQ 			( blob.origin.y, 2);
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
	img.SetPixel(12, 10, 1);
	img.SetPixel(12, 11, 2);
	img.SetPixel(12, 12, 3);
	img.SetPixel(12, 13, 4);
	img.SetPixel(11, 12, 5);
	img.SetPixel(10, 12, 6);
	img.SetPixel(11, 14, 7);

	a.SpreadGrassFire<100>(0, &img);

	EXPECT_EQ(a.pixels, 7);
	EXPECT_EQ(a.intensity, 28);
	EXPECT_EQ(a.width, 3);
	EXPECT_EQ(a.height, 5);
	EXPECT_FLOAT_EQ(a.centroid.x, 11.14285714);
	EXPECT_FLOAT_EQ(a.centroid.y, 12.5);
}




TEST ( SpreadGrassFire, OnePixelNix)
{
	image_processing::Blob a(12, 20);
	Image img(200, 200);
	img.SetPixel(12, 20, 2);

	a.SpreadGrassFire<100>(1, &img);

	EXPECT_EQ(a.pixels, 1);
	EXPECT_EQ(a.intensity, 2);
	EXPECT_EQ(a.width, 1);
	EXPECT_EQ(a.height, 1);
	EXPECT_FLOAT_EQ(a.centroid.x, 12);
	EXPECT_FLOAT_EQ(a.centroid.y, 20);
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
	img.SetPixel(12, 10, 1);
	img.SetPixel(12, 11, 2);
	img.SetPixel(12, 12, 3);
	img.SetPixel(12, 13, 4);
	img.SetPixel(11, 12, 5);
	img.SetPixel(10, 12, 6);
	img.SetPixel(11, 10, 7);

	a.SpreadGrassFire<100>(1, &img);

	EXPECT_EQ(a.pixels, 6);
	EXPECT_EQ(a.intensity, 27);
	EXPECT_EQ(a.width, 3);
	EXPECT_EQ(a.height, 4);
	EXPECT_FLOAT_EQ(a.centroid.x, 11.111111111);
	EXPECT_FLOAT_EQ(a.centroid.y, 11.555555556);

}



TEST ( SpreadGrassFire, InvalidIntensity )
{
	image_processing::Blob a(0, 0);
	Image img(10, 10);

	a.SpreadGrassFire<100>(10, &img);

	EXPECT_NEAR(a.centroid.x, 0, 0.01);
	EXPECT_NEAR(a.centroid.y, 0, 0.01);
	EXPECT_EQ(a.width, 0);
	EXPECT_EQ(a.height, 0);
	EXPECT_EQ(a.pixels, 0);
	EXPECT_EQ(a.intensity, 0);
}









/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|													|
|			------- FindCentroid	-------			|
|													|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

TEST ( FindCentroid, Standard )
{
	// Value:		5 	6	0	2	1	1	2	0	6
	// Position:	0	7	8	9	10	11	12	13	14
	image_processing::Blob a(0, 0);
	EXPECT_FLOAT_EQ(a.FindCentroid(10, 1, 11, 1), 10.5);
	EXPECT_FLOAT_EQ(a.FindCentroid(10.5, 2, 9, 2), 9.75);
	EXPECT_FLOAT_EQ(a.FindCentroid(9.75, 4, 12, 2), 10.5);
	EXPECT_FLOAT_EQ(a.FindCentroid(10.5, 6, 7, 6), 8.75);
	EXPECT_FLOAT_EQ(a.FindCentroid(8.75, 12, 14, 6), 10.5);
	EXPECT_FLOAT_EQ(a.FindCentroid(10.5, 18, 0, 5), 8.217391304);
}






/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|													|
|			------- FindBlobs	-------				|
|													|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

TEST ( FindBlobs, Valid )
{
	const int size_list = 10;
	const int size_blob = 100;
	image_processing::Image img(100, 100);
	ArrayList<Blob, size_list> blobs;
	Blob::FindBlobs<size_list, size_blob>(0, &img, &blobs);
	EXPECT_EQ(blobs.Size(), 0);

	img.SetPixel(0, 0, 1);
	blobs = ArrayList<Blob, size_list>();
	Blob::FindBlobs<size_list, size_blob>(0, &img, &blobs);
	EXPECT_EQ(blobs.Size(), 1);

	img.SetPixel(99, 99, 2);
	img.SetPixel(99, 2, 2);
	blobs = ArrayList<Blob, size_list>();
	Blob::FindBlobs<size_list, size_blob>(0, &img, &blobs);
	EXPECT_EQ(blobs.Size(), 2);

	blobs = ArrayList<Blob, size_list>();
	Blob::FindBlobs<size_list, size_blob>(1, &img, &blobs);
	EXPECT_EQ(blobs.Size(), 0);
}



TEST ( FindBlobs, MaxThreshold)
{
	const int size_list = 10;
	const int size_blob = 10;
	image_processing::Image img(100, 100);
	ArrayList<Blob, size_list> blobs;

	img.SetPixel(0, 0, 255);
	Blob::FindBlobs<size_list, size_blob>(255, &img, &blobs);
	EXPECT_EQ(blobs.Size(), 0);
}



TEST ( FindBlobs, Bounds )
{
	const int size_list = 10;
	const int size_blob = 10;
	image_processing::Image img(100, 200);
	ArrayList<Blob, size_list> blobs;
	img.SetPixel(0, 0, 255);
	img.SetPixel(0, 199, 255);
	img.SetPixel(99, 0, 255);
	img.SetPixel(99, 199, 255);

	Blob::FindBlobs<size_list, size_blob>(254, &img, &blobs);
	EXPECT_EQ(blobs.Size(), 4);
}





/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
|													|
|		------- SortByIntensity	-------				|
|													|
\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

TEST ( SortByIntensity, All )
{
	image_processing::Blob b1;
	image_processing::Blob b2;

	b1.intensity = 10;
	b2.intensity = 1;

	EXPECT_TRUE(image_processing::Blob::SortByIntensity(b1, b2));
	EXPECT_FALSE(image_processing::Blob::SortByIntensity(b2, b1));

	b2.intensity = 10;
	EXPECT_TRUE(image_processing::Blob::SortByIntensity(b1, b2));
}
