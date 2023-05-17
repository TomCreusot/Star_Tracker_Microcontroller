//! Implementation for KernelIterator
use super::KernelIterator;

impl KernelIterator
{
	/// Initializes the iterator.
	/// # Arguments
	/// * `size` - The size of the array.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::tracking_mode::KernelIterator;
	/// let mut  iter = KernelIterator::new(0);
	/// assert!(iter.i == iter.j && iter.i == iter.k);
	/// ```
	pub fn new ( size : usize ) -> KernelIterator
	{
		return KernelIterator{size: size, dj: 1, dk: 1, di: 0, i: 0, j: 0, k: 0};
	}



	/// Steps the iterator, STEP TO START ITERATING.
	/// # Returns
	/// True if there is more steps in the sequence.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::tracking_mode::KernelIterator;
	///	let mut iter = KernelIterator::new(4);
	/// assert!(iter.step());
	/// assert_eq!(iter.i, 0);
	/// assert_eq!(iter.j, 1);
	/// assert_eq!(iter.k, 2);
	/// assert!(iter.step());
	/// assert_eq!(iter.i, 1);
	/// assert_eq!(iter.j, 2);
	/// assert_eq!(iter.k, 3);
	/// assert!(iter.step());
	/// assert_eq!(iter.i, 0);
	/// assert_eq!(iter.j, 1);
	/// assert_eq!(iter.k, 3);
	/// assert!(iter.step());
	/// assert_eq!(iter.i, 0);
	/// assert_eq!(iter.j, 2);
	/// assert_eq!(iter.k, 3);
	/// assert!(!iter.step());
	/// ```
	pub fn step ( &mut self ) -> bool
	{
		let n = self.size;
		if n == 0 || n == 1 || n == 2
		{
			return false;
		}
		// Yep, this looks far better as a for loop.
		// for dj in 1..n - 2
		// 		for dk in 1..n - 1 - dj
		// 			for dk in 1..n - 1 - dj
		if self.di < n - self.dj - self.dk		// iterate dI
		{
			self.di += 1;
		}
		else
		{
			self.di = 1;
			if self.dk < n - self.dj - 1		// Iterate dK
			{
				self.dk += 1;
			}
			else
			{
				self.dk = 1;
				if self.dj < n - 2				// Iterate dJ
				{
					self.dj += 1;
				}
				else
				{
					return false;
				}
			}
		}
		self.update_ijk();
		return true;
	}


	/// Updates the i, j and k values from di, dj and dk.
	///
	/// # Example
	/// ```
	/// // There is no reason why you should use this.
	/// ```
	fn update_ijk ( &mut self )
	{
		self.i = self.di - 1;
		self.j = self.di + self.dj;
		self.k = self.j + self.dk;

		self.j -= 1;
		self.k -= 1;
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
	use tracking_mode::KernelIterator;

	#[test]
	fn test_new ( )
	{
		let iter = KernelIterator::new(0);
		assert_eq!(iter.size, 0);
		assert_eq!(iter.dj, 1);
		assert_eq!(iter.dk, 1);
		assert_eq!(iter.di, 0);
		assert_eq!(iter.i, 0);
		assert_eq!(iter.j, 0);
		assert_eq!(iter.k, 0);
	}



//
//  step ( )
//
	#[test]
	fn test_step_insufficient_elements ( )
	{
		let mut iter = KernelIterator::new(0);
		assert!(!iter.step());

		iter = KernelIterator::new(1);
		assert!(!iter.step());

		iter = KernelIterator::new(2);
		assert!(!iter.step());
	}

	#[test]
	fn test_step_3_sequence ( )
	{
		let mut iter = KernelIterator::new(3);
		assert!(iter.step());
		assert_eq!(iter.i, 0);
		assert_eq!(iter.j, 1);
		assert_eq!(iter.k, 2);
		assert!(!iter.step());
	}

	#[test]
	fn test_step_4_sequence ( )
	{
		let mut iter = KernelIterator::new(4);
		assert!(iter.step());
		assert_eq!(iter.i, 0);
		assert_eq!(iter.j, 1);
		assert_eq!(iter.k, 2);
		assert!(iter.step());
		assert_eq!(iter.i, 1);
		assert_eq!(iter.j, 2);
		assert_eq!(iter.k, 3);
		assert!(iter.step());
		assert_eq!(iter.i, 0);
		assert_eq!(iter.j, 1);
		assert_eq!(iter.k, 3);
		assert!(iter.step());
		assert_eq!(iter.i, 0);
		assert_eq!(iter.j, 2);
		assert_eq!(iter.k, 3);
		assert!(!iter.step());
	}

	#[test]
	fn test_step_5_sequence ( )
	{
		let mut iter = KernelIterator::new(5);
		assert!(iter.step());
		assert_eq!(iter.i, 0);
		assert_eq!(iter.j, 1);
		assert_eq!(iter.k, 2);
		assert!(iter.step());
		assert_eq!(iter.i, 1);
		assert_eq!(iter.j, 2);
		assert_eq!(iter.k, 3);
		assert!(iter.step());
		assert_eq!(iter.i, 2);
		assert_eq!(iter.j, 3);
		assert_eq!(iter.k, 4);
		assert!(iter.step());
		assert_eq!(iter.i, 0);
		assert_eq!(iter.j, 1);
		assert_eq!(iter.k, 3);
		assert!(iter.step());
		assert_eq!(iter.i, 1);
		assert_eq!(iter.j, 2);
		assert_eq!(iter.k, 4);
		assert!(iter.step());
		assert_eq!(iter.i, 0);
		assert_eq!(iter.j, 1);
		assert_eq!(iter.k, 4);
		assert!(iter.step());
		assert_eq!(iter.i, 0);
		assert_eq!(iter.j, 2);
		assert_eq!(iter.k, 3);
		assert!(iter.step());
		assert_eq!(iter.i, 1);
		assert_eq!(iter.j, 3);
		assert_eq!(iter.k, 4);
		assert!(iter.step());
		assert_eq!(iter.i, 0);
		assert_eq!(iter.j, 2);
		assert_eq!(iter.k, 4);
		assert!(iter.step());
		assert_eq!(iter.i, 0);
		assert_eq!(iter.j, 3);
		assert_eq!(iter.k, 4);
		assert!(!iter.step());
	}


	#[test]
	fn test_update_ijk ( )
	{
		// Enjoy these senseless values :P
		let mut iter = KernelIterator::new(0);
		iter.di = 1;
		iter.update_ijk();
		assert_eq!(iter.i, iter.di - 1);
		assert_eq!(iter.j, iter.di + iter.dj - 1);
		assert_eq!(iter.k, iter.di + iter.dj + iter.dk - 1);
	}

}
