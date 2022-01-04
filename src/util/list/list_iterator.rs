//! Due to limitations with lifeimes and types, the default ToIterator trait cannot be used.
//! Instead the iterator staticaly creates a new iterator from an input list.
use super::{List, ListIterator};
use std::iter::Iterator;



impl<'a, T> ListIterator<'a, T> 
{
	pub fn new ( list : &'a dyn List<T> ) -> ListIterator<'a, T>
	{
		return ListIterator{ list: list, index: 0 };
	}

	// /// Removes an element at the specified location and moves the cursor left.
	// fn remove ( )
	// {
	// 	if ( self.index < self.list.size() )
	// 	{
	// 		list.pop(index);
	// 		if ( 1 < self.index )
	// 		{
	// 			self.index -= 1;
	// 		}
	// 	}
	// }

}

impl <'a, T> Iterator for ListIterator<'a, T> {
    
	type Item = T;
	
    fn next ( &mut self ) -> Option<T> 
	{
        if self.index < self.list.size()
		{
			let val = Some(self.list.get(self.index));
			self.index += 1;
			return val;
		}
		return None;
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
	use crate::util::list::{List, ArrayList, ListIterator};
	
	#[test]
	fn test_iter_array_list ( )
	{
		let mut list : ArrayList<u32, 5> = ArrayList::new();
		list.push_back(0);
		list.push_back(1);
		list.push_back(2);
		list.push_back(3);
		list.push_back(4);
		
		let mut iter : ListIterator<u32> = ListIterator::new(&list);
		assert_eq!(iter.next(), Some(0));
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.next(), Some(4));
		assert_eq!(iter.next(), None);
		assert_eq!(iter.next(), None);
	}


	#[test]
	fn test_iter_vec ( )
	{
		let mut list : Vec<u32> = Vec::new();
		list.push_back(0);
		list.push_back(1);
		list.push_back(2);
		list.push_back(3);
		list.push_back(4);
		
		let mut iter : ListIterator<u32> = ListIterator::new(&list);
		assert_eq!(iter.next(), Some(0));
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.next(), Some(4));
		assert_eq!(iter.next(), None);
		assert_eq!(iter.next(), None);
	}
	
	
	// #[test]
	// fn test_remove_array_list ( )
	// {
	// 	let mut list : ArrayList<u32, 5> = ArrayList::new();
	// 	list.push_back(0);
	// 	list.push_back(1);
	// 	list.push_back(2);
	// 	list.push_back(3);
	// 	list.push_back(4);
	// 
	// 	let mut iter : ListIterator<u32> = ListIterator::new(&list);
	// 	iter.remove();
	// 	assert_eq!(iter.next(), Some(1));
	// 	assert_eq!(iter.next(), Some(2));
	// 	iter.remove();
	// 	assert_eq!(iter.next(), Some(4));
	// 	assert_eq!(iter.next(), None);
	// 	iter.remove(); // Should not crash.
	// 	assert_eq!(iter.next(), None);
	// }
	
}