//! Implementation of [Linear Lookup](crate::util::linear_lookup::LinearLookup) for Vec.  
//! Only should compile with the nix flag.
//! * `nix`  - Unix operating system in use.
use crate::util::linear_lookup::LinearLookup;
use crate::util::list::List;
use crate::util::err::Error;

impl <T> LinearLookup<T> for Vec<T> where T: Clone
{
	fn size ( &self ) -> usize
	{ return (self as &dyn List<T>).size()        }

	fn get ( &self, index: usize ) -> T
	{ return (self as &dyn List<T>).get(index);   }

	fn set ( &mut self, index: usize, val: T )	-> Error<()>
	{ return (self as &mut dyn List<T>).set(index, val); }
}

impl<T: Sized> From<&dyn LinearLookup<T>> for Vec<T> {
    fn from(val: &dyn LinearLookup<T>) -> Vec<T> {
        val.into()
    }
}


//###############################################################################################//
//###############################################################################################//
//
//										Unit Tests
//	This is a bit necessary...
//###############################################################################################//
//###############################################################################################//

#[cfg(test)]
#[allow(unused_must_use)]
mod test
{
	use crate::util::linear_lookup::LinearLookup;
	use crate::util::err::Error;
	use crate::util::err::Errors;
	
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
}