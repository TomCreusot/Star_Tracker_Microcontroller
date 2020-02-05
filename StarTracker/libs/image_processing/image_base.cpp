#include "image_base.hpp"
namespace ip
{

bool ImageBase::validPixel	( uint x, uint y )
{
	return x < getWidth() && y < getHeight();
}



void ImageBase::findMinMax ( uint x, uint y, uint sampleRadius, byte& min, byte& max )
{
	min = getPixel(x, y);
	max = getPixel(x, y);
	int tx = x - sampleRadius;
	int ty = y - sampleRadius;
	for ( uint xx = (tx > 0 ? (uint)tx : 0); xx <= x + sampleRadius; xx++ )
	{

		for ( uint yy = (ty > 0 ? (uint)ty : 0); yy <= y + sampleRadius; yy++ )
		{
			if ( validPixel(xx, yy) )
			{
				if ( getPixel(xx, yy) < min ) min = getPixel(xx, yy);
				if ( getPixel(xx, yy) > max ) max = getPixel(xx, yy);
			}
		}
	}
}



void ImageBase::adaptiveThreshold ( uint sampleRadius, float aggression )
{
	decltype(*this) outOfPlace(*this);

	for ( uint x = 0; x < getWidth(); x++ )
	{
		for ( uint y = 0; y < getHeight(); y++ )
		{
			// Taking the area around the pixel.
			int intensity = 0;
			byte min, max;
			findMinMax(x, y, sampleRadius, min, max);

			// Mean method.
			intensity = (float) min * (1 - aggression) + (float) max * (aggression);
			if ( outOfPlace.getPixel(x, y) < intensity || intensity == 0)
			{
				setPixel(x, y, 0);
			}
		}
	}
}
}
