/*
 *	File: 		GetImage.hpp
 *	Author:		Tom Creusot
 *  Purpose:
 *				The microcontroller version of get Image, this gets an image
 				from a camera.
 *
 * Requires:	../../GetImageInterface.hpp
 * Header For: GetImage.cpp
 */

#ifndef GET_INTERFACE_HPP
#define GET_INTERFACE_HPP

#include "../../GetImageInterface.hpp"

namespace gi
{

class GetImage : public GetImageInterface
{
	/**
	 * @brief		Constructs with camera
	 */

	GetImage ( );


	/**
	 * @brief	Acquires an image.
	 * @return	The image to observe.
	 */

	ImageInterface getImage ( );
}
}

#endif
