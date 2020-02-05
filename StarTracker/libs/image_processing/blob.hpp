/*
 *	File: 		Blob.hpp
 *	Author:		Tom Creusot
 *  Purpose:
 *				To search for and store details on a blob.
 *
 *	Reference:
 *				Uses simmilar logic to:
 *				http://what-when-how.com/
 				introduction-to-video-and-image-processing/
				blob-analysis-introduction-to-video-and-image-processing-part-1/
 *
 * Header For: Blob.cpp.
 */

#ifndef BLOB_HPP
#define BLOB_HPP

#include <queue>
#include <cmath>

#include "libs/utils/utils.hpp"
#include "libs/utils/point.hpp"
#include "libs/utils/array_list.hpp"
#include "image_base.hpp"

using namespace util;

namespace ip
{

/**
 * This class is to provide details on a single blob.
 * Refer to Blob.cpp
 */

class Blob
{
protected:
	// The top-left position:
	Point<uint> origin;
	// The width and height of the blob, if it is elongated or large, it is probably not a star.
	uint width, height;
	// The number of pixels.
	uint pixels;
	// Sum of the intensity of all the pixels.
	uint intensity;
	// The center weighted point.
	Point<decimal> centroid;


public:



	/**
	 * @brief 				Peforms grassfire blob detection on the desired image.
	 * @param img			The image to examine.
	 * @param threshold		What the intensity must be above to qualify as a blob. (0 means 1 qualifies).
	 * @param list			The list to append to.
	 * @details				Calls Blob.grassfire which deletes all pixels considered a blob.
	 */

	static void findBlobs ( ImageInterface& img, byte threshold, ArrayList<Blob>& list );





	/**
	 * @brief	Default Constructor.
	 * @details	Sets everything to 0.
	 */

	Blob					( );


	/**
	 * @brief Creates a blob at the position provided.
	 * @param x The initial x position.
	 * @param y The initial y position.
	 */

	Blob 					( uint x, uint y );




	/// @return The number of pixels in the blob.
	uint getPixels			( );
	/// @return The intensity of all the pixels added together.
	uint getIntensity		( );
	/// @return The actual center of the blob x.
	decimal getCentroidX	( );
	/// @return The actual center of the blob y.
	decimal getCentroidY	( );

	/// @return The bounding width of the blob.
	uint getWidth			( );
	/// @return The bounding height of the blob.
	uint getHeight			( );



	/**
	 * @brief Uses the grass fire method to find the true bounds of the blob.
	 * Sets any used pixels to 0.
	 *
	 * @param img			The image to examine SETS ALL USED PIXELS TO 0.
	 * @param inten		The brightness cut off (0 means will allow 1).
	 */

	void spreadGrassFire ( ImageInterface& image, uint intense );


	/**
	 * @brief			Finds the new center of mass in one dimention.
	 *
	 * @param centroid	The old center of mass.
	 * @param point		The new point position to add.
	 * @param weight	The weight of the new blob.
	 *
	 * @return			The new center of mass.
	 */

	decimal findCentroid ( decimal centroid, uint intense,
													uint point, byte weight );
};
}
#endif
