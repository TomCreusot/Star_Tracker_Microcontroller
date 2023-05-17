//! Implementation of [ArrayList](crate::util::list::ArrayList).
use crate::core_include::*;

use crate::util::list::ArrayList;
use crate::util::list::List;

use crate::util::err::Error;
use crate::util::err::Errors;

//###############################################################################################//
//								---	ArrayList Constructor ---
//###############################################################################################//

impl <T, const N: usize> ArrayList <T, N>
{
	/// Constructor
	/// Use `new` or `from_array` to ensure the list is correctly setup.  
	/// This will create a new list with the given capacity and type with zero size.  
	/// # Examples 
	/// ```
	/// use star_tracker_lib::util::list::ArrayList;
	///
	/// let list: ArrayList<u8, 10> = ArrayList::new(); // Creates a list of size 0, capacity 10.
	/// ```
	pub fn new ( ) -> ArrayList<T, N>
	{
		use core::mem::MaybeUninit;
		ArrayList
		{
			array:  unsafe{MaybeUninit::uninit().assume_init()}, // Auto inits array.
			end:    0,
		}
	}
}

impl <T, const N: usize> ArrayList <T, N> where T: Copy
{
	/// Creates an array list from an array.
	/// # Arguments
	/// * `array` - The array to copy.
	/// # Returns
	/// Array < Capacity  A copy of the array.  
	/// Array > Capacity  Errors::InvalidSize  
	/// # Examples
	/// ```
	/// use star_tracker_lib::util::list::ArrayList;
	/// use star_tracker_lib::util::list::List;
	///
	/// let array: [u32; 5] = [2, 4, 6, 8, 10];
	/// let list: ArrayList<u32, 5> = ArrayList::from_array(&array);
	/// assert_eq!(list.get(0), 2);
	/// assert_eq!(list.get(1), 4);
	/// assert_eq!(list.get(2), 6);
	/// assert_eq!(list.get(3), 8);
	/// assert_eq!(list.get(4), 10);
	/// ```
	pub fn from_array ( array: &[T; N] ) -> Self
	{
		let mut list : ArrayList<T, N> = ArrayList::new();
		for e in array
		{
			let _ = list.push_back(e.clone());
		}
		return list;
	}
}


//###############################################################################################//
//								---	ArrayList Implementation ---
//###############################################################################################//
impl<T, const N: usize> List<T> for ArrayList<T, N> where T: Clone
{

	/// Finds the max number of elements that can be stored in the list.
	/// # Examples
	/// ```
	/// use star_tracker_lib::util::list::ArrayList;
	/// use star_tracker_lib::util::list::List;
	/// use star_tracker_lib::util::err::Errors;
	///
	/// const max_capacity: usize = 2;
	/// let mut lst: ArrayList<u32, max_capacity> = ArrayList::new();
	/// assert_eq!(lst.capacity(), max_capacity);
	///
	/// lst.push_back(1);
	/// lst.push_back(2);
	/// assert!(lst.push_back(3).is_err()); // The capacity is 2, cannot add another element.
	/// ```
	fn capacity ( &self ) -> usize
	{
		return N;
	}


	/// Finds how many elements are in the list.
	///
	/// # Examples
	/// ```
	/// use star_tracker_lib::util::list::ArrayList;
	/// use star_tracker_lib::util::list::List;
	///
	/// let mut lst: ArrayList<u32, 10> = ArrayList::new();
	/// assert_eq!(0, lst.size());
	/// lst.push_back(1);
	/// assert_eq!(1, lst.size());
	///
	/// lst.push_back(1);
	/// assert_eq!(2, lst.size());
	/// ```
	fn size ( &self ) -> usize
	{
		return self.end;
	}


