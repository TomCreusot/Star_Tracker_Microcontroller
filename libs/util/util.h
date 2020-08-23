/**
 *	@file			util.h
 *	@author			Tom Creusot
 *  @brief			Standard conventions for the program.
 *	@namespace util 	To reduce coupling, all required elements for multiple modules are stored in this namespace.
 */

#pragma once
/// @namespace util	Any items which fit into multiple modules.
namespace util
{
	/// @var byte 8 bits.
	typedef unsigned char byte;
	/// @var uint A positive integer, useful for index and size.
	typedef unsigned int uint;
	/// @var decimal Depends on precision, float or double.
	typedef double decimal;
}
