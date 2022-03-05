use super::List;
use util::err::{Errors, Error};



//###############################################################################################//
//								---	ListIterator Implementation ---
//###############################################################################################//
/*impl<'a, T> IntoIterator for &'a dyn List<T> where T: Clone {
    type Item = T;
    type IntoIter = ListIterator<'a, T>;

}*/



impl <T> List <T> for Vec <T> where T: Clone
{	
    /// Finds the max number of elements that can be stored in the list.
    fn capacity ( &self ) -> usize
    {
        return usize::max_value();//Vec::capacity(&self);//(isize::max_value() - 1) as usize;
    }

    /// Finds how many elements are in the list.
    /// # Returns
    /// The size.
    fn size ( &self ) -> usize
    {
        return self.len() as usize;
    }

    /// Checks if the List is at maximum capacity.
    /// # Returns
    /// True if full.
    fn is_full ( &self ) -> bool
    {
        return false;
    }

    /// Checks if the List is empty.
    /// # Returns
    /// True if empty.
    ///
    /// # Example
    /// lst : ArrayList<UInt, 2> = ArrayList::new();
    fn is_empty ( &self ) -> bool
    {
        return self.len() == 0;
    }

    /// Gets the element at the specified index.
    /// # Arguments
    ///	* 'index' - The index of the element to receive.
    /// # Returns
    /// The value at the index.
    fn get ( &self, index : usize ) -> T
    {
        assert!(index < self.len(), "Out of bounds");
		return self[index].clone();
    }

    /// Sets the element at the specified index.
    /// # Arguments
    ///	* 'index' - The index of the element to receive.
    /// * 'value' - The value to assign.
    fn set ( &mut self, index : usize, value : T ) -> Error<()>
    {
		if index < self.size()
		{
			self[index] = value;
			return Ok(());
		}
		return Err(Errors::OutOfBounds);
    }

    /// Adds an element to the end of the list.
    /// # Arguments
    /// * 'value' - the value to add to the end.
    fn push_back ( &mut self, value : T ) -> Error<()>
    {
		if self.size() < (self as &dyn List<T>).capacity()
		{
			self.push(value);
			return Ok(());
		}
		return Err(Errors::InvalidSize);
    }


