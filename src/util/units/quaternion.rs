//! This is the implementation of Quaternion.
use super::{Quaternion, Cartesian3D, AngleAxis};
use util::aliases::Decimal;

impl Quaternion
{
	/// Constructor with cartesian angle axis.
	/// The quaternion describes the rotation multiplier to acheive the angle axis rotation.
	/// # Arguments
	/// * `a` - The angle and axis to rotate around.
	///
	/// # Returns
	/// A quaternion for a rotation around the angle axis.
	///
	/// # Example
	/// ```
	/// use star_tracker::util::units::{Quaternion, Cartesian3D, Radians, AngleAxis};
	/// use star_tracker::util::aliases::M_PI;
	/// let angle = Radians(M_PI / 2.0);
	/// let axis = Cartesian3D{x: 1.0, y: 0.0, z: 0.0};
	/// let angle_axis = AngleAxis{angle: angle, axis: axis};
	/// let q = Quaternion::new(angle_axis);
	/// assert_eq!(q.w, (angle / 2.0).cos());
	/// assert_eq!(q.x, (angle / 2.0).sin() * axis.x);
	/// assert_eq!(q.y, 0.0);
	/// assert_eq!(q.z, 0.0);
	/// ```
	pub fn new ( a : AngleAxis ) -> Quaternion
	{
		assert!((a.axis.magnitude() - 1.0).abs() < 0.0000001, "Not unit vector");
		return Quaternion {
			w: (a.angle / 2.0).0.cos(),
			x: (a.angle / 2.0).0.sin() * a.axis.x,
			y: (a.angle / 2.0).sin() * a.axis.y,
			z: (a.angle / 2.0).sin() * a.axis.z,
		};
	}


	/// Rotates a point around this quaternion.
	/// # Arguments
	/// The point to rotate.
	/// # Returns
	/// The rotated point.
	///
	/// # Example
	/// ```
	/// use star_tracker::util::units::{Quaternion, Cartesian3D, Radians, AngleAxis};
	/// use star_tracker::util::aliases::M_PI;
	/// // Rotates point around the z axis where the point is 10 degrees x of z
	/// let axis = Cartesian3D{x: 0.0, y: 0.0, z: 1.0};
	/// let angle = Radians(M_PI / 4.0); // 45 degrees
	/// let angle_axis = AngleAxis{angle: angle, axis: axis};
	/// let offset_angle = Radians(M_PI / 18.0); // 10 degrees x of z.
	/// let pt = Cartesian3D{x: (offset_angle).sin(), y: 0.0, z: (offset_angle).cos()};
	/// let q = Quaternion::new(angle_axis);
	/// let rotated = q.rotate_point(pt);
	/// let expected = Cartesian3D{
	/// 						x: (offset_angle).sin() * (Radians(M_PI/2.0) + angle).sin(),
	/// 						y: (offset_angle).sin() * (Radians(M_PI/2.0) + angle).cos(),
	/// 						z: pt.z};
	/// assert!((rotated.x - expected.x).abs() < 0.0001);
	/// assert!((rotated.y - expected.y).abs() < 0.0001);
	/// assert!((rotated.z - expected.z).abs() < 0.0001);
	/// ```
	pub fn rotate_point ( self, pt: Cartesian3D ) -> Cartesian3D
	{
		let q_pt = Quaternion{w: 0.0, x: pt.x, y: pt.y, z: pt.z};
		let rotation = self.conjugate() * q_pt * self;
		return Cartesian3D{x: rotation.x, y: rotation.y, z: rotation.z};
	}


	/// Finds the conjugate of this quaternion.
	/// # Returns
	/// The conjugate of the current quaternion.
	/// # Example
	/// ```
	/// use star_tracker::util::units::Quaternion;
	/// let q = Quaternion{w: 1.0, x: 2.0, y: 3.0, z: 4.0};
	///	assert_eq!(q.conjugate(), Quaternion{w: 1.0, x: -2.0, y: -3.0, z: -4.0});
	/// ```
	pub fn conjugate ( &self ) -> Quaternion
	{
		return Quaternion {
			w: self.w,
			x: -self.x,
			y: -self.y,
			z: -self.z,
		};
	}
	
	
	
