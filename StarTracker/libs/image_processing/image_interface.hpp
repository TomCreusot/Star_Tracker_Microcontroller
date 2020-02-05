/*
 *	File: 		ImageInterface.hpp
 *	Author:		Tom Creusot
 *  Purpose:
 *				The interface for ImageBase and Image.
 *
 * Header For: ImageBase.hpp, impl/ * / Image.
 */

#ifndef IMAGE_INTERFACE_HPP
#define IMAGE_INTERFACE_HPP

// The resolution of the embedded camera
#define IMAGE_WIDTH 640
#define IMAGE_HEIGHT 480


#include "../utils/utils.hpp"

namespace ip
{
/**
 * Stores a grayscale 8-bit image.
 */

class ImageInterface
{
public:

	/**
	* @brief Getter for image width.
	* @return The width of the image.
	*/
	virtual uint getWidth 	( ) = 0;

	/**
	* @brief Getter for image height.
	* @return The height of the image.
	*/
	virtual uint getHeight 	( ) = 0;


	/**
	 * @brief Finds if the specified location is within the bounds of the image.
	 * @param x		The x position.
	 * @param y		The y position.
	 * @return 		True if safe to access.
	 */
	virtual bool validPixel ( uint x, uint y ) = 0;

	/**
	* @brief	Returns the pixels intensity.
	* @param x	The x coordinate.
	* @param y	The y coordinate.
	* @return	The intensity of the image at the x and y coordinate.
	*/
	virtual byte getPixel	( uint x, uint y ) = 0;


	/**
	* @brief		Sets the pixel intensity of a part on an image.
	* @param x		The x coordinate.
	* @param y		The y coordinate.
	* @param color	The intensity.
	*/
	virtual void setPixel	( uint x, uint y, byte color ) = 0;
};
}

#endif
