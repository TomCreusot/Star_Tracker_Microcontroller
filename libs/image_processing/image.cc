#include "image.h"
namespace image_processing
{

Image::Image			( )
{
	// Sets all pixels to 0 incase of a resize and sets the size to 0.
	width = height = 0;
	for ( uint xx = 0; xx < kImageWidthMax; xx++ )
		for ( uint yy = 0; yy < kImageHeightMax; yy++ ) image[yy][xx] = 0;
}


Image::Image			( uint w, uint h )
{
	// Sets all pixels to 0 incase of a resize and sets the size to the specified size.
	width = w;
	height = h;
	for ( uint yy = 0; yy < kImageHeightMax; yy++ )
	{
		for ( uint xx = 0; xx < kImageWidthMax; xx++ )	image[yy][xx] = 0;
	}
}


Image::Image				( const Image& img )
{
	// Copies the image.
	width = img.GetWidth();
	height = img.GetHeight();

	for ( uint y = 0; y < height; y++ )
		for ( uint x = 0; x < width; x++ )
			SetPixel(x, y, img.GetPixel(x, y));
}



uint Image::GetWidth	( ) const { return width;  }
uint Image::GetHeight	( ) const { return height; }


byte Image::GetPixel	( uint x, uint y ) const		{ return image[y][x];  }
void Image::SetPixel	( uint x, uint y, byte color ) 	{ image[y][x] = color; }

bool Image::ValidPixel	( uint x, uint y ) const
{
	return x < GetWidth() && y < GetHeight();
}







void Image::SetWidthHeight ( uint w, uint h )
{
	// Sets the width and height of the image.
	if ( w <= kImageWidthMax && h <= kImageHeightMax )
	{
		width = w; height = h;
	}
}







void Image::FindMinMax ( uint x, uint y, uint sampleRadius,
														 byte* min, byte* max )
{
	// Finds the min and max intensity pixel in the given area.

	// Sets to a default pixel within the bounds.
	*min = GetPixel(x, y);
	*max = GetPixel(x, y);
	// Start and end position.
	uint sx = (x > sampleRadius ? x - sampleRadius : 0);
	uint sy = (y > sampleRadius ? y - sampleRadius : 0);
	uint ex = x + sampleRadius;
	uint ey = y + sampleRadius;

	for ( uint xx = sx; xx <= ex; xx++ )
	{
		for ( uint yy = sy; yy <= ey; yy++ )
		{
			if ( ValidPixel(xx, yy) )
			{
				if ( GetPixel(xx, yy) < *min ) 		*min = GetPixel(xx, yy);
				else if ( GetPixel(xx, yy) > *max )	*max = GetPixel(xx, yy);
			}
		}
	}
}



void Image::AdaptiveThreshold ( uint sampleRadius, float aggression )
{
	Image outOfPlace(*this);

	for ( uint y = 0; y < GetHeight(); y++ )
	{
		for ( uint x = 0; x < GetWidth(); x++ )
		{
			// Taking the area around the pixel.
			byte min, max;
			outOfPlace.FindMinMax(x, y, sampleRadius, &min, &max);

			// min/max-mean method.
			int intensity = min * (1 - aggression) + max * (aggression);
			if ( GetPixel(x, y) < intensity || intensity == 0)
			{
				SetPixel(x, y, 0);
			}
		}
	}
}


}
