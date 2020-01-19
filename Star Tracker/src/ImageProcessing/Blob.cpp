/*
 *	File: 		Blob.cpp
 *	Author:		Tom Creusot
 *  Purpose:
 *				Stores the functions in charge of the wildfire blob detection.
 *
 *	Ideal calls:
 *				Initialize with the alternate constructor and call spreadBlob().
 *
 *	Reference:
 *				Uses simmilar logic to:
 *				http://what-when-how.com/
 				introduction-to-video-and-image-processing/
				blob-analysis-introduction-to-video-and-image-processing-part-1/
 *
 *	Note:
 *				Calling spreadBlob will damage the image,
 				if you want to reuse it, make a copy first.
 *
 * File For: 	ImageProcessing.hpp.
 */


#include "ImageProcessing.hpp"

namespace ip
{

	/**
	 * @brief Stops compiler complaining when creating a vector/array.
	 * Creates a blob with every value being 0.
	 */

	 Blob::Blob ( )
	 {
		 Blob::minX = 0;
		 Blob::maxX = 0;
		 Blob::maxY = 0;
		 Blob::minY = 0;
		 pixels = 0;
		 centroid.x = 0;
		 centroid.y = 0;
		 intensity = 0;
	 }


	/**
	 * @brief Creates a blob at the position provided.
	 * @param x The initial x position.
	 * @param y The initial y position.
	 */

	Blob::Blob ( int x, int y )
	{
		Blob::minX = x;
		Blob::minY = y;
		Blob::maxX = x;
		Blob::maxY = y;
		pixels = 1;
		centroid.x = x;
		centroid.y = y;
		intensity = 0;
	}



	/**
	 * @brief Decides if the pixel is within the bounds and threshold distance.
	 * @param x The pixel x to test.
	 * @param y The pixel y to test.
	 * @return	If the pixel is close to the blob.
	 */

	bool Blob::withinThreshold ( int x, int y, int distT )
	{
		return
			( ( Blob::minX - distT <= x && x <= Blob::maxX + distT ) &&
			  ( Blob::minY - distT <= y && y <= Blob::maxY + distT ) );
	}



	/// @return The width of the blob. WHEN ALL BOUNDS ON 0, THIS WILL BE 1.
	int Blob::width ( )
	{
		return maxX - minX + 1;
	}

	/// @return The height of the blob. WHEN ALL BOUNDS ON 0, THIS WILL BE 1.
	int Blob::height ( )
	{
		return maxY - minY + 1;
	}

	/// @return The center of min/maxX
	int Blob::roughX ( )
	{
		return (maxX + minX) / 2;
	}

	/// @return The center of min/maxX
	int Blob::roughY ( )
	{
		return ( maxY + minY ) / 2;
	}




	/**
	 * @brief Uses the grass fire method to find the true bounds of the blob.
	 * Sets any used pixels to 0.
	 *
	 * @param img			The image to examine SETS ALL USED PIXELS TO 0.
	 * @param width			The width of the image.
	 * @param height		The height of the image.
	 * @param brightness	The brightness cut off.
	 */

	void Blob::spreadBlob ( byte** img, int width, int height, int brightness )
	{
		intensity = (uint)img[minY][minX];
		pixels = 0;
		centroid.x = 0;
		centroid.y = 0;
		float x = 0;
		float y = 0;

		// Breadth first searching.
		queue<Point<int>> q;
		q.push(Point<int>(minX, minY));

		while ( !q.empty() )
		{
			Point<int> cur = q.front();
			q.pop();
			if (img[cur.y][cur.x] != 0) // if previously used?
			{
				if ( pixelExist(
							img, width, height, brightness, cur.x + 1, cur.y) )
				{ // Right
					q.push(Point<int>(cur.x + 1, cur.y));
					maxX = max(maxX, cur.x + 1);
				}
				if ( pixelExist(
							img, width, height, brightness, cur.x - 1, cur.y) )
				{ // Left
					q.push(Point<int>(cur.x - 1, cur.y));
					minX = min(minX, cur.x - 1);
				}
				if ( pixelExist(
							img, width, height, brightness, cur.x, cur.y + 1) )
				{ // Down
					q.push(Point<int>(cur.x, cur.y + 1));
					maxY = max(maxY, cur.y + 1);
				}
				if ( pixelExist(
							img, width, height, brightness, cur.x, cur.y - 1) )
				{ // Up
					q.push(Point<int>(cur.x, cur.y - 1));
					minY = min(minY, cur.y - 1);
				}

				intensity += (uint)img[cur.y][cur.x];
				img[cur.y][cur.x] = 0;
				pixels++;
				x += cur.x;
				y += cur.y;
			}
		}
		centroid.x = x / (float) pixels;
		centroid.y = y / (float) pixels;
	}




	/**
	 * @brief Tests the pixel position is within the bounds of the image
	 					and tests the brightness of the pixel is > brightness.
	 * @param img			The image to test.
	 * @param width			The width of the image.
	 * @param height		The height of the image.
	 * @param brightness 	The brighness to qualify.
	 * @param x				The x position to observe.
	 * @param y				The y position to observe.
	 * @return				If valid pixel.
	 */

	bool Blob::pixelExist ( byte** img, int width, int height,
												int brightness, int x, int y )
	{
		return
			0 <= x && 0 <= y &&
			x < width && y < height &&
			img[y][x] >= brightness;
	}







#ifdef COMPUTER
	/**
	 * @brief Like java toString(), except it prints it.
	 */

	void Blob::print ( )
	{
		cout << "Blob" << endl <<
				"minX: " << minX << ",\t maxX: " << maxX
										<< ",\t width: " << width() << endl <<
				"minY: " << minY << ",\t maxY: " << maxY
										<< ",\t height " << height() << endl <<
				"intensity: " << intensity << ",\t pixels: " << pixels << endl
				<< "centroid: ";
				centroid.print();
				cout << "\t\trough x: " + roughX() << "\trough y: " << roughY();
	}

#endif



}
