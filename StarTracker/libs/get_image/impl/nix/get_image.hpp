/*
 *	File: 		GetImage.hpp
 *	Author:		Tom Creusot
 *  Purpose:
 *				The unix version of get Image, this gets an image from a
 				specified bitmap image.
 *
 * Requires:	../../GetImageInterface.hpp
 * Header For: GetImage.cpp
 */

#ifndef GET_INTERFACE_HPP
#define GET_INTERFACE_HPP

#include "../../GetImageInterface.hpp"

#include <string>
#include "../../Utils/Point.inc"
#include "EasyBMP.h"

using namespace std;

namespace gi
{
class GetImage : public GetImageInterface
{
private:
	BMP* image;
	std::string file;
public:
	/**
	 * @brief		Constructs with the file name.
	 * @param f		The file name and location.
	 */

	GetImage ( std::string& f );


	/**
	 * @brief	Acquires an image.
	 * @return	The image to observe.
	 */

	ImageInterface getImage ( );

};
}

#endif
