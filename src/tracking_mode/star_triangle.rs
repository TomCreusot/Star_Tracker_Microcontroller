//! Implementation of StarTriangle
use super::StarTriangle;
// use super::StarPair;
use crate::util::aliases::Decimal;
use crate::util::units::Cartesian3D;

/// A specularity test.
/// Ignore means the area was too small.
/// Valid means that the sign is a valid way of checking specularity in the current triangle.
#[derive(Debug)]
#[derive(PartialEq)]
enum Specularity
{
	Ignore,
	Valid(bool)
}
/*

impl StarTriangle<Equatorial>
{

	/// Checks if the provided triangle exists.
	/// This is done by:
	/// * Finding every pair in the database that matches the provided pair.
	/// * Looping through each set of pairs until a match is found where all stars are connected (and not the same).
	/// * Matching each star to the corresponding database star.
	/// * Checking the specularity of the triangle is the same or small.
	///
	/// # Arguments
	/// * `triangle` - The star triangle.
	/// # Returns
	/// An optional which is none if the triangle is invalid.
	/// Returns the positions of the stars from the database if found.
	fn verify_triangle ( triangle : StarTriangle ) -> Option<StarTriangle>
	{
		// Input StarPair, output List<StarPair> from database.
		let a_b = database.searcj(&StarPair(a, b));
		let b_c = database.verify_pair(&StarPair(b, c));
		let a_c = database.verify_pair(&StarPair(a, c));

		if a_b.is_none() || b_c.is_none() || a_c.is_none()
		{
			return None;
		}

		// a must be touching b which must be touching c.
		// a_b could be (a, b) or (b, a), until confirmed, this is (x, y).
		for ii in 0..a_b.unwrap().size()
		{
			for jj in 0..b_c.unwrap().size()
			{
				for kk in 0..a_c.unwrap().size()
				{
					// Find which star belongs to which point on the triangle.
					// Convert StarPair to Equatorial.
					let a = similar_both(&a_b.unwrap().get(i), &a_c.unwrap().get(i));
					let b = similar_both(&a_b.unwrap().get(i), &b_c.unwrap().get(i));
					let c = similar_both(&a_c.unwrap().get(i), &b_c.unwrap().get(i));

					// Checks if a triangle is made.
					if !(a.is_none() && b.is_none() && c.is_none())
					{
						let triangle_database = StarTriangle(a, b, c);
						let s1 = get_specular(triangle);
						let s2 = get_specular(triangle_database);

						if ( s1 == Specularity.Ignore || s2 == Specularity.Ignore || s1 == s2 )
						{
							return StarTriangle(a, b, c);
						}
					}
				}
			}
		}
		return None;
	}
}
*/




