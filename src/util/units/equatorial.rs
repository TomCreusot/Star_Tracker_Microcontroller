//! The implementation of Equatorial.
use std::ops::RangeInclusive;
use crate::util::aliases::{Decimal, M_PI};
use super::{Equatorial, Cartesian3D, Radians};

impl Equatorial
{
	/// The range for declination.
	pub fn range_dec ( ) -> RangeInclusive<Radians>
	{
		return Radians(-M_PI / 2.0) ..= Radians(M_PI / 2.0);
	}
	
	/// The range for right ascention
	pub fn range_ra ( ) -> RangeInclusive<Radians>
	{
		return Radians(0.0) ..= Radians(2.0 * M_PI);
	}
	
	/// Returns dec + 180 degrees to the declination to match with spherical coordinates.
	///	# Example
	/// ```
	/// use star_tracker::util::units::{Equatorial, Radians};
	/// use star_tracker::util::aliases::M_PI;
	/// let equ = Equatorial { ra : Radians(0.0), dec: Radians(-M_PI / 2.0) };
	/// assert_eq!( equ.get_phi(), Radians(0.0) );
	/// ```
	pub fn get_phi ( &self ) -> Radians
	{
		return self.dec + Radians(M_PI / 2.0);
	}

	/// Sets phi (start at z = +1).
	///	# Example
	/// ```
	/// use star_tracker::util::units::{Equatorial, Radians};
	/// use star_tracker::util::aliases::M_PI;
	/// let mut equ = Equatorial { ra : Radians(0.0), dec: Radians(0.0) };
	/// equ.set_phi( Radians(M_PI / 4.0) );
	/// assert_eq!( equ.dec, Radians(-M_PI / 4.0) );
	/// ```
	pub fn set_phi ( &mut self, angle: Radians )
	{
		self.dec = angle - Radians(M_PI / 2.0);
	}


	/// USE Cartesian3D.angular_distance IF YOU HAVE A CARTESIAN, CONVERTING TO EQUATORIAL HAS A SINGULARITY!!!
	/// Finds the angle between the 2 points on a sphere.
	/// # Example
	/// ```
	/// use star_tracker::util::units::{Equatorial, Radians};
	/// use star_tracker::util::aliases::M_PI;
	/// let mut equ1 = Equatorial { ra : Radians(M_PI), dec: Radians(M_PI / 2.0) };
	/// let mut equ2 = Equatorial { ra : Radians(-M_PI), dec: Radians(-M_PI / 2.0) }; // 180 degrees because of dec
	/// assert_eq!(equ1.angle_distance(equ2), Radians(M_PI));
	///
	/// equ1 = Equatorial { ra : Radians(0.0), dec: Radians(M_PI / 4.0) };
	/// equ2 = Equatorial { ra : Radians(M_PI), dec: Radians(M_PI / 4.0) };
	/// assert_eq!(equ1.angle_distance(equ2), Radians(M_PI/2.0));
	/// ```
	pub fn angle_distance ( &self, other: Equatorial ) -> Radians
	{
		let cur = self.to_cartesian3();
		let oth = other.to_cartesian3();
	
		let dot = cur.dot(&oth);
		let mag_cur = (cur.x.powf(2.0) + cur.y.powf(2.0) + cur.z.powf(2.0)).sqrt();
		let mag_oth = (oth.x.powf(2.0) + oth.y.powf(2.0) + oth.z.powf(2.0)).sqrt();
	
		return Radians((dot / (mag_cur * mag_oth)).acos());
	}

