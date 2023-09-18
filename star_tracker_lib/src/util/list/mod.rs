//! A universal implementation of an array list, this is setup for a static ArrayList and Vec.  
//!  
//! To enable the feature for Vec, you must compile with the feature "nix".  
//!
//! Vec already is an implementation of a list, however, it is declared on the heap.  
//! List provides a default implementation which can be used as a heap or stack object.  
//! The main implementations are for ArrayList or Vec.  
//! For embedded systems, it is advised to use ArrayList as it is declared on the stack.  
//! For other systems, it is advised to use the vec implementation as it will never fill up.  
//!
//! # Array List Structure
//! An array list is an oversized array with a pointer.  
//! Like a stack, elements can be appended to the end and popped from the end quick and easily.  
//! However, unlike a stack, an array_list is designed to access elements throughout like an array or linked list.  
//!  Consider the following diagram.
//! ``` text
//! | 0 | 1 | 2 | 3 | 4 | 5 | pointer = 0
//!   x   x   x   x   x   x
//!
//! Add an element (3).
//! | 0 | 1 | 2 | 3 | 4 | 5 | pointer = 1
//!   3   x   x   x   x   x

//! Add some elements (2, 5, 4).
//! | 0 | 1 | 2 | 3 | 4 | 5 | pointer = 4
//!   3   2   5   4   x   x
//!
//! Returns 3: arraylist.get(0)
//! Returns 2: arraylist.get(1)
//! Returns 5: arraylist.get(2)
//! Returns 4: arraylist.get(3)
//! ```
//!
//! # Memory Allocation
//! The heap can cause problems on microprocessors;
//! - It takes time to allocate memory to the heap as oppose to the stack which is instant.
//! - The size of the program is variable and it is hard to predict if the system will run out of memory.
//! - Memory will be fragmented and scattered.  
//! Consider running a program for a long time which allocates to a single memory location:
//!    ``` text
//!    The program allocates every location.
//!    | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
//!      u   u   u   u   u   u   u   u   u   u   Free = 0
//!
//!    Some memory is free.
//!    | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
//!      u   u   _   u   u   _   u   _   u   u    Free = 3
//!
//!    Attempt to allocate an array of size 3.
//!    | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
//!      u   u   _   u   u   _   u   _   u   u    Free = 3
//!    ERROR as the memory is fragmented and the array cannot fit.
//!    ```
//!  
//! Using a stack ensures that all the memory is available to the processor.  
//! In big systems (i.e. computer), this is not nessisary to consider due to the processor speed and memory size.  
#[cfg(any(feature = "nix", test))] pub mod vec;
pub mod list_iterator;
pub mod array_list;

use crate::core_include::*;

use crate::util::err::Error;
use crate::util::err::Errors;


//###############################################################################################//
//									---	List Trait ---
//###############################################################################################//

/// A set of instructions for a list/arraylist data structure.  
///
/// Vec already is an implementation of a list, however, it is declared on the heap.  
/// List provides a default implementation which can be used as a heap or stack object.  
/// The main implementations are for ArrayList or Vec.  
/// For embedded systems, it is advised to use ArrayList as it is declared on the stack.  
/// For other systems, it is advised to use the vec implementation as it will never fill up.  
///
/// # Memory Allocation
/// The heap can cause problems on microprocessors;
/// - It takes time to allocate memory to the heap as oppose to the stack which is instant.
/// - The size of the program is variable and it is hard to predict if the system will run out of memory.
/// - Memory will be fragmented and scattered.  
/// Consider running a program for a long time which allocates to a single memory location:
///    ``` text
///    The program allocates every location.
///    | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
///      u   u   u   u   u   u   u   u   u   u   Free = 0
///
///    Some memory is free.
///    | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
///      u   u   _   u   u   _   u   _   u   u    Free = 3
///
///    Attempt to allocate an array of size 3.
///    | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
///      u   u   _   u   u   _   u   _   u   u    Free = 3
///    ERROR as the memory is fragmented and the array cannot fit.
///    ```
///  
/// Using a stack ensures that all the memory is available to the processor.  
/// In big systems (i.e. computer), this is not nessisary to consider due to the processor speed and memory size.  
pub trait List<T>
{
	/// Finds the max number of elements that can be stored in the list.  
	/// If invalid list, returns 0.  
	fn capacity ( &self ) -> usize;

