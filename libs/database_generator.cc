/**
 *	File: 		demo.cc
 *	Author:		Tom Creusot
 *  Purpose:
 *				To generate a database which can be used as a c header.
 */

#include <iostream>
#include <chrono> // Get Time

#include "libs/star_tracker/star_set.h"
#include "libs/nix/star.h"
#include "libs/nix/properties.h"
#include "libs/nix/fill_template.h"
#include "libs/nix/linked_list.h"
#include "libs/util/util.h"
#include "libs/util/point.h"
#include "libs/util/array_list.h"

using namespace util;
using namespace star_tracker;
using namespace std;
using namespace nix;


int main ( int argc, char** argv )
{


	if ( argc == 2 )
	{
		cout << "reading properties file:..." << endl;
		Properties p;
		p.ReadFile(argv[1]);
		string DATABASE_FILE 		= p.GetString("database_file");
		string TEMPLATE_FILE		= p.GetString("template_file");
		string OUTPUT_FILE	 	= p.GetString("template_output_file");
		string OUTPUT_FILE_DIR		= p.GetString("template_output_directory");


		util::uint C_MAG 			= p.GetInteger("mag_column");
		util::uint C_RA				= p.GetDecimal("ra_column");
		util::uint C_DEC			= p.GetDecimal("dec_column");
		util::uint FOV 				= p.GetInteger("fov");
		util::uint CUTOFF_MAG		= p.GetInteger("cutoff_mag");
		util::uint PILOT_SETS		= p.GetInteger("pilot_sets");

		StarSet::set_pixel_resolution(1);
		StarSet::set_fov(FOV);

		cout << "reading database file:..." << std::flush;

		util::LinkedList<Star> stars;
		Star::StarsFromCSV<0>(		DATABASE_FILE, CUTOFF_MAG,
									C_RA, C_DEC, C_MAG,
									&stars	);

		cout <<  "\tfound: " << stars.Size()  << endl;
		cout << "sorting:..." << endl;
		stars.Sort(star_tracker::Star::SortByMagnitude);

		cout << "\rgenerating sets:..." << endl;
		util::LinkedList<StarSet> sets;

		for ( uint i = 0; i < stars.Size(); i++ )
		{
			util::LinkedList<Point<decimal>> combinations;
			Star::FindCloseStars<0, 0>(i, PILOT_SETS, FOV, stars, &combinations);
			StarSet::GenerateSetsPilots<0, 0>(combinations, 0, combinations.Size(), &sets);
			cout << "\r" << sets.Size() << std::flush;
		}
		cout << endl;

		/*while ( !sets.IsEmpty() )
		{
			string str;
			sets.PopFront().ToArrayString(&str);
			cout << str << endl;
		}*/


		cout << "generating string:..." << endl;
		LinkedList<string> list;
		while ( sets.Size() > 0 )
		{
			string str;
			str += "{";
			sets.PopFront().ToArrayString(&str);
			str += "},\n";
			list.PushBack(str);
			cout << "\r" << sets.Size() << " remaining" << std::flush;
		}
		cout << endl;
		string* str = LinkedList<string>::ListToString(list);

		nix::FillTemplate file_template;
		file_template.AddKey("$(file)", OUTPUT_FILE);
		file_template.AddKey("$(num_elements)", to_string(list.Size()));
		file_template.AddKey("$(array_name)", "database_array");
		file_template.AddKey("$(array_elements)", *str);


		file_template.ReplaceFile(TEMPLATE_FILE, OUTPUT_FILE_DIR+OUTPUT_FILE);

	}
	else
	{
		cout << "ERROR, Please Enter the name of the properties file" << endl;
	}




}
