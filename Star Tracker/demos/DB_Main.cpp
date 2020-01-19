#include <iostream>
#include <chrono> // Get Time

#include "../src/ImageProcessing/ImageProcessing.hpp"
#include "../src/StarTracker/StarTracker.hpp"
#include "../src/Database/Database.hpp"
#include "../src/EasyBMP/EasyBMP.h"

using namespace std;
using namespace ip;
using namespace db;

uint getMillis ( );

int main ( int argc, char** argv )
{
	if (argc > 2)
	{
		BMP img;
		img.ReadFromFile(argv[1]);

		byte** im = ip::bmpToArray(img);

		ip::adaptiveThreshold(im, img.TellWidth(), img.TellHeight(), 10, 0.9f); byte threshold = 2;
		std::list<Blob>* blobs = ip::findBlobs(im, img.TellWidth(), img.TellHeight(), threshold);

		int numBlobs = 4;
		ip::Blob* points = ip::getMainPoints(*blobs, numBlobs);

		list<decimal> listAngles = st::pilotAngles(numBlobs, points, atoi(argv[2]));


		while ( listAngles.size() > 0 )
		{
			db::findInDatabase("database.csv", listAngles.back(), 0.0003);
			cout << listAngles.back() << endl;
			listAngles.pop_front();
		}




		// (Put deletes here)
	}
	else
	{
		std::cout << "To test, execute with a file name and the number of pilots to use." << endl << endl;
	}


	return 0;
}



uint getMillis ( )
{
	return std::chrono::duration_cast<std::chrono::milliseconds>(
		std::chrono::system_clock::now().time_since_epoch()
	).count();
}
