//! Implementation for KernelIterator
use super::StarPair;

impl <T> StarPair <T> where T : PartialEq + Copy + std::fmt::Debug
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
	/// use star_tracker::tracking_mode::StarPair;
	/// 
	///	let pair_a = StarPair(3, 2);
	///	let pair_b = StarPair(3, 1);
	/// assert_eq!(StarPair::<usize>::find_same(&pair_a, &pair_b).unwrap(), 3);
	///
	///	let pair_a = StarPair(1, 2);
	/// let pair_b = StarPair(3, 4);
	/// assert_eq!(StarPair::<usize>::find_same(&pair_a, &pair_b), None);
	/// ```
	
	pub fn find_same ( pair_a: &StarPair<T>, pair_b: &StarPair<T> ) -> Option<T>
	{
		// Check pair_a position of pair_a
		if pair_a.0 == pair_b.0 || pair_a.0 == pair_b.1
		{
			return Some(pair_a.0);
		}
		else if pair_a.1 == pair_b.0 || pair_a.1 == pair_b.1
		{
			return Some(pair_a.1);
		}
		else
		{
			return None;
		}
	}
	
	
	/// Checks if a star is in both pairs.
	/// # Arguments
	/// * `pair_a` - The first angle.
	/// * `pair_b` - The second angle.
	/// # Returns
	/// The simmilar item.
	///
	/// # Example
	/// ```
	/// use star_tracker::tracking_mode::StarPair;
	/// 
	///	let pair_a = StarPair(3, 2);
	///	let pair_b = StarPair(3, 1);
	///	let pair_c = StarPair(0, 0);
	/// assert!(StarPair::<usize>::has_same(&pair_a, &pair_b));
	/// assert!(!StarPair::<usize>::has_same(&pair_a, &pair_c));
	/// ```

	pub fn has_same ( pair_a: &StarPair<T>, pair_b: &StarPair<T> ) -> bool
	{
		return StarPair::find_same(&pair_a, &pair_b).is_some();
	}
	
	
	/// Checks if the element exists in the pair.
	/// # Arguments
	/// * `same` - The element to check exists.
	///
	/// # Example
	/// ```
	/// use star_tracker::tracking_mode::StarPair;
	/// let pair_a = StarPair(0, 0);
	/// let pair_b = StarPair(0, 1);
	//
	/// assert!(pair_a.has(&0));
	/// assert!(!pair_a.has(&1));
	/// assert!(pair_b.has(&0));
	/// assert!(pair_b.has(&1));
	/// assert!(!pair_b.has(&2));
	/// ```
	
	pub fn has ( &self, same: &T ) -> bool
	{
		return self.0 == same.clone() || self.1 == same.clone();
	}
	
	
	
	
	
	/// Finds the element which is not `not`, if `not` is not in the star pair, None is returned.
	/// # Arguments
	/// * `not` - The element to not return.
	/// # Returns
	/// The element in the struct that is not not or None or None if both are not not.
	///
	/// # Example
	/// ```
	/// use star_tracker::tracking_mode::StarPair;
	/// let pair_a = StarPair(1, 1);
	/// let pair_b = StarPair(0, 1);
	///
	/// assert!(pair_a.find_not(&1).is_none());
	/// assert_eq!(pair_a.find_not(&0).expect("should be Some"), 1);
	/// assert_eq!(pair_b.find_not(&0).expect("should be Some"), 1);
	/// assert_eq!(pair_b.find_not(&1).expect("should be Some"), 0);
	/// ```
	pub fn find_not ( &self, not: &T ) -> Option<T>
	{
		if self.0 != not.clone()
		{
			return Some(self.0);
		}
		else if self.1 != not.clone()
		{
			return Some(self.1);
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
mod test
{
	use tracking_mode::StarPair;

	#[test]
	fn test_find_same_identical ( )
	{
		let pair_a = StarPair(1, 1);
		let pair_b = StarPair(2, 2);
		assert_eq!(StarPair::<usize>::find_same(&pair_a, &pair_b), None);
	}

	#[test]
	fn test_find_same_identical_wrong_order ( )
	{
		let pair_a = StarPair(1, 2);
		let pair_b = StarPair(2, 1);
		assert_eq!(StarPair::<usize>::find_same(&pair_a, &pair_b), Some(1));
	}



	#[test]
	fn test_find_same_different ( )
	{
		let pair_a = StarPair(1, 2);
		let pair_b = StarPair(3, 4);
		assert_eq!(StarPair::<usize>::find_same(&pair_a, &pair_b), None);
	}

	#[test]
	fn test_find_same_flipped ( )
	{
		let pair_a = StarPair(1, 2);
		let pair_b = StarPair(3, 1);
		assert_eq!(StarPair::<usize>::find_same(&pair_a, &pair_b).unwrap(), 1);
	}

	#[test]
	fn test_find_same_side ( )
	{
		let pair_a = StarPair(3, 2);
		let pair_b = StarPair(3, 1);
		assert_eq!(StarPair::<usize>::find_same(&pair_a, &pair_b).unwrap(), 3);
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
		assert!(pair_a.has(&1));
		assert!(pair_a.has(&3));
		assert!(pair_b.has(&0));
	}

	#[test]
	fn test_has_false ( )
	{
		let pair_a = StarPair(1, 3);
		let pair_b = StarPair(0, 0);
		assert!(!pair_a.has(&0));
		assert!(!pair_a.has(&4));
		assert!(!pair_b.has(&1));
		assert!(!pair_b.has(&2));
	}
	
	
	
	
	#[test]
	fn test_find_not_none ( )
	{
		let pair = StarPair(1, 1);
		// let not_a = 2;
		assert!(pair.find_not(&1).is_none());
	}
	
	#[test]
	fn test_find_not_valid ( )
	{
		let pair = StarPair(2, 1);
		assert_eq!(pair.find_not(&1).expect("should be Some"), 2);
		assert_eq!(pair.find_not(&2).expect("should be Some"), 1);	
	}

}
