/*
 *	File: 		ImageProcessing.cpp
 *	Author:		Tom Creusot
 *  Purpose:
 *				Stores the functions in charge of mangaing the blobs.
 *
 *	Ideal calls:
 *				Call a threshold method, input it into findBlobs(),
 										call getMainPoints() sort in intensity.
 *
 *	Note:
 *				Calling spreadBlob will damage the image,
 				if you want to reuse it, make a copy first.
 *
 * File For: 	ImageProcessing.hpp.
 */


#include "ImageProcessing.hpp"



namespace ip
{


/**
 * @brief Finds the threshold via finding the min/max of pixels around the pixel.
 * Sets all pixels invalid to 0.
 *
 * @param img			The image to observe.
 * @param width			The width of the image.
 * @param height 		The height of the image.
 * @param sampleSize	The radius of pixels to sample
 									(the smaller the number, the faster it is).
 * @param aggression	0 to 1, 0 accepting most pixels,
 								1 accepting only the highest pixel.
 */

void adaptiveThreshold	( byte** img, int width, int height,
											int sampleSize, float aggression )
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
						if ( outOfPlace[yy][xx] < min )
													min = outOfPlace[yy][xx];
						else if (outOfPlace[yy][xx] > max)
													max = outOfPlace[yy][xx];
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





/**
 * @brief Finds all the blobs in an image with the specified parameters.
 *
 * @param img 		The image as an array.
 * @param width 	The width of the image.
 * @param height 	The height of the image.
 * @param bright	The brightness cut off.
 * @param dist		The distance cut off.
 * @return 			The blobs.
 *
 * THE RETURNED POINTER MUST BE DISPOSED!
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
 */

Blob* getMainPoints ( std::list<Blob> points, int num )
{
	Blob* set = new Blob[num];

	uint prevSize = -1; //-1 as unsigned is the largest int.
	for ( int i = 0; i < num; i++ )
	{
		set[i] = *points.begin();
		for ( std::list<Blob>::const_iterator it = points.begin();
													it != points.end(); it++ )
		{
			if ( it->intensity < prevSize && it->intensity > set[i].intensity )
			{
				set[i] = *it;
			}
		}
		prevSize = set[i].intensity;
	}
	return set;
}




#ifdef COMPUTER
/**
 * @brief Converts a list of points into an array.
 * This would be an alturnative to using getMainPoints.
 * @param point The values to be converted.
 * @return		The array format.
 *
 * THE RETURNED POINTER MUST BE DISPOSED!
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
 *
 * THE RETURNED POINTER MUST BE DISPOSED!
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
 *
 * THE RETURNED POINTER MUST BE DISPOSED!
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
 *
 * THE RETURNED POINTER MUST BE DISPOSED!
 */

byte** bmpToArray ( BMP& img )
{
	byte** im = new byte*[img.TellHeight()];

	for ( int y = 0; y < img.TellHeight(); y++ )
	{
		im[y] = new byte[img.TellWidth()];
		for (int x = 0; x < img.TellWidth(); x++ )
		{
			byte brightness = 	img(x, y) -> Red / 3 +
								img(x, y) -> Green / 3 +
								img(x, y) -> Blue / 3;
			im[y][x] = brightness;
		}
	}
	return im;
}

#endif
}
