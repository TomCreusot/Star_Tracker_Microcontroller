#include "get_image.h"

namespace nix
{

GetImage::GetImage ( ){}

GetImage::GetImage ( string file )
{
	const char* c_str = file.c_str();
	bmp.ReadFromFile(c_str);
	cout << bmp.TellWidth() << "..." << endl;
}



void GetImage::GetBMP ( Image* img )
{
	img->SetWidthHeight(bmp.TellWidth(), bmp.TellHeight());


	for ( uint y = 0; y < img->GetHeight(); y++ )
	for ( uint x = 0; x < img->GetWidth(); x++ )
	{
		byte brightness = 	bmp(x, y) -> Red / 3 +
							bmp(x, y) -> Green / 3 +
							bmp(x, y) -> Blue / 3;
		img->SetPixel(x, y, brightness);
	}
}

void GetImage::SetBMP ( Image& img )
{
	bmp.SetSize(img.GetWidth(), img.GetHeight());
	for ( uint x = 0; x < img.GetWidth(); x++ )
		for ( uint y = 0; y < img.GetHeight(); y++ )
		{
			bmp(x, y) -> Red = img.GetPixel(x, y);
			bmp(x, y) -> Green = img.GetPixel(x, y);
			bmp(x, y) -> Blue = img.GetPixel(x, y);
		}
}




void GetImage::WriteImage	( string file )
{
	const char* c_str = file.c_str();
	bmp.WriteToFile(c_str);
}

}
