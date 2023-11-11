//! Implementation of Constellation.
use core_include::*;

use super::SpecularityConstruct;
use super::TriangleConstruct;
use super::PyramidConstruct;
use super::AbandonSearch;
use super::Constellation;
use super::StarTriangle;
use super::StarPyramid;
use super::ConstellationResult;
use super::AbandonSearchFailures;
use crate::tracking_mode::database::ChunkIterator;

use crate::util::units::Equatorial;
use crate::util::units::Radians;
use crate::util::units::Match;
use crate::util::list::List;
use crate::util::aliases::Decimal;
use crate::util::err::Error;
use crate::core_include::RangeInclusive;

impl Constellation
{
/// Finds as many star matches as possible until `required` is met.
/// Similar to `find` except it uses indices and allows you to increase the accuracy past 4 stars.
/// If in the current iteration, more than `required` is found, all of the found stars will be used.
///
/// # Arguments
/// * `stars`           - The observed (image) stars.
/// * `database`        - The database storing reference
/// * `gen_tri`         - An object for generating a StarTriangle.
/// * `gen_pyr`         - An object for generating a Pyramid.
/// * `gen_spec`        - An object for checking Specularity.
/// * `angle_tolerance` - How much error a star pair can have until it is not considered the same.
/// 	Used for searching the database.
/// * `num_stars`       - The how many stars are required.  
///                       If min is reached, the current constellation will be returned.  
///                       If between min and max stars, more stars will be searched for, on max, the constellation is immediately returned and the search ends.  
///                       If min is reached, true is returned, otherwise it will continue searching.  
///                       It is recommended to use `4..=4` for 4 stars to be found.
///   
/// * `abort`           - A way of abandoning a search if it takes too long.
///     Ensure abort has been reset and started before calling this method.
///
/// # Returns
/// True if the minimum number of stars could be found.
/// False if it failed to find the minimum stars.
/// matches will have a value if 3 stars are found, even if min stars were not found.
pub fn find	(
	stars          : &dyn List<Equatorial>,
	database       : &mut dyn ChunkIterator,
	gen_tri        : &mut dyn TriangleConstruct,
	gen_spec       : &mut dyn SpecularityConstruct,
	abort          : &mut dyn AbandonSearch,
	angle_tolerance: Radians,
	num_stars      : RangeInclusive<usize>,
	matches        : &mut dyn List<Match<usize>>,
) -> ConstellationResult
{
	let mut result = ConstellationResult::ErrorNoTriangleMatch{fails: 0};
	let mut should_abort  = abort.should_abort();
	database.begin();
	gen_tri.begin(angle_tolerance, stars);

	// Loop through every possible combination of star combination using the kernel_iterator.
	// Using star_triangle_iterator to find triangle matches in the database.
	while !should_abort && let Some(iter) = gen_tri.next(stars, database)
	{
		// input and output both make triangles of the same length.
		let input  : Error<StarTriangle<Equatorial>> = iter.input.search_list(stars);
		let output : Error<StarTriangle<Equatorial>> = 
			iter.output.search_database(database.get_database());

		// ~~~~~~~~~~~~~~~~~~~~~~~~~	 A valid match was found.		~~~~~~~~~~~~~~~~~~~~
		// If the stars can be found in the database and the observed list (bug if not),
		// The code can continue.
		if input.is_ok() && output.is_ok()
		{
			let input = input.unwrap();
			let output = output.unwrap();

			// ~~~~~~~~~~~~~~~~~~~~~	 The specularity in/out match	~~~~~~~~~~~~~~~~~~~~
			// If the triangles are not flipped, the code can continue.
			if gen_spec.same(&input.to_vector3(), &output.to_vector3())
			{
				result = ConstellationResult::ErrorInsufficientPyramids{fails: *result.get_fails()};

				matches.clear();
				let _=
				matches.push_back(Match{input: iter.input.0, output: iter.output.0, weight: 1.0});
				let _=
				matches.push_back(Match{input: iter.input.1, output: iter.output.1, weight: 1.0});
				let _=
				matches.push_back(Match{input: iter.input.2, output: iter.output.2, weight: 1.0});

				// ~~~~~~~~~~~~~	Searching through pilot stars.		~~~~~~~~~~~~~~~~~~~~
				while matches.size() < *num_stars.end() && 
					let Some(pilot) = gen_tri.next_pilot(stars, database) 
				{
					let _=
					matches.push_back(Match{input: pilot.input, output: pilot.output, weight:1.0});
				}
				
				if *num_stars.start() <= matches.size()
				{ 
					return ConstellationResult::Success{fails: *result.get_fails()};
				}
			}
		}

		*result.get_fails() += 1;
		should_abort = abort.should_abort();
	}
	if should_abort
	{
		return ConstellationResult::ErrorAborted{fails: *result.get_fails()};
	}
	return result;
}
}



