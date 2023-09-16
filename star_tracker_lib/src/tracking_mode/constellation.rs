//! Implementation of Constellation.
use core_include::*;

use super::SpecularityConstruct;
use super::TriangleConstruct;
use super::PyramidConstruct;
use super::AbandonSearch;
use super::Constellation;
use super::StarTriangle;
use super::StarPyramid;
use crate::tracking_mode::database::ChunkIterator;

use crate::util::units::Equatorial;
use crate::util::units::Radians;
use crate::util::units::Match;
use crate::util::list::List;
use crate::util::aliases::Decimal;
use crate::util::err::Error;

impl Constellation
{

/// Finds a StarPyramid or StarTriangle if possible.
///
/// # Arguments
/// * `stars`           - The observed (image) stars.
/// * `database`        - The database storing reference
/// * `gen_tri`         - An object for generating a StarTriangle.
/// * `gen_pyr`         - An object for generating a Pyramid.
/// * `gen_spec`        - An object for checking Specularity.
/// * `angle_tolerance` - How much error a star pair can have until it is not considered the same.
/// 	Used for searching the database.
/// * `abort`           - A way of abandoning a search if it takes too long.
///
/// # Returns
/// If a star pyramid can be formed: Match{input: Observed Stars, output: Corresponding Database}.
/// None if a star pyramid and star triangle cannot be formed.
///
/// NOT IMPLEMENTED (in the case of a star triangle, it is not as accurate, left out)
/// If a star triangle can be formed: Match{input: Observed Stars, output: Corresponding Database}.
pub fn find(
		stars          : &dyn List<Equatorial>,
		database       : &mut dyn ChunkIterator,
		gen_tri        : &mut dyn TriangleConstruct,
		gen_pyr        : &mut dyn PyramidConstruct,
		gen_spec       : &mut dyn SpecularityConstruct,
		angle_tolerance: Radians,
		abort          : &dyn AbandonSearch,
	) -> Constellation
{
	database.begin();
	let mut fallback = Constellation::None; // If a pyramid cannot be made, can a triangle be made?
	let mut lowest_error = Decimal::MAX;
	gen_tri.begin(angle_tolerance, stars);
	
	// Loop through every possible combination of star combination using the kernel_iterator.
	// Using star_triangle_iterator to find triangle matches in the database.
	while !abort.should_abort() && let Some(iter) = gen_tri.next(stars, database)
	{
		// input and output both make triangles of the same length.
		let input  : Error<StarTriangle<Equatorial>> = iter.input.search_list(stars);
		let output : Error<StarTriangle<Equatorial>> = iter.output.search_database(database.get_database());
		let error  : Decimal = iter.weight;
		
		// ~~~~~~~~~~~~~~~~~~~~~~~~~	 A valid match was found.		~~~~~~~~~~~~~~~~~~~~
		// If the stars can be found in the database and the observed list (bug if not),
		// The code can continue.
		if input.is_ok() && output.is_ok()
		{
			let input = input.unwrap();
			let output = output.unwrap();
			
			// ~~~~~~~~~~~~~~~~~~~~~	 The speculariy in/out match	~~~~~~~~~~~~~~~~~~~~
			// If the triangles are not flipped, the code can continue.
			if gen_spec.same(&input.to_vector3(), &output.to_vector3())
			{
				let result = 
					gen_pyr.find_pilot(stars, database, angle_tolerance, iter.input, iter.output);
				
				// If a pyramid cannot be found.
				if error < lowest_error
				{
					fallback =
					Constellation::Triangle(Match{input: input, output: output, weight: 1.0});
					lowest_error = error;
				}
				
				// ~~~~~~~~~~~~~	A match is found.				~~~~~~~~~~~~~~~~~~~~
				// A pilot was found.
				if let Result::Ok(found) = result
				{
					// ~~~~~~~~~	Get the star from the database.	~~~~~~~~~~~~~~~~~~~~
					let pil_in = stars.get(found.input);
					let pil_out = database.get_database().find_star(found.output);
					
					if let Result::Ok(out) = pil_out
					{
						let pyr_in  = StarPyramid(input.0, input.1, input.2, pil_in);
						let pyr_out = StarPyramid(output.0, output.1, output.2, out);
						return Constellation::Pyramid(
							Match{input: pyr_in, output: pyr_out, weight: 1.0});
						}
					}
				}
			}
	}
	return fallback;
}


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
///                       If between min and max stars, more stars will be searched for, on max, the constellation is immediatly returned and the search ends.  
///                       If min is reached, true is returned, otherwise it will continue searching.  
///                       It is recomended to use `4..=4` for 4 stars to be found.
///   
/// * `abort`           - A way of abandoning a search if it takes too long.
///
/// # Returns
/// True if the minimum number of stars could be found.
/// False if it failed to find the minimum stars.
/// matches will have a value if 3 stars are found, even if min stars were not found.
///
/// NOT IMPLEMENTED (in the case of a star triangle, it is not as accurate, left out)
/// If a star triangle can be formed: Match{input: Observed Stars, output: Corresponding Database}.
pub fn find_all	(
	stars          : &dyn List<Equatorial>,
	database       : &mut dyn ChunkIterator,
	gen_tri        : &mut dyn TriangleConstruct,
	gen_spec       : &mut dyn SpecularityConstruct,
	abort          : &dyn AbandonSearch,
	angle_tolerance: Radians,
	num_stars      : std::ops::RangeInclusive<usize>,
	matches        : &mut dyn List<Match<usize>>,
) -> bool
{
	database.begin();
	gen_tri.begin(angle_tolerance, stars);

	// Loop through every possible combination of star combination using the kernel_iterator.
	// Using star_triangle_iterator to find triangle matches in the database.
	while !abort.should_abort() && let Some(iter) = gen_tri.next(stars, database)
	{
		// input and output both make triangles of the same length.
		let input  : Error<StarTriangle<Equatorial>> = iter.input.search_list(stars);
		let output : Error<StarTriangle<Equatorial>> = iter.output.search_database(database.get_database());

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
				matches.clear();
				let _=
				matches.push_back(Match{input: iter.input.0, output: iter.output.0, weight: 1.0});
				let _=
				matches.push_back(Match{input: iter.input.1, output: iter.output.1, weight: 1.0});
				let _=
				matches.push_back(Match{input: iter.input.2, output: iter.output.2, weight: 1.0});

				// ~~~~~~~~~~~~~	Searching through pilot stars.		~~~~~~~~~~~~~~~~~~~~
				while let Some(pilot) = gen_tri.next_pilot(stars, database) 
					&& matches.size() < *num_stars.end()
				{
					let _=
					matches.push_back(Match{input: pilot.input, output: pilot.output, weight:1.0});
				}

				if *num_stars.start() <= matches.size() { return true; }
			}
		}
	}
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
	use super::Constellation;
	use super::StarTriangle;
	use super::StarPyramid;
	use super::Match;
	use crate::tracking_mode::database::MockDatabase;
	use crate::tracking_mode::database::ChunkIteratorNone;
	use crate::tracking_mode::MockSpecularityConstruct;
	use crate::tracking_mode::MockTriangleConstruct;
	use crate::tracking_mode::MockPyramidConstruct;
	use crate::tracking_mode::MockAbandonSearch;

	use crate::util::units::Equatorial;
	use crate::util::units::Radians;
	use crate::util::aliases::Decimal;
	use crate::util::err::Errors;



	#[no_coverage]
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


