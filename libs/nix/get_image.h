/*
 *	To obtain a byte image via bmp on nix.
 *	@file		get_image.h
 *	@author		Tom Creusot
 */

#pragma once

#include <string>

#include "EasyBMP.h"
#include "libs/util/util.h"
#include "libs/util/array_list.h"
#include "libs/util/point.h"
#include "libs/image_processing/image.h"

using namespace std;
using namespace util;
using namespace image_processing;
///	@namespace get_image	Obtaining and printing the image.
namespace nix
{
/**
 *	To read and write an image to a file (nix implementation).
 *	@details
 *				Only uses .bmp, tested only with windows .bmp
 *	@example
 *				GetImage bmp("image_file.bmp");			// Reads in an bmp image.
 *				image_processing::Image img;
 *				img.GetBMP(&img);						// Copies bmp to img.
 *				bmp.DrawPoints(point_list, 255, 0, 255);// Draws a purple pixel on every point in the list.
 *				bmp.WriteImage("out.bmp");				// Draws the image with the points.
 *
 *	@author		Tom Creusot
 */

class GetImage
{
private:
	BMP bmp;

public:
	/**
	 * @brief		Default Constructor.
	 */

	GetImage ( );


	/**
	 * @brief 			Alternate Constructor.
	 * @param file [in]	The file to read.
	 * @details			Sets the bitmap variable to the file.
	 */

	GetImage ( string file );



	/**
	 * @brief 				Draws point on the bitmap image.
	 * @param points [in]	The points to set the color of.
	 * @param r				The value of red.
	 * @param g				The value of green.
	 * @param b				The value of blue.
	 * @tparam N			The max size of the array list.
	 */

	template<const uint N>
	void DrawPoints(ArrayList<Point<decimal>,N>& points, byte r, byte g, byte b)
	{
		for ( uint i = 0; i < points.Size(); i++ )
		{
			bmp(points.Get(i).x, points.Get(i).y)->Red = r;
			bmp(points.Get(i).x, points.Get(i).y)->Green = g;
			bmp(points.Get(i).x, points.Get(i).y)->Blue = b;
		}
	}



	/**
	 * @brief			Reads a bmp file from the computer and converts it to a byte image.
	 * @param img [out]	The image to copy/write to.
	 */

	void GetBMP ( Image* img );

	/**
	 * @brief Sets the bitmap variable to the image supplied.
	 * @param img [in]	The image to copy.
	 */

	void SetBMP ( Image& img );






	/**
	 * @brief				Writes the bitmap to the file specified.
	 * @param file [in]		The file to read from.
	 */

	void WriteImage	( string file );


};
}
