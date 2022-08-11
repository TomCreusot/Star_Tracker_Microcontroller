//! Designed to share similarities between Vec, ArrayList and [;]
//! Mainly for passing Vec into [;]

use util::list::ArrayList;
use util::list::List;
use util::err::Error;

pub trait LinearLookup <T>
{
	fn size  ( &self ) -> usize;
	fn get  ( &self, index: usize ) -> T;
	fn set  ( &mut self, index: usize, val: T ) -> Error<()>;
}


impl<T, const N : usize> LinearLookup<T> for ArrayList<T, N> where T: Clone
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
	{ self[index] = val; return Ok(());	}
}