	/// Finds how many elements are in the list.  
	/// # Returns
	/// The size.
	fn size ( &self ) -> usize;

	/// Checks if the List is at maximum capacity.
	/// # Returns
	/// True if full.
	fn is_full ( &self ) -> bool;

	/// Checks if the List is empty.
	/// # Returns
	/// True if empty.
	///
	/// # Examples
	/// lst: ArrayList<UInt, 2> = ArrayList::new();
	fn is_empty ( &self ) -> bool;

	/// Gets the element at the specified index.
	/// # Arguments
	///	* 'index' - The index of the element to receive.
	/// # Returns
	/// The value at the index.
	fn get ( &self, index: usize ) -> T;

	/// Sets the element at the specified index.
	/// # Arguments
	///	* 'index' - The index of the element to receive.
	/// * 'value' - The value to assign.
	fn set ( &mut self, index: usize, value: T ) -> Error<()>;

	/// Adds an element to the end of the list.
	/// # Arguments
	/// * 'value' - the value to add to the end.
	fn push_back ( &mut self, value: T ) -> Error<()>;

	/// Removes an element from the end of the list.
	/// # Returns
	/// The value removed.
	fn pop_back ( &mut self ) -> Error<T>;

	/// Sets the counter to 0 so all elements will be override and the list is essentially cleared.
	fn clear ( &mut self );

	/// Sorts the list
	/// # Arguments
	/// * 'in_order' - A function which returns TRUE if it is in order.
	fn sort_order ( &mut self, in_order: fn (& T, & T) -> bool );

	/// Slots an element into the list so it is in sorted order by shifting everything right.
	/// # Arguments
	/// * `to_slot` - The element to add.
	/// * `in_order` - The ordering method.
	///
	/// # Returns
	/// True if inserted, false if there is no space and it will trail the last element.
	fn slot ( &mut self, to_slot: T, in_order: fn (& T, & T) -> bool ) -> bool;


	/// Removes element at index, reduces size of array list, moves everything left right of point.
	/// # Arguments
	/// * `index` - The index to remove.
	/// # Return
	/// index < size  The element at that point.
	/// size = 0      InvalidSize
	/// size =< index OutOfBounds
	/// # Examples
	/// ```
	/// use star_tracker_lib::util::list::List;
	/// use star_tracker_lib::util::list::ArrayList;
	///	let mut list: ArrayList<usize, 4> = ArrayList::from_array(&[0, 1, 2, 3]);
	///
	/// assert_eq!((& mut list as &mut dyn List<usize>).pop(1), Ok(1));
	/// assert_eq!(list.size(), 3);
	/// assert_eq!(list.get(0), 0);
	/// assert_eq!(list.get(1), 2);
	/// assert_eq!(list.get(2), 3);
	///
	/// assert_eq!((& mut list as &mut dyn List<usize>).pop(0), Ok(0));
	/// assert_eq!(list.size(), 2);
	/// assert_eq!(list.get(0), 2);
	/// assert_eq!(list.get(1), 3);
	///
	/// assert_eq!((&mut list as &mut dyn List<usize>).pop(1), Ok(3));
	/// assert_eq!(list.size(), 1);
	/// assert_eq!(list.get(0), 2);
	///
	/// assert!((&mut list as &mut dyn List<usize>).pop(1).is_err());
	/// assert_eq!(list.size(), 1);
	/// assert_eq!(list.get(0), 2);
	///
	/// assert_eq!((&mut list as &mut dyn List<usize>).pop(0), Ok(2));
	/// assert_eq!(list.size(), 0);
	///
	/// assert!((&mut list as &mut dyn List<usize>).pop(0).is_err());
	/// assert_eq!(list.size(), 0);
	/// ```
	fn pop ( &mut self, index: usize ) -> Error<T>
	{
		if self.size() == 0
		{
			return Result::Err(Errors::InvalidSize);
		}
		
		if index < self.size()
		{
			let value = self.get(index);
			let mut ii = index;
			while ii < self.size() - 1
			{
				self.set(ii, self.get(ii + 1))?;
				ii += 1;
			}
			let _ = self.pop_back();
			return Result::Ok(value);
		}
		return Result::Err(Errors::OutOfBounds);
	}


