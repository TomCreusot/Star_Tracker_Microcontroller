//! Implementation of [List](crate::util::list::List)
use crate::util::list::List;

use crate::util::err::Errors;
use crate::util::err::Error;

impl <T> List <T> for Vec <T> where T: Clone
{
	/// Returns usize::max_value().  
	/// A Vec will not exceed capacity as it will dynamically resize.  
	/// This conflicts with Vec, to use, follow the example.
	/// # Example
	/// ```
	/// use star_tracker::util::list::List;
	/// let vec: Vec<usize> = Vec::new();
	/// List::capacity(&vec); // This is how to call conflicting packages.
	/// ```
	fn capacity ( &self ) -> usize
	{
		return usize::max_value();
	}

	/// Finds how many elements are in the list.  
	/// Equivalent to [Vec.len()](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.len).
	fn size ( &self ) -> usize
	{
		return self.len() as usize;
	}

	/// Returns false.  
	/// A vec is never full as it will dynamically resize.  
	fn is_full ( &self ) -> bool
	{
		return false;
	}

	/// Returns true if empty.  
	/// Equivalent to [Vec::is_empty()](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.is_empty).
	fn is_empty ( &self ) -> bool
	{
		return self.len() == 0;
	}

	/// Gets the element at the specified index.  
	/// Equivalent to `vec[index];`
	fn get ( &self, index: usize ) -> T
	{
		return self[index].clone();
	}

	/// Sets the element at the specified index.
	/// Equivalent to `vec[index] = value;`
	fn set ( &mut self, index: usize, value: T ) -> Error<()>
	{
		if index < self.size()
		{
			self[index] = value;
			return Ok(());
		}
		return Err(Errors::OutOfBounds);
	}

	/// Adds an element to the end of the list.
	/// Equivalent to [vec.push(value)](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push).
	fn push_back ( &mut self, value: T ) -> Error<()>
	{
		if self.size() < (self as &dyn List<T>).capacity()
		{
			self.push(value);
			return Ok(());
		}
		return Err(Errors::InvalidSize);
	}


	/// Removes an element from the end of the list.
	/// Equivalent to [vec.pop()](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.pop).
	fn pop_back ( &mut self ) -> Error<T>
	{
		if self.is_empty()
		{
			return Err(Errors::InvalidSize);
		}
		return Ok(self.pop().unwrap());
	}


	/// Sets the counter to 0 so all elements will be override and the list is essentialy cleared.
	/// Equivalent to [vec.clear()](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.clear).
	fn clear ( &mut self )
	{
		self.clear();
	}

	/// Sorts list based on the input function.  
	/// VERY SLOW!!!
	/// The list uses the same logic as the array list.
	/// # Arguments
	/// * 'in_order' - A function which returns TRUE if it is in order.
	/// # Examples
	/// ```
	/// use star_tracker::util::list::List;
	/// fn ascending ( small: & u32, large: & u32 ) -> bool { return small < large; }
	///
	/// let mut lst: Vec<u32> = Vec::new();
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

			let mut temp: T = self.get(jj).clone();
			while jj > 0 && in_order(&mut temp, &mut self.get(jj - 1))
			{
				self.set(jj, self.get(jj - 1)).expect("This should be within the bounds.");
				jj -= 1;
			}
			self.set(jj, temp).expect("This should be within the bounds.");
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
	/// use star_tracker::util::list::List;
	///	fn sort_ascending ( small: & i32, large: & i32 ) -> bool { return small < large; }
	///
	/// let mut input: Vec<i32> = Vec::new();
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
	/// to_slot = 5;
	/// assert!(input.slot(to_slot, sort_ascending)); // 0 1 2 3 5
	/// to_slot = 4;
	/// assert!(input.slot(to_slot, sort_ascending)); // 0 1 2 3 4 5
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
					self.set(jj, to_insert).expect("This should be within the bounds.");
					to_insert = to_move.clone();
					jj+=1;
				}

				self.push_back(to_insert).expect("Vec should not run out of space.");
				return true;
			}
		}
		self.push_back(to_slot).expect("Vec should not run out of space.");
		return true;
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
	use crate::util::err::Errors;


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
	// Capacity in a vec is near infinite.
	fn test_capacity ( )
	{
		let lst: Vec<u32> = Vec::new();
		assert_eq!(List::capacity(&lst), usize::max_value());
	}

//										~ size ~												 //
	#[test]
	// Size will return the number of elements in the array.
	fn test_size ( )
	{
		let mut lst: Vec<i32> = Vec::new();
		assert_eq!(lst.size(), 0);
		lst.push_back(1);
		assert_eq!(lst.size(), 1);
		lst.push_back(2);
		assert_eq!(lst.size(), 2);
	}
	
	
