//! Designed to share similarities between Vec, ArrayList and Arrays.  
//! This is required for the database.  
//! When constructing the database, it is ideal to store the elements as a Vec.  
//! When storing the database, it should be stored as an array.  
//! # Example
//! ```
//! use star_tracker::util::linear_lookup::LinearLookup;
//! use star_tracker::util::list::ArrayList;
//!
//! struct AnnoyingToStore<'a> ( pub &'a dyn LinearLookup<u32> );
//!
//! let lst: ArrayList<u32, 3> = ArrayList::from_array(&[1,2,3]).expect("");
//! let array      = AnnoyingToStore(&[1,2,3]);                    // Stored as an array.
//! let vec        = AnnoyingToStore(&vec!(1,2,3));                // Stored as a list.
//! let array_list = AnnoyingToStore(&lst);                        // Stored as an ArrayList.
//!
//! // Methods provided are: get(usize) -> T, set(usize, T) -> Error<()>, size() -> usize
//! assert_eq!(array.0.get(0), 1);
//! assert_eq!(array.0.size(), 3);
//! ```
use crate::util::list::ArrayList;
use crate::util::list::List;
use crate::util::err::Errors;
use crate::util::err::Error;

/// Designed to share similarities between Vec, ArrayList and Arrays.  
/// This is required for the database.  
/// When constructing the database, it is ideal to store the elements as a Vec.  
/// When storing the database, it should be stored as an array.  
pub trait LinearLookup <T>
{
	/// Returns the size of the list (array.len(), vec.len(), array_list.size()).
	fn size  ( &self ) -> usize;
	
	/// Returns the value at the index (array\[index\], vec\[index\], array_list.set(index)).
	/// # Arguments
	/// * `index` - The index to get the value at.
	fn get  ( &self, index: usize ) -> T;
	
	/// Sets the value at the index.  
	/// array\[index\] = val, vec\[index\] = val, array_list.set(index, val).  
	/// # Arguments
	/// * `index` - The index to get the value at.
	/// * `val`   - The value to assign.
	/// # Returns
	/// array     : Ok.  
	/// vec       : Ok if within size, otherwise Errors::OutOfBounds.  
	/// array_list: Ok if within size, otherwise Errors::OutOfBounds.  
	fn set  ( &mut self, index: usize, val: T ) -> Error<()>;
}


impl<T, const N: usize> LinearLookup<T> for ArrayList<T, N> where T: Clone
{
	fn size ( &self ) -> usize
	{	return (self as &dyn List<T>).size()			}

	fn get ( &self, index: usize ) -> T
	{ return (self as &dyn List<T>).get(index);		}

	fn set ( &mut self, index: usize, val: T )	-> Error<()>
	{ return (self as &mut dyn List<T>).set(index, val);	}

}



impl <T> LinearLookup<T> for Vec<T> where T: Clone
{
	fn size ( &self ) -> usize
	{	return (self as &dyn List<T>).size()			}

	fn get ( &self, index: usize ) -> T
	{ return (self as &dyn List<T>).get(index);		}

	fn set ( &mut self, index: usize, val: T )	-> Error<()>
	{ return (self as &mut dyn List<T>).set(index, val);	}
}


impl <T, const N: usize> LinearLookup<T> for [T; N] where T: Clone, T: Copy
{
	fn size ( &self ) -> usize
	{	return (self as &[T;N]).len()		}

	fn get ( &self, index: usize ) -> T
	{ return self[index];		}

	fn set ( &mut self, index: usize, val: T )	-> Error<()>
	{ 
		if self.len() < index
		{
			return Err(Errors::OutOfBounds);
		}
		self[index] = val; 
		return Ok(());
	}
}



//###############################################################################################//
//###############################################################################################//
//
//										Unit Tests
//	This is abit unnessisary...
//###############################################################################################//
//###############################################################################################//

#[cfg(test)]
#[allow(unused_must_use)]
mod test
{
	use crate::util::linear_lookup::LinearLookup;
	use crate::util::list::ArrayList;
	use crate::util::err::Error;
	use crate::util::err::Errors;
	
	#[test]
	// These methods are really basic.
	// Im just going to test everything together.
	pub fn test_array_list ( ) -> Error<()>
	{
		let mut list: ArrayList<u32, 3> = ArrayList::from_array(&[1,2,3])?;
		assert_eq!(list.size(), 3);
		assert_eq!(list.get(0), 1);
		assert_eq!(list.get(1), 2);
		assert_eq!(list.get(2), 3);
		
		assert_eq!(list.set(0, 3), Ok(()));
		assert_eq!(list.get(0), 3);
		
		assert_eq!(list.set(4, 3), Err(Errors::OutOfBounds));
		return Ok(());
	}
	
	#[test]
	// These methods are really basic.
	// Im just going to test everything together.
	pub fn test_vec ( ) -> Error<()>
	{
		let mut list: Vec<u32> = vec!(1,2,3);
		assert_eq!(list.size(), 3);
		assert_eq!(list.get(0), 1);
		assert_eq!(list.get(1), 2);
		assert_eq!(list.get(2), 3);
		
		assert_eq!(list.set(0, 3), Ok(()));
		assert_eq!(list.get(0), 3);
		
		assert_eq!(list.set(4, 3), Err(Errors::OutOfBounds));
		return Ok(());
	}
	
	#[test]
	// These methods are really basic.
	// Im just going to test everything together.
	pub fn test_array ( ) -> Error<()>
	{
		let mut list = [1,2,3];
		assert_eq!(list.size(), 3);
		assert_eq!(list.get(0), 1);
		assert_eq!(list.get(1), 2);
		assert_eq!(list.get(2), 3);
		
		assert_eq!(list.set(0, 3), Ok(()));
		assert_eq!(list.get(0), 3);
		
		assert_eq!(list.set(4, 3), Err(Errors::OutOfBounds));
		return Ok(());
	}
	
}