	/// Checks if the ArrayList is at maximum capacity.  
	/// Returns true when full.  
	///
	/// # Examples
	/// ```
	/// use star_tracker_lib::util::list::ArrayList;
	/// use star_tracker_lib::util::list::List;
	///
	/// let mut lst: ArrayList<u32, 2> = ArrayList::new();
	/// lst.push_back(1);
	/// assert!(!lst.is_full());
	///
	/// lst.push_back(2);
	/// assert!(lst.is_full());
	/// ```
	fn is_full ( &self ) -> bool
	{
		return self.end == self.array.len();
	}


	/// Checks if the ArrayList contains no elements.  
	/// Returns true if the array size is 0.  
	///
	/// # Examples
	/// ```
	/// use star_tracker_lib::util::list::ArrayList;
	/// use star_tracker_lib::util::list::List;
	///
	/// let mut lst: ArrayList<i32, 2> = ArrayList::new();
	/// assert!(lst.is_empty());
	///
	/// lst.push_back(1);
	/// assert!(!lst.is_empty());
	/// ```
	fn is_empty ( &self ) -> bool
	{
		return self.end == 0;
	}

	/// Returns the element at the specified index.  
	/// Equivalent to `array[index];`  
	/// There are no safety checks as it is expected you know how to index an array.  
	///
	/// # Examples
	/// ```
	/// use star_tracker_lib::util::list::ArrayList;
	/// use star_tracker_lib::util::list::List;
	///
	/// let mut lst: ArrayList<i32, 2> = ArrayList::new();
	/// lst.push_back(1);
	/// assert_eq!(1, lst.get(0));
	/// ```
	fn get ( &self, index: usize ) -> T
	{
		assert!(index < self.size(), "Out of Bounds");
		return self.array[index].clone();
	}

	/// Sets the element at the specified index.  
	/// Equivalent to `array[index] = value;`  
	///  
	/// # Examples
	/// ```
	/// use star_tracker_lib::util::list::ArrayList;
	/// use star_tracker_lib::util::list::List;
	/// use star_tracker_lib::util::err::Errors;
	///
	/// let mut lst: ArrayList<i32, 2> = ArrayList::new();
	/// lst.push_back(0);
	/// lst.set(0, 10);
	/// assert_eq!(10, lst.get(0));
	/// assert_eq!(lst.set(1, 10), Err(Errors::OutOfBounds)); // Out of bounds.
	/// ```
	fn set ( &mut self, index: usize, value: T ) -> Error<()>
	{
		if self.size() <= index
		{
			return Err(Errors::OutOfBounds);
		}
		self.array[index] = value;
		return Ok(());
	}


	/// Adds an element to the end of the list with the provided value.  
	/// # Returns
	/// If is_full() Errors::InvalidSize  
	/// else         Ok(())  
	/// # Examples
	/// ```
	/// use star_tracker_lib::util::list::ArrayList;
	/// use star_tracker_lib::util::list::List;
	/// use star_tracker_lib::util::err::Errors;
	/// 
	/// let mut lst: ArrayList<u32, 2> = ArrayList::new();
	/// lst.push_back(2);
	/// lst.push_back(1);
	/// assert_eq!(lst.push_back(1), Err(Errors::InvalidSize)); // Full
	/// assert_eq!(lst.get(0), 2);
	/// assert_eq!(lst.get(1), 1);
	/// ```
	fn push_back ( &mut self, value: T ) -> Error<()>
	{
		if self.capacity() <= self.size()
		{
			return Err(Errors::InvalidSize);
		}
		self.array[self.end] = value.clone();
		self.end += 1;
		return Ok(());
	}

	/// Removes an element from the end of the list.  
	/// # Returns
	/// If is_empty() The value removed.  
	/// else          Errors::InvalidSize.  
	/// # Examples
	/// ```
	/// use star_tracker_lib::util::list::ArrayList;
	/// use star_tracker_lib::util::list::List;
	/// use star_tracker_lib::util::err::Errors;
	///
	/// let mut lst: ArrayList<u32, 2> = ArrayList::new();
	/// lst.push_back(2);
	/// lst.push_back(1);
	/// assert_eq!(lst.pop_back(), Ok(1));
	/// assert_eq!(lst.pop_back(), Ok(2));
	/// assert_eq!(lst.pop_back(), Err(Errors::InvalidSize));
	/// ```
	fn pop_back ( &mut self ) -> Error<T>
	{
		if self.is_empty()
		{
			return Err(Errors::InvalidSize);
		}
		self.end -= 1;
		return Ok(self.array[self.end].clone());
	}


