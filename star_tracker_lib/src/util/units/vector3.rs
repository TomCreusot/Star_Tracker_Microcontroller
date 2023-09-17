//! Implementation for [Vector3](crate::util::units::Vector3).
use core_include::*;

use crate::util::aliases::DECIMAL_PRECISION;
use crate::util::aliases::Decimal;
use crate::util::aliases::M_PI;
use crate::util::err::Errors;
use crate::util::err::Error;
use crate::util::units::Equatorial;
use crate::util::units::Vector3;
use crate::util::units::Vector2;
use crate::util::units::Radians;
use crate::util::units::Matrix;
use crate::util::units::MatPos;

use crate::util::Maths;


impl Vector3
{
//###############################################################################################//
//									--- Operations ---
//###############################################################################################//
	/// Finds the magnitude (length) of the vector.  
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Vector3;
	/// use star_tracker_lib::util::test::TestEqual;
	///
	/// let c = Vector3{x: 10.3, y: 23.1, z: 12.3};
	/// assert!(c.magnitude().test_close(&28.124544440, 0.00001));
	/// ```
	pub fn magnitude ( &self ) -> Decimal
	{
		return (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt();
	}


	/// Normalizes the vector so the magnitude/length is 1.  
	/// Returns Errors::NaN magnitude is 0.  
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Vector3;
	/// use star_tracker_lib::util::test::TestEqual;
	///
	/// let mut c = Vector3{x: 10.3, y: 23.1, z: 12.3};
	/// let c_out = Vector3{x: 0.366228, y: 0.8213466, z: 0.43734};
	/// c.normalize().expect("Will return Errors::NaN if magnitude is 0.");
	/// assert!(c.test_close(&c_out, 0.0001));
	/// ```
	pub fn normalize ( &mut self ) -> Error<()>
	{
		let magnitude  = self.magnitude();
		if magnitude < DECIMAL_PRECISION
		{
			return Result::Err(Errors::NaN);
		}
		self.x /= magnitude;
		self.y /= magnitude;
		self.z /= magnitude;
		return Result::Ok(());
	}


	/// Returns a new normalized vector of the current direction.  
	/// Returns Errors::NaN magnitude is 0.  
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Vector3;
	/// use star_tracker_lib::util::test::TestEqual;
	///
	/// let mut c = Vector3{x: 10.3, y: 23.1, z: 12.3};
	/// let c_out = Vector3{x: 0.366228, y: 0.8213466, z: 0.43734};
	/// assert!(c.normalized().expect("NaN if magnitude is 0.").test_close(&c_out, 0.0001));
	/// ```
	pub fn normalized ( &self ) -> Error<Self>
	{
		let magnitude  = self.magnitude();
		if magnitude < DECIMAL_PRECISION
		{
			return Result::Err(Errors::NaN);
		}
		return Result::Ok(Vector3{x: self.x/magnitude, y: self.y/magnitude, z: self.z/magnitude});
	}


	/// Finds the cross product between self and the input object.  
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Vector3;
	///
	/// let a = Vector3 { x: -1.0, y: 2.0, z: 10.0 };
	/// let b = Vector3 { x: 9.0, y: 3.0, z: -4.0 };
	/// assert_eq!(a.cross(b), Vector3{x: -38.0, y: 86.0, z: -21.0});
	/// assert_eq!(b.cross(a), Vector3{x: 38.0, y: -86.0, z: 21.0});
	/// ```
	pub fn cross ( &self, other: Vector3 ) -> Vector3
	{
		return Vector3
		{
			x: self.y * other.z - self.z * other.y,
			y: self.z * other.x - self.x * other.z,
			z: self.x * other.y - self.y * other.x
		};
	}

	/// Finds the dot product between the Vector3 points.
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Vector3;
	///
	/// let a = Vector3 { x: 2.0, y: 3.0, z: 4.0 };
	/// let b = Vector3 { x: 5.0, y: 6.0, z: 7.0 };
	/// assert_eq!(a.dot(b), 56.0);
	/// ```
	pub fn dot ( &self, other: Vector3 ) -> Decimal
	{ return self.x * other.x + self.y * other.y + self.z * other.z; }




	/// Finds the angle between the 2 points on a sphere.  
	/// The great circle distance.  
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Vector3;
	/// use star_tracker_lib::util::units::Degrees;
	///
	/// let mut car1 = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
	/// let mut car2 = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
	/// assert_eq!(car1.angle_distance(car2), Degrees(90.0).to_radians());
	/// ```
	pub fn angle_distance ( &self, oth: Vector3 ) -> Radians
	{
		let dot = self.dot(oth);

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


//###############################################################################################//
//									--- Conversion ---
//###############################################################################################//


	/// Converts the coordinates into a column matrix form (vertical set of rows).
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Vector3;
	/// use star_tracker_lib::util::units::Matrix;
	/// use star_tracker_lib::util::units::MatPos;
	///
	/// let c = Vector3 {x: 1.0, y: 2.0, z: 3.0};
	/// let m: Matrix<3,1> = c.to_matrix_column();
	/// assert_eq!(c.x, m.get(MatPos{row: 0, col: 0}));
	/// assert_eq!(c.y, m.get(MatPos{row: 1, col: 0}));
	/// assert_eq!(c.z, m.get(MatPos{row: 2, col: 0}));
	/// ```
	pub fn to_matrix_column ( &self ) -> Matrix<3,1>
	{
		let mut mat: Matrix<3,1> = Matrix::new();
		mat.set(MatPos{row: 0, col: 0}, self.x);
		mat.set(MatPos{row: 1, col: 0}, self.y);
		mat.set(MatPos{row: 2, col: 0}, self.z);
		return mat;
	}

	/// Converts the coordinates into a row matrix form (horizontal set of columns).
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Vector3;
	/// use star_tracker_lib::util::units::Matrix;
	/// use star_tracker_lib::util::units::MatPos;
	///
	/// let c = Vector3 {x: 1.0, y: 2.0, z: 3.0};
	/// let m: Matrix<1,3> = c.to_matrix_row();
	/// assert_eq!(c.x, m.get(MatPos{row: 0, col: 0}));
	/// assert_eq!(c.y, m.get(MatPos{row: 0, col: 1}));
	/// assert_eq!(c.z, m.get(MatPos{row: 0, col: 2}));
	/// ```
	pub fn to_matrix_row ( &self ) -> Matrix<1,3>
	{
		let mut mat: Matrix<1,3> = Matrix::new();
		mat.set(MatPos{row: 0, col: 0}, self.x);
		mat.set(MatPos{row: 0, col: 1}, self.y);
		mat.set(MatPos{row: 0, col: 2}, self.z);
		return mat;
	}


	/// Converts the coordinates into a column matrix form (vertical set of rows).
	/// This is in homogeneous form for matrix transformations.
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Vector3;
	/// use star_tracker_lib::util::units::Matrix;
	/// use star_tracker_lib::util::units::MatPos;
	///
	/// let c = Vector3 {x: 1.0, y: 2.0, z: 3.0};
	/// let m: Matrix<4,1> = c.to_matrix_column_homo();
	/// assert_eq!(c.x, m.get(MatPos{row: 0, col: 0}));
	/// assert_eq!(c.y, m.get(MatPos{row: 1, col: 0}));
	/// assert_eq!(c.z, m.get(MatPos{row: 2, col: 0}));
	/// assert_eq!(1.0, m.get(MatPos{row: 3, col: 0}));
	/// ```
	pub fn to_matrix_column_homo ( &self ) -> Matrix<4,1>
	{
		let mut mat: Matrix<4,1> = Matrix::new();
		mat.set(MatPos{row: 0, col: 0}, self.x);
		mat.set(MatPos{row: 1, col: 0}, self.y);
		mat.set(MatPos{row: 2, col: 0}, self.z);
		mat.set(MatPos{row: 3, col: 0}, 1.0);
		return mat;
	}


	/// Converts self to vector2 (LOSSY).
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Vector3;
	/// use star_tracker_lib::util::units::Vector2;
	///
	/// let vec_3 = Vector3{x: 12.3, y: 23.4, z: 34.5};
	/// let vec_2 = vec_3.to_vector2();
	/// assert_eq!(vec_2.x, vec_3.x);
	/// assert_eq!(vec_2.y, vec_3.y);
	/// ```
	pub fn to_vector2 ( &self ) -> Vector2
	{
		return Vector2{x: self.x, y: self.y};
	}


	/// Converts Vector3 to Equatorial coordinates.
	/// # Example
	/// ```
	/// use star_tracker_lib::util::aliases::M_PI;
	/// use star_tracker_lib::util::units::{Equatorial, Vector3, Radians};
	/// use star_tracker_lib::util::aliases::Decimal;
	/// use star_tracker_lib::util::test::TestEqual;
	///
	/// let mut c = Vector3 { x: 0.5, y: 0.5, z: -0.7071067812 };
	/// let mut e = c.to_equatorial();
	/// let mut compare = Equatorial{ra: Radians(M_PI / 4.0), dec: Radians(-M_PI / 4.0)};
	/// assert!(compare.test_close(&e, 0.0000001));
	///
	/// c = Vector3 { x: 3.0, y: 4.0, z: 5.0 };
	/// e = c.to_equatorial();
	/// compare = Equatorial{ra: Radians(0.927295218), dec: Radians(0.78539816)};
	/// assert!(compare.test_close(&e, 0.0000001));
	/// ```
	pub fn to_equatorial ( &self ) -> Equatorial
	{
		// ra = atan(y/x)
		// dec = acos(z / sqrt(x^2 + y ^2 + z^2)) = atan(sqrt(x^2 + y^2) / z)
		let mut ra = (self.y).atan2(self.x);
		
		if ra < 0.0
		{
			ra = -ra.abs() + M_PI * 2.0;
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

	use crate::util::units::Vector3;
	use crate::util::units::Matrix;
	use crate::util::units::MatPos;
	use crate::util::units::Equatorial;
	use crate::util::units::Radians;
	use crate::util::units::Degrees;
	use crate::util::units::Decimal;
	use crate::util::aliases::M_PI;
	use crate::util::test::TestEqual;
	use crate::util::err::Errors;
	use crate::util::err::Error;

//###############################################################################################//
//
//										Features
//
// pub fn magnitude      ( &self )        -> Decimal
// pub fn normalize      ( &mut self )    -> Error<()>
// pub fn normalized     ( &self )        -> Error<Self>
// pub fn cross          ( &self, &Self ) -> Vector3
// pub fn dot            ( &self, &Self ) -> Decimal
// pub fn angle_distance ( &self, &Self ) -> Radians
//
//###############################################################################################//
//										~ magnitude ~											 //
	#[test]
	fn test_magnitude ( )
	{
		let c = Vector3{x: 10.3, y: 23.1, z: 12.3};
		assert!(c.magnitude().test_close(&28.1245463, 0.0001));
	}


//										~ normalize ~											 //
	#[test]
	// If the magnitude is not 0, the object should be normalized.
	fn test_normalize ( )
	{
		let mut c = Vector3{x: 10.3, y: 23.1, z: 12.3};
		let c_out = Vector3{x: 0.366228, y: 0.8213466, z: 0.43734};
		c.normalize().expect("This is fine.");
		assert!(c.test_close(&c_out, 0.00001));
	}
	
	#[test]
	// If the magnitude is not 0, the object should be normalized.
	fn test_normalize_error ( )
	{
		let mut c = Vector3{x: 0.0, y: 0.0, z: 0.0};
		assert_eq!(c.normalize(), Err(Errors::NaN));
	}

//										~ normalized ~											 //
	#[test]
	fn test_normalized ( )
	{
		let c = Vector3{x: 10.3, y: 23.1, z: 12.3};
		let c_out = Vector3{x: 0.366228, y: 0.8213466, z: 0.43734};
		assert!(c.normalized().expect("Not 0 vector").test_close(&c_out, 0.00001));
	}

	#[test]
	// If the magnitude is not 0, the object should be normalized.
	fn test_normalized_error ( )
	{
		let mut c = Vector3{x: 0.0, y: 0.0, z: 0.0};
		assert_eq!(c.normalize(), Err(Errors::NaN));
	}
	
//										~ cross ~												 //
	#[test]
	fn test_cross_zero ( )
	{
		let a = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
		let b = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
		assert_eq!(a.cross(b), Vector3{x: 0.0, y: 0.0, z: 0.0});
	}

	#[test]
	fn test_cross_single_axis ( )
	{
		let a = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
		let b = Vector3 { x: 0.0, y: 2.0, z: 0.0 };
		let c = Vector3 { x: 0.0, y: 0.0, z: 3.0 };
		assert_eq!(a.cross(b), Vector3{x: 0.0, y: 0.0, z: 2.0});
		assert_eq!(b.cross(a), Vector3{x: 0.0, y: 0.0, z: -2.0});
		assert_eq!(a.cross(c), Vector3{x: 0.0, y: -3.0, z: 0.0});
		assert_eq!(c.cross(a), Vector3{x: 0.0, y: 3.0, z: 0.0});
		assert_eq!(b.cross(c), Vector3{x: 6.0, y: 0.0, z: 0.0});
		assert_eq!(c.cross(b), Vector3{x: -6.0, y: 0.0, z: 0.0});
	}

	#[test]
	fn test_cross_dual_axis ( )
	{
		let a = Vector3 { x: 1.0, y: 2.0, z: 0.0 };
		let b = Vector3 { x: 0.0, y: 3.0, z: 4.0 };
		let c = Vector3 { x: 5.0, y: 0.0, z: 6.0 };
		assert_eq!(a.cross(b), Vector3{x: 8.0, y: -4.0, z: 3.0});
		assert_eq!(b.cross(a), Vector3{x: -8.0, y: 4.0, z: -3.0});
		assert_eq!(a.cross(c), Vector3{x: 12.0, y: -6.0, z: -10.0});
		assert_eq!(c.cross(a), Vector3{x: -12.0, y: 6.0, z: 10.0});
		assert_eq!(b.cross(c), Vector3{x: 18.0, y: 20.0, z: -15.0});
		assert_eq!(c.cross(b), Vector3{x: -18.0, y: -20.0, z: 15.0});
	}

	#[test]
	fn test_cross_tri_axis ( )
	{
		let a = Vector3 { x: -1.0, y: 2.0, z: 10.0 };
		let b = Vector3 { x: 9.0, y: 3.0, z: -4.0 };
		assert_eq!(a.cross(b), Vector3{x: -38.0, y: 86.0, z: -21.0});
		assert_eq!(b.cross(a), Vector3{x: 38.0, y: -86.0, z: 21.0});
	}


//										~ dot ~													 //
	#[test]
	fn test_dot ( )
	{
		let a = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
		let b = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
		let c = Vector3 { x: 1.0, y: 1.0, z: 0.0 };
		let d = Vector3 { x: 0.0, y: 0.0, z: 1.0 };
		let e = Vector3 { x: 1.0, y: 0.0, z: 1.0 };
		let f = Vector3 { x: 0.0, y: 1.0, z: 1.0 };
		let g = Vector3 { x: 1.0, y: 1.0, z: 1.0 };

		assert_eq!(a.dot(g), 1.0);
		assert_eq!(a.dot(b), 0.0);
		assert_eq!(b.dot(g), 1.0);
		assert_eq!(c.dot(g), 2.0);
		assert_eq!(d.dot(g), 1.0);
		assert_eq!(e.dot(g), 2.0);
		assert_eq!(f.dot(g), 2.0);
	}


//										~ angle_distance ~										 //
	#[test]
	fn test_angle_distance ( )
	{
		let car1 = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
		let mut car2 = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
		assert_eq!(car1.angle_distance(car2), Degrees(90.0).to_radians());
		car2.x = 0.0;
		car2.y = 1.0;
		assert_eq!(car1.angle_distance(car2), Degrees(0.0).to_radians());
		car2.x = 0.0;
		car2.y = -1.0;
		assert_eq!(car1.angle_distance(car2), Degrees(180.0).to_radians());
	}







//###############################################################################################//
//
//										Matrix
//
// pub fn to_matrix_column      ( &self ) -> Matrix<3,1> 
// pub fn to_matrix_row         ( &self ) -> Matrix<1,3> 
// pub fn to_matrix_column_homo ( &self ) -> Matrix<4,1> 
// pub fn to_vector2            ( &self ) -> Vector2 
// pub fn to_equatorial         ( &self ) -> Equatorial 
//
//###############################################################################################//
//										~ to_matrix_column ~									 //
	#[test]
	fn test_to_matrix_column ( )
	{
		let c = Vector3 {x: 1.0, y: 2.0, z: 3.0};
		let m: Matrix<3,1> = c.to_matrix_column();
		assert_eq!(c.x, m.get(MatPos{row: 0, col: 0}));
		assert_eq!(c.y, m.get(MatPos{row: 1, col: 0}));
		assert_eq!(c.z, m.get(MatPos{row: 2, col: 0}));
	}

//										~ to_matrix_row ~										 //
	#[test]
	fn test_to_matrix_row ( )
	{
		let c = Vector3 {x: 1.0, y: 2.0, z: 3.0};
		let m: Matrix<1,3> = c.to_matrix_row();
		assert_eq!(c.x, m.get(MatPos{row: 0, col: 0}));
		assert_eq!(c.y, m.get(MatPos{row: 0, col: 1}));
		assert_eq!(c.z, m.get(MatPos{row: 0, col: 2}));
	}

//										~ to_matrix_column_homo ~								 //
	#[test]
	fn test_to_matrix_column_homo ( )
	{
		let c = Vector3 {x: 1.0, y: 2.0, z: 3.0};
		let m: Matrix<4,1> = c.to_matrix_column_homo();
		assert_eq!(c.x, m.get(MatPos{row: 0, col: 0}));
		assert_eq!(c.y, m.get(MatPos{row: 1, col: 0}));
		assert_eq!(c.z, m.get(MatPos{row: 2, col: 0}));
		assert_eq!(1.0, m.get(MatPos{row: 3, col: 0}));
	}


//										~ to_vector2 ~											 //
	#[test]
	fn test_to_vector_2 ( )
	{
		let vec_3 = Vector3{x: 12.3, y: 23.4, z: 34.5};
		let vec_2 = vec_3.to_vector2();
		assert_eq!(vec_2.x, vec_3.x);
		assert_eq!(vec_2.y, vec_3.y);
	}




//										~ to_equatorial ~										 //
	#[test]
	fn test_to_equatorial ( )
	{
		let mut c = Vector3 { x: 0.5, y: 0.5, z: -0.7071067812 };
		let mut e = c.to_equatorial();
		let mut compare = Equatorial{ra: Radians(M_PI / 4.0), dec: Radians(-M_PI / 4.0)};
		assert!(e.test_close(&compare, 0.000001));
		c = Vector3 { x: 3.0, y: 4.0, z: 5.0 };
		e = c.to_equatorial();
		compare = Equatorial{ra: Radians(0.927295218), dec: Radians(0.78539816)};
		assert!(e.test_close(&compare, 0.000001));
	}

	#[test]
	fn test_to_equatorial_z_full ( )
	{
		let mut c = Vector3 { x: 0.0, y: 0.0, z: 10000.0 };
		let mut e = c.to_equatorial();
		assert_eq!(e.ra,  Radians(0.0));
		assert_eq!(e.dec, Radians(M_PI / 2.0));
		c = Vector3 { x: 0.0, y: 0.0, z: -10000.0 };
		 e = c.to_equatorial();
		assert_eq!(e.ra,  Radians(0.0));
		assert_eq!(e.dec, Radians(-M_PI / 2.0));
	}

	#[test]
	fn test_to_equatorial_on_ra ( )
	{
		let mut c = Vector3 { x: 1.0, y: 0.0, z: 0.0 }; // x is 0
		let mut e = c.to_equatorial();
		assert_eq!(e.ra,  Degrees(0.0).to_radians());
		
		c = Vector3{ x: 1.0, y: 1.0, z: 0.0 }.normalized().expect("");
		e = c.to_equatorial();
		e.ra.test_close(&Degrees(45.0).to_radians(), 0.001);
		
		c = Vector3{ x: 0.0, y: 1.0, z: 0.0 }.normalized().expect("");
		e = c.to_equatorial();
		e.ra.test_close(&Degrees(90.0).to_radians(), 0.001);
		
		c = Vector3{ x: -1.0, y: 1.0, z: 0.0 }.normalized().expect("");
		e = c.to_equatorial();
		e.ra.test_close(&Degrees(135.0).to_radians(), 0.001);
		
		c = Vector3{ x: -1.0, y: 0.0, z: 0.0 }.normalized().expect("");
		e = c.to_equatorial();
		e.ra.test_close(&Degrees(180.0).to_radians(), 0.001);
		
		c = Vector3{ x: -1.0, y: -1.0, z: 0.0 }.normalized().expect("");
		e = c.to_equatorial();
		e.ra.test_close(&Degrees(225.0).to_radians(), 0.001);
		
		c = Vector3{ x: 0.0, y: -1.0, z: 0.0 }.normalized().expect("");
		e = c.to_equatorial();
		e.ra.test_close(&Degrees(270.0).to_radians(), 0.001);
		
		c = Vector3{ x: 1.0, y: -1.0, z: 0.0 }.normalized().expect("");
		e = c.to_equatorial();
		e.ra.test_close(&Degrees(315.0).to_radians(), 0.001);
	}


	#[test]
	fn test_to_equatorial_z_high ( )
	{
		let mut c = Vector3 { x: 0.1, y: 0.1, z: 10000.0 };
		let mut e = c.to_equatorial();
		let mut compare = Equatorial{ra: Radians(0.7853), dec: Radians(M_PI / 2.0)};
		assert!(e.test_close(&compare, 0.001));
		c = Vector3 { x: 0.1, y: 0.1, z: -10000.0 };
		e = c.to_equatorial();
		compare = Equatorial{ra: Radians(0.7853), dec: -Radians(M_PI / 2.0)};
		assert!(e.test_close(&compare, 0.001));
	}


	#[test]
	// #[no_coverage]
	fn test_to_equatorial_random ( )
	{
		let mut rng = rand::thread_rng();
		for _ in 0..100
		{
			let e = Equatorial{
				ra:  Radians(rng.gen::<Decimal>() * M_PI * 2.0),
				dec: Radians(rng.gen::<Decimal>() * M_PI - M_PI / 2.0)};

			let c = e.to_vector3();
			assert!(e.angle_distance(c.to_equatorial()) < Radians(0.01));
		}
	}
	
	
	
	
	
//###############################################################################################//
//
//										Debug
// Display: Show neat (3dp)
// Debug: Show everything (all dp)
// 
//###############################################################################################//
	//								- Display / Debug fmt -										//
	#[test]
	fn test_display_fmt ( )
	{
		let vec = Vector3 { x: 1.1234, y: 2.1234, z: 3.1234 };
		assert_eq!(format!("{:123414}", vec), "Vector3(1.123, 2.123, 3.123)");
	}
	
	
	#[test]
	fn test_debug_fmt ( )
	{
		let vec = Vector3 { x: 1.1234, y: 2.1234, z: 3.1234 };
		assert_eq!(format!("{:?}", vec), "Vector3(x: 1.1234, y: 2.1234, z: 3.1234)");
	}
	
	
}
