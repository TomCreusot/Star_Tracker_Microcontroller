//! Implementation of [Equatorial](crate::util::units::Equatorial).
use std::fmt;
use std::ops::RangeInclusive;

use super::Equatorial;
use super::Vector3;
use super::Radians;
use crate::util::aliases::Decimal;
use crate::util::aliases::M_PI;

impl Equatorial
{
	/// Returns an equatorial coordinate at ra: 0, dec: 0 (on the equator at 0 hours).  
	/// Useful for initialization or testing.  
	/// # Example
	/// ```
	/// use star_tracker::util::units::Equatorial;
	/// use star_tracker::util::units::Radians;
	///
	/// assert_eq!(Equatorial{ra: Radians(0.0), dec: Radians(0.0)}, Equatorial::zero());
	/// ```
	pub const fn zero ( ) -> Equatorial
	{
		return Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
	}


	/// The range for declination.  
	/// Useful for random genration testing.  
	pub fn range_dec ( ) -> RangeInclusive<Radians>
	{
		return Radians(-M_PI / 2.0) ..= Radians(M_PI / 2.0);
	}

	/// The range for right ascention
	/// Useful for random genration testing.  
	pub fn range_ra ( ) -> RangeInclusive<Radians>
	{
		return Radians(0.0) ..= Radians(2.0 * M_PI);
	}

	/// Returns dec + 180 degrees to the declination to match with spherical coordinates.
	///	# Example
	/// ```
	/// use star_tracker::util::units::Equatorial;
	/// use star_tracker::util::units::Radians;
	/// use star_tracker::util::aliases::M_PI;
	///
	/// let equ = Equatorial { ra: Radians(0.0), dec: Radians(-M_PI / 2.0) };
	/// assert_eq!( equ.get_phi(), Radians(0.0) );
	/// ```
	pub fn get_phi ( &self ) -> Radians
	{
		return self.dec + Radians(M_PI / 2.0);
	}

	/// Sets phi (start at z = +1).
	///	# Example
	/// ```
	/// use star_tracker::util::units::Equatorial;
	/// use star_tracker::util::units::Radians;
	/// use star_tracker::util::aliases::M_PI;
	///
	/// let mut equ = Equatorial { ra: Radians(0.0), dec: Radians(0.0) };
	/// equ.set_phi( Radians(M_PI / 4.0) );
	/// assert_eq!( equ.dec, Radians(-M_PI / 4.0) );
	/// ```
	pub fn set_phi ( &mut self, angle: Radians )
	{
		self.dec = angle - Radians(M_PI / 2.0);
	}


	/// USE Vector3.angular_distance IF YOU HAVE A CARTESIAN, CONVERTING TO EQUATORIAL HAS A SINGULARITY!!!  
	/// Finds the angle between the 2 points on a sphere.  
	/// Just a shortcut to Vector3 equivalent.
	/// # Example
	/// ```
	/// use star_tracker::util::units::Equatorial;
	/// use star_tracker::util::units::Radians;
	/// use star_tracker::util::aliases::M_PI;
	///
	/// let mut equ1 = Equatorial { ra: Radians(M_PI), dec: Radians(M_PI / 2.0) };
	/// let mut equ2 = Equatorial { ra: Radians(-M_PI), dec: Radians(-M_PI / 2.0) }; // 180 degrees because of dec
	/// assert_eq!(equ1.angle_distance(equ2), Radians(M_PI));
	///
	/// equ1 = Equatorial { ra: Radians(0.0), dec: Radians(M_PI / 4.0) };
	/// equ2 = Equatorial { ra: Radians(M_PI), dec: Radians(M_PI / 4.0) };
	/// assert_eq!(equ1.angle_distance(equ2), Radians(M_PI/2.0));
	/// ```
	pub fn angle_distance ( &self, other: Equatorial ) -> Radians
	{
		let cur = self.to_vector3();
		let oth = other.to_vector3();
		return cur.angle_distance(oth);
	}

