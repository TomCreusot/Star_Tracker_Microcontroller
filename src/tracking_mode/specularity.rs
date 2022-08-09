//! Implementation for Specularity
use super::SpecularityConstruct;
use super::StarTriangle;
use super::Specularity;

use crate::util::units::Vector3;

use crate::config::TrackingModeConsts;


impl <T: 'static> SpecularityConstruct<T> for Specularity where T: TrackingModeConsts
{
	/// Returns true if the triangle is the same orientation OR a triangle is IGNORE.
	fn same ( &mut self, a: &StarTriangle<Vector3>, b: &StarTriangle<Vector3> ) -> bool
	{
		let aa = Specularity::new::<T>(a);
		let bb = Specularity::new::<T>(b);
		return aa == bb || aa == Specularity::Ignore || bb == Specularity::Ignore;
	}
}

impl Specularity
{
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
	/// use star_tracker::tracking_mode::StarTriangle;
	/// use star_tracker::tracking_mode::Specularity;
	/// use star_tracker::util::units::Vector3;
	/// use star_tracker::util::aliases::Decimal;
	/// use star_tracker::config::TrackingModeConsts;
	///
	///	pub struct MockConfig ( );
	///	impl TrackingModeConsts for MockConfig
	///	{	const PAIRS_MAX : usize = 0; const TRIANGLES_MAX   : usize = 0;
	///		const SPECULARITY_MIN : Decimal = 239.0;
	///	}
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
	/// // SPECULARITY::new::<MockConfig>::(st1); // Specularity::Valid(true);
	/// // SPECULARITY::new::<MockConfig>::(st2); // Specularity::Valid(false);
	/// // SPECULARITY::new::<MockConfig>::(st3); // Specularity::Valid(false);
	/// // SPECULARITY::new::<MockConfig>::(st4); // Specularity::Ignore;
	/// ```
	pub fn new <T> ( triangle: &StarTriangle<Vector3> ) -> Specularity
		where T: TrackingModeConsts
	{
		let cross = triangle.1.cross(triangle.2).dot(triangle.0);
		if cross.abs() < T::SPECULARITY_MIN
		{
			return Specularity::Ignore;
		}
		return Specularity::Valid(cross > 0.0);
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

	use crate::util::units::Vector3;
	use crate::util::aliases::Decimal;

	use crate::config::TrackingModeConsts;


	//
	// fn get_specular ( &self ) -> Specularity
	//

	#[test]
	fn test_get_specular_valid_size ( )
	{

		pub struct MockConfig ( );
		impl TrackingModeConsts for MockConfig
		{	const PAIRS_MAX : usize = 0; const TRIANGLES_MAX   : usize = 0;
			const SPECULARITY_MIN : Decimal = 239.0;
		}

		let pt1 = Vector3 { x: -1.0, y: 2.0, z: 3.0 };
		let pt2 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
		let pt3 = Vector3 { x: 7.0, y: 8.0, z: -9.0 };

		// pt2 x pt3 = (-93, 78, -3) . pt1 = 240 = true
		let st1 : StarTriangle<Vector3> = StarTriangle(pt1.clone(), pt2.clone(), pt3.clone() );
		// pt2 x pt3 = (93, -78, 3) . pt1 = -240 = false
		let st2 : StarTriangle<Vector3> = StarTriangle(pt1.clone(), pt3.clone(), pt2.clone() );
		// pt1 x pt3 = (-42, 12, -22) . pt2 = -240 = false
		let st3 : StarTriangle<Vector3> = StarTriangle(pt2.clone(), pt1.clone(), pt3.clone() );

		if let Specularity::Valid(spec1) = Specularity::new::<MockConfig>(&st1)
		{	assert!(spec1);					}
		else
		{	panic!("Spec1 was Invalid?");	}
		if let Specularity::Valid(spec2) = Specularity::new::<MockConfig>(&st2)
		{	assert!(!spec2);				}
		else
		{	panic!("Spec2 was invalid?");	}
		if let Specularity::Valid(spec3) = Specularity::new::<MockConfig>(&st3)
		{	assert!(!spec3);				}
		else
		{	panic!("Spec3 was invalid?");	}
	}




	#[test]
	fn test_get_specular_ignore ( )
	{
		pub struct MockConfig ( );
		impl TrackingModeConsts for MockConfig
		{	const PAIRS_MAX : usize = 0; const TRIANGLES_MAX : usize = 0;
			const SPECULARITY_MIN : Decimal = 250.0;
		}

		let pt1 = Vector3 { x: -1.0, y: 2.0, z: 3.0 };
		let pt2 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
		let pt3 = Vector3 { x: 7.0, y: 8.0, z: -9.0 };

		// pt2 x pt3 = (-93, 78, -3) . pt1 = 240 = true
		let st1 : StarTriangle<Vector3> = StarTriangle(pt1.clone(), pt2.clone(), pt3.clone() );
		// pt2 x pt3 = (93, -78, 3) . pt1 = -240 = false
		let st2 : StarTriangle<Vector3> = StarTriangle(pt1.clone(), pt3.clone(), pt2.clone() );
		// pt1 x pt3 = (-42, 12, -22) . pt2 = -240 = false
		let st3 : StarTriangle<Vector3> = StarTriangle(pt2.clone(), pt1.clone(), pt3.clone() );

		assert_eq!(Specularity::new::<MockConfig>(&st1), Specularity::Ignore);
		assert_eq!(Specularity::new::<MockConfig>(&st2), Specularity::Ignore);
		assert_eq!(Specularity::new::<MockConfig>(&st3), Specularity::Ignore);
	}






	//
	// fn same ( &Vector3, &Vector3 ) -> bool
	//
	#[test]
	fn test_same_ignore ( )
	{
		pub struct MockConfig ( );
		impl TrackingModeConsts for MockConfig
		{	const PAIRS_MAX : usize = 0; const TRIANGLES_MAX : usize = 0;
			const SPECULARITY_MIN : Decimal = 239.0;
		}

		let pt1 = Vector3 { x: -1.0, y: 2.0, z: 3.0 };
		let pt2 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
		let pt3 = Vector3 { x: 7.0, y: 8.0, z: -9.0 };
		let mut substitute = Specularity::Ignore;

		// pt2 x pt3 = (-93, 78, -3) . pt1 = 240 = true
		let st1 : StarTriangle<Vector3> = StarTriangle(pt1.clone(), pt2.clone(), pt3.clone() );
		// pt2 x pt3 = (93, -78, 3) . pt1 = -240 = false
		let st2 : StarTriangle<Vector3> = StarTriangle(pt1.clone(), pt3.clone(), pt2.clone() );
		// Ignore
		let st3 : StarTriangle<Vector3> = StarTriangle(pt1.clone(), pt1.clone(), pt1.clone() );


		assert!(SpecularityConstruct::<MockConfig>::same(&mut substitute, &st3, &st1));
		assert!(SpecularityConstruct::<MockConfig>::same(&mut substitute, &st3, &st2));
		assert!(SpecularityConstruct::<MockConfig>::same(&mut substitute, &st3, &st3));
	}


	#[test]
	fn test_same_valid ( )
	{
		pub struct MockConfig ( );
		impl TrackingModeConsts for MockConfig
		{	const PAIRS_MAX : usize = 0; const TRIANGLES_MAX : usize = 0;
			const SPECULARITY_MIN : Decimal = 239.0;
		}

		let pt1 = Vector3 { x: -1.0, y: 2.0, z: 3.0 };
		let pt2 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
		let pt3 = Vector3 { x: 7.0, y: 8.0, z: -9.0 };
		let mut substitute = Specularity::Ignore;

		// pt2 x pt3 = (-93, 78, -3) . pt1 = 240 = true
		let st1 : StarTriangle<Vector3> = StarTriangle(pt1.clone(), pt2.clone(), pt3.clone() );
		// pt2 x pt3 = (93, -78, 3) . pt1 = -240 = false
		let st2 : StarTriangle<Vector3> = StarTriangle(pt1.clone(), pt3.clone(), pt2.clone() );

		assert!(SpecularityConstruct::<MockConfig>::same(&mut substitute, &st1, &st1));
		assert!(SpecularityConstruct::<MockConfig>::same(&mut substitute, &st2, &st2));
	}


	#[test]
	fn test_invalid ( )
	{
		pub struct MockConfig ( );
		impl TrackingModeConsts for MockConfig
		{	const PAIRS_MAX : usize = 0; const TRIANGLES_MAX : usize = 0;
			const SPECULARITY_MIN : Decimal = 250.0;
		}

		let pt1 = Vector3 { x: -1.0, y: 2.0, z: 3.0 };
		let pt2 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
		let pt3 = Vector3 { x: 7.0, y: 8.0, z: -9.0 };
		let mut substitute = Specularity::Ignore;

		// pt2 x pt3 = (-93, 78, -3) . pt1 = 240 = true
		let st1 : StarTriangle<Vector3> = StarTriangle(pt1.clone(), pt2.clone(), pt3.clone() );
		// pt2 x pt3 = (93, -78, 3) . pt1 = -240 = false
		let st2 : StarTriangle<Vector3> = StarTriangle(pt1.clone(), pt3.clone(), pt2.clone() );

		assert!(SpecularityConstruct::<MockConfig>::same(&mut substitute, &st1, &st2));
		assert!(SpecularityConstruct::<MockConfig>::same(&mut substitute, &st2, &st1));
	}
}
