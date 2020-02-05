/*
 *	File: 		Image.hpp
 *	Author:		Tom Creusot
 *  Purpose:
 *				Stores an image as a byte array (unsigned char).
 *				The unix implementation.
 * Header For: Image.cpp.
 */

#ifndef IMAGE_IMPL_NIX_HPP
#define IMAGE_IMPL_NIX_HPP

#include "libs/image_processing/image_base.hpp"
#include <vector>

using namespace std;




namespace ip
{


class ImageNix : public ImageBase
{
protected:
		std::vector < std::vector<byte> > img;

public:
	/**
	* @brief 	Default Constructor.
	* @details	A single pixel of value 0.
	*/
	ImageNix	( );

	/**
	* @brief	Alternate Constructor.
	* @param w	The width of the image.
	* @param h	The height of the image.
	* @details	Sets all pixels to 0.
	*/

	ImageNix	( uint w, uint h );

	/**
	* @brief		Copy Constructor.
	* @param image The image to copy.
	*/

	ImageNix	( ImageNix& image );


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



	typedef ImageNix Image;
}

#endif
