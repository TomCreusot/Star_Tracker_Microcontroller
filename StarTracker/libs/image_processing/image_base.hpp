/*
 *	File: 		ImageBase.hpp
 *	Author:		Tom Creusot
 *  Purpose:
 *				The core functionality of the Image class.
 *
 * Requires:	ImageInterface.hpp
 * Header For: impl/ * /Image.cpp
 */

#ifndef IMAGE_BASE_HPP
#define IMAGE_BASE_HPP

#include "image_interface.hpp"

namespace ip
{
class ImageBase : public ImageInterface
{
public:

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

	void findMinMax(uint x, uint y, uint sampleRadius, byte& min, byte& max);


	/**
	* @brief				Reduces the image to 0 or important factors.
	* @param sampleRadius	Distance to take the average from the focus pixel.
	* @return				Average intensity of important pixels in the image.
	* @details				Uses the average min/max method.
	*/

	void adaptiveThreshold ( uint sampleRadius, float aggression );
};
}



#endif
