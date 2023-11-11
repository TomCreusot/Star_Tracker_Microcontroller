//! Implementation of StarPyramid.
use core_include::*;

use super::PyramidConstruct;
use super::StarTriangle;
use super::StarPyramid;
use super::PilotFinder;
use super::StarPair;

use crate::tracking_mode::database::ChunkIterator;
use crate::tracking_mode::database::SearchResult;

use crate::util::units::Equatorial;
use crate::util::units::Radians;
use crate::util::units::Match;
use crate::util::list::ArrayList;
use crate::util::list::List;
use crate::util::err::Errors;
use crate::util::err::Error;



/// Creates a PilotFinder using vectors.
/// This allows it to store as many arc angle matches as it needs.
#[cfg(any(feature = "nix", test))]
#[macro_export]
macro_rules! pilot_finder_vec {
	() => {
		PilotFinder{sides_a: &mut Vec::new(), sides_b: &mut Vec::new(), sides_c: &mut Vec::new()}
	}
}

/// Creates a PilotFinder using vectors.
/// This allows it to store as many arc angle matches as it needs.
#[macro_export]
macro_rules! pilot_finder_array {
	($length:expr) => {
		PilotFinder{
			sides_a: &mut ArrayList::<SearchResult, $length>::new(), 
			sides_b: &mut ArrayList::<SearchResult, $length>::new(), 
			sides_c: &mut ArrayList::<SearchResult, $length>::new()}
	}
}

impl<'a> PyramidConstruct for PilotFinder<'a>
{
	/// Finds the pilot 
	/// # Arguments
	/// * `stars` - The stars from the image. 
	/// * `database` - The database to lookup.
	/// * `angle_tolerance` - How much error a star pair can have until it is not considered the same.
	/// 	Used for searching the database.
	/// * `input` - The star triangle from the input (what stars are being used).
	/// * `output` - The star triangle from the output in the same order as input.
	/// # Returns
	/// Ok(pilot) if valid.
	fn find_pilot (	
				&mut self,
				stars          : &dyn List<Equatorial>, 
				database       : &dyn ChunkIterator, 
				angle_tolerance: Radians,
				input          : StarTriangle<usize>,
				output         : StarTriangle<usize>,
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
			
				self.sides_a.clear();
				self.sides_b.clear();
				self.sides_c.clear();

				// Find the side angles to the pilot, if same for each star, it is the pilot.
				database.find_close_ref_region(side_a, angle_tolerance, self.sides_a);
				database.find_close_ref_region(side_b, angle_tolerance, self.sides_b);
				database.find_close_ref_region(side_c, angle_tolerance, self.sides_c);

				let pilot = self.confirm_pilot(output);

				if pilot.is_some()
				{
					return Result::Ok(Match{input: ii, output: pilot.unwrap(), weight: 1.0});
				}
			}
		}
		return Result::Err(Errors::NoMatch);
	}
}




