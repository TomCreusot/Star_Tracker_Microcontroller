/*
 *	File: 		Database.cpp
 *	Author:		Tom Creusot
 *  Purpose:
 *				Finds a specified value in a tree database
 									and returns the corresponding infomation.
 *
 *	Reference:
 *			../../Database
 *
 *  Ideal Calls: findInDatabase().
 *
 *  cpp For: Database.hpp
 */

#include "Database.hpp"
namespace db
{



// These are file private
int getChildLeftIndex ( int parent );
int getChildRightIndex ( int parent );




#ifdef COMPUTER

/**
 * @brief Searches a csv database in LINEAR order to find close matches.
 *
 * @param file 		The name of the file.
 * @param angle 	The angle to search for.
 * @param tolerance The offset the angle is alowed to be.
 * @return 			The list of valid angles.
 */

std::list<ip::Point<decimal>>* findInDatabase
						( string fileName, decimal angle, decimal tolerance )
{
	std::list<ip::Point<decimal>>* origins =
										new std::list<ip::Point<decimal>>();

	std::fstream file;
	file.open(fileName, std::fstream::in);


	while ( !file.eof() )
	{
		char* line = new char[200];
		file.getline(line, 200);
		decimal angleD = atof(std::strtok(line, ","));
		if ( abs(angle - angleD) < tolerance )
		{
			ip::Point<decimal>* point = new ip::Point<decimal>();
			point -> x = atof(std::strtok(NULL, ","));
			point -> y = atof(std::strtok(NULL, ","));
			origins -> push_back(*point);
		}
	//	else cout << "nup" << endl;
	}
	return origins;
}

/*
std::vector<ip::Point>& findInDatabase ( db::decimal angle, db::decimal tolerance )
{
	vector<ip::Point> points;
	vector<string> lines; // Using a computer, it can store the whole list.

	fstream file;
	file.open("", std::fstream::in);

	int index = 0;
	while ( !file.eof )
	{
		readLine(lines[index], 1000);
		index++;
	}

	moveToNode (potential, )
	file.close();
}

void moveToNode ( std::vector<ip::Point>& lines, std::vector<ip::Point>& potential, int position )
{
	if ( atof(lines) )

	moveToNode(lines, potential, );
}
*/

#else
/*
std::vector<ip::Point>& findInDatabase ( db::decimal angle, db::decimal tolerance )
{




}


*/
#endif





/**
 * Gets the index of the child to the left.
 *
 * @param parent The current index.
 * @return The index of the parents left child.
 */

int getChildLeftIndex ( int parent )
{
	return parent * 2 + (1 * SIZE_LEAF);
}


/**
 * Gets the index of the child to the right.
 * @param parent The current index.
 * @return The index of the parents right child.
 */

int getChildRightIndex ( int parent )
{
	return parent * 2 + (2 * SIZE_LEAF);
}




}
