//! The implementation of Cartesian3D.
use crate::util::aliases::Decimal;
use super::{Equatorial, Cartesian3D, Radians};

impl Cartesian3D
{
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
	/// let mut c = Cartesian3D { x: 0.5, y: 0.5, z: -0.7071067812 };
	/// let mut e = c.to_equatorial();
	/// assert_eq!(e.ra, Radians(M_PI / 4.0));
	/// assert_eq!(e.dec, Radians(-M_PI / 4.0));
	///
	/// c = Cartesian3D { x: 3.0, y: 4.0, z: 5.0 };
	/// e = c.to_equatorial();
	/// assert_eq!(e.ra, Radians(0.927295218));
	/// assert_eq!(e.dec, Radians(0.78539816));
	/// ```
	pub fn to_equatorial ( &self ) -> Equatorial
	{
		use std::f32::consts::PI;

		// ra = atan(y/x)
		// dec = acos(z / sqrt(x^2 + y ^2 + z^2)) = atan(sqrt(x^2 + y^2) / z)
		let mut ra = 0.0;
		if self.x != 0.0
		{
			ra = (self.y / self.x).atan();
		}
		let mut dec = 0.0;
		if self.z != 0.0
		{
			if self.x == 0.0 || self.y == 0.0
			{
				dec = (PI / 2.0).copysign(self.z);
			}
			else
			{
				dec = ((self.x.powf(2.0) + self.y.powf(2.0)).sqrt() / self.z).atan();
			}
		}
		// eq.set_phi(dec);
		let eq = Equatorial{ra: Radians(ra), dec: Radians(dec) };
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
	use util::units::Cartesian3D;
	use util::units::Radians;
//###############################################################################################//
//										---	Equatorial ---
//###############################################################################################//



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
	//  to_equatorial ( ) -> Equatorial
	//
	#[test]
	fn test_to_equatorial ( )
	{
		let mut c = Cartesian3D { x: 0.5, y: 0.5, z: -0.7071067812 };
		let mut e = c.to_equatorial();
		assert_eq!(e.ra,  Radians(PI / 4.0));
		assert_eq!(e.dec, Radians(-PI / 4.0));

		c = Cartesian3D { x: 3.0, y: 4.0, z: 5.0 };
		e = c.to_equatorial();
		assert_eq!(e.ra,  Radians(0.927295218));
		assert_eq!(e.dec, Radians(0.78539816));
	}


	#[test]
	fn test_to_equatorial_z_zero ( )
	{
		let c = Cartesian3D { x: 0.5, y: 0.5, z: 0.0 };
		let e = c.to_equatorial();
		assert_eq!(e.ra,  Radians(PI / 4.0));
		assert_eq!(e.dec, Radians(0.0));
	}

	#[test]
	fn test_to_equatorial_z_full ( )
	{
		let mut c = Cartesian3D { x: 0.0, y: 0.0, z: 10000.0 };
		let mut e = c.to_equatorial();
		assert_eq!(e.ra,  Radians(0.0));
		assert_eq!(e.dec, Radians(PI / 2.0));
		c = Cartesian3D { x: 0.0, y: 0.0, z: -10000.0 };
		 e = c.to_equatorial();
		assert_eq!(e.ra,  Radians(0.0));
		assert_eq!(e.dec, Radians(-PI / 2.0));
	}


	#[test]
	fn test_to_equatorial_z_high ( )
	{
		let mut c = Cartesian3D { x: 0.1, y: 0.1, z: 10000.0 };
		let mut e = c.to_equatorial();
		assert_eq!(e.ra,  Radians(0.785398163));
		assert_eq!(e.dec, Radians(0.000014142136));
		c = Cartesian3D { x: 0.1, y: 0.1, z: -10000.0 };
		e = c.to_equatorial();
		assert_eq!(e.ra,   Radians(0.785398163));
		assert_eq!(e.dec,  Radians(-0.000014142136));
}
}
