#pragma once
#include <string>
#include "libs/util/util.h"


namespace config
{
/// The field of view of the camera
const decimal	fov				= 10;


/// The column (starting at 0) of the apparant magnitude in the origional hip database.
const uint		mag_column		= 13;
/// The column (starting at 0) of the right ascention (2000) in the origional hip database.
const uint		ra_column		= 7;
/// The column (starting at 0) of the declination (2000) in the origional hip database.
const uint		dec_column		= 8;


/// Any value above this magnitude will not be used in the calculations.
const uint		cutoff_mag		= 4;
/// The number of stars to consider for each pilot.
const uint		pilot_sets		= 7;




/// The file to read from for the database (csv).
const string	database_file	= "hygdata_v3.csv";

/// The file template to fill out with the database.
const string	template_file	= "../libs/star_tracker/database_template.cc";

/// Where the filled database template should be copied to.
const string	out_file		= "../libs/star_tracker/database_array.h";

}
