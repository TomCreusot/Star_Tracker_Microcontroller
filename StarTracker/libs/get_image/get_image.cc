#include "get_image.h"

namespace get_image
{

GetImage::GetImage ( char* file )
{
	bmp.ReadFromFile(file);
}


void GetImage::getImage ( Image& img )
{
	img.setWidthHeight(bmp.TellWidth(), bmp.TellHeight());


	for ( uint y = 0; y < img.getHeight(); y++ )
		for ( uint x = 0; x < img.getWidth(); x++ )
		{
			byte brightness = 	bmp(x, y) -> Red / 3 +
								bmp(x, y) -> Green / 3 +
								bmp(x, y) -> Blue / 3;
			img.setPixel(x, y, brightness);
		}
}



void GetImage::drawPoints ( ArrayList<Point<decimal>>& points,
														byte r, byte g, byte b )
{
	for ( uint i = 0; i < points.size(); i++ )
	{
		bmp(points[i].x, points[i].y)->Red = r;
		bmp(points[i].x, points[i].y)->Green = g;
		bmp(points[i].x, points[i].y)->Blue = b;
	}
}



void GetImage::copyToBMP ( Image& img )
{
	bmp.SetSize(img.getWidth(), img.getHeight());
	for ( uint x = 0; x < img.getWidth(); x++ )
   		for ( uint y = 0; y < img.getHeight(); y++ )
		{
			bmp(x, y) -> Red = img.getPixel(x, y);
			bmp(x, y) -> Green = img.getPixel(x, y);
			bmp(x, y) -> Blue = img.getPixel(x, y);
		}
}


void GetImage::writeImage	( char* file )
{
	bmp.WriteToFile(file);
}

}
