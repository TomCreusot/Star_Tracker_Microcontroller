//! The implementation of ArrayList.

use super::{List, ArrayList};
use util::err::{Error, Errors};

//###############################################################################################//
//								---	ArrayList Constructor ---
//###############################################################################################//

impl <T, const N: usize> ArrayList <T, N>
{
	/// Constructor
	pub fn new ( ) -> ArrayList<T, N>
	{
		use std::mem::{MaybeUninit};
		ArrayList
		{
			array:  unsafe{MaybeUninit::uninit().assume_init()}, // Auto inits array.
			end:    0,
		}
	}
}



//###############################################################################################//
//								---	ListIterator Implementation ---
//###############################################################################################//
/*impl<'a, T, const N : usize> IntoIterator for &'a ArrayList<T, N> where T: Clone {
    type Item = T;
    type IntoIter = ListIterator<'a, T>;

	/// Creates an iterator for arraylist.
    fn into_iter(self) -> Self::IntoIter {
        ListIterator {
            list: self,
            index: 0,
        }
    }
}*/




//###############################################################################################//
//								---	ArrayList Implementation ---
//###############################################################################################//
impl<T, const N : usize> List<T> for ArrayList<T, N> where T: Clone
{

	/// Finds the max number of elements that can be stored in the list.
	/// # Example
	/// ```
	/// use star_tracker::util::list::{ArrayList, List};
	/// let lst : ArrayList<u32, 10> = ArrayList::new();
	/// assert_eq!(lst.capacity(), 10);
	/// //unsafe
	/// //{
	/// //	const size : usize = lst.capacity() as const usize;
	/// //	let lst2 : ArrayList<u32, size> = ArrayList::new();
	/// //}
	/// ```
	fn capacity ( &self ) -> usize
	{
		return N;
	}


	/// Finds how many elements are in the list.
	/// # Returns
	/// The size.
	///
	/// # Example
	/// ```
	/// use star_tracker::util::list::{ArrayList, List};
	/// let mut lst : ArrayList<u32, 10> = ArrayList::new();
	/// assert_eq!(0, lst.size());
	/// lst.push_back(1);
	/// assert_eq!(1, lst.size());
	/// lst.push_back(1);
	/// assert_eq!(2, lst.size());
	/// ```
	fn size ( &self ) -> usize
	{
		return self.end;
	}


	/// Checks if the ArrayList is at maximum capacity.
	/// # Returns
	/// True if full.
	///
	/// # Example
	/// ```
	/// use star_tracker::util::list::{ArrayList, List};
	/// let mut lst : ArrayList<u32, 2> = ArrayList::new();
	/// lst.push_back(1);
	/// assert!(!lst.is_full());
	/// lst.push_back(2);
	/// assert!(lst.is_full());
	/// ```
	fn is_full ( &self ) -> bool
	{
		return self.end == self.array.len();
	}


	/// Checks if the ArrayList is at maximum capacity.
	/// # Returns
	/// True if full.
	///
	/// # Example
	/// ```
	/// use star_tracker::util::list::{ArrayList, List};
	/// let mut lst : ArrayList<i32, 2> = ArrayList::new();
	/// assert!(lst.is_empty());
	/// lst.push_back(1);
	/// assert!(!lst.is_empty());
	/// ```
	fn is_empty ( &self ) -> bool
	{
		return self.end == 0;
	}

	/// Gets the element at the specified index.
	/// # Arguments
	///	* `index` - The index of the
	/// # Returns
	/// The value at the index.
	///
	/// # Example
	/// ```
	/// use star_tracker::util::list::{ArrayList, List};
	/// let mut lst : ArrayList<i32, 2> = ArrayList::new();
	/// lst.push_back(1);
	/// assert_eq!(1, lst.get(0));
	/// ```
	fn get ( &self, index : usize ) -> T
	{
		assert!(index < self.end, "Out of bounds");
		return self.array[index].clone();
	}

	/// Sets the element at the specified index.
	/// # Arguments
	///	* `index` - The index of the element to receive.
	/// * `value` - The value to assign.
	///
	/// # Example
	/// ```
	/// use star_tracker::util::list::{ArrayList, List};
	/// let mut lst : ArrayList<i32, 2> = ArrayList::new();
	/// lst.push_back(0);
	/// lst.push_back(1);
	/// lst.set(0, 10);
	/// lst.set(1, 5);
	/// assert_eq!(10, lst.get(0));
	/// assert_eq!(5, lst.get(1));
	/// ```
	fn set ( &mut self, index : usize, value : T ) -> Error<()>
    {
		if self.size() <= index
		{
			return Err(Errors::OutOfBounds);
		}
        self.array[index] = value;
		return Ok(());
    }


