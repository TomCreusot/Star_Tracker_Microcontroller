//! Implementation of StarTriangle
use super::TriangleConstruct;
use super::StarTriangle;
use super::Match;
// use super::StarPair;
// use super::StarPair;
use crate::tracking_mode::database::Database;
use crate::util::aliases::Decimal;
use crate::util::units::Cartesian3D;
use crate::util::units::Equatorial;
use crate::util::list::List;

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

impl TriangleConstruct for StarTriangle<usize>
{
	/// Finds every triangle from the provided stars which matches the database.
	///
	///
	///
	///
	///
	///
	///
	fn find_match_triangle /*<const PAIR_SIZE : usize>*/ ( 
								&self,
								stars: &dyn List<Equatorial>, 
								database: &dyn Database, 
								triangles: &mut dyn List<Match<StarTriangle<usize>>>
							)
	{
	}
}



impl StarTriangle<usize>
{
	pub fn search_database ( &self, database: &dyn Database ) -> Result<StarTriangle<Equatorial>, ()>
	{
		let a = database.find_star(self.0);
		let b = database.find_star(self.1);
		let c = database.find_star(self.2);
		
		if a.is_err() || b.is_err() || c.is_err()
		{
			return Err(());
		}
		
		return Ok(StarTriangle(a.unwrap(), b.unwrap(), c.unwrap()));
	}


	pub fn search_list ( &self, list: &dyn List<Equatorial> )->Result<StarTriangle<Equatorial>, ()>
	{
		if self.0 < list.size() && self.1 < list.size() && self.2 < list.size()
		{
			let a = list.get(self.0);
			let b = list.get(self.1);
			let c = list.get(self.2);
			
			return Ok(StarTriangle(a, b, c));
		}
		
		return Err(());
	}
}


impl StarTriangle<Equatorial>
{

}





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
	// use crate::tracking_mode::StarPair;
	use crate::tracking_mode::star_triangle::Specularity;
	// use crate::tracking_mode::star_triangle::Specularity;
	use crate::util::units::Cartesian3D;
	use crate::util::units::Equatorial;
	use crate::util::units::Radians;
	use crate::util::aliases::Decimal;
	use crate::tracking_mode::database::MockDatabase;

/*
	//
	// fn find_match ( &List<Equatorial>, &Database, &mut List<StarTriangle<usize>> )
	//

	
	#[test]
	fn find_match_triangle_not_enough_stars ( )
	{
		let lst_star : Vec<Equatorial> = vec![];
		let database = MockDatabase::new();
		let lst_tri : Vec<StarTriangle<usize>> = Vec::new();
		
		const PAIR_SIZE : usize = 10;
		
		// Size 0
		StarTriangle::find_match :: <PAIR_SIZE> ( &lst_star, &database, &lst_tri );
		assert_eq!(lst_tri.len(), 0);
		
		// Size 1
		lst_star.push(Equatorial{ra: Radians(0.0), dec: Radians(1.0)});
		StarTriangle.find_match :: <PAIR_SIZE> ( &lst_star, &database, &lst_tri );
		assert_eq!(lst_tri.len(), 0);

		// Size 2
		lst_star.push(Equatorial{ra: Radians(0.0), dec: Radians(1.0)});
		StarTriangle.find_match :: <PAIR_SIZE> ( &lst_star, &database, &lst_tri );
		assert_eq!(lst_tri.len(), 0);
	}
	



*/





	//
	// fn search_database ( &self, database: &dyn Database ) -> Result<StarTriangle<Equatorial>>
	//
	#[test]
	fn test_search_database_invalid ( )
	{
		let triangle = StarTriangle(1, 1, 1);
		let mut database = MockDatabase::new();
		database.expect_find_star().times(3).returning(|_| Err(()));
		assert!(triangle.search_database(&database).is_err());
	}

	#[test]
	fn test_search_database (  )
	{
		let triangle = StarTriangle(1, 2, 3);
		let mut database = MockDatabase::new();
		database.expect_find_star().times(3)
			.returning(|i| Ok(Equatorial{ra: Radians(i as Decimal), dec: Radians(i as Decimal + 1.0)}));
		
		let wrapped = triangle.search_database(&database);
		let triangle_eq = wrapped.expect("Returned Err()");
		assert_eq!(triangle_eq.0.ra,  Radians(1.0));
		assert_eq!(triangle_eq.0.dec, Radians(2.0));
		
		assert_eq!(triangle_eq.1.ra,  Radians(2.0));
		assert_eq!(triangle_eq.1.dec, Radians(3.0));
		
		assert_eq!(triangle_eq.2.ra,  Radians(3.0));
		assert_eq!(triangle_eq.2.dec, Radians(4.0));
	}


	//
	// fn search_list ( &self, database: &dyn Database ) -> Result<StarTriangle<Equatorial>>
	//
	#[test]
	fn test_search_list_invalid ( )
	{
		let triangle = StarTriangle(1, 2, 3);
		let eq = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let mut lst : Vec<Equatorial> = vec![];
		
		assert!(triangle.search_list(&lst).is_err());
		lst.push(eq);
		assert!(triangle.search_list(&lst).is_err());
		lst.push(eq);
		assert!(triangle.search_list(&lst).is_err());
		lst.push(eq);
		assert!(triangle.search_list(&lst).is_err());
	}
/*
	#[test]
	fn test_search_list (  )
	{
		let triangle = StarTriangle(1, 2, 3);
		let mut database = MockDatabase::new();
		database.expect_find_star().times(3)
			.returning(|i| Ok(Equatorial{ra: Radians(i as Decimal), dec: Radians(i as Decimal + 1.0)}));
		
		let res = triangle.search_database(&database);
		let triangle_eq = res.expect("Returned Err()");
		assert_eq!(triangle_eq.0.ra,  Radians(1.0));
		assert_eq!(triangle_eq.0.dec, Radians(2.0));
		
		assert_eq!(triangle_eq.1.ra,  Radians(2.0));
		assert_eq!(triangle_eq.1.dec, Radians(3.0));
		
		assert_eq!(triangle_eq.2.ra,  Radians(3.0));
		assert_eq!(triangle_eq.2.dec, Radians(4.0));
	}
	*/
	
	


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