#include "ImageProcessing.hpp"


namespace ip
{

	/**
	 * @brief Finds the threshold by finding a point above the validAmount % of all pixels.
	 * (Just testing).
	 *
	 * @param img The image to observe.
	 * @param width The width of the image.
	 * @param height The height of the image.
	 * @param colorSpace The # of bands in the histogram (generateHistogram).
	 * @param validAmount The % of pixels which should count as the background.
	 *
	 * @return The a value from 0 to 255 representing the brightenss to fit the specifications..
	 */

	byte percentThreshold ( byte** img, const int width, const int height, int colorSpace, float validAmount )
	{
		uint* colors = generateHistogram(img, width, height, colorSpace);

		// "validAmount"% of all pixels.
		uint max = (unsigned long)((float) width * (float) height * validAmount);
		uint count = 0;
		int space = 0;
		// Finds the color space which is higher than the "validAmount"%
		while (count < max && space < colorSpace)
		{
			count += colors[space];
			space++;
		}
		delete[] colors;
		return space * 255 / colorSpace;
	}




	/**
	 *
	 * WIP
	 *
	 *
	 */



	byte otsuThreshold ( byte** img, const int width, const int height, int colorSpace, int startLocation, int finalTravelTolerance )
	{
		uint* colors = generateHistogram(img, width, height, colorSpace);

		float prevThresh = -finalTravelTolerance;
	 	float curThresh = startLocation;

		while ( abs(curThresh - prevThresh) > finalTravelTolerance )
		{
			for ( int i = 0; i < colorSpace; i++)
			{
				prevThresh += colorSpace *
			}
		}
		return curThresh;
	}




	/**
	 * @brief Generates an array of ints representing the count of each intencity band.
	 * Each band is the size of "255/numColorSpaces".
	 * Entering 2 as "numColorSpaces" will result in 0 - 127.
	 * Entering 255 as "numColorSpaces" will result in each intencity fitting into a band.
	 *
	 * @param img	The image to generate a histogram for.
	 * @param width	The width of the image.
	 * @param height The height of the image.
	 * @param numColorSpaces	The number of bands of the histogram.
	 */

	uint* generateHistogram ( byte** img, const int width, const int height, const int numColorSpaces )
	{
		uint* colors = new uint[numColorSpaces];

		// Initializes the array to 0.
		for (int i = 0; i < numColorSpaces; i++)
		{
			colors[i] = 0;
		}

		// Sorts each pixel into a specific color range.
		for ( int y = 0; y < height; y++ )
		{
			for ( int x = 0; x < width; x++ )
			{
				uint c = (uint) img[y][x] * numColorSpaces / 255;
				colors[c]++;
			}
		}
		return colors;
	}









	/**
	 * @brief Finds all the blobs in an image with the specified parameters.
	 * @param img 		The image as an array.
	 * @param width 	The width of the image.
	 * @param height 	The height of the image.
	 * @param bright	The brightness cut off.
	 * @param dist		The distance cut off.
	 * @return 			The blobs THIS MUST BE DISPOSED OF!!!.
	 */

	std::list<Blob>* findBlobs ( byte** img, const int width, const int height, const int bright, const int dist )
	{
		std::list<Blob>* points = new std::list<Blob>();

		for ( int y = 0; y < height; y++ )
		{
			for ( int x = 0; x < width; x++ )
			{
				// Is the pixel valid?
				if ( img[y][x] > bright )
				{
					bool found = false;
					// Is it within the bounds of another blob?
					for ( std::list<Blob>::iterator it = points->begin();
													it != points->end() && !found; ++it )
					{
						if ( it -> withinThreshold(x, y, dist) )
						{
							it -> add(x, y);
							found = true;
						}
					}
					// Should it be made a new blob?
					if ( !found || points->empty() )
					{
						Blob* blob = new Blob(x, y);
						points->push_back(*blob);
					}
				}
			}
		}
		return points;
	}





	/**
	 * @brief			Finds the brightest points from the list.
	 * @param points	The points to examine.
	 * @param num		The number of stars to append to the list.
	 * @return			The brightest points (0 index is brightest).
	 *
	 * THE RETURNED POINTER MUST BE DISPOSED!

	 * Potential improvements:
	 *		Find the smallest value in the list, assign it to all the slots of the new array.
	 *		Loop through once, every time the value is > a list element, insert and shuffle up.
	 *
	 *		Does not account for if points is smaller than num!!!!
	 */

	Blob* getMainPoints ( std::list<Blob> points, int num )
	{
		Blob* set = new Blob[num];

		int prevSize = -1;
		for (int i = 0; i < num; i++)
		{
			set[i] = *points.begin();

			for (std::list<Blob>::const_iterator it = points.begin(); it != points.end(); it++)
			{
				if ((it->size < prevSize || prevSize < 0) && it->size > set[i].size)
				{
					set[i] = *it;
				}
			}
			prevSize = set[i].size;
		}
		return set;
	}




	#ifdef DEBUG_IMAGE_PROCESSING
	Blob* listToArray ( std::list<Blob>* points )
	{
		Blob* list = new Blob[points->size()];
		std::list<Blob>::const_iterator it = points->begin();
		for ( uint i = 0; i < points->size(); i++ )
		{
			list[i] = *it;
			it++;
		}
		return list;
	}
	#endif
}