//										~ is_full ~												 //
	#[test]
	// The list is never full.
	fn test_is_full_size_0 ( )
	{
		let lst: Vec<i32> = Vec::new();
		assert!(!lst.is_full());
	}
	
	#[test]
	// The list is never full.
	fn test_is_full_size_1 ( )
	{
		let mut lst: Vec<f32> = Vec::new();
		lst.push_back(0.1);
		assert!(!lst.is_full());
	}
	
	#[test]
	// The list is never full.
	fn test_is_full_size_1_empty ( )
	{
		let lst: Vec<f32> = Vec::new();
		assert!(!lst.is_full());
	}


//										~ is_empty ~											 //
	#[test]
	// If the list has no capacity, it is empty.
	fn test_is_empty_size_0 ( )
	{
		let lst: Vec<u32> = Vec::new();
		assert!(lst.is_empty());
	}
	
	#[test]
	// If the list has N capacity, it is empty when there are no elements.
	fn test_is_empty_size_1 ( )
	{
		let lst: Vec<u32> = Vec::new();
		assert!(lst.is_empty());
	}
	
	#[test]
	// If the list has N capacity, it is not empty when there are elements contained.
	fn test_is_empty_size_1_full ( )
	{
		let mut lst: Vec<u32> = Vec::new();
		lst.push_back(1);
		assert!(!lst.is_empty());
	}


//										~ get ~													 //
	#[test]
	// If the get tries to access an element inside of the bounds, it should access it.
	fn test_get_valid ( )
	{
		let mut lst: Vec<u32> = Vec::new();
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
		let mut lst: Vec<u32> = Vec::new();
		let mut val = 2;
		lst.push_back(val);
		
		// Test removal of ownership.
		val = 10;
		assert!(val != lst.get(0));
	}
	
	#[test]
	#[should_panic]
	// get() is core to everything, it was deemed too difficult to do error checking.
	// Error checking would also slow the program.
	fn test_get_not_enough_elements ( )
	{
		let lst: Vec<u32> = Vec::new();
		lst.get(1);
	}
	
	#[test]
	#[should_panic]
	// get() is core to everything, it was deemed too difficult to do error checking.
	// Error checking would also slow the program.
	fn test_get_out_of_bounds ( )
	{
		let lst: Vec<u32> = Vec::new();
		lst.get(11);
	}


//										~ set ~													 //
	#[test]
	// Should correctly set elements in the list.
	// Elements should be persistant.
	fn test_set_valid ( )
	{
		let mut lst: Vec<u32> = Vec::new();
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
		let mut lst: Vec<u32> = Vec::new();
		assert!(lst.set(0, 0).is_err());
	}

	#[test]
	// SHOULD FAIL.
	// If there is no elements in the list, an error is returned.
	fn test_set_out_of_bounds ( )
	{
		let mut lst: Vec<u32> = Vec::new();
		assert!(lst.set(0, 0).is_err());
	}



//										~ push_back ~											 //
	#[test]
	// There is room, the array list can add elements to the first available slot.
	fn test_push_back_valid ( )
	{
		let mut lst: Vec<u32> = Vec::new();
		assert_eq!(lst.push_back(1), Ok(()));
		assert_eq!(lst.push_back(2), Ok(()));
		assert_eq!(lst.get(0), 1);
		assert_eq!(lst.get(1), 2);
	}




//										~ pop_back ~											 //
	#[test]
	// When there is at least one element, the list should be able to pop the element off the end.
	// The returned value should be this value.
	fn test_pop_back_valid ( )
	{
		let mut lst: Vec<u32> = Vec::new();
		lst.push_back(0);
		lst.push_back(1);
		assert_eq!(lst.pop_back(), Ok(1));
		assert_eq!(lst.pop_back(), Ok(0));
	}

	#[test]
	// When there is no elements left to be popped, an error should be output.
	fn test_pop_back_invalid ( )
	{
		let mut lst: Vec<u32> = Vec::new();
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
		let mut lst: Vec<u32> = Vec::new();
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
		let mut lst: Vec<i32> = Vec::new();
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
		let mut lst: Vec<i32> = Vec::new();
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
		let mut lst: Vec<i32> = Vec::new();
		lst.sort_order(sort_ascending);
	}





//										~ slot ~												 //
	#[test]
	// A vec is never full, the elements should fill in sorted order.
	fn test_slot_ascending_not_full ( )
	{
		let mut input: Vec<i32> = Vec::new();

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
	// A vec is never full, the elements should fill in sorted order.
	fn test_slot_descending_not_full ( )
	{
		let mut input: Vec<i32> = Vec::new();

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
}
