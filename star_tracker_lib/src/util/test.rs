//! This is a series of functions and traits to aid in testing.

#[cfg(test)]      use std::fmt::Debug;
#[cfg(not(test))] use core::fmt::Debug;


use crate::util::aliases::DECIMAL_PRECISION;
use crate::util::aliases::Decimal;
use crate::util::Maths;
use crate::util::units::Degrees;
use crate::util::units::Radians;
use crate::util::units::Hours;
use crate::util::units::Quaternion;
use crate::util::units::Vector2;
use crate::util::units::Vector3;
use crate::util::units::Equatorial;
use crate::util::units::AngleAxis;

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
		#[cfg(test)] println!("left: {:?} \t\t right: {:?}", self, other);
		return self.test_close(other, DECIMAL_PRECISION_TEST);
	}
	
	#[inline]
	#[track_caller]
	/// This automatically calls an assert if test_close fails.  
	/// The call stack will display the line which calls this.
	fn assert_close ( &self, other: &Self, precision: Decimal ) where Self:Debug
	{
		assert!(self.test_close(other, precision), 
		"values not close left: {:?} | right; {:?} || precision {}", self, other, precision);
	}
}






//###############################################################################################//
//										---	Decimal ---
//###############################################################################################//
impl TestEqual for Decimal {
	fn test_close ( &self, other: &Self, precision: Decimal ) -> bool
	{ return (self - other).abs() < precision }
}

//###############################################################################################//
//										---	Degrees ---
//###############################################################################################//
impl TestEqual for Degrees {
	fn test_close ( &self, other: &Self, precision: Decimal ) -> bool
	{ return (self.0 - other.0).abs() < precision }
}

//###############################################################################################//
//										---	Radians ---
//###############################################################################################//
impl TestEqual for Radians {
	fn test_close ( &self, other: &Self, precision: Decimal ) -> bool
	{ return (self.0 - other.0).abs() < precision }
}

//###############################################################################################//
//										---	Hours ---
//###############################################################################################//
impl TestEqual for Hours {
	fn test_close ( &self, other: &Self, precision: Decimal ) -> bool
	{ return (self.0 - other.0).abs() < precision }
}

//###############################################################################################//
//										---	Quaternion ---
//###############################################################################################//
impl TestEqual for Quaternion {
	fn test_close ( &self, other: &Self, precision: Decimal ) -> bool
	{
		return
		(self.w - other.w).abs() < precision &&
		(self.x - other.x).abs() < precision &&
		(self.y - other.y).abs() < precision &&
		(self.z - other.z).abs() < precision;
	}
}

//###############################################################################################//
//										---	Vector2 ---
//###############################################################################################//
impl TestEqual for Vector2 {
	fn test_close ( &self, other: &Self, precision: Decimal ) -> bool
	{
		return
		(self.x - other.x).abs() < precision &&
		(self.y - other.y).abs() < precision;
	}
}

//###############################################################################################//
//										---	Vector3 ---
//###############################################################################################//
impl TestEqual for Vector3 {
	fn test_close ( &self, other: &Self, precision: Decimal ) -> bool
	{
		return
		(self.x - other.x).abs() < precision &&
		(self.y - other.y).abs() < precision &&
		(self.z - other.z).abs() < precision;
	}
}

//###############################################################################################//
//										---	Equatorial ---
//###############################################################################################//
impl TestEqual for Equatorial {
	fn test_close ( &self, other: &Self, precision: Decimal ) -> bool
	{ return (self.ra.0- other.ra.0).abs() < precision &&
		(self.dec.0 - other.dec.0).abs() < precision; }
}

//###############################################################################################//
//										---	AngleAxis ---
//###############################################################################################//
impl TestEqual for AngleAxis {
	fn test_close ( &self, other: &Self, precision: Decimal ) -> bool
	{ return self.angle.test_close(&other.angle, precision) &&
		other.axis.test_close(&other.axis, precision); }
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