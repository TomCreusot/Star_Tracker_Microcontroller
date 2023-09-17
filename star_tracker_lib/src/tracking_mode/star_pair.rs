//! Implementation for KernelIterator
use core_include::*;

use super::StarPair;
use util::list::List;

// use crate::tracking_mode::database::SearchSave;



impl <T> StarPair <T> where T : PartialEq + Copy + core::fmt::Debug
{
	/// Finds the element which is unique to both pairs.
	/// If multiple similarities, returns the first found.
	/// # Arguments
	/// * `pair_a` - The first angle.
	/// * `pair_b` - The second angle.
	/// # Returns
	/// The similar item or None if available.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::tracking_mode::StarPair;
	/// 
	///	let pair_a = StarPair(3, 2);
	///	let pair_b = StarPair(3, 1);
	/// assert_eq!(StarPair::<usize>::find_same(pair_a, pair_b).unwrap(), 3);
	///
	///	let pair_a = StarPair(1, 2);
	/// let pair_b = StarPair(3, 4);
	/// assert_eq!(StarPair::<usize>::find_same(pair_a, pair_b), None);
	/// ```
	
	pub fn find_same ( pair_a: StarPair<T>, pair_b: StarPair<T> ) -> Option<T>
	{
		// Check pair_a position of pair_a
		if pair_a.0 == pair_b.0 || pair_a.0 == pair_b.1
		{
			return Option::Some(pair_a.0);
		}
		else if pair_a.1 == pair_b.0 || pair_a.1 == pair_b.1
		{
			return Option::Some(pair_a.1);
		}
		else
		{
			return Option::None;
		}
	}
	
	
	/// Checks if a star is in both pairs.
	/// # Arguments
	/// * `pair_a` - The first angle.
	/// * `pair_b` - The second angle.
	/// # Returns
	/// The similar item.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::tracking_mode::StarPair;
	/// 
	///	let pair_a = StarPair(3, 2);
	///	let pair_b = StarPair(3, 1);
	///	let pair_c = StarPair(0, 0);
	/// assert!(StarPair::<usize>::has_same(&pair_a, &pair_b));
	/// assert!(!StarPair::<usize>::has_same(&pair_a, &pair_c));
	/// ```

	pub fn has_same ( pair_a: &StarPair<T>, pair_b: &StarPair<T> ) -> bool
	{
		return StarPair::find_same(*pair_a, *pair_b).is_some();
	}
	
	
	/// Checks if the star pair contains the same elements.
	/// Even in a different order.
	/// # Arguments
	/// * `pair_a` - The first angle.
	/// * `pair_b` - The second angle.
	/// # Returns
	/// True if they contain the same stars.
	pub fn are_same ( pair_a: &StarPair<T>, pair_b: &StarPair<T> ) -> bool
	{
		{
			// Add unit test
		}
		return 
			(pair_a.0 == pair_b.0 && pair_a.1 == pair_b.1) ||
			(pair_a.0 == pair_b.1 && pair_a.1 == pair_b.0);
	}
	
	
	/// Checks if the element exists in the pair.
	/// # Arguments
	/// * `same` - The element to check exists.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::tracking_mode::StarPair;
	/// let pair_a = StarPair(0, 0);
	/// let pair_b = StarPair(0, 1);
	//
	/// assert!(pair_a.has(0));
	/// assert!(!pair_a.has(1));
	/// assert!(pair_b.has(0));
	/// assert!(pair_b.has(1));
	/// assert!(!pair_b.has(2));
	/// ```
	
	pub fn has ( &self, same: T ) -> bool
	{
		return self.0 == same.clone() || self.1 == same.clone();
	}
	
	
	
	
	
