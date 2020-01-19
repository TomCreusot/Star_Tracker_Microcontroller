#ifndef UNIT_TEST_HPP
#define UNIT_TEST_HPP

#include "../src/Database/Database.hpp"
#include "../src/ImageProcessing/ImageProcessing.hpp"
#include "../src/StarTracker/StarTracker.hpp"

//Unix color codes
static const string ANSI_RESET = "\u001B[0m";
static const string ANSI_RED = "\u001B[31m";
static const string ANSI_GREEN = "\u001B[32m";
static const string ANSI_WHITE = "\u001B[37m";

static const string ANSI_BLACK_BACKGROUND = "\u001B[40m";
static const string ANSI_RED_BACKGROUND = "\u001B[41m";
static const string ANSI_WHITE_BACKGROUND = "\u001B[47m";




// UnitTest.cpp
void printHeader ( string fileName, string testFileName );
bool printPass ( string test, bool pass );
bool equal ( decimal d1, decimal d2 );



// StarTracker.Test.cpp
namespace stT
{
	bool run ( );
	bool testFindAngleValid ( );
	bool testFindAngleAllSame ( );
	bool testFindAngleAllInLine ( );
}


// ImageProcessing.Test.cpp
namespace ipT
{
	bool run ( );
	bool testBlobWithinThreshold ( );
	bool testBlobDimentions ( );
	bool testBlobSpreadBlob ( );
	bool testBlobPixelExists ( );


	bool testPointSet ( );
	bool testPointEqual ( );
	bool testPointDistance ( );
}


#endif
