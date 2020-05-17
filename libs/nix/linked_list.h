/**
 *  For nix it is usualy more ideal to use a linked list.
 *	This class pretends to be an array list but is accualy a linked list.
 *	@file 		array_list_mock.h
 *	@author		Tom Creusot
 */

#pragma once
#include <iostream>
#include "libs/util/array_list.h"
#include "libs/util/util.h"

using namespace std;

namespace util
{
/**
 * Due to the implementation of the ArrayList, an actual variable is required to push_tail into the regular list.
 * Because of this, it will insert the parameter which will immediatly leave scope.
 * This means that a different implementation which copies the value is required.
 *
 * @example
 *		int i = 0;
 *		int j = 1;
 *		int k = 2;
 *		LinkedListNode<int> node1(i, NULL, NULL);
 *		LinkedListNode<int> node2(j, &node1, NULL);
 *		LinkedListNode<int> node3(k, &node2, NULL);
 *		// node1.next = node2, node2.prev = node1, node2.next = node3, node3.prev = node2.
 *		// node1.prev = NULL, node3.next = NULL.
 *
 *
 *		LinkedListNode<int>* it = &node1;
 *		it = it->next; // node2
 *		it = it->next; // node3
 *
 *	@tparam T	The datatype to be stored.
 *
 *	@author Tom Creusot
 */

template <class T>
class LinkedListNode
{
public:
	/// The value of the node.
	T value;
	/// The next element in the linkedlist.
	LinkedListNode<T>* next;
	/// The previous element in the linkedlist.
	LinkedListNode<T>* prev;


	/**
	 *	@brief	Alternate Constructor constructs a node with a copy of the value.
	 *	@param value [in]		The value to copy into the node.
	 *	@param prev	[in/out]	Sets the previous value of this node and the next value of the previous node if not NULL.
	 *	@param next	[in/out]	Sets the next value of this node and the previous value of the next node if not NULL.
	 */

	LinkedListNode( T value, LinkedListNode<T>* prev, LinkedListNode<T>* next )
	{
		this->value = value;
		this->next = next;
		this->prev = prev;

		if ( prev != NULL )	prev->next = this;
		if ( next != NULL )	next->prev = this;
	}



	/**
	 * @brief	Removes the current node from the list and links the adjacent nodes.
	 */

	void RemoveNode ( )
	{
		if ( prev != NULL )	prev->next = next;
		if ( next != NULL ) next->prev = prev;
		next = NULL;
		prev = NULL;
	}


	/**
	 * @brief	Safely inserts the node between prev and next.
	 * @param prev	[in]	The node to insert after.
	 * @param next	[in]	The node to insert before.
	 */

	void InsertNode ( LinkedListNode<T>* prev, LinkedListNode<T>* next )
	{
		this->next = next;
		this->prev = prev;
		if ( prev != NULL )
		{
			prev->next = this;
		}
		if ( next != NULL )
		{
			next->prev = this;
		}
	}
};







/**
 *	A linked list wrapped in the array list implementation.
 *	This allows dynamic resizing.
 *
 *	@details
 *		This should be avoided, however a useful place to use this is in database_generator to save memory.
 *		Due to the lack of use, error handling has not been implemented as if there is a problem it will crash anyway.
 *		It may also be ideal to run the demo as array_linked_lists to test if they attempt to exceed the bounds, in such a case, it should also crash.
 *
 *	@tparam T	The type to store.
 *
 *	@example
 *		LinkedList<int> list;	// List of any size
 *		ArrayList<int, 0> polymorph; // Treat exactly the same as normal except there are no bounds.
 */

template <class T>
class LinkedList : public util::ArrayList<T, 0>
{
public:
	/// The first element of the linked list.
	LinkedListNode<T>* head;
	/// The last element of the linked list.
	LinkedListNode<T>* tail;
	/// The size of the linked list. (assuming you have not messed with the nodes)
	uint size = 0;


	LinkedList ( ) : ArrayList<T, 0>()
	{
		head = NULL;
		tail = NULL;
	}


	/**
	 * @brief	Destructor.
	 */
	~LinkedList ( )
	{
		LinkedListNode<T>* node = head;
		while ( node != NULL )
		{
			LinkedListNode<T>* cur = node;
			node = cur->next;
			free(cur);
		}
	}



	/**
	 * @brief 	Finds if the list has no elements.
	 * @return 	True if the "array" has no elements.
	 */

	inline bool IsEmpty 		( ) const
	{
		return head == NULL;
	}



