#include "gtest/gtest.h"
#include "blob.h"

using namespace image_processing;

//////////////////////////////////////////////////////////////////////////////
//																			//
//								Constructors							//
//																			//
//////////////////////////////////////////////////////////////////////////////


TEST ( DefaultConstructor, Standard )
{
	image_processing::Blob blob;
	EXPECT_FLOAT_EQ 	( blob.centroid.x, 0);
	EXPECT_FLOAT_EQ 	( blob.centroid.y, 0);
	EXPECT_EQ 			( blob.pixels, 0);
	EXPECT_EQ 			( blob.intensity, 0);
	EXPECT_EQ 			( blob.boundsMin.x, 0);
	EXPECT_EQ 			( blob.boundsMin.y, 0);
	EXPECT_EQ 			( blob.boundsMax.x, 0);
	EXPECT_EQ 			( blob.boundsMax.y, 0);
}


TEST ( AlternateConstructor, Standard )
{
	image_processing::Blob blob ( 1, 2 );

	EXPECT_FLOAT_EQ 	( blob.centroid.x, 1);
	EXPECT_FLOAT_EQ 	( blob.centroid.y, 2);
	EXPECT_EQ 			( blob.pixels, 0);
	EXPECT_EQ 			( blob.intensity, 0);
	EXPECT_EQ 			( blob.boundsMin.x, 1);
	EXPECT_EQ 			( blob.boundsMin.y, 2);
	EXPECT_EQ 			( blob.boundsMax.x, 1);
	EXPECT_EQ 			( blob.boundsMax.y, 2);
}



//////////////////////////////////////////////////////////////////////////////
//																			//
//								Find Blobs									//
//																			//
//////////////////////////////////////////////////////////////////////////////

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