impl ConstellationResult
{
	/// Returns the number of fails inside any Constellation Result which can be edited.
	pub fn get_fails ( &mut self ) -> &mut usize
	{
		match self
		{
			ConstellationResult::ErrorNoTriangleMatch      { fails } => fails,
			ConstellationResult::ErrorAborted              { fails } => fails,
			ConstellationResult::ErrorInsufficientPyramids { fails } => fails,
			ConstellationResult::Success                   { fails } => fails,
		}
	}
}

impl AbandonSearchFailures
{
	/// Creates a new abandon search.  
	/// When there has been `max_failures` failures, the search will abort.
	pub fn new ( max_failures: usize ) -> Self { return Self { max: max_failures, count: 0 }; }
}

impl AbandonSearch for AbandonSearchFailures
{
	/// Calling this will increment count.
	/// If max is smaller than count, this will return true.
	fn should_abort ( &mut self ) -> bool
	{
		if self.max <= self.count
		{
			return true;
		}
		self.count += 1;

		return false;
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
	use std::ops::RangeInclusive;

	use super::Constellation;
	use super::StarTriangle;
	use super::StarPyramid;
	use super::Match;
	use crate::tracking_mode::database::MockDatabase;
	use crate::tracking_mode::database::MockChunkIterator;
	use crate::tracking_mode::database::ChunkIteratorNone;
	use crate::tracking_mode::database::ChunkIterator;
	use crate::tracking_mode::MockSpecularityConstruct;
	use crate::tracking_mode::MockTriangleConstruct;
	use crate::tracking_mode::MockPyramidConstruct;
	use crate::tracking_mode::MockAbandonSearch;
	use crate::tracking_mode::AbandonSearch;
	use crate::tracking_mode::AbandonSearchFailures;
	use crate::tracking_mode::ConstellationResult;

	use crate::util::units::Equatorial;
	use crate::util::units::Radians;
	use crate::util::aliases::Decimal;
	use crate::util::err::Errors;
	use crate::util::list::List;



	#[coverage(off)]
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

	// Returns an AbandonSearch which is mocked and will return false every time it is called.
	fn abandon_never ( ) -> MockAbandonSearch
	{
		let mut abandon = MockAbandonSearch::new(); 
		abandon.expect_should_abort().returning(||false);
		return abandon;
	}



//###############################################################################################//
//
//										AbandonSearchFailures
//
// pub fn AbandonSearchFailures::new ( usize ) -> Self
// pub fn begin        ( &mut self )
// pub fn should_abort ( &mut self ) -> bool
//
//###############################################################################################//

	#[test]
	// When new is called, the max capacity should be set correctly 
	// and the count should be at its default value.
	fn test_abandon_new ( )
	{
		let val = AbandonSearchFailures::new(10);
		assert_eq!(val.max, 10);
		assert_eq!(val.count, 0);
	}
	
	#[test]
	// The count value should be incremented until a failure occurs.
	fn test_abandon_should_abort ( )
	{
		let mut val = AbandonSearchFailures::new(3);
		assert_eq!(val.should_abort(), false);
		assert_eq!(val.max, 3);
		assert_eq!(val.count, 1);

		assert_eq!(val.should_abort(), false);
		assert_eq!(val.max, 3);
		assert_eq!(val.count, 2);

		assert_eq!(val.should_abort(), false);
		assert_eq!(val.max, 3);
		assert_eq!(val.count, 3);

		assert_eq!(val.should_abort(), true);
		assert_eq!(val.max, 3);
		assert_eq!(val.count, 3);
	}



//###############################################################################################//
//
//										ConstellationResult
//
// pub fn ConstellationResult::get_fails ( &mut self ) -> &mut usize
//
//###############################################################################################//

	#[test]
	fn test_get_fails ( )
	{
		let mut a = ConstellationResult::ErrorNoTriangleMatch{fails: 1};
		assert_eq!(*a.get_fails(), 1);
		*a.get_fails() = 11;
		assert_eq!(*a.get_fails(), 11);
		
		a = ConstellationResult::ErrorAborted{fails: 2};
		assert_eq!(*a.get_fails(), 2);
		*a.get_fails() = 22;
		assert_eq!(*a.get_fails(), 22);
		
		a = ConstellationResult::ErrorInsufficientPyramids{fails: 3};
		assert_eq!(*a.get_fails(), 3);
		*a.get_fails() = 33;
		assert_eq!(*a.get_fails(), 33);
		
		a = ConstellationResult::Success{fails: 4};
		assert_eq!(*a.get_fails(), 4);
		*a.get_fails() = 44;
		assert_eq!(*a.get_fails(), 44);
	}
	
//###############################################################################################//
//
//										find
//
// pub fn find	(
// 	stars          : &dyn List<Equatorial>,
// 	database       : &mut dyn ChunkIterator,
// 	gen_tri        : &mut dyn TriangleConstruct,
// 	gen_spec       : &mut dyn SpecularityConstruct,
// 	abort          : &dyn AbandonSearch,
// 	angle_tolerance: Radians,
// 	num_stars      : std::ops::RangeInclusive<usize>,
// 	matches        : &mut dyn List<Match<usize>>,
// ) -> bool
//
//###############################################################################################//



	#[test]
	// Checks the database is reset before anything happens.
	// Without a reset database, some regions of the sky will be missed.
	// This could result in unreliable inconsistent results. 
	fn test_find_database_reset ( )
	{
		let stars : Vec<Equatorial> = vec![];

		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_s = MockSpecularityConstruct::new();
		let mut mock_c = MockChunkIterator::new();
		let mut abandon = abandon_never();
		let angle_tolerance = Radians(0.0);
		let mut matches = Vec::<Match<usize>>::new();

		mock_c.expect_begin().times(1).returning(|| return);
		mock_t.expect_begin().times(1).returning(|_, _| return);
		mock_t.expect_next().times(1).returning(|_, _| return None);
	
		let passed = Constellation::find (&stars, &mut mock_c, &mut mock_t, &mut mock_s, &mut abandon, angle_tolerance, 0..=0, &mut matches);
		assert!(matches!(passed, ConstellationResult::ErrorNoTriangleMatch{fails: 0}));
		assert_eq!(matches.len(), 0);
	}
	
	#[test]
	// If there is only 0, 1 or 2 stars, there is not enough to find a match.
	// False should be returned with no stars added to the output list.
	fn test_find_insufficient_input_stars ( )
	{
		let a = Equatorial::north();
		let stars_0 : Vec<Equatorial> = vec![];
		let stars_1 : Vec<Equatorial> = vec![a];
		let stars_2 : Vec<Equatorial> = vec![a, a];
		
		let mock_d     = MockDatabase::new();
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_s = MockSpecularityConstruct::new();
		let mut chunk  = ChunkIteratorNone::new(&mock_d);
		let mut abandon = abandon_never();
		let angle_tolerance = Radians(0.0);
		let mut matches = Vec::<Match<usize>>::new();
		
		mock_t.expect_begin().times(1 * 3).returning(|_, _| return);
		mock_t.expect_next().times(1 * 3).returning(|_, _| return None);
		
		let passed = Constellation::find (&stars_0, &mut chunk, &mut mock_t, &mut mock_s, &mut abandon, angle_tolerance, 0..=0, &mut matches);
		assert!(matches!(passed, ConstellationResult::ErrorNoTriangleMatch{fails: 0}));

		assert_eq!(matches.len(), 0);
		
		let passed = Constellation::find (&stars_1, &mut chunk, &mut mock_t, &mut mock_s, &mut abandon, angle_tolerance, 0..=0, &mut matches);
		assert!(matches!(passed, ConstellationResult::ErrorNoTriangleMatch{fails: 0}));
		assert_eq!(matches.len(), 0);
		
		let passed = Constellation::find (&stars_2, &mut chunk, &mut mock_t, &mut mock_s, &mut abandon, angle_tolerance, 0..=0, &mut matches);
		assert!(matches!(passed, ConstellationResult::ErrorNoTriangleMatch{fails: 0}));
		assert_eq!(matches.len(), 0);
	}
	
	#[test]
	// If there is 3 or more stars but none form a triangle.
	// False should be returned with no stars added to the output list.
	fn test_find_no_triangle_match ( )
	{
		let a = Equatorial::north();
		let stars_3 : Vec<Equatorial> = vec![a, a, a];
		let stars_4 : Vec<Equatorial> = vec![a, a, a, a];
		
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_s = MockSpecularityConstruct::new();
		let mut mock_c = MockChunkIterator::new();
		let mut abandon = abandon_never();
		let angle_tolerance = Radians(0.0);
		let mut matches = Vec::<Match<usize>>::new();
		
		mock_c.expect_begin()       .times(1 * 2).returning(|| return);
		mock_t.expect_begin()       .times(1 * 2).returning(|_, _| return);
		mock_t.expect_next()        .times(1 * 2).returning(|_, _| return None);
		
		let passed = Constellation::find (&stars_3, &mut mock_c, &mut mock_t, &mut mock_s, &mut abandon, angle_tolerance, 0..=0, &mut matches);
		assert!(matches!(passed, ConstellationResult::ErrorNoTriangleMatch{fails: 0}));
		assert_eq!(matches.len(), 0);
		let passed = Constellation::find (&stars_4, &mut mock_c, &mut mock_t, &mut mock_s, &mut abandon, angle_tolerance, 0..=0, &mut matches);
		assert!(matches!(passed, ConstellationResult::ErrorNoTriangleMatch{fails: 0}));
		assert_eq!(matches.len(), 0);
	}

	
	#[test]
	// If the input or output stars were to somehow changed.
	// This should never happen...
	// It is just to satisfy llcov.
	fn test_find_edge_case ( )
	{
		let a = Equatorial::north();
		let stars : Vec<Equatorial> = vec![a, a, a];
		
		let mut mock_d = MockDatabase::new();
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_s = MockSpecularityConstruct::new();
		let mut abandon = abandon_never();
		let angle_tolerance = Radians(0.0);
		let mut matches = Vec::<Match<usize>>::new();
		
		let mut i = 0;
		mock_t.expect_begin()       .times(1).returning(|_, _| return);
		mock_d.expect_find_star()   .returning(|a| 
			if a == 0 { Ok(Equatorial::north()) } else { Err(Errors::NoMatch) });
			mock_t.expect_next().returning(move |_, _| 
				{
			if i == 0
			{
				i += 1;
				return Some(Match{
					input: StarTriangle(10, 10, 10), 
					output: StarTriangle(0, 0, 0), weight: 0.0});
				}
				else if i == 1
			{
				i += 1;
				return Some(Match{
					input: StarTriangle(0, 0, 0), 
					output: StarTriangle(1, 1, 1), weight: 0.0});
					
				}
				return None;	
			});
			let mut mock_c = ChunkIteratorNone::new(&mock_d);
			
			let passed = Constellation::find (&stars, &mut mock_c, &mut mock_t, &mut mock_s, &mut abandon, angle_tolerance, 0..=0, &mut matches);
			assert!(matches!(passed, ConstellationResult::ErrorNoTriangleMatch{fails: 2}));
			assert_eq!(matches.len(), 0);
		}

		
		
		
		
		#[test]
		// If there is only 3 stars that can be found and the min stars is 4.
		// The output list should have the stars in it and false should be returned.
		fn test_find_triangle_invalid_specularity ( )
		{
			let stars = vec![Equatorial::north(), Equatorial::south(), Equatorial::zero()];
			
			let mut mock_d = MockDatabase::new();
			let mut mock_t = MockTriangleConstruct::new();
			let mut mock_s = MockSpecularityConstruct::new();
			let mut abandon = abandon_never();
			let angle_tolerance = Radians(0.0);
			let mut matches = Vec::<Match<usize>>::new();
			
			let mut i = 0;  // 1 every iteration.
			let mut ii = 0; // 3 every iteration.
			mock_t.expect_begin().times(1).returning(|_, _| return);
			mock_d.expect_find_star().times(stars.len() * 3).returning(move |a| {
				assert_eq!(a, i);
				ii += 1;
				if 2 < ii
				{
					ii = 0;
					i+=1;
				}
				return Ok(Equatorial::north())});
				
			mock_s.expect_same().times(stars.len()).returning(move |a, b| {
				let cloned = vec![Equatorial::north(), Equatorial::south(),	Equatorial::zero()];
				let s = cloned[i].to_vector3();
				let north = Equatorial::north().to_vector3();
				assert_eq!(*a, StarTriangle(s, s, s));
				assert_eq!(*b, StarTriangle(north, north, north));
				i+=1;
				return false; });
					
			mock_t.expect_next().times(stars.len() + 1).returning(move |_, _| {
				if i < 3
				{
					let val = Some(Match{
						input: StarTriangle(i, i, i), 
						output: StarTriangle(i, i, i), weight: 0.0});
						i += 1;
						return val;
				} else { return None; }});
			
			let mut mock_c = ChunkIteratorNone::new(&mock_d);
			
			let passed = Constellation::find (&stars, &mut mock_c, &mut mock_t, &mut mock_s, &mut abandon, angle_tolerance, 0..=0, &mut matches);
			assert!(matches!(passed, ConstellationResult::ErrorNoTriangleMatch{fails: 3}));
			assert_eq!(matches.len(), 0);
		}
	
	
		
		
	#[test]
	// If there is only 3 stars that can be found and the min stars is 4.
	// The output list should have the stars in it and false should be returned.
	fn test_find_triangle_no_pyramids ( )
	{
		let stars = vec![Equatorial::north(), Equatorial::south(), Equatorial::zero()];
		let stars_count = stars.len();
		
		let mut mock_d = MockDatabase::new();
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_s = MockSpecularityConstruct::new();
		let mut abandon = abandon_never();
		let angle_tolerance = Radians(0.0);
		let mut matches = Vec::<Match<usize>>::new();
		
		let mut i = 0;  // 1 every iteration.
		mock_t.expect_begin()     .times(1).returning(|_, _| return);
		mock_t.expect_next()      .times(stars.len() + 1).returning(move |_, _| {
			if i < stars_count
			{
				let val = Some(Match{
					input: StarTriangle(i, i, i), output: StarTriangle(i, i, i), weight: 0.0});
					i += 1;
					return val;
				} else { return None; }});
				
				
				mock_d.expect_find_star() .times(stars.len() * 3).returning(|_| return Ok(Equatorial::north()));
		mock_s.expect_same()      .times(stars.len()).returning(|_, _| true);
		mock_t.expect_next_pilot().times(stars.len()).returning(|_, _| None);


		let mut mock_c = ChunkIteratorNone::new(&mock_d);
		
		let passed = Constellation::find (&stars, &mut mock_c, &mut mock_t, &mut mock_s, &mut abandon, angle_tolerance, 4..=4, &mut matches);
		let _expected = ConstellationResult::ErrorInsufficientPyramids{fails: stars_count};
		assert!(matches!(passed, _expected));
		assert_eq!(matches.len(), 3); // The last closest match
		assert_eq!(matches[0].input, 2);
		assert_eq!(matches[1].input, 2);
		assert_eq!(matches[2].input, 2);
		assert_eq!(matches[0].output, 2);
		assert_eq!(matches[1].output, 2);
		assert_eq!(matches[2].output, 2);
	}

	

	
	
	#[test]
	// If there is enough stars and a triangle is found but no pyramid match can be found.
	// The triangle should be put in the output list and false should be returned.
	fn test_find_triangle_not_enough_pyramids ( )
	{
		let stars = vec![Equatorial::north(), Equatorial::south(), Equatorial::zero()];
		let stars_count = stars.len();

		let mut mock_d = MockDatabase::new();
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_s = MockSpecularityConstruct::new();
		let mut abandon = abandon_never();
		let angle_tolerance = Radians(0.0);
		let mut matches = Vec::<Match<usize>>::new();
		
		let mut i = 0;  // 1 every iteration.
		let mut needs_2_pilots = false;
		mock_t.expect_begin()     .times(1).returning(|_, _| return);
		mock_t.expect_next()      .times(stars.len() + 1).returning(move |_, _| {
			if i < stars_count
			{
				let val = Some(Match{
					input: StarTriangle(i, i, i), output: StarTriangle(i, i, i), weight: 0.0});
				i += 1;
				return val;
			} else { return None; }});

		mock_d.expect_find_star() .times(stars.len() * 3).returning(|_| return Ok(Equatorial::north()));
		mock_s.expect_same()      .times(stars.len()).returning(|_, _| true);
		mock_t.expect_next_pilot().times(stars.len() * 2).returning(move |_, _| 
		{
			needs_2_pilots = !needs_2_pilots;
			i+=1;
			if needs_2_pilots { return Some(Match{input: i*100, output: (i + 1)*100, weight: 0.0}); }
			return None;
		});


		let mut mock_c = ChunkIteratorNone::new(&mock_d);
				
		let passed = Constellation::find (&stars, &mut mock_c, &mut mock_t, &mut mock_s, &mut abandon, angle_tolerance, 5..=5, &mut matches);
		let _expected = ConstellationResult::ErrorInsufficientPyramids{fails: stars_count};
		assert!(matches!(passed, _expected));
		assert_eq!(matches.len(), 4);
		assert_eq!(matches[0].input, 2);
		assert_eq!(matches[1].input, 2);
		assert_eq!(matches[2].input, 2);
		assert_eq!(matches[3].input, 100 * 5);
		assert_eq!(matches[0].output, 2);
		assert_eq!(matches[1].output, 2);
		assert_eq!(matches[2].output, 2);
		assert_eq!(matches[3].output, 100 * 6);
	}

	
	#[test]
	// If there is enough stars and a pyramid can be formed and only 4 stars are required.
	fn test_find_valid_4_stars ( )
	{
		let stars = vec![Equatorial::north()];

		let mut mock_d = MockDatabase::new();
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_s = MockSpecularityConstruct::new();
		let mut abandon = abandon_never();
		let angle_tolerance = Radians(0.0);
		let mut matches = Vec::<Match<usize>>::new();
		
		mock_t.expect_begin()     .times(1).returning(|_, _| return);
		mock_t.expect_next()      .times(1).returning(|_, _| 
			Some(Match{input: StarTriangle(0, 0, 0), output: StarTriangle(1, 2, 3), weight: 0.0}));

		mock_d.expect_find_star() .times(3).returning(|_| return Ok(Equatorial::north()));
		mock_s.expect_same()      .times(1).returning(|_, _| true);
		mock_t.expect_next_pilot().times(1).returning(|_, _| 
			return Some(Match{input: 200, output: 100, weight: 0.0}));


		let mut mock_c = ChunkIteratorNone::new(&mock_d);
				
		let passed = Constellation::find (&stars, &mut mock_c, &mut mock_t, &mut mock_s, &mut abandon, angle_tolerance, 4..=4, &mut matches);
		assert!(matches!(passed, ConstellationResult::Success{fails: 0}));
		assert_eq!(matches.len(), 4);
		assert_eq!(matches[0].input, 0);
		assert_eq!(matches[1].input, 0);
		assert_eq!(matches[2].input, 0);
		assert_eq!(matches[3].input, 200);
		assert_eq!(matches[0].output, 1);
		assert_eq!(matches[1].output, 2);
		assert_eq!(matches[2].output, 3);
		assert_eq!(matches[3].output, 100);
	}
	
	#[test]
	// If there is enough stars and a pyramid can be formed and only 4 stars are required.
	// If there is also a failed match, the failure should be tallied.
	fn test_find_valid_4_stars_with_a_fail ( )
	{
		let stars = vec![Equatorial::north()];

		let mut mock_d = MockDatabase::new();
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_s = MockSpecularityConstruct::new();
		let mut abandon = abandon_never();
		let angle_tolerance = Radians(0.0);
		let mut matches = Vec::<Match<usize>>::new();
		
		let mut count = 0;
		mock_t.expect_begin().times(1).returning(|_, _| return);
		mock_t.expect_next() .times(1 * 2).returning(|_, _|
			return Some(Match{input: StarTriangle(0, 0, 0), output: StarTriangle(1, 2, 3), weight: 0.0})
		);

		mock_d.expect_find_star() .times(3 * 2).returning(|_| return Ok(Equatorial::north()));
		mock_s.expect_same()      .times(1 * 2).returning(|_, _| true);
		mock_t.expect_next_pilot().times(1 * 2).returning(move |_, _| 
		{
			count += 1; 
			if count == 1
			{
				return None;
			}
			return Some(Match{input: 200, output: 100, weight: 0.0});
		});

		let mut mock_c = ChunkIteratorNone::new(&mock_d);
				
		let passed = Constellation::find (&stars, &mut mock_c, &mut mock_t, &mut mock_s, &mut abandon, angle_tolerance, 4..=4, &mut matches);
		assert!(matches!(passed, ConstellationResult::Success{fails: 1}));
		assert_eq!(matches.len(), 4);
		assert_eq!(matches[0].input, 0);
		assert_eq!(matches[1].input, 0);
		assert_eq!(matches[2].input, 0);
		assert_eq!(matches[3].input, 200);
		assert_eq!(matches[0].output, 1);
		assert_eq!(matches[1].output, 2);
		assert_eq!(matches[2].output, 3);
		assert_eq!(matches[3].output, 100);
	}
	
	
	#[test]
	// If the number of valid pyramid stars exceed the required number of stars, no more should be added.
	// The function will abort true with a list of valid stars.
	fn test_find_valid_too_many_pyramids ( )
	{
		let stars = vec![Equatorial::north()];

		let mut mock_d = MockDatabase::new();
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_s = MockSpecularityConstruct::new();
		let mut abandon = abandon_never();
		let angle_tolerance = Radians(0.0);
		let mut matches = Vec::<Match<usize>>::new();
		
		mock_t.expect_begin()     .times(1).returning(|_, _| return);
		mock_t.expect_next()      .times(1).returning( |_, _| 
			Some(Match{input: StarTriangle(0, 0, 0), output: StarTriangle(1, 2, 3), weight: 0.0}));

		mock_d.expect_find_star() .times(3).returning(|_| return Ok(Equatorial::north()));
		mock_s.expect_same()      .times(1).returning(|_, _| true);

		let mut i = 0;
		mock_t.expect_next_pilot().times(2).returning(move |_, _| 
			{
				i += 1;
				return Some(Match{input: i * 200, output: i * 100, weight: 0.0});
			});


		let mut mock_c = ChunkIteratorNone::new(&mock_d);
				
		let passed = Constellation::find (&stars, &mut mock_c, &mut mock_t, &mut mock_s, &mut abandon, angle_tolerance, 5..=5, &mut matches);
		assert!(matches!(passed, ConstellationResult::Success{fails: 0}));
		assert_eq!(matches.len(), 5);
		assert_eq!(matches[0].input, 0);
		assert_eq!(matches[1].input, 0);
		assert_eq!(matches[2].input, 0);
		assert_eq!(matches[3].input, 200);
		assert_eq!(matches[4].input, 400);
		assert_eq!(matches[0].output, 1);
		assert_eq!(matches[1].output, 2);
		assert_eq!(matches[2].output, 3);
		assert_eq!(matches[3].output, 100);
		assert_eq!(matches[4].output, 200);
	}



	
	#[test]
	fn test_find_abort ( )
	{
		let stars = vec![Equatorial::north()];

		let mut mock_d = MockDatabase::new();
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_s = MockSpecularityConstruct::new();
		let mut abandon = MockAbandonSearch::new();
		let angle_tolerance = Radians(0.0);
		let mut matches = Vec::<Match<usize>>::new();
		
		mock_t.expect_begin()     .times(1).returning(|_, _| return);
		mock_t.expect_next()      .times(2).returning( |_, _| 
			Some(Match{input: StarTriangle(0, 0, 0), output: StarTriangle(1, 2, 3), weight: 0.0}));

		mock_d.expect_find_star() .times(3*2).returning(|_| return Ok(Equatorial::north()));
		mock_s.expect_same()      .times(2).returning(|_, _| true);
		mock_t.expect_next_pilot().times(2).returning(|_, _| None);

		let mut i = 0;
		abandon.expect_should_abort().times(2 + 1).returning(move || 
		{
			i += 1;
			return !(i < 3);
		});

		let mut mock_c = ChunkIteratorNone::new(&mock_d);
				
		let passed = Constellation::find (&stars, &mut mock_c, &mut mock_t, &mut mock_s, &mut abandon, angle_tolerance, 5..=5, &mut matches);
		assert!(matches!(passed, ConstellationResult::ErrorAborted{fails: 2}));
		assert_eq!(matches.len(), 3);
	}
}
