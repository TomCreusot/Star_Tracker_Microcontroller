//! This is the implementation of Quaternion.
use crate::util::{aliases::Decimal, coordinates::{Quaternion, Cartesian3D}};
use std::ops::Mul;


impl Quaternion
{
	/// Constructor with cartesian angle axis.
	/// The quaternion describes the rotation multiplier to acheive the angle axis rotation.
	/// # Arguments
	/// * `angle` - The angle to rotate.
	/// * `axis` - The axis to rotate around.
	/// 
	/// # Returns
	/// A quaternion for a rotation around the angle axis.
	///
	/// # Example
	/// ```
	/// use star_tracker::util::coordinates::{Quaternion, Cartesian3D};
	/// use std::f32::consts::PI;
	/// let angle = PI / 2.0;
	/// let axis = Cartesian3D{x: 1.0, y: 0.0, z: 0.0};
	/// let q = Quaternion::new(angle, axis);
	/// assert_eq!(q.w, (angle / 2.0).cos());
	/// assert_eq!(q.x, (angle / 2.0).sin() * axis.x);
	/// assert_eq!(q.y, 0.0);
	/// assert_eq!(q.z, 0.0);
	/// ```
	pub fn new ( angle: Decimal, axis: Cartesian3D<Decimal> ) -> Quaternion
	{
		let magnitude = axis.x*axis.x + axis.y*axis.y + axis.z*axis.z;
		assert!((magnitude - 1.0) < 0.0001, "Not unit vector");
		return Quaternion { 
			w: (angle / 2.0).cos(),
			x: (angle / 2.0).sin() * axis.x, 
			y: (angle / 2.0).sin() * axis.y, 
			z: (angle / 2.0).sin() * axis.z, 
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
	/// use star_tracker::util::coordinates::{Quaternion, Cartesian3D};
	/// use std::f32::consts::PI;
	/// // Rotates point around the z axis where the point is 10 degrees x of z
	/// let axis = Cartesian3D{x: 0.0, y: 0.0, z: 1.0};
	/// let offset_angle = PI / 18.0; // 10 degrees x of z.
	/// let angle = PI / 4.0; // 45 degrees
	/// let pt = Cartesian3D{x: (offset_angle).sin(), y: 0.0, z: (offset_angle).cos()};
	/// let q = Quaternion::new(angle, axis);
	/// let rotated = q.rotate_point(pt);
	/// let expected = Cartesian3D{	
	/// 						x: (offset_angle).sin() * (PI/2.0 + angle).sin(),
	/// 						y: (offset_angle).sin() * (PI/2.0 + angle).cos(), 
	/// 						z: pt.z}; 
	/// assert!((rotated.x - expected.x).abs() < 0.0001);
	/// assert!((rotated.y - expected.y).abs() < 0.0001);
	/// assert!((rotated.z - expected.z).abs() < 0.0001);
	/// ```
	pub fn rotate_point ( self, pt: Cartesian3D<Decimal> ) -> Cartesian3D<Decimal>
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
	/// use star_tracker::util::coordinates::Quaternion;
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
} 



impl Mul for Quaternion
{
	type Output = Self;
	/// Finds the Hamilton Product.
	/// # Example
	/// ``` 
	/// use star_tracker::util::coordinates::Quaternion;
	/// let q = Quaternion{w: 1.0, x: 2.0, y: 3.0, z: 4.0};
	/// let r = Quaternion{w: 4.0, x: 3.0, y: 2.0, z: 1.0};
	/// assert_eq!(q * r, Quaternion{w: -12.0, x: 6.0, y: 24.0, z: 12.0});
	/// ```
	fn mul ( self, rhs: Self ) -> Self
	{
		let x = self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y;
		let y = self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x;
		let z = self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w;
		let w = self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z;
		return Quaternion{w: w , x: x, y: y, z: z};
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
	use util::coordinates::{Quaternion, Equatorial, Cartesian3D};
	use util::aliases::Decimal;

	fn assert_close ( a: Decimal, b: Decimal )
	{
		if (a - b).abs() > 0.00001
		{
			panic!("\n\nassert_close failed: \n\tleft: `{}`\n\tright: `{}`\n\n", a, b);
		}
	}	
	fn assert_close_cartesian ( a: Cartesian3D<Decimal>, b: Cartesian3D<Decimal> )
	{
		assert_close(a.x, b.x);
		assert_close(a.y, b.y);
		assert_close(a.z, b.z);
	}	

	//
	//  new <T: Cartesian3D>( angle: Decimal, axis: T ) -> Quaternion
	//
	#[test]
	fn test_new ( )
	{
		let angle = PI / 2.0;
		let axis = Cartesian3D{x: 1.0, y: 0.0, z: 0.0};
		let q = Quaternion::new(angle, axis);
		assert_close(q.w, (angle / 2.0).cos());
		assert_close(q.x, (angle / 2.0).sin() * axis.x);
		assert_close(q.y, 0.0);
		assert_close(q.z, 0.0);
	}

	#[test]
	#[should_panic = "Not unit vector"]
	#[allow(unused_variables)]
	fn test_new_panic ( )
	{
		let angle = PI;
		let axis = Cartesian3D{x: 1.0, y: 1.0, z: 1.0};
		let q = Quaternion::new(angle, axis);
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
	//        /
	//     +x 
	
	
	
	//
	// rotate_point ( Cartesian3D<Decimal> ) -> Cartesian3D<Decimal>
	//
	#[test]
	fn test_rotate_point_x ( )
	{
		let axis = Cartesian3D{x: 1.0, y: 0.0, z: 0.0};
		let mut angle = 0.0;
		let mut q = Quaternion::new(angle, axis);
		
		let mut pt : Cartesian3D<Decimal> = Cartesian3D{x: 0.0, y: 0.0, z: 1.0};
		let mut rotated = q.rotate_point(pt);
		let mut expected : Cartesian3D<Decimal> = Cartesian3D{x: 0.0, y: 0.0, z: 1.0};
		assert_close_cartesian(expected, rotated);
		
		angle = PI / 2.0;
		q = Quaternion::new(angle, axis);
		rotated = q.rotate_point(pt);
		expected = Cartesian3D{x: 0.0, y: 1.0, z: 0.0};
		assert_close_cartesian(expected, rotated);
		
		angle = PI;
		q = Quaternion::new(angle, axis);
		rotated = q.rotate_point(pt);
		expected = Cartesian3D{x: 0.0, y: 0.0, z: -1.0};
		assert_close_cartesian(expected, rotated);
		
		let offset_angle = PI / 18.0; // 10 degrees z of x.
		angle = PI / 4.0; // 45 degrees
		pt = Cartesian3D{x: (offset_angle).cos(), y: (offset_angle).sin(), z: 0.0};
		q = Quaternion::new(angle, axis);
		rotated = q.rotate_point(pt);
		expected = Cartesian3D{	x: pt.x, 
								y: (offset_angle).sin() * (PI/2.0 + angle).sin(), 
								z: (offset_angle).sin() * (PI/2.0 + angle).cos()};
		assert_close_cartesian(expected, rotated);
	}

	#[test]
	fn test_rotate_point_y ( )
	{
		let axis = Cartesian3D{x: 0.0, y: 1.0, z: 0.0};
		let mut angle = 0.0;
		let mut q = Quaternion::new(angle, axis);
		
		let mut pt : Cartesian3D<Decimal> = Cartesian3D{x: 0.0, y: 0.0, z: 1.0};
		let mut rotated = q.rotate_point(pt);
		let mut expected : Cartesian3D<Decimal> = Cartesian3D{x: 0.0, y: 0.0, z: 1.0};
		assert_close_cartesian(expected, rotated);
		
		angle = PI / 2.0;
		q = Quaternion::new(angle, axis);
		rotated = q.rotate_point(pt);
		expected = Cartesian3D{x: -1.0, y: 0.0, z: 0.0};
		assert_close_cartesian(expected, rotated);
		
		angle = PI;
		q = Quaternion::new(angle, axis);
		rotated = q.rotate_point(pt);
		expected = Cartesian3D{x: 0.0, y: 0.0, z: -1.0};
		assert_close_cartesian(expected, rotated);
		
		let offset_angle = PI / 18.0; // 10 degrees z of y.
		angle = PI / 4.0; // 45 degrees
		pt = Cartesian3D{x: 0.0, y: (offset_angle).cos(), z: (offset_angle).sin()};
		q = Quaternion::new(angle, axis);
		rotated = q.rotate_point(pt);
		expected = Cartesian3D{	x: (offset_angle).sin() * (PI/2.0 + angle).cos(), 
								y: pt.y, 
								z: (offset_angle).sin() * (PI/2.0 + angle).sin()};
		assert_close_cartesian(expected, rotated);
	}
	
	#[test]
	fn test_rotate_point_z ( )
	{
		let axis = Cartesian3D{x: 0.0, y: 0.0, z: 1.0};
		let mut angle = 0.0;
		let mut q = Quaternion::new(angle, axis);
		
		let mut pt : Cartesian3D<Decimal> = Cartesian3D{x: 1.0, y: 0.0, z: 0.0};
		let mut rotated = q.rotate_point(pt);
		let mut expected : Cartesian3D<Decimal> = Cartesian3D{x: 1.0, y: 0.0, z: 0.0};
		assert_close_cartesian(expected, rotated);
		
		angle = PI / 2.0;
		q = Quaternion::new(angle, axis);
		rotated = q.rotate_point(pt);
		expected = Cartesian3D{x: 0.0, y: -1.0, z: 0.0};
		assert_close_cartesian(expected, rotated);
		
		angle = PI;
		q = Quaternion::new(angle, axis);
		rotated = q.rotate_point(pt);
		expected = Cartesian3D{x: -1.0, y: 0.0, z: 0.0};
		assert_close_cartesian(expected, rotated);
		
		let offset_angle = PI / 18.0; // 10 degrees x of z.
		angle = PI / 4.0; // 45 degrees
		pt = Cartesian3D{x: (offset_angle).sin(), y: 0.0, z: (offset_angle).cos()};
		q = Quaternion::new(angle, axis);
		rotated = q.rotate_point(pt);
		expected = Cartesian3D{	
								x: (offset_angle).sin() * (PI/2.0 + angle).sin(),
								y: (offset_angle).sin() * (PI/2.0 + angle).cos(), 
								z: pt.z}; 
		assert_close_cartesian(expected, rotated);
	}

}
