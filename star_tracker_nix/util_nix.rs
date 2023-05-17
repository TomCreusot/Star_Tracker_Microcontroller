use std::fmt;
use util::test::TestEqual;



//###############################################################################################//
//										---	Hours ---
//###############################################################################################//
/// Returns a time format.  
/// This is structured as `##h ##m ###.##s`.  
/// You can find this used as `ra` in [Equatorial](crate::util::units::Equatorial).  
/// # Example
/// ```
/// use star_tracker::util::units::Hours;
///
/// let angle = Hours(12.345);
/// assert_eq!(angle.time_format(), "12h 20m 42.00s");
/// ```
pub fn time_format ( &self ) -> String
{
	return format!("{:2.0}h {:2.0}m {:5.2}s", self.hours(), self.minutes(), self.seconds());
}




//###############################################################################################//
//										---	Equatorial ---
//###############################################################################################//

impl Equatorial
{
	/// Prints in standard ra: hours, dec: degrees.
	pub fn print_standard ( &self ) -> String
	{
		let ra = self.ra.to_hours().time_format();

		let dec_degrees = self.dec.to_degrees().0;
		
		// let mut dec_hour = self.dec.to_hours();
		// dec_hour = Hours(dec_hour.0.fract());
		// let dec_minutes = dec_hour.minutes();
		// let dec_seconds = dec_hour.seconds();
		let dec_minutes = (dec_degrees.fract() * 60.0).copysign(1.0); // arc minutes (1/60 degree).
		let dec_seconds = (dec_minutes.fract() * 60.0).copysign(1.0); // arc minutes (1/60 degree).
		let dec = format!("{:2.0}° {:2.0}' {:5.2}\"", dec_degrees, dec_minutes, dec_seconds);

		return format!("J200( {} | {} )", ra, dec);
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


	#[test]
	fn test_time_format ( )
	{
		let angle = Hours(12.345);
		assert_eq!(angle.time_format(), "12h 20m 42.00s");
	}
	
	
	
	#[test]
	fn test_print_standard ( )
	{
		let eq = Equatorial{ra: Hours(12.43).to_radians(), dec: Degrees(20.1234).as_radians()};
		assert_eq!(eq.print_standard(), "J200( 12h 25m 48.00s | 20°  7' 24.24\" )");
		
	}
}