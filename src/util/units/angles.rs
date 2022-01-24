/// Implementation for Degrees and Radians.
use crate::util::aliases::{M_PI, Decimal};

use super::{Degrees, Radians, Hours};
impl Degrees
{
	/// Converts to radians.
 	pub fn to_radians ( &self ) -> Radians
 	{
 		return Radians(self.0 * M_PI / 180.0);
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
	/// Converts to radians.
	pub fn to_radians ( &self ) -> Radians
	{
		return Radians ( self.0 * M_PI / 12.0 );
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
	use util::aliases::{M_PI, Decimal};
	use util::units::{Radians, Degrees, Hours};

	#[test]
	fn to_radians ( )
	{
		assert_eq!(Degrees(0.0).to_radians(),   Radians(0.0));
		assert_eq!(Degrees(45.0).to_radians(),  Radians(M_PI / 4.0));
		assert_eq!(Degrees(90.0).to_radians(),  Radians(M_PI / 2.0));
		assert_eq!(Degrees(180.0).to_radians(), Radians(M_PI));
		assert_eq!(Degrees(360.0).to_radians(), Radians(M_PI * 2.0));

		assert_eq!(Hours(0.0).to_radians(),   Radians(0.0));
		assert_eq!(Hours(6.0).to_radians(),  Radians(M_PI / 2.0));
		assert_eq!(Hours(12.0).to_radians(),  Radians(M_PI));
		assert_eq!(Hours(18.0).to_radians(),  Radians(M_PI * 6.0 / 4.0));
		assert_eq!(Hours(24.0).to_radians(),  Radians(M_PI * 2.0));
	}

	#[test]
	fn to_degrees ( )
	{
		assert_eq!(Degrees(0.0),   Radians(0.0).to_degrees());
		assert_eq!(Degrees(45.0),  Radians(M_PI / 4.0).to_degrees());
		assert_eq!(Degrees(90.0),  Radians(M_PI / 2.0).to_degrees());
		assert_eq!(Degrees(180.0), Radians(M_PI).to_degrees());
		assert_eq!(Degrees(360.0), Radians(M_PI * 2.0).to_degrees());
	}
	
	
	#[test]
	fn to_hours ( )
	{
		assert_eq!(Hours(0.0),   Radians(0.0).to_hours());
		assert_eq!(Hours(6.0),  Radians(M_PI / 2.0).to_hours());
		assert_eq!(Hours(12.0),  Radians(M_PI).to_hours());
		assert_eq!(Hours(18.0),  Radians(M_PI * 6.0 / 4.0).to_hours());
		assert_eq!(Hours(24.0),  Radians(M_PI * 2.0).to_hours());
	}
	
	
	
	#[test]
	fn test_sin ( )
	{
		assert_eq!((0.1 as Decimal).sin(),   Radians(0.1).sin());
		assert_eq!((0.4 as Decimal).sin(),   Radians(0.4).sin());
		assert_eq!((100.0 as Decimal).sin(), Radians(100.0).sin());
		
		assert_eq!((0.1 as Decimal).sin(),   Degrees(0.1).sin());
		assert_eq!((0.4 as Decimal).sin(),   Degrees(0.4).sin());
		assert_eq!((100.0 as Decimal).sin(), Degrees(100.0).sin());
		
	}
	
	
	#[test]
	fn test_cos ( )
	{
		assert_eq!((0.1 as Decimal).cos(),   Radians(0.1).cos());
		assert_eq!((0.4 as Decimal).cos(),   Radians(0.4).cos());
		assert_eq!((100.0 as Decimal).cos(), Radians(100.0).cos());
	
		assert_eq!((0.1 as Decimal).cos(),   Degrees(0.1).cos());
		assert_eq!((0.4 as Decimal).cos(),   Degrees(0.4).cos());
		assert_eq!((100.0 as Decimal).cos(), Degrees(100.0).cos());
	}
	
	
}