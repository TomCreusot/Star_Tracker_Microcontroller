/**
 *	Stores a byte image.
 *	@file 		image.h
 *	@author		Tom Creusot
 */

#pragma once
#include <cmath>
#include "libs/util/util.h"
#include "libs/util/array_list.h"

#include "config/runtime.h"

using namespace util;

/// @namespace image_processing	This is designed to peform blob detection on an image.
namespace image_processing
{

/**
 *	Stores an 8 bit image and peforms thresholding for blob detection.
 *
 *	@details
 *		The maximum size of the image is the specified Properties::kImageWidth
 *		and Properties::kImageHeight.
 *
 *	@example
 *		Image img(10, 10);									// Creates an image 10x10 with all pixels set to 0.
 *		cout << img.GetWidth() << endl;						// 10.
 *		img.SetPixel(2, 3, 20)								// Sets the brightness of the pixel (2,3).
 *		cout << img.GetPixel(2, 3) << endl; 				// 20.
 *
 *		const int list_size = 40;
 *		ArrayList<int, list_size> histogram(list_size);		// A brightness histogram with 20 bars.
 *		img.GenerateHistogram<list_size>(histogram);		// histogram[0] = 10 * 10 - 1 = 99, histogram[20/255*40 = 3] = 1.
 *
 *		Thresholding finds the bar that the aggression % of pixels is less.
 *		int threshold = img.PercentThreshold<list_size>(0, histogram);		// 0
 *		int threshold = img.PercentThreshold<list_size>(0.5, histogram);	// 0 (50 % of pixels <= 0)
 *		int threshold = img.PercentThreshold<list_size>(0.01, histogram);	// 3 (100 % of pixels <= 20 <- index 3)
 *
 *	@author	Tom Creusot
 */

class Image
{
private:
	/// The current dimentions of the image.
	uint width, height;
	/// The image.
	byte image[config::image_width][config::image_height];


public:

	/**
	 * @brief	Default Constructor.
	 * @details	A single pixel of value 0.
	 */

	Image	( );

	/**
	 * @brief	Alternate Constructor.
	 * @param w	The width of the image.
	 * @param h	The height of the image.
	 * @details	Sets all pixels to 0.
	 */

	Image	( uint w, uint h );


	/**
	 * @brief			Copy Constructor.
	 * @param img [in]	The image to copy.
	 */

	Image	( const Image& img );


	/**
	 * @brief	Getter for image width.
	 * @return 	The width of the image.
	 */

	uint GetWidth 	( ) const;

	/**
	 * @brief	Getter for image height.
	 * @return 	The height of the image.
	 */

	uint GetHeight 	( ) const;



	/**
	 * @brief	The maximum height of the image.
	 * @return	The maximum height of the image.
	 */
	static const uint MaxHeight	( );

	/**
	* @brief	The maximum width of the image.
	* @return	The maximum width of the image.
	*/
	static const uint MaxWidth	( );




	/**
	 * @brief	Returns the pixels intensity.
	 * @param x	The x coordinate.
	 * @param y	The y coordinate.
	 * @return	The intensity of the image at the x and y coordinate.
	 */

	byte GetPixel	( uint x, uint y ) const ;


	/**
	 * @brief		Finds if the specified location is within the bounds of the image.
	 * @param x		The x position.
	 * @param y		The y position.
	 * @return 		True if safe to access.
	 */

	bool ValidPixel ( uint x, uint y ) const;






	/**
	 * @brief		Sets the pixel intensity of a part on an image.
	 * @param x		The x coordinate.
	 * @param y		The y coordinate.
	 * @param color	The intensity.
	 */

	void SetPixel	( uint x, uint y, byte color );


	/**
	 * @brief		If valid, sets the width and height.
	 * @param w		The new width of the image.
	 * @param h		The new height of the image.
	 * @details		If the width or height exceeds IMAGE_*_MAX, neither will be set.
	 */

	void SetWidthHeight ( uint w, uint h );





	/**
	 * @brief				Finds the minimum and maximum values in the specified area.
	 * @param x				[in]	The middle x position.
	 * @param y				[in]	The middle y position.
	 * @param sampleRadius	[in]	The distance from the middle to sample.
	 * @param min 			[out]	The minimum variable to set.
	 * @param max 			[out]	The maximum variable to set.
	 */

	void FindMinMax ( uint x, uint y, uint sampleRadius, byte* min, byte* max );


	/**
	 * @brief				Reduces the image to 0 or important factors.
	 * @param sampleRadius	Distance to take the average from the focus pixel.
	 * @param aggression		The percentage of pixels to be removed (1 means delete everything).
	 * @details				Uses the average min/max method.
	 */

	void AdaptiveThreshold ( uint sampleRadius, float aggression );



	/**
	 * @brief					Deletes any pixel below the specified % of brightness in the image.
	 * @param aggression [in]	0 does not change any pixels, 1 sets all to 0.
	 * @param histogram	 [in]	The brightness histogram.
	 * @tparam N				The size of the array list "histogram".
	 *
	 * @return					The pixel brightness which is above the threshold.
	 */

	template<const int N>
	uint PercentThreshold ( decimal aggression, ArrayList<uint, N>& histogram )
	{
		// Finds where the cutoff is (inclusive).
		uint threshold = lround((decimal)width * (decimal)height * aggression);
		uint current = 0;
		uint i = 0;
		while ( current + histogram.Get(i) < threshold && i < histogram.Size() )
		{
			current += histogram.Get(i);
			i++;
		}
		return lround(255.0 * (decimal)i / (decimal)histogram.Size());
	}



	/**
	 * @brief					Fills an array with a colormap.
	 * @param histogram [out]	The array to fill.
	 * @tparam N				The size of the array list "histogram".
	 * @details					0 brightness will be [0].
	 */

	template<const int N>
	void GenerateHistogram ( ArrayList<uint, N>* histogram )
	{	// Fills the histogram with intensities.

		// Ensures all values are 0 to start with.
		for ( uint i = 0; i < histogram->Size(); i++ )
			histogram->Get(i) = 0;

		//
		for ( uint xx = 0; xx < width; xx++ )
			for ( uint yy = 0; yy < height; yy++ )
			{
				uint i = (uint)(
					(decimal) GetPixel(xx, yy) * (decimal) histogram->Size()
																		/ 256);
				histogram->Get(i)++;
			}
	}
};
}
