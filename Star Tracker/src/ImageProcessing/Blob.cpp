#include "ImageProcessing.hpp"

namespace ip
{

	/**
	 * @brief Stops compiler complaining when creating a vector/array.
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
	 * @brief Decides if the pixel is within the bounds + the threshold distance.
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



	/// @return The width of the blob.
	int Blob::width ( )
	{
		return maxX - minX;
	}

	/// @return The height of the blob.
	int Blob::height ( )
	{
		return maxY - minY;
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
	 * @param img			The image to examine (will set all used pixels to 0).
	 * @param width			The width of the image.
	 * @param height		The height of the image.
	 * @param brightness	The brightness cut off.
	 */

	void Blob::spreadBlob ( byte** img, const int width, const int height, const int brightness )
	{
		intensity = 0;
		pixels = 0;
		// For adding all the pixels it may exceed the maximum for an integer.
		centroid = Point<int>();
		float x = 0;
		float y = 0;

		// Breadth first searching.
		queue<Point<int>> q;
		q.push(Point<int>(minX, minY));

		while ( !q.empty() )
		{
			Point<int> cur = q.front();
			q.pop();

			if ( pixelExist(img, width, height, brightness, cur.x + 1, cur.y) ) // Right
			{
				q.push(Point<int>(cur.x + 1, cur.y));
				maxX = max(maxX, cur.x + 1);
			}
			if ( pixelExist(img, width, height, brightness, cur.x - 1, cur.y) ) // Left
			{
				q.push(Point<int>(cur.x - 1, cur.y));
				minX = min(minX, cur.x - 1);
			}
			if ( pixelExist(img, width, height, brightness, cur.x, cur.y + 1) ) // Down
			{
				q.push(Point<int>(cur.x, cur.y + 1));
				maxY = max(maxY, cur.y + 1);
			}
			if ( pixelExist(img, width, height, brightness, cur.x, cur.y - 1) ) // Up
			{
				q.push(Point<int>(cur.x, cur.y - 1));
				minY = min(minY, cur.y - 1);
			}
			intensity += img[cur.y][cur.x];
			pixels++;
			x += cur.x;
			y += cur.y;
			img[cur.y][cur.x] = 0;
		}
		centroid.x = round(x / (float) pixels);
		centroid.y = round(y / (float) pixels);
	}




	/**
	 * @brief Tests the pixel position is within the bounds of the image and tests the brightness of the pixel is > brightness.
	 * @param img			The image to test.
	 * @param width			The width of the image.
	 * @param height		The height of the image.
	 * @param brightness 	The brighness to qualify.
	 * @param x				The x position to observe.
	 * @param y				The y position to observe.
	 * @return				If valid pixel.
	 */

	bool Blob::pixelExist ( byte** img, const int width, const int height, const int brightness, const int x, const int y )
	{
		return
			0 < x && 0 < y &&
			x < width && y < height &&
			img[y][x] > brightness;
	}







#ifdef DEBUG_IMAGE_PROCESSING
	/**
	 * @brief Like java toString(), except it prints it.
	 */

	void Blob::print ( )
	{
		cout << "Blob" << endl <<
		 		"minX: " << minX << ",\t maxX: " << maxX << ",\t width: " << maxX - minX << endl <<
				"minY: " << minY << ",\t maxY: " << maxY << ",\t height " << maxY - minY << endl <<
				"intensity: " << intensity << endl;
	}

#endif



}
