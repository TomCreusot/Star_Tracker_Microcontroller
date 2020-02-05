#include "blob.hpp"
namespace ip
{


void Blob::findBlobs ( ImageInterface& img, byte threshold, ArrayList<Blob>& list )
{
	for ( uint xx = 0; xx < img.getWidth(); xx++ )
	{
		for ( uint yy = 0; yy < img.getHeight(); yy++ )
		{
			if ( img.getPixel(xx, yy) > threshold )
			{
				Blob blob(xx, yy);
				blob.spreadGrassFire(img, threshold);
				list.push_back(blob);
			}
		}
	}
}




Blob::Blob					( )
{
	origin = util::Point<uint>(0, 0);

	width = 0;
	height = 0;
	intensity = 0;
	pixels = 0;
	centroid = util::Point<decimal>(0, 0);
}


Blob::Blob					( uint x, uint y )
{
	origin = util::Point<uint>(x, y);

	width = 0;
	height = 0;
	intensity = 0;
	pixels = 0;
	centroid = util::Point<decimal>((decimal)x, (decimal)y);
}


uint	Blob::getWidth		( )	{	return width;	}
uint	Blob::getHeight		( )	{	return height;	}

uint 	Blob::getPixels		( )	{	return pixels;		}
uint 	Blob::getIntensity	( )	{	return intensity;	}
decimal	Blob::getCentroidX	( )	{	return centroid.x;	}
decimal	Blob::getCentroidY	( ) {	return centroid.y;	}



void 	Blob::spreadGrassFire ( ImageInterface& img, uint intense )
{
	std::queue<util::Point<uint>> q;
	uint minX = round(centroid.x), minY = round(centroid.y);
	uint maxX = minX, maxY = minY;

	q.push(Point<uint>(minX, minY));

	while ( !q.empty() )
	{
		util::Point<uint> pt = q.front();
		q.pop();

		if( img.validPixel(pt.x, pt.y) && img.getPixel(pt.x, pt.y) > intense )
		{	// 4 directional
			q.push(util::Point<uint>(pt.x + 1, pt.y));
			q.push(util::Point<uint>(pt.x - 1, pt.y));
			q.push(util::Point<uint>(pt.x, pt.y + 1));
			q.push(util::Point<uint>(pt.x, pt.y - 1));
			// Diagonals
			q.push(util::Point<uint>(pt.x - 1, pt.y - 1));
			q.push(util::Point<uint>(pt.x - 1, pt.y + 1));
			q.push(util::Point<uint>(pt.x + 1, pt.y - 1));
			q.push(util::Point<uint>(pt.x + 1, pt.y + 1));

			minX 		= std::min(pt.x, minX );
			minY 		= std::min(pt.y, minY );
			maxX 		= std::max(pt.x, maxX );
			maxY 		= std::max(pt.y, maxY );

			const byte px = img.getPixel(pt.x, pt.y);
			// Setting new centroid.
			centroid.x = findCentroid(centroid.x, intensity, pt.x, px);
			centroid.y = findCentroid(centroid.y, intensity, pt.y, px);
			// Setting intensity.
			intensity += (uint) px;
			// Setting pixels
			pixels++;
			// Stops reading same px
			img.setPixel(pt.x, pt.y, 0);
		}
	}
	if ( pixels == 0 )	width = height = 0;
	else
	{
		width = maxX - minX + 1;
		height = maxY - minY + 1;
	}
}



decimal Blob::findCentroid ( decimal centroid, uint intense,
													uint point, byte weight )
{
	return ((centroid * (decimal)intense) + ((decimal)point * (decimal)weight))
										/ (decimal)(intense + (decimal)weight);
}

}
