/**
 *	File: 		image.h
 *	Author:		Tom Creusot
 *  Purpose:
 *				Stores a byte image.
 *
 * Header For: image.cc
 */

#pragma once

// The resolution of the embedded camera
#define IMAGE_WIDTH_MAX 640
#define IMAGE_HEIGHT_MAX 480

#include "libs/util/util.h"
using namespace util;

namespace image_processing
{
/**
 * Stores a grayscale 8-bit image.
 */

class Image
{
private:
	uint width, height;
	byte image[IMAGE_HEIGHT_MAX][IMAGE_WIDTH_MAX];


public:

	/**
	* @brief 	Default Constructor.
	* @details	A single pixel of value 0.
	*/

	Image	( );

	/**
	* @brief	Alternate Constructor.
	* @param w	The width of the image.
	* @param h	The height of the image.
	* @details	Sets all pixels to 0.
	*/

	Image	( uint w, uint h );


	/**
	* @brief		Copy Constructor.
	* @param img	The image to copy.
	*/

	Image	( Image& img );


	/**
	* @brief Getter for image width.
	* @return The width of the image.
	*/

	uint getWidth 	( );

	/**
	* @brief Getter for image height.
	* @return The height of the image.
	*/

	uint getHeight 	( );


	/**
	* @brief	Returns the pixels intensity.
	* @param x	The x coordinate.
	* @param y	The y coordinate.
	* @return	The intensity of the image at the x and y coordinate.
	*/

	byte getPixel	( uint x, uint y );


	/**
	* @brief		Sets the pixel intensity of a part on an image.
	* @param x		The x coordinate.
	* @param y		The y coordinate.
	* @param color	The intensity.
	*/

	void setPixel	( uint x, uint y, byte color );


	/**
	 * @brief		If valid, sets the width and height.
	 * @param w		The new width of the image.
	 * @param h		The new height of the image.
	 * @details		If the width or height exceeds IMAGE_*_MAX, neither will be set.
	 */

	void setWidthHeight ( uint w, uint h );




	/**
	 * @brief Finds if the specified location is within the bounds of the image.
	 * @param x		The x position.
	 * @param y		The y position.
	 * @return 		True if safe to access.
	 */

	bool validPixel ( uint x, uint y );


	/**
	 * @brief Finds the minimum and maximum values in the specified area.
	 * @param x				The middle x position.
	 * @param y				The middle y position.
	 * @param sampleRadius	The distance from the middle to sample.
	 * @param min			The minimum variable to set.
	 * @param max			The maximum variable to set.
	 */

	void findMinMax ( uint x, uint y, uint sampleRadius, byte& min, byte& max );


	/**
	* @brief				Reduces the image to 0 or important factors.
	* @param sampleRadius	Distance to take the average from the focus pixel.
	* @return				Average intensity of important pixels in the image.
	* @details				Uses the average min/max method.
	*/

	void adaptiveThreshold ( uint sampleRadius, float aggression );
};
}