	/// Finds any associated items and appends self to out.
	/// # Arguments
	/// * `list_b` - A list to compare with list_a.
	/// * `compare` - A function to compare the two (usually a == b).
	/// * `out` - The output array which holds the similarities.
	/// # Returns
	/// Errors::InvalidSize if the list is full and more elements should be added.
	/// Ok(()) otherwise.
	/// # Examples
	/// ```
	/// use star_tracker_lib::util::list::List;
	/// use star_tracker_lib::util::list::ArrayList;
	/// let mut list_a: ArrayList<usize, 10> = ArrayList::new();
	/// let list_b: ArrayList<usize, 3> = ArrayList::from_array(&[0, 1, 2]);
	/// list_a.push_back(1);
	/// list_a.push_back(2);
	/// list_a.push_back(3);
	/// fn func ( x: &usize, y: &usize ) -> bool {x == y};
	/// let mut out: ArrayList<usize, 10> = ArrayList::new();
	///
	/// list_a.find_match ( &list_b, func, &mut out );
	/// assert_eq!(out.size(), 2);
	/// assert_eq!(out.get(0), 1);
	/// assert_eq!(out.get(1), 2);
	/// ```
	fn find_match ( &self, list_b: &dyn List<T>,
			compare: fn(&T, &T) -> bool, out: &mut dyn List<T> ) -> Error<()>
	{
		for ii in 0..self.size()
		{
			for jj in 0..list_b.size()
			{
				if compare(&self.get(ii), &list_b.get(jj)) && !out.is_full()
				{
					out.push_back(self.get(ii))?; // No more room to fit elements.
				}
			}
		}
		return Result::Ok(());
	}


	/// Removes all elements which do not share the similarities in the compare function and list.
	/// # Arguments
	/// * `other` - The list to compare to.
	/// * `compare` - The function to compare the lists together.
	///
	/// # Examples
	/// ```
	/// use star_tracker_lib::util::list::List;
	/// use star_tracker_lib::util::list::ArrayList;
	/// let mut list_a: ArrayList<usize, 10> = ArrayList::new();
	/// let list_b: ArrayList::<usize, 3> = ArrayList::from_array(&[0, 1, 2]);
	/// list_a.push_back(1);
	/// list_a.push_back(2);
	/// list_a.push_back(3);
	/// fn func ( x: &usize, y: &usize ) -> bool {x == y};
	/// let mut out: ArrayList<usize, 0> = ArrayList::new();
	///
	/// list_a.remove_diff( &list_b, func );
	/// assert_eq!(out.size(), 0);
	/// ```
	fn remove_diff ( &mut self, other: &dyn List<T>, compare: fn(&T, &T) -> bool )
	{
		let mut ii = 0;
		while ii < self.size()
		{
			let mut keep = false;
			for jj in 0..other.size()
			{
				if compare(&self.get(ii), &other.get(jj))
				{
					keep = true;
				}
			}
			if keep
			{
				ii += 1;
			}
			else
			{
				self.pop(ii).expect("WHY IS IT NOT POPPING???");
			}
		}
	}
}



//###############################################################################################//
//									---	ArrayList Struct ---
//###############################################################################################//

