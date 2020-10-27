/**
 *  A resizable array which can be declared on the stack.
 *	@file 		array_list.h
 *	@author		Tom Creusot
 */

#pragma once
#include <iostream>
namespace util
{


/**
 *	@brief	A resizable array which can be declared on the stack.
 *	@details
 *			A stack array which has list like properties.
 *			This has a fixed size.
 *			In default conditions, this is treated to copy the value when inserting.
 *			This stops the requirement of new in a local scope.
 *
 *	@tparam T	The datatype to store.
 *	@tparam N	The maximum size.
 *
 *	@example
 *			ArrayList<int, 10> array(5);
 *			array.PushBack(123);
 *			cout << array.MaxSize() << endl;	// 10
 *			cout << array.Size() << endl;		// 6;
 *			cout << array.Get(5) << endl;		// 123;
 *			cout << array.PopBack() << endl;	// 123;
 *			array.ReduceSize(3);
 *			cout << array.Size() << endl;		// 3;
 *
 *
 * @author Tom Creusot
 */

template <class T, const int N>
class ArrayList
{
protected:
	/// The array.
	T array[N];
	/// end is the number of elements Size().
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
	* @brief Alternate Constructor.
	* @param size	The size of the array to be sized to.
	* @details Does NOT FILL THE ARRAY
	*/

	ArrayList			( int size )
	{
		end = (size > N ? N : size);
	}


	/**
	 * @brief				Copy Constructor.
	 * @param list	[in]	The list to copy.
	 */

	ArrayList			( ArrayList& list )
	{
		end = 0;
		for ( uint i = 0; i < list.Size(); i++ )
		{
			PushBack(list.Get(i));
		}
	}


	/**
	 * @brief 	Finds if the array has no elements.
	 * @return 	True if the array has no elements.
	 */

	virtual /*inline*/ bool IsEmpty 		( ) const
	{
		return end == 0;
	}


	/**
	 * @brief	Finds if the array is full.
	 * @return	True if the array cannot store any more elements.
	 */

	virtual	/*inline*/ bool IsFull 		( ) const
	{
		return end == N;
	}


	/**
	 * @returns The maximum size the array can store.
	 */

	/*inline*/ constexpr uint MaxSize ( )
	{
		return N;
	}

	/**
	 * @brief		Getter for the number of elements.
	 * @return	The number of elements.
	 */

	virtual	/*inline*/ uint Size	( ) const
	{
		return end;
	}


	/**
	 * @brief		Resizes the list to the new size if it is smaller.
	 *				This is useful if you want to remove some points from the end.
	 * @param size	The new size.
	 */

	void ReduceSize	( uint size )
	{
		if (end > size)	end = size;
	}

	/**
	 * @brief Returns the element at the specified position.
	 * @param position	The index.
	 * @return [in/out]	The value at the index.
	 */

	virtual T& Get ( uint position )
	{
		return array[position];
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

	virtual bool PushBack	( T val )
	{
		bool valid = !IsFull();
		if ( valid )
		{
			array[end] = val;
			end++;
		}
		return valid;
	}


	/**
	 * @brief		Appends an element to the end of the list if the condition is true.
	 * @param val	The element to add.
	 * @param add	Adds val to the end if true.
	 * @return		True if added.
	 */

	virtual bool PushBackValid	( T val, bool add )
	{
		if ( add )
			return PushBack(val);
		return false;
	}



	/**
	 * @brief	Removes the element at the end of the list.
	 * @return	The element removed.
	 */

	virtual T PopBack		( )
	{
		if ( !IsEmpty() )
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
	* @brief		Applies a function call to all elements in the array.
	* @param func 	The function to call for each element.
	*/

	void CallAll ( void ( *func )(T*)  )
	{
		for ( uint i = 0; i < Size(); i++ )
		{
			func(&array[i]);
		}
	}


	/**
	 * 	@todo:	THIS IS REALY POORLY MADE, FIX.
	 *
	 *	@brief					Sorts the list into ascending/decending order.
	 *	@param in_order	[in]	must return true if the first parameter should be BEFORE OR EQUAL the second parameter, FALSE if AFTER.
	 *
	 */

	void Sort ( bool (*in_order)(T&, T&) )
	{
		for ( uint ii = 1; ii < Size(); ii++ )
		{
			uint jj = ii;

			T temp = array[jj];
			while ( jj > 0 && in_order(temp, array[jj - 1]) )
			{
				array[jj] = array[jj - 1];
				jj--;
			}
			array[jj] = temp;
		}
	}



	/**
	 * @brief	Slots an element into the list so it is in sorted order by shifting everything right.
	 * @param to_slot	[in]	The element to insert.
	 * @param in_order	[in]	The sorting method.
	 * @return					If the element was inserted.
	 */

	bool Slot (	T& to_slot, bool (*in_order)(T&, T&) )
	{
		for ( uint ii = 0; ii < Size(); ii++ )
		{
			// To insert element, everything must be shifted right.
			if ( in_order(to_slot, Get(ii)) )
			{
				T to_move, to_insert = to_slot;
				uint jj;

				for ( jj = ii; jj < Size(); jj++ )
				{
					to_move = Get(jj);
					Get(jj) = to_insert;
					to_insert = to_move;
				}

				// If to_slot was slotted and to_insert would be pushed off the normal size.
				if ( jj < MaxSize() )
					PushBack(to_insert);

				return true;
			}
		}

		// If there is room to add it at the end.
		if ( Size() < MaxSize() )
		{
			PushBack(to_slot);
			return true;
		}

		return false;
	}
};
}
