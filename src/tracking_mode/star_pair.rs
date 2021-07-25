//! Implementation for KernelIterator
use super::StarPair;

impl StarPair <usize>
{

	/// Finds the element which is unique to both pairs.
	/// # Arguments
	/// * `pair_a` - The first angle.
	/// * `pair_b` - The second angle.
	/// # Returns
	/// The index to the similar item in pair_a if there is one.
	///
	/// # Example
	/// ```
	/// use star_tracker::tracking_mode::StarPair;
	/// use star_tracker::util::units::Equatorial;
	/// use star_tracker::util::units::Radians;
	/// 
	///	let pair_a = StarPair(3, 2);
	///	let pair_b = StarPair(3, 1);
	/// assert_eq!(pair_a.similar_both(&pair_b).unwrap(), 3);
	///
	///	let pair_a = StarPair(1, 2);
	/// let pair_b = StarPair(3, 4);
	/// assert_eq!(pair_a.similar_both(&pair_b), None);
	///
	///	let pair_a = StarPair(1, 2);
	/// let pair_b = StarPair(2, 1);
	/// assert_eq!(pair_a.similar_both(&pair_b), None);
	///
	/// let pair_a = StarPair(1, 1);
	/// let pair_b = StarPair(2, 2);
	/// assert_eq!(pair_a.similar_both(&pair_b), None);
	/// ```
	
	pub fn similar_both ( &self, other: &StarPair<usize> ) -> Option<usize>
	{
		// Check first position of pair_a
		if self.0 == other.0 || self.0 == other.1
		{
			if self.1 == other.0 || self.1 == other.1
			{
				// Identical.
				return None;
			}
			return Some(self.0);
		}
		else if self.1 == other.0 || self.1 == other.1
		{
			return Some(self.1);
		}
		else
		{
			return None;
		}
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
	fn test_similar_both_identical ( )
	{
		let pair_a = StarPair(1, 1);
		let pair_b = StarPair(2, 2);
		assert_eq!(pair_a.similar_both(&pair_b), None);
	}

	#[test]
	fn test_similar_both_identical_wrong_order ( )
	{
		let pair_a = StarPair(1, 2);
		let pair_b = StarPair(2, 1);
		assert_eq!(pair_a.similar_both(&pair_b), None);
	}




	#[test]
	fn test_similar_both_different ( )
	{
		let pair_a = StarPair(1, 2);
		let pair_b = StarPair(3, 4);
		assert_eq!(pair_a.similar_both(&pair_b), None);
	}

	#[test]
	fn test_flipped ( )
	{
		let pair_a = StarPair(1, 2);
		let pair_b = StarPair(3, 1);
		assert_eq!(pair_a.similar_both(&pair_b).unwrap(), 1);
	}

	#[test]
	fn test_same_side ( )
	{
		let pair_a = StarPair(3, 2);
		let pair_b = StarPair(3, 1);
		assert_eq!(pair_a.similar_both(&pair_b).unwrap(), 3);
	}

}
