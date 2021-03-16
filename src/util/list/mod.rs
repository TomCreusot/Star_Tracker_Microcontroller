//! This is the implementation for a generic linear list (i.e ArrayList, LinkedList).
//! It contains operations such as:
//! ``` ignore
//! List
//! 	size      ( ) -> usize                           // The number of elements currently stored.
//! 	is_full   ( ) -> bool                            // If there is no more capcity.
//! 	is_empty  ( ) -> bool                            // If the list is empty.
//! 	get       ( index : usize ) -> T                 // Gets a copy of the value.
//! 	set       ( index : usize, value : T ) -> ()     // Creates and assigns a copy of the value.
//! 	push_back ( value : T ) -> ()                    // Copies value to the end of the list.
//! 	pop_back  ( ) -> T                               // Removes and returns copy of last value.
//! 	sort      ( ) -> ()                              // Sorts the list.
//! ```

pub mod array_list;


//###############################################################################################//
//									---	List Trait ---
//###############################################################################################//

/// Any collection which can be expressed as a linear list.
/// Useful for vect!, LinkedList and ArrayList.
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
	/// # Example
	/// lst : ArrayList<UInt, 2> = ArrayList::new();
	fn is_empty ( &self ) -> bool;

	/// Gets the element at the specified index.
	/// # Arguments
	///	* 'index' - The index of the element to receive.
	/// # Returns
	/// The value at the index.
	fn get ( &self, index : usize ) -> T;

	/// Sets the element at the specified index.
	/// # Arguments
	///	* 'index' - The index of the element to receive.
	/// * 'value' - The value to assign.
	fn set ( &mut self, index : usize, value : T );

	/// Adds an element to the end of the list.
	/// # Arguments
	/// * 'value' - the value to add to the end.
	fn push_back ( &mut self, value : T );

	/// Removes an element from the end of the list.
	/// # Returns
	/// The value removed.
	fn pop_back ( &mut self ) -> T;

	/// Sorts the list
	/// # Arguments
	/// * 'in_order' - A function which returns TRUE if it is in order.
	fn sort ( &mut self, in_order: fn (& T, & T) -> bool );

	/// Slots an element into the list so it is in sorted order by shifting everything right.
	/// # Arguments
	/// * `to_slot` - The element to add.
	/// * `in_order` - The ordering method.
	///
	/// # Returns
	/// True if inserted, false if there is no space and it will trail the last element.
	fn slot ( &mut self, to_slot : T, in_order: fn (& T, & T) -> bool ) -> bool;
}



//###############################################################################################//
//									---	ArrayList Struct ---
//###############################################################################################//

/// This is a data structure which can be implemented on the stack and store a resizing linear set of generic data types **Do not use any heap allocated objects**.
/// This is implemented by an array and a end point.
/// On creation, an empty array is created and an index is set to 0. As the array is filled, the index increments indicating the new end.
///
/// # Example
/// The following example will:
/// * Create an ArrayList of u32 of size max_capacity (3).
/// * Fill the array with: {2, 2, 1}.
/// * Change the list to: {3, 2, 1}.
/// * Sort the list in ascending order.
/// * Remove all the elments one by one from the end.
/// ```
/// use star_tracker::util::list::{ArrayList, List};
/// fn sort_ascending ( left : & u32, right : & u32 ) -> bool	{  return left < right;  }
///
///
/// const max_capacity : usize = 3;
/// let mut lst : ArrayList<u32, max_capacity> = ArrayList::new();
///
/// assert!(lst.is_empty());
///
/// lst.push_back(2);
/// lst.push_back(2);
/// lst.push_back(1);
/// assert!(lst.is_full());
/// assert_eq!(lst.size(), 3);
///
/// assert_eq!(lst.get(0), 2);
/// lst.set(0, 3);
/// assert_eq!(lst.get(0), 3);
///
/// lst.sort(sort_ascending); // 1, 2, 3
///
/// assert_eq!(lst.pop_back(), 3); // 1, 2 | 3
/// assert_eq!(lst.pop_back(), 2); // 1    | 2
/// assert_eq!(lst.pop_back(), 1); //      | 1
///
/// lst.slot(10, sort_ascending);
/// lst.slot(5, sort_ascending);
/// lst.slot(1, sort_ascending);
///
/// assert_eq!(lst.pop_back(), 10); // 1, 5 | 10
/// assert_eq!(lst.pop_back(), 5); // 1    | 5
/// assert_eq!(lst.pop_back(), 1); //      | 1
/// ```
pub struct ArrayList <T, const N : usize>
{
	array : [T; N],
	end : usize,
}
