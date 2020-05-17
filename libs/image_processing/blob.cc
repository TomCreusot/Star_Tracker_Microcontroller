#include "blob.h"
namespace image_processing
{


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






decimal Blob::FindCentroid ( decimal centroid, uint intense,
													uint point, byte weight )
{
	return ((centroid * (decimal)intense) + ((decimal)point * (decimal)weight))
										/ (decimal)(intense + (decimal)weight);
}



bool Blob::SortByIntensity ( Blob& larger, Blob& smaller )
{
	return larger.intensity >= smaller.intensity;
}


}
