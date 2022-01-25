use super::AngleAxis;
use super::Quaternion;

use util::test::TestEqual;
use util::aliases::DECIMAL_PRECISION;
use std::fmt;



impl AngleAxis
{
	/// Constructs a quaternion from this angle axis.
	/// # Example
	/// ```
	/// use star_tracker::util::aliases::M_PI;
	/// use star_tracker::util::units::{Cartesian3D, AngleAxis, Radians, Quaternion};
	/// let mut axis : Cartesian3D = Cartesian3D{x: 1.0, y: 2.0, z: 3.0};
	///	let angle = Radians(M_PI / 3.1);
	///	axis.normalize();
	///	let angle_axis : AngleAxis = AngleAxis{angle: angle, axis: axis};
	///	assert_eq!(angle_axis, angle_axis.to_quaternion().to_angle_axis());
	/// ```
	pub fn to_quaternion ( &self ) -> Quaternion
	{
		assert!(self.axis.magnitude().test_close(&1.0, DECIMAL_PRECISION), "Not unit vector");
		return Quaternion {
			w: (self.angle / 2.0).0.cos(),
			x: (self.angle / 2.0).0.sin() * self.axis.x,
			y: (self.angle / 2.0).sin() * self.axis.y,
			z: (self.angle / 2.0).sin() * self.axis.z,
		};
	}

/*
	/// Outputs a number which allows you to compare the similarity between 2 angle axis rotations.  
	/// This is done by generating a point adjacent to the 2 axis.  
	/// The vector is rotated by each rotation and the angular distance is found.  
	pub fn compare ( &self, mut other: AngleAxis ) -> Radians
	{
		if 
			(self.axis.x+other.axis.x).abs() < 0.0000001 &&
			(self.axis.y+other.axis.y).abs() < 0.0000001 &&
			(self.axis.z+other.axis.z).abs() < 0.0000001 // Handle singularity
		{
			other.axis = Cartesian3D{x: -other.axis.x, y: -other.axis.y, z: -other.axis.z};
			other.angle.0 = -other.angle.0;
		}
		if self.axis.angle_distance(other.axis).0.abs() < 0.00000001
		{
			return Radians(0.0);
		}
		let mut pt = self.axis.cross(&other.axis);
		// let mut pt = Cartesian3D{x: 0.0, y: 0.0, z: 1.0};
		pt.normalize();
		let rotation_1 = *self.to_quaternion();
		let rotation_2 = other.to_quaternion();
		
		let out_1 = rotation_1.rotate_point(pt);
		let out_2 = rotation_2.rotate_point(pt);
		
		let angle = out_1.angle_distance(out_2);
		return angle;
	}
*/
}





impl fmt::Display for AngleAxis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "angle: {:?} degrees \t|\t axis: {:?}", self.angle.to_degrees(), self.axis)?;
		return Ok(());
	}
}


impl fmt::Debug for AngleAxis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return (self as &dyn fmt::Display).fmt(f);
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
	use util::units::{Cartesian3D, Radians, AngleAxis};
	
	
	
	//
	//  to_quaternion ( &self ) -> Quaternion
	//
	#[test]
	fn test_to_quaternion ( )
	{
		let angle = Radians(M_PI / 2.0);
		let axis = Cartesian3D{x: 1.0, y: 0.0, z: 0.0};
		let angle_axis = AngleAxis{angle:angle, axis: axis};
		let q = angle_axis.to_quaternion();
		assert_eq!(q.w, (angle / 2.0).cos());
		assert_eq!(q.x, (angle / 2.0).sin() * axis.x);
		assert_eq!(q.y, 0.0);
		assert_eq!(q.z, 0.0);
	}

	#[test]
	#[should_panic = "Not unit vector"]
	#[allow(unused_variables)]
	fn test_to_quaternion_panic ( )
	{
		let angle = Radians(M_PI);
		let axis = Cartesian3D{x: 1.0, y: 1.0, z: 1.0};
		let angle_axis = AngleAxis{angle:angle, axis: axis};
		let q = angle_axis.to_quaternion();
	}
	
	/*
	
	#[test]
	pub fn test_compare_angle ( )
	{
		let a_1 =AngleAxis{angle: Radians(0.00000), axis:Cartesian3D{x: 1.0, y: 0.0, z: 0.0}};
		let mut a_2 =AngleAxis{angle: Radians(0.00000), axis:Cartesian3D{x: 1.0, y: 0.1, z: 0.0}};
		a_2.axis.normalize();
		let mut prev = a_1.compare(a_2);
		
		for _i in 0..18
		{
			a_2.angle.0 += Degrees(10.0).to_radians().0;
			let curr = a_1.compare(a_2);
			assert!(prev.0.abs() < curr.0.abs());
			prev = curr;
		}
		for _i in 0..18
		{
			a_2.angle.0 += Degrees(10.0).to_radians().0;
			let curr = a_1.compare(a_2);
			assert!(curr.0.abs() < prev.0.abs());
			prev = curr;
		}
	}
*/
}