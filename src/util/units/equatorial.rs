//! The implementation of Equatorial.
use std::fmt;
use std::ops::RangeInclusive;
use crate::util::aliases::{Decimal, M_PI};
use super::{Equatorial, Vector3, Radians};

impl Equatorial
{
	/// Returns an equatorial coordinate at ra: 0, dec: 0 (on the equator at 0 hours).
	/// Useful for initialization or testing.
	///
	/// # Returns
	/// Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
	///
	/// # Example
	/// ```
	/// use star_tracker::util::units::{Equatorial, Radians};
	/// assert_eq!(Equatorial{ra: Radians(0.0), dec: Radians(0.0)}, Equatorial::zero());
	/// ```
	pub const fn zero ( ) -> Equatorial
	{
		return Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
	}


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


	/// USE Vector3.angular_distance IF YOU HAVE A CARTESIAN, CONVERTING TO EQUATORIAL HAS A SINGULARITY!!!
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
		let cur = self.to_vector3();
		let oth = other.to_vector3();
		return cur.angle_distance(oth);
		//
		// let dot = cur.dot(oth);
		// let mag_cur = (cur.x.powf(2.0) + cur.y.powf(2.0) + cur.z.powf(2.0)).sqrt();
		// let mag_oth = (oth.x.powf(2.0) + oth.y.powf(2.0) + oth.z.powf(2.0)).sqrt();
		//
		// return Radians((dot / (mag_cur * mag_oth)).acos());
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
		let cur = self.to_vector3();
		let oth = other.to_vector3();
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



	/// Sets the points of the input equatorial array as a set of evenly spaced points.
	/// This uses the fibinachi golden ratio algorithm.
	///
	/// # Arguments
	/// * `num_points` - The number of points on the sphere.
	/// # Example
	/// ```
	/// use star_tracker::util::aliases::M_PI;
	/// use star_tracker::util::units::{Radians, Equatorial};
	/// use star_tracker::util::aliases::Decimal;
	/// let points = Equatorial::evenly_distribute_points(Radians(10.0));
	/// let mut eq : Vec<Equatorial> = Equatorial::evenly_distribute(points);
	/// ```
	pub fn evenly_distribute ( num_points : usize ) -> Vec<Equatorial>
	{
		let mut output : Vec<Equatorial> = Vec::with_capacity(num_points);
		let golden_ratio = (1.0 + (5.0 as Decimal).powf(0.5)) / 2.0;

		for i in 0..num_points
		{
			let mut theta = 2.0 * M_PI * (i as Decimal) / golden_ratio;
			let phi = (1.0 - 2.0 * (i as Decimal + 0.5) / num_points as Decimal).acos();

			theta = theta % (Equatorial::range_ra().end().0);

			let mut val = Equatorial{ra: Radians(theta), dec: Radians(0.0)};
			val.set_phi(Radians(phi));
			output.push(val);
		}
		return output
	}

	/// Returns the separation between neighboring points of the `evenly_distribute` function.
	/// This is not perfectly accurate, give some tolerance when using this.
	/// # Arguments
	/// * `number_of_points` - The number of points to insert on the sphere.
	/// # Asserts
	/// The `number_of_points` must be above 100 points as bellow this is less accurate.
	pub fn evenly_distribute_angle ( number_of_points: usize ) -> Radians
	{
		assert!(99 < number_of_points, "This model is inaccurate below 100 points.");
		return Radians(3.3255 * (number_of_points as Decimal).powf(-0.4974));
	}

	/// Returns the number of points required to get a particular angle using `evenly_distribute`.
	/// This is not perfectly accurate, give some tolerance when using this.
	/// # Arguments
	/// * `angle_distance` - The distance each neighbor needs to be apart.
	pub fn evenly_distribute_points ( angle_distance: Radians ) -> usize
	{
		return (11.2019 * angle_distance.0.powf(-2.0104)) as usize;
	}


