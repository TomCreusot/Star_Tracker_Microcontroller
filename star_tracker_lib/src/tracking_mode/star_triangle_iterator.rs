///! Implementation for StarTriangleIterator
use crate::core_include::*;

use crate::tracking_mode::StarTriangleIterator;
use crate::tracking_mode::TriangleConstruct;
// use crate::tracking_mode::IterationResult;
use crate::tracking_mode::KernelIterator;
use crate::tracking_mode::StarTriangle;

use crate::tracking_mode::database::ChunkIterator;

use crate::util::list::ArrayList;
use crate::util::list::List;
use crate::util::units::Equatorial;
use crate::util::units::Radians;
use crate::util::units::Match;


impl <const N: usize> TriangleConstruct for StarTriangleIterator<N>
{
	/// Call this to get the next StarTriangle observed/database pair.
	/// # Arguments
	/// * `stars` - The observed stars in the image.
	/// * `database` - The database of stars to search through.
	/// # Returns
	/// * None if there is no more available star triangles with the given parameters.
	/// * Some(Match{input: observed star triangle, output: database match}) if possible.
	fn next ( &mut self, stars: &dyn List<Equatorial>, database: &mut dyn ChunkIterator
															) -> Option<Match<StarTriangle<usize>>>
	{
		self.index_p = -1;
		let mut tries : Option<Match<StarTriangle<usize>>> = Option::None;
		'_outer: loop // This is the correct use of a do while loop.
		{

			// Once all possiblities for a single kernal step are exhausted.
			// The kernal will step, new stars will be chosen and a list of database matches are generated.
			while !StarTriangleIterator::<N>::step(
				&mut self.index_a, &mut self.index_b, &mut self.index_c,
				self.pair_a.size(), self.pair_b.size(), self.pair_c.size())
			{
				 // Rust implementation of a do while loop.
				if !self.prep_new_kernel(stars, database) { break '_outer; }
			}

			let a = self.pair_a.get(self.index_a as usize);
			let b = self.pair_b.get(self.index_b);
			let c = self.pair_c.get(self.index_c);

			let triangle = StarTriangle::construct_triangle(a.result, b.result, c.result);

			// Leaves the loop if a, b and c sides of the database are connected.
			if triangle.is_some()
			{
				let input  = self.input;
				let output = triangle.unwrap();
				let error  = a.error + b.error + c.error;
				tries = Option::Some(Match{input: input, output: output, weight: error});
				break '_outer; // Rust implementation of a do while loop.
			}
		}
		self.expected_triangle = tries;
		return tries;
	}



	/// Iterates though suitable pilot stars for the given star triangle.
	/// Ensure the database iterator has not iterated since calling next.
	/// # Arguments
	/// * `stars`    - The observed stars in the image.
	/// * `database` - The database of stars to search through (That has not been iterated since next).
	/// # Returns
	/// * None if there is no more available pilot stars.
	/// * Some(Match{input: observed star triangle, output: database match}) if possible.
	fn next_pilot ( &mut self, stars: &dyn List<Equatorial>, database: &mut dyn ChunkIterator
																		) -> Option<Match<usize>>
	{
		if let Some(expected) = self.expected_triangle
		{
			'_outer: loop // This is the correct use of a do while loop.
			{
				while !( StarTriangleIterator::<N>::step(
					&mut self.index_p_a, &mut self.index_p_b, &mut self.index_p_c,
					self.pair_p_a.size(), self.pair_p_b.size(), self.pair_p_c.size()) )
				{
					if !self.prep_new_pilot(stars, database)
					{
						break '_outer;
					}
				}

				// Gets the pairs between each of the star triangle stars and the pilot.
				let a = self.pair_p_a.get(self.index_p_a as usize);
				let b = self.pair_p_b.get(self.index_p_b);
				let c = self.pair_p_c.get(self.index_p_c);

				// Ensures the stars from the triangle are in the database result.
				let mut valid = a.result.has(expected.output.0);
				valid        &= b.result.has(expected.output.1);
				valid        &= c.result.has(expected.output.2);

				if valid
				{
					// Ensures the pilot is the same in each pair.
					let pilot = a.result.find_not(expected.output.0);

					if let Some(pilot) = pilot
					{
						valid &= b.result.has(pilot);
						valid &= c.result.has(pilot);

						if valid
						{
							let input  = self.index_p as usize;
							let output = pilot as usize;
							let error  = a.error + b.error + c.error;
							return Some(Match{input: input, output: output, weight: error});
						}
					}
				}
			}
		}
		return Option::None;
	}



	/// Prepares the StarTriangleIterator for iterating.
	/// # Arguments
	/// * `angle_tolerance` - When searching the database, the tolerance to use.
	/// * `stars` - The observed stars.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::tracking_mode::StarTriangleIterator;
	/// use star_tracker_lib::tracking_mode::TriangleConstruct;
	/// use star_tracker_lib::util::units::Radians;
	/// use star_tracker_lib::util::units::Equatorial;
	/// use star_tracker_lib::util::list::ArrayList;
	///
	/// // The stars found in the image.
	/// let obs_stars : ArrayList<Equatorial, 2> =
	/// 	ArrayList::from_array(&[Equatorial::zero(),Equatorial::zero()]);
	/// // The tolerance allowed whens searching the database.
	/// let angle = Radians(1.0);
	/// // The amount of elements that can be examined when searching the database.
	/// const NUM_MATCH : usize = 4;
	/// // Construct the iterator.
	/// let mut iterator: StarTriangleIterator<NUM_MATCH> = StarTriangleIterator::new();
	/// iterator.begin(angle, &obs_stars);
	/// ```
	fn begin ( &mut self, angle_tolerance: Radians, stars: &dyn List<Equatorial> )
	{
		self.kernel = KernelIterator::new(stars.size());
		self.pair_a.clear();
		self.pair_b.clear();
		self.pair_c.clear();
		self.pair_p_a.clear();
		self.pair_p_b.clear();
		self.pair_p_c.clear();
		self.index_p = -1;
		self.index_a = -1;
		self.index_b = 0;
		self.index_c = 0;
		self.index_p_a = -1;
		self.index_p_b = 0;
		self.index_p_c = 0;
		self.input = StarTriangle(0,0,0);
		self.expected_triangle = None;
		self.angle_tolerance = angle_tolerance;
	}

}




impl<const N: usize> StarTriangleIterator<N>
{
	/// Constructs a new StarTriangleIterator.
	/// DOES NOT ASSIGN ANY VALUES.
	/// CALL `begin` AFTER THIS.
	pub fn new ( ) -> Self
	{
		return Self
		{
			kernel: KernelIterator::new(0),
			pair_a: ArrayList::new(),
			pair_b: ArrayList::new(),
			pair_c: ArrayList::new(),
			pair_p_a: ArrayList::new(),
			pair_p_b: ArrayList::new(),
			pair_p_c: ArrayList::new(),
			index_p: -1,
			index_a: -1,
			index_b: 0,
			index_c: 0,
			index_p_a: -1,
			index_p_b: 0,
			index_p_c: 0,
			input: StarTriangle(0,0,0),
			expected_triangle: Option::None,
			angle_tolerance: Radians(0.0),
		};
	}



