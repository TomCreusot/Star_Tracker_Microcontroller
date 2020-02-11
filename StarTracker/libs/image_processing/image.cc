#include "image.h"
namespace image_processing
{

Image::Image			( )
{
	width = height = 0;
	for ( uint xx = 0; xx < IMAGE_WIDTH_MAX; xx++ )
		for ( uint yy = 0; yy < IMAGE_HEIGHT_MAX; yy++ ) image[yy][xx] = 0;
}


Image::Image			( uint w, uint h )
{
	width = w;
	height = h;
	for ( uint yy = 0; yy < IMAGE_HEIGHT_MAX; yy++ )
	{
		for ( uint xx = 0; xx < IMAGE_WIDTH_MAX; xx++ )	image[yy][xx] = 0;
	}
}


Image::Image				( Image& img )
{
	width = img.getWidth();
	height = img.getHeight();

	for ( uint y = 0; y < height; y++ )
		for ( uint x = 0; x < width; x++ )
			setPixel(x, y, img.getPixel(x, y));
}



uint Image::getWidth	( ) { return width;  }
uint Image::getHeight	( ) { return height; }


byte Image::getPixel	( uint x, uint y ) 				{ return image[y][x];  }
void Image::setPixel	( uint x, uint y, byte color ) 	{ image[y][x] = color; }


void Image::setWidthHeight ( uint w, uint h )
{
	if ( w <= IMAGE_WIDTH_MAX && h <= IMAGE_HEIGHT_MAX )
	{
		width = w; height = h;
	}
}














bool Image::validPixel	( uint x, uint y )
{
	return x < getWidth() && y < getHeight();
}



void Image::findMinMax ( uint x, uint y, uint sampleRadius,
														 byte& min, byte& max )
{
	min = getPixel(x, y);
	max = getPixel(x, y);
	uint sx = (x > sampleRadius ? x - sampleRadius : 0);
	uint sy = (y > sampleRadius ? y - sampleRadius : 0);
	uint ex = x + sampleRadius;
	uint ey = y + sampleRadius;

	for ( uint xx = sx; xx <= ex; xx++ )
	{
		for ( uint yy = sy; yy <= ey; yy++ )
		{
			if ( validPixel(xx, yy) )
			{
				if ( getPixel(xx, yy) < min ) min = getPixel(xx, yy);
				else if ( getPixel(xx, yy) > max ) max = getPixel(xx, yy);
			}
		}
	}
}



void Image::adaptiveThreshold ( uint sampleRadius, float aggression )
{
	Image outOfPlace(*this);

	for ( uint y = 0; y < getHeight(); y++ )
	{
		for ( uint x = 0; x < getWidth(); x++ )
		{
			// Taking the area around the pixel.
			byte min, max;
			outOfPlace.findMinMax(x, y, sampleRadius, min, max);

			// min/max-mean method.
			int intensity = min * (1 - aggression) + max * (aggression);
			if ( getPixel(x, y) < intensity || intensity == 0)
			{
				setPixel(x, y, 0);
			}
		}
	}
}





}