	/// Sets the counter to 0 so all elements will be override and the list is essentialy cleared.
	/// # Examples
	/// ```
	/// use star_tracker_lib::util::list::ArrayList;
	/// use star_tracker_lib::util::list::List;
	///
	/// let mut lst: ArrayList<u32, 2> = ArrayList::new();
	/// lst.push_back(2);
	/// lst.push_back(1);
	///
	/// // The list is now empty.
	/// lst.clear();
	/// assert_eq!(0, lst.size());
	/// ```
	fn clear ( &mut self )
	{
		self.end = 0;
	}


	/// Sorts the list.  
	/// VERY SLOW!!!  
	/// # Arguments
	/// * `in_order` - A function which returns TRUE if it is in order.  
	///  
	/// This means that the first argument is a smaller index than the second.
	///
	/// # Examples
	/// ```
	/// use star_tracker_lib::util::list::ArrayList;
	/// use star_tracker_lib::util::list::List;
	/// fn ascending ( small: & u32, large: & u32 ) -> bool { return small < large; }
	///
	/// let mut lst: ArrayList<u32, 2> = ArrayList::new();
	/// lst.push_back(1);          // 1
	/// lst.push_back(0);          // 1 0
	/// lst.sort_order(ascending);
	/// assert_eq!(lst.get(0), 0);
	/// assert_eq!(lst.get(1), 1); // 0 1
	/// ```
	fn sort_order ( &mut self, in_order: fn (& T, & T) -> bool )
	{
		for ii in 0..self.size()
		{
			let mut jj: usize = ii;

			let mut temp: T = self.array[jj].clone();
			while jj > 0 && in_order(&mut temp, &mut self.array[jj - 1])
			{
				self.array[jj] = self.array[jj - 1].clone();
				jj -= 1;
			}
			self.array[jj] = temp;
		}
	}

	/// Slots an element into the list so it is in sorted order by shifting everything right.
	/// # Arguments
	/// * `to_slot` - The element to add.
	/// * `in_order` - A function which returns TRUE if it is in order.
	///  
	/// This means that the first argument is a smaller index than the second.
	///
	/// # Returns
	/// True if inserted, false if there is no space and it will trail the last element
	///
	/// # Examples
	/// ```
	/// use star_tracker_lib::util::list::ArrayList;
	/// use star_tracker_lib::util::list::List;
	///	fn sort_ascending ( small: & i32, large: & i32 ) -> bool { return small < large; }
	///
	/// let mut input: ArrayList<i32, 6> = ArrayList::new();
	///
	/// // The list is not full, elements will be added until it is full.
	/// let mut to_slot = 0;
	/// assert!(input.slot(to_slot, sort_ascending)); // 0
	/// to_slot = 2;
	/// assert!(input.slot(to_slot, sort_ascending)); // 0 2
	/// to_slot = 3;
	/// assert!(input.slot(to_slot, sort_ascending)); // 0 2 3
	/// to_slot = 1;
	/// assert!(input.slot(to_slot, sort_ascending)); // 0 1 2 3
	/// to_slot = 4;
	/// assert!(input.slot(to_slot, sort_ascending)); // 0 1 2 3 4
	/// to_slot = 5;
	/// assert!(input.slot(to_slot, sort_ascending)); // 0 1 2 3 4 5
	///
	/// // The list is full, every element added now will push an element out of the list.
	/// to_slot = -1;
	/// assert!(input.slot(to_slot, sort_ascending)); // -1 0 1 2 3 4  | 5
	/// to_slot = -2;
	/// assert!(input.slot(to_slot, sort_ascending)); // -2 -1 0 1 2 3 | 4
	/// to_slot = 1;
	/// assert!(input.slot(to_slot, sort_ascending)); // -2 -1 0 1 1 2 | 3
	/// to_slot = 10; // Too big to fit
	/// assert!(!input.slot(to_slot, sort_ascending));// -2 -1 0 1 1 2 | 10
	/// ```
	fn slot ( &mut self, to_slot: T, in_order: fn (& T, & T) -> bool ) -> bool
	{
		for ii in 0..self.size()
		{
			// If must slot in the middle.
			if in_order(&to_slot, &self.get(ii))
			{
				let mut to_move: T;
				let mut to_insert = to_slot.clone();
				let mut jj = ii;
				while jj < self.size()
				{
					to_move = self.get(jj);
					self.set(jj, to_insert).expect("Why cannot this be set?");
					to_insert = to_move.clone();
					jj+=1;
				}

				if jj < N
				{
					self.push_back(to_insert).expect("There should be more room.");
				}

				return true;
			}
		}

		// If there is room to add it at the end.
		if self.size() < N
		{
			self.push_back(to_slot).expect("There should be more room.");
			return true;
		}

		// Nowhere to fit.
		return false;
	}
}



