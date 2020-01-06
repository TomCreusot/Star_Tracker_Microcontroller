#include <cmath>
#include <list>
#include <iostream>
#include <vector>
#include <cmath>
#ifndef IMAGE_PROCESSING_HPP
#define IMAGE_PROCESSING_HPP


#define DEBUG_IMAGE_PROCESSING


using namespace std;

typedef unsigned char byte;
typedef unsigned int uint;


namespace ip
{

	/**
	 * This class is to provide details on a single blob.
	 * Refer to Blob.cpp
	 */

	class Blob
	{
	public:
		//x and y represent the top left position.
		int minX, maxX, minY, maxY, size;

		Blob							();
	 	Blob 							( int x, int y );
		bool withinThreshold 			( int x, int y, int distT );
		void add 						( int x, int y );
		#ifdef DEBUG_IMAGE_PROCESSING
			void print					();
		#endif
	};


	//Refere to ImageProcessing.cpp
	std::list<Blob>* findBlobs ( byte** img, const int width, const int height, const int bright, const int dist );

	//Refere to ImageProcessing.cpp
	byte percentThreshold ( byte** img, const int width, const int height, int colorSpace, float validAmount );
	byte otsuThreshold ( byte** img, const int width, const int height, int colorSpace, int startLocation, int finalTravelTolerance );
	uint* generateHistogram ( byte** img, const int width, const int height, const int numColorSpaces );

	//Refere to ImageProcessing.cpp
	Blob* getMainPoints ( std::list<Blob> points, int num );

	//Refere to ImageProcessing.cpp
	#ifdef DEBUG_IMAGE_PROCESSING
	Blob* listToArray ( std::list<Blob>* points );
	#endif
}

#endif