    /// Removes an element from the end of the list.
    /// # Returns
    /// The value removed.
    fn pop_back ( &mut self ) -> T
    {
        assert!(!self.is_empty(), "List is empty");
        return self.pop().unwrap();
    }
	
	
	/// Sets the counter to 0 so all elements will be override and the list is essentialy cleared.
	/// # Example
	/// ``` 
	/// use star_tracker::util::list::List;
	/// let mut lst : Vec<u32> = Vec::new();
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
		self.clear();
	}

    /// Sorts the list
    /// # Arguments
    /// * 'in_order' - A function which returns TRUE if it is in order.
    fn sort ( &mut self, in_order: fn (& T, & T) -> bool )
    {
        for ii in 0..self.size()
        {
            let mut jj : usize = ii;

            let mut temp : T = self.get(jj).clone();
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
    /// * `in_order` - The ordering method.
    ///
    /// # Returns
    /// True if inserted, false if there is no space and it will trail the last element.
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
	use crate::util::list::{List};

	//
	// capacity ( ) -> const usize
	//
	// #[test]
	// fn test_capacity ( )
	// {
	// 	let size = (isize::MAX - 1) as usize;
	// 	let lst : Vec<u32> = Vec::with_capacity(1);
	// //	assert_eq!(lst.capacity(), size);
	// }



//
// size ( ) -> usize
//
	#[test]
	fn test_size ( )
	{
		let mut lst : Vec<i32> = Vec::new();
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
	fn test_is_full_size ( )
	{
		let lst : Vec<i32> = Vec::new();
		assert!(!lst.is_full());
	}



//
// is_empty ( ) -> bool
//
	#[test]
	fn test_is_empty_size_0 ( )
	{
        let mut lst : Vec<i32> = Vec::new();
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
		let mut lst : Vec<u32> = Vec::new();
		println!("{:?}", lst.push_back(0));
		lst.push_back(1);
		let mut first = lst.get(0);
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
		let lst : Vec<u32> = Vec::new();
		lst.get(1);
	}



//
// set ( T )
//
	#[test]
	fn test_set_valid ( )
	{
		let mut lst : Vec<u32> = Vec::new();
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
		let mut lst : Vec<u32> = Vec::new();
		assert!(lst.set(0, 0).is_err());
	}


//
//  push_back ( T )
//
	#[test]
	fn test_push_back_valid ( )
	{
		let mut lst : Vec<u32> = Vec::new();
		lst.push_back(0);
		lst.push_back(1);
		assert_eq!(lst.pop_back(), 1);
		assert_eq!(lst.pop_back(), 0);
	}




//
//  pop_back ( )
//
	#[test]
	fn test_pop_back_valid ( )
	{
		let mut lst : Vec<u32> = Vec::new();
		lst.push_back(0);
		lst.push_back(1);
		assert_eq!(lst.pop_back(), 1);
		assert_eq!(lst.pop_back(), 0);
	}

	#[test]
	#[should_panic = "List is empty"]
	fn test_pop_back_invalid ( )
	{
		let mut lst : Vec<u32> = Vec::new();
		lst.pop_back();
	}


	//
	// clear ( &mut self )
	//
	
	#[test]
	// Clear should set list to 0 and override any values when pushback occures.
	fn test_clear ( )
	{
		let mut lst : Vec<u32> = Vec::new();
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
	fn test_sort_ascending ( )
	{
		let mut lst : Vec<i32> = Vec::new();
		lst.push_back(2);
		lst.push_back(1);
		lst.push_back(0);
		lst.sort(sort_ascending);
		assert_eq!(lst.pop_back(), 2);
		assert_eq!(lst.pop_back(), 1);
		assert_eq!(lst.pop_back(), 0);
	}

	#[test]
	fn test_sort_descending ( )
	{
		let mut lst : Vec<i32> = Vec::new();
		lst.push_back(0);
		lst.push_back(1);
		lst.push_back(2);
		lst.sort(sort_descending);
		assert_eq!(lst.pop_back(), 0);
		assert_eq!(lst.pop_back(), 1);
		assert_eq!(lst.pop_back(), 2);
	}

	#[test]
	fn test_sort_empty ( )
	{
		let mut lst : Vec<i32> = Vec::new();
		lst.sort(sort_ascending);
	}




//
// slot ( to_slot : T, fn ( &mut T, &mut T ) -> bool ) -> bool
//
	#[test]
	fn test_slot_ascending ( )
	{
		let mut input : Vec<i32> = Vec::new();

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
		assert!(input.slot(to_slot, sort_ascending));//-1, 0, 1, 2, 3, 4, 5
		to_slot = -2;
		assert!(input.slot(to_slot, sort_ascending));//-2, -1, 0, 1, 2, 3, 4, 5
		to_slot = 1;
		assert!(input.slot(to_slot, sort_ascending));//-2, -1, 0, 1, 1, 2, 3, 4, 5
		to_slot = 10;
		assert!(input.slot(to_slot, sort_ascending));//-2, -1, 0, 1, 1, 2, 3, 4, 5, 10

		assert_eq!(input.get(0), -2);
		assert_eq!(input.get(1), -1);
		assert_eq!(input.get(2), 0);
		assert_eq!(input.get(3), 1);
		assert_eq!(input.get(4), 1);
		assert_eq!(input.get(5), 2);
		assert_eq!(input.get(6), 3);
		assert_eq!(input.get(7), 4);
		assert_eq!(input.get(8), 5);
		assert_eq!(input.get(9), 10);
	}

	#[test]
	fn test_slot_descending ( )
	{
		let mut input : Vec<i32> = Vec::new();

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
		assert!(input.slot(to_slot, sort_descending)); // 100, 3, 2, 1, 0, -1
		to_slot = 5;
		assert!(input.slot(to_slot, sort_descending)); // 100, 5, 3, 2, 1, 0, -1
		to_slot = 1;
		assert!(input.slot(to_slot, sort_descending)); // 100, 5, 3, 2, 1, 1, 0, -1
		to_slot = 101;
		assert!(input.slot(to_slot, sort_descending)); // 101, 100, 5, 3, 2, 1, 1, 0, -1
		to_slot = -100;
		assert!(input.slot(to_slot, sort_descending)); // 101, 100, 5, 3, 2, 1, 1, 0, -1

		assert_eq!(input.get(0), 101);
		assert_eq!(input.get(1), 100);
		assert_eq!(input.get(2), 5);
		assert_eq!(input.get(3), 3);
		assert_eq!(input.get(4), 2);
		assert_eq!(input.get(5), 1);
		assert_eq!(input.get(6), 1);
		assert_eq!(input.get(7), 0);
		assert_eq!(input.get(8), -1);
	}
}