	/// Finds the distance between the 2 points on a unit sphere.
	/// # Example
	/// ```
	/// use star_tracker::util::units::{Radians, Equatorial};
	/// use star_tracker::util::aliases::M_PI;
	/// let mut equ1 = Equatorial { ra : Radians(M_PI), dec: Radians(M_PI / 2.0) };
	/// let mut equ2 = Equatorial { ra : Radians(-M_PI), dec: Radians(-M_PI / 2.0) }; // 180 degrees because of dec
	/// assert_eq!(equ1.planar_distance(equ2), 2.0);
	///
	/// equ1 = Equatorial { ra : Radians(0.0), dec: Radians(M_PI / 4.0) };
	///	equ2 = Equatorial { ra : Radians(M_PI), dec: Radians(M_PI / 4.0) };
	///	assert_eq!(equ1.planar_distance(equ2), 1.4142135623730951);
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
	/// use star_tracker::util::aliases::M_PI;
	/// use star_tracker::util::units::{Radians, Equatorial};
	/// let equ = Equatorial { ra : Radians(M_PI / 4.0), dec: Radians(M_PI / 4.0) };
	/// let cart = equ.to_cartesian3();
	/// assert!((cart.x - 0.5).abs() < 0.0001);
	/// assert!((cart.y - 0.5).abs() < 0.0001);
	/// assert!((cart.z - -0.7071067812).abs() < 0.0001);
	/// ```
	pub fn to_cartesian3 ( &self ) -> Cartesian3D
	{
		return Cartesian3D {
			x: self.ra.cos() * self.get_phi().sin(),
			y: self.ra.sin() * self.get_phi().sin(),
			z: self.get_phi().cos()
		};
	}
	
	
	