impl <T, const N: usize> Clone for ArrayList<T, N> where T:Clone
{
	fn clone ( &self ) -> Self
	{
		let mut list: Self = ArrayList::new();
		for i in 0..self.size()
		{
			let _ = list.push_back(self.get(i));
		}
		return list;
	}
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
	use crate::util::list::List;
	use crate::util::list::ArrayList;
	use crate::util::err::Errors;
	use crate::util::err::Error;

//###############################################################################################//
//
//										Constructors
// pub fn new ( ) -> Self
// pub fn from_array ( &[T; N] ) -> Error<Self>
//
//###############################################################################################//

	#[test]
	// A newely constructed array list should have 0 size.
	fn test_new ( )
	{
		let list: ArrayList<u32, 10> = ArrayList::new();
		assert_eq!(list.end, 0);
		assert_eq!(list.array.len(), 10);
	}

	#[test]
	// Test if the array is filling the whole array list.
	// Every value should be assigned in the list.
	fn test_from_array ( ) -> Error<()>
	{
		let array: [u32; 5] = [2, 4, 6, 8, 10];
		let list: ArrayList<u32, 5> = ArrayList::from_array(&array);
		assert_eq!(list.get(0), 2);
		assert_eq!(list.get(1), 4);
		assert_eq!(list.get(2), 6);
		assert_eq!(list.get(3), 8);
		assert_eq!(list.get(4), 10);
		return Ok(());
	}
	
	
//###############################################################################################//
//
//										Basic Accessors
//
// pub fn capacity  ( &self ) -> usize
// pub fn size      ( &self ) -> usize
// pub fn is_full   ( &self ) -> bool
// pub fn is_empty  ( &self ) -> bool
//
// pub fn get       ( &self, index: usize ) -> T
// pub fn set       ( &mut self, index: usize, value: T ) -> Error<()>
// pub fn push_back ( &mut self, value: T ) -> Error<()>
// pub fn pop_back  ( &mut self, value: T ) -> Error<T>
//
//###############################################################################################//
//										~ capacity ~											 //
	#[test]
	// Capacity will always return the input size of the array list.
	fn test_capacity ( )
	{
		let lst: ArrayList<u32, 10> = ArrayList::new();
		assert_eq!(lst.capacity(), 10);
	}

//										~ size ~												 //
	#[test]
	// Size will return the number of elements in the array.
	fn test_size ( )
	{
		let mut lst: ArrayList<i32, 2> = ArrayList::new();
		assert_eq!(lst.size(), 0);
		lst.push_back(1);
		assert_eq!(lst.size(), 1);
		lst.push_back(2);
		assert_eq!(lst.size(), 2);
	}
	
	
//										~ is_full ~												 //
	#[test]
	// If the list has 0 capacity, it is always full.
	fn test_is_full_size_0 ( )
	{
		let lst: ArrayList<i32, 0> = ArrayList::new();
		assert!(lst.is_full());
	}
	
