/**
 *	File: 		demo.cc
 *	Author:		Tom Creusot
 *  Purpose:
 *				To generate a database which can be used as a c header.
 */

#include <iostream>
#include <chrono> // Get Time

#include "libs/star_tracker/star_set.h"
#include "libs/star_tracker/database.h"
#include "libs/nix/star.h"
#include "libs/nix/fill_template.h"
#include "libs/nix/linked_list.h"
#include "libs/util/util.h"
#include "libs/util/point.h"
#include "libs/util/array_list.h"

#include "config/database.h"

using namespace util;
using namespace star_tracker;
using namespace std;
using namespace nix;



int main ( int argc, char** argv )
{


	if ( argc == 2 )
	{

/**********************************************************************
*			Gets properties input file.
*/
		cout << "Aliasing Variables" << endl;
		string DATABASE_FILE 		= config::database_file;
		string TEMPLATE_FILE		= config::template_file;
		string OUTPUT_FILE	 		= config::out_file;


		util::uint C_MAG 			= config::mag_column;
		util::uint C_RA				= config::ra_column;
		util::uint C_DEC			= config::dec_column;
		util::decimal FOV 			= config::fov / 180.0 * M_PI;
		util::uint CUTOFF_MAG		= config::cutoff_mag;
		util::uint PILOT_SETS		= config::pilot_sets;


/**********************************************************************
 *			Reads the database and sorts by magnitude.
 */
		cout << "reading database file:..." << endl;

		util::LinkedList<Star> stars;
		Star::StarsFromCSV<0>(		DATABASE_FILE, CUTOFF_MAG,
									C_RA, C_DEC, C_MAG, &stars		);

		cout <<  "found: " << stars.Size() << " stars." << endl;
		cout << "sorting:..." << endl;
		stars.Sort(star_tracker::Star::SortByMagnitude);



/**********************************************************************
 *			Creates StarSets from the sorted list of sets.
 */
		cout << "\rgenerating sets:..." << endl;
		util::LinkedList<StarSet> sets;

		for ( uint i = 0; i < stars.Size(); i++ )
		{
			util::LinkedList<Point<decimal>> combinations;
			Star::FindCloseStars<0, 0>(i, PILOT_SETS, FOV, stars, &combinations);
			StarSet::GenerateSets<0, 0>(combinations, &sets);
			cout << "\r" << sets.Size() << std::flush;
		}
		cout << endl;



/**********************************************************************
 *			Sets up array string to fill out template.
 */
		cout << "generating string:..." << endl;
		LinkedList<string> list;
		while ( sets.Size() > 0 )
		{
			string str;
			str += "{";										// Start Array Row
			StarSet pop = sets.PopFront();
			Database::ToArrayString(pop, &str);				// Print Elements
			str += "}";										// End Array Row
			if ( sets.Size() > 0 ) str += ",\n";			// If not last row
			list.PushBack(str);
			cout << "\r" << sets.Size() << " remaining    " << std::flush;
		}
		cout << endl;
		string* str = LinkedList<string>::ListToString(list);



/**********************************************************************
 *			Fills a template file with the required variables.
 */
		nix::FillTemplate file_template;			// Setup Template
		file_template.AddKey("$(file)", OUTPUT_FILE);
		file_template.AddKey("$(num_elements)", to_string(list.Size()));
		file_template.AddKey("$(array_name)", "database_array");
		file_template.AddKey("$(array_elements)", *str);
		file_template.AddKey("$(fov)", to_string(FOV));

		file_template.ReplaceFile(TEMPLATE_FILE, OUTPUT_FILE);
	}
	else
	{
		cout << "ERROR, Please Enter the name of the properties file" << endl;
	}




}
