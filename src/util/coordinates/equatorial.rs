//! The implementation of Equatorial.
use std::f32::consts::PI;
use crate::util::aliases::Decimal;
use super::{Equatorial, Cartesian3D};

impl Equatorial
{
	/// Returns the hour value instead of the decimal value.
	///	This is not useful for calculations, it is mostly for readability.
	///	# Example
	/// ```
	/// use std::f32::consts::PI;
	/// use star_tracker::util::coordinates::Equatorial;
	/// let equ = Equatorial { ra : PI, dec : 0.0 };
	/// assert_eq!( equ.get_ra_hour(), 12.0 );
	/// ```
	pub fn get_ra_hour ( &self ) -> Decimal
	{	return self.ra / PI * 12.0;				}

	/// Sets the struct in decimal provided the angle in hours.
	///	# Example
	/// ```
	/// use std::f32::consts::PI;
	/// use star_tracker::util::coordinates::Equatorial;
	/// let mut equ = Equatorial { ra : 0.0, dec : 0.0 };
	/// equ.set_ra_hour(12.0);
	/// assert_eq!( equ.ra, PI );
	/// ```
	pub fn set_ra_hour ( &mut self, angle_in_hours : Decimal )
	{	self.ra = angle_in_hours * PI / 12.0;	}

	/// Returns the degree value instead of the radian value.
	///	# Example
	/// ```
	/// use std::f32::consts::PI;
	/// use star_tracker::util::coordinates::Equatorial;
	/// let equ = Equatorial { ra : 0.0, dec : PI / 4.0 };
	/// assert_eq!( equ.get_dec_degrees(), 45.0 );
	/// ```
	pub fn get_dec_degrees ( &self ) -> Decimal
	{	return self.dec * 180.0 / PI;			}

	/// Sets the struct in decimal provided the angle in hours.
	///	# Example
	/// ```	
	/// use std::f32::consts::PI;
	/// use star_tracker::util::coordinates::Equatorial;
	/// let mut equ = Equatorial { ra : 0.0, dec : 0.0 };
	/// equ.set_dec_degrees(45.0);
	/// assert_eq!( equ.dec, PI / 4.0 );
	/// ```
	pub fn set_dec_degrees ( &mut self, angle_in_degrees : Decimal )
	{	self.dec = angle_in_degrees * PI / 180.0; }


	/// Returns dec + 180 degrees to the declination to match with spherical coordinates.
	///	# Example
	/// ```
	/// use std::f32::consts::PI;
	/// use star_tracker::util::coordinates::Equatorial;
	/// let equ = Equatorial { ra : 0.0, dec: -PI / 2.0 };
	/// assert_eq!( equ.get_phi(), 0.0 );
	/// ```
	pub fn get_phi ( &self ) -> Decimal
	{
		return self.dec + PI / 2.0;
	}

	/// Sets phi (start at z = +1).
	///	# Example
	/// ```
	/// use std::f32::consts::PI;
	/// use star_tracker::util::coordinates::Equatorial;
	/// let mut equ = Equatorial { ra : 0.0, dec: 0.0 };
	/// equ.set_phi( PI / 4.0 );
	/// assert_eq!( equ.dec, -PI / 4.0 );
	/// ```
	pub fn set_phi ( &mut self, angle: Decimal )
	{
		self.dec = angle - PI / 2.0;
	}

	/// Finds the angle between the 2 points on a sphere.
	/// # Example
	/// ```
	/// use std::f32::consts::PI;
	/// use star_tracker::util::coordinates::Equatorial;
	/// let mut equ1 = Equatorial { ra : PI, dec: PI / 2.0 };
	/// let mut equ2 = Equatorial { ra : -PI, dec: -PI / 2.0 }; // 180 degrees because of dec
	/// assert_eq!(equ1.angle_distance(equ2), PI);
	/// 
	/// equ1 = Equatorial { ra : 0.0, dec: PI / 4.0 };
	/// equ2 = Equatorial { ra : PI, dec: PI / 4.0 };
	/// assert_eq!(equ1.angle_distance(equ2), PI/2.0);
	/// ```
	pub fn angle_distance ( &self, other: Equatorial ) -> Decimal
	{
		let cur = self.to_cartesian3();
		let oth = other.to_cartesian3();

		let dot = cur.x * oth.x + cur.y * oth.y + cur.z * oth.z;
		let mag_cur = (cur.x.powf(2.0) + cur.y.powf(2.0) + cur.z.powf(2.0)).sqrt();
		let mag_oth = (oth.x.powf(2.0) + oth.y.powf(2.0) + oth.z.powf(2.0)).sqrt();

		return (dot / (mag_cur * mag_oth)).acos();
	}

	/// Finds the distance between the 2 points on a unit sphere.
	/// # Example
	/// ```
	///	use std::f32::consts::PI;
	/// use star_tracker::util::coordinates::Equatorial;
	/// let mut equ1 = Equatorial { ra : PI, dec: PI / 2.0 };
	/// let mut equ2 = Equatorial { ra : -PI, dec: -PI / 2.0 }; // 180 degrees because of dec
	/// assert_eq!(equ1.planar_distance(equ2), 2.0);
	///	
	/// equ1 = Equatorial { ra : 0.0, dec: PI / 4.0 };
	///	equ2 = Equatorial { ra : PI, dec: PI / 4.0 };
	///	assert_eq!(equ1.planar_distance(equ2), 1.4142135);
	/// ```
	pub fn planar_distance ( &self, other: Equatorial ) -> Decimal
	{
		let cur = self.to_cartesian3();
		let oth = other.to_cartesian3();
		return
		((cur.x - oth.x).powf(2.0) + (cur.y - oth.y).powf(2.0) + (cur.z - oth.z).powf(2.0)).sqrt();
	}

	/// Gets a 3d cartesian coordinate from the equatorial unit sphere.
	/// # Returns
	/// The cartesian points on a unit sphere.
	///
	/// # Example
	/// ```
	/// use std::f32::consts::PI;
	/// use star_tracker::util::coordinates::Equatorial;
	/// let equ = Equatorial { ra : PI / 4.0, dec: PI / 4.0 };
	/// let cart = equ.to_cartesian3();
	/// assert!((cart.x - 0.5).abs() < 0.0001);
	/// assert!((cart.y - 0.5).abs() < 0.0001);
	/// assert!((cart.z - -0.7071067812).abs() < 0.0001);
	/// ```
	pub fn to_cartesian3 ( &self ) -> Cartesian3D<Decimal>
	{
		return Cartesian3D {
			x: self.ra.cos() * self.get_phi().sin(),
			y: self.ra.sin() * self.get_phi().sin(),
			z: self.get_phi().cos()
		};
	}
	
	
	/// Converts cartesian3D to equatorial coordinates.
	/// # Arguments
	/// * `c` - The cartesian coordinates in Decimal format as a unit vector.
	/// # Returns
	/// The equatorial equivalent of the unit vector.
	/// 
	/// # Example
	/// ```
	/// use std::f32::consts::PI;
	/// use star_tracker::util::coordinates::{Equatorial, Cartesian3D};
	/// use star_tracker::util::aliases::Decimal;
	///	let c : Cartesian3D<Decimal> = Cartesian3D { x: 0.5, y: 0.5, z: -0.7071067812 };
	///	let e = Equatorial::from_cartesian3(c);
	///	assert!((e.ra - PI / 4.0).abs() < 0.0001);
	///	assert!((e.dec - PI / 4.0).abs() < 0.0001);
	/// ```
	
	pub fn from_cartesian3 ( c : Cartesian3D<Decimal> ) -> Equatorial
	{
		let mut eq = Equatorial { ra: (c.y / c.x).atan(), dec: 0.0 };
		eq.set_phi((c.z / (c.x.powf(2.0) + c.y.powf(2.0) + c.z.powf(2.0)).sqrt()).acos());
		return eq;
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
	use std::f32::consts::PI;
//###############################################################################################//
//										---	Equatorial ---
//###############################################################################################//

	use util::coordinates::Equatorial;
	use util::coordinates::Cartesian3D;
	use util::aliases::Decimal;

	fn assert_close ( a: Decimal, b: Decimal )
	{
		if (a - b).abs() > 0.00001
		{
			panic!("\n\nassert_close failed: \n\tleft: `{}`\n\tright: `{}`\n\n", a, b);
		}
	}


	//
	//  getters / setters
	//
	#[test]
	fn test_get_ra_hour ( )
	{
		let equ = Equatorial { ra : PI, dec : 0.0 };
		assert_eq!( equ.get_ra_hour(), 12.0 );
	}
	#[test]
	fn test_set_ra_hour ( )
	{
		let mut equ = Equatorial { ra : 0.0, dec : 0.0 };
		equ.set_ra_hour(12.0);
		assert_eq!( equ.ra, PI );
	}
	#[test]
	fn test_get_dec_degrees ( )
	{
		let equ = Equatorial { ra : 0.0, dec : PI / 4.0 };
		assert_eq!( equ.get_dec_degrees(), 45.0 );
	}
	#[test]
	fn test_set_dec_degrees ( )
	{
		let mut equ = Equatorial { ra : 0.0, dec : 0.0 };
		equ.set_dec_degrees(45.0);
		assert_eq!( equ.dec, PI / 4.0 );
	}
	#[test]
	fn test_get_phi ( )
	{
		let mut equ = Equatorial { ra: 0.0, dec: 0.0 };
		assert_eq!(equ.get_phi(), equ.dec + PI / 2.0);

		equ = Equatorial { ra: 2.0, dec: 1.0 };
		assert_eq!(equ.get_phi(), equ.dec + PI / 2.0);

		equ = Equatorial { ra: 2.0, dec: -1.0 };
		assert_eq!(equ.get_phi(), equ.dec + PI / 2.0);
	}
	
	#[test]
	fn test_set_phi ( )
	{
		let mut equ = Equatorial { ra : 0.0, dec: 0.0 };
		equ.set_phi( PI / 4.0 );
		assert_eq!( equ.dec, -PI / 4.0 );
	}



	//
	//  testangle_distance ( Equatorial ) -> Decimal
	//

	#[test]
	fn test_angle_distance ( )
	{
		let mut equ1 = Equatorial { ra : PI, dec: PI / 2.0 };
		let mut equ2 = Equatorial { ra : -PI, dec: -PI / 2.0 }; // 180 degrees because of dec
		assert_close(equ1.angle_distance(equ2), PI);
		
		equ1 = Equatorial { ra : 0.0, dec: PI / 4.0 };
		equ2 = Equatorial { ra : PI, dec: PI / 4.0 };
		assert_close(equ1.angle_distance(equ2), PI/2.0);
	}

	#[test]
	fn test_angle_distance_latitude ( )
	{
		let mut equ1 = Equatorial { ra : 0.0, dec: PI / 2.0 };
		let mut equ2 = Equatorial { ra : 0.0, dec: -PI / 2.0 };
		assert_eq!(equ1.angle_distance(equ2), PI);
		
		equ1 = Equatorial { ra : 0.0, dec: -PI * 2.0 };
		equ2 = Equatorial { ra : 0.0, dec: PI * 2.0 };
		assert_eq!(equ1.angle_distance(equ2), 0.0);
	}

	#[test]
	fn test_angle_distance_longitude ( )
	{
		let equ1 = Equatorial { ra : PI / 2.0, dec: 0.0 };
		let equ2 = Equatorial { ra : -PI / 2.0, dec: 0.0 };
		assert_eq!(equ1.angle_distance(equ2), PI);
	}

	#[test]
	fn test_angle_distance_90_dec ( )
	{
		let equ1 = Equatorial { ra : PI, dec: PI / 2.0 };
		let equ2 = Equatorial { ra : -PI, dec: 0.0 };
		assert_eq!(equ1.angle_distance(equ2), PI / 2.0);
	}




	//
	//  planar_distance ( Equatorial ) -> Decimal
	//
	
	#[test]
	fn test_planar_distance ( )
	{
		let mut equ1 = Equatorial { ra : PI, dec: PI / 2.0 };
		let mut equ2 = Equatorial { ra : -PI, dec: -PI / 2.0 }; // 180 degrees because of dec
		assert_close(equ1.planar_distance(equ2), 2.0);
		
		equ1 = Equatorial { ra : 0.0, dec: PI / 4.0 };
		equ2 = Equatorial { ra : PI, dec: PI / 4.0 };
		assert_close(equ1.planar_distance(equ2), 1.4142135);
	}


	#[test]
	fn test_planar_distance_ra ( )
	{
		let mut equ1 = Equatorial { ra : PI, dec: 0.0 };
		let mut equ2 = Equatorial { ra : -PI, dec: 0.0 };
		assert_close(equ1.planar_distance(equ2), 0.0);
		
		equ1 = Equatorial { ra : 0.0, dec: 0.0 };
		equ2 = Equatorial { ra : -PI, dec: 0.0 };
		assert_close(equ1.planar_distance(equ2), 2.0);
		
		equ1 = Equatorial { ra : 0.0, dec: PI / 2.0 };
		equ2 = Equatorial { ra : -PI, dec: PI / 2.0 };
		assert_close(equ1.planar_distance(equ2), 0.0);
	}

	#[test]
	fn test_planar_distance_dec ( )
	{
		let mut equ1 = Equatorial { ra : 0.0, dec: PI/2.0 };
		let mut equ2 = Equatorial { ra : 0.0, dec: PI/2.0 };
		assert_eq!(equ1.planar_distance(equ2), 0.0);
		
		equ1 = Equatorial { ra : 0.0, dec: -PI / 2.0 };
		equ2 = Equatorial { ra : 0.0, dec: PI / 2.0 };
		assert_eq!(equ1.planar_distance(equ2), 2.0);
	}


	//
	//  to_cartesian3 ( &self ) -> Cartesian3D<Decimal>
	//

	#[test]
	fn test_to_cartesian3 ( )
	{
		let equ = Equatorial { ra : PI / 4.0, dec: PI / 4.0 };
		let cart = equ.to_cartesian3();
		assert_close(cart.x, 0.5);
		assert_close(cart.y, 0.5);
		assert_close(cart.z, -0.7071067812);
	}
	
	#[test]
	fn test_to_cartesian3_x ( )
	{
		let mut equ = Equatorial { ra : 0.0, dec: 0.0 };
		let mut cart = equ.to_cartesian3();
		assert_close(cart.x, 1.0);
		assert_close(cart.y, 0.0);
		assert_close(cart.z, 0.0);

		equ = Equatorial { ra : PI, dec: 0.0 };
		cart = equ.to_cartesian3();
		assert_close(cart.x, -1.0);
		assert_close(cart.y, 0.0);
		assert_close(cart.z, 0.0);
		
		equ = Equatorial { ra : -PI, dec: 0.0 };
		cart = equ.to_cartesian3();
		assert_close(cart.x, -1.0);
		assert_close(cart.y, 0.0);
		assert_close(cart.z, 0.0);
	}

	#[test]
	fn test_to_cartesian3_y ( )
	{
		let mut equ = Equatorial { ra : PI / 2.0, dec: 0.0 };
		let mut cart = equ.to_cartesian3();
		assert_close(cart.x, 0.0);
		assert_close(cart.y, 1.0);
		assert_close(cart.z, 0.0);

		equ = Equatorial { ra : -PI / 2.0, dec: 0.0 };
		cart = equ.to_cartesian3();
		assert_close(cart.x, 0.0);
		assert_close(cart.y, -1.0);
		assert_close(cart.z, 0.0);
	}

	#[test]
	fn test_to_cartesian3_z ( )
	{
		let mut equ = Equatorial { ra : 0.0, dec: PI / 2.0 };
		let mut cart = equ.to_cartesian3();
		assert_close(cart.x, 0.0);
		assert_close(cart.y, 0.0);
		assert_close(cart.z, -1.0);

		equ = Equatorial { ra : 0.0, dec: -PI / 2.0 };
		cart = equ.to_cartesian3();
		assert_close(cart.x, 0.0);
		assert_close(cart.y, 0.0);
		assert_close(cart.z, 1.0);
	}
	
	
	
	
	//
	//  from_cartesian3 ( &Cartesian3D<Decimal> ) -> Equatorial
	//
	#[test]
	fn test_from_cartesian3 ( )
	{
		let c : Cartesian3D<Decimal> = Cartesian3D { x: 0.5, y: 0.5, z: -0.7071067812 };
		let e = Equatorial::from_cartesian3(c);
		assert_close(e.ra, PI / 4.0);
		assert_close(e.dec, PI / 4.0);
	}
	
	
	
	
	
	
	
	
	#[test]
	fn test_partial_eq ( )
	{
		let equ1 = Equatorial { ra : 1.23, dec: 3.45 };
		let mut equ2 = Equatorial { ra : 1.23, dec: 3.45 };
		assert!(equ1.eq(&equ2));
		assert!(!equ1.ne(&equ2));
		
		equ2.ra = 0.0;
		assert!(!equ1.eq(&equ2));
		assert!(equ1.ne(&equ2));
		
		equ2.ra = 1.23;
		equ2.dec = 0.0;
		assert!(!equ1.eq(&equ2));
		assert!(equ1.ne(&equ2));
		
	}
}
