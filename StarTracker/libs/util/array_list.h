/**
 *	File: 		array_list.h
 *	Author:		Tom Creusot
 *  Purpose:
 *				A stack array which has list like properties.
 *				This has a fixed size.
 *				In default conditions, this is treated to copy the value when inserting.
 *				This stops the requirement of new in a local scope.
 */

#pragma once
#define ARRAY_LIST_SIZE 100

namespace util
{



template <class T>
class ArrayList
{
private:
	// The array.
	T array[ARRAY_LIST_SIZE];
	// end is the number of elements, it will be max_index + 1
	uint end;

public:

	/**
	 * @brief 		Default Constructor.
	 * @details		Sets all values to null and the array_size to 0.
	 */

 	ArrayList			( )
 	{
 		end = 0;
 	}



	/**
	 * @brief 	Finds if the array has no elements.
	 * @return 	True if the array has no elements.
	 */

 	bool empty			( )
 	{
 		return end == 0;
 	}


	/**
	 * @brief	Finds if the array is full.
	 * @return	True if the array cannot store any more elements.
	 */

 	bool full			( )
 	{
 		return end == ARRAY_LIST_SIZE;
 	}


	/**
	 * @brief		Getter for the number of elements.
	 * @return	The number of elements.
	 */

	uint size			( )
 	{
 		return end;
 	}

	/**
	 * @brief	Getter of ARRAY_LIST_SIZE.
	 * @return	ARRAY_LIST_SIZE.
	 */

 	uint max_size 		( )
 	{
 		return ARRAY_LIST_SIZE;
 	}







	/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
	|																	|
	|						---- Push/Pop ----							|
	|																	|
	\*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/


	/**
	 * @brief		Appends an element to the end of the list.
	 * @param val	The element to add.
	 * @return		True if added.
	 */

 	bool push_back	( T val )
 	{
 		bool valid = !full();
 		if ( valid )
 		{
 			array[end] = val;
 			end++;
 		}
 		return valid;
 	}



	/**
	 * @brief	Removes the element at the end of the list.
	 * @return	The element removed.
	 */

 	T& pop_back		( )
 	{
 		if ( !empty() )
 		{
 			end--;
			return array[end];
 		}
 		return array[end];
 	}






   /*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*\
   |																	|
   |						---- Others ----							|
   |																	|
   \*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

   /**
    * @brief	Accesses the specific element from the start of the array.
    * @param	index	The index to search from the start.
    * @details	This will work even if the array has shuffled.
    */

	T& operator[] ( uint index )
	{
		return array[index];
	}


};
}
