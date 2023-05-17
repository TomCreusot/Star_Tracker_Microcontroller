//! Implementation of [Degrees](crate::util::units::Degrees),
//! [Radians](crate::util::units::Radians) and
//! [Hours](crate::util::units::Hours).
use crate::util::aliases::Decimal;
use crate::util::aliases::M_PI;
use crate::util::Maths;

use crate::util::units::Degrees;
use crate::util::units::Radians;
use crate::util::units::Hours;

impl Degrees
{
	/// Converts to Radians.
	#[inline]
 	pub fn to_radians ( &self ) -> Radians
 	{
 		return Radians(self.0 * M_PI / 180.0);
 	}
	
	/// Constant version of to_radians, only works with a constant degrees.
	pub const fn as_radians ( &self ) -> Radians
	{
		return Radians(self.0 * M_PI / 180.0)
	}
	
	/// Converts to Hours.
	pub fn to_hours ( &self ) -> Hours
	{
		return Hours(self.0 * 24.0 / 360.0 );
	}
	
	/// Accessor for sin.
	pub fn sin ( &self ) -> Decimal
	{
		return self.0.sin();
	}

	/// Accessor for cos.
	pub fn cos ( &self ) -> Decimal
	{
		return self.0.cos();
	}
}






impl Radians
{
	/// Converts to degrees.
	#[inline]
 	pub fn to_degrees ( &self ) -> Degrees
 	{
 		return Degrees(self.0 / M_PI * 180.0);
 	}
	
	/// Converts to hours format.
	pub fn to_hours ( &self ) -> Hours
	{
		return Hours ( self.0 * 12.0 / M_PI );
	}
	
	/// Accessor for sin.
	pub fn sin ( &self ) -> Decimal
	{
		return self.0.sin();
	}

	/// Accessor for cos.
	pub fn cos ( &self ) -> Decimal
	{
		return self.0.cos();
	}
}


impl Hours
{
	/// Returns the hour component of hours minutes seconds.
	pub fn hours ( &self ) -> Decimal { return self.0.floor(); }
	
	/// Returns the minutes component of hours minutes seconds.
	pub fn minutes ( &self ) -> Decimal	
	{	
		return (self.0.fract().copysign(1.0) * 60.0).floor();
	}

	/// Returns the seconds component of hours minutes seconds.
	pub fn seconds ( &self ) -> Decimal
	{	
		return (self.0.fract().copysign(1.0) * 60.0).fract() * 60.0;
	}
	