impl StarTriangle<Cartesian3D>
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
	/// use star_tracker::util::units::Cartesian3D;
	/// use star_tracker::util::aliases::Decimal;
	/// let pt1 = Cartesian3D { x: -1.0, y: 2.0, z: 3.0 };
	/// let pt2 = Cartesian3D { x: 4.0, y: 5.0, z: 6.0 };
	/// let pt3 = Cartesian3D { x: 7.0, y: 8.0, z: -9.0 };
	///
	/// const specularity_min : Decimal = 239.0;
	/// // pt2 x pt3 = (-93, 78, -3) . pt1 = 240 = true
	/// let st1 : StarTriangle<Cartesian3D> = StarTriangle(pt1.clone(), pt2.clone(), pt3.clone() );
	/// // pt2 x pt3 = (93, -78, 3) . pt1 = -240 = false
	/// let st2 : StarTriangle<Cartesian3D> = StarTriangle(pt1.clone(), pt3.clone(), pt2.clone() );
	/// // pt1 x pt3 = (-42, 12, -22) . pt2 = -240 = false
	/// let st3 : StarTriangle<Cartesian3D> = StarTriangle(pt2.clone(), pt1.clone(), pt3.clone() );
	///
	/// // st1.get_specular(specularity_min); // Specularity::Valid(true);
	/// // st2.get_specular(specularity_min); // Specularity::Valid(false);
	/// // st3.get_specular(specularity_min); // Specularity::Valid(false);
	/// // st3.get_specular(specularity_min); // Specularity::Ignore;
	/// ```
	fn get_specular ( &self, specularity_min : Decimal ) -> Specularity
	{
		let cross = self.1.cross(&self.2).dot(&self.0);
		if cross.abs() < specularity_min
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
	use crate::tracking_mode::star_triangle::Specularity;
	// use crate::tracking_mode::star_triangle::Specularity;
	// use crate::util::coordinates::Equatorial;
	use crate::util::units::Cartesian3D;
	use crate::util::aliases::Decimal;


	#[test]
	fn test_verify_triangle ( )
	{
		
	}


	#[test]
	fn test_get_specular_valid_size ( )
	{
		let pt1 = Cartesian3D { x: -1.0, y: 2.0, z: 3.0 };
		let pt2 = Cartesian3D { x: 4.0, y: 5.0, z: 6.0 };
		let pt3 = Cartesian3D { x: 7.0, y: 8.0, z: -9.0 };

		const SPECULARITY_MIN : Decimal = 239.0;

		// pt2 x pt3 = (-93, 78, -3) . pt1 = 240 = true
		let st1 : StarTriangle<Cartesian3D> = StarTriangle(pt1.clone(), pt2.clone(), pt3.clone() );
		// pt2 x pt3 = (93, -78, 3) . pt1 = -240 = false
		let st2 : StarTriangle<Cartesian3D> = StarTriangle(pt1.clone(), pt3.clone(), pt2.clone() );
		// pt1 x pt3 = (-42, 12, -22) . pt2 = -240 = false
		let st3 : StarTriangle<Cartesian3D> = StarTriangle(pt2.clone(), pt1.clone(), pt3.clone() );

		if let Specularity::Valid(spec1) = st1.get_specular(SPECULARITY_MIN)
		{
			assert!(spec1);
		}
		else
		{
			panic!("Spec1 was Invalid?");
		}
		if let Specularity::Valid(spec2) = st2.get_specular(SPECULARITY_MIN)
		{
			assert!(!spec2);
		}
		else
		{
			panic!("Spec2 was invalid?");
		}
		if let Specularity::Valid(spec3) = st3.get_specular(SPECULARITY_MIN)
		{
			assert!(!spec3);
		}
		else
		{
			panic!("Spec3 was invalid?");
		}
	}


	#[test]
	fn test_get_specular_ignore ( )
	{
		let pt1 = Cartesian3D { x: -1.0, y: 2.0, z: 3.0 };
		let pt2 = Cartesian3D { x: 4.0, y: 5.0, z: 6.0 };
		let pt3 = Cartesian3D { x: 7.0, y: 8.0, z: -9.0 };

		const SPECULARITY_MIN : Decimal = 241.0;

		// pt2 x pt3 = (-93, 78, -3) . pt1 = 240 = true
		let st1 : StarTriangle<Cartesian3D> = StarTriangle(pt1.clone(), pt2.clone(), pt3.clone() );
		// pt2 x pt3 = (93, -78, 3) . pt1 = -240 = false
		let st2 : StarTriangle<Cartesian3D> = StarTriangle(pt1.clone(), pt3.clone(), pt2.clone() );
		// pt1 x pt3 = (-42, 12, -22) . pt2 = -240 = false
		let st3 : StarTriangle<Cartesian3D> = StarTriangle(pt2.clone(), pt1.clone(), pt3.clone() );

		assert_eq!(st1.get_specular(SPECULARITY_MIN), Specularity::Ignore);
		assert_eq!(st2.get_specular(SPECULARITY_MIN), Specularity::Ignore);
		assert_eq!(st3.get_specular(SPECULARITY_MIN), Specularity::Ignore);
	}

}