	/// Finds the element which is not `not`, if both 0 and 1 equal `not`, None is returned.
	/// # Arguments
	/// * `not` - The element to not return.
	/// # Returns
	/// The element in the struct that is not not or None or None if both are not not.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::tracking_mode::StarPair;
	/// let pair_a = StarPair(1, 1);
	/// let pair_b = StarPair(0, 1);
	///
	/// assert!(pair_a.find_not(1).is_none());
	/// assert_eq!(pair_a.find_not(0).expect("should be Some"), 1);
	/// assert_eq!(pair_b.find_not(0).expect("should be Some"), 1);
	/// assert_eq!(pair_b.find_not(1).expect("should be Some"), 0);
	/// ```
	pub fn find_not ( &self, not: T ) -> Option<T>
	{
		if self.0 != not
		{
			return Option::Some(self.0);
		}
		else if self.1 != not
		{
			return Option::Some(self.1);
		}
		return Option::None;
	}
}

impl StarPair<usize>
{
	/// Finds the index of a star pair containing both values if it exists.
	/// # Arguments
	/// * `find` - A set of 2 stars that must be found (any order).
	/// * `list` - The list to search through.
	/// # Returns
	/// None if there is not a match or the first valid result.
	///
	/// ```
	/// use star_tracker_lib::util::list::ArrayList;
	/// use star_tracker_lib::util::list::List;
	/// use star_tracker_lib::tracking_mode::StarPair;
	///
	///	let mut lst : ArrayList<StarPair<usize>, 10> = ArrayList::new();
	/// lst.push_back(StarPair(0, 1)).expect("");
	/// lst.push_back(StarPair(2, 3)).expect("");
	/// lst.push_back(StarPair(4, 5)).expect("");
	/// 
	/// let mut find = StarPair(0, 1);
	/// assert_eq!(StarPair::index_of(find, &lst), Some(0));
	/// find = StarPair(2, 3);
	/// assert_eq!(StarPair::index_of(find, &lst), Some(1));
	/// find = StarPair(4, 5);
	/// assert_eq!(StarPair::index_of(find, &lst), Some(2));
	/// ```
	pub fn index_of ( find: Self, list: &dyn List<Self> ) -> Option<usize>
	{
		for i in 0..list.size()
		{
			let pair = list.get(i);
			if (pair.0 == find.0 && pair.1 == find.1) || (pair.0 == find.1 && pair.1 == find.0)
			{
				return Option::Some(i);
			}
		}
		return Option::None;
	}
}


// impl SearchSave for StarPair<usize>
// {
// 	/// Call to return the constructed requested type.
// 	fn Save ( result: StarPair<usize>, region: BitField, error: Decimal ) -> Self
// 	{
// 		return result;
// 	}
// }



//###############################################################################################//
//###############################################################################################//
//
//										Unit Tests
//
//###############################################################################################//
//###############################################################################################//


#[cfg(test)]
mod test
{
	use util::list::List;
	
	use tracking_mode::StarPair;

	#[test]
	fn test_find_same_identical ( )
	{
		let pair_a = StarPair(1, 1);
		let pair_b = StarPair(2, 2);
		assert_eq!(StarPair::<usize>::find_same(pair_a, pair_b), None);
	}

	#[test]
	fn test_find_same_identical_wrong_order ( )
	{
		let pair_a = StarPair(1, 2);
		let pair_b = StarPair(2, 1);
		assert_eq!(StarPair::<usize>::find_same(pair_a, pair_b), Some(1));
	}



	#[test]
	fn test_find_same_different ( )
	{
		let pair_a = StarPair(1, 2);
		let pair_b = StarPair(3, 4);
		assert_eq!(StarPair::<usize>::find_same(pair_a, pair_b), None);
	}

	#[test]
	fn test_find_same_flipped ( )
	{
		let pair_a = StarPair(1, 2);
		let pair_b = StarPair(3, 1);
		assert_eq!(StarPair::<usize>::find_same(pair_a, pair_b).unwrap(), 1);
	}

	#[test]
	fn test_find_same_side ( )
	{
		let pair_a = StarPair(3, 2);
		let pair_b = StarPair(3, 1);
		assert_eq!(StarPair::<usize>::find_same(pair_a, pair_b).unwrap(), 3);
	}







