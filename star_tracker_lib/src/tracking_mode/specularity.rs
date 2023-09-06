//! Implementation for Specularity
use super::SpecularityConstruct;
use super::StarTriangle;
use super::Specularity;
use super::SpecularityResult;

use crate::util::units::Vector3;
use crate::util::aliases::Decimal;


use crate::util::Maths;



impl SpecularityConstruct for Specularity
{
	/// Returns true if the triangle is the same orientation OR a triangle is IGNORE.
	fn same ( &mut self, a: &StarTriangle<Vector3>, b: &StarTriangle<Vector3> ) -> bool
	{
		let aa = self.test(a);
		let bb = self.test(b);
		return aa == bb || aa == SpecularityResult::Ignore || bb == SpecularityResult::Ignore;
	}
}

impl Specularity
{
	/// Sets up the specularity test with an ideal value.  
	/// The value will ensure all triangles which are not close to a strait line are valid.  
	/// The ideal value used is 0.001.  
	pub fn default ( ) -> Self
	{
		return Self{specularity_min: 0.001};
	}
	
	/// You should use Specularity::default() if you don't know what your doing.  
	/// Specularity Min is the value until specularity should be ignored as the star triangle is a straight line.  
	/// The default value is 0.001 which has been hand picked.  
	/// Specularity ranges from ~0.0001 to ~0.01, if you use 1.0, the specularity step will be ignored.
	pub fn new ( specularity_min: Decimal ) -> Self
	{
		return Self{specularity_min: specularity_min};
	}




