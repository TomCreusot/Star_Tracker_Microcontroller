/*
 *	Reads config files, reduces the number of command line arguments.
 *	@file	properties.h
 *	@author	Tom Creusot
 */

#pragma once
#include <map>
#include <string>
#include <iostream>
#include <fstream>

#include "libs/util/util.h"

using namespace std;
using namespace util;

namespace nix
{

/**
 *	@brief	Reads config files, reduces the number of command line arguments.
 * 	@details
 *			All lines with "name" = value will be accepted.
 *			All spaces and tabs will be removed before reading.
 *			All lines that are invalid are ignored, otherwise you can use # to comment.
 *			The variables will be read as a string, when you try and access it, it will convert it to the requested datatype.
 *
 *	@example
 *		Config c;								// Creates the object.
 *		c.ReadFile("config.properties");		// Reads in the variables from the file.
 *		int a = p.GetInteger("variable_int");	// a now is what "variable_int" was.
 *
 *		Sample file (config.properites):
 *		#This is a config file
 *		variable_int		= 1 # This is a valid line.
 *		vari a ble = 20			# This is also a valid line "variable = 20".
 *		ar=afv=ew				# This line will be ignored.
 *
 *	@author	Tom Creusot
 */

class Config
{
protected:
	/// The name and values.
	map<string, string> hash;

public:

	/**
	 *	@brief	Reads a file and adds the elements to the hashmap.
	 *	@param file	The file to read from.
	 */
	void ReadFile		( string file );


	/**
	 *	@brief	Adds an element to the hashmap, this is useful for the test harness.
	 *  @param 	name 	The name of the variable to add.
	 *	@param 	value 	The value of the variable to add.
	 */

	void Add			( string name, string value );


	/**
	 *	*brief	Retrievs a value from the hashmap and trys to convert to an integer.
	 *	@param 	name	The name of the variable.
	 *	@throws illegal_argument
	 *	@throws out_of_range
	 *	@returns	The integer if valid.
	 */

	int GetInteger		( string name );


	/**
	 *	@brief Retrieves a value from the hashmap and trys to convert to a decimal.
	 *	@param name	The name of the variable.
	 *	@throws illegal_argument
	 *	@throws out_of_range
	 *	@returns	The decimal if valid.
	 */

	decimal GetDecimal 	( string name );


	/**
	 *	@brief Retrievs a value from the hashmap as a string.
	 *	@param name	The name of the variable.
	 *	@returns	The string if valid.
	 */

	string GetString 	( string name );


	/**
	 *	@brief		Prints an error message in an easy to read format.
	 *	@param name	The key which does not exist.
	 */

	void PrintError ( string name );


	/**
	 *	@brief Converts a string to a char SAFELY
	 *	@param name		[in]	The name of the variable.
	 *	@param array	[out]	A character array in the correct format.
	 *	@details Since std::string has size, it seems to ignore the null terminator.
	 */

	static void ConvertString ( string& name, char* array );

	/**
	 * @brief				Sets the string size to before the '#' character.
	 * @param line [out]	The line to adjust.
	 */

	static void RemoveAfterComment ( string* line );


	/**
	 * @brief				Removes tabs and spaces from the line.
	 * @param line [out]	The line to remove tabs and spaces from.
	 */

	static void RemoveTabsSpaces ( string* line);


	/**
	 * @brief				Separates the line into the name and value with an '='.
	 * @param line 	[in]	The line to read.
	 * @param name 	[out]	The lhs.
	 * @param value	[out]	The rhs.
	 * @return				True if valid.
	 */

	static bool SeparateNameValue ( string& line, string* name, string* value );
};
}