	#[test]
	fn test_has_same_correct ( )
	{
		let pair_a = StarPair(1, 3);
		let pair_b = StarPair(1, 2);
		assert!(StarPair::<usize>::has_same(&pair_a, &pair_b));
	}

	#[test]
	fn test_has_same_invalid ( )
	{
		let pair_a = StarPair(2, 2);
		let pair_b = StarPair(1, 1);
		assert!(!StarPair::<usize>::has_same(&pair_a, &pair_b));
	}
	


	#[test]
	fn test_has_true ( )
	{
		let pair_a = StarPair(1, 3);
		let pair_b = StarPair(0, 0);
		assert!(pair_a.has(1));
		assert!(pair_a.has(3));
		assert!(pair_b.has(0));
	}

	#[test]
	fn test_has_false ( )
	{
		let pair_a = StarPair(1, 3);
		let pair_b = StarPair(0, 0);
		assert!(!pair_a.has(0));
		assert!(!pair_a.has(4));
		assert!(!pair_b.has(1));
		assert!(!pair_b.has(2));
	}
	
	
	
	
	#[test]
	fn test_find_not_none ( )
	{
		let pair = StarPair(1, 1);
		// let not_a = 2;
		assert!(pair.find_not(1).is_none());
	}
	
	#[test]
	fn test_find_not_valid ( )
	{
		let pair = StarPair(2, 1);
		assert_eq!(pair.find_not(1).expect("should be Some"), 2);
		assert_eq!(pair.find_not(2).expect("should be Some"), 1);	
	}




	//
	// fn are_same ( &self, Self ) -> bool
	#[test]
	fn test_are_same_equal ( )
	{
		let sp1 = StarPair(0, 1);
		let sp2 = StarPair(1, 0);
		assert!(StarPair::<usize>::are_same(&sp1, &sp1),"should be true.");
		assert!(StarPair::<usize>::are_same(&sp1, &sp2),"should be true.");
	}
	#[test]
	fn test_are_same_not_equal ( )
	{
		let sp1 = StarPair(0, 1);
		let sp2 = StarPair(0, 2);
		let sp3 = StarPair(1, 2);
		let sp4 = StarPair(2, 1);
		assert!(!StarPair::<usize>::are_same(&sp1, &sp2),"should not be true.");
		assert!(!StarPair::<usize>::are_same(&sp1, &sp3),"should not be true.");
		assert!(!StarPair::<usize>::are_same(&sp1, &sp4),"should not be true.");
	}



	//
	// fn index_of ( Self, &dyn List<Self> ) -> Optional<usize>
	// where T: usize
	#[test]
	fn test_index_of_found ( )
	{
		let mut lst : Vec<StarPair<usize>> = Vec::new();
		lst.push_back(StarPair(0, 1)).expect("");
		lst.push_back(StarPair(2, 3)).expect("");
		lst.push_back(StarPair(4, 5)).expect("");
		
		let mut find = StarPair(0, 1);
		assert_eq!(StarPair::index_of(find, &lst), Some(0));
		find = StarPair(2, 3);
		assert_eq!(StarPair::index_of(find, &lst), Some(1));
		find = StarPair(4, 5);
		assert_eq!(StarPair::index_of(find, &lst), Some(2));
	}

	//
	// fn index_of ( Self, &dyn List<Self> ) -> Optional<usize>
	// where T: usize
	#[test]
	fn test_index_of_fail ( )
	{
		let mut lst : Vec<StarPair<usize>> = Vec::new();
		lst.push_back(StarPair(0, 1)).expect("");
		lst.push_back(StarPair(2, 3)).expect("");
		lst.push_back(StarPair(4, 5)).expect("");
		
		let mut find = StarPair(1, 2);
		assert_eq!(StarPair::index_of(find, &lst), None);
		 find = StarPair(3, 4);
		assert_eq!(StarPair::index_of(find, &lst), None);
		find = StarPair(5, 6);
		assert_eq!(StarPair::index_of(find, &lst), None);
	}
}