	/// Steps a set of 3 indices where a is changing every time, b is changing every a times and c is changing a*b times.
	/// Presequence: a: -1, b: 0, c: 0.
	/// # Returns
	/// False if the sequence ended.
	fn step ( a: &mut isize, b: &mut usize, c: &mut usize,
				a_max: usize, b_max: usize, c_max: usize ) -> bool
	{
		if *a < a_max as isize - 1                     { *a += 1;      }
		else
		{
			*a = 0;
			if (*b as isize) < b_max as isize - 1      { *b += 1;      }
			else
			{
				*b = 0;
				if (*c as isize) < c_max as isize - 1  { *c += 1;      }
				else                                   { return false; }
			}
		}
		return *a < a_max as isize && *b < b_max && *c < c_max;
	}




	/// When a new kernal step is required:
	/// - Finds the angle distance between the stars.
	/// - Finds a long list of matches from the database to compare with.
	/// # Arguments
	/// * `stars` - The stars in the image.
	/// * `database` - The database where the stars can be searched.
	fn prep_new_kernel ( &mut self, stars: &dyn List<Equatorial>,
		database: &mut dyn ChunkIterator ) -> bool
	{
		// Steps to the next region.
		// If at final region, steps to next kernal.
		// If at final kernal, aborts iteration.
		if !database.next()
		{
			database.begin();
			if !self.kernel.step()
			{
				return false;
			}
		}

		// Ensures input is set.
		self.input = StarTriangle(self.kernel.i, self.kernel.j, self.kernel.k);

		// The angular distance between observed stars.
		let side_a = stars.get(self.kernel.i).angle_distance(stars.get(self.kernel.j)); // i, j
		let side_b = stars.get(self.kernel.i).angle_distance(stars.get(self.kernel.k)); // i, k
		let side_c = stars.get(self.kernel.j).angle_distance(stars.get(self.kernel.k)); // j, k

		self.pair_a.clear();
		self.pair_b.clear();
		self.pair_c.clear();

		// Search the database for each side.
		database.find_close_ref_region(side_a, self.angle_tolerance, &mut self.pair_a);
		database.find_close_ref_region(side_b, self.angle_tolerance, &mut self.pair_b);
		database.find_close_ref_region(side_c, self.angle_tolerance, &mut self.pair_c);

		// With new arrays, the iterations must go back to the begining.
		self.index_a = -1;
		self.index_b = 0;
		self.index_c = 0;
		return true;
	}
	
	


