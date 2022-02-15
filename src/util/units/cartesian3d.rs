//! The implementation of Cartesian3D.
use crate::util::aliases::Decimal;
use crate::util::aliases::M_PI;
use super::{Equatorial, Cartesian3D, Radians, Matrix, MatPos};

impl Cartesian3D
{
	/// Finds the magnitude of the vector.
	/// # Example
	/// ```
	/// use star_tracker::util::units::Cartesian3D;
	/// use star_tracker::util::test::TestEqual;
	/// let c = Cartesian3D{x: 10.3, y: 23.1, z: 12.3};
	/// assert!(c.magnitude().test_close(&28.124544440, 0.00001));
	/// ```
	pub fn magnitude ( &self ) -> Decimal
	{
		assert!(self.x != 0.0 || self.y != 0.0 || self.z != 0.0, 
			"Cannot find the magnitude of a zero vector.");
		return (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt();
	}
	
	
	/// Normalizes the vector so the magnitude is 1.
	/// # Example
	/// ```
	/// use star_tracker::util::units::Cartesian3D;
	/// use star_tracker::util::test::TestEqual;
	/// let mut c = Cartesian3D{x: 10.3, y: 23.1, z: 12.3};
	/// let c_out = Cartesian3D{x: 0.366228, y: 0.8213466, z: 0.43734};
	/// c.normalize();
	/// assert!(c.test_close(&c_out, 0.0001));
	/// ```
	pub fn normalize ( &mut self )
	{
		let magnitude  = self.magnitude();
		self.x /= magnitude;
		self.y /= magnitude;
		self.z /= magnitude;
	}
	
	
	/// Finds the cross product between self and the input object.
	/// # Arguments
	/// * `other` - The other cartesian3d.
	///
	/// # Returns
	/// The cross product.
	///
	/// # Example
	/// ```
	/// use star_tracker::util::units::Cartesian3D;
	/// let a = Cartesian3D { x: -1.0, y: 2.0, z: 10.0 };
	/// let b = Cartesian3D { x: 9.0, y: 3.0, z: -4.0 };
	/// assert_eq!(a.cross(&b), Cartesian3D{x: -38.0, y: 86.0, z: -21.0});
	/// assert_eq!(b.cross(&a), Cartesian3D{x: 38.0, y: -86.0, z: 21.0});
	/// ```
	pub fn cross ( &self, other: &Cartesian3D ) -> Cartesian3D
	{
		return Cartesian3D
		{
			x: self.y * other.z - self.z * other.y,
			y: self.z * other.x - self.x * other.z,
			z: self.x * other.y - self.y * other.x
		};
	}

	/// Finds the dot product between the cartesian3D points.
	/// # Arguments
	/// * `other` - The other cartesian3D.
	///
	/// # Returns
	/// The dot product.
	///
	/// # Example
	/// ```
	/// use star_tracker::util::units::Cartesian3D;
	/// let a = Cartesian3D { x: 2.0, y: 3.0, z: 4.0 };
	/// let b = Cartesian3D { x: 5.0, y: 6.0, z: 7.0 };
	/// assert_eq!(a.dot(&b), 56.0);
	/// ```
	pub fn dot ( &self, other: &Cartesian3D ) -> Decimal
	{	return self.x * other.x + self.y * other.y + self.z * other.z;			}




	/// Finds the angle between the 2 points on a sphere.
	/// # Example
	/// ```
	/// use star_tracker::util::units::{Cartesian3D, Degrees};
	/// let mut car1 = Cartesian3D { x: 0.0, y: 1.0, z: 0.0 };
	/// let mut car2 = Cartesian3D { x: 1.0, y: 0.0, z: 0.0 };
	/// assert_eq!(car1.angle_distance(car2), Degrees(90.0).to_radians());
	/// ```
	pub fn angle_distance ( &self, oth: Cartesian3D ) -> Radians
	{
		let dot = self.dot(&oth);
		
		let mag_cur = self.magnitude();
		let mag_oth = oth.magnitude();
		
		assert!(0.0 < (mag_cur * mag_oth).abs());
		let mut cos = dot / (mag_cur * mag_oth);
		if 1.0 < cos  // floating point errors can get slightly above.
		{
			cos = 1.0;
		}
		
		return Radians(cos.acos());
	}



	/// Converts the coordinates into a matrix form [x, y, z].
	/// # Example
	/// ```
	/// use star_tracker::util::units::Cartesian3D;
	/// use star_tracker::util::units::Matrix;
	/// use star_tracker::util::units::MatPos;
	/// let c = Cartesian3D {x: 1.0, y: 2.0, z: 3.0};
	/// let m : Matrix<1,3> = c.to_matrix();
	/// assert_eq!(c.x, m.get(MatPos{row: 0, col: 0}));
	/// assert_eq!(c.y, m.get(MatPos{row: 0, col: 1}));
	/// assert_eq!(c.z, m.get(MatPos{row: 0, col: 2}));
	/// ```
	pub fn to_matrix ( &self ) -> Matrix<1,3>
	{
		let mut mat : Matrix<1,3> = Matrix::new();
		mat.set(MatPos{row: 0, col: 0}, self.x);
		mat.set(MatPos{row: 0, col: 1}, self.y);
		mat.set(MatPos{row: 0, col: 2}, self.z);
		return mat;
	}



	/// Converts cartesian3D to equatorial coordinates.
	/// # Arguments
	/// * `c` - The cartesian coordinates in Decimal format as a unit vector.
	/// # Returns
	/// The equatorial equivalent of the unit vector.
	///
	/// # Example
	/// ```
	/// use star_tracker::util::aliases::M_PI;
	/// use star_tracker::util::units::{Equatorial, Cartesian3D, Radians};
	/// use star_tracker::util::aliases::Decimal;
	/// use star_tracker::util::test::TestEqual;
	/// let mut c = Cartesian3D { x: 0.5, y: 0.5, z: -0.7071067812 };
	/// let mut e = c.to_equatorial();
	/// let mut compare = Equatorial{ra: Radians(M_PI / 4.0), dec: Radians(-M_PI / 4.0)};
	/// assert!(compare.test_close(&e, 0.0000001));
	///
	/// c = Cartesian3D { x: 3.0, y: 4.0, z: 5.0 };
	/// e = c.to_equatorial();
	/// compare = Equatorial{ra: Radians(0.927295218), dec: Radians(0.78539816)};
	/// assert!(compare.test_close(&e, 0.0000001));
	/// ```
	pub fn to_equatorial ( &self ) -> Equatorial
	{
		// ra = atan(y/x)
		// dec = acos(z / sqrt(x^2 + y ^2 + z^2)) = atan(sqrt(x^2 + y^2) / z)
		let mut ra = 0.0;
		if self.x != 0.0
		{
			ra = (self.y).atan2(self.x);
			ra = (ra + M_PI * 2.0) % (M_PI * 2.0);
		}

		let phi =(self.z / (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()).acos();
		
		let mut eq = Equatorial{ra: Radians(ra), dec: Radians(0.0)};
		eq.set_phi(Radians(phi));
		eq.dec = -eq.dec;
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
	use rand::prelude::*;
	
	use util::units::Cartesian3D;
	use util::units::Matrix;
	use util::units::MatPos;
	use util::units::Equatorial;
	use util::units::Radians;
	use util::units::Degrees;
	use util::units::Decimal;
	use util::aliases::M_PI;
	use util::test::TestEqual;
	
	
//###############################################################################################//
//										---	Equatorial ---
//###############################################################################################//

	//
	// magnitude ( &self ) -> Decimal
	//
	#[test]
	fn test_magnitude ( )
	{
		let c = Cartesian3D{x: 10.3, y: 23.1, z: 12.3};
		assert!(c.magnitude().test_close(&28.1245463, 0.0001));
	}
	
	
	//
	// normalize ( &self )
	//
	#[test]
	fn test_normalize ( )
	{
		let mut c = Cartesian3D{x: 10.3, y: 23.1, z: 12.3};
		let c_out = Cartesian3D{x: 0.366228, y: 0.8213466, z: 0.43734};
		c.normalize();
		assert!(c.test_close(&c_out, 0.00001));
	}

	//
	// cross ( &Cartesian3D ) -> Cartesian3D
	//
	#[test]
	fn test_cross_zero ( )
	{
		let a = Cartesian3D { x: 0.0, y: 0.0, z: 0.0 };
		let b = Cartesian3D { x: 0.0, y: 0.0, z: 0.0 };
		assert_eq!(a.cross(&b), Cartesian3D{x: 0.0, y: 0.0, z: 0.0});
	}

	#[test]
	fn test_cross_single_axis ( )
	{
		let a = Cartesian3D { x: 1.0, y: 0.0, z: 0.0 };
		let b = Cartesian3D { x: 0.0, y: 2.0, z: 0.0 };
		let c = Cartesian3D { x: 0.0, y: 0.0, z: 3.0 };
		assert_eq!(a.cross(&b), Cartesian3D{x: 0.0, y: 0.0, z: 2.0});
		assert_eq!(b.cross(&a), Cartesian3D{x: 0.0, y: 0.0, z: -2.0});
		assert_eq!(a.cross(&c), Cartesian3D{x: 0.0, y: -3.0, z: 0.0});
		assert_eq!(c.cross(&a), Cartesian3D{x: 0.0, y: 3.0, z: 0.0});
		assert_eq!(b.cross(&c), Cartesian3D{x: 6.0, y: 0.0, z: 0.0});
		assert_eq!(c.cross(&b), Cartesian3D{x: -6.0, y: 0.0, z: 0.0});
	}

	#[test]
	fn test_cross_dual_axis ( )
	{
		let a = Cartesian3D { x: 1.0, y: 2.0, z: 0.0 };
		let b = Cartesian3D { x: 0.0, y: 3.0, z: 4.0 };
		let c = Cartesian3D { x: 5.0, y: 0.0, z: 6.0 };
		assert_eq!(a.cross(&b), Cartesian3D{x: 8.0, y: -4.0, z: 3.0});
		assert_eq!(b.cross(&a), Cartesian3D{x: -8.0, y: 4.0, z: -3.0});
		assert_eq!(a.cross(&c), Cartesian3D{x: 12.0, y: -6.0, z: -10.0});
		assert_eq!(c.cross(&a), Cartesian3D{x: -12.0, y: 6.0, z: 10.0});
		assert_eq!(b.cross(&c), Cartesian3D{x: 18.0, y: 20.0, z: -15.0});
		assert_eq!(c.cross(&b), Cartesian3D{x: -18.0, y: -20.0, z: 15.0});
	}

	#[test]
	fn test_cross_tri_axis ( )
	{
		let a = Cartesian3D { x: -1.0, y: 2.0, z: 10.0 };
		let b = Cartesian3D { x: 9.0, y: 3.0, z: -4.0 };
		assert_eq!(a.cross(&b), Cartesian3D{x: -38.0, y: 86.0, z: -21.0});
		assert_eq!(b.cross(&a), Cartesian3D{x: 38.0, y: -86.0, z: 21.0});
	}


	//
	// dot ( &Cartesian3D ) -> Decimal
	//
	#[test]
	fn test_dot ( )
	{
		let a = Cartesian3D { x: 1.0, y: 0.0, z: 0.0 };
		let b = Cartesian3D { x: 0.0, y: 1.0, z: 0.0 };
		let c = Cartesian3D { x: 1.0, y: 1.0, z: 0.0 };
		let d = Cartesian3D { x: 0.0, y: 0.0, z: 1.0 };
		let e = Cartesian3D { x: 1.0, y: 0.0, z: 1.0 };
		let f = Cartesian3D { x: 0.0, y: 1.0, z: 1.0 };
		let g = Cartesian3D { x: 1.0, y: 1.0, z: 1.0 };

		assert_eq!(a.dot(&g), 1.0);
		assert_eq!(a.dot(&b), 0.0);
		assert_eq!(b.dot(&g), 1.0);
		assert_eq!(c.dot(&g), 2.0);
		assert_eq!(d.dot(&g), 1.0);
		assert_eq!(e.dot(&g), 2.0);
		assert_eq!(f.dot(&g), 2.0);
	}









	//
	// angular_distance ( Cartesian3D ) -> Radians
	//

	#[test]
	fn test_angle_distance ( )
	{
		let car1 = Cartesian3D { x: 0.0, y: 1.0, z: 0.0 };
		let mut car2 = Cartesian3D { x: 1.0, y: 0.0, z: 0.0 };
		assert_eq!(car1.angle_distance(car2), Degrees(90.0).to_radians());
		car2.x = 0.0;
		car2.y = 1.0;
		assert_eq!(car1.angle_distance(car2), Degrees(0.0).to_radians());
		car2.x = 0.0;
		car2.y = -1.0;
		assert_eq!(car1.angle_distance(car2), Degrees(180.0).to_radians());
	}










	//
	// to_matrix ( &self ) -> Matrix<1,3>
	//

	#[test]
	fn test_to_matrix ( )
	{
		let c = Cartesian3D {x: 1.0, y: 2.0, z: 3.0};
		let m : Matrix<1,3> = c.to_matrix();
		assert_eq!(c.x, m.get(MatPos{row: 0, col: 0}));
		assert_eq!(c.y, m.get(MatPos{row: 0, col: 1}));
		assert_eq!(c.z, m.get(MatPos{row: 0, col: 2}));
	}
	
	//
	//  to_equatorial ( ) -> Equatorial
	//
	#[test]
	fn test_to_equatorial ( )
	{
		let mut c = Cartesian3D { x: 0.5, y: 0.5, z: -0.7071067812 };
		let mut e = c.to_equatorial();
		let mut compare = Equatorial{ra: Radians(M_PI / 4.0), dec: Radians(-M_PI / 4.0)};
		assert!(e.test_close(&compare, 0.000001));
		c = Cartesian3D { x: 3.0, y: 4.0, z: 5.0 };
		e = c.to_equatorial();
		compare = Equatorial{ra: Radians(0.927295218), dec: Radians(0.78539816)};
		assert!(e.test_close(&compare, 0.000001));
	}

	// #[test]
	// fn test_to_equatorial_z_zero ( )
	// {
	// 	let c = Cartesian3D { x: 0.5, y: 0.5, z: 0.0 };
	// 	let e = c.to_equatorial();
	// 	assert_eq!(e.ra,  Radians(M_PI / 4.0));
	// 	assert_eq!(e.dec, Radians(0.0));
	// }

	#[test]
	fn test_to_equatorial_z_full ( )
	{
		let mut c = Cartesian3D { x: 0.0, y: 0.0, z: 10000.0 };
		let mut e = c.to_equatorial();
		assert_eq!(e.ra,  Radians(0.0));
		assert_eq!(e.dec, Radians(M_PI / 2.0));
		c = Cartesian3D { x: 0.0, y: 0.0, z: -10000.0 };
		 e = c.to_equatorial();
		assert_eq!(e.ra,  Radians(0.0));
		assert_eq!(e.dec, Radians(-M_PI / 2.0));
	}


	#[test]
	fn test_to_equatorial_z_high ( )
	{
		let mut c = Cartesian3D { x: 0.1, y: 0.1, z: 10000.0 };
		let mut e = c.to_equatorial();
		let mut compare = Equatorial{ra: Radians(0.7853), dec: Radians(M_PI / 2.0)};
		assert!(e.test_close(&compare, 0.001));
		c = Cartesian3D { x: 0.1, y: 0.1, z: -10000.0 };
		e = c.to_equatorial();
		compare = Equatorial{ra: Radians(0.7853), dec: -Radians(M_PI / 2.0)};
		assert!(e.test_close(&compare, 0.001));
	}
	
	
	#[test]
	fn test_to_equatorial_random ( )
	{
		// let mut c = Cartesian3D { x: -1.0, y: 0.1, z: -1.0 };
		// for i in 0..20
		// {
		// 	// c.z += 0.1;
		// 	c.x += 0.1;
		// 	println!("{:?} \t\t {:?}", c, c.to_equatorial());
		// }
		// let mut e = Equatorial { ra: Radians(0.0), dec: Radians(0.0)};//-M_PI / 2.0) };
		// for i in 0..20
		// {
		// 	e.ra.0 += M_PI / 10.0;
		// 	println!("{:?} \t\t {:?}", e, e.to_cartesian3());
		// }
		// panic!("");
		let mut rng = rand::thread_rng();
		for _i in 0..100
		{
			let e = Equatorial{
				ra:  Radians(rng.gen::<Decimal>() * M_PI * 2.0), 
				dec: Radians(rng.gen::<Decimal>() * M_PI - M_PI / 2.0)};
			
			let c = e.to_cartesian3();
			assert!(e.angle_distance(c.to_equatorial()) < Radians(0.00001));
		}
	}
}