/// An implementation of an Array List with no allocations to the heap.  
///
/// Choose this data structure if:
/// * You need a **resizable**, linear data structure of which you can access elements with an index.
/// * You have limited memory on your device (i.e. microprocessor).
/// * You need a reliable system which has a known memory usage.
/// * You have a slow processor which will make heap allocations slow.  
///
/// # Example
/// The following example will show the versatility of an array_list.
/// ```
/// use star_tracker_lib::util::list::ArrayList;
/// use star_tracker_lib::util::list::List;
///
///
/// // Creates an ArrayList which stores u32's, can store a maximum of 3 elements and is empty.
/// const max_capacity: usize = 3;
/// let mut lst: ArrayList<u32, max_capacity> = ArrayList::new();
/// assert!(lst.is_empty());
///
/// // Adds elements to the back of the list.
/// lst.push_back(2); // 2
/// lst.push_back(2); // 2, 2
/// lst.push_back(1); // 2, 2, 1
///
/// assert!(lst.is_full());
/// assert_eq!(lst.size(), 3);
///
/// // Assigns the value at the provided index.
/// lst.set(0, 3);    // 321
///
/// // Sorts the list with a custom sorting function.
/// fn sort_ascending ( left: & u32, right: & u32 ) -> bool	{  return left < right;  }
/// lst.sort_order(sort_ascending); // 1, 2, 3
///
/// // Verifies the list is in sorted order and removes each element.
/// assert_eq!(lst.pop_back(), Ok(3));  // 1, 2 | 3
/// assert_eq!(lst.pop_back(), Ok(2));  // 1    | 2
/// assert_eq!(lst.pop_back(), Ok(1));  //      | 1
///
/// // Slot fits elements into the array given the custom sorting function.
/// // As the core of this struct uses an array, consider this should not be used on a large array due to complexity and time.
/// lst.slot(10, sort_ascending); // 10
/// lst.slot(5, sort_ascending);  // 5, 10
/// lst.slot(1, sort_ascending);  // 1, 5, 10
///
///
/// // The list can also be created from an array.
/// // This is a useful function for test harnesses.
/// let mut lst_2: ArrayList<u32, 5> = ArrayList::from_array(&[1, 1, 2, 3, 5]);
/// 
/// // You can remove an element at an index and shuffle everything down.
/// // This is not recommended in a large list as shuffling everything takes time.
/// assert_eq!(lst.pop(0), Ok(1)); // 1, 2, 3, 5, _
/// 
/// // You can find elements which match between two lists of any type.
/// pub fn equal ( a: &u32, b: &u32 ) -> bool { return a == b; }
/// let lst_3 = ArrayList::from_array(&[2, 3, 5, 8]);
/// let mut lst_4: ArrayList<u32, 10> = ArrayList::new();
/// lst_2.find_match(&lst_3, equal, &mut lst_4);
///
/// assert_eq!(lst_4.size(), 3);
/// assert_eq!(lst_4.get(0), 2);
/// assert_eq!(lst_4.get(1), 3);
/// assert_eq!(lst_4.get(2), 5);
///
/// 
/// // You can also reduce your array to only have similarities to the other list.
/// lst_2.remove_diff(&lst_3, equal);
/// assert_eq!(lst_2.size(), 3);
/// assert_eq!(lst_2.get(0), 2);
/// assert_eq!(lst_2.get(1), 3);
/// assert_eq!(lst_2.get(2), 5);
///
///
/// // For extra memory security, you can reuse the same array by using clear().
/// lst_4.clear();
/// assert_eq!(lst_4.size(), 0);
///
/// // You can also clone the array (safely).
/// let lst_2_cloned = lst_2.clone();
/// assert_eq!(lst_2_cloned.size(), 3);
/// assert_eq!(lst_2_cloned.get(0), 2);
/// assert_eq!(lst_2_cloned.get(1), 3);
/// assert_eq!(lst_2_cloned.get(2), 5);
/// ```

// fn find_match ( &self, list_b: &dyn List<T>,
// 		compare: fn(&T, &T) -> bool, out: &mut dyn List<T> )
pub struct ArrayList <T, const N: usize>
{
	array: [T; N],
	end:   usize,
}


//###############################################################################################//
//									---	ListIterator ---
//###############################################################################################//

/// Due to limitations with lifetimes and types, the default ToIterator trait cannot be used.  
/// Instead the iterator statically creates a new iterator from an input list.  
/// 
/// # Example
/// ```
/// use star_tracker_lib::util::list::ListIterator;
/// use star_tracker_lib::util::list::ArrayList;
/// 
/// let lst : ArrayList<u32, 4> = ArrayList::from_array(&[1,2,3,4]);
/// let mut iter = ListIterator::new(&lst);
///
/// while let Some(e) = iter.next()
/// {
/// 	println!("{}",e);
/// }
///
/// ```
#[derive(Copy)]
#[derive(Clone)]
pub struct ListIterator <'a, T>
{
	list : &'a dyn List <T>,
	index: usize,
}









//###############################################################################################//
//###############################################################################################//
//
//										Unit Tests
//
//###############################################################################################//
//###############################################################################################//


