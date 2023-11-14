//! An array list using a CArray instead of a rust array.
use star_tracker_lib::core_include::*;

use star_tracker_lib::util::LinearLookup::CArray;
use star_tracker_lib::util::list::List;
use star_tracker_lib::util::err::Error;
use star_tracker_lib::util::err::Errors;

//###############################################################################################//
//									---	CList ---
//###############################################################################################//

pub struct CList<T>
{
	array: CArray<T>,
	end: usize,
}

impl <T> CList<T>
{
	pub fn new ( array: CArray<T> ) -> Self
	{
		return Self
		{
			array: array, end: 0
		};
	}
}

//###############################################################################################//
//								---	List Implementation ---
//###############################################################################################//
impl<T> List<T> for CList<T> where T: Clone
{

	/// Finds the max number of elements that can be stored in the list.
	fn capacity ( &self ) -> usize
	{
		return self.array.size;
	}


	/// Finds how many elements are in the list.
	fn size ( &self ) -> usize
	{
		return self.end;
	}


	/// Checks if the ArrayList is at maximum capacity.  
	/// Returns true when full.  
	fn is_full ( &self ) -> bool
	{
		return self.end == self.capacity();
	}


	/// Checks if the ArrayList contains no elements.  
	/// Returns true if the array size is 0.  
	fn is_empty ( &self ) -> bool
	{
		return self.end == 0;
	}

	/// Returns the element at the specified index.  
	/// Equivalent to `array[index];`  
	/// There are no safety checks as it is expected you know how to index an array.  
	fn get ( &self, index: usize ) -> T
	{
		return self.array.get(index).clone();
	}

	/// Sets the element at the specified index.  
	/// Equivalent to `array[index] = value;`  
	fn set ( &mut self, index: usize, value: T ) -> Error<()>
	{
		if self.size() <= index
		{
			return Err(Errors::OutOfBounds);
		}
		self.array.get(index) = value;
		return Ok(());
	}


	/// Adds an element to the end of the list with the provided value.  
	/// # Returns
	/// If is_full() Errors::InvalidSize  
	/// else         Ok(())  
	fn push_back ( &mut self, value: T ) -> Error<()>
	{
		if self.capacity() <= self.size()
		{
			return Err(Errors::InvalidSize);
		}
		self.array.set(self.end, value.clone());
		self.end += 1;
		return Ok(());
	}

	/// Removes an element from the end of the list.  
	/// # Returns
	/// If is_empty() The value removed.  
	/// else          Errors::InvalidSize.  
	fn pop_back ( &mut self ) -> Error<T>
	{
		if self.is_empty()
		{
			return Err(Errors::InvalidSize);
		}
		self.end -= 1;
		return Ok(self.array.get(self.end).clone());
	}


	/// Sets the counter to 0 so all elements will be override and the list is essentiality cleared.
	fn clear ( &mut self )
	{
		self.end = 0;
	}
}