	/// Converts to radians.
	pub fn to_radians ( &self ) -> Radians
	{
		return Radians ( self.0 * M_PI / 12.0 );
	}
	
	
	/// Converts to Degrees.
	pub fn to_degrees ( &self ) -> Degrees
	{
		return Degrees ( self.0 * 360.0 / 24.0 );
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
	use crate::util::units::Radians;
	use crate::util::units::Degrees;
	use crate::util::units::Hours;
	use crate::util::aliases::Decimal;
	use crate::util::aliases::M_PI;

	
//###############################################################################################//
//
//								Degrees/Hours/Radians
//
// pub       fn to_radians ( &self ) -> Radians;
// pub const fn as_radians ( &self ) -> Radians;
// pub       fn to_hours   ( &self ) -> Hours;
// pub       fn sin        ( )       -> Decimal;
// pub       fn cos        ( )       -> Decimal;
//
//###############################################################################################//

//										~ to_radians ~											 //
	#[test]
	fn to_radians_from_degrees ( )
	{
		assert_eq!(Degrees(0.0).to_radians(),   Radians(0.0));
		assert_eq!(Degrees(45.0).to_radians(),  Radians(M_PI / 4.0));
		assert_eq!(Degrees(90.0).to_radians(),  Radians(M_PI / 2.0));
		assert_eq!(Degrees(180.0).to_radians(), Radians(M_PI));
		assert_eq!(Degrees(360.0).to_radians(), Radians(M_PI * 2.0));
	}
	
	#[test]
	fn to_radians_from_hours ( )
	{
		assert_eq!(Hours(0.0).to_radians(),   Radians(0.0));
		assert_eq!(Hours(6.0).to_radians(),  Radians(M_PI / 2.0));
		assert_eq!(Hours(12.0).to_radians(),  Radians(M_PI));
		assert_eq!(Hours(18.0).to_radians(),  Radians(M_PI * 6.0 / 4.0));
		assert_eq!(Hours(24.0).to_radians(),  Radians(M_PI * 2.0));
	}

//										~ as_radians ~											 //
	#[test]
	fn as_radians_from_degrees ( )
	{
		assert_eq!(Degrees(0.0).as_radians(),   Radians(0.0));
		assert_eq!(Degrees(45.0).as_radians(),  Radians(M_PI / 4.0));
		assert_eq!(Degrees(90.0).as_radians(),  Radians(M_PI / 2.0));
		assert_eq!(Degrees(180.0).as_radians(), Radians(M_PI));
		assert_eq!(Degrees(360.0).as_radians(), Radians(M_PI * 2.0));
	}

//										~ to_degrees ~											 //
	#[test]
	fn to_degrees_from_radians ( )
	{
		assert_eq!(Degrees(0.0),   Radians(0.0).to_degrees());
		assert_eq!(Degrees(45.0),  Radians(M_PI / 4.0).to_degrees());
		assert_eq!(Degrees(90.0),  Radians(M_PI / 2.0).to_degrees());
		assert_eq!(Degrees(180.0), Radians(M_PI).to_degrees());
		assert_eq!(Degrees(360.0), Radians(M_PI * 2.0).to_degrees());
		assert_eq!(Degrees(0.0),   Radians(0.0).to_degrees());
	}
		
	#[test]
	fn to_degrees_from_hours ( )
	{
		assert_eq!(Hours(3.0).to_degrees(),  Radians(M_PI / 4.0).to_degrees());
		assert_eq!(Hours(6.0).to_degrees(), Radians(M_PI / 2.0).to_degrees());
		assert_eq!(Hours(12.0).to_degrees(), Radians(M_PI).to_degrees());
		assert_eq!(Hours(24.0).to_degrees(), Radians(M_PI * 2.0).to_degrees());
	}
	
	
//										~ to_hours ~											 //
	#[test]
	fn to_hours_from_radians ( )
	{
		assert_eq!(Hours(0.0),   Radians(0.0).to_hours());
		assert_eq!(Hours(6.0),   Radians(M_PI / 2.0).to_hours());
		assert_eq!(Hours(12.0),  Radians(M_PI).to_hours());
		assert_eq!(Hours(18.0),  Radians(M_PI * 6.0 / 4.0).to_hours());
		assert_eq!(Hours(24.0),  Radians(M_PI * 2.0).to_hours());
	}	
	
	#[test]
	fn to_hours_from_degrees ( )
	{
		assert_eq!(Degrees(0.0).to_hours(),   Radians(0.0).to_hours());
		assert_eq!(Degrees(90.0).to_hours(),  Radians(M_PI / 2.0).to_hours());
		assert_eq!(Degrees(180.0).to_hours(),  Radians(M_PI).to_hours());
		assert_eq!(Degrees(270.0).to_hours(),  Radians(M_PI * 6.0 / 4.0).to_hours());
		assert_eq!(Degrees(360.0).to_hours(),  Radians(M_PI * 2.0).to_hours());
	}
	
	
	
//										~ sin ~													 //
	#[test]
	fn test_sin_radians ( )
	{
		assert_eq!((0.1 as Decimal).sin(),   Radians(0.1).sin());
		assert_eq!((0.4 as Decimal).sin(),   Radians(0.4).sin());
		assert_eq!((100.0 as Decimal).sin(), Radians(100.0).sin());
	}
	
	#[test]
	fn test_sin_degrees ( )
	{
		assert_eq!((0.1 as Decimal).sin(),   Degrees(0.1).sin());
		assert_eq!((0.4 as Decimal).sin(),   Degrees(0.4).sin());
		assert_eq!((100.0 as Decimal).sin(), Degrees(100.0).sin());		
	}
	

//										~ cos ~													 //
	#[test]
	fn test_cos_radians ( )
	{
		assert_eq!((0.1 as Decimal).cos(),   Radians(0.1).cos());
		assert_eq!((0.4 as Decimal).cos(),   Radians(0.4).cos());
		assert_eq!((100.0 as Decimal).cos(), Radians(100.0).cos());
	}
	
	#[test]
	fn test_cos_degrees ( )
	{
		assert_eq!((0.1 as Decimal).cos(),   Degrees(0.1).cos());
		assert_eq!((0.4 as Decimal).cos(),   Degrees(0.4).cos());
		assert_eq!((100.0 as Decimal).cos(), Degrees(100.0).cos());
	}
	
	
	
//###############################################################################################//
//
//									Hours
//
// pub fn time_format ( &self ) -> String
// pub fn hours       ( &self ) -> Decimal
// pub fn minutes     ( &self ) -> Decimal
// pub fn seconds     ( &self ) -> Decimal
//
//###############################################################################################//

	
	#[test]
	fn test_hours ( )
	{
		let angle = Hours(12.345);
		assert_eq!(angle.hours(), 12.0);
	}
	
	#[test]
	fn test_minutes ( )
	{
		let angle = Hours(12.345);
		assert_eq!(angle.minutes(), 20.0);
	}
	
	#[test]
	fn test_seconds ( )
	{
		let angle = Hours(12.345);
		assert_eq!(angle.seconds().round(), 42.0);
	}
}