	/// Finds the distance between the 2 points cutting a strait 2d line through the sphere.
	/// # Example
	/// ```
	/// use star_tracker::util::units::Radians;
	/// use star_tracker::util::units::Equatorial;
	/// use star_tracker::util::aliases::M_PI;
	///
	/// let mut equ1 = Equatorial { ra: Radians(M_PI), dec: Radians(M_PI / 2.0) };
	/// let mut equ2 = Equatorial { ra: Radians(-M_PI), dec: Radians(-M_PI / 2.0) }; // 180 degrees because of dec
	/// assert_eq!(equ1.planar_distance(equ2), 2.0);
	///
	/// equ1 = Equatorial { ra: Radians(0.0), dec: Radians(M_PI / 4.0) };
	/// equ2 = Equatorial { ra: Radians(M_PI), dec: Radians(M_PI / 4.0) };
	/// assert_eq!(equ1.planar_distance(equ2), 1.4142135623730951);
	/// ```
	pub fn planar_distance ( &self, other: Equatorial ) -> Decimal
	{
		let cur = self.to_vector3();
		let oth = other.to_vector3();
		return
		((cur.x - oth.x).powf(2.0) + (cur.y - oth.y).powf(2.0) + (cur.z - oth.z).powf(2.0)).sqrt();
	}

	/// Gets a 3d cartesian coordinate from the equatorial unit sphere.
	///
	/// # Example
	/// ```
	/// use star_tracker::util::units::Equatorial;
	/// use star_tracker::util::units::Radians
	/// use star_tracker::util::aliases::M_PI;
	/// let equ = Equatorial { ra: Radians(M_PI / 4.0), dec: Radians(M_PI / 4.0) };
	/// let cart = equ.to_vector3();
	/// assert!((cart.x - 0.5).abs() < 0.0001);
	/// assert!((cart.y - 0.5).abs() < 0.0001);
	/// assert!((cart.z - 0.7071067812).abs() < 0.0001);
	/// ```
	pub fn to_vector3 ( &self ) -> Vector3
	{
		return Vector3 {
			x: self.ra.cos() * self.get_phi().sin(),
			y: self.ra.sin() * self.get_phi().sin(),
			z: -self.get_phi().cos()
		};
	}
}


//###############################################################################################//
//							---	Debug ---
//###############################################################################################//


impl fmt::Display for Equatorial {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "Equatorial({:.2}, {:.2})d", self.ra.to_degrees().0, self.dec.to_degrees().0)?;
		return Ok(());
	}
}


impl fmt::Debug for Equatorial {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "Equatorial(ra: {}, dec: {})", self.ra, self.dec)?;
		return Ok(());
	}
}