	pub fn icosohedron ( ) -> Vec<Equatorial>
	{
		let mut points : Vec<Equatorial> = Vec::new();
		let phi = (1.0 + (5.0 as Decimal).powf(0.5)) / 2.0; // Golden Ratio

		// points:
		// ( 0,  ±1, ±φ )
		// ( ±1, ±φ, 0  )
		// ( ±φ, 0,  ±1 )

		points.push(Vector3{x:  phi, y:  0.0, z:  1.0}.to_equatorial());
		points.push(Vector3{x:  phi, y:  0.0, z: -1.0}.to_equatorial());
		points.push(Vector3{x: -phi, y:  0.0, z:  1.0}.to_equatorial());
		points.push(Vector3{x: -phi, y:  0.0, z: -1.0}.to_equatorial());

		points.push(Vector3{x:  0.0, y:  1.0, z:  phi}.to_equatorial());
		points.push(Vector3{x:  0.0, y:  1.0, z: -phi}.to_equatorial());
		points.push(Vector3{x:  0.0, y: -1.0, z:  phi}.to_equatorial());
		points.push(Vector3{x:  0.0, y: -1.0, z: -phi}.to_equatorial());

		points.push(Vector3{x:  1.0, y:  phi, z: 0.0}.to_equatorial());
		points.push(Vector3{x:  1.0, y: -phi, z: 0.0}.to_equatorial());
		points.push(Vector3{x: -1.0, y:  phi, z: 0.0}.to_equatorial());
		points.push(Vector3{x: -1.0, y: -phi, z: 0.0}.to_equatorial());

		return points;
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

	use rand::prelude::*;

	use util::units::{Equatorial, Vector3, Radians, Degrees};
	use util::aliases::Decimal;
	use util::aliases::M_PI;
	use util::test::TestEqual;


	fn assert_close ( a: Decimal, b: Decimal ) -> bool
	{
		return (a - b).abs() < 0.00001;
	}


	//
	// zero ( ) -> Equatorial{ra: Radians(0.0), dec: Radians(0.0)}
	//

	#[test]
	fn test_zero ( )
	{
		assert_eq!(Equatorial{ra: Radians(0.0), dec: Radians(0.0)}, Equatorial::zero());
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
	//  to_vector3 ( &self ) -> Vector3
	//

	#[test]
	fn test_to_vector3 ( )
	{
		let equ = Equatorial { ra : Radians(M_PI / 4.0), dec: Radians(M_PI / 4.0) };
		let cart = equ.to_vector3();
		assert_close(cart.x, 0.5);
		assert_close(cart.y, 0.5);
		assert_close(cart.z, -0.7071067812);
	}

	#[test]
	fn test_to_vector3_x ( )
	{
		let mut equ = Equatorial { ra : Radians(0.0), dec: Radians(0.0) };
		let mut cart = equ.to_vector3();
		assert_close(cart.x, 1.0);
		assert_close(cart.y, 0.0);
		assert_close(cart.z, 0.0);

		equ = Equatorial { ra : Radians(M_PI), dec: Radians(0.0) };
		cart = equ.to_vector3();
		assert_close(cart.x, -1.0);
		assert_close(cart.y, 0.0);
		assert_close(cart.z, 0.0);

		equ = Equatorial { ra : Radians(-M_PI), dec: Radians(0.0) };
		cart = equ.to_vector3();
		assert_close(cart.x, -1.0);
		assert_close(cart.y, 0.0);
		assert_close(cart.z, 0.0);
	}

	#[test]
	fn test_to_vector3_y ( )
	{
		let mut equ = Equatorial { ra : Radians(M_PI / 2.0), dec: Radians(0.0) };
		let mut cart = equ.to_vector3();
		assert_close(cart.x, 0.0);
		assert_close(cart.y, 1.0);
		assert_close(cart.z, 0.0);

		equ = Equatorial { ra : Radians(-M_PI / 2.0), dec: Radians(0.0) };
		cart = equ.to_vector3();
		assert_close(cart.x, 0.0);
		assert_close(cart.y, -1.0);
		assert_close(cart.z, 0.0);
	}

	#[test]
	fn test_to_vector3_z ( )
	{
		let mut equ = Equatorial { ra : Radians(0.0), dec: Radians(M_PI / 2.0) };
		let mut cart = equ.to_vector3();
		assert_close(cart.x, 0.0);
		assert_close(cart.y, 0.0);
		assert_close(cart.z, -1.0);

		equ = Equatorial { ra : Radians(0.0), dec: Radians(-M_PI / 2.0) };
		cart = equ.to_vector3();
		assert_close(cart.x, 0.0);
		assert_close(cart.y, 0.0);
		assert_close(cart.z, 1.0);
	}


	#[test]
	fn test_to_cartesian_3_random ( )
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
			c.normalize();
			let e = c.to_equatorial();

			assert!(e.test_close(&c.to_equatorial(), 0.0001));
		}
	}









