//! Implementation of StarPyramid.
// use crate::util::units::Cartesian3D;
use crate::util::units::Equatorial;
use crate::util::list::List;
use crate::util::list::ArrayList;
use crate::tracking_mode::database::Database;
// use super::TriangleConstruct;
use super::StarPyramid;
use super::StarPair;
use super::StarTriangle;
// use super::Constellation;
use super::Match;



impl StarPyramid <usize>
{
	pub fn find_pilot <const PAIR_SIZE : usize> (	stars : &dyn List<Equatorial>, 
				database : &dyn Database, 
				triangle : StarTriangle<usize>  
			) -> Result<Match<usize>, ()>
	{
		for i in 0..stars.size()
		{
			if i != triangle.0 && i != triangle.1 && i != triangle.2
			{
				let star = stars.get(i);
				let side_a = stars.get(triangle.0).angle_distance(star);
				let side_b = stars.get(triangle.1).angle_distance(star);
				let side_c = stars.get(triangle.2).angle_distance(star);
				
				let mut sides_a : ArrayList<StarPair<usize>, PAIR_SIZE> = ArrayList::new();
				let mut sides_b : ArrayList<StarPair<usize>, PAIR_SIZE> = ArrayList::new();
				let mut sides_c : ArrayList<StarPair<usize>, PAIR_SIZE> = ArrayList::new();
				
				// Find the side angles to the pilot, if same for each star, it is the pilot.
				database.find_close_ref(side_a, &mut sides_a);
				database.find_close_ref(side_b, &mut sides_b);
				database.find_close_ref(side_c, &mut sides_c);

				// After removing, the remaining star is the pilot.
				sides_a.remove_diff(&sides_b, StarPair::has_same);
				sides_a.remove_diff(&sides_c, StarPair::has_same);

				
				if sides_a.size() > 0 && (sides_a.get(0).has(&triangle.0))
				{
					assert!(sides_a.size() == 1, 
						"Database is not unique enough, found multiple pyramid matches");
					let pilot = sides_a.get(0).find_not(&0).expect("Invalid Pilot");
					return Ok(Match{input: i, output: pilot});
				}		
			}
		}
		return Err(());
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
	use crate::tracking_mode::StarPyramid;
	use crate::tracking_mode::StarPair;
	// use crate::tracking_mode::star_triangle::Specularity;
	use crate::util::units::Cartesian3D;
	use crate::util::units::Equatorial;
	use crate::util::units::Radians;
	use crate::util::aliases::Decimal;
	use crate::tracking_mode::database::MockDatabase;


	//
	// fn search_database ( &self, database: &dyn Database ) -> Result<StarTriangle<Equatorial>>
	//
	#[test]
	// If the stars list has less than 4 stars, it is invalid.
	fn test_new_invalid_length ( )
	{
		let star = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let mut stars : Vec<Equatorial> = vec![];
		let database = MockDatabase::new();
		let triangle = StarTriangle(0, 1, 2);
		const MAX_ARRAY_PAIR_SIZE : usize = 10;
		
		assert! ( StarPyramid::new::<MAX_ARRAY_PAIR_SIZE>(&stars, &database, triangle).is_err());
		stars.push(star);
		assert! ( StarPyramid::new::<MAX_ARRAY_PAIR_SIZE>(&stars, &database, triangle).is_err());
		stars.push(star);
		assert! ( StarPyramid::new::<MAX_ARRAY_PAIR_SIZE>(&stars, &database, triangle).is_err());
		stars.push(star);
	}
	
	
	#[test]
	// If the database cannot identify pilot, for every candidate, it should return Error.
	// A star from the triangle must not be used.
	fn test_new_triangle_not_found ( )
	{
		let star = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let pilot = Equatorial{ra: Radians(1.0), dec: Radians(1.0)};
		let mut stars : Vec<Equatorial> = vec![star, star, star, pilot];
		let mut database = MockDatabase::new();
		let triangle = StarTriangle(0, 1, 2);
		const MAX_ARRAY_PAIR_SIZE : usize = 10;
		
		// One triangle candidate, 3 should be accessed (ONLY).
		database.expect_find_close_ref()
			.times(3)
			.returning(|_, _| ())
			.withf(|find, _| find.0 > 0.001); // Does not append any stars.
		assert! ( StarPyramid::new::<MAX_ARRAY_PAIR_SIZE>(&stars, &database, triangle).is_err() );


		// Two triangle candidate, 3, 4 should be accessed (ONLY).
		stars.push(pilot);
		database.expect_find_close_ref()
			.times(6)
			.returning(|_, _| ())
			.withf(|find, _| find.0 > 0.001); // Does not append any stars.
		assert! ( StarPyramid::new::<MAX_ARRAY_PAIR_SIZE>(&stars, &database, triangle).is_err() );
	}
	
	
	
	
	#[test]
	// If the database can identify the pilot BUT none of the stars match up,
	// The function should continue searching until it returns ERROR.
	fn test_new_database_found_invalid_matches ( )
	{
		let star = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let pilot = Equatorial{ra: Radians(1.01), dec: Radians(0.0)};
		let mut stars : Vec<Equatorial> = vec![star, star, star, pilot];
		let mut database = MockDatabase::new();
		let triangle = StarTriangle(0, 1, 2);
		const MAX_ARRAY_PAIR_SIZE : usize = 10;
		
		// An index is not repeated 3 times. Therefore no match.
		static OUTPUT : [StarPair<usize>; 3] = [StarPair(0, 1), StarPair(1, 2), StarPair(2, 3)];
		let mut index = 0;
		// One triangle candidate, 3 should be accessed (ONLY).
		database.expect_find_close_ref()
			.times(3)
			.returning(move |_, found| {found.push_back(OUTPUT[index]); index+=1;});
		assert! ( StarPyramid::new::<MAX_ARRAY_PAIR_SIZE>(&stars, &database, triangle).is_err() );		
	}
	

	#[test]
	// If the database can identify the pilot, the first match should be returned.
	fn test_new_triangle_found ( )
	{
		let star = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let pilot = Equatorial{ra: Radians(1.01), dec: Radians(0.0)};
		let mut stars : Vec<Equatorial> = vec![star, star, star, pilot];
		let mut database = MockDatabase::new();
		let triangle = StarTriangle(0, 1, 2);
		const MAX_ARRAY_PAIR_SIZE : usize = 10;
		
		// 0 is repeated 3 times, that is the pilot, Success.
		static OUTPUT : [StarPair<usize>; 3] = [StarPair(0, 1), StarPair(0, 2), StarPair(0, 3)];
		let mut index = 0;
		// One triangle candidate, 3 should be accessed (ONLY).
		database.expect_find_close_ref()
			.times(3)
			.returning(move |_, found| {found.push_back(OUTPUT[index]); index+=1;});
		assert! ( StarPyramid::new::<MAX_ARRAY_PAIR_SIZE>(&stars, &database, triangle).is_ok() );
	}
}