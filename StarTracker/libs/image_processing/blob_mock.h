/*
 *	File: 		blob_mock.h
 *	Author:		Tom Creusot
 *  Purpose:
 				Allows the setting of size for testing instead of Blob.
 */

#ifndef BLOB_MOCK_HPP
#define BLOB_MOCK_HPP

#include "libs/image_processing/blob.h"

namespace image_processing
{

class BlobMock : public Blob
{
public:
	/**
	 * @brief				Initializes ideal elements for testing.
	 * @param	intensity	Setter of intensity.
	 * @param	x_			Setter of centroid x.
	 * @param	y_			Setter of centroid y.
	 * @param	width_		Setter of width, pixels.
	 * @param	height_		Setter of height, pixels.
	 */

	BlobMock ( int intensity_, decimal x_, decimal y_, uint width_, uint height_ ) : Blob(0, 0)
	{
		intensity = intensity_;
		centroid = Point<decimal>(x_, y_);
		width = width_;
		height = height_;
		pixels = width * height;
	}


	/**
	 * @brief				Initializes ideal elements for testing.
	 * @param	intensity	Setter of intensity / pixels.
	 * @param	x_			Setter of centroid x.
	 * @param	y_			Setter of centroid y.
	 */

	BlobMock ( int intensity_, decimal x_, decimal y_ ) : Blob(0, 0)
	{
		intensity = pixels = intensity_;
		centroid = Point<decimal>(x_, y_);
	}



	/**
	 * @brief				Initializes ideal elements for testing.
	 * @param	intensity	Setter of intensity size and pixels.
	 */

	BlobMock ( uint intensity_ ) : Blob(0, 0)
	{
		intensity = pixels = intensity_;
	}
};
}
#endif
