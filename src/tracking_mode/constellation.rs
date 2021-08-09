//! Implementation of Constellation.
// use crate::util::units::Cartesian3D;
use crate::util::units::Equatorial;
use crate::util::list::List;
use crate::util::list::ArrayList;
use crate::tracking_mode::database::Database;
use super::TriangleConstruct;
use super::StarTriangle;
use super::Constellation;
use super::Match;


impl Constellation
{
    /// Creates unique sets of TrackingMode's from the location of the stars on an equatorial plane.
	/// These are then compared with the database and the accurate sets from the database will be returned.
	/// # Arguments
	/// * `stars` - The list of stars in order of magnitude (descending).
	///
	/// # Returns
	/// The triangle of the image and the triangle of the database.
	///
	/// # Example
	/// ```
	/// panic!("NYI");
	/// ```
	fn new <const TRI_SIZE: usize> ( 
										stars : &dyn List<Equatorial>, 
										database : &dyn Database, 
										gen : &dyn TriangleConstruct 
									) -> Constellation
    {
		// Not enough stars, instant fail.
		if stars.size() < 3
		{
			return Constellation::None;
		}
		// Enough to make a triangle constellation.
		else if stars.size() == 3
		{
			let mut triangles: ArrayList<Match<StarTriangle<usize>>, TRI_SIZE> = ArrayList::new();
			gen.find_match_triangle ( stars, database, &mut triangles );
			if triangles.size() > 0
			{
				let input = triangles.get(0).output.search_list(stars);
				let output = triangles.get(0).input.search_database(database);
				assert!(input.is_ok());	assert!(output.is_ok());
				return Constellation::Triangle(Match{input:input.unwrap(),output:output.unwrap()});
			}
			else
			{
				return Constellation::None;
			}
		}
		return Constellation::None;
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
	use std::mem;
	
	use crate::util::units::Cartesian3D;
	use crate::util::units::Equatorial;
	use crate::util::units::Radians;
	// use crate::util::list::List;
	use super::Constellation;
	use super::StarTriangle;
	use super::Match;
	use crate::tracking_mode::MockTriangleConstruct;
	use crate::tracking_mode::database::MockDatabase;
	// use crate::tracking_mode::database::Database;
	
	
	fn compare_triangle_eq ( 
		input: StarTriangle<Equatorial>, 
		output: StarTriangle<Equatorial> )
	{
		assert!((output.0.ra  - input.0.ra).0.abs()  < 0.01);
		assert!((output.0.dec - input.0.dec).0.abs() < 0.01);
		assert!((output.1.ra  - input.1.ra).0.abs()  < 0.01);
		assert!((output.1.dec - input.1.dec).0.abs() < 0.01);
		assert!((output.2.ra  - input.2.ra).0.abs()  < 0.01);
		assert!((output.2.dec - input.2.dec).0.abs() < 0.01);	
	}
	
	
	// use super::StarTriangle;
	
	#[test]
	// If there is less than 3 stars input, None should be returned.
	fn test_new_invalid_number_stars ( )
	{
		let a = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let mut stars : Vec<Equatorial> = vec![];
		const TRIANGLE_SIZE : usize = 10;
		let mock_g = MockTriangleConstruct::new();
		let mock_d = MockDatabase::new();
		// mock.expect_find_match_triangle
		
		assert_eq! ( mem::discriminant(&Constellation::new::<TRIANGLE_SIZE>(&stars, &mock_d, &mock_g)), 
					 mem::discriminant(&Constellation::None) );
		
		stars = vec![a];
		assert_eq! ( mem::discriminant(&Constellation::new::<TRIANGLE_SIZE>(&stars, &mock_d, &mock_g)), 
					 mem::discriminant(&Constellation::None) );
		
		stars = vec![a, a];
		assert_eq! ( mem::discriminant(&Constellation::new::<TRIANGLE_SIZE>(&stars, &mock_d, &mock_g)), 
					 mem::discriminant(&Constellation::None) );
	}
	



	#[test]
	// If there is 3 stars and a triangle cannot be formed, None should be returned.
	fn test_new_invalid_3_stars ( )
	{
		let a = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let mut stars : Vec<Equatorial> = vec![a, a, a];
		const TRIANGLE_SIZE : usize = 10;
		let mut mock_g = MockTriangleConstruct::new();
		let mock_d = MockDatabase::new();
		
		// mock_g.expect_find_match_triangle().withf(|triangles : &mut dyn List<StarTriangle<Cartesian3D>>| triangles = ArrayList<>
		
		mock_g.expect_find_match_triangle().times(1).returning(|_, _, _| ());
		
		assert_eq! ( mem::discriminant(&Constellation::new::<TRIANGLE_SIZE>(&stars, &mock_d, &mock_g)), 
					 mem::discriminant(&Constellation::None) );
	}
	


	#[test]
	// If there is 3 stars and a triangle can be formed, a triangle should be returned.
	fn test_new_valid_3_stars ( )
	{
		const TRIANGLE_SIZE : usize = 10;
		let mut mock_g = MockTriangleConstruct::new();
		let mut mock_d = MockDatabase::new();
		
		// These are points from the image referenced by TRI_IN
		let a = Equatorial{ra: Radians(1.0), dec: Radians(0.1)};
		let b = Equatorial{ra: Radians(2.0), dec: Radians(0.2)};
		let c = Equatorial{ra: Radians(3.0), dec: Radians(0.3)};
		let stars : Vec<Equatorial> = vec![a, b, c];
		
		// The software function will call find_match_triangle;
		// It will return the triangle from the image (TRI_IN) with the matched database triangle (TRI_OUT).
		// This will then be converted to the equatorial coordinates from the reference locations.
		static TRI_IN  : StarTriangle<usize> = StarTriangle( 0, 1, 2 );
		static TRI_OUT : StarTriangle<usize> = StarTriangle( 0, 1, 2 );
		static MATCH_TRI : Match<StarTriangle<usize>> = Match{input: TRI_IN, output: TRI_OUT};
		
		mock_g.expect_find_match_triangle().times(1)
				.returning(|_, _, triangles| (*triangles).push_back(MATCH_TRI));
		
		// The output for converting the TRI_OUT to equatorial (searching database).
		static STAR_OUT : [Equatorial ; 3] =
			[Equatorial{ra: Radians(111.2), dec: Radians(222.3)},
			Equatorial{ra: Radians(11.2), dec: Radians(22.3)},
			Equatorial{ra: Radians(1.2), dec: Radians(2.3)}];
		mock_d.expect_find_star().times(3).returning(|i| Ok(STAR_OUT[i]));
		
		let triangle = &Constellation::new::<TRIANGLE_SIZE>(&stars, &mock_d, &mock_g);
		if let Constellation::Triangle(t) = triangle
		{
			compare_triangle_eq(t.input, StarTriangle(stars[0], stars[1], stars[2]));
			compare_triangle_eq(t.output, StarTriangle(STAR_OUT[0], STAR_OUT[1], STAR_OUT[2]));
		}
		else
		{
			panic!("Invalid Triangle: {:?}", triangle);
		}
	}













	#[test]
	// If there is 4 stars and a triangle cannot be formed, None should be returned.
	fn test_new_invalid_4_stars ( )
	{
		let a = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let stars : Vec<Equatorial> = vec![a, a, a, a];
		const TRIANGLE_SIZE : usize = 10;
		let mut mock_g = MockTriangleConstruct::new();
		let mock_d = MockDatabase::new();
		
		// mock_g.expect_find_match_triangle().withf(|triangles : &mut dyn List<StarTriangle<Cartesian3D>>| triangles = ArrayList<>
		
		mock_g.expect_find_match_triangle().returning(|_, _, triangles| ());
		
		assert_eq! ( mem::discriminant(&Constellation::new::<TRIANGLE_SIZE>(&stars, &mock_d, &mock_g)), 
					 mem::discriminant(&Constellation::None) );
	}
	




/*

	#[test]
	// If there is 4 stars and a triangle can be formed, a star pyramid should be created.
	// The final star should be
	fn test_new_valid_4_stars (  )
	{
		let a = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let mut stars : Vec<Equatorial> = vec![a, a, a, a];
		const TRIANGLE_SIZE : usize = 10;
		let mut mock_g = MockTriangleConstruct::new();
		let mock_d = MockDatabase::new();
		
		static tri_1 : StarTriangle<Cartesian3D> = StarTriangle(
															Cartesian3D{x: 0.0, y: 1.1, z: 2.2}, 
															Cartesian3D{x: 3.3, y: 4.4, z: 5.5}, 
															Cartesian3D{x: 6.6, y: 7.7, z: 8.8});
															
		static tri_2 : StarTriangle<Cartesian3D> = StarTriangle(
															Cartesian3D{x: 0.0, y: 1.1, z: 2.2}, 
															Cartesian3D{x: 3.3, y: 4.4, z: 5.5}, 
															Cartesian3D{x: 6.6, y: 7.7, z: 8.8});
		mock_g.expect_find_match_triangle()
				.returning(|stars, database, triangles| (*triangles).push_back(tri));
		
		let triangle = &Constellation::new::<TRIANGLE_SIZE>(&stars, &mock_d, &mock_g);
		if let Constellation::Triangle(t) = triangle
		{
			assert!((t.0.x - tri.0.x).abs() < 0.01);
			assert!((t.0.y - tri.0.y).abs() < 0.01);
			assert!((t.0.z - tri.0.z).abs() < 0.01);
			
			assert!((t.1.x - tri.1.x).abs() < 0.01);
			assert!((t.1.y - tri.1.y).abs() < 0.01);
			assert!((t.1.z - tri.1.z).abs() < 0.01);
			
			assert!((t.2.x - tri.2.x).abs() < 0.01);
			assert!((t.2.y - tri.2.y).abs() < 0.01);
			assert!((t.2.z - tri.2.z).abs() < 0.01);
		}
		else
		{
			panic!("Invalid Triangle: {:?}", triangle);
		}
	}*/
	
}