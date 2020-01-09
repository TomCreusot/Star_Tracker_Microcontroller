#include <cmath>
#include <list>
#include <iostream>
#include <vector>
#include <cmath>
#include <queue>

#ifndef IMAGE_PROCESSING_HPP
#define IMAGE_PROCESSING_HPP


#define DEBUG_IMAGE_PROCESSING
#ifdef DEBUG_IMAGE_PROCESSING
	#include "../EasyBMP/EasyBMP.h"
	#include <iostream>
#endif


using namespace std;

// Convienence
typedef unsigned char byte;
typedef unsigned int uint;


namespace ip
{

	/**
	 * This struct is designed to store the x and y position in any datatype.
	 */

	template <class T>
	struct Point
	{
 	 	T x, y;

		Point ( )
		{
			x = 0;
			y = 0;
		}

		Point ( T x_, T y_ )
		{
			x = x_;
			y = y_;
		}
	};



	/**
	 * This class is to provide details on a single blob.
	 * Refer to Blob.cpp
	 */

	class Blob
	{
	public:
		// The bounding rectangle:
		int minX, maxX, minY, maxY;
		// The number of pixels in the bounding box which are valid.
		int pixels;
		// Sum of the intensity of all the pixels.
		uint intensity;
		// The center weighted point.
		Point<int> centroid;

		Blob					( );
	 	Blob 					( int x, int y );
		bool withinThreshold 	( int x, int y, int distT );
		void add 				( int x, int y );

		void spreadBlob 		( byte** img, const int width, const int height, const int brightness );
		bool pixelExist 		( byte** img, const int width, const int height, const int brightness, const int x, const int y );

		int width				( );
		int height				( );
		int roughX				( );
		int roughY				( );

		#ifdef DEBUG_IMAGE_PROCESSING
			void print			( );
		#endif
	};







	//Refere to ImageProcessing.cpp
	std::list<Blob>* findBlobs ( byte** img, const int width, const int height, const int bright );

	//Refere to ImageProcessing.cpp
	byte percentThreshold ( byte** img, const int width, const int height, int colorSpace, float validAmount );
	byte otsuThreshold ( byte** img, const int width, const int height, int colorSpace, int startLocation, int finalTravelTolerance );
	uint* generateHistogram ( byte** img, const int width, const int height, const int numColorSpaces );

	//Refere to ImageProcessing.cpp
	Blob* getMainPoints ( std::list<Blob> points, int num );

	//Refere to ImageProcessing.cpp
	#ifdef DEBUG_IMAGE_PROCESSING
	Blob* listToArray ( std::list<Blob>* points );
	BMP* combineImages ( byte** img1, byte** img2, const int width, const int height );
	#endif
}

#endif
