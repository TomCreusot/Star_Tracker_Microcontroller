#include <iostream>
#include "properties.h"

namespace nix
{

void Properties::ReadFile ( string file )
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


void Properties::Add			( string name, string value )
{
	hash.insert(pair<string, string>(name, value));
}


int Properties::GetInteger		( string name )
{
	return std::stoi(hash[name]);
}

decimal Properties::GetDecimal 	( string name )
{
	return std::stof(hash[name]);
}

string Properties::GetString 	( string name )
{
	return string(hash[name]);
}




void Properties::ConvertString ( string& str, char* array )
{
	for ( uint i = 0; i < str.length(); i++ ) array[i] = str[i];//str.c_str()[i];
	array[str.length()] = '\0';
}





void Properties::RemoveAfterComment( string* line )
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


void Properties::RemoveTabsSpaces ( string* line )
{
	string comp;
	for ( uint i = 0; i < line->length(); i++ )
	{
		if ( (*line)[i] != ' ' && (*line)[i] != '\t' )	comp += (*line)[i];
	}

	*line = comp;
}



bool Properties::SeparateNameValue ( string& line, string* name, string* value )
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





}
