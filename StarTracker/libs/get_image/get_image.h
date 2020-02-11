/*
 *	File: 		get_image.h
 *	Author:		Tom Creusot
 *  Purpose:	To obtain a byte image via bmp on nix.
 *
 *	Header For: get_image.cc .
 */

#pragma once

#include "EasyBMP.h"
#include "libs/util/util.h"
#include "libs/util/array_list.h"
#include "libs/util/point.h"
#include "libs/image_processing/image.h"

using namespace util;
using namespace image_processing;

namespace get_image
{

class GetImage
{
private:
	BMP bmp;

public:
	/**
	 * @brief 		Alternate Constructor.
	 * @param file	The file to read.
	 * @details		Sets the bitmap variable to the file.
	 */

	GetImage ( char* file );


	/**
	 * @brief			Reads a bmp file from the computer and converts it to a byte image.
	 * @param file [in]	The file to read from.
	 * @param img [out]	The image to copy to.
	 * @details			nix implementation.
	 */

	void getImage ( Image& img );


	/**
	 * @brief 			Draws point on the bitmap image.
	 * @param points	The points to set the color of.
	 * @param r			The value of red.
	 * @param g			The value of green.
	 * @param b			The value of blue.
	 */

	void drawPoints(ArrayList<Point<decimal>>& points, byte r, byte g, byte b );


	/**
	 * @brief Sets the bitmap variable to the image supplied.
	 * @param img	The image to copy.
	 */

	void copyToBMP ( Image& img );




	/**
	 * @brief			Writes the bitmap to the file specified.
	 * @param file		The file to read from.
	 */

	void writeImage	( char* file );


};
}