impl Equatorial
{
	/// Prints in standard ra: hours, dec: degrees.
	pub fn print_standard ( &self ) -> String
	{
		let ra_hours = self.ra.0 / M_PI * 12.0;
		let ra_minutes = ra_hours.fract() * 60.0;
		let ra_seconds = ra_minutes.fract() * 60.0;
		let ra = format!("{:2.0}h {:2.0}m {:5.2}s", ra_hours, ra_minutes, ra_seconds);

		let dec_degrees = self.dec.to_degrees().0;
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
	use rand::prelude::*;

	use crate::util::aliases::Decimal;
	use crate::util::aliases::M_PI;
	use crate::util::units::Equatorial;
	use crate::util::units::Radians;
	use crate::util::units::Vector3;
	use crate::util::test::TestEqual;


	fn assert_close ( a: Decimal, b: Decimal ) -> bool
	{
		return (a - b).abs() < 0.00001;
	}


//###############################################################################################//
//
//									Constructors and Accessors
//
// pub fn zero       ( &self ) -> Equatorial
// pub fn range_ra   ( &self ) -> RangeInclusive<Radians>
// pub fn range_dec  ( &self ) -> RangeInclusive<Radians>
// pub fn get_phi    ( &self ) -> Radians
// pub fn set_phi    ( &self ) -> Radians
//
//###############################################################################################//
//										~ zero ~												 //
	#[test]
	fn test_zero ( )
	{
		assert_eq!(Equatorial{ra: Radians(0.0), dec: Radians(0.0)}, Equatorial::zero());
	}

//										~ range_dec / range_ra ~								 //
	#[test]
	fn test_range_dec_ra ( )
	{
		assert_eq!(Equatorial::range_ra(),  Radians(0.0) ..= Radians(M_PI * 2.0));
		assert_eq!(Equatorial::range_dec(), Radians(-M_PI / 2.0) ..= Radians(M_PI) / 2.0);
	}

//										~ get_phi ~												 //
	#[test]
	fn test_get_phi ( )
	{
		let mut equ = Equatorial { ra: Radians(0.0), dec: Radians(0.0) };
		assert_eq!(equ.get_phi().0, equ.dec.0 + M_PI / 2.0);

		equ = Equatorial { ra: Radians(2.0), dec: Radians(1.0) };
		assert_eq!(equ.get_phi().0, equ.dec.0 + M_PI / 2.0);

		equ = Equatorial { ra: Radians(2.0), dec: Radians(-1.0) };
		assert_eq!(equ.get_phi().0, equ.dec.0 + M_PI / 2.0);
	}

//										~ set_phi ~												 //
	#[test]
	fn test_set_phi ( )
	{
		let mut equ = Equatorial { ra: Radians(0.0), dec: Radians(0.0) };
		equ.set_phi( Radians(M_PI / 4.0) );
		assert_eq!( equ.dec.0, -M_PI / 4.0 );
	}



//###############################################################################################//
//
//									Others
//
// pub fn angle_distance    ( &self, Self )    -> Radians
// pub fn planar_distance   ( &self, Self )    -> Radians
// pub fn to_vector3        ( &self )          -> Vector3
//
//###############################################################################################//
//										~ angle_distance ~										 //
	#[test]
	fn test_angle_distance ( )
	{
		let mut equ1 = Equatorial { ra: Radians(M_PI), dec: Radians(M_PI / 2.0) };
		let mut equ2 = Equatorial { ra: Radians(-M_PI), dec: Radians(-M_PI / 2.0) }; // 180 degrees because of dec
		assert_eq!(equ1.angle_distance(equ2).0, M_PI);

		equ1 = Equatorial { ra: Radians(0.0), dec: Radians(M_PI / 4.0) };
		equ2 = Equatorial { ra: Radians(M_PI),  dec: Radians(M_PI / 4.0) };
		assert!((equ1.angle_distance(equ2).0 - M_PI/2.0).abs() < 0.0000001);
	}

	#[test]
	fn test_angle_distance_latitude ( )
	{
		let mut equ1 = Equatorial { ra: Radians(0.0), dec: Radians(M_PI / 2.0) };
		let mut equ2 = Equatorial { ra: Radians(0.0), dec: Radians(-M_PI / 2.0) };
		assert_eq!(equ1.angle_distance(equ2).0, M_PI);

		equ1 = Equatorial { ra: Radians(0.0), dec: Radians(-M_PI * 2.0) };
		equ2 = Equatorial { ra: Radians(0.0), dec: Radians(M_PI * 2.0) };
		assert_eq!(equ1.angle_distance(equ2).0, 0.0);
	}

	#[test]
	fn test_angle_distance_longitude ( )
	{
		let equ1 = Equatorial { ra: Radians(M_PI / 2.0), dec: Radians(0.0) };
		let equ2 = Equatorial { ra: Radians(-M_PI / 2.0), dec: Radians(0.0) };
		assert_eq!(equ1.angle_distance(equ2).0, M_PI);
	}

	#[test]
	fn test_angle_distance_90_dec ( )
	{
		let equ1 = Equatorial { ra: Radians(M_PI), dec: Radians(M_PI / 2.0) };
		let equ2 = Equatorial { ra: Radians(-M_PI), dec: Radians(0.0) };
		assert_eq!(equ1.angle_distance(equ2).0, M_PI / 2.0);
	}



//										~ planar_distance ~										 //
	#[test]
	fn test_planar_distance ( )
	{
		let mut equ1 = Equatorial { ra: Radians(M_PI), dec: Radians(M_PI / 2.0) };
		let mut equ2 = Equatorial { ra: Radians(-M_PI), dec: Radians(-M_PI / 2.0) }; // 180 degrees because of dec
		assert_close(equ1.planar_distance(equ2), 2.0);

		equ1 = Equatorial { ra: Radians(0.0), dec: Radians(M_PI / 4.0) };
		equ2 = Equatorial { ra: Radians(M_PI), dec: Radians(M_PI / 4.0) };
		assert_close(equ1.planar_distance(equ2), 1.4142135);
	}


	#[test]
	fn test_planar_distance_ra ( )
	{
		let mut equ1 = Equatorial { ra: Radians(M_PI), dec: Radians(0.0) };
		let mut equ2 = Equatorial { ra: Radians(-M_PI), dec: Radians(0.0) };
		assert_close(equ1.planar_distance(equ2), 0.0);

		equ1 = Equatorial { ra: Radians(0.0), dec: Radians(0.0) };
		equ2 = Equatorial { ra: Radians(-M_PI), dec: Radians(0.0) };
		assert_close(equ1.planar_distance(equ2), 2.0);

		equ1 = Equatorial { ra: Radians(0.0), dec: Radians(M_PI / 2.0) };
		equ2 = Equatorial { ra: Radians(-M_PI), dec: Radians(M_PI / 2.0) };
		assert_close(equ1.planar_distance(equ2), 0.0);
	}

	#[test]
	fn test_planar_distance_dec ( )
	{
		let mut equ1 = Equatorial { ra: Radians(0.0), dec: Radians(M_PI/2.0) };
		let mut equ2 = Equatorial { ra: Radians(0.0), dec: Radians(M_PI/2.0) };
		assert_eq!(equ1.planar_distance(equ2), 0.0);

		equ1 = Equatorial { ra: Radians(0.0), dec: Radians(-M_PI / 2.0) };
		equ2 = Equatorial { ra: Radians(0.0), dec: Radians(M_PI / 2.0) };
		assert_eq!(equ1.planar_distance(equ2), 2.0);
	}


//										~ to_vector3 ~											 //
	#[test]
	fn test_to_vector3 ( )
	{
		let equ = Equatorial { ra: Radians(M_PI / 4.0), dec: Radians(M_PI / 4.0) };
		let cart = equ.to_vector3();
		assert_close(cart.x, 0.5);
		assert_close(cart.y, 0.5);
		assert_close(cart.z, -0.7071067812);
	}

	#[test]
	fn test_to_vector3_x ( )
	{
		let mut equ = Equatorial { ra: Radians(0.0), dec: Radians(0.0) };
		let mut cart = equ.to_vector3();
		assert_close(cart.x, 1.0);
		assert_close(cart.y, 0.0);
		assert_close(cart.z, 0.0);

		equ = Equatorial { ra: Radians(M_PI), dec: Radians(0.0) };
		cart = equ.to_vector3();
		assert_close(cart.x, -1.0);
		assert_close(cart.y, 0.0);
		assert_close(cart.z, 0.0);

		equ = Equatorial { ra: Radians(-M_PI), dec: Radians(0.0) };
		cart = equ.to_vector3();
		assert_close(cart.x, -1.0);
		assert_close(cart.y, 0.0);
		assert_close(cart.z, 0.0);
	}

	#[test]
	fn test_to_vector3_y ( )
	{
		let mut equ = Equatorial { ra: Radians(M_PI / 2.0), dec: Radians(0.0) };
		let mut cart = equ.to_vector3();
		assert_close(cart.x, 0.0);
		assert_close(cart.y, 1.0);
		assert_close(cart.z, 0.0);

		equ = Equatorial { ra: Radians(-M_PI / 2.0), dec: Radians(0.0) };
		cart = equ.to_vector3();
		assert_close(cart.x, 0.0);
		assert_close(cart.y, -1.0);
		assert_close(cart.z, 0.0);
	}

	#[test]
	fn test_to_vector3_z ( )
	{
		let mut equ = Equatorial { ra: Radians(0.0), dec: Radians(M_PI / 2.0) };
		let mut cart = equ.to_vector3();
		assert_close(cart.x, 0.0);
		assert_close(cart.y, 0.0);
		assert_close(cart.z, -1.0);

		equ = Equatorial { ra: Radians(0.0), dec: Radians(-M_PI / 2.0) };
		cart = equ.to_vector3();
		assert_close(cart.x, 0.0);
		assert_close(cart.y, 0.0);
		assert_close(cart.z, 1.0);
	}


	#[test]
	fn test_to_vector3_3_random ( )
	{
		let mut rng = rand::thread_rng();
		for _i in 0..100
		{
			let mut c =
			Vector3{
				x: rng.gen::<Decimal>() - 0.5,
				y: rng.gen::<Decimal>() - 0.5,
				z: rng.gen::<Decimal>() - 0.5,
			};
			c.normalize().expect("make sure it is not 0,0,0");
			let e = c.to_equatorial();

			assert!(e.test_close(&c.to_equatorial(), 0.0001));
		}
	}
}
