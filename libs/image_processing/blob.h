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

// #include <queue>
#include <iostream>

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
	/// The top-left position:
	Point<uint> origin;
	/// The width of the blob, if it is elongated or large, it is probably not a star.
	uint width;
	/// The height of the blob, if it is elongated or large, it is probably not a star.
	uint height;
	/// The number of pixels.
	uint pixels;
	/// Sum of the intensity of all the pixels.
	uint intensity;
	/// The center weighted point.
	Point<decimal> centroid;


public:
	/**
	 * @brief							Peforms grassfire blob detection on the desired image.
	 * @param threshold 	[in]		What the intensity must be above to qualify as a blob. (0 means 1 qualifies).
	 * @param img			[in/out]	The image to examine.
	 * @param list			[out]		The list to append to.
	 *
	 * @tparam N1			The size of the array list "list".
	 * @tparam N2			The max size of a "blob".
	 *
	 * @details	Calls Blob.grassfire which deletes all pixels considered a blob.
	 */

	template<const uint N1, const uint N2>
	static void FindBlobs ( byte threshold, Image* img, ArrayList<Blob, N1>* list )
	{
		for ( uint y = 0; y < img->GetHeight(); y++ )
		{
			for ( uint x = 0; x < img->GetWidth(); x++ )
			{
				// Is the pixel valid?
				if ( img->GetPixel(x, y) > threshold )
				{
					Blob blob(x, y);
					blob.SpreadGrassFire<N2> ( threshold, img );
					list->PushBack(blob);
				}
			}
		}
	}


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




	// /// @return The number of pixels in the blob.
	// uint 	inline	get_pixels			( ) const	{ return pixels;		}
	// /// @return The intensity of all the pixels added together.
	// uint 	inline 	get_intensity		( ) const	{ return intensity;		}
	// /// @return The actual center of the blob x.
	// decimal	inline	get_centroid_x		( ) const	{ return centroid.x;	}
	// /// @return The actual center of the blob y.
	// decimal	inline	get_centroid_y		( ) const	{ return centroid.y;	}
	// /// @return The left of the blob.
	// decimal inline	get_origin_x		( ) const	{ return origin.x;		}
	// /// @return The top of the blob.
	// decimal inline	get_origin_y		( ) const	{ return origin.y;		}
	//
	// /// @return The bounding width of the blob.
	// uint	inline	get_width			( ) const	{ return width;			}
	// /// @return The bounding height of the blob.
	// uint	inline	get_height			( )	const	{ return height;		}



	/**
	 * @brief	Uses the grass fire method to find the true bounds of the blob.
	 * Sets any used pixels to 0.
	 *
	 * @param threshold			The brightness cut off (0 means will allow 1).
	 * @param img		[out]	The image to examine SETS ALL USED PIXELS TO 0.
	 *
	 * @tparam	N				The max size of a blob.
	 */

	template<const uint N>
	void SpreadGrassFire ( uint threshold, Image* img )
	{
		util::ArrayList<util::Point<util::uint>, N> q;

		uint minX = round(centroid.x), minY = round(centroid.y);
		uint maxX = minX, maxY = minY;

		q.PushBack(Point<uint>(minX, minY));

		while ( !q.IsEmpty() )
		{
			util::Point<util::uint> pt = q.PopBack();
			if(img->ValidPixel(pt.x, pt.y)&&img->GetPixel(pt.x, pt.y)>threshold)
			{	// 4 directional

				q.PushBack(util::Point<uint>(pt.x + 1, pt.y));
				q.PushBack(util::Point<uint>(pt.x - 1, pt.y));
				q.PushBack(util::Point<uint>(pt.x, pt.y + 1));
				q.PushBack(util::Point<uint>(pt.x, pt.y - 1));
				// Diagonals
				q.PushBack(util::Point<uint>(pt.x - 1, pt.y - 1));
				q.PushBack(util::Point<uint>(pt.x - 1, pt.y + 1));
				q.PushBack(util::Point<uint>(pt.x + 1, pt.y - 1));
				q.PushBack(util::Point<uint>(pt.x + 1, pt.y + 1));


				minX 		= (pt.x < minX ? pt.x : minX);
				minY 		= (pt.y < minY ? pt.y : minY);
				maxX 		= (pt.x > maxX ? pt.x : maxX);
				maxY 		= (pt.y > maxY ? pt.y : maxY);

				const byte px = img->GetPixel(pt.x, pt.y);
				// Setting new centroid.
				centroid.x = FindCentroid(centroid.x, intensity, pt.x, px);
				centroid.y = FindCentroid(centroid.y, intensity, pt.y, px);
				// Setting intensity.
				intensity += (uint) px;
				// Setting pixels
				pixels++;
				// Stops reading same px
				img->SetPixel(pt.x, pt.y, 0);
			}
		}
		if ( pixels == 0 )	width = height = 0;
		else
		{
			width = maxX - minX + 1;
			height = maxY - minY + 1;
		}
	}

	/**
	 * @brief			Finds the new center of mass in one dimention.
	 *
	 * @param centroid	The old center of mass.
	 * @param intense	The previous intensity.
	 * @param point		The new point position to add.
	 * @param weight	The weight of the new blob.
	 *
	 * @return			The new center of mass.
	 */

	decimal FindCentroid ( decimal centroid, uint intense,
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

};
}
