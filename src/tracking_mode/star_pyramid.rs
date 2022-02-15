//! Implementation of StarPyramid.
use super::PyramidConstructBackEnd;
use super::PyramidConstruct;
use super::StarTriangle;
use super::StarPyramid;
use super::StarPair;
use super::Match;

use crate::tracking_mode::database::Database;

use crate::config::TrackingModeConsts;

use crate::util::units::Equatorial;
use crate::util::list::ArrayList;
use crate::util::list::List;
use crate::util::err::Errors;
use crate::util::err::Error;


impl <T: 'static> PyramidConstruct <T>  for StarPyramid<usize>
		// where T: TrackingModeConsts,
		// [(); T::PAIRS_MAX] : Sized
		where T: TrackingModeConsts, ArrayList<(), {T::PAIRS_MAX}> : Sized
{
	/// Finds the pilot 
	/// # Arguments
	/// * `stars` - The stars from the image. 
	/// * `database` - The database to lookup.
	/// * `input` - The star triangle from the input (what stars are being used).
	/// * `output` - The star triangle from the output in the same order as input.
	/// # Returns
	/// Ok(pilot) if valid.
	fn find_pilot (	
				&mut self,
				stars    : &dyn List<Equatorial>, 
				database : &dyn Database, 
				input    : StarTriangle<usize>,
				output   : StarTriangle<usize>,
			) -> Error<Match<usize>>
	{
		for ii in 0..stars.size()
		{
			if ii != input.0 && ii != input.1 && ii != input.2
			{
				let star = stars.get(ii);
				let side_a = stars.get(input.0).angle_distance(star);
				let side_b = stars.get(input.1).angle_distance(star);
				let side_c = stars.get(input.2).angle_distance(star);
				
				let mut sides_a: ArrayList<StarPair<usize>, {T::PAIRS_MAX}> = ArrayList::new();
				let mut sides_b: ArrayList<StarPair<usize>, {T::PAIRS_MAX}> = ArrayList::new();
				let mut sides_c: ArrayList<StarPair<usize>, {T::PAIRS_MAX}> = ArrayList::new();
				
				// Find the side angles to the pilot, if same for each star, it is the pilot.
				database.find_close_ref(side_a, T::ANGLE_TOLERANCE, &mut sides_a);
				database.find_close_ref(side_b, T::ANGLE_TOLERANCE, &mut sides_b);
				database.find_close_ref(side_c, T::ANGLE_TOLERANCE, &mut sides_c);

				let pilot = self.confirm_pilot(output, &mut sides_a, &sides_b, &sides_c);
				if pilot.is_some()
				{
					return Ok(Match{input: ii, output: pilot.unwrap(), weight: 1.0});
				}
			}
		}
		return Err(Errors::NoMatch);
	}
	
}


