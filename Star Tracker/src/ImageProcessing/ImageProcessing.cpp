/*
 *	File: 		ImageProcessing.cpp
 *	Author:		Tom Creusot
 *  Purpose:
 *				Stores the functions in charge of mangaing the blobs.
 *
 *	Ideal calls:
 *				Call a threshold method, input it into findBlobs(), call getMainPoints() sort in intensity.
 *
 *	Note:
 *				Calling spreadBlob will damage the image, if you want to reuse it,
 *				make a copy first.
 *
 * File For: 	ImageProcessing.hpp, blob.cpp.
 */


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

	byte percentThreshold ( byte** img, int width, int height, int colorSpace, float validAmount )
	{
		uint* colors = generateHistogram(img, width, height, colorSpace);

		// "validAmount"% of all pixels.
		uint max = (unsigned long)((float) width * (float) height * validAmount);
		uint count = 0;
		int space = 0;
		// Finds the color space which is higher than the "validAmount"%
		while ( count < max && space < colorSpace )
		{
			count += colors[space];
			space++;
		}
		delete[] colors;
		return space * 255 / colorSpace;
	}


	/**
	 * @brief Finds the threshold via finding the min/max of pixels around the pixel.
	 * Sets all pixels invalid to 0.
	 *
	 * @param img			The image to observe.
	 * @param width			The width of the image.
	 * @param height 		The height of the image.
	 * @param sampleSize	The radius of pixels to sample (the smaller the number, the faster it is).
	 * @param aggression	0 to 1, 0 accepting most pixels, 1 accepting only the highest pixel.
	 */

	void adaptiveThreshold	( byte** img, int width, int height, int sampleSize, float aggression )
	{
		byte outOfPlace[height][width];
		for ( int y = 0; y < height; y++ )
		{
			for ( int x = 0; x < width; x++ )
			{
				outOfPlace[y][x] = img[y][x];
			}
		}

		for ( int y = 0; y < height; y++ )
		{
			for ( int x = 0; x < width; x++ )
			{
				// Taking the area around the pixel.
				int intensity = 0;
				int min = outOfPlace[y][x];
				int max = outOfPlace[y][x];
				for ( int yy = y - sampleSize; yy < y + sampleSize; yy++ )
				{
					for ( int xx = x - sampleSize; xx < x + sampleSize; xx++ )
					{

						if ( yy > 0 && yy < height && xx > 0 && xx < width )
						{
							if ( outOfPlace[yy][xx] < min ) 	min = outOfPlace[yy][xx];
							else if (outOfPlace[yy][xx] > max) 	max = outOfPlace[yy][xx];
						}
					}
				}
				// Mean method.
				intensity = min * (1 - aggression) + max * (aggression);
				if ( outOfPlace[y][x] < intensity || intensity == 0)
				{
					img[y][x] = 0;
				}
			}
		}
	}


	// byte valleyThreshold (byte** img, int width, int height, int color)


	/**
	 * @brief Finds the threshold of the image using otsu's thresholding.
	 *
	 * @param width The width of the image.
	 * @param height The height of the image.
	 * @param colorSpace	The # of bands in the histogram (generateHistogram).
	 *



	byte otsuThreshold ( byte** img, int width, int height, int colorSpace, int startLocation, int finalTravelTolerance )
	{
		uint* colors = generateHistogram(img, width, height, colorSpace);

		float prevThresh = -finalTravelTolerance;
	 	float curThresh = startLocation;

		while ( abs(curThresh - prevThresh) > finalTravelTolerance )
		{
			for ( int i = 0; i < colorSpace; i++)
			{
				prevThresh += colorSpace * ;
			}
		}
		return curThresh;
	}
*/



	/**
	 * @brief Generates an array of ints representing the count of each intensity band.
	 * Each band is the size of "255/numColorSpaces".
	 * Entering 2 as "numColorSpaces" will result in 0 - 127.
	 * Entering 255 as "numColorSpaces" will result in each intensity fitting into a band.
	 *
	 * @param img	The image to generate a histogram for.
	 * @param width	The width of the image.
	 * @param height The height of the image.
	 * @param numColorSpaces	The number of bands of the histogram.
	 */

	uint* generateHistogram ( byte** img, int width, int height, int numColorSpaces )
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
	 *
	 * @param img 		The image as an array.
	 * @param width 	The width of the image.
	 * @param height 	The height of the image.
	 * @param bright	The brightness cut off.
	 * @param dist		The distance cut off.
	 * @return 			The blobs THIS MUST BE DISPOSED OF!!!.
	 */

	std::list<Blob>* findBlobs ( byte** img, int width, int height, int bright )
	{
		std::list<Blob>* points = new std::list<Blob>();

		for ( int y = 0; y < height; y++ )
		{
			for ( int x = 0; x < width; x++ )
			{
				// Is the pixel valid?
				if ( img[y][x] > bright )
				{
					Blob* blob = new Blob(x, y);
					blob->spreadBlob ( img, width, height, bright );
					points -> push_back(*blob);
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
	 *
	 * Potential improvements:
	 *		Find the smallest value in the list, assign it to all the slots of the new array.
	 *		Loop through once, every time the value is > a list element, insert and shuffle up.
	 *
	 *		Does not account for if points is smaller than num!!!!
	 */

	Blob* getMainPoints ( std::list<Blob> points, int num )
	{
		Blob* set = new Blob[num];

		uint prevSize = -1;
		for ( int i = 0; i < num; i++ )
		{
			set[i] = *points.begin();

			for ( std::list<Blob>::const_iterator it = points.begin(); it != points.end(); it++ )
			{
				if ( (it->intensity < prevSize || prevSize < 0) && it->intensity > set[i].intensity )
				{
					set[i] = *it;
				}
			}
			prevSize = set[i].intensity;
		}
		return set;
	}




	#ifdef DEBUG_IMAGE_PROCESSING
	/**
	 * @brief Converts a list of points into an array.
	 * This would be an alturnative to using getMainPoints.
	 * @param point The values to be converted.
	 * @return		The array format.
	 */

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


	/**
	 * @brief			Combines 2 arrays into 2 images side by side.
	 * @param img1 		The left image.
	 * @param img2		The right image.
	 * @param width		The width of the image.
	 * @param height 	The height of the images.
	 * @return 			The image.
	 */

	BMP* combineImages ( byte** img1, byte** img2, int width, int height )
	{
		BMP* bmp = new BMP();
		bmp -> SetSize(width * 2, height);
		bmp -> SetBitDepth(8);
		for ( int y = 0; y < height; y++ )
		{
			for ( int x = 0; x < width; x++ )
			{
				(*bmp)(x, y) 		-> Red		= img1[y][x];
				(*bmp)(x, y) 		-> Green	= img1[y][x];
				(*bmp)(x, y) 		-> Blue 	= img1[y][x];

				(*bmp)(x + width, y)-> Red 		= img2[y][x];
				(*bmp)(x + width, y)-> Green	= img2[y][x];
				(*bmp)(x + width, y)-> Blue		= img2[y][x];
			}
		}
		return bmp;
	}



	/**
	* @brief			Combines an image and an array to 2 images side by side.
	* @param bmpImg 	The left image.
	* @param img		The right image.
	* @param width		The width of the image.
	* @param height 	The height of the images.
	* @return 			The image.
	*/

	BMP* combineImages ( BMP& bmpImg, byte** img, int width, int height )
	{
		BMP* bmp = new BMP();
		bmp -> SetSize(width * 2, height);
		bmp -> SetBitDepth(24);
		for ( int y = 0; y < height; y++ )
		{
			for ( int x = 0; x < width; x++ )
			{
				(*bmp)(x, y) 		-> Red		= bmpImg(x, y) -> Red;
				(*bmp)(x, y) 		-> Green	= bmpImg(x, y) -> Green;
				(*bmp)(x, y) 		-> Blue		= bmpImg(x, y) -> Blue;

				(*bmp)(x + width, y)-> Red 		= img[y][x];
				(*bmp)(x + width, y)-> Green	= img[y][x];
				(*bmp)(x + width, y)-> Blue		= img[y][x];
			}
		}
		return bmp;
	}




	/**
	 * @brief		Converts a bitmap image into a 2d array of bytes.
	 * @param img	The image to convert.
	 * @return		The array.
	 */

	byte** bmpToArray ( BMP& img )
	{
		byte** im = new byte*[img.TellHeight()];

		for ( int y = 0; y < img.TellHeight(); y++ )
		{
			im[y] = new byte[img.TellWidth()];
			for (int x = 0; x < img.TellWidth(); x++ )
			{
				im[y][x] = img(x, y) -> brightness();
			}
		}
		return im;
	}

	#endif
}
