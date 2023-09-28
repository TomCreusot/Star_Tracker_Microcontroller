//! Implementation of [ListIterator](crate::util::list::list_iterator).  
//!
//! Due to limitations with lifetimes and types, the default ToIterator trait cannot be used.  
//! Instead the iterator statically creates a new iterator from an input list.  
use core::iter::Iterator;
use crate::core_include::*;

use crate::util::list::ListIterator;
use crate::util::list::List;



impl<'a, T: Clone> ListIterator<'a, T>
{
	/// Input the list to be iterated from.
	pub fn new ( list: &'a dyn List<T> ) -> ListIterator<'a, T>
	{
		return ListIterator{ list: list, index: 0 };
	}
}

impl <'a, T: Clone> Iterator for ListIterator<'a, T> {

	type Item = T;

	/// Moves to the next element.
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
	/// 	println!("{}", e);
	/// }
	/// ```
	fn next ( &mut self ) -> Option<T>
	{
		if self.index < self.list.size()
		{
			let val = Option::Some(self.list.get(self.index));
			self.index += 1;
			return val;
		}
		return Option::None;
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
	use crate::util::list::ListIterator;

	#[test]
	// All functionallity of list_iterator for arraylist will be tested together.
	fn test_iter_array_list ( )
	{
		let mut list: ArrayList<u32, 5> = ArrayList::new();
		list.push_back(0);
		list.push_back(1);
		list.push_back(2);
		list.push_back(3);
		list.push_back(4);

		let mut iter: ListIterator<u32> = ListIterator::new(&list);
		assert_eq!(iter.next(), Some(0));
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.next(), Some(4));
		assert_eq!(iter.next(), None);
		assert_eq!(iter.next(), None);
	}
}
