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
	/// * `other`     - The element to compare with.
	/// * `precision` - The tolerance.
	/// # Returns
	/// True if equal.
	fn test_close ( &self, other: &Self, precision: Decimal ) -> bool;


	/// Auto implemented function which calls test_equal with precision of DECIMAL_PRECISION_TEST.
	/// # Arguments
	/// * `other` - The element to compare to.
	/// # Returns
	/// True if equal.
	fn test_equal ( &self, other: &Self ) -> bool where Self: Debug
	{
		println!("left: {:?} \t\t right: {:?}", self, other);
		return self.test_close(other, DECIMAL_PRECISION_TEST);
	}
	
	#[inline]
	#[track_caller]
	fn assert_close ( &self, other: &Self, precision: Decimal ) where Self:Debug
	{
		assert!(self.test_close(other, precision), 
		"values not close left: {:?} | right; {:?} || precision {}", self, other, precision);
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
	use crate::util::test::DECIMAL_PRECISION_TEST;
	use crate::util::test::TestEqual;
	use crate::util::units::Degrees;


//###############################################################################################//
//
//										TestEqual
//
// pub fn test_equal   ( &self, &Self ) -> bool
// pub fn assert_close ( &self, &Self, Decimal ) -> PANIC
//
//###############################################################################################//
//										~ magnitude ~											 //
	#[test]
	fn test_equal_close_valid ( )
	{
		let a = Degrees(123.0);
		let b = Degrees(123.0 + DECIMAL_PRECISION_TEST * 0.9);
		assert!(a.test_equal(&b));
	}
	
	#[test]
	fn test_equal_close_invalid ( )
	{
		let a = Degrees(123.0);
		let b = Degrees(123.0 + DECIMAL_PRECISION_TEST * 1.1);
		assert!(!a.test_equal(&b));
	}
	
	#[test] #[should_panic]
	fn test_assert_close_invalid ( )
	{
		let a = Degrees(123.0);
		let b = Degrees(123.0 + 10.0);
		a.assert_close(&b, 9.9);
	}
	
	#[test]
	fn test_assert_close_valid ( )
	{
		let a = Degrees(123.0);
		let b = Degrees(123.0 + 10.0);
		a.assert_close(&b, 10.1);
	}


}