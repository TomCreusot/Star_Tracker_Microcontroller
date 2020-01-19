#include "UnitTest.hpp"


/**
 * @brief prints an easy to read header.
 * @param fileName The name of the file.
 * @param testFileName The name of the testFile.
 */

void printHeader ( string fileName, string testFileName )
{
	cout << endl << endl << "-------  " <<
	 	fileName << " | " << testFileName << "  ------------------" << endl;
}



/**
 * @brief Prints a readable unit test id and if passed or not.
 * @param test The function.
 * @param pass If the test passed.
 * @return pass.
 */

bool printPass ( string test, bool pass )
{
	cout << test << ":" << endl;
	if ( pass )	cout << ANSI_WHITE_BACKGROUND << ANSI_GREEN << "PASSED" << ANSI_RESET << endl;
	else		cout << ANSI_RED_BACKGROUND << ANSI_WHITE << "FAILED" << ANSI_RESET << endl;
	cout << endl;
	return pass;
}



/**
* @brief Eases comparisons.
* @param s A double.
* @param d A double.
* @return true if equal.
*/

bool equal ( decimal d1, decimal d2 )
{
	return abs(d1 - d2) < 0.001d;
}