	#[test]
	// If the list has N capacity, it is only full when there is no room left.
	fn test_is_full_size_1 ( )
	{
		let mut lst: ArrayList<f32, 1> = ArrayList::new();
		lst.push_back(0.1);
		assert!(lst.is_full());
	}
	
	#[test]
	// If the list has N capacity, it will not be full unless there is no room.
	fn test_is_full_size_1_empty ( )
	{
		let lst: ArrayList<f32, 1> = ArrayList::new();
		assert!(!lst.is_full());
	}


//										~ is_empty ~											 //
	#[test]
	// If the list has no capacity, it is empty.
	fn test_is_empty_size_0 ( )
	{
		let lst: ArrayList<u32, 0> = ArrayList::new();
		assert!(lst.is_empty());
	}
	
	#[test]
	// If the list has N capacity, it is empty when there are no elements.
	fn test_is_empty_size_1 ( )
	{
		let lst: ArrayList<u32, 1> = ArrayList::new();
		assert!(lst.is_empty());
	}
	
	#[test]
	// If the list has N capacity, it is not empty when there are elements contained.
	fn test_is_empty_size_1_full ( )
	{
		let mut lst: ArrayList<u32, 1> = ArrayList::new();
		lst.push_back(1);
		assert!(!lst.is_empty());
	}


//										~ get ~													 //
	#[test]
	// If the get tries to access an element inside of the bounds, it should access it.
	fn test_get_valid ( )
	{
		let mut lst: ArrayList<u32, 2> = ArrayList::new();
		lst.push_back(0);
		lst.push_back(1);
		let first  = lst.get(0);
		let second = lst.get(1);
		assert_eq!(first, 0);
		assert_eq!(second, 1);
	}
	
	#[test]
	// The array must own a copy of the values it creates.
	fn test_get_ownership ( )
	{
		let mut lst: ArrayList<u32, 1> = ArrayList::new();
		let mut val = 2;
		lst.push_back(val);
		
		// Test removal of ownership.
		val = 10;
		assert!(val != lst.get(0));
	}
	
	#[test]
	#[should_panic = "Out of Bounds"]
	// get() is core to everything, it was deemed too difficult to do error checking.
	// Error checking would also slow the program.
	fn test_get_not_enough_elements ( )
	{
		let lst: ArrayList<u32, 1> = ArrayList::from_array(&[1]);
		lst.get(0); // to satisfy lcov
		lst.get(1);
	}


//										~ set ~													 //
	#[test]
	// Should correctly set elements in the list.
	// Elements should be persistant.
	fn test_set_valid ( )
	{
		let mut lst: ArrayList<u32, 2> = ArrayList::new();
		lst.push_back(0);
		lst.push_back(1);
		lst.set(0, 10);
		lst.set(1, 22);
		assert_eq!(lst.get(0), 10);
		assert_eq!(lst.get(1), 22);
	}
	
	#[test]
	// SHOULD FAIL.
	// If there is no elements in the list, an error is returned.
	fn test_set_not_enough_elements ( )
	{
		let mut lst: ArrayList<u32, 1> = ArrayList::new();
		assert!(lst.set(0, 0).is_err());
	}

	#[test]
	// SHOULD FAIL.
	// If there is no elements in the list, an error is returned.
	fn test_set_out_of_bounds ( )
	{
		let mut lst: ArrayList<u32, 0> = ArrayList::new();
		assert!(lst.set(0, 0).is_err());
	}



//										~ push_back ~											 //
	#[test]
	// When there is room, the array list can add elements to the first available slot.
	fn test_push_back_valid ( )
	{
		let mut lst: ArrayList<u32, 2> = ArrayList::new();
		assert_eq!(lst.push_back(1), Ok(()));
		assert_eq!(lst.push_back(2), Ok(()));
		assert_eq!(lst.get(0), 1);
		assert_eq!(lst.get(1), 2);
	}

