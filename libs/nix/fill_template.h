/**
 *	@brief Replaces a template file variable slots, this is for constructing a database.
 *
 *
 *	@file fill_template.h
 *	@author Tom Creusot.
 *
 */


#include <list>
#include <string>
#include <iostream>
#include <istream>
#include <fstream>

using namespace std;
/// @namespace nix Unix implementation.
namespace nix
{

/**
 * @brief	This class is to replace "keys" with "values" in a template file.
 *
 * @example
 *		FillTemplate template;
 *		template.AddKey("key", "value");
 *		string str = "keys of a quadratic";
 *		template.ReplaceVariables(&str); //values of a quadratic.
 *		ReplaceFile("template", "out_database");
 *
 * @author 	Tom Creusot.
 */
class FillTemplate
{
private:
	/// Each line of the file being read in.
	list<string> file_lines;

public:
	/// The key to replace with a value.
	list<string> keys;
	/// The value to replace the key.
	list<string> values;


	/**
	 * @brief	Opens a file, replaces the keys with values, then outputs to the output file.
	 * @param file_input	[in]	The file to read.
	 * @param file_output	[in]	The file to write.
	 */

	void ReplaceFile ( string file_input, string file_output )
	{
		cout << "Reading file: " << file_input << endl;
		ifstream inStrm;
		inStrm.open(file_input);
		if ( !inStrm.is_open() )	cout << "invalid file: " << file_input << endl;
		else
		{
			string line;
			while ( std::getline(inStrm, line) )
			{
				string copy(line);
				file_lines.push_back(copy);
			}
		}
		inStrm.close();

		cout << "Filling file." << endl;
		for ( list<string>::iterator it = file_lines.begin(); it != file_lines.end(); it++ )
		{
			ReplaceVariables(&(*it));
		}

		cout << "Writing to file: " << file_output << endl;


		ofstream outStrm;
		outStrm.open(file_output);
		for ( list<string>::iterator it = file_lines.begin(); it != file_lines.end(); it++ )
		{
			outStrm << *it << endl;
		}
		outStrm.close();

	}




	/**
	 * @brief	Appends a key-value set to the end of keys and values to be used with ReplaceVariables.
	 * @param key	The key to add.
	 * @param value	The value for the key.
	 */

	void AddKey ( string key, string value )
	{
		keys.push_back(key);
		values.push_back(value);
	}


	/**
	 * @brief	Replaces all instances of a key with a value.
	 * @param	replace [in/out]	The string to replace the key for a value.
	 */

	void ReplaceVariables ( string* replace )
	{
		list<string>::iterator k_it = keys.begin();
		list<string>::iterator v_it = values.begin();
		while ( k_it != keys.end() && v_it != values.end() )
		{
			Replace(*k_it, *v_it, replace);
			k_it++;
			v_it++;
		}
	}


	/**
	 * @brief	Replaces the key with value iteratively.
	 * @param key				The value to be replaced.
	 * @param value				The replacement for key.
	 * @param str	[in/out]	The string to replace key with value.
	 */

	static void Replace ( string key, string value, string* str )
	{
		uint index = str->find(key);
		while ( index != std::string::npos && index != (uint)-1 )
		{
			str->replace(index, key.size(), value);
			index = str->find(key);
		}
	}

};
}