//###############################################################################################//
//
//										find
//
// pub fn find	<T: TrackingModeConsts> (
// 										stars    : &dyn List<Equatorial>,
// 										database : &dyn Database,
// 										gen_tri  : &mut dyn TriangleConstruct,
// 										gen_pyr  : &mut dyn PyramidConstruct<T>,
// 										gen_spec : &mut dyn SpecularityConstruct<T>
// 									) -> Constellation
// 		where T: 'static + TrackingModeConsts,
// 		ArrayList<(), {T::PAIRS_MAX}> : Sized,
//
//###############################################################################################//

	#[test]
	// If there is no triangles, None should be returned.
	fn test_find_no_triangles ( )
	{
		let a = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let stars : Vec<Equatorial> = vec![a, a];
		let mock_d = MockDatabase::new();
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_p = MockPyramidConstruct::new();
		let mut mock_s = MockSpecularityConstruct::new();
		let mut chunk  = ChunkIteratorNone::new(&mock_d);
		let mut abandon = MockAbandonSearch::new(); abandon.expect_should_abort().returning(||false);
		let angle_tolerance = Radians(0.0);

		mock_t.expect_begin()
			.times(1)
			.withf(move |angle,stars| (*angle - angle_tolerance).abs() < 0.001
				&& stars.size() == 2)
			.returning(|_, _| return);

		mock_t.expect_next().times(1).returning(|_, _| return None);

		let result = Constellation::find (
			&stars, &mut chunk, &mut mock_t, &mut mock_p, &mut mock_s, angle_tolerance, &abandon);

		assert_eq!(Constellation::None, result);

	}




	#[test]
	// If there is triangles but for some reason, the database or observed stars are invalid.
	// It should not fail, it should continue until all triangle are exhausted, then return None.
	fn test_find_error_match_triangles ( )
	{
		let a = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let stars : Vec<Equatorial> = vec![a, a];
		let mut mock_d = MockDatabase::new();
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_p = MockPyramidConstruct::new();
		let mut mock_s = MockSpecularityConstruct::new();
		let mut abandon = MockAbandonSearch::new(); abandon.expect_should_abort().returning(||false);
		let angle_tolerance = Radians(0.0);

		mock_d.expect_find_star().times(3 * 3).returning(|_| Err(Errors::NoMatch));
		let mut chunk  = ChunkIteratorNone::new(&mock_d);

		mock_t.expect_begin().returning(|_, _| return);

		let mut i = 0;
		mock_t.expect_next().returning(move |_, _|
		{
			i += 1;
			if 3 < i
			{
				return None;
			}
			else
			{
				let triangle = StarTriangle(3,3,3);
				return Some(Match{input: triangle, output: triangle, weight: 1.0});
			}
		} );

		// 3 * 3 times as there is 3 iterations and find_star is called 3 times per loop.


		let result = Constellation::find (
			&stars, &mut chunk, &mut mock_t, &mut mock_p, &mut mock_s, angle_tolerance, &abandon);

			assert_eq!(Constellation::None, result);
	}





	#[test]
	// If there is triangles and the triangle is valid, the specularity should be tested.
	// If the specularity is invalid, another triangle should be tested until all are exaused.
	// None will be returned.
	fn test_find_invalid_specularity ( )
	{
		let a = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let stars : Vec<Equatorial> = vec![a, a, a, a];
		let mut mock_d = MockDatabase::new();
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_p = MockPyramidConstruct::new();
		let mut mock_s = MockSpecularityConstruct::new();
		let mut abandon = MockAbandonSearch::new(); abandon.expect_should_abort().returning(||false);
		let angle_tolerance = Radians(0.0);

		mock_d.expect_find_star().returning(|_| return Ok(Equatorial::zero()));
		let mut chunk  = ChunkIteratorNone::new(&mock_d);

		mock_t.expect_begin().returning(|_, _| return);

		let mut i = 0;
		mock_t.expect_next().returning(move |_, _|
		{
			i += 1;
			if 3 < i
			{
				return None;
			}
			else
			{
				let triangle = StarTriangle(0,0,0);
				return Some(Match{input: triangle, output: triangle, weight: 1.0});
			}
		} );



		mock_s.expect_same().times(3).returning(|_, _| return false);

		let result = Constellation::find (
			&stars, &mut chunk, &mut mock_t, &mut mock_p, &mut mock_s, angle_tolerance, &abandon);

		assert_eq!(Constellation::None, result);
	}


	#[test]
	// 4 Stars FAIL
	// If there is triangles and the triangle is valid and the specularity is valid.
	// If there are 4 stars, a pyramid must be found, if it cannot be found a triangle is returned
	fn test_find_4_stars_fail_find_pilot ( )
	{
		let a = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let stars : Vec<Equatorial> = vec![a, a, a, a];
		let mut mock_d = MockDatabase::new();
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_p = MockPyramidConstruct::new();
		let mut mock_s = MockSpecularityConstruct::new();
		let mut abandon = MockAbandonSearch::new(); abandon.expect_should_abort().returning(||false);
		let angle_tolerance = Radians(0.0);

		mock_d.expect_find_star().returning(|_| return Ok(Equatorial::zero()));
		let mut chunk  = ChunkIteratorNone::new(&mock_d);


		mock_t.expect_begin().returning(|_, _| return);

		let mut i = 0;
		mock_t.expect_next().returning(move |_, _|
		{
			i += 1;
			if 3 < i
			{
				return None;
			}
			else
			{
				let triangle = StarTriangle(0,0,0);
				return Some(Match{input: triangle, output: triangle, weight: 1.0});
			}
		} );



		mock_s.expect_same().times(3).returning(|_, _| return true);

		mock_p.expect_find_pilot().times(3).returning(move |_,_,a,_,_|  
			{
				assert_eq!(a, angle_tolerance);
				Err(Errors::NoMatch)
			});

		let result = Constellation::find (
			&stars, &mut chunk, &mut mock_t, &mut mock_p, &mut mock_s, angle_tolerance, &abandon);

		let expected_fallback = Match{input: StarTriangle(a,a,a), output: StarTriangle(a,a,a), weight: 1.0};
		assert_eq!(Constellation::Triangle(expected_fallback), result);
	}

	#[test]
	// 4 Stars Fail
	// If there is triangles and the triangle is valid and the specularity is valid.
	// If there are 4 stars, if a pyramid can be found but the observed or database values are wrong.
	// ...
	fn test_find_4_stars_fail_database_search ( )
	{
		let a = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let stars : Vec<Equatorial> = vec![a, a, a, a];
		let mut mock_d = MockDatabase::new();
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_p = MockPyramidConstruct::new();
		let mut mock_s = MockSpecularityConstruct::new();
		let mut abandon = MockAbandonSearch::new(); abandon.expect_should_abort().returning(||false);
		let angle_tolerance = Radians(0.0);

		mock_d.expect_find_star()
		.returning(|index|
			if index == 0
			{
				return Ok(Equatorial::zero())
			}
			else
			{
				return Err(Errors::OutOfBounds)
			});

		let mut chunk  = ChunkIteratorNone::new(&mock_d);


		mock_t.expect_begin().returning(|_, _| return);

		let mut i = 0;
		mock_t.expect_next().returning(move |_, _|
		{
			i += 1;
			if 3 < i
			{
				return None;
			}
			else
			{
				let triangle = StarTriangle(0,0,0);
				return Some(Match{input: triangle, output: triangle, weight: 1.0});
			}
		} );


		mock_s.expect_same().times(3).returning(|_, _| return true);

		mock_p.expect_find_pilot().times(3)
			.returning(|_,_,_,_,_| Ok(Match{input: 1, output: 1, weight: 0.0}));

		let result = Constellation::find (
			&stars, &mut chunk, &mut mock_t, &mut mock_p, &mut mock_s, angle_tolerance, &abandon);


		let expected_fallback = Match{input: StarTriangle(a,a,a), output: StarTriangle(a,a,a), weight: 1.0};
		assert_eq!(Constellation::Triangle(expected_fallback), result);
	}



	#[test]
	// 4 Stars SUCCESS
	// If there is triangles and the triangle is valid and the specularity is valid.
	// If there are 4 stars, if a pyramid is made, Pyramid should be returned.
	fn test_success_first_try ( )
	{
		let a = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let b = Equatorial{ra: Radians(1.0), dec: Radians(1.0)};
		let c = Equatorial{ra: Radians(2.0), dec: Radians(2.0)};
		let d = Equatorial{ra: Radians(3.0), dec: Radians(3.0)};
		let stars : Vec<Equatorial> = vec![a, b, c, d];
		let mut mock_d = MockDatabase::new();
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_p = MockPyramidConstruct::new();
		let mut mock_s = MockSpecularityConstruct::new();
		let mut abandon = MockAbandonSearch::new(); abandon.expect_should_abort().returning(||false);
		let angle_tolerance = Radians(0.0);

		mock_d.expect_find_star()
		.returning(|i|
			return Ok(
				Equatorial{ra: Radians(i as Decimal+10.0), dec: Radians(i as Decimal+10.0)}));
		let mut chunk  = ChunkIteratorNone::new(&mock_d);


		mock_t.expect_begin().returning(|_, _| return);

		mock_t.expect_next().returning(move |_, _|
		{		let triangle = StarTriangle(0,1,2);
				return Some(Match{input: triangle, output: triangle, weight: 1.0}); } );



		mock_s.expect_same().returning(|_, _| return true);

		mock_p.expect_find_pilot().times(1)
			.returning(|_,_,_,_,_| Ok(Match{input: 3, output: 3, weight: 0.0}));

		let result = Constellation::find (
			&stars, &mut chunk, &mut mock_t, &mut mock_p, &mut mock_s, angle_tolerance, &abandon);

		let expect_input = StarPyramid(stars[0], stars[1], stars[2], stars[3]);
		let expect_output = StarPyramid(
			Equatorial{ra: Radians(10.0), dec: Radians(10.0)},
			Equatorial{ra: Radians(11.0), dec: Radians(11.0)},
			Equatorial{ra: Radians(12.0), dec: Radians(12.0)},
			Equatorial{ra: Radians(13.0), dec: Radians(13.0)});
		let expect = Match{input: expect_input, output: expect_output, weight: 1.0};

		assert_eq!(Constellation::Pyramid(expect), result);
	}




	#[test]
	// 4 Stars SUCCESS
	// If there is triangles and the triangle is valid and the specularity is valid.
	// If there are 4 stars, if a pyramid is made, Pyramid should be returned even on a fail.
	fn test_success_second_try ( )
	{
		let a = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let b = Equatorial{ra: Radians(1.0), dec: Radians(1.0)};
		let c = Equatorial{ra: Radians(2.0), dec: Radians(2.0)};
		let d = Equatorial{ra: Radians(3.0), dec: Radians(3.0)};
		let stars : Vec<Equatorial> = vec![a, b, c, d];
		let mut mock_d = MockDatabase::new();
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_p = MockPyramidConstruct::new();
		let mut mock_s = MockSpecularityConstruct::new();
		let mut abandon = MockAbandonSearch::new(); abandon.expect_should_abort().returning(||false);
		let angle_tolerance = Radians(0.0);

		mock_d.expect_find_star()
		.returning(|i|
			return Ok(
				Equatorial{ra: Radians(i as Decimal+10.0), dec: Radians(i as Decimal+10.0)}));
		let mut chunk  = ChunkIteratorNone::new(&mock_d);

		mock_t.expect_begin().returning(|_, _| return);

		mock_t.expect_next().returning(move |_, _|
		{		let triangle = StarTriangle(0,1,2);
				return Some(Match{input: triangle, output: triangle, weight: 1.0});	} );


		mock_s.expect_same().returning(|_, _| return true);

		let mut already_called = false;
		mock_p.expect_find_pilot().times(2)
			.returning(move |_,_,_,_,_|
				{
					if already_called { return Ok(Match{input: 3, output: 3, weight: 0.0}) }
					else { already_called = true; return Err(Errors::NoMatch); }
				});

		let result = Constellation::find (
			&stars, &mut chunk, &mut mock_t, &mut mock_p, &mut mock_s, angle_tolerance, &abandon);

		let expect_input = StarPyramid(stars[0], stars[1], stars[2], stars[3]);
		let expect_output = StarPyramid(
			Equatorial{ra: Radians(10.0), dec: Radians(10.0)},
			Equatorial{ra: Radians(11.0), dec: Radians(11.0)},
			Equatorial{ra: Radians(12.0), dec: Radians(12.0)},
			Equatorial{ra: Radians(13.0), dec: Radians(13.0)});
		let expect = Match{input: expect_input, output: expect_output, weight: 1.0};

		assert_eq!(Constellation::Pyramid(expect), result);
	}




	#[test]
	// The function went over time or something...
	// Nothing could be found...
	fn test_abort_none ( )
	{
		let a = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let stars : Vec<Equatorial> = vec![a, a, a, a];
		let mut mock_d = MockDatabase::new();
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_p = MockPyramidConstruct::new();
		let mut mock_s = MockSpecularityConstruct::new();
		let mut abandon = MockAbandonSearch::new();
		abandon.expect_should_abort().returning(|| true);
		let angle_tolerance = Radians(0.0);

		mock_d.expect_find_star()
		.returning(|_| return Ok(Equatorial::zero()));

		let mut chunk  = ChunkIteratorNone::new(&mock_d);


		mock_t.expect_begin().returning(|_, _| return);

		mock_t.expect_next().returning(|_, _| return None );

		let result = Constellation::find (
			&stars, &mut chunk, &mut mock_t, &mut mock_p, &mut mock_s, angle_tolerance, &abandon);

		assert_eq!(Constellation::None, result);
	}
}