	//
	// fn evenly_distribute ( &mut [Equatorial] )
	//

	#[test]
	fn test_evenly_distribute_not_enough_elements ( )
	{	// Should not panic!
		let n = 0;
		let _eq : Vec<Equatorial> = Equatorial::evenly_distribute(n);
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
		let n : usize = 1000;
		let eq : Vec<Equatorial> = Equatorial::evenly_distribute(n);

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
		let n : usize = 100;
		let variance = 0.1;
		let eq : Vec<Equatorial> = Equatorial::evenly_distribute(n);

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


	#[test]
	fn test_evenly_distribute_angle ( )
	{
		let method = Equatorial::evenly_distribute_angle;
		assert!((method(100)   - Degrees(19.284).to_radians()).abs() < 1.0 );
		assert!((method(125)   - Degrees(17.164).to_radians()).abs() < 1.0 );
		assert!((method(250)   - Degrees(12.276).to_radians()).abs() < 0.1 );
		assert!((method(500)   - Degrees( 8.734).to_radians()).abs() < 0.1 );
		assert!((method(1000)  - Degrees( 6.111).to_radians()).abs() < 0.1 );
		assert!((method(2000)  - Degrees( 4.343).to_radians()).abs() < 0.1 );
		assert!((method(4000)  - Degrees( 3.108).to_radians()).abs() < 0.1 );
		assert!((method(8000)  - Degrees( 2.18 ).to_radians()).abs() < 0.1 );
		assert!((method(10000) - Degrees( 1.971).to_radians()).abs() < 0.1 );
	}

	#[test]
	#[should_panic]
	fn test_evenly_distribute_angle_panic ( )
	{
		Equatorial::evenly_distribute_angle(99);
	}


	#[test]
	fn test_evenly_distribute_angle_points ( )
	{
		let method = Equatorial::evenly_distribute_points;
		assert!(method(Degrees(19.284).to_radians()).saturating_sub(100) < 15);
		assert!(method(Degrees(17.164).to_radians()).saturating_sub(125) < 15);
		assert!(method(Degrees(12.276).to_radians()).saturating_sub(250) < 15);
		assert!(method(Degrees( 8.734).to_radians()).saturating_sub(500) < 15);
		assert!(method(Degrees( 6.111).to_radians()).saturating_sub(1000) < 15);
		assert!(method(Degrees( 4.343).to_radians()).saturating_sub(2000) < 15);
		assert!(method(Degrees( 3.108).to_radians()).saturating_sub(4000) < 15);
		assert!(method(Degrees( 2.180).to_radians()).saturating_sub(8000) < 15);
		assert!(method(Degrees( 1.971).to_radians()).saturating_sub(10000) < 15);
	}
	// Numbers used for golden circle calibration.
	// 1.533 1.039 0.666 0.471 0.336 0.3 0.214 0.1524 0.107 0.076 0.054 0.038 0.034
	// 5 10 25 50 100 125 250 500 1000 2000 4000 8000 10000

}
