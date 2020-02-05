/*
 *	File: 		GetImageInterface.hpp
 *	Author:		Tom Creusot
 *  Purpose:
 *				The core interface for the implementation of GetImage.
 *
 * Header For: impl/ * /GetImage.hpp
 */

#ifndef GET_IMAGE_INTERFACE_HPP
#define GET_IMAGE_INTERFACE_HPP

#include "../ImageProcessing/ImageInterface.hpp"

using namespace ip;
namespace gi
{

class GetImageInterface
{
	/**
	 * @brief	Acquires an image.
	 * @return	The image to observe.
	 */

	virtual ip::ImageBase getImage ( ) = 0;
}



}
#endif