	/// Adds an element to the end of the list.
	/// # Arguments
	/// * `value` - the value to add to the end.
	///
	/// # Example
	/// ```
	/// use star_tracker::util::list::{ArrayList, List};
	/// let mut lst : ArrayList<u32, 2> = ArrayList::new();
	/// lst.push_back(2);
	/// lst.push_back(1);
	/// assert_eq!(lst.get(0), 2);
	/// assert_eq!(lst.get(1), 1);
	/// assert!(lst.is_full());
	/// ```
	fn push_back ( &mut self, value : T ) -> Error<()>
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
	/// The value removed.
	///
	/// # Example
	/// ```
	/// use star_tracker::util::list::{ArrayList, List};
	/// let mut lst : ArrayList<u32, 2> = ArrayList::new();
	/// lst.push_back(2);
	/// lst.push_back(1);
	/// assert_eq!(lst.pop_back(), 1);
	/// assert_eq!(lst.pop_back(), 2);
	/// assert!(lst.is_empty());
	/// ```
	fn pop_back ( &mut self ) -> T
	{
		assert!(!self.is_empty(), "List is empty");
		self.end -= 1;
		return self.array[self.end].clone();
	}
	
	
	/// Sets the counter to 0 so all elements will be override and the list is essentialy cleared.
	/// # Example
	/// ``` 
	/// use star_tracker::util::list::{ArrayList, List};
	/// let mut lst : ArrayList<u32, 2> = ArrayList::new();
	/// lst.push_back(2);
	/// lst.push_back(1);
	/// lst.clear();
	/// assert_eq!(0, lst.size());
	/// lst.push_back(10);
	/// assert_eq!(1, lst.size());
	/// assert_eq!(10, lst.get(0));
	/// ```
	fn clear ( &mut self )
	{
		self.end = 0;
	}
	