impl PyramidConstructBackEnd for StarPyramid<usize>
{
	/// Finds the index of the pilot and confirms it as valid.
	/// # Arguments
	/// * `output` - The confirmed triangle of the database.
	/// * `pair_a` - The found stars matching the distance from output.0 to pilot (consumed).
	/// * `pair_b` - The found stars matching the distance from output.1 to pilot.
	/// * `pair_c` - The found stars matching the distance from output.2 to pilot.
	/// # Returns
	/// The database catalogue index to the pilot or none if pilot could not be confirmed.
	fn confirm_pilot ( 
					&mut self,
					output: StarTriangle<usize>, 
					sides_a: &mut dyn List<StarPair<usize>>, 
					sides_b: &dyn List<StarPair<usize>>, 
					sides_c: &dyn List<StarPair<usize>> ) -> Option<usize>
	{
		// After removing, the remaining star is the pilot and hopefuly the star 0.
		sides_a.remove_diff(sides_b, StarPair::has_same);
		sides_a.remove_diff(sides_c, StarPair::has_same);
		
		// Look through all the potential (a, pilot) pairs.
		for i in 0..sides_a.size()
		{
			// The pilot is the opposite to a.
			let a_pilot = sides_a.get(i);
			let pilot_wrapped = a_pilot.find_not(output.0);
			
			if let Some(pilot) = pilot_wrapped {
				
				let b_pilot = StarPair(output.1, pilot);
				let c_pilot = StarPair(output.2, pilot);
				
				let connected_a = 
					output.has(sides_a.get(i).0) || output.has(sides_a.get(i).1);
				let connected_b = StarPair::index_of(b_pilot, sides_b).is_some();
				let connected_c = StarPair::index_of(c_pilot, sides_c).is_some();
				
				if connected_a && connected_b && connected_c
				{							
					return Some(pilot);
				}	
			}	
		}
		return None;
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
#[allow(unused_must_use)]
mod test
{
	use crate::util::list::List;
	
	use crate::tracking_mode::PyramidConstructBackEnd;
	use crate::tracking_mode::PyramidConstruct;
	use crate::tracking_mode::StarTriangle;
	use crate::tracking_mode::StarPyramid;
	use crate::tracking_mode::StarPair;
	use crate::tracking_mode::Match;
	
	use crate::tracking_mode::MockPyramidConstructBackEnd;
	use crate::tracking_mode::database::MockDatabase;
	
	use crate::util::units::Equatorial;
	use crate::util::units::Radians;
	
	use crate::util::aliases::Decimal;
	
	use crate::config::TrackingModeConsts;

//###############################################################################################//
//
//										Confirm Pilot
//
// fn confirm_pilot ( 
// 			&mut self, 
//			&output, 
//			&mut dyn List<StarPair<usize>>, 
//			&dyn List<StarPair<usize>>, &dyn List<StarPair<usize>> ) -> Option<usize>
//
//###############################################################################################//

	#[test]
	// If there are no similarities between each list (database search for each distance),
	// there is no unique pilot.
	fn test_confirm_pilot_no_similarities ( )
	{
		let mut pyr = StarPyramid(0,0,0,0);
		let output = StarTriangle(0, 6, 12);
		let mut stars_a :Vec<StarPair<usize>>=vec![StarPair(0, 1), StarPair(2, 3), StarPair(4, 5)];
		let stars_b :Vec<StarPair<usize>>=vec![StarPair(6, 7), StarPair(8, 9), StarPair(10, 11)];
		let stars_c :Vec<StarPair<usize>>=vec![StarPair(12, 13),StarPair(14, 15),StarPair(16, 17)];
		
		assert_eq!(pyr.confirm_pilot(output, &mut stars_a, &stars_b, &stars_c), None);
		assert_eq!(stars_a.size(), 0); // Should have had everything removed.
	}
	
	
	#[test]
	// If there is a unique feature with each list (a pilot was found), but a was not found in
	// the match, the result must be discarded.
	fn test_confirm_pilot_a_not_found ( )
	{
		let mut pyr = StarPyramid(0,0,0,0);
		let output = StarTriangle(100, 101, 102); // This is not part of any of the lists.
		let mut stars_a : Vec<StarPair<usize>> = vec![StarPair(0, 1),   StarPair(4, 5)];
		let stars_b     : Vec<StarPair<usize>> = vec![StarPair(6, 1),   StarPair(10, 11)];
		let stars_c     : Vec<StarPair<usize>> = vec![StarPair(12, 13), StarPair(16, 1)];
		
		assert_eq!(pyr.confirm_pilot(output, &mut stars_a, &stars_b, &stars_c), None);
		assert_eq!(stars_a.size(), 1); // Any without the pilot must be removed, only 1 is valid.
	}
	

	#[test]
	// If there is a unique feature with each list (a pilot was found), but a was not found in
	// the match, the result must be discarded.
	fn test_confirm_pilot_a_2_not_found ( )
	{
		let mut pyr = StarPyramid(0,0,0,0);
		let output = StarTriangle(100, 6, 16); // This is not part of any of the lists.
		let mut stars_a : Vec<StarPair<usize>> = vec![StarPair(0, 1),   StarPair(4, 5)];
		let stars_b     : Vec<StarPair<usize>> = vec![StarPair(6, 1),   StarPair(10, 11)];
		let stars_c     : Vec<StarPair<usize>> = vec![StarPair(12, 13), StarPair(16, 1)];
		
		assert_eq!(pyr.confirm_pilot(output, &mut stars_a, &stars_b, &stars_c), None);
		assert_eq!(stars_a.size(), 1); // Any without the pilot must be removed, only 1 is valid.
	}

	#[test]
	// If there is a unique feature with each list (a pilot was found), but b was not found in
	// the match, the result must be discarded.
	fn test_confirm_pilot_b_not_found ( )
	{
		let mut pyr = StarPyramid(0,0,0,0);
		let output = StarTriangle(0, 101, 16); // This is not part of any of the lists.
		let mut stars_a : Vec<StarPair<usize>> = vec![StarPair(0, 1),   StarPair(4, 5)];
		let stars_b     : Vec<StarPair<usize>> = vec![StarPair(6, 1),   StarPair(10, 11)];
		let stars_c     : Vec<StarPair<usize>> = vec![StarPair(12, 13), StarPair(16, 1)];
		
		assert_eq!(pyr.confirm_pilot(output, &mut stars_a, &stars_b, &stars_c), None);
		assert_eq!(stars_a.size(), 1); // Any without the pilot must be removed, only 1 is valid.
	}

	#[test]
	// If there is a unique feature with each list (a pilot was found), but c was not found in
	// the match, the result must be discarded.
	fn test_confirm_pilot_c_not_found ( )
	{
		let mut pyr = StarPyramid(0,0,0,0);
		let output = StarTriangle(0, 6, 102); // This is not part of any of the lists.
		let mut stars_a : Vec<StarPair<usize>> = vec![StarPair(0, 1),   StarPair(4, 5)];
		let stars_b     : Vec<StarPair<usize>> = vec![StarPair(6, 1),   StarPair(10, 11)];
		let stars_c     : Vec<StarPair<usize>> = vec![StarPair(12, 13), StarPair(16, 1)];
		
		assert_eq!(pyr.confirm_pilot(output, &mut stars_a, &stars_b, &stars_c), None);
		assert_eq!(stars_a.size(), 1); // Any without the pilot must be removed, only 1 is valid.
	}

	//																							//
	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Valid ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //
	//																							//

	#[test]
	// If:
	// - Star a,b and c all have a matching star (pilot).
	// - Star of the matching set, Star a,b and c have a match to the output.
	// The pilot is the matching star for Star a,b and c
	fn test_confirm_pilot_valid ( )
	{
		let mut pyr = StarPyramid(0,0,0,0);
		let output = StarTriangle(0, 6, 16); // This is not part of any of the lists.
		let mut stars_a : Vec<StarPair<usize>> = vec![StarPair(0, 1),   StarPair(4, 5)];
		let stars_b     : Vec<StarPair<usize>> = vec![StarPair(6, 1),   StarPair(10, 11)];
		let stars_c     : Vec<StarPair<usize>> = vec![StarPair(12, 13), StarPair(16, 1)];
		
		let output = pyr.confirm_pilot(output, &mut stars_a, &stars_b, &stars_c);
		assert_eq!(output, Some(1));
		assert_eq!(stars_a.size(), 1); // Any without the pilot must be removed, only 1 is valid.
	}
	
	#[test]
	// There may be multiple potential stars which pass the first checks.
	// Unfortuantly there is no way to identify which one is the correct one.
	// It should output the first valid pilot.
	fn test_confirm_pilot_valid_multiple_finds ( )
	{
		let mut pyr = StarPyramid(0,0,0,0);
		let output = StarTriangle(0, 6, 16); // This is not part of any of the lists.
		let mut stars_a : Vec<StarPair<usize>> = vec![StarPair(0, 1),   StarPair(4, 2)];
		let stars_b     : Vec<StarPair<usize>> = vec![StarPair(6, 1),   StarPair(10, 2)];
		let stars_c     : Vec<StarPair<usize>> = vec![StarPair(12, 2), StarPair(16, 1)];
		
		// There are 2 potential pilots, the program will take the first (1).
		let output = pyr.confirm_pilot(output, &mut stars_a, &stars_b, &stars_c);
		assert_eq!(output, Some(1));
		assert_eq!(stars_a.size(), 2); // Any without the pilot must be removed, only 1 is valid.
	}
	
	
	








	
//###############################################################################################//
//
//										Find Pilot
//
// fn find_pilot (	
// 			&mut self,
// 			stars    : &dyn List<Equatorial>, 
// 			database : &dyn Database, 
// 			input    : StarTriangle<usize>,
// 			output   : StarTriangle<usize> ) -> Error<Match<usize>>
//
//###############################################################################################//




	/// A mock for the sizes for the arrays, the arrays are not expected to exceed this size.
	pub struct MockConfigBig ( );
	impl TrackingModeConsts for MockConfigBig
	{
		const PAIRS_MAX       : usize = 10;
		const TRIANGLES_MAX   : usize = 10;
		const SPECULARITY_MIN : Decimal = 300.0;
	}
	
	
	#[test]
	// For a pilot to be found, there must be atleast 4 known stars in the stars list.
	fn test_find_pilot_invalid_length ( )
	{
		let star = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let mut stars : Vec<Equatorial> = vec![];
		let database = MockDatabase::new();
		let triangle = StarTriangle(0, 1, 2);
		
		
		let mut pyr : StarPyramid<usize> = StarPyramid(0,0,0,0);
		assert!(PyramidConstruct::<MockConfigBig>::
			find_pilot(&mut pyr, &stars, &database, triangle, triangle).is_err());
		stars.push(star);
		
		assert!(PyramidConstruct::<MockConfigBig>::
			find_pilot(&mut pyr, &stars, &database, triangle, triangle).is_err());
		stars.push(star);
		
		assert!(PyramidConstruct::<MockConfigBig>::
			find_pilot(&mut pyr, &stars, &database, triangle, triangle).is_err());
		stars.push(star);
	}


	
	
	#[test]
	// The pilot must not be one of the 3 found stars.
	// The loop where i is one of them should not occur.
	fn test_find_pilot_skip_loops ( )
	{
		let star = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let pilot = Equatorial{ra: Radians(1.0), dec: Radians(1.0)};
		let stars : Vec<Equatorial> = vec![star, pilot, star, pilot, star];
		let mut database = MockDatabase::new();
		let triangle = StarTriangle(0, 2, 3);
		
		let mut pyr : StarPyramid<usize> = StarPyramid(0,0,0,0);
		// This should be called 2 * 3 times as there are 3 calls per loop and 2 valid entries.
		database.expect_find_close_ref()
			.times(2 * 3) 						// Called 3 times per loop
			.returning(|_, _| ());
			// .withf(|find, _| {find.0 < 0.0}/*0.1 < find.0*/); // star and pilot.
			
		assert! ( PyramidConstruct::<MockConfigBig>::
			find_pilot(&mut pyr, &stars, &database, triangle, triangle).is_err() );
	}
	
	
	
	fn test_find_pilot_valid ( )
	{
		let star = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let pilot = Equatorial{ra: Radians(1.0), dec: Radians(1.0)};
		let stars : Vec<Equatorial> = vec![star, star, pilot, star];
		let mut database = MockDatabase::new();
		let triangle = StarTriangle(0, 1, 3);
		
		let mut pyr : StarPyramid<usize> = StarPyramid(0,0,0,0);
		let mut pyr_mock = MockPyramidConstructBackEnd::new();
			
		static OUTPUT: [StarPair<usize>; 3] = [StarPair(0, 4), StarPair(1, 4), StarPair(2, 4)];
		let mut index = 0;
		database.expect_find_close_ref()
			.times(1 * 3)
			.returning(move |_, found| {found.push_back(OUTPUT[index]); index+=1;});
		
		pyr_mock.expect_confirm_pilot()
			.times(1)
			.returning(|_, _, _, _| {return Some(4)});
			
		assert_eq! ( PyramidConstruct::<MockConfigBig>::
			find_pilot(&mut pyr, &stars, &database, triangle, triangle).expect("Err output"), 
			Match::<usize>{input: 1, output: 4, weight: 1.0} );
	}
}