#[cfg(test)]
#[allow(unused_must_use)]
mod test
{
	use crate::util::list::ArrayList;
	use crate::util::list::List;
	use crate::util::err::Errors;
	use crate::util::err::Error;


//###############################################################################################//
//
//										Standard Implementation
//
// pub fn pop         ( &mut self, usize ) -> Error<T>
// pub fn find_match  ( &self, &dyn List<T>, fn(&T, &T) -> bool, &mut dyn List<T> )
// pub fn remove_diff ( &mut self, other: &dyn List<T>, compare: fn(&T, &T) -> bool )
//
//###############################################################################################//
//										~ pop ~													 //
	
	#[test]
	// An arraylist should be able to pop an element if it is not empty.
	fn test_pop_array_list_not_empty ( ) -> Error<()>
	{
		let mut list: ArrayList<usize, 4> = ArrayList::from_array(&[0, 1, 2, 3]);

		assert_eq!(list.pop(1), Ok(1));
		assert_eq!(list.size(), 3);
		assert_eq!(list.get(0), 0);
		assert_eq!(list.get(1), 2);
		assert_eq!(list.get(2), 3);

		assert_eq!(list.pop(0), Ok(0));
		assert_eq!(list.size(), 2);
		assert_eq!(list.get(0), 2);
		assert_eq!(list.get(1), 3);

		assert_eq!(list.pop(1), Ok(3));
		assert_eq!(list.size(), 1);
		assert_eq!(list.get(0), 2);

		assert_eq!(list.pop(0), Ok(2));
		assert_eq!(list.size(), 0);

		return Ok(());
	}
	
	#[test]
	// If the arraylist is empty, it should output an error if a pop is attempted.
	fn test_pop_array_list_empty ( )
	{
		let mut list: ArrayList<usize, 0> = ArrayList::new();
		assert_eq!(list.pop(0), Err(Errors::InvalidSize));
	}
	
	#[test]
	// If the index is out of bounds, the vector should produce an error.
	fn test_pop_array_out_of_bounds ( )
	{
		let mut list: ArrayList<usize, 2> = ArrayList::new();
		list.push_back(1);
		assert_eq!(list.pop(2), Err(Errors::OutOfBounds));
	}
	


















//										~ find_match ~											 //
	#[test]
	// Function should work on vectors and array lists.
	// Since the parameters are interchangeable, they are tested in the same test.
	fn test_find_match ( )
	{
		let mut list_a: ArrayList<usize, 10> = ArrayList::new();
		let list_b: ArrayList<usize, 3> = ArrayList::from_array(&[0, 1, 2]);
		list_a.push_back(1);
		list_a.push_back(2);
		list_a.push_back(3);
		fn func ( x: &usize, y: &usize ) -> bool {x == y}
		let mut out: ArrayList<usize, 10> = ArrayList::new();

		list_a.find_match ( &list_b, func, &mut out );
		assert_eq!(out.size(), 2);
		assert_eq!(out.get(0), 1);
		assert_eq!(out.get(1), 2);
	}


	#[test]
	//
	fn test_find_match_list_full ( )
	{
		let mut list_a: ArrayList<usize, 10> = ArrayList::new();
		let list_b: ArrayList<usize, 3> = ArrayList::from_array(&[0, 1, 2]);
		list_a.push_back(1);
		list_a.push_back(2);
		list_a.push_back(3);
		fn func ( x: &usize, y: &usize ) -> bool {x == y}
		let mut out: ArrayList<usize, 0> = ArrayList::new();

		list_a.find_match ( &list_b, func, &mut out );
		assert_eq!(out.size(), 0);
	}


//										~ remove_diff ~											 //
	#[test]
	fn test_remove_diff ( )
	{
		let mut list_a: ArrayList<usize, 10> = ArrayList::new();
		let list_b: ArrayList<usize, 3> = ArrayList::from_array(&[0, 1, 2]);
		list_a.push_back(1);
		list_a.push_back(2);
		list_a.push_back(3);
		fn func ( x: &usize, y: &usize ) -> bool {x == y}

		list_a.remove_diff ( &list_b, func );
		assert_eq!(list_a.size(), 2);
		assert_eq!(list_a.get(0), 1);
		assert_eq!(list_a.get(1), 2);
	}
}
