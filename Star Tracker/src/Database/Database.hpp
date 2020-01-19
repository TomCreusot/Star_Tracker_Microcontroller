/*
 *	File: 		Database.hpp
 *	Author:		Tom Creusot
 *  Purpose:
 *				Finds a specified value in a tree database
 									and returns the corresponding infomation.
 *
 *	Reference:
 *			../../Database
 *
 *	Header For: 	Database.cpp
 */

#ifndef DATABASE_HPP
#define DATABASE_HPP

#include "../ImageProcessing/ImageProcessing.hpp" // for Point and decimal


// SIZE_LEAF: The number of corresponding bytes in each node of the tree.

#ifdef COMPUTER
	#define SIZE_LEAF 1
	#include <fstream>
	#include <cstring>
	#include <string>
#else
	#define SIZE_LEAF 3
#endif


using namespace std;
using namespace ip;


namespace db
{
	// Refere to Database.cpp
	std::list<ip::Point<decimal>>* findInDatabase
						( string fileName, decimal angle, decimal tolerance );
}

#endif
