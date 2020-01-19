/*
 *	File: 		ImageProcessing.hpp
 *	Author:		Tom Creusot
 *  Purpose:
 *				To provide a simple light weight form of blob detection.
 *				It is designed to run fast on a microcontroller to detect stars.
 *
 *	Reference:
 *				Uses simmilar logic to:
 *				http://what-when-how.com/
 				introduction-to-video-and-image-processing/
				blob-analysis-introduction-to-video-and-image-processing-part-1/
 *
 * Header For: 	Blob.cpp, ImageProcessing.cpp.
 */


#ifndef IMAGE_PROCESSING_HPP
#define IMAGE_PROCESSING_HPP

#define COMPUTER

#include <cmath>
#include <list>
#include <iostream>
#include <vector>
#include <cmath>
#include <queue>

#include "Point.cpp"

//These are only needed in debugging.
#ifdef COMPUTER
	#include "../EasyBMP/EasyBMP.h"
	#include <iostream>
#endif


using namespace std;

// Input double or float ... depending on the resolution.
typedef float decimal;

// Convienence
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
		// The bounding rectangle:
		int minX, maxX, minY, maxY;
		// The number of pixels in the bounding box which are valid.
		int pixels;
		// Sum of the intensity of all the pixels.
		uint intensity;
		// The center weighted point.
		Point<decimal> centroid;

		Blob					( );
	 	Blob 					( int x, int y );
		bool withinThreshold 	( int x, int y, int distT );
		void add 				( int x, int y );

		void spreadBlob
						( byte** img, int width, int height, int brightness );
		bool pixelExist ( byte** img, int width, int height, int brightness,
																int x, int y );

		int width				( );
		int height				( );
		int roughX				( );
		int roughY				( );

		#ifdef COMPUTER
			void print			( );
		#endif
	};






	//Refere to ImageProcessing.cpp
	void adaptiveThreshold	( byte** img, int width, int height,
											int sampleSize, float aggression );
	std::list<Blob>* findBlobs ( byte** img, int width, int height, int bright );
	Blob* getMainPoints ( std::list<Blob> points, int num );

	#ifdef COMPUTER
		//Refere to ImageProcessing.cpp
		Blob* listToArray 	( std::list<Blob>* points );
		BMP* combineImages 	( byte** img1, byte** img2, int width, int height );
		BMP* combineImages 	( BMP& img1, byte** img2, int width, int height );
		byte** bmpToArray 	( BMP& img );
	#endif
}

#endif
