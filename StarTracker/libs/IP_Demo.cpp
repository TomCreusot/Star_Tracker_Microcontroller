/*
 *	File: 		IP_Test.cpp
 *	Author:		Tom Creusot
 *  Purpose:
 *				To visualy show the workings of the software.
 */

#include <iostream>
#include <chrono> // Get Time

#include "libs/ImageProcessing/ImageProcessing.hpp"
#include "libs/StarTracker/StarTracker.hpp"
#include "libs/Database/Database.hpp"
#include "libs/EasyBMP/EasyBMP.h"

int main ( int argc, char** argv )
{
	if ( argc == 4 )
	{
		Image img = ia:getImage(argv[1]);
		ip::adaptiveThreshold(img, atoi(argv[2]));
		cout << ip::getBlobs(img, atoi(argv[3])).size() << endl;

	}
	else
	{
		cout << "ERROR, Please Enter:"
			 << "\n\tThe image to read"
			 << "\n\tThe tolerance of the threshold"
			 << "\n\tThe max number of stars to display"
			 << endl;
	}


}
