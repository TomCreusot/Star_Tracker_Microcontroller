#include <iostream>
#include <chrono> // Get Time

#include "../src/ImageProcessing/ImageProcessing.hpp"
#include "../src/EasyBMP/EasyBMP.h"

using namespace std;
using namespace ip;

uint getMillis ( );

int main ( int argc, char** argv )
{
	if (argc > 2)
	{
		BMP img;
		img.ReadFromFile(argv[1]);

		int time = getMillis();
		byte** im = ip::bmpToArray(img);

		time = getMillis() - time;
		cout << "Created array in: " << time << "ms" << endl;
		time = getMillis();



		//byte threshold = percentThreshold(im, img.TellWidth(), img.TellHeight(), 255, 0.997f);

		adaptiveThreshold(im, img.TellWidth(), img.TellHeight(), 5, 0.9f); byte threshold = 20;

		byte** im2 = new byte*[img.TellHeight()];
		for ( int y = 0; y < img.TellHeight(); y++ )
		{
			im2[y] = new byte[img.TellWidth()];
			for ( int x = 0; x < img.TellWidth(); x++ )
			{
				im2[y][x] = im[y][x];
			}
		}
		time = getMillis() - time;
		cout << "Found threshold in: " << time << "ms" << endl;
		time = getMillis();


		std::list<Blob>* blobs = ip::findBlobs(im, img.TellWidth(), img.TellHeight(), threshold);

		time = getMillis() - time;
		cout << "Found: " << blobs -> size() << " blobs in: " << time << "ms" << endl;
		time = getMillis();


		int numBlobs = atoi(argv[2]);
		ip::Blob* points = ip::getMainPoints(*blobs, numBlobs);
		time = getMillis() - time;
		cout << "Sorted: " << numBlobs << " blobs in: " << time << "ms" << endl;

	//	points = ip::listToArray(blobs);numBlobs = blobs->size();


		delete blobs;



		BMP* bmp = ip::combineImages(img, im2, img.TellWidth(), img.TellHeight());

		for ( int i = 0; i < numBlobs; i++ )
		{
			 (*bmp)(img.TellWidth() + points[i].centroid.x, points[i].centroid.y) -> Blue = 200;
			 (*bmp)(img.TellWidth() + points[i].centroid.x, points[i].centroid.y) -> Red = 255;
			 (*bmp)(img.TellWidth() + points[i].centroid.x, points[i].centroid.y) -> Green = 0;
			(*bmp)(points[i].centroid.x, points[i].centroid.y) -> Red = 255 - (200 * i / numBlobs);
			(*bmp)(points[i].centroid.x, points[i].centroid.y) -> Blue = 0;
			(*bmp)(points[i].centroid.x, points[i].centroid.y) -> Green = 0;
		}

		bmp -> WriteToFile("test_output.bmp");
		delete bmp;

		for ( int y = 0; y < img.TellHeight(); y++ )
		{
			delete[] im[y];
		}
		delete[] im;
		delete[] points;
	}
	else
	{
		std::cout << "To test, execute with a file name and the number of blobs to sample." << endl << endl;
	}


	return 0;
}



uint getMillis ( )
{
	return std::chrono::duration_cast<std::chrono::milliseconds>(
		std::chrono::system_clock::now().time_since_epoch()
	).count();
}
