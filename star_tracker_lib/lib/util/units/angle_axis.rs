//! Implementation of AngleAxis.
use core_include::*;

use crate::util::units::Quaternion;
use crate::util::units::AngleAxis;
use crate::util::units::Matrix;
use crate::util::units::MatPos;

use crate::util::Maths;

// use crate::util::aliases::DECIMAL_PRECISION;
// use crate::util::test::TestEqual;



impl AngleAxis
{
//###############################################################################################//
//									--- Conversion ---
//###############################################################################################//

	/// Constructs a quaternion from this angle axis.
	/// # Example
	/// ```
	/// use star_tracker_lib::util::aliases::M_PI;
	/// use star_tracker_lib::util::units::Vector3;
	/// use star_tracker_lib::util::units::AngleAxis;
	/// use star_tracker_lib::util::units::Radians;
	/// use star_tracker_lib::util::units::Quaternion;
	///
	/// let mut axis : Vector3 = Vector3{x: 1.0, y: 2.0, z: 3.0};
	///	let angle = Radians(M_PI / 3.1);
	///	axis.normalize();
	///	let angle_axis : AngleAxis = AngleAxis{angle: angle, axis: axis};
	///	assert_eq!(angle_axis, angle_axis.to_quaternion().to_angle_axis());
	/// ```
	pub fn to_quaternion ( &self ) -> Quaternion
	{
		let axis_r = self.axis.normalized();
		if let Result::Ok(axis) = axis_r
		{
			return Quaternion {
				w: (self.angle / 2.0).0.cos(),
				x: (self.angle / 2.0).0.sin() * axis.x,
				y: (self.angle / 2.0).sin() * axis.y,
				z: (self.angle / 2.0).sin() * axis.z,
			};
		}
		return Quaternion{w: 1.0, x: 1.0, y: 1.0, z: 1.0}; // Identity, no rotation.
	}


	/// Converts angle axis to represent the position and rotation of a camera in matrix form.
	/// This can be used for cv based matrix transformations.
	/// This can be used in a 3x3 or a 4x4 size.
	pub fn to_matrix ( &self ) -> Matrix<4,4>
	{
		// Follow this equation:
		// [Rodregues Formula](https://mathworld.wolfram.com/RodriguesRotationFormula.html)
		// let mut mat : Matrix<3,3> = Matrix::new();
		let identity : Matrix<4,4> = Matrix::identity();
		let mut w : Matrix<4,4> = Matrix::new(); // Anti symmetric matrix.
		// Horizontal [x, y, z] * transform method.
		// w.set(MatPos{row: 0, col: 1}, -self.axis.z);
		// w.set(MatPos{row: 0, col: 2}, self.axis.y);
		//
		// w.set(MatPos{row: 1, col: 0}, self.axis.z);
		// w.set(MatPos{row: 1, col: 2}, -self.axis.x);
		//
		// w.set(MatPos{row: 2, col: 0}, -self.axis.y);
		// w.set(MatPos{row: 2, col: 1}, self.axis.x);

		// Open CV transform method (vertical position).
		w.set(MatPos{row: 1, col: 0}, -self.axis.z);
		w.set(MatPos{row: 2, col: 0}, self.axis.y);

		w.set(MatPos{row: 0, col: 1}, self.axis.z);
		w.set(MatPos{row: 2, col: 1}, -self.axis.x);

		w.set(MatPos{row: 0, col: 2}, -self.axis.y);
		w.set(MatPos{row: 1, col: 2}, self.axis.x);

		return identity + w * self.angle.0.sin() + w*w * (1.0 - self.angle.0.cos());
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
	
	use crate::util::units::AngleAxis;
	use crate::util::units::Vector3;
	use crate::util::units::Radians;
	use crate::util::units::Degrees;
	use crate::util::aliases::M_PI;

//###############################################################################################//
//
//									Conversion
//
// pub fn to_quaternion ( &self ) -> Quaternion
// pub fn to_matrix     ( &self ) -> Matrix<4,4>
//
//###############################################################################################//
//										~ to_quaternion ~										 //

	//
	//  to_quaternion ( &self ) -> Quaternion
	//
	#[test]
	fn test_to_quaternion ( )
	{
		let angle = Radians(M_PI / 2.0);
		let axis = Vector3{x: 1.0, y: 0.0, z: 0.0};
		let angle_axis = AngleAxis{angle:angle, axis: axis};
		let q = angle_axis.to_quaternion();
		assert_eq!(q.w, (angle / 2.0).cos());
		assert_eq!(q.x, (angle / 2.0).sin() * axis.x);
		assert_eq!(q.y, 0.0);
		assert_eq!(q.z, 0.0);
	}

	#[test]
	// If the axis is not defined, the rotation should be 0.
	fn test_to_quaternion_identity ( )
	{
		let angle = Radians(M_PI / 2.0);
		let axis = Vector3{x: 0.0, y: 0.0, z: 0.0};
		let angle_axis = AngleAxis{angle: angle, axis: axis};
		let q = angle_axis.to_quaternion();
		assert_eq!(q.w, 1.0);
		assert_eq!(q.x, 1.0);
		assert_eq!(q.y, 1.0);
		assert_eq!(q.z, 1.0);
	}


//										~ to_matrix ~											 //
	#[test]
	fn test_to_matrix_3x3 ( )
	{
		let mut rng = rand::thread_rng();
		for _ in 0..100
		{
			let angle = Radians(rng.gen_range(-M_PI..M_PI));
			let mut axis = Vector3 {
				x: rng.gen_range(-1.0..1.0),
				y: rng.gen_range(-1.0..1.0),
				z: rng.gen_range(-1.0..1.0) };

			axis.normalize().expect("dont let it be 0,0,0");
			let angle_axis = AngleAxis{angle: angle, axis: axis};
			let q = angle_axis.to_quaternion();
			let m = angle_axis.to_matrix();

			let point = Vector3 {
				x: rng.gen_range(-1.0..1.0),
				y: rng.gen_range(-1.0..1.0),
				z: rng.gen_range(-1.0..1.0) };

			assert_eq!(q.rotate_point(point), (m * point.to_matrix_column_homo()).to_vector3());
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
		let angle = Degrees(0.1234).as_radians();
		let axis  = Vector3 { x: 1.1234, y: 2.1234, z: 3.1234 };
		let aa    = AngleAxis{angle: angle, axis: axis};
		assert_eq!(format!("{}", aa), "AngleAxis(0.123d,\tVector3(1.123, 2.123, 3.123))");
	}
	
	
	#[test]
	fn test_debug_fmt ( )
	{
		let angle = Radians(0.123);
		let axis = Vector3 { x: 1.1234, y: 2.1234, z: 3.1234 };
		let aa    = AngleAxis{angle: angle, axis: axis};
		assert_eq!(format!("{:?}", aa), 
			"AngleAxis(Radians(0.123),\tVector3(x: 1.1234, y: 2.1234, z: 3.1234))");
	}
	
}