	/// Sets the points of the input equatorial array as a set of evenly spaced points.
	/// This uses the fibinachi golden ratio algorithm.
	///
	/// # Arguments
	/// * `points` - The array to apply points to.
	/// # Example
	pub fn evenly_distribute ( points : &mut [Equatorial] )
	{
		let golden_ratio = (1.0 + (5.0 as Decimal).powf(0.5)) / 2.0;

		for i in 0..points.len()
		{
			let mut theta = 2.0 * M_PI * (i as Decimal) / golden_ratio;
			let phi = (1.0 - 2.0 * (i as Decimal + 0.5) / points.len() as Decimal).acos();

			theta = theta % (Equatorial::range_ra().end().0);

			let mut val = Equatorial{ra: Radians(theta), dec: Radians(0.0)};
			val.set_phi(Radians(phi));
			points[i] = val;
		}

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
//###############################################################################################//
//										---	Equatorial ---
//###############################################################################################//

	use util::units::{Equatorial, Radians};
	use util::aliases::Decimal;
	use util::aliases::M_PI;


	fn assert_close ( a: Decimal, b: Decimal ) -> bool
	{
		return (a - b).abs() < 0.00001;
	}

	//
	// Getters
	//

	#[test]
	fn test_range_dec_ra ( )
	{
		assert_eq!(Equatorial::range_ra(),  Radians(0.0)   ..= Radians(M_PI * 2.0));
		assert_eq!(Equatorial::range_dec(), Radians(-M_PI / 2.0) ..= Radians(M_PI) / 2.0);
	}

	
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

	#[test]
	fn test_set_phi ( )
	{
		let mut equ = Equatorial { ra : Radians(0.0), dec: Radians(0.0) };
		equ.set_phi( Radians(M_PI / 4.0) );
		assert_eq!( equ.dec.0, -M_PI / 4.0 );
	}



	//
	//  testangle_distance ( Equatorial ) -> Decimal
	//

	#[test]
	fn test_angle_distance ( )
	{
		let mut equ1 = Equatorial { ra : Radians(M_PI), dec: Radians(M_PI / 2.0) };
		let mut equ2 = Equatorial { ra : Radians(-M_PI), dec: Radians(-M_PI / 2.0) }; // 180 degrees because of dec
		assert_eq!(equ1.angle_distance(equ2).0, M_PI);

		equ1 = Equatorial { ra : Radians(0.0), dec: Radians(M_PI / 4.0) };
		equ2 = Equatorial { ra : Radians(M_PI),  dec: Radians(M_PI / 4.0) };
		assert!((equ1.angle_distance(equ2).0 - M_PI/2.0).abs() < 0.0000001);
	}

	#[test]
	fn test_angle_distance_latitude ( )
	{
		let mut equ1 = Equatorial { ra : Radians(0.0), dec: Radians(M_PI / 2.0) };
		let mut equ2 = Equatorial { ra : Radians(0.0), dec: Radians(-M_PI / 2.0) };
		assert_eq!(equ1.angle_distance(equ2).0, M_PI);

		equ1 = Equatorial { ra : Radians(0.0), dec: Radians(-M_PI * 2.0) };
		equ2 = Equatorial { ra : Radians(0.0), dec: Radians(M_PI * 2.0) };
		assert_eq!(equ1.angle_distance(equ2).0, 0.0);
	}

	#[test]
	fn test_angle_distance_longitude ( )
	{
		let equ1 = Equatorial { ra : Radians(M_PI / 2.0), dec: Radians(0.0) };
		let equ2 = Equatorial { ra : Radians(-M_PI / 2.0), dec: Radians(0.0) };
		assert_eq!(equ1.angle_distance(equ2).0, M_PI);
	}

	#[test]
	fn test_angle_distance_90_dec ( )
	{
		let equ1 = Equatorial { ra : Radians(M_PI), dec: Radians(M_PI / 2.0) };
		let equ2 = Equatorial { ra : Radians(-M_PI), dec: Radians(0.0) };
		assert_eq!(equ1.angle_distance(equ2).0, M_PI / 2.0);
	}




	//
	//  planar_distance ( Equatorial ) -> Decimal
	//

	#[test]
	fn test_planar_distance ( )
	{
		let mut equ1 = Equatorial { ra : Radians(M_PI), dec: Radians(M_PI / 2.0) };
		let mut equ2 = Equatorial { ra : Radians(-M_PI), dec: Radians(-M_PI / 2.0) }; // 180 degrees because of dec
		assert_close(equ1.planar_distance(equ2), 2.0);

		equ1 = Equatorial { ra : Radians(0.0), dec: Radians(M_PI / 4.0) };
		equ2 = Equatorial { ra : Radians(M_PI), dec: Radians(M_PI / 4.0) };
		assert_close(equ1.planar_distance(equ2), 1.4142135);
	}


	#[test]
	fn test_planar_distance_ra ( )
	{
		let mut equ1 = Equatorial { ra : Radians(M_PI), dec: Radians(0.0) };
		let mut equ2 = Equatorial { ra : Radians(-M_PI), dec: Radians(0.0) };
		assert_close(equ1.planar_distance(equ2), 0.0);

		equ1 = Equatorial { ra : Radians(0.0), dec: Radians(0.0) };
		equ2 = Equatorial { ra : Radians(-M_PI), dec: Radians(0.0) };
		assert_close(equ1.planar_distance(equ2), 2.0);

		equ1 = Equatorial { ra : Radians(0.0), dec: Radians(M_PI / 2.0) };
		equ2 = Equatorial { ra : Radians(-M_PI), dec: Radians(M_PI / 2.0) };
		assert_close(equ1.planar_distance(equ2), 0.0);
	}

	#[test]
	fn test_planar_distance_dec ( )
	{
		let mut equ1 = Equatorial { ra : Radians(0.0), dec: Radians(M_PI/2.0) };
		let mut equ2 = Equatorial { ra : Radians(0.0), dec: Radians(M_PI/2.0) };
		assert_eq!(equ1.planar_distance(equ2), 0.0);

		equ1 = Equatorial { ra : Radians(0.0), dec: Radians(-M_PI / 2.0) };
		equ2 = Equatorial { ra : Radians(0.0), dec: Radians(M_PI / 2.0) };
		assert_eq!(equ1.planar_distance(equ2), 2.0);
	}


	//
	//  to_cartesian3 ( &self ) -> Cartesian3D
	//

	#[test]
	fn test_to_cartesian3 ( )
	{
		let equ = Equatorial { ra : Radians(M_PI / 4.0), dec: Radians(M_PI / 4.0) };
		let cart = equ.to_cartesian3();
		assert_close(cart.x, 0.5);
		assert_close(cart.y, 0.5);
		assert_close(cart.z, -0.7071067812);
	}

	#[test]
	fn test_to_cartesian3_x ( )
	{
		let mut equ = Equatorial { ra : Radians(0.0), dec: Radians(0.0) };
		let mut cart = equ.to_cartesian3();
		assert_close(cart.x, 1.0);
		assert_close(cart.y, 0.0);
		assert_close(cart.z, 0.0);

		equ = Equatorial { ra : Radians(M_PI), dec: Radians(0.0) };
		cart = equ.to_cartesian3();
		assert_close(cart.x, -1.0);
		assert_close(cart.y, 0.0);
		assert_close(cart.z, 0.0);

		equ = Equatorial { ra : Radians(-M_PI), dec: Radians(0.0) };
		cart = equ.to_cartesian3();
		assert_close(cart.x, -1.0);
		assert_close(cart.y, 0.0);
		assert_close(cart.z, 0.0);
	}

	#[test]
	fn test_to_cartesian3_y ( )
	{
		let mut equ = Equatorial { ra : Radians(M_PI / 2.0), dec: Radians(0.0) };
		let mut cart = equ.to_cartesian3();
		assert_close(cart.x, 0.0);
		assert_close(cart.y, 1.0);
		assert_close(cart.z, 0.0);

		equ = Equatorial { ra : Radians(-M_PI / 2.0), dec: Radians(0.0) };
		cart = equ.to_cartesian3();
		assert_close(cart.x, 0.0);
		assert_close(cart.y, -1.0);
		assert_close(cart.z, 0.0);
	}

	#[test]
	fn test_to_cartesian3_z ( )
	{
		let mut equ = Equatorial { ra : Radians(0.0), dec: Radians(M_PI / 2.0) };
		let mut cart = equ.to_cartesian3();
		assert_close(cart.x, 0.0);
		assert_close(cart.y, 0.0);
		assert_close(cart.z, -1.0);

		equ = Equatorial { ra : Radians(0.0), dec: Radians(-M_PI / 2.0) };
		cart = equ.to_cartesian3();
		assert_close(cart.x, 0.0);
		assert_close(cart.y, 0.0);
		assert_close(cart.z, 1.0);
	}
	
	
	

	
	
	
	
	
	//
	// fn evenly_distribute ( &mut [Equatorial] )
	//

	#[test]
	fn test_evenly_distribute_not_enough_elements ( )
	{	// Should not panic!
		const N : usize = 0;
		let mut eq : [Equatorial; N] = [Equatorial{ra: Radians(0.0), dec: Radians(0.0)}; N];
		Equatorial::evenly_distribute(&mut eq);
	}

	#[test]
	fn test_evenly_distribute_within_range ( )
	{	// may have different angle coordinates.
		let mut e = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		e.set_phi(Radians(0.0));
		println!("phi 0: {}", e.dec);
		e.set_phi(Radians(M_PI / 4.0));
		println!("phi 45: {}", e.dec);
		e.set_phi(Radians(M_PI / 2.0));
		println!("phi 90: {}", e.dec);
		e.set_phi(Radians(M_PI));
		
		println!("phi 180: {}", e.dec);
		const N : usize = 1000;
		let mut eq : [Equatorial; N] = [Equatorial{ra: Radians(0.0), dec: Radians(0.0)}; N];
		Equatorial::evenly_distribute(&mut eq);

		let range_ra = Equatorial::range_ra();
		let range_dec = Equatorial::range_dec();
		let mut close_max_ra = false;
		let mut close_max_dec = false;
		let mut close_min_ra = false;
		let mut close_min_dec = false;
		for e in eq.iter()
		{
			if !(range_ra.start().0 <= e.ra.0 && e.ra.0 <= range_ra.end().0)
			|| !(range_dec.start().0 <= e.dec.0 && e.dec.0 <= range_dec.end().0)
			{
				panic!("Out of range: ({:?} to {:?}, {:?} to {:?}), was: ({}, {})",
				range_ra.start().0, range_ra.end().0,
				range_dec.start().0, range_dec.end().0,
				e.ra.0, e.dec.0);
			}
			close_min_ra  |= e.ra.0 < range_ra.start().0 + 1.5;
			close_min_dec |= e.dec.0 < range_dec.start().0 + 1.5;
			close_max_ra  |= range_ra.end().0 - 0.5 < e.ra.0;
			close_max_dec |= range_dec.end().0 - 0.5 < e.dec.0;
		}
		assert!(close_min_ra);
		assert!(close_min_dec);
		assert!(close_max_ra);
		assert!(close_max_dec);
	}

	#[test]
	fn test_evenly_distribute_evenly_distributed ( )
	{	// Each element should not vary in distance by a small number
		const N : usize = 100;
		let variance = 0.1;
		let mut eq : [Equatorial; N] = [Equatorial{ra: Radians(0.0), dec: Radians(0.0)}; N];
		Equatorial::evenly_distribute(&mut eq);

		let mut compare : Option<Decimal> = None;
		for e in eq.iter()
		{
			let mut current = 0.0;
			for ee in eq.iter()
			{
				current += ee.angle_distance(*e).0;
			}

			if compare == None
			{
				compare = Some(current);
			}
			else if variance < (current - compare.unwrap()).abs()
			{
				panic!("Variance too high: {}", current - compare.unwrap());
			}
		}
	}
}
