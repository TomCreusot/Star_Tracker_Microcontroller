#include "blob.h"
namespace image_processing
{


Blob::Blob					( )
{
	boundsMax = Point<uint>();
	boundsMin = Point<uint>();

	intensity = 0;
	pixels = 0;
	centroid = Point<decimal>(0, 0);
}




Blob::Blob					( uint x, uint y )
{
	boundsMin = Point<uint>(x, y);
	boundsMax = boundsMin;

	intensity = 0;
	pixels = 0;
	centroid = Point<decimal>((decimal)x, (decimal)y);
}




void Blob::ConsumePixel ( Point<uint>& pt, Image* img )
{
	boundsMin.x	= fmin(pt.x, boundsMin.x);
	boundsMin.y	= fmin(pt.y, boundsMin.y);
	boundsMax.x	= fmax(pt.x, boundsMax.x);
	boundsMax.y	= fmax(pt.y, boundsMax.y);

	const byte intense = img->GetPixel(pt.x, pt.y);
	centroid.x = FindCentroid(centroid.x, intensity, pt.x, intense);
	centroid.y = FindCentroid(centroid.y, intensity, pt.y, intense);

	// Setting intensity.
	intensity += (uint) img->GetPixel(pt.x, pt.y);
	// Setting pixels
	pixels++;
	// Stops reading same px
	img->SetPixel(pt.x, pt.y, 0);
}


decimal Blob::FindCentroid ( decimal centroid, uint intensity,
													uint point, byte weight )
{
	return ((centroid * (decimal)intensity) +
											((decimal)point * (decimal)weight))
										/ (decimal)(intensity + (decimal)weight);
}



bool Blob::SortByIntensity ( Blob& larger, Blob& smaller )
{
	return larger.intensity >= smaller.intensity;
}

bool Blob::SortByIntensityAscending ( Blob& smaller, Blob& larger )
{
	return larger.intensity >= smaller.intensity;
}


}
