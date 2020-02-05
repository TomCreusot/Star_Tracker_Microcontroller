#include "image_impl.hpp"
namespace ip
{
ImageEmbedded::ImageEmbedded	( )
{
	for ( int yy = 0; yy < IMAGE_HEIGHT; yy++ )
		for ( int xx = 0; xx < IMAGE_WIDTH; xx++ )
			img[yy][xx] = 0;
}

ImageEmbedded::ImageEmbedded	( ImageEmbedded& image )
{
	for ( int yy = 0; yy < IMAGE_HEIGHT; yy++ )
		for ( int xx = 0; xx < IMAGE_WIDTH; xx++ )
			img[yy][xx] = image.getPixel(xx, yy);
}


uint ImageEmbedded::getWidth  	( ) { return IMAGE_WIDTH;  }
uint ImageEmbedded::getHeight 	( ) { return IMAGE_HEIGHT; }


byte ImageEmbedded::getPixel	( uint x, uint y ) { return img[y][x]; }

void ImageEmbedded::setPixel	( uint x, uint y, byte color )
														{ img[y][x] = color; }
}