	/// Finds the dot product of the quaternions.
	/// Useful for comparing.
	/// # Returns
	/// The dot product of the quaternion.
	///
	/// # Example
	/// ```
	/// use star_tracker::util::units::Cartesian3D;
	/// use star_tracker::util::units::Quaternion;
	/// use star_tracker::util::units::AngleAxis;
	/// use star_tracker::util::units::Radians;
	///
	/// //Compares 2 quaternions which are identical but are represented incorrectly as angle axis.
	/// let a_1 = AngleAxis{angle: Radians(90.000001), axis: Cartesian3D{x: 1.0, y: 0.0, z: 0.0}};
	/// let a_2 = AngleAxis{angle: Radians(-90.0001), axis: Cartesian3D{x: -1.0, y: 0.0, z: 0.0}};
	/// let a_3 = AngleAxis{angle: Radians(-90.0001), axis: Cartesian3D{x: 0.0, y: 1.0, z: 0.0}};
	/// 
	/// let q_1 = Quaternion::new(a_1);
	/// let q_2 = Quaternion::new(a_2);
	/// let q_3 = Quaternion::new(a_3);
	/// 
	/// assert!(1.0 - q_1.dot(q_2).abs() < 0.00001);
	/// assert!(0.00001 < 1.0 - q_1.dot(q_3).abs());
	/// ```
	pub fn dot ( &self, other: Quaternion ) -> Decimal
	{
		return self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w;
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
	
	use util::aliases::{M_PI};
	use util::units::{Quaternion, Cartesian3D, Radians, AngleAxis};

	//
	//  new <T: Cartesian3D>( angle: Decimal, axis: T ) -> Quaternion
	//
	#[test]
	fn test_new ( )
	{
		let angle = Radians(M_PI / 2.0);
		let axis = Cartesian3D{x: 1.0, y: 0.0, z: 0.0};
		let angle_axis = AngleAxis{angle:angle, axis: axis};
		let q = Quaternion::new(angle_axis);
		assert_eq!(q.w, (angle / 2.0).cos());
		assert_eq!(q.x, (angle / 2.0).sin() * axis.x);
		assert_eq!(q.y, 0.0);
		assert_eq!(q.z, 0.0);
	}

	#[test]
	#[should_panic = "Not unit vector"]
	#[allow(unused_variables)]
	fn test_new_panic ( )
	{
		let angle = Radians(M_PI);
		let axis = Cartesian3D{x: 1.0, y: 1.0, z: 1.0};
		let angle_axis = AngleAxis{angle:angle, axis: axis};
		let q = Quaternion::new(angle_axis);
	}

	//
	//	conjugate ( &self ) -> Quaternion
	//
	#[test]
	fn test_conjugate ( )
	{
		let q = Quaternion{w: 1.0, x: 2.0, y: 3.0, z: 4.0};
		assert_eq!(q.conjugate(), Quaternion{w: 1.0, x: -2.0, y: -3.0, z: -4.0});
	}


	//
	//	mul ( &self, rhs: &Self ) -> Self
	//
	#[test]
    fn mul_quaternion() {
        let q = Quaternion{w: 1.0, x: 2.0, y: 3.0, z: 4.0};
        let r = Quaternion{w: 4.0, x: 3.0, y: 2.0, z: 1.0};
        assert_eq!(q * r, Quaternion{w: -12.0, x: 6.0, y: 24.0, z: 12.0});
    }







	//           +y
	//           |
	//           |
	//          /--------- +z
	//         /
	//       +x



	//
	// rotate_point ( Cartesian3D ) -> Cartesian3D
	//
	#[test]
	fn test_rotate_point_x ( )
	{
		let axis = Cartesian3D{x: 1.0, y: 0.0, z: 0.0};
		let angle = Radians(0.0);
		let mut angle_axis = AngleAxis{angle:angle, axis: axis};
		let mut q = Quaternion::new(angle_axis);

		let mut pt : Cartesian3D = Cartesian3D{x: 0.0, y: 0.0, z: 1.0};
		let mut rotated = q.rotate_point(pt);
		let mut expected : Cartesian3D = Cartesian3D{x: 0.0, y: 0.0, z: 1.0};
		assert_eq!(expected, rotated);

		angle_axis.angle = Radians(M_PI / 2.0);
		q = Quaternion::new(angle_axis);
		rotated = q.rotate_point(pt);
		expected = Cartesian3D{x: 0.0, y: 1.0, z: 0.0};
		assert_eq!(expected, rotated);

		angle_axis.angle = Radians(M_PI);
		q = Quaternion::new(angle_axis);
		rotated = q.rotate_point(pt);
		expected = Cartesian3D{x: 0.0, y: 0.0, z: -1.0};
		assert_eq!(expected, rotated);

		let offset_angle = Radians(M_PI / 18.0); // 10 degrees z of x.
		angle_axis.angle = Radians(M_PI / 4.0); // 45 degrees
		pt = Cartesian3D{x: (offset_angle).0.cos(), y: (offset_angle).0.sin(), z: 0.0};
		q = Quaternion::new(angle_axis);
		rotated = q.rotate_point(pt);
		expected = Cartesian3D{	
						x: pt.x,
						y: offset_angle.0.sin() * (Radians(M_PI/2.0) + angle_axis.angle).0.sin(),
						z: -offset_angle.0.sin() * (Radians(M_PI/2.0) + angle_axis.angle).0.sin()};
		assert_eq!(expected, rotated);
	}

	#[test]
	fn test_rotate_point_y ( )
	{
		let axis = Cartesian3D{x: 0.0, y: 1.0, z: 0.0};
		let angle = Radians(0.0);
		let mut angle_axis = AngleAxis{angle:angle, axis: axis};
		let mut q = Quaternion::new(angle_axis);

		let mut pt : Cartesian3D = Cartesian3D{x: 0.0, y: 0.0, z: 1.0};
		let mut rotated = q.rotate_point(pt);
		let mut expected : Cartesian3D = Cartesian3D{x: 0.0, y: 0.0, z: 1.0};
		assert_eq!(expected, rotated);

		angle_axis.angle = Radians(M_PI / 2.0);
		q = Quaternion::new(angle_axis);
		rotated = q.rotate_point(pt);
		expected = Cartesian3D{x: -1.0, y: 0.0, z: 0.0};
		assert_eq!(expected, rotated);

		angle_axis.angle = Radians(M_PI);
		q = Quaternion::new(angle_axis);
		rotated = q.rotate_point(pt);
		expected = Cartesian3D{x: 0.0, y: 0.0, z: -1.0};
		assert_eq!(expected, rotated);

		let offset_angle = Radians(M_PI / 18.0); // 10 degrees z of y.
		angle_axis.angle = Radians(M_PI / 4.0); // 45 degrees
		pt = Cartesian3D{x: 0.0, y: offset_angle.0.cos(), z: offset_angle.0.sin()};
		q = Quaternion::new(angle_axis);
		rotated = q.rotate_point(pt);
		expected = Cartesian3D{	
						x: offset_angle.0.sin() * (Radians(M_PI/2.0) + angle_axis.angle).0.cos(),
						y: pt.y,
						z: offset_angle.0.sin() * (Radians(M_PI/2.0) + angle_axis.angle).0.sin()};
		assert_eq!(expected, rotated);
	}

	#[test]
	fn test_rotate_point_z ( )
	{
		let axis = Cartesian3D{x: 0.0, y: 0.0, z: 1.0};
		let angle = Radians(0.0);
		let mut angle_axis = AngleAxis{angle:angle, axis: axis};
		let mut q = Quaternion::new(angle_axis);

		let mut pt : Cartesian3D = Cartesian3D{x: 1.0, y: 0.0, z: 0.0};
		let mut rotated = q.rotate_point(pt);
		let mut expected : Cartesian3D = Cartesian3D{x: 1.0, y: 0.0, z: 0.0};
		assert_eq!(expected, rotated);

		angle_axis.angle = Radians(M_PI / 2.0);
		q = Quaternion::new(angle_axis);
		rotated = q.rotate_point(pt);
		expected = Cartesian3D{x: 0.0, y: -1.0, z: 0.0};
		assert_eq!(expected, rotated);

		angle_axis.angle = Radians(M_PI);
		q = Quaternion::new(angle_axis);
		rotated = q.rotate_point(pt);
		expected = Cartesian3D{x: -1.0, y: 0.0, z: 0.0};
		assert_eq!(expected, rotated);

		let offset_angle = Radians(M_PI / 18.0); // 10 degrees x of z.
		angle_axis.angle = Radians(M_PI / 4.0); // 45 degrees
		pt = Cartesian3D{x: offset_angle.0.sin(), y: 0.0, z: offset_angle.0.cos()};
		q = Quaternion::new(angle_axis);
		rotated = q.rotate_point(pt);
		expected = Cartesian3D{
						x: (offset_angle).0.sin() * (Radians(M_PI/2.0) + angle_axis.angle).0.sin(),
						y: (offset_angle).0.sin() * (Radians(M_PI/2.0) + angle_axis.angle).0.cos(),
						z: pt.z};
		assert_eq!(expected, rotated);
	}





	#[test]
	fn test_dot_usecase_singularity_angle_axis ( )
	{
		//Compares 2 quaternions which are identical but are represented incorrectly as angle axis.
		let a_1 = AngleAxis{angle: Radians(90.000001), axis: Cartesian3D{x: 1.0, y: 0.0, z: 0.0}};
		let a_2 = AngleAxis{angle: Radians(-90.0001), axis: Cartesian3D{x: -1.0, y: 0.0, z: 0.0}};
		let a_3 = AngleAxis{angle: Radians(-90.0001), axis: Cartesian3D{x: 0.0, y: 1.0, z: 0.0}};
		
		let q_1 = Quaternion::new(a_1);
		let q_2 = Quaternion::new(a_2);
		let q_3 = Quaternion::new(a_3);
		
		assert!(1.0 - q_1.dot(q_2).abs() < 0.00001);
		assert!(0.00001 < 1.0 - q_1.dot(q_3).abs());
	}
	
	
	#[test]
	fn test_dot ( )
	{
		let q_1 = Quaternion{w: 0.1, x: 2.0, y: 30.0, z: 400.0};
		let q_2 = Quaternion{w: 1.1, x: 0.02, y: 0.003, z: 0.0004};
		
		assert_eq!(q_1.dot(q_2), 0.4);
	}
}