impl<'a> PilotFinder<'a>
{
/// Finds the index of the pilot and confirms it as valid.
/// # Arguments
/// * `output` - The confirmed triangle of the database.
/// * `pair_a` - The found stars matching the distance from output.0 to pilot (consumed).
/// * `pair_b` - The found stars matching the distance from output.1 to pilot.
/// * `pair_c` - The found stars matching the distance from output.2 to pilot.
/// # Returns
/// The database catalogue index to the pilot or none if pilot could not be confirmed.
fn confirm_pilot ( &mut self, output: StarTriangle<usize> ) -> Option<usize>
{
	// After removing, the remaining star is the pilot and hopefully the star 0.
	self.sides_a.remove_diff(self.sides_b, SearchResult::has_same_star);
	self.sides_a.remove_diff(self.sides_c, SearchResult::has_same_star);
	
	// Look through all the potential (a, pilot) pairs.
	for i in 0..self.sides_a.size()
	{
		// The pilot is the opposite to a.
		let a_pilot = self.sides_a.get(i).result;
		let pilot_wrapped = a_pilot.find_not(output.0);
		if let Some(pilot) = pilot_wrapped 
		{
			let b_pilot = StarPair(output.1, pilot);
			let c_pilot = StarPair(output.2, pilot);
			
			let connected_a = 
					output.has(self.sides_a.get(i).result.0) 
				|| output.has(self.sides_a.get(i).result.1);
			let connected_b = SearchResult::index_of_pair(b_pilot, self.sides_b).is_some();
			let connected_c = SearchResult::index_of_pair(c_pilot, self.sides_c).is_some();
			
			if connected_a && connected_b && connected_c
			{
				return Option::Some(pilot);
			}
		}
	}
	return Option::None;
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
	
	use crate::tracking_mode::PyramidConstruct;
	use crate::tracking_mode::StarTriangle;
	use crate::tracking_mode::StarPyramid;
	use crate::tracking_mode::StarPair;
	use crate::tracking_mode::PilotFinder;
	
	use crate::tracking_mode::database::MockDatabase;
	use crate::tracking_mode::database::ChunkIteratorNone;
	use crate::tracking_mode::database::MockChunkIterator;
	use crate::tracking_mode::database::SearchResult;
	
	use crate::util::units::Equatorial;
	use crate::util::units::Radians;
	use crate::util::units::Match;
	
	use crate::util::aliases::Decimal;
	
	// Creates a search result with error 0.
	// Input the star pair first and second pos.
	#[coverage(off)]
	fn sr ( a: usize, b: usize ) -> SearchResult
	{
		return SearchResult{result: StarPair(a, b), error: 0.0};
	}


	macro_rules! pilot_finder_vec_test {
		( $vec1:expr, $vec2:expr, $vec3:expr ) => {
		PilotFinder{sides_a: &mut $vec1, sides_b: &mut $vec2, sides_c: &mut $vec3}
		}
	}
	

//###############################################################################################//
//
//										Confirm Pilot
//
// // Note, the search result error is not used in this function.
// fn confirm_pilot ( 
// 			&mut self, 
//			&output ) -> Option<usize>
//
//###############################################################################################//

	#[test]
	// If there are no similarities between each list (database search for each distance),
	// there is no unique pilot.
	fn test_confirm_pilot_no_similarities ( )
	{
		let output = StarTriangle(0, 6, 12);
		let mut stars_a : Vec<SearchResult> = vec![sr(0, 1),   sr(2, 3),   sr(4, 5)];
		let mut stars_b : Vec<SearchResult> = vec![sr(6, 7),   sr(8, 9),   sr(10, 11)];
		let mut stars_c : Vec<SearchResult> = vec![sr(12, 13), sr(14, 15), sr(16, 17)];
		let mut finder = pilot_finder_vec_test!(stars_a, stars_b, stars_c);
		
		assert_eq!(finder.confirm_pilot(output), None);
		assert_eq!(stars_a.size(), 0); // Should have had everything removed.
	}
	
	
	#[test]
	// If there is a unique feature with each list (a pilot was found), but a was not found in
	// the match, the result must be discarded.
	fn test_confirm_pilot_a_not_found ( )
	{
		let output = StarTriangle(100, 101, 102); // This is not part of any of the lists.
		let mut stars_a : Vec<SearchResult> = vec![sr(0, 1),   sr(4, 5)];
		let mut stars_b : Vec<SearchResult> = vec![sr(6, 1),   sr(10, 11)];
		let mut stars_c : Vec<SearchResult> = vec![sr(12, 13), sr(16, 1)];
		let mut finder = pilot_finder_vec_test!(stars_a, stars_b, stars_c);
		
		assert_eq!(finder.confirm_pilot(output), None);
		assert_eq!(stars_a.size(), 1); // Any without the pilot must be removed, only 1 is valid.
	}
	
	
	#[test]
	// If there is a unique feature with each list (a pilot was found), but a was not found in
	// the match, the result must be discarded.
	fn test_confirm_pilot_a_2_not_found ( )
	{
		let output = StarTriangle(100, 6, 16); // This is not part of any of the lists.
		let mut stars_a : Vec<SearchResult> = vec![sr(0, 1),   sr(4, 5)];
		let mut stars_b : Vec<SearchResult> = vec![sr(6, 1),   sr(10, 11)];
		let mut stars_c  : Vec<SearchResult> = vec![sr(12, 13), sr(16, 1)];
		let mut finder = pilot_finder_vec_test!(stars_a, stars_b, stars_c);
		
		assert_eq!(finder.confirm_pilot(output), None);
		assert_eq!(stars_a.size(), 1); // Any without the pilot must be removed, only 1 is valid.
	}
	
	#[test]
	// If there is a unique feature with each list (a pilot was found), but b was not found in
	// the match, the result must be discarded.
	fn test_confirm_pilot_b_not_found ( )
	{
		let output = StarTriangle(0, 101, 16); // This is not part of any of the lists.
		let mut stars_a : Vec<SearchResult> = vec![sr(0, 1),   sr(4, 5)];
		let mut stars_b : Vec<SearchResult> = vec![sr(6, 1),   sr(10, 11)];
		let mut stars_c : Vec<SearchResult> = vec![sr(12, 13), sr(16, 1)];
		let mut finder = pilot_finder_vec_test!(stars_a, stars_b, stars_c);
		
		assert_eq!(finder.confirm_pilot(output), None);
		assert_eq!(stars_a.size(), 1); // Any without the pilot must be removed, only 1 is valid.
	}
	
	#[test]
	// If there is a unique feature with each list (a pilot was found), but c was not found in
	// the match, the result must be discarded.
	fn test_confirm_pilot_c_not_found ( )
	{
		let output = StarTriangle(0, 6, 102); // This is not part of any of the lists.
		let mut stars_a : Vec<SearchResult> = vec![sr(0, 1),   sr(4, 5)];
		let mut stars_b : Vec<SearchResult> = vec![sr(6, 1),   sr(10, 11)];
		let mut stars_c : Vec<SearchResult> = vec![sr(12, 13), sr(16, 1)];
		let mut finder = pilot_finder_vec_test!(stars_a, stars_b, stars_c);
		
		assert_eq!(finder.confirm_pilot(output), None);
		assert_eq!(stars_a.size(), 1); // Any without the pilot must be removed, only 1 is valid.
	}
	
	#[test]
	// On line 122: `if let Some(pilot) = pilot_wrapped`, llcov thinks this can fail.
	// This satisfies the test and makes the function impossibly safely fail.
	fn test_confirm_pilot_llcov_edge_case ( )
	{
		// For None to be returned, both stars_a need to be equal to output.0
		let output = StarTriangle(0, 6, 102);
		let mut stars_a : Vec<SearchResult> = vec![sr(0, 0)];
		let mut stars_b : Vec<SearchResult> = vec![sr(6, 0)];
		let mut stars_c : Vec<SearchResult> = vec![sr(16, 0)];
		let mut finder = pilot_finder_vec_test!(stars_a, stars_b, stars_c);
		
		assert_eq!(finder.confirm_pilot(output), None);
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
		let output = StarTriangle(0, 6, 16); // This is not part of any of the lists.
		let mut stars_a : Vec<SearchResult> = vec![sr(0, 1),   sr(4, 5)];
		let mut stars_b     : Vec<SearchResult> = vec![sr(6, 1),   sr(10, 11)];
		let mut stars_c     : Vec<SearchResult> = vec![sr(12, 13), sr(16, 1)];
		let mut finder = pilot_finder_vec_test!(stars_a, stars_b, stars_c);
		
		let output = finder.confirm_pilot(output);
		assert_eq!(output, Some(1));
		assert_eq!(stars_a.size(), 1); // Any without the pilot must be removed, only 1 is valid.
	}
	
	#[test]
	// There may be multiple potential stars which pass the first checks.
	// Unfortunately there is no way to identify which one is the correct one.
	// It should output the first valid pilot.
	fn test_confirm_pilot_valid_multiple_finds ( )
	{
		let output = StarTriangle(0, 6, 16); // This is not part of any of the lists.
		let mut stars_a : Vec<SearchResult> = vec![sr(0, 1),   sr(4, 2)];
		let mut stars_b     : Vec<SearchResult> = vec![sr(6, 1),   sr(10, 2)];
		let mut stars_c     : Vec<SearchResult> = vec![sr(12, 2), sr(16, 1)];
		let mut finder = pilot_finder_vec_test!(stars_a, stars_b, stars_c);
		
		// There are 2 potential pilots, the program will take the first (1).
		let output = finder.confirm_pilot(output);
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
// 			database : &dyn ChunkIterator, 
// 			input    : StarTriangle<usize>,
// 			output   : StarTriangle<usize> ) -> Error<Match<usize>>
//
//###############################################################################################//


	
	
	#[test]
	// For a pilot to be found, there must be at least 4 known stars in the stars list.
	fn test_find_pilot_invalid_length ( )
	{
		let star = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let mut stars : Vec<Equatorial> = vec![];
		let database = MockDatabase::new();
		let triangle = StarTriangle(0, 1, 2);
		
		let chunk  = ChunkIteratorNone::new(&database);
		let angle_tolerance = Radians(0.0001);

		let mut finder = pilot_finder_vec!();
		assert!(finder.find_pilot(&stars, &chunk, angle_tolerance, triangle, triangle).is_err());
		stars.push(star);
		
		assert!(finder.find_pilot(&stars, &chunk, angle_tolerance, triangle, triangle).is_err());
		stars.push(star);
		
		assert!(finder.find_pilot(&stars, &chunk, angle_tolerance, triangle, triangle).is_err());
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
		let angle_tolerance = Radians(0.0001);
		
		
		database.expect_find_close_ref_range()
		.times(2 * 3) 						// Called 3 times per loop
		.returning(|_, _| return 0..0);
		// .withf(|find, _| {find.0 < 0.0}/*0.1 < find.0*/); // star and pilot.
		
		let chunk  = ChunkIteratorNone::new(&database);
		
		let mut finder = pilot_finder_vec!();
		// This should be called 2 * 3 times as there are 3 calls per loop and 2 valid entries.
		
		assert! ( finder.find_pilot(&stars, &chunk, angle_tolerance, triangle, triangle).is_err() );
	}
	
	
	#[test]
	fn test_find_pilot_valid ( )
	{
		let star = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let pilot = Equatorial{ra: Radians(1.0), dec: Radians(1.0)};
		let stars : Vec<Equatorial> = vec![star, star, pilot, star];
		let triangle = StarTriangle(0, 1, 3);
		
		let angle_tolerance = Radians(0.0001);	
		
		let mut finder = pilot_finder_vec!();
		static OUTPUT: [StarPair<usize>; 3] = [StarPair(0, 4), StarPair(1, 4), StarPair(3, 4)];
		let mut index = 0;
		
		
		let mut chunk  = MockChunkIterator::new();
		chunk.expect_find_close_ref_region().times(1 * 3)
		.returning(move |_,_, found|
			{found.push_back(SearchResult{result: OUTPUT[index], error: 0.0}); index+=1;});
			
			
		assert_eq! (finder.find_pilot(&stars, &chunk, angle_tolerance, triangle, triangle)
				.expect("Err output"), 
			Match::<usize>{input: 2, output: 4, weight: 1.0} );
	}
}