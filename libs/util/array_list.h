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

	virtual		/*inline*/ bool IsFull 		( ) const
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

	virtual		/*inline*/ uint Size	( ) const
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
	 * 	@todo:	THIS IS REALY POORLY MADE, FIX.
	 *
	 *	@brief					Sorts the list into ascending/decending order.
	 *	@param inOrder	[in]	must return true if the first parameter should be BEFORE OR EQUAL the second parameter, FALSE if AFTER.
	 *
	 */

	void Sort ( bool (*inOrder)(T&, T&) )
	{
		for ( uint ii = 1; ii < Size(); ii++ )
		{
			uint jj = ii;

			T temp = array[jj];
			while ( jj > 0 && inOrder(temp, array[jj - 1]) )
			{
				array[jj] = array[jj - 1];
				jj--;
			}
			array[jj] = temp;
		}
	}



	/**
	 * @brief	Either calls SlotFilling or SlotFull if the area is full or not.
	 * @param min_index	The inclusive start of the area to slot.
	 * @param max_index The exclusive end of the area to slot.
	 * @param to_slot	The element to slot.
	 * @param in_order	The sorting method.
	 * @return If the element was inserted.
	 */

	bool Slot (	uint min_index, uint max_index,
				T& to_slot, bool (*in_order)(T&, T&) )
	{
		if ( max_index <= Size() )
		{
			return SlotFull(min_index, max_index, to_slot, in_order);
		}
		else
		{
			return SlotFilling(min_index, max_index, to_slot, in_order);
		}
	}

	/**
	 * @brief	Fills a section of a list in order by shifting everything right.
	 * @param min_index	[in]	The lower end of the array to use (inclusive).
	 * @param max_index [in]	The higher end of the array to use (exclusive).
	 * @param to_slot	[in]	The element to insert.
	 * @param in_order	[in]	The sorting method.
	 * @return	True.
	 */

	bool SlotFilling (	uint min_index, uint max_index,
						T& to_slot, bool (*in_order)(T&, T&) )
	{
		for ( uint ii = min_index; ii < max_index && ii < Size(); ii++ )
		{
			bool left_curr = in_order(to_slot, Get(ii));
			if ( left_curr )
			{
				T to_insert = to_slot;
				for ( uint jj = ii; jj < max_index && jj < Size(); jj++ )
				{
					T to_move = Get(jj);
					Get(jj) = to_insert;
					to_insert = to_move;
				}
				PushBack(to_insert);
				return true;
			}
		}
		PushBack(to_slot);
		return true;
	}

	/**
	 * @brief	Slots an element into the list so it is in sorted order by shifting everything left.
	 * @param min_index	[in]	The lower end of the array to use (inclusive).
	 * @param max_index [in]	The higher end of the array to use (exclusive).
	 * @param to_slot	[in]	The element to insert.
	 * @param in_order	[in]	The sorting method.
	 * @return					If the element was inserted.
	 */

	bool SlotFull ( uint min_index, uint max_index,
					T& to_slot, bool (*in_order)(T&, T&) )
	{
		// The element is too small/big and it falls off the list at the start.
		if ( in_order(to_slot, Get(min_index)) )
		{
			return false;
		}
		// It is inserted.
		for ( uint ii = min_index + 1; ii <= max_index; ii++ )
		{
			bool right_prev = in_order(Get(ii - 1), to_slot);
			bool left_curr = in_order(to_slot, Get(ii)) || ii == max_index;
			// To insert element, everything must be shifted left.
			if ( right_prev && left_curr )
			{
				// Shift everything back.
				T to_insert = to_slot;
				// using int instead of uint as int(-1) > uint(0) ?
				for ( int jj = ii - 1; jj >= (int)min_index; jj-- )
				{
					T to_move = Get(jj);
					Get(jj) = to_insert;
					to_insert = to_move;
				}
				return true;
			}
		}
		return false;
	}
};
}