	/// When all the possible pilots are exausted for a given input star, moves to a new input.
	/// # Arguments
	/// * `stars` - The stars in the image.
	/// * `database` - The database where the stars can be searched.
	fn prep_new_pilot ( &mut self, stars: &dyn List<Equatorial>,
		database: &mut dyn ChunkIterator ) -> bool
	{
		if let Some(expected) = self.expected_triangle
		{
			// Move to next viable pilot.
			self.index_p += 1;
			while
				self.index_p == expected.input.0 as isize || 
				self.index_p == expected.input.1 as isize || 
				self.index_p == expected.input.2 as isize
			{
				self.index_p += 1;
			}
			if stars.size() <= self.index_p as usize { return false; }
			
			
			let pilot_in = stars.get(self.index_p as usize);
			let side_a = stars.get(expected.input.0).angle_distance(pilot_in);
			let side_b = stars.get(expected.input.1).angle_distance(pilot_in);
			let side_c = stars.get(expected.input.2).angle_distance(pilot_in);
			
			self.pair_p_a.clear();
			self.pair_p_b.clear();
			self.pair_p_c.clear();
			database.find_close_ref_region(side_a, self.angle_tolerance, &mut self.pair_p_a);
			database.find_close_ref_region(side_b, self.angle_tolerance, &mut self.pair_p_b);
			database.find_close_ref_region(side_c, self.angle_tolerance, &mut self.pair_p_c);

			self.index_p_a = -1;
			self.index_p_b = 0;
			self.index_p_c = 0;
			return true;
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
	use crate::tracking_mode::StarTriangleIterator;
	use crate::tracking_mode::TriangleConstruct;
	use crate::tracking_mode::KernelIterator;
	use crate::tracking_mode::StarTriangle;
	use crate::tracking_mode::StarPair;
	use crate::tracking_mode::database::MockChunkIterator;
	use crate::tracking_mode::database::SearchResult;

	use crate::util::units::Equatorial;
	use crate::util::units::Radians;
	use crate::util::units::Match;
	use crate::util::list::List;

//###############################################################################################//
//
//										Begin
//
// fn new <NUMBER_MATCHES> ( &mut self, angle_tolerance: Radians, stars: &dyn List<Equatorial> )
//
//###############################################################################################//

	#[test]
	// The required variables to be set are:
	// - kernel (requires the number of stars to begin).
	// - pair_a, pair_b, pair_c capacity must be the value of N.
	fn test_begin ( )
	{
		let stars : Vec<Equatorial>=vec![Equatorial::zero(),Equatorial::zero(),Equatorial::zero()];
		let angle = Radians(1.0);
		const NUM_MATCH : usize = 4;
		let mut iterator: StarTriangleIterator<NUM_MATCH> = StarTriangleIterator::new();
		iterator.begin(angle, &stars);
		
		assert_eq!(stars.size(), iterator.kernel.size);
		assert_eq!(3, iterator.kernel.size);
		assert_eq!(Option::None, iterator.expected_triangle);
		
		assert_eq!(NUM_MATCH, iterator.pair_a.capacity());
		assert_eq!(NUM_MATCH, iterator.pair_b.capacity());
		assert_eq!(NUM_MATCH, iterator.pair_c.capacity());
		
		assert_eq!(NUM_MATCH, iterator.pair_p_a.capacity());
		assert_eq!(NUM_MATCH, iterator.pair_p_b.capacity());
		assert_eq!(NUM_MATCH, iterator.pair_p_c.capacity());
		
		assert_eq!(-1, iterator.index_p);
		assert_eq!(-1, iterator.index_a);
		assert_eq!(0, iterator.index_b);
		assert_eq!(0, iterator.index_c);
		
		assert_eq!(-1, iterator.index_p_a);
		assert_eq!(0, iterator.index_p_b);
		assert_eq!(0, iterator.index_p_c);
	}




//###############################################################################################//
//
//										Next
//
// 	pub fn next ( &mut self, stars: &dyn List<Equatorial>, database: &dyn Database
// 													) -> Option<Match<StarTriangle<usize>>>
//
//###############################################################################################//

	#[test]
	// If the kernel cannot progress (due to no stars), none is returned,
	fn test_next_no_stars ( )
	{
		let stars : Vec<Equatorial> = Vec::new();
		let angle = Radians(0.123);
		const NUM_MATCH : usize = 4;
		let mut iterator: StarTriangleIterator<NUM_MATCH> = StarTriangleIterator::new();
		iterator.begin(angle, &stars);

		let mut chunk = MockChunkIterator::new();
		chunk.expect_next().times(1).returning(| | false);
		chunk.expect_begin().times(1).returning(|| return);
		chunk.expect_find_close_ref_region().times(0);

		assert_eq!(None, iterator.next(&stars, &mut chunk));
		assert_eq!(-1, iterator.index_p); // Pilot should be reset.
	}



	#[test]
	// If a match cannot be made, a proper handle should occure
	fn test_next_no_match ( )
	{
		let mut stars : Vec<Equatorial> = Vec::new();
		stars.push_back(Equatorial{ra: Radians(0.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(1.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(2.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(3.0), dec: Radians(0.0)});
		let angle = Radians(0.123);
		const NUM_MATCH : usize = 4;
		let mut iterator: StarTriangleIterator<NUM_MATCH> = StarTriangleIterator::new();
		iterator.begin(angle, &stars);

		let mut chunk = MockChunkIterator::new();
		let mut first = false;
		chunk.expect_begin().times(5).returning(|| return); // 4 combinations + final check.
		chunk.expect_next().times(9).returning(
			move | | { let val = first; first = !first; return val; }); // 2 chunks (*2)
		chunk.expect_find_close_ref_region().times(3 * 4 * 2).returning(|_,_,_| ());

		// Should loop until finished
		assert_eq!(None, iterator.next(&stars, &mut chunk));
		
		assert_eq!(-1, iterator.index_p); // Pilot should be reset.
	}


	#[test]
	// If the kernel cannot progress, none is returned,
	fn test_next ( )
	{
		let mut stars : Vec<Equatorial> = Vec::new();
		stars.push_back(Equatorial{ra: Radians(0.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(1.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(2.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(3.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(4.0), dec: Radians(0.0)});
		let angle = Radians(0.123);
		const NUM_MATCH : usize = 4;
		let mut iterator: StarTriangleIterator<NUM_MATCH> = StarTriangleIterator::new();
		iterator.begin(angle, &stars);

		// Triangles:
		// 0, 1, 2
		// 0, 2, 3
		// 0, 2, 4
		let outputs =
		[
			StarPair(100, 101),
			StarPair(0, 1),
			StarPair(0, 2),
			StarPair(0, 0),

			StarPair(1, 2),
			StarPair(102, 103),
			StarPair(2, 3),
			StarPair(2, 4),

			StarPair(2, 0),
			StarPair(3, 0),
			StarPair(0, 4),
			StarPair(104, 105),
		];
		let mut index = 0;
		let mut chunk = MockChunkIterator::new();
		chunk.expect_begin().returning(|| return);
		chunk.expect_next().returning(|| return false);
		chunk.expect_find_close_ref_region().times(3)
			.returning(move |_, _, found|
				{
					found.push_back(SearchResult{result: outputs[index], error: 0.0});
					index += 1;
					found.push_back(SearchResult{result: outputs[index], error: 0.0});
					index += 1;
					found.push_back(SearchResult{result: outputs[index], error: 0.0});
					index += 1;
					found.push_back(SearchResult{result: outputs[index], error: 0.0});
					index += 1;
				}
			);

		let mut actual = iterator.next(&stars, &mut chunk);
		let mut expect = Match{input:StarTriangle(0,1,2),output: StarTriangle(1,0,2), weight: 0.0};
		assert_eq!(StarTriangle(0,1,2), iterator.input);
		assert_eq!(4, iterator.pair_a.size());
		assert_eq!(StarPair(100, 101), iterator.pair_a.get(0).result);
		assert_eq!(StarPair(0, 1),     iterator.pair_a.get(1).result);
		assert_eq!(StarPair(0, 2),     iterator.pair_a.get(2).result);
		assert_eq!(StarPair(0, 0),     iterator.pair_a.get(3).result);
		assert_eq!(4, iterator.pair_b.size());
		assert_eq!(StarPair(1, 2),     iterator.pair_b.get(0).result);
		assert_eq!(StarPair(102, 103), iterator.pair_b.get(1).result);
		assert_eq!(StarPair(2, 3),     iterator.pair_b.get(2).result);
		assert_eq!(StarPair(2, 4),     iterator.pair_b.get(3).result);
		assert_eq!(4, iterator.pair_c.size());
		assert_eq!(StarPair(2, 0),     iterator.pair_c.get(0).result);
		assert_eq!(StarPair(3, 0),     iterator.pair_c.get(1).result);
		assert_eq!(StarPair(0, 4),     iterator.pair_c.get(2).result);
		assert_eq!(StarPair(104, 105), iterator.pair_c.get(3).result);
		assert_eq!((1, 0, 0), (iterator.index_a, iterator.index_b, iterator.index_c));
		assert_eq!(Some(expect), actual);

		actual = iterator.next(&stars, &mut chunk);
		expect = Match{input:StarTriangle(0, 1, 2),output: StarTriangle(2, 0, 3), weight:0.0};
		assert_eq!(Some(expect), actual);

		actual = iterator.next(&stars, &mut chunk);
		expect = Match{input:StarTriangle(0, 1, 2),output: StarTriangle(2, 0, 4), weight: 0.0};
		assert_eq!(Some(expect), actual);

		iterator.kernel.size = 0;
		actual = iterator.next(&stars, &mut chunk);
		assert_eq!(None, actual);

		let outputs_2 = [StarPair(0,1), StarPair(1,2), StarPair(2,0)];
		iterator.kernel = KernelIterator::new(outputs_2.len());
		index = 0;
		chunk = MockChunkIterator::new();
		chunk.expect_begin().returning(|| return);
		chunk.expect_next().returning(|| return false);
		chunk.expect_find_close_ref_region().times(3)
			.returning(move |_, _, found|
				{
					found.push_back(SearchResult{result: outputs_2[index], error: 0.0});
					index+=1;
				}
			);

		actual = iterator.next(&stars, &mut chunk);
		expect = Match{input:StarTriangle(0, 1, 2),output: StarTriangle(1, 0, 2),weight: 0.0};
		assert_eq!(Some(expect), actual);
		
		assert_eq!(-1, iterator.index_p); // Pilot should be reset.
	}




//###############################################################################################//
//
//										NextPilot
//
// 	pub fn next_pilot ( &mut self, stars: &dyn List<Equatorial>, database: &dyn Database
// 													) -> Option<Match<StarTriangle<usize>>>
//
//###############################################################################################//

	#[test]
	// If a star triangle is not setup, the pyramid should return none.
	fn test_next_pilot_invalid_triangle ( )
	{
		let mut stars : Vec<Equatorial> = Vec::new();
		stars.push_back(Equatorial{ra: Radians(0.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(1.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(2.0), dec: Radians(0.0)});
		let angle = Radians(0.123);
		const NUM_MATCH : usize = 4;
		let mut iterator: StarTriangleIterator<NUM_MATCH> = StarTriangleIterator::new();
		iterator.begin(angle, &stars);
		
		// The iterator must not try to find a pyramid set.
		let mut chunk = MockChunkIterator::new();
		chunk.expect_find_close_ref_region().times(0);
		
		assert_eq!(Option::None, iterator.next_pilot(&stars, &mut chunk));
		assert_eq!(-1, iterator.index_p); // If anything happened, this would not be -1.
	}
	
	
	#[test]
	// If the triangle is setup BUT:
	// There are only 3 stars (need 4).
	fn test_next_pilot_ ( )
	{
		let mut stars : Vec<Equatorial> = Vec::new();
		stars.push_back(Equatorial{ra: Radians(0.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(1.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(2.0), dec: Radians(0.0)});
		let angle = Radians(0.123);
		const NUM_MATCH : usize = 4;
		let mut iterator: StarTriangleIterator<NUM_MATCH> = StarTriangleIterator::new();
		iterator.begin(angle, &stars);
		
		// The iterator must not try to find a pyramid set.
		let mut chunk = MockChunkIterator::new();
		chunk.expect_find_close_ref_region().times(0); // Doesnt reach the search yet...
			// .returning(move |_, _, found|
			// 	{
			// 
			// 		// found.push_back(SearchResult{result: outputs[index], error: 0.0});
			// 		// index += 1;
			// 		// found.push_back(SearchResult{result: outputs[index], error: 0.0});
			// 		// index += 1;
			// 		// found.push_back(SearchResult{result: outputs[index], error: 0.0});
			// 		// index += 1;
			// 		// found.push_back(SearchResult{result: outputs[index], error: 0.0});
			// 		// index += 1;
			// 	}
			// );
		let triangle = StarTriangle(0, 1, 2);
		iterator.expected_triangle = Some(Match{input: triangle, output: triangle, weight: 0.0});
		
		assert_eq!(Option::None, iterator.next_pilot(&stars, &mut chunk));
		assert_eq!(3, iterator.index_p); // Exceeded the index of the array.
	}

	
	
	#[test]
	// If the triangle is setup AND there are enough stars BUT:
	// There are no matches.
	fn test_next_pilot_no_matches ( )
	{
		let mut stars : Vec<Equatorial> = Vec::new();
		stars.push_back(Equatorial{ra: Radians(0.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(1.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(2.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(3.0), dec: Radians(0.0)});
		let angle = Radians(0.123);
		const NUM_MATCH : usize = 4;
		let mut iterator: StarTriangleIterator<NUM_MATCH> = StarTriangleIterator::new();
		iterator.begin(angle, &stars);
		
		let mut chunk = MockChunkIterator::new();
		chunk.expect_find_close_ref_region().times(3) // None of the angles produces a valid result
			.returning(move |angle, _, found|
			{ 
				found.push_back(SearchResult{result: 
					StarPair(angle.0 as usize + 2, angle.0 as usize + 100), error: 0.0}); 
			});
		let triangle = StarTriangle(2, 0, 3);
		iterator.expected_triangle = Some(Match{input: triangle, output: triangle, weight: 0.0});
		
		assert_eq!(Option::None, iterator.next_pilot(&stars, &mut chunk));
		assert_eq!(4, iterator.index_p); // Exceeded the index of the array.
	}
	
	
	
	#[test]
	// If the triangle is setup AND there are enough stars AND there are matches BUT:
	// The star triangle is not present in the pilot search.
	fn test_next_pilot_triangle_absent ( )
	{
		let mut stars : Vec<Equatorial> = Vec::new();
		stars.push_back(Equatorial{ra: Radians(0.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(1.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(2.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(3.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(4.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(5.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(6.0), dec: Radians(0.0)});
		let angle = Radians(0.123);
		const NUM_MATCH : usize = 4;
		let mut iterator: StarTriangleIterator<NUM_MATCH> = StarTriangleIterator::new();
		iterator.begin(angle, &stars);
		
		let mut chunk = MockChunkIterator::new();
		chunk.expect_find_close_ref_region().times(3* 4)
			.returning(move |angle, _, found|
			{ 
				// 0: 0 | 1 | 2
				// 1: 0 | 0 | 1
				// 4: 0 | 0 | 0
				found.push_back(SearchResult{result: 
					StarPair(angle.0 as usize + 1, (angle / 2.1).0.floor() as usize), error: 0.0}); 
			});
		let triangle = StarTriangle(2, 3, 5);
		iterator.expected_triangle = Some(Match{input: triangle, output: triangle, weight: 0.0});
		
		assert_eq!(Option::None, iterator.next_pilot(&stars, &mut chunk));
		assert_eq!(7, iterator.index_p);
	}


	
	
	
	#[test]
	// If the triangle is setup AND there are enough stars AND there are matches AND:
	// The star triangle is present in the pilot search.
	// It should work.
	fn test_next_pilot ( )
	{
		let mut stars : Vec<Equatorial> = Vec::new();
		stars.push_back(Equatorial{ra: Radians(0.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(1.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(2.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(3.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(4.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(5.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(6.0), dec: Radians(0.0)});
		let angle = Radians(0.123);
		const NUM_MATCH : usize = 4;
		let mut index = 0;
		let mut iterator: StarTriangleIterator<NUM_MATCH> = StarTriangleIterator::new();
		iterator.begin(angle, &stars);
		
		let mut chunk = MockChunkIterator::new();
		chunk.expect_find_close_ref_region().times(3* 3)
			.returning(move |angle, _, found|
			{ 
				// 0: 0 | 1 | 2
				// 1: 0 | 0 | 1
				// 4: 0 | 0 | 0
				let i = if index % 3 == 0 { 2 } else if index % 3 == 1 { 3 } else { 5 }; 
				found.push_back(SearchResult{result: 
					StarPair(i, (angle / 2.1).0.floor() as usize), error: 0.0});
				index += 1;
			});
		let triangle = StarTriangle(2, 3, 5);
		iterator.expected_triangle = Some(Match{input: triangle, output: triangle, weight: 0.0});
		
		let result = Option::Some(
			Match{input: 4, output: 0, weight: 0.0}
		);
		assert_eq!(result, iterator.next_pilot(&stars, &mut chunk));
		assert_eq!(4, iterator.index_p); // the correct pilot.
	}


//###############################################################################################//
//
//										Step
//
// 	fn step ( &mut self ) -> bool
//
//###############################################################################################//

	#[test]
	// When there is 0 for a max size of any input, false should be returned.
	fn test_step_invalid ( )
	{
		let mut a = -1;
		let mut b = 0;
		let mut c = 0;
		let size_a = 0;
		let size_b = 10;
		let size_c = 10;
		assert!(!StarTriangleIterator::<0>::step(&mut a, &mut b, &mut c, size_a, size_b, size_c));
		
		let size_a = 10;
		let size_b = 0;
		let size_c = 10;
		assert!(!StarTriangleIterator::<0>::step(&mut a, &mut b, &mut c, size_a, size_b, size_c));
		
		let size_a = 10;
		let size_b = 10;
		let size_c = 0;
		assert!(!StarTriangleIterator::<0>::step(&mut a, &mut b, &mut c, size_a, size_b, size_c));
		
		let size_a = 10;
		let size_b = 0;
		let size_c = 0;
		assert!(!StarTriangleIterator::<0>::step(&mut a, &mut b, &mut c, size_a, size_b, size_c));
		
		let size_a = 0;
		let size_b = 0;
		let size_c = 10;
		assert!(!StarTriangleIterator::<0>::step(&mut a, &mut b, &mut c, size_a, size_b, size_c));
		
		let size_a = 0;
		let size_b = 10;
		let size_c = 0;
		assert!(!StarTriangleIterator::<0>::step(&mut a, &mut b, &mut c, size_a, size_b, size_c));
		
		let size_a = 0;
		let size_b = 0;
		let size_c = 0;
		assert!(!StarTriangleIterator::<0>::step(&mut a, &mut b, &mut c, size_a, size_b, size_c));
		
	}

	#[test]
	fn test_step ( )
	{
		let mut a = -1;
		let mut b = 0;
		let mut c = 0;
		let size_a = 3;
		let size_b = 2;
		let size_c = 2;

		assert!(StarTriangleIterator::<0>::step(&mut a, &mut b, &mut c, size_a, size_b, size_c));
		assert_eq!((0, 0, 0), (a, b, c));
		
		assert!(StarTriangleIterator::<0>::step(&mut a, &mut b, &mut c, size_a, size_b, size_c));
		assert_eq!((1, 0, 0), (a, b, c));
		
		assert!(StarTriangleIterator::<0>::step(&mut a, &mut b, &mut c, size_a, size_b, size_c));
		assert_eq!((2, 0, 0), (a, b, c));
		
		
		assert!(StarTriangleIterator::<0>::step(&mut a, &mut b, &mut c, size_a, size_b, size_c));
		assert_eq!((0, 1, 0), (a, b, c));
		
		assert!(StarTriangleIterator::<0>::step(&mut a, &mut b, &mut c, size_a, size_b, size_c));
		assert_eq!((1, 1, 0), (a, b, c));
		
		assert!(StarTriangleIterator::<0>::step(&mut a, &mut b, &mut c, size_a, size_b, size_c));
		assert_eq!((2, 1, 0), (a, b, c));
		
		
		assert!(StarTriangleIterator::<0>::step(&mut a, &mut b, &mut c, size_a, size_b, size_c));
		assert_eq!((0, 0, 1), (a, b, c));
		
		assert!(StarTriangleIterator::<0>::step(&mut a, &mut b, &mut c, size_a, size_b, size_c));
		assert_eq!((1, 0, 1), (a, b, c));
		
		assert!(StarTriangleIterator::<0>::step(&mut a, &mut b, &mut c, size_a, size_b, size_c));
		assert_eq!((2, 0, 1), (a, b, c));
		
		
		assert!(StarTriangleIterator::<0>::step(&mut a, &mut b, &mut c, size_a, size_b, size_c));
		assert_eq!((0, 1, 1), (a, b, c));
		
		assert!(StarTriangleIterator::<0>::step(&mut a, &mut b, &mut c, size_a, size_b, size_c));
		assert_eq!((1, 1, 1), (a, b, c));
		
		assert!(StarTriangleIterator::<0>::step(&mut a, &mut b, &mut c, size_a, size_b, size_c));
		assert_eq!((2, 1, 1), (a, b, c));
		
		
		
		assert!(!StarTriangleIterator::<0>::step(&mut a, &mut b, &mut c, size_a, size_b, size_c));
	}

//###############################################################################################//
//
//										Prep New Kernel
//
// 	fn prep_new_kernel ( &mut self, stars: &dyn List<Equatorial>, database: &dyn Database )-> bool
//
//###############################################################################################//

	#[test]
	// When the kernel is out of steps, it should return false and abort.
	fn test_prep_new_kernel_cant_step ( )
	{
		let stars : Vec<Equatorial> = Vec::new();
		let angle = Radians(0.0);
		const NUM_MATCH : usize = 4;
		let mut iterator: StarTriangleIterator<NUM_MATCH> = StarTriangleIterator::new();
		iterator.begin(angle, &stars);

		let mut chunk = MockChunkIterator::new();
		chunk.expect_find_close_ref_region().times(0);
		chunk.expect_next().times(1).returning(|| return false);
		chunk.expect_begin().times(1).returning(|| return );

		assert!(!iterator.prep_new_kernel(&stars, &mut chunk));
	}


	#[test]
	// Indexing should be set to false.
	// Input should be set the value of the kernel iterator.
	// The database should be called for each star pair corresponding to the kernel:
	// 	- a: i,j
	// 	- b: i,k
	// 	- c: j,k
	// The database must also be called with the given angle tolerance.
	fn test_prep_new_kernel ( )
	{
		let mut stars : Vec<Equatorial> = Vec::new();
		stars.push_back(Equatorial{ra: Radians(0.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(1.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(3.0), dec: Radians(0.0)});
		// stars.push_back(Equatorial{ra: Radians(3.0), dec: Radians(0.0)});
		// stars.push_back(Equatorial{ra: Radians(4.0), dec: Radians(0.0)});
		let angle = Radians(0.123);
		const NUM_MATCH : usize = 4;
		let mut iterator: StarTriangleIterator<NUM_MATCH> = StarTriangleIterator::new();
		iterator.begin(angle, &stars);

		let mut chunk = MockChunkIterator::new();
		chunk.expect_find_close_ref_region().times(3)
			.returning(|angle, _, found|
				found.push_back(SearchResult{result: StarPair(0, angle.0.round() as usize), error: 0.0}).expect(""))
			.withf(|_, tolerance, _| return *tolerance == Radians(0.123) );

		chunk.expect_next().times(1).returning(|| return false);
		chunk.expect_begin().times(1).returning(|| return);

		assert!(iterator.prep_new_kernel(&stars, &mut chunk));
		assert_eq!(iterator.kernel.i, iterator.input.0);
		assert_eq!(iterator.kernel.j, iterator.input.1);
		assert_eq!(iterator.kernel.k, iterator.input.2);
		assert_eq!(StarPair(0, 1), iterator.pair_a.get(0).result); // (0,0) to (0,1)
		assert_eq!(StarPair(0, 3), iterator.pair_b.get(0).result); // (0,0) to (0,2)
		assert_eq!(StarPair(0, 2), iterator.pair_c.get(0).result); // (0,1) to (0,2)
		assert_eq!(iterator.index_a, -1);
		assert_eq!(iterator.index_b, 0);
		assert_eq!(iterator.index_c, 0);
	}


	
//###############################################################################################//
//
//										Prep New Pilot
//
// 	fn prep_new_pilot ( &mut self, stars: &dyn List<Equatorial>, database: &dyn Database )-> bool
//
//###############################################################################################//

	#[test]
	// If a star triangle could not be found, a pilot should not be found.
	fn test_prep_new_pilot_triangle_invalid ( )
	{
		let mut stars : Vec<Equatorial> = Vec::new();
		stars.push_back(Equatorial{ra: Radians(0.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(1.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(3.0), dec: Radians(0.0)});
		let angle = Radians(0.123);
		const NUM_MATCH : usize = 4;
		let mut iterator: StarTriangleIterator<NUM_MATCH> = StarTriangleIterator::new();
		iterator.begin(angle, &stars);
		
		let mut chunk = MockChunkIterator::new();
		chunk.expect_find_close_ref_region().times(0);
		assert!(!iterator.prep_new_pilot(&stars, &mut chunk));
	}
	
	
	
	#[test]
	// The index should avoid indices in the star triangle.
	fn test_prep_new_pilot_triangle_valid ( )
	{
		let mut stars : Vec<Equatorial> = Vec::new();
		stars.push_back(Equatorial{ra: Radians(0.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(0.1), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(0.2), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(0.3), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(0.4), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(0.5), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(0.6), dec: Radians(0.0)});
		let angle = Radians(0.123);
		const NUM_MATCH : usize = 4;
		let mut iterator: StarTriangleIterator<NUM_MATCH> = StarTriangleIterator::new();
		iterator.begin(angle, &stars);
	
		let mut chunk = MockChunkIterator::new();
		chunk.expect_find_close_ref_region().times(3 * 4)
			.returning(|angle, _, found|
				{
					let pair = StarPair(0, (angle.0 * 10.0).round() as usize);
					let _=found.push_back(
						SearchResult{result: pair, error: 0.0});
				})
			.withf(|_, tolerance, _| return *tolerance == Radians(0.123) );
			
		iterator.expected_triangle = Some(Match{
			input: StarTriangle(2, 4, 5), 
			output: StarTriangle(0, 0, 0), 
			weight: 0.0});
	
		assert!(iterator.prep_new_pilot(&stars, &mut chunk));
		assert_eq!(iterator.index_p, 0);
		assert_eq!(iterator.index_p_a, -1);
		assert_eq!(iterator.index_p_b,  0);
		assert_eq!(iterator.index_p_c,  0);
		assert_eq!(iterator.pair_p_a.get(0).result, StarPair(0, 2)); // 0 to 2
		assert_eq!(iterator.pair_p_b.get(0).result, StarPair(0, 4)); // 0 to 4
		assert_eq!(iterator.pair_p_c.get(0).result, StarPair(0, 5)); // 0 to 5
		
		assert!(iterator.prep_new_pilot(&stars, &mut chunk));
		assert_eq!(iterator.index_p, 1);
		assert_eq!(iterator.index_p_a, -1);
		assert_eq!(iterator.index_p_b,  0);
		assert_eq!(iterator.index_p_c,  0);
		assert_eq!(iterator.pair_p_a.get(0).result, StarPair(0, 1)); // 1 to 2
		assert_eq!(iterator.pair_p_b.get(0).result, StarPair(0, 3)); // 1 to 4
		assert_eq!(iterator.pair_p_c.get(0).result, StarPair(0, 4)); // 1 to 5
		
		assert!(iterator.prep_new_pilot(&stars, &mut chunk));
		assert_eq!(iterator.index_p, 3);
		assert_eq!(iterator.index_p_a, -1);
		assert_eq!(iterator.index_p_b,  0);
		assert_eq!(iterator.index_p_c,  0);
		assert_eq!(iterator.pair_p_a.get(0).result, StarPair(0, 1)); // 3 to 2
		assert_eq!(iterator.pair_p_b.get(0).result, StarPair(0, 1)); // 3 to 4
		assert_eq!(iterator.pair_p_c.get(0).result, StarPair(0, 2)); // 3 to 5
		
		assert!(iterator.prep_new_pilot(&stars, &mut chunk));
		assert_eq!(iterator.index_p, 6);
		assert_eq!(iterator.index_p_a, -1);
		assert_eq!(iterator.index_p_b,  0);
		assert_eq!(iterator.index_p_c,  0);
		assert_eq!(iterator.pair_p_a.get(0).result, StarPair(0, 4)); // 6 to 2
		assert_eq!(iterator.pair_p_b.get(0).result, StarPair(0, 2)); // 6 to 4
		assert_eq!(iterator.pair_p_c.get(0).result, StarPair(0, 1)); // 6 to 5
		
		assert!(!iterator.prep_new_pilot(&stars, &mut chunk));
	}
	
	
	



}
