/*
 *	File: 		Image.hpp
 *	Author:		Tom Creusot
 *  Purpose:
 *				Stores an image as a byte array (unsigned char).
 *				This is optimised for a microcontroller, having heap memory.
 * Header For: Image.cpp.
 */

#ifndef IMAGE_EMBEDDED_IMPL_HPP
#define IMAGE_EMBEDDED_IMPL_HPP

#include "libs/image_processing/image_base.hpp"

namespace ip
{
class ImageEmbedded : public ImageBase
{
protected:
	byte img [IMAGE_HEIGHT][IMAGE_WIDTH] = {0};

public:
	/**
	* @brief 	Default Constructor.
	* @details	Sets size to IMAGE_STM_WIDTH/Height with all values 0.
	*/
	ImageEmbedded	( );


	/**
	* @brief		Copy Constructor.
	* @param image The image to copy.
	*/

	ImageEmbedded	( ImageEmbedded& image );


	/**
	* @brief Getter for image width.
	* @return The width of the image.
	*/
	uint getWidth 	( );


	/**
	* @brief Getter for image height.
	* @return The height of the image.
	*/
	uint getHeight 	( ) ;



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
};

typedef ImageEmbedded  Image;
}
#endif
