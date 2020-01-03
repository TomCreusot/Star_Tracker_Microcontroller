#include "ImageProcessing.hpp"


namespace ip
{
	/**
	 * @brief Finds all the blobs in an image with the specified parameters.
	 * @param img 		The image as an array.
	 * @param width 	The width of the image.
	 * @param height 	The height of the image.
	 * @param bright	The brightness cut off.
	 * @param dist		The distance cut off.
	 * @return 			The blobs THIS MUST BE DISPOSED OF!!!.
	 */

	std::list<Blob>* findBlobs ( byte** img, int width, int height, int bright, int dist )
	{
		std::list<Blob>* points = new std::list<Blob>();

		for ( int y = 0; y < height; y++ )
		{
			for ( int x = 0; x < width; x++ )
			{
				if ( img[y][x] > bright )
				{
					bool found = false;
					for ( std::list<Blob>::iterator it = points->begin();
													it != points->end() && !found; ++it )
					{
						if ( it -> withinThreshold(x, y, dist) )
						{
							it -> add(x, y);
							found = true;
						}
					}

					if ( points->empty() || !found )
					{
						Blob* blob = new Blob(x, y);
						points->push_back(*blob);
					}
				}
			}
		}
		return points;
	}










}