	#[test]
	// When there is no room, the list should output an error.
	fn test_push_back_invalid ( )
	{
		let mut lst: ArrayList<u32, 0> = ArrayList::new();
		assert_eq!(lst.push_back(1), Err(Errors::InvalidSize));
	}



//										~ pop_back ~											 //
	#[test]
	// When there is at least one element, the list should be able to pop the element off the end.
	// The returned value should be this value.
	fn test_pop_back_valid ( )
	{
		let mut lst: ArrayList<u32, 2> = ArrayList::new();
		lst.push_back(0);
		lst.push_back(1);
		assert_eq!(lst.pop_back(), Ok(1));
		assert_eq!(lst.pop_back(), Ok(0));
	}

	#[test]
	// When there is no elements left to be popped, an error should be output.
	fn test_pop_back_invalid ( )
	{
		let mut lst: ArrayList<u32, 0> = ArrayList::new();
		assert_eq!(lst.pop_back(), Err(Errors::InvalidSize));
	}




//###############################################################################################//
//
//										Extra Functionality
//
// pub fn clear      ( &mut self );
// pub fn sort_order ( &mut self, fn (&T, &T) -> bool );
// pub fn slot       ( &mut self, T, fn (&T, &T) -> bool ) -> bool
//
//###############################################################################################//

//										~ clear ~												 //
	#[test]
	// Clear should set list to 0 and override any values when pushback occures.
	fn test_clear ( )
	{
		let mut lst: ArrayList<u32, 2> = ArrayList::new();
		lst.push_back(2);
		lst.push_back(1);
		lst.clear();
		assert_eq!(0, lst.size());
		lst.push_back(10);
		assert_eq!(1, lst.size());
		assert_eq!(10, lst.get(0));
	}

//										~ sort ~												 //
	fn sort_ascending ( lowest: & i32, highest: & i32 ) -> bool {return lowest < highest;}
	fn sort_descending ( highest: & i32, lowest: & i32 ) -> bool {return lowest < highest;}

	#[test]
	// All elements should be in sorted ascending order after being passed through the function.
	fn test_sort_order_ascending ( )
	{
		let mut lst: ArrayList<i32, 3> = ArrayList::new();
		lst.push_back(2);
		lst.push_back(1);
		lst.push_back(0);
		lst.sort_order(sort_ascending);
		assert_eq!(lst.pop_back(), Ok(2));
		assert_eq!(lst.pop_back(), Ok(1));
		assert_eq!(lst.pop_back(), Ok(0));
	}
	
	#[test]
	// All elements should be in sorted descending order after being passed through the function.
	fn test_sort_order_descending ( )
	{
		let mut lst: ArrayList<i32, 3> = ArrayList::new();
		lst.push_back(0);
		lst.push_back(1);
		lst.push_back(2);
		lst.sort_order(sort_descending);
		assert_eq!(lst.pop_back(), Ok(0));
		assert_eq!(lst.pop_back(), Ok(1));
		assert_eq!(lst.pop_back(), Ok(2));
	}
	
	#[test]
	// Nothing should go wrong.
	fn test_sort_order_empty ( )
	{
		let mut lst: ArrayList<i32, 0> = ArrayList::new();
		lst.sort_order(sort_ascending);
	}





//										~ slot ~												 //
	#[test]
	// When the list is not full, the elements should fill in sorted order.
	fn test_slot_ascending_not_full ( )
	{
		let mut input: ArrayList<i32, 6> = ArrayList::new();

		let mut to_slot = 0;
		assert!(input.slot(to_slot, sort_ascending)); // 0
		to_slot = 2;
		assert!(input.slot(to_slot, sort_ascending)); // 0 2
		to_slot = 3;
		assert!(input.slot(to_slot, sort_ascending)); // 0 2 3
		to_slot = 1;
		assert!(input.slot(to_slot, sort_ascending)); // 0 1 2 3
		to_slot = 5;
		assert!(input.slot(to_slot, sort_ascending)); // 0 1 2 3 5
		to_slot = 4;
		assert!(input.slot(to_slot, sort_ascending)); // 0 1 2 3 4 5

		assert_eq!(input.size(), 6);
	}
	
