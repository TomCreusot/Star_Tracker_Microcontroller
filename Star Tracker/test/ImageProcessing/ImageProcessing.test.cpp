#include <iostream>

#include "../../src/ImageProcessing/ImageProcessing.hpp"
#include <iostream>
#include "../../src/EasyBMP/EasyBMP.h"
using namespace std;
using namespace ip;

int main()
{
	BMP img;
	img.ReadFromFile("Test.bmp");

	byte** im = new byte*[img.TellHeight()];

	for ( int y = 0; y < img.TellHeight(); y++ )
	{
		im[y] = new byte[img.TellWidth()];
		for (int x = 0; x < img.TellWidth(); x++ )
		{
			im[y][x] = img(x, y) -> brightness();
		}
	}

	std::list<Blobs>* blobs = ip::findBlobs(im, img.TellWidth(), img.TellHeight(), 10, 1);

	cout << "Found: " << blobs->size() << " blobs." << endl;



	for ( int y = 0; y < img.TellHeight(); y++ )
	{
		im[y] = new byte[img.TellWidth()];
		for (int x = 0; x < img.TellWidth(); x++ )
		{
			for ( std::list<Blob>::iterator it = points->begin();
											it != points->end() && !found; ++it )
			{
				if ( it -> withinThreshold(x, y, dist) )
				{
					img(x, y) -> Red = 255;
					img(x, y) -> Green = 0;
					img(x, y) -> Blue = 0;
				}
			}
		}
	}



	img.WriteToFile("test_output.bmp");


	return 0
}
