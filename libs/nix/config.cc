#include <iostream>
#include "config.h"

namespace nix
{

void Config::ReadFile ( string file )
{
	string line;
	ifstream fileStrm;
	fileStrm.open(file);
	if ( !fileStrm.is_open() )		cout << "invalid file: " << file << endl;
	while ( std::getline(fileStrm, line) )
	{
		string name;
		string value;

		RemoveAfterComment(&line);
		RemoveTabsSpaces(&line);
		if ( SeparateNameValue(line, &name, &value) )
		{
			hash.insert(pair<string, string>(name, value));
		}
	}
}


void Config::Add			( string name, string value )
{
	hash.insert(pair<string, string>(name, value));
}


int Config::GetInteger		( string name )
{
	if ( hash.count(name) == 0 )
	{
		PrintError(name);
	}
	return std::stoi(hash[name]);
}

decimal Config::GetDecimal 	( string name )
{
	if ( hash.count(name) == 0 )
	{
		PrintError(name);
	}
	return std::stof(hash[name]);
}

string Config::GetString 	( string name )
{
	if ( hash.count(name) == 0 )
	{
		PrintError(name);
	}
	return string(hash[name]);
}




void Config::ConvertString ( string& str, char* array )
{
	for ( uint i = 0; i < str.length(); i++ ) array[i] = str[i];//str.c_str()[i];
	array[str.length()] = '\0';
}





void Config::RemoveAfterComment( string* line )
{
	bool found = false;
	for ( uint i = 0; i < line->length() && !found; i++ )
	{
		found = (*line)[i] == '#';
		if ( found )
		{
			line->resize(i);
		}
	}
}


void Config::RemoveTabsSpaces ( string* line )
{
	string comp;
	for ( uint i = 0; i < line->length(); i++ )
	{
		if ( (*line)[i] != ' ' && (*line)[i] != '\t' )	comp += (*line)[i];
	}

	*line = comp;
}



bool Config::SeparateNameValue ( string& line, string* name, string* value )
{
	bool found = false;
	for ( uint i = 0; i < line.length(); i++ )
	{
		const char val = line[i];
		if ( val == '=' )		found = true;		// Swap to value.
		else if ( val == '\r' );					// Breaks code if inside.
		else if ( !found )		*name += val;		// Name
		else					*value += val;		// Value
	}
	return found;
}


inline void Config::PrintError ( string name )
{
	const string red_font = "\033[0;31m";
	const string red_highlight = "\e[41m";
	const string default_font = "\x1B[0m";
	cout << red_highlight << "ERROR, invalid key: " << name << default_font << endl;
}


}
