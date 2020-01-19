#include "UnitTest.hpp"

int main ()
{
	int numTests = 0;
	int testsPassed = 0;


	testsPassed += stT::run();
	numTests++;

	testsPassed += ipT::run();
	numTests++;


	cout << endl << endl
		 << testsPassed << " of " << numTests << " passed" << endl << endl;
	return 0;
}
