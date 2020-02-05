#include "image_impl.hpp"
namespace ip
{

ImageNix::ImageNix			( )
{
	img = vector< vector < byte > >( 1 );
	img[0] = vector< byte >( 1 );
}


ImageNix::ImageNix			( uint w, uint h )
{
	uint width = w;
	uint height = h;
	img = vector< vector < byte > >(height);
	for ( uint yy = 0; yy < height; yy++ )
	{
		img[yy] = vector< byte >(width);
		for ( uint xx = 0; xx < width; xx++ )	img[yy][xx] = 0;
	}
}


ImageNix::ImageNix			( ImageNix& image )
{
	uint width = image.getWidth();
	uint height = image.getHeight();
	img = vector< vector < byte > >(height);
	for ( uint y = 0; y < height; y++ )
	{
		img[y] = vector<byte>(width);
		for ( uint x = 0; x < width; x++ )	img[y][x] = image.getPixel(x, y);
	}
}

uint ImageNix::getWidth		( ) { return img[0].size();  }
uint ImageNix::getHeight	( ) { return img.size(); }


byte ImageNix::getPixel		( uint x, uint y ) 				{ return img[y][x];}
void ImageNix::setPixel		( uint x, uint y, byte color ) 	{img[y][x] = color;}
}
