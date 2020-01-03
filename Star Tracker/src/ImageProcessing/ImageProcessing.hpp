#include <cmath>
#include <list>
#include <iostream>
#ifndef IMAGE_PROCESSING_HPP
#define IMAGE_PROCESSING_HPP
using namespace std;
typedef unsigned char byte;
#define VALID_PREPROCESSOR defined(_WIND32) || defined(_WIND64) || defined(unix) || defined(__APPLE__)



namespace ip
{

	/**
	 * This class is to provide details on a single blob.
	 */

	class Blob
	{
	public:
		//x and y represent the top left position.
		int minX, maxX, minY, maxY;

	 	Blob 							( int x, int y );
		bool withinThreshold 			( int x, int y, int distT );
		void add 						( int x, int y );

	};


	//Refere to ImageProcessing.cpp
	std::list<Blob>* findBlobs ( byte** img, int width, int height, int bright, int dist );


/**
	#if VALID_PREPROCESSOR
	void draw (char** img, int width, int height)
	{
		//STUB
	}

	void drawPoints (char** img, int width, int height, list<Blob>& points)
	{
		//STUB
	}
	#endif

*/


}

#endif
