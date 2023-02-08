//! This is a series of functions and traits to aid in testing.

use std::fmt::Debug;

use crate::util::aliases::DECIMAL_PRECISION;
use crate::util::aliases::Decimal;


/// When testing something close to the precision of Decimal which can be slightly inaccurate.
pub const DECIMAL_PRECISION_TEST :	Decimal = DECIMAL_PRECISION * 1000.0;


/// An equality test with a precision variable so the equality can be better checked.
pub trait TestEqual
{
	/// An equality test with a precision variable so the equality can be better checked.
	/// # Arguments
	/// * `other` 		- The element to compare with.
	/// * `precision`	- The tolerance.
	/// # Returns
	/// True if equal.
	fn test_close ( &self, other: &Self, precision: Decimal ) -> bool;


	/// Auto implemented function which calls test_equal with precision of DECIMAL_PRECISION_TEST.
	/// # Arguments
	/// * `other`		- The element to compare to.
	/// # Returns
	/// True if equal.
	fn test_equal ( &self, other: &Self ) -> bool where Self: Debug
	{
		println!("left: {:?} \t\t right: {:?}", self, other);
		return self.test_close(other, DECIMAL_PRECISION_TEST);
	}
/*

	/// Auto implemented function which calls test_equal with precision of DECIMAL_PRECISION_TEST.
	/// # Arguments
	/// * `other`		- The element to compare to.
	/// # Returns
	/// True if equal.
	fn test_equal ( &self, other: &Self ) -> bool
	{
		return self.test_close(other, DECIMAL_PRECISION_TEST);
	}*/
}