TEST ( FindBlobs, ExceedsList )
{
	const int size_list = 10;
	const int size_blob = 100;
	image_processing::Image img(100, 100);
	img.SetPixel(0, 0, 1);
	img.SetPixel(2, 0, 2);
	img.SetPixel(4, 0, 3);
	img.SetPixel(6, 0, 4);
	img.SetPixel(8, 0, 5);
	img.SetPixel(10, 0, 6);
	img.SetPixel(12, 0, 7);
	img.SetPixel(14, 0, 8);
	img.SetPixel(16, 0, 9);
	img.SetPixel(18, 0, 10);
	img.SetPixel(20, 0, 11);

	ArrayList<Blob, size_list> blobs;
	Blob::FindBlobs<size_list, size_blob>(0, &img, &blobs);
	EXPECT_EQ(blobs.Get(0).intensity, 11);
	EXPECT_EQ(blobs.Get(1).intensity, 10);
	EXPECT_EQ(blobs.Get(2).intensity, 9);
	EXPECT_EQ(blobs.Get(3).intensity, 8);
	EXPECT_EQ(blobs.Get(4).intensity, 7);
	EXPECT_EQ(blobs.Get(5).intensity, 6);
	EXPECT_EQ(blobs.Get(6).intensity, 5);
	EXPECT_EQ(blobs.Get(7).intensity, 4);
	EXPECT_EQ(blobs.Get(8).intensity, 3);
	EXPECT_EQ(blobs.Get(9).intensity, 2);

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





//////////////////////////////////////////////////////////////////////////////
//																			//
//								GrassFire									//
//																			//
//////////////////////////////////////////////////////////////////////////////
// The grass fire method slowly spreads out over a set of pixels above the threshold.
// Currently it is set to have no diagonals as stars usualy dont.

TEST ( SpreadGrassFire, Valid )
{
	//		|	10	|	11	|	12	|
	//		------------------------
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

	EXPECT_EQ(a.pixels, 6);//7);
	EXPECT_EQ(a.intensity, 21);//28);
	EXPECT_EQ(a.boundsMin.x, 10);
	EXPECT_EQ(a.boundsMin.y, 10);
	EXPECT_EQ(a.boundsMax.x, 12);
	EXPECT_EQ(a.boundsMax.y, 13);//14);
	// EXPECT_EQ(a.width, 3);
	// EXPECT_EQ(a.height, 4);//5);
	EXPECT_FLOAT_EQ(a.centroid.x, 11.1904761904762);//11.14285714);
	EXPECT_FLOAT_EQ(a.centroid.y, 12);//12.5);
}




TEST ( SpreadGrassFire, SinglePixel )
{
	image_processing::Blob a(12, 20);
	Image img(200, 200);
	img.SetPixel(12, 20, 2);

	a.SpreadGrassFire<100>(1, &img);

	EXPECT_EQ(a.pixels, 1);
	EXPECT_EQ(a.intensity, 2);
	EXPECT_EQ(a.boundsMin.x, 12);
	EXPECT_EQ(a.boundsMin.y, 20);
	EXPECT_EQ(a.boundsMax.x, 12);
	EXPECT_EQ(a.boundsMax.y, 20);
	EXPECT_FLOAT_EQ(a.centroid.x, 12);
	EXPECT_FLOAT_EQ(a.centroid.y, 20);
}



TEST ( FindNeighbours, Bounds )
{
	image_processing::Blob a(0, 0);
	Image img(1, 1);
	img.SetPixel(0, 0, 100);

	ArrayList<Point<uint>, 10> stack;
	Point<uint> pt;
	a.FindNeighbours<10>(0, pt, &img, &stack);
	EXPECT_EQ(stack.Size(), 0);
}

TEST ( FindNeighbours, Adjacent )
{
	image_processing::Blob a(1, 1);
	Image img(10, 10);
	img.SetPixel(1, 0, 100);
	img.SetPixel(0, 1, 100);
	img.SetPixel(2, 1, 100);
	img.SetPixel(1, 2, 100);

	ArrayList<Point<uint>, 10> stack;
	Point<uint> pt(1, 1);
	a.FindNeighbours<10>(1, pt, &img, &stack);
	EXPECT_EQ(stack.Get(0).x, 2);
	EXPECT_EQ(stack.Get(0).y, 1);

	EXPECT_EQ(stack.Get(1).x, 0);
	EXPECT_EQ(stack.Get(1).y, 1);

	EXPECT_EQ(stack.Get(2).x, 1);
	EXPECT_EQ(stack.Get(2).y, 2);

	EXPECT_EQ(stack.Get(3).x, 1);
	EXPECT_EQ(stack.Get(3).y, 0);
	EXPECT_EQ(stack.Size(), 4);
}




TEST ( ConsumePixel, Valid )
{
	image_processing::Blob a(1, 1);
	Image img(10, 10);

	Point<uint> pt(1, 2);
	img.SetPixel(pt.x, pt.y, 20);

	a.intensity = 10;
	a.pixels = 1;

	a.ConsumePixel(pt, &img);
	EXPECT_EQ(img.GetPixel(pt.x, pt.y), 0);
	EXPECT_EQ(a.boundsMin.x, 1);
	EXPECT_EQ(a.boundsMin.y, 1);
	EXPECT_EQ(a.boundsMax.x, 1);
	EXPECT_EQ(a.boundsMax.y, 2);
	EXPECT_EQ(a.intensity, 30);
	EXPECT_EQ(a.pixels, 2);
	EXPECT_FLOAT_EQ(a.centroid.x, 1);

	decimal centroid = ((decimal)pt.y * 20.0 + 10.0) / ((decimal)a.intensity);
	EXPECT_FLOAT_EQ(a.centroid.y, centroid);
}






//////////////////////////////////////////////////////////////////////////////
//																			//
//								Find Centroid								//
//																			//
//////////////////////////////////////////////////////////////////////////////
// This is a way of searching for recalculating moments to find the true center of a blob.

TEST ( FindCentroid, Valid )
{
	// Value:		5 	6	0	2	1	1	2	0	6
	// Position:	0	7	8	9	10	11	12	13	14
	EXPECT_FLOAT_EQ(Blob::FindCentroid(10, 1, 11, 1),		10.5);
	EXPECT_FLOAT_EQ(Blob::FindCentroid(10.5, 2, 9, 2),		9.75);
	EXPECT_FLOAT_EQ(Blob::FindCentroid(9.75, 4, 12, 2),		10.5);
	EXPECT_FLOAT_EQ(Blob::FindCentroid(10.5, 6, 7, 6),		8.75);
	EXPECT_FLOAT_EQ(Blob::FindCentroid(8.75, 12, 14, 6),	10.5);
	EXPECT_FLOAT_EQ(Blob::FindCentroid(10.5, 18, 0, 5),		8.217391304);
}







//////////////////////////////////////////////////////////////////////////////
//																			//
//								ToPointList									//
//																			//
//////////////////////////////////////////////////////////////////////////////
// To reduce coupling and memory, blobs should be converted to Points as the rest is not useful.

TEST ( ToPointList, Valid )
{
	const uint LIST_SIZE = 4;
	ArrayList<Blob, LIST_SIZE> blobs;
	ArrayList<Point<decimal>, LIST_SIZE> points;

	blobs.PushBack(Blob(0, 0));
	blobs.PushBack(Blob(1, 1));
	blobs.PushBack(Blob(2, 2));
	blobs.PushBack(Blob(3, 3));

	Blob::ToPointList<LIST_SIZE>(blobs, &points);

	for ( uint i = 0; i < blobs.Size(); i++ )
	{
		EXPECT_FLOAT_EQ(points.Get(i).x, i);
		EXPECT_FLOAT_EQ(points.Get(i).y, i);
	}
}


//////////////////////////////////////////////////////////////////////////////
//																			//
//								SortByIntensity								//
//																			//
//////////////////////////////////////////////////////////////////////////////
// Function for the ArrayList to sort brightest first.

TEST ( SortByIntensity, Valid )
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



TEST ( SortByIntensityAscending, Valid )
{
	image_processing::Blob b1;
	image_processing::Blob b2;

	b1.intensity = 10;
	b2.intensity = 1;

	EXPECT_FALSE(image_processing::Blob::SortByIntensityAscending(b1, b2));
	EXPECT_TRUE(image_processing::Blob::SortByIntensityAscending(b2, b1));

	b2.intensity = 10;
	EXPECT_TRUE(image_processing::Blob::SortByIntensity(b1, b2));
}