	#[test]
	// Once full, the list should kick the highest value out.
	// If the value to add is kicked, the function should return false.
	fn test_slot_ascending_full ( )
	{ 
		let mut input: ArrayList<i32, 6> = ArrayList::from_array(&[0, 1, 2, 3, 4, 5]);

		let mut to_slot = -1;
		assert!(input.slot(to_slot, sort_ascending));// -1  0  1  2  3  4
		to_slot = -2;
		assert!(input.slot(to_slot, sort_ascending));// -2 -1  0  1  2  3
		to_slot = 1;
		assert!(input.slot(to_slot, sort_ascending));// -2 -1  0  1  1  2
		to_slot = 10;
		assert!(!input.slot(to_slot, sort_ascending));//-2 -1  0  1  1  2

		assert_eq!(input.get(0), -2);
		assert_eq!(input.get(1), -1);
		assert_eq!(input.get(2), 0);
		assert_eq!(input.get(3), 1);
		assert_eq!(input.get(4), 1);
		assert_eq!(input.get(5), 2);
	}


	#[test]
	// When the list is not full, the elements should fill in sorted order.
	fn test_slot_descending_not_full ( )
	{
		let mut input: ArrayList<i32, 6> = ArrayList::new();

		let mut to_slot = -1;
		assert!(input.slot(to_slot, sort_descending)); // -1
		to_slot = 0;
		assert!(input.slot(to_slot, sort_descending)); // 0  -1
		to_slot = 1;
		assert!(input.slot(to_slot, sort_descending)); // 1   0  -1
		to_slot = 2;
		assert!(input.slot(to_slot, sort_descending)); // 2   1   0   -1
		to_slot = 3;
		assert!(input.slot(to_slot, sort_descending)); // 3   2   1   0  -1
		to_slot = 100;
		assert!(input.slot(to_slot, sort_descending)); // 100 3   2   1   0   -1
	}
	
	#[test]
	// Once full, the list should kick the lowest value out.
	// If the value to add is kicked, the function should return false.
	fn test_slot_descending_full ( )
	{
		let mut input: ArrayList<i32, 6> = ArrayList::from_array(&[100,3,2, 1, 0, -1]);
		
		assert_eq!(input.size(), 6);
		// Full, 100, 3, 2, 1, 0, -1
		let mut to_slot = -1;
		assert!(!input.slot(to_slot, sort_descending)); // 100, 3, 2, 1, 0, -1
		to_slot = 5;
		assert!(input.slot(to_slot, sort_descending)); // 100, 5, 3, 2, 1, 0
		to_slot = 1;
		assert!(input.slot(to_slot, sort_descending)); // 100, 5, 3, 2, 1, 1
		to_slot = 101;
		assert!(input.slot(to_slot, sort_descending)); // 101, 100, 5, 3, 2, 1
		to_slot = -100;
		assert!(!input.slot(to_slot, sort_descending)); // 101, 100, 5, 3, 2, 1

		assert_eq!(input.get(0), 101);
		assert_eq!(input.get(1), 100);
		assert_eq!(input.get(2), 5);
		assert_eq!(input.get(3), 3);
		assert_eq!(input.get(4), 2);
		assert_eq!(input.get(5), 1);
	}

//###############################################################################################//
//
//										Clone
//
/// pub fn clone ( &self ) -> Self
//
//###############################################################################################//

	#[test]
	fn test_clone ( )
	{
		let a : ArrayList<u32, 4> = ArrayList::from_array(&[10, 20, 30, 40]);
		let b = a.clone();
		
		assert_eq!(a.get(0), b.get(0));
		assert_eq!(a.get(1), b.get(1));
		assert_eq!(a.get(2), b.get(2));
		assert_eq!(a.get(3), b.get(3));
	}
}





