	/// Finds the specularity of the triangle.
	/// # Arguments
	/// * `triangle` - The triangle.
	/// * `specularity_min` - The minimum specularity until the triangle is too small to measure.
	///
	/// # TODO
	/// A triangle of small area (points close together or in line) may be invalid.
	/// Any error may cause a change in specularity.
	/// Area should be considered in this function at some point.
	///
	/// # Returns
	/// true if +'ve specularity.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::tracking_mode::StarTriangle;
	/// use star_tracker_lib::tracking_mode::Specularity;
	/// use star_tracker_lib::util::units::Vector3;
	/// use star_tracker_lib::util::aliases::Decimal;
	///
	/// let spec = Specularity::new(239.0);
	///
	/// let pt1 = Vector3 { x: -1.0, y: 2.0, z: 3.0 };
	/// let pt2 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
	/// let pt3 = Vector3 { x: 7.0, y: 8.0, z: -9.0 };
	///
	/// // pt2 x pt3 = (-93, 78, -3) . pt1 = 240 = true
	/// let st1 : StarTriangle<Vector3> = StarTriangle(pt1.clone(), pt2.clone(), pt3.clone() );
	/// // pt2 x pt3 = (93, -78, 3) . pt1 = -240 = false
	/// let st2 : StarTriangle<Vector3> = StarTriangle(pt1.clone(), pt3.clone(), pt2.clone() );
	/// // pt1 x pt3 = (-42, 12, -22) . pt2 = -240 = false
	/// let st3 : StarTriangle<Vector3> = StarTriangle(pt2.clone(), pt1.clone(), pt3.clone() );
	///
	/// spec.test(&st1); // SpecularityResult::Valid(true);
	/// spec.test(&st2); // SpecularityResult::Valid(false);
	/// spec.test(&st3); // SpecularityResult::Valid(false);
	/// ```
	pub fn test ( &self, triangle: &StarTriangle<Vector3> ) -> SpecularityResult
	{
		let cross = triangle.1.cross(triangle.2).dot(triangle.0);
		if cross.abs() < self.specularity_min
		{
			return SpecularityResult::Ignore;
		}
		return SpecularityResult::Valid(cross > 0.0);
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
	use crate::tracking_mode::StarTriangle;
	use crate::tracking_mode::Specularity;
	use crate::tracking_mode::SpecularityConstruct;
	use crate::tracking_mode::SpecularityResult;

	use crate::util::units::Vector3;
	use crate::util::aliases::Decimal;

	//
	// fn get_specular ( &self ) -> Specularity
	//

	#[test]
	#[no_coverage]
	fn test_get_specular_valid_size ( )
	{
		let spec = Specularity::new(239.0);

		let pt1 = Vector3 { x: -1.0, y: 2.0, z: 3.0 };
		let pt2 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
		let pt3 = Vector3 { x: 7.0, y: 8.0, z: -9.0 };

		// pt2 x pt3 = (-93, 78, -3) . pt1 = 240 = true
		let st1 : StarTriangle<Vector3> = StarTriangle(pt1.clone(), pt2.clone(), pt3.clone() );
		// pt2 x pt3 = (93, -78, 3) . pt1 = -240 = false
		let st2 : StarTriangle<Vector3> = StarTriangle(pt1.clone(), pt3.clone(), pt2.clone() );
		// pt1 x pt3 = (-42, 12, -22) . pt2 = -240 = false
		let st3 : StarTriangle<Vector3> = StarTriangle(pt2.clone(), pt1.clone(), pt3.clone() );

		if let SpecularityResult::Valid(spec1) = spec.test(&st1)
		{	assert!(spec1);					}
		else
		{	panic!("Spec1 was Invalid?");	}
		if let SpecularityResult::Valid(spec2) = spec.test(&st2)
		{	assert!(!spec2);				}
		else
		{	panic!("Spec2 was invalid?");	}
		if let SpecularityResult::Valid(spec3) = spec.test(&st3)
		{	assert!(!spec3);				}
		else
		{	panic!("Spec3 was invalid?");	}
	}




	#[test]
	fn test_get_specular_ignore ( )
	{
		let spec = Specularity::new(250.0);

		let pt1 = Vector3 { x: -1.0, y: 2.0, z: 3.0 };
		let pt2 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
		let pt3 = Vector3 { x: 7.0, y: 8.0, z: -9.0 };

		// pt2 x pt3 = (-93, 78, -3) . pt1 = 240 = true
		let st1 : StarTriangle<Vector3> = StarTriangle(pt1.clone(), pt2.clone(), pt3.clone() );
		// pt2 x pt3 = (93, -78, 3) . pt1 = -240 = false
		let st2 : StarTriangle<Vector3> = StarTriangle(pt1.clone(), pt3.clone(), pt2.clone() );
		// pt1 x pt3 = (-42, 12, -22) . pt2 = -240 = false
		let st3 : StarTriangle<Vector3> = StarTriangle(pt2.clone(), pt1.clone(), pt3.clone() );

		assert_eq!(spec.test(&st1), SpecularityResult::Ignore);
		assert_eq!(spec.test(&st2), SpecularityResult::Ignore);
		assert_eq!(spec.test(&st3), SpecularityResult::Ignore);
	}






	//
	// fn same ( &Vector3, &Vector3 ) -> bool
	//
	#[test]
	fn test_same_ignore ( )
	{
		let mut spec = Specularity::new(239.0);
		
		let pt1 = Vector3 { x: -1.0, y: 2.0, z:  3.0 };
		let pt2 = Vector3 { x: 4.0,  y: 5.0, z:  6.0 };
		let pt3 = Vector3 { x: 7.0,  y: 8.0, z: -9.0 };
		
		// pt2 x pt3 = (-93, 78, -3) . pt1 = 240 = true
		let st1 : StarTriangle<Vector3> = StarTriangle(pt1.clone(), pt2.clone(), pt3.clone() );
		// pt2 x pt3 = (93, -78, 3) . pt1 = -240 = false
		let st2 : StarTriangle<Vector3> = StarTriangle(pt1.clone(), pt3.clone(), pt2.clone() );
		// Ignore
		let st3 : StarTriangle<Vector3> = StarTriangle(pt1.clone(), pt1.clone(), pt1.clone() );
		
		assert!(spec.same(&st3, &st1));
		assert!(spec.same(&st2, &st3));
		assert!(spec.same(&st3, &st3));
	}
	
	
	#[test]
	fn test_same_valid ( )
	{
		let mut spec = Specularity::new(239.0);
		
		let pt1 = Vector3 { x: -1.0, y: 2.0, z: 3.0 };
		let pt2 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
		let pt3 = Vector3 { x: 7.0, y: 8.0, z: -9.0 };
		
		// pt2 x pt3 = (-93, 78, -3) . pt1 = 240 = true
		let st1 : StarTriangle<Vector3> = StarTriangle(pt1.clone(), pt2.clone(), pt3.clone() );
		// pt2 x pt3 = (93, -78, 3) . pt1 = -240 = false
		let st2 : StarTriangle<Vector3> = StarTriangle(pt1.clone(), pt3.clone(), pt2.clone() );
		
		assert!(spec.same(&st1, &st1));
		assert!(spec.same(&st2, &st2));
	}
	
	
	#[test]
	fn test_same_invalid ( )
	{
		let mut spec = Specularity::new(250.0);

		let pt1 = Vector3 { x: -1.0, y: 2.0, z: 3.0 };
		let pt2 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
		let pt3 = Vector3 { x: 7.0, y: 8.0, z: -9.0 };

		// pt2 x pt3 = (-93, 78, -3) . pt1 = 240 = true
		let st1 : StarTriangle<Vector3> = StarTriangle(pt1.clone(), pt2.clone(), pt3.clone() );
		// pt2 x pt3 = (93, -78, 3) . pt1 = -240 = false
		let st2 : StarTriangle<Vector3> = StarTriangle(pt1.clone(), pt3.clone(), pt2.clone() );

		assert!(spec.same(&st1, &st2));
		assert!(spec.same(&st2, &st1));
	}
}
