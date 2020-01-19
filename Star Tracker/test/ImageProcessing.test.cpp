#include "UnitTest.hpp"
using namespace st;
using namespace ip;
using namespace db;


namespace ipT
{


bool run ( )
{
	bool valid = true;
	printHeader(
		"ImageProcessing.cpp, Blob.cpp, Point.cpp", "ImageProcessing.Test.cpp");

	valid &= printPass("Blob::withinThreshold", testBlobWithinThreshold());
	valid &= printPass("Blob::width/height/roughX/roughY",testBlobDimentions());
	valid &= printPass("Blob::spreadBlob", testBlobSpreadBlob());
	valid &= printPass("Blob::pixelExists", testBlobPixelExists());

	valid &= printPass("Point::set", testPointSet());
	valid &= printPass("Point::equal", testPointEqual());
	valid &= printPass("Point::distance", testPointDistance());
	return valid;
}








 /**
 * @brief Tests Blob.withinThreshold function.
 * @return True if all tests passed.
 */

 bool testBlobWithinThreshold ( )
 {
	 bool valid = true;
	 Blob a (0, 0);
	 // 0 Distance, 0 bounds, within.
	 valid &= a.withinThreshold(0, 0, 0);
	 a.minX = -2;
	 a.minY = -2;
	 // 0 Distance, 2, 2 bounds, on border.
	 valid &= a.withinThreshold(0, 0, 0);

	 // 0 Distance, 2, 2 bounds, off border.
	 valid &= !a.withinThreshold(0, 1, 0);

	 // 0 Distance, 2, 2 bounds, off border, within dist.
	 valid &= a.withinThreshold(0, 1, 1);

	 return valid;
 }



/**
* @brief Tests Blob.width(),height(),roughX(),roughY() function.
* @return True if all tests passed.
*/

bool testBlobDimentions ( )
{
	bool valid = true;
	Blob a (0, 0);

	valid &= a.width() == 1;
	valid &= a.height() == 1;
	valid &= a.roughX() == 0;
	valid &= a.roughY() == 0;

	a.minX = -2;
	valid &= a.width() == 3;
	valid &= a.height() == 1;
	valid &= a.roughX() == -1;
	valid &= a.roughY() == 0;

	a.minY = -3;
	valid &= a.width() == 3;
	valid &= a.height() == 4;
	valid &= a.roughX() == -1;
	valid &= a.roughY() == -1;

	a.maxX = -10;				// Yep this is supposed to be weired
	valid &= a.width() == -7;
	valid &= a.height() == 4;
	valid &= a.roughX() == -6;
	valid &= a.roughY() == -1;
	return valid;
}


/**
 * Tests Blob.spreadBlob function.
 * @return True if all tests passed.
 */

bool testBlobSpreadBlob ( )
{
	bool valid = true;

	int imgSize = 10;
	Blob a(imgSize / 2 + 1, 0);
	byte** img = new byte*[imgSize];
	for ( int y = 0; y < imgSize; y++ )
	{
		img[y] = new byte[imgSize];
		for ( int x = 0; x < imgSize / 2; x++ )				img[y][x] = 0;
		for ( int x = imgSize / 2; x < imgSize; x++)		img[y][x] = 100;
	}

	// Rectangle from top left to midX, full Y, evenly spaced.
	// Intencity = imgSize * 1.5 * 100
	a.spreadBlob(img, imgSize, imgSize, 1);
	valid &= a.minX 		== imgSize / 2;
	valid &= a.minY 		== 0;
	valid &= a.maxX 		== imgSize - 1;
	valid &= a.maxY 		== imgSize - 1;
	valid &= a.width() 		== imgSize / 2;
	valid &= a.height() 	== imgSize;
	cout << valid << a.width() << " " << a.centroid.x << "  " << a.centroid.y << endl;
	valid &= equal(a.centroid.y, (decimal) imgSize / 2);
	valid &= equal(a.centroid.x, (decimal) imgSize / 4 + 1);
	valid &= a.intensity 	== (unsigned int) imgSize * 150;

	for ( int y = 0; y < imgSize; y++ ) delete[] img[y];
	delete[] img;

	return valid;
}










/**
* @brief Tests pixelExist function.
* @return True if all tests passed.
*/

bool testBlobPixelExists ( )
{
	bool valid = true;
	byte** img = new byte*[2];
	img[0] = new byte[2];
	img[1] = new byte[2];

	img[0][0] = 0;
	img[0][1] = 0;
	img[1][0] = 0;
	img[1][1] = 0;

	Blob a (0, 0);
	// Valid on bounds
	valid &= a.pixelExist(img, 2, 2, -1, 0, 0);
	// Valid in bounds
	valid &= a.pixelExist(img, 2, 2, -1, 1, 1);

	// Invalid out of bounds negative x
	valid &= !a.pixelExist(img, 2, 2, -1, -1, 0);
	// Invalid out of bounds negative y
	valid &= !a.pixelExist(img, 2, 2, -1, 0, -1);
	// Invalid out of bounds positive x
	valid &= !a.pixelExist(img, 2, 2, -1, 2, 0);
	// Invalid out of bounds negative y
	valid &= !a.pixelExist(img, 2, 2, -1, 0, -2);

	// Invalid brightness inside bounds
	valid &= !a.pixelExist(img, 2, 2, 1, 0, 0);
	// Invalid brightness outside bounds
	valid &= !a.pixelExist(img, 2, 2, 1, -100, 0);

	delete[] img[0];
	delete[] img[1];
	delete[] img;

	return valid;
}










/**
* @brief Tests Point.set (degrees, minutes, seconds) function.
* @return True if all tests passed.
*/

bool testPointSet ( )
{
	bool valid = true;

	Point<decimal> a(2, 4, 6, 10, 20, 30);
	valid &= equal(a.x, 2.068333);
	valid &= equal(a.y, 10.34167);

	a.set(0, 0, 0, -1, -2, -3);
	valid &= equal(a.x, 0);
	valid &= equal(a.y, -1.034167);

	return valid;
}




/**
 * @brief Tests Point.equal function.
 * @return True if all tests passed.
 */

bool testPointEqual ( )
{
	bool valid = true;

	Point<int> a(10, 20);
	Point<int> b(10, 20);
	valid &= a.equal(b);

	a.set(11, 20);
	valid &= !a.equal(b);

	Point<decimal> c(10, 20);
	Point<decimal> d(10, 20);
	valid &= c.equal(d);

	c.set(0, 0);
	valid &= !c.equal(d);

	return valid;
}


/**
 * @brief Tests Point.distance function.
 * @return True if all tests passed.
 */

bool testPointDistance ( )
{
	bool valid = true;
	// Same place
	Point<int> a(10, 20);
	Point<int> b(10, 20);
	valid &= a.distance(b) == 0;
	// Whole number
	a.set(-10, 20);
	valid &= a.distance(b) == 20;

	// Decimal number
	a.set(1, 0);
	valid &= a.distance(b) == 21; // 21.9


	// Using decimal same place
	Point<decimal> c(10, 20);
	Point<decimal> d(10, 20);
	valid &= equal(c.distance(d), 0);

	// Whole number
	c.set(-10, 20);
	valid &= equal(c.distance(d), 20);

	// Decimal number
	c.set(1, 0);
	valid &= equal(c.distance(d), 21.9317121994613); // 21.9

	return valid;
}

}