	/**
	 * @brief 	A linked list is never full.
	 * @return 	False.
	 */

	inline bool IsFull 		( ) const
	{
		return false;
	}


	/**
	 * @brief 	The size of the list.
	 * @return 	The size of the list.
	 */

	inline uint	Size 		( ) const
	{
		return size;
	}


	/**
	 * @brief Returns the element at the specified position, (SLOW AS FOR ITERATION).
	 * @param position	The index.
	 * @return [in/out]	The value at the index.
	 */

	T& Get ( uint position )
	{
		uint i = 0;
		LinkedListNode<T>* it = head;
		while ( i < position && it != NULL )
		{
			i++;
			it = it->next;
		}
		return it->value;
	}




	/**
	 * @brief		Appends an element to the end of the list.
	 * @param val	The element to add.
	 * @return		True if added.
	 */

	bool PushBack	( T val )
	{
		LinkedListNode<T>* node;
		if ( size == 0 )
		{
			node = new LinkedListNode<T>(val, NULL, NULL);
			head = node;
		}
		else
		{
			node = new LinkedListNode<T>(val, tail, NULL);
		}
		tail = node;
		size++;
		return true;
	}




	/**
	 * @brief		Appends a node to the end of the list.
	 * @param node [in]	The node to add.
	 * @return		True if added.
	 */

	bool PushNodeBack	( LinkedListNode<T>* node )
	{
		if ( head == NULL ) head = node;
		node->RemoveNode();
		node->InsertNode(tail, NULL);
		tail = node;
		size++;
		return true;
	}










	/**
	 * @brief	Removes the element at the end of the list.
	 * @return	The element removed.
	 */

	T PopBack		( )
	{
		if ( tail == NULL ) throw std::string("You cannot pop back when empty");
		T val = tail->value;
		if ( tail->prev != NULL ) // More than one element.
		{
			tail = tail->prev;
			delete tail->next;
			tail->next = NULL;
		}
		else
		{
			delete tail;
			tail = NULL;
			head = NULL;
		}
		size--;
		return val;
	}





	/**
	 * @brief	Removes the element at the start of the list.
	 * @return	The element removed.
	 */

	T PopFront		( )
	{
		if ( head == NULL ) throw std::string("You cannot pop front when empty");
		T val = head->value;
		if ( head->next != NULL ) // More than one element.
		{
			head = head->next;
			delete head->prev;
			head->prev = NULL;
		}
		else
		{
			delete head;
			head = NULL;
			tail = NULL;
		}
		size--;
		return val;
	}





	/**
	 * 	@todo:	THIS IS REALY POORLY MADE, FIX.
	 *
	 *	@brief					Sorts the list into order.
	 *	@param inOrder	[in]	must return true if the first parameter should be BEFORE OR EQUAL the second parameter, FALSE if AFTER.
	 *
	 */

	void Sort ( bool (*inOrder)(T&, T&) )
	{
		LinkedList<T> list;
		LinkedListNode<T>* first = head;
		LinkedListNode<T>* cur = head;
		// Moves all the elements from this list to the next.
		while ( head != NULL )
		{
			// Finds the most inOrder element in this list.
			cur = head;
			first = head;
			while ( cur != NULL )
			{
				if ( inOrder(cur->value, first->value) )
				{
					first = cur;
				}
				cur = cur->next;
			}
			// Makes sure the head and tail points to not first.
			if ( head == first )	head = first->next;
			if ( tail == first )	tail = first->prev;

			// Removes the node from this list and inserts it at the back of list.
			list.PushNodeBack(first);

		}
		// Copy the list across to this list.
		head = list.head;
		tail = list.tail;
		// Stop the destructor deleting the node.
		list.head = NULL;
		list.tail = NULL;
	}



	/**
	 * @brief	Converts a linked list of strings to a string.
	 * @param list [in]	The list of strings.
	 * @return The malloced string.
	 */

	static string* ListToString ( LinkedList<string>& list )
	{
		LinkedListNode<string>* node = list.head;
		unsigned long size = 0;
		string* str;

		while ( node != NULL )
		{
			size+=node->value.size();
			node = node->next;
		}

		str = new string(size, '\0');
		node = list.head;
		int i = 0;
		while ( node != NULL )
		{
			for ( uint jj = 0; jj < node->value.size(); jj++ )
			{
				(*str)[i] = node->value[jj];
				i++;
			}
			node = node->next;
		}
		return str;
	}
};
}
