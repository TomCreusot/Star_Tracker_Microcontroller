use star_tracker_lib::util::units::Hours;
use star_tracker_lib::util::units::Equatorial;



/// Formats the struct in a more user friendly format.
pub trait Formatted
{
    /// Formats the struct in a more user friendly format.
    fn formatted ( &self ) -> String;
}


impl Formatted for Hours
{
/// Prints hours in h/m/s format.
fn formatted ( &self ) -> String
{
	return format!("{:2.0}h {:2.0}m {:5.2}s", self.hours(), self.minutes(), self.seconds());
}
}



impl Formatted for Equatorial
{
/// Prints in standard ra: h/m/s, dec: d/am/as.
fn formatted ( &self ) -> String
{
	let ra = self.ra.to_hours().formatted();

	let dec_degrees = self.dec.to_degrees().0;
	let dec_minutes = (dec_degrees.fract() * 60.0).copysign(1.0); // arc minutes (1/60 degree).
	let dec_seconds = (dec_minutes.fract() * 60.0).copysign(1.0); // arc seconds (1/60 degree).
	let dec = format!("{:2.0}° {:2.0}' {:5.2}\"", dec_degrees, dec_minutes, dec_seconds);

	return format!("J200( {} | {} )", ra, dec);
}
}