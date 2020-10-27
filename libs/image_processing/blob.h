/*
 *	To search for and store details on a blob.
 *	@file	blob.h
 *	@author	Tom Creusot
 *
 *	@reference
 *				Uses simmilar logic to:
 *				http://what-when-how.com/
 				introduction-to-video-and-image-processing/
				blob-analysis-introduction-to-video-and-image-processing-part-1/
 */

#pragma once

#include <cmath>

#include "libs/util/util.h"
#include "libs/util/point.h"
#include "libs/util/array_list.h"
#include "image.h"


using namespace util;
using namespace std;
/// @namespace image_processing
namespace image_processing
{

/**
 * @brief This class is to provide details on a single blob.
 *
 * @details
 *		To do get this information, it must use the grassfire method.
 *		This method looks at the pixels adjacent to the start pixel and if they are within the bounds, it will add them to a queue.
 *		When it is added to the queue it will set the pixel to 0.
 *		Once it is done this, it will have the dimentions of the pixel.
 *
 *		To find the centroid of the pixel, it uses a weighted average of:
 *			(prevous_centroid * previous_intensity + add_point * add_intensity) / (previous_intensity + add_intensity).
 *
 *		In this process the image is destroyed as all blobs are set to 0.
 *
 * @example
 *		const uint array_size = 100;
 *		ArrayList<Blob, array_size> blobs;
 *		Blob::FindBlobs<array_size>(img.PercentThreshold<array_size>(0.5, histogram), &img, &array_size);
 *		blobs.Get(0).get_centroid_x(); // The center weighted pixel on the x axis.
 *		blobs.Get(0).get_centroid_y(); // The center weighted pixel on the y axis.
 *
 * @author Tom Creusot
 */

class Blob
{
// protected:
public:
	/// Top-Left Position
	Point<uint> boundsMin;
	/// Bottom-Right Position
	Point<uint> boundsMax;
	/// The number of pixels.
	uint pixels;
	/// Sum of the intensity of all the pixels.
	uint intensity;
	/// The center weighted point.
	Point<decimal> centroid;


public:
	/**
	* @brief		Default Constructor.
	* @details		Sets everything to 0.
	*/

	Blob					( );


	/**
	* @brief Creates a blob at the position provided.
	* @param x The initial x position.
	* @param y The initial y position.
	*/

	Blob 					( uint x, uint y );




	/**
	* @brief			Peforms grassfire blob detection on the desired image.
	*					Prioritieses the brightest blobs.
	* @param threshold 	[in]		What the intensity must be above to qualify as a blob. (0 means 1 qualifies).
	* @param img		[in/out]	The image to examine.
	* @param list		[out]		The list to append to.
	*
	* @tparam NL		The size of the array list "list".
	* @tparam NB		The max size of a "blob".
	*
	* @details	Calls Blob.grassfire which deletes all pixels considered a blob.
	*/

	template<const uint NL, const uint NB>
	static void FindBlobs ( byte threshold, Image* img, ArrayList<Blob, NL>* list )
	{
		for ( uint y = 0; y < img->GetHeight(); y++ )
		{
			for ( uint x = 0; x < img->GetWidth(); x++ )
			{
				// Is the pixel valid?
				if ( img->GetPixel(x, y) > threshold )
				{
					Blob blob(x, y);
					blob.SpreadGrassFire<NB> ( threshold, img );

					list->Slot(blob, &SortByIntensity);
					// list->PushBack(blob);
				}
			}
		}
	}



	/**
	* @brief	Uses the grass fire method to find the true bounds of the blob.
	* Sets any used pixels to 0.
	*
	* @param threshold			The brightness cut off (exclusive).
	* @param img		[out]	The image to examine SETS ALL USED PIXELS TO 0.
	*
	* @tparam N					The max size of a blob.
	*/

	template<const uint N>
	void SpreadGrassFire ( uint threshold, Image* img )
	{
		ArrayList<Point<uint>, N> stack;

		stack.PushBack(Point<uint>(boundsMin.x, boundsMin.y));

		while ( !stack.IsEmpty() )
		{
			Point<uint> pt = stack.PopBack();
			FindNeighbours<N>(threshold, pt, img, &stack);
			ConsumePixel(pt, img);
		}
	}



	/**
	* @brief	Finds all pixels around the current pixel
	* @param threshold			The brightness cut off (exclusive).
	* @param pt			[in]	The point on the image.
	* @param img		[in]	The image to observer.
	* @param stack		[out]	The arraylist to append the points to.
	*/

	template<const uint N>
	void FindNeighbours ( uint threshold, Point<uint>& pt, Image* img, ArrayList<Point<uint>, N>* stack )
	{
		// 4 directional
		Point<uint> cur(pt.x + 1, pt.y);	// Right
		bool valid = img->ValidPixel(cur.x, cur.y);
		if ( valid && threshold < img->GetPixel(cur.x, cur.y) )
			stack->PushBack(cur);

		cur = Point<uint>(pt.x - 1, pt.y); // Left
		valid = img->ValidPixel(cur.x, cur.y);
		if ( valid && threshold < img->GetPixel(cur.x, cur.y) )
			stack->PushBack(cur);

		cur = Point<uint>(pt.x, pt.y + 1); // Down
		valid = img->ValidPixel(cur.x, cur.y);
		if ( valid && threshold < img->GetPixel(cur.x, cur.y) )
			stack->PushBack(cur);

		cur = Point<uint>(pt.x, pt.y - 1); // Up
		valid = img->ValidPixel(cur.x, cur.y);
		if ( valid && threshold < img->GetPixel(cur.x, cur.y) )
			stack->PushBack(cur);
	}



	/**
	* @brief Sets current pixel to 0 and adjusts the blobs dimentions to acomidate it.
	* @param pt		[out]		The position of the pixel.
	* @param img	[in/out]	The image to read / write the pixel to.
	*/

	void ConsumePixel ( Point<uint>& pt, Image* img );


	/**
	* @brief			Finds the new center of mass in one dimention.
	*
	* @param centroid	The old center of mass.
	* @param intensity	The previous intensity.
	* @param point		The new point position to add.
	* @param weight	The weight of the new blob.
	*
	* @return			The new center of mass.
	*/

	static decimal FindCentroid ( decimal centroid, uint intensity,
													uint point, byte weight );





	/**
	* @brief					Converts the Blob list to a Point list using the centroid.
	* @param blobs		[in]	The list of blobs.
	* @param points	[out]	The list of centroids to be created.
	*
	* @tparam N				The size of the array list "points".
	*/

	template<const int N>
	static void ToPointList ( 	ArrayList<Blob, N>& blobs,
										ArrayList<Point<decimal>, N>* points )
	{
		for ( uint i = 0; i < blobs.Size(); i++ )
		{
			points->PushBack(blobs.Get(i).centroid);
		}
	}


	/**
	* @brief				Used for util::ArrayList<Blob>.sort() to sort in decending order.
	* @param larger  [in]	The element that should be larger than the other.
	* @param smaller [in]	The element that should be smaller than the other.
	* @return				True if in order.
	*/

	static bool SortByIntensity ( Blob& larger, Blob& smaller );


	/**
	* @brief				Used for util::ArrayList<Blob>.sort() to sort in ascending order.
	* @param larger  [in]	The element that should be larger than the other.
	* @param smaller [in]	The element that should be smaller than the other.
	* @return				True if in order.
	*/

	static bool SortByIntensityAscending ( Blob& smaller, Blob& larger );

};
}
