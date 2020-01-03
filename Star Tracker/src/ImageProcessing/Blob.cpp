#include "ImageProcessing.hpp"

namespace ip
{
	/**
	 * @brief Creates a blob at the position provided.
	 */

	Blob::Blob ( int x, int y )
	{
		Blob::minX = x;
		Blob::minY = y;
		Blob::maxX = x;
		Blob::maxY = y;
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
			(Blob::minX - distT <= x && x <= Blob::maxX + distT ) &&
			(Blob::minY - distT <= y && y <= Blob::maxY + distT );
	}




	/**
	 * @brief 	 Sets the bounds by adding the current pixel.
	 * @param x The pixel x to add.
	 * @param y The pixel y to add.
	 */

	void Blob::add ( int x, int y )
	{
		Blob::minX = min(x, Blob::minX);
		Blob::minY = min(y, Blob::minY);
		Blob::maxX = max(x, Blob::maxX);
		Blob::maxY = max(y, Blob::maxY);
	}


}
