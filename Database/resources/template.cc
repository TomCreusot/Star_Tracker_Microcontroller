/**
 *	File: 		$(file)
 *	Author:		Tom Creusot
 *  Purpose:	A 2d array containing the database {angle, ra, dec}.
 */

#pragma once

#include "libs/util/util.h"
using namespace util;
namespace database
{

const int database_size = $(num_elements);

const decimal $(array_name) [$(num_elements)][5] =
$(array_elements)


}
