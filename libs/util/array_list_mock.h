/**
 *  A mock of array_list to help test a variety of different functions.
 *	@file 		array_list_mock.h
 *	@author		Tom Creusot
 */

#pragma once
#include "array_list.h"

namespace util
{

/**
 *	A mock of ArrayList, this allows you to view changes to the array list during its runtime.
 *	@details
 *		Currently watches PushBack, PopBack, Get.
 *
 *	@tparam T	The type to store.
 *	@tparam N	The size of the array list.
 *	@tparam N2	The size of the inner array lists.
 *
 *	@example
 *		ArrayListMock<int, 10, 10> mock;
 *		ArrayList<int, 10>* list = &mock;
 *		list.PushBack(10);
 *		list.PushBack(20);
 *		list.Get(1);
 *		list.Get(0);
 *		list.PopBack();
 *		cout << list.push_list.Get(0) << endl; // 10
 *		cout << list.push_list.Get(0) << endl; // 20
 *		cout << list.get_list.Get(0) << endl; // 1
 *		cout << list.get_list.Get(0) << endl; // 0
 *		cout << list.pop_list.Get(0) << endl; // 20
 */

template <class T, const int N, const int N2>
class ArrayListMock : public ArrayList<T, N>
{
public:
	/// When an item is pushed onto the main list, it will also be added to the end of this list.
	ArrayList<T, N2> push_list;
	/// When an item is poped from the main list, it will be added to the end of this list.
	ArrayList<T, N2> pop_list;
	/// When Get is called from the main list, it will be added to the end of this list.
	ArrayList<uint, N2> get_list;

	/**
	 * @brief		Appends an element to the end of the list.
	 * @param val	The element to add.
	 * @return		True if added.
	 */

	bool PushBack	( T val )
	{
		bool b = ArrayList<T, N>::PushBack(val);
		push_list.PushBack(val);
		return b;
	}



	/**
	 * @brief	Removes the element at the end of the list.
	 * @return	The element removed.
	 */

	T PopBack		( )
	{
		T val = ArrayList<T, N>::PopBack();
		pop_list.PushBack(val);
		return val;
	}





	/**
	 * @brief			Records the position accessed.
	 * @param position	The position of the element to access.
	 * @return			The element removed.
	 */

	T& Get			( uint position )
	{
		get_list.PushBack(position);
		return ArrayList<T, N>::Get(position);
	}



};
}