	/// Sorts the list
	/// # Arguments
	/// * `in_order` - A function which returns TRUE if it is in order.
	///
	/// # Example
	/// ```
	/// use star_tracker::util::list::{ArrayList, List};
	/// fn ascending ( small : & u32, large : & u32 ) -> bool { return small < large; }
	/// let mut lst : ArrayList<u32, 2> = ArrayList::new();
	/// lst.push_back(1);
	/// lst.push_back(0);
	/// lst.sort_order(ascending);
	/// assert_eq!(lst.get(0), 0);
	/// assert_eq!(lst.get(1), 1);
	/// ```
	fn sort_order ( &mut self, in_order: fn (& T, & T) -> bool )
	{
		for ii in 0..self.size()
		{
			let mut jj : usize = ii;

			let mut temp : T = self.array[jj].clone();
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
	/// * `in_order` - The ordering method.
	///
	/// # Returns
	/// True if inserted, false if there is no space and it will trail the last element
	///
	/// # Example
	/// ```
	/// use star_tracker::util::list::{ArrayList, List};
	///	fn sort_ascending ( small : & i32, large : & i32 ) -> bool { return small < large; }
	/// let mut input : ArrayList<i32, 6> = ArrayList::new();
	///
	/// let mut to_slot = 0;
	/// assert!(input.slot(to_slot, sort_ascending));
	/// to_slot = 2;
	/// assert!(input.slot(to_slot, sort_ascending));
	/// to_slot = 3;
	/// assert!(input.slot(to_slot, sort_ascending));
	/// to_slot = 1;
	/// assert!(input.slot(to_slot, sort_ascending));
	/// to_slot = 4;
	/// assert!(input.slot(to_slot, sort_ascending));
	/// to_slot = 5;
	/// assert!(input.slot(to_slot, sort_ascending));
	///
	/// assert_eq!(input.size(), 6);
	/// // Full, 0, 1, 2, 3, 4, 5
	/// to_slot = -1;
	/// assert!(input.slot(to_slot, sort_ascending));//-1, 0, 1, 2, 3, 4
	/// to_slot = -2;
	/// assert!(input.slot(to_slot, sort_ascending));//-2, -1, 0, 1, 2, 3
	/// to_slot = 1;
	/// assert!(input.slot(to_slot, sort_ascending));//-2, -1, 0, 1, 1, 2
	/// to_slot = 10;
	/// assert!(!input.slot(to_slot, sort_ascending));//-2, -1, 0, 1, 1, 2
	///
	/// assert_eq!(input.get(0), -2);
	/// assert_eq!(input.get(1), -1);
	/// assert_eq!(input.get(2), 0);
	/// assert_eq!(input.get(3), 1);
	/// assert_eq!(input.get(4), 1);
	/// assert_eq!(input.get(5), 2);
	/// ```
	fn slot ( &mut self, to_slot : T, in_order: fn (& T, & T) -> bool ) -> bool
	{
		for ii in 0..self.size()
		{
			// If must slot in the middle.
			if in_order(&to_slot, &self.get(ii))
			{
				let mut to_move : T;
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
	use crate::util::list::{List, ArrayList};

	//
	// capacity ( ) -> const usize
	//
	#[test]
	fn test_capacity ( )
	{
		let lst : ArrayList<u32, 10> = ArrayList::new();
		assert_eq!(lst.capacity(), 10);
	}



//
// size ( ) -> usize
//
	#[test]
	fn test_size ( )
	{
		let mut lst : ArrayList<i32, 2> = ArrayList::new();
		assert_eq!(lst.size(), 0);
		lst.push_back(1);
		assert_eq!(lst.size(), 1);
		lst.push_back(2);
		assert_eq!(lst.size(), 2);
	}
//
//	is_full ( ) -> bool
//
	#[test]
	fn test_is_full_size_0 ( )
	{
		let lst : ArrayList<i32, 0> = ArrayList::new();
		assert!(lst.is_full());
	}
	#[test]
	fn test_is_full_size_1 ( )
	{
		let mut lst : ArrayList<f32, 1> = ArrayList::new();
		assert!(!lst.is_full());
		lst.push_back(0.1);
		assert!(lst.is_full());
	}




//
// is_empty ( ) -> bool
//
	#[test]
	fn test_is_empty_size_0 ( )
	{
		let lst : ArrayList<u32, 0> = ArrayList::new();
		assert!(lst.is_empty());
	}

/* FAILING???
*/
	#[test]
	fn test_is_empty_size_1 ( )
	{
		let mut lst : ArrayList<u32, 1> = ArrayList::new();
		assert!(lst.is_empty());
		lst.push_back(22);
		assert!(!lst.is_empty());
	}


//
// get(usize index) -> T
//
	#[test]
	fn test_get_read ( )
	{
		let mut lst : ArrayList<u32, 2> = ArrayList::new();
		lst.push_back(0);
		lst.push_back(1);
		let mut  first = lst.get(0);
		let second = lst.get(1);
		assert_eq!(first, 0);
		assert_eq!(second, 1);

		// Test removal of ownership.
		first = 10;
		assert!(first != lst.get(0));
	}

	#[test]
	#[should_panic = "Out of bounds"]
	fn test_get_not_enough_elements ( )
	{
		let lst : ArrayList<u32, 10> = ArrayList::new();
		lst.get(1);
	}
	#[test]
	#[should_panic = "Out of bounds"]
	fn test_get_out_of_bounds ( )
	{
		let lst : ArrayList<u32, 10> = ArrayList::new();
		lst.get(11);
	}


//
// set ( T )
//
	#[test]
	fn test_set_valid ( )
	{
		let mut lst : ArrayList<u32, 2> = ArrayList::new();
		lst.push_back(0);
		lst.push_back(1);
		lst.set(0, 10);
		lst.set(1, 22);
		assert_eq!(lst.get(0), 10);
		assert_eq!(lst.get(1), 22);
	}

	#[test]
	fn test_set_not_enough_elements ( )
	{
		let mut lst : ArrayList<u32, 1> = ArrayList::new();
		assert!(lst.set(0, 0).is_err());
	}

	#[test]
	fn test_set_out_of_bounds ( )
	{
		let mut lst : ArrayList<u32, 0> = ArrayList::new();
		assert!(lst.set(0, 0).is_err());
	}



//
//  pop_back ( T )
//
	#[test]
	fn test_push_back_valid ( )
	{
		let mut lst : ArrayList<u32, 2> = ArrayList::new();
		lst.push_back(0);
		lst.push_back(1);
		assert_eq!(lst.pop_back(), 1);
		assert_eq!(lst.pop_back(), 0);
	}

	#[test]
	#[should_panic = "List is empty"]
	fn test_push_back_invalid ( )
	{
		let mut lst : ArrayList<u32, 0> = ArrayList::new();
		lst.pop_back();
	}



//
//  pop_back ( )
//
	#[test]
	fn test_pop_back_valid ( )
	{
		let mut lst : ArrayList<u32, 2> = ArrayList::new();
		lst.push_back(0);
		lst.push_back(1);
		assert_eq!(lst.pop_back(), 1);
		assert_eq!(lst.pop_back(), 0);
	}

	#[test]
	#[should_panic = "List is empty"]
	fn test_pop_back_invalid ( )
	{
		let mut lst : ArrayList<u32, 0> = ArrayList::new();
		lst.pop_back();
	}




	//
	// clear ( &mut self )
	//
	
	#[test]
	// Clear should set list to 0 and override any values when pushback occures.
	fn test_clear ( )
	{
		let mut lst : ArrayList<u32, 2> = ArrayList::new();
		lst.push_back(2);
		lst.push_back(1);
		lst.clear();
		assert_eq!(0, lst.size());
		lst.push_back(10);
		assert_eq!(1, lst.size());
		assert_eq!(10, lst.get(0));
	}

//
// sort ( fn ( &mut T, &mut T ) -> bool )
//
	fn sort_ascending ( lowest : & i32, highest : & i32 ) -> bool {return lowest < highest;}
	fn sort_descending ( highest : & i32, lowest : & i32 ) -> bool {return lowest < highest;}

	#[test]
	fn test_sort_order_ascending ( )
	{
		let mut lst : ArrayList<i32, 3> = ArrayList::new();
		lst.push_back(2);
		lst.push_back(1);
		lst.push_back(0);
		lst.sort_order(sort_ascending);
		assert_eq!(lst.pop_back(), 2);
		assert_eq!(lst.pop_back(), 1);
		assert_eq!(lst.pop_back(), 0);
	}

	#[test]
	fn test_sort_order_descending ( )
	{
		let mut lst : ArrayList<i32, 3> = ArrayList::new();
		lst.push_back(0);
		lst.push_back(1);
		lst.push_back(2);
		lst.sort_order(sort_descending);
		assert_eq!(lst.pop_back(), 0);
		assert_eq!(lst.pop_back(), 1);
		assert_eq!(lst.pop_back(), 2);
	}

	#[test]
	fn test_sort_order_empty ( )
	{
		let mut lst : ArrayList<i32, 0> = ArrayList::new();
		lst.sort_order(sort_ascending);
	}




//
// slot ( to_slot : T, fn ( &mut T, &mut T ) -> bool ) -> bool
//
	#[test]
	fn test_slot_ascending ( )
	{
		let mut input : ArrayList<i32, 6> = ArrayList::new();

		let mut to_slot = 0;
		assert!(input.slot(to_slot, sort_ascending));
		to_slot = 2;
		assert!(input.slot(to_slot, sort_ascending));
		to_slot = 3;
		assert!(input.slot(to_slot, sort_ascending));
		to_slot = 1;
		assert!(input.slot(to_slot, sort_ascending));
		to_slot = 4;
		assert!(input.slot(to_slot, sort_ascending));
		to_slot = 5;
		assert!(input.slot(to_slot, sort_ascending));

		assert_eq!(input.size(), 6);
		// Full, 0, 1, 2, 3, 4, 5
		to_slot = -1;
		assert!(input.slot(to_slot, sort_ascending));//-1, 0, 1, 2, 3, 4
		to_slot = -2;
		assert!(input.slot(to_slot, sort_ascending));//-2, -1, 0, 1, 2, 3
		to_slot = 1;
		assert!(input.slot(to_slot, sort_ascending));//-2, -1, 0, 1, 1, 2
		to_slot = 10;
		assert!(!input.slot(to_slot, sort_ascending));//-2, -1, 0, 1, 1, 2

		assert_eq!(input.get(0), -2);
		assert_eq!(input.get(1), -1);
		assert_eq!(input.get(2), 0);
		assert_eq!(input.get(3), 1);
		assert_eq!(input.get(4), 1);
		assert_eq!(input.get(5), 2);
	}

	#[test]
	fn test_slot_descending ( )
	{
		let mut input : ArrayList<i32, 6> = ArrayList::new();

		let mut to_slot = -1;
		assert!(input.slot(to_slot, sort_descending));
		to_slot = 0;
		assert!(input.slot(to_slot, sort_descending));
		to_slot = 1;
		assert!(input.slot(to_slot, sort_descending));
		to_slot = 2;
		assert!(input.slot(to_slot, sort_descending));
		to_slot = 3;
		assert!(input.slot(to_slot, sort_descending));
		to_slot = 100;
		assert!(input.slot(to_slot, sort_descending));

		assert_eq!(input.size(), 6);
		// Full, 100, 3, 2, 1, 0, -1
		to_slot = -1;
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
}
