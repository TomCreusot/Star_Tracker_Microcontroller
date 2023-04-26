///! Implementation for StarTriangleIterator
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
		let mut tries : Option<Match<StarTriangle<usize>>> = None;
		'_outer: loop // This is the correct use of a do while loop.
		{
			// Once all possiblities for a single kernal step are exhausted.
			// The kernal will step, new stars will be chosen and a list of database matches are generated.
			while !self.step()
			{
				if !self.prep_new_kernel(stars, database)
				{
					break '_outer; // Rust implementation of a do while loop.
				}
			}
			
			let a = self.pair_a.get(self.index_a);
			let b = self.pair_b.get(self.index_b);
			let c = self.pair_c.get(self.index_c);
			let triangle = StarTriangle::construct_triangle(a.result, b.result, c.result);

			// Leaves the loop if a, b and c sides of the database are connected.
			if triangle.is_some()
			{
				let input  = self.input;
				let output = triangle.unwrap();
				let error  = a.error + b.error + c.error;
				tries = Some(Match{input: input, output: output, weight: error});
				break '_outer; // Rust implementation of a do while loop.
			}
		}
		return tries;
	}

	/// Prepares the StarTriangleIterator for iterating.
	/// # Arguments
	/// * `angle_tolerance` - When searching the database, the tolerance to use.
	/// * `stars` - The observed stars.
	///
	/// # Example
	/// ```
	/// use star_tracker::tracking_mode::StarTriangleIterator;
	/// use star_tracker::tracking_mode::TriangleConstruct;
	/// use star_tracker::util::units::Radians;
	/// use star_tracker::util::units::Equatorial;
	///
	/// // The stars found in the image.
	/// let obs_stars : Vec<Equatorial>=vec![Equatorial::zero(),Equatorial::zero()];
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
		self.indexing = false;
		self.index_a = 0;
		self.index_b = 0;
		self.index_c = 0;
		self.input = StarTriangle(0,0,0);
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
			indexing: false,
			index_a: 0,
			index_b: 0,
			index_c: 0,
			input: StarTriangle(0,0,0),
			angle_tolerance: Radians(0.0),
		};
	}



	/// Steps the index of a, b and c to get a new value.
	/// # Returns
	/// False if the sequence ended.
	fn step ( &mut self ) -> bool
	{
		// println!("  STEP");
		// If new kernal was made, the values must be reset.
		if !self.indexing
		{
			self.index_a = 0;
			self.index_b = 0;
			self.index_c = 0;
			self.indexing = true;
			return 0 < self.pair_a.size() && 0 < self.pair_b.size() && 0 < self.pair_c.size();
		}

		if self.index_a < self.pair_a.size() - 1
		{
			self.index_a += 1;
		}
		else
		{
			self.index_a = 0;

			if self.index_b < self.pair_b.size() - 1
			{
				self.index_b += 1;
			}
			else
			{
				self.index_b = 0;

				if self.index_c < self.pair_c.size() - 1
				{
					self.index_c += 1;
				}
				else
				{
					return false;
				}
			}
		}
		return true;
	}




	/// When a new kernal step is required:
	/// - Finds the angle distance between the stars.
	/// - Finds a long list of matches from the database to compare with.
	/// # Arguments
	/// * `stars` - The stars in the image.
	/// * `database` - The database where the stars can be searched.
	fn prep_new_kernel ( &mut self, stars: &dyn List<Equatorial>, database: &mut dyn ChunkIterator ) -> bool
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
		self.indexing = false;

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
		
		return true;
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
//										New
//
// fn new <NUMBER_MATCHES> ( stars : &dyn List<Equatorial> )
//
//###############################################################################################//

	#[test]
	// The required variables to be set are:
	// - kernel (requires the number of stars to begin).
	// - indexing (must be false otherwise it will start the iterations 1 step early).
	// - pair_a, pair_b, pair_c capacity must be the value of N.
	fn test_begin ( )
	{
		let stars : Vec<Equatorial>=vec![Equatorial::zero(),Equatorial::zero(),Equatorial::zero()];
		let angle = Radians(1.0);
		const NUM_MATCH : usize = 4;
		let mut iterator: StarTriangleIterator<NUM_MATCH> = StarTriangleIterator::new();
		iterator.begin(angle, &stars);
		assert!(!iterator.indexing);
		assert_eq!(stars.size(), iterator.kernel.size);
		assert_eq!(NUM_MATCH, iterator.pair_a.capacity());
		assert_eq!(NUM_MATCH, iterator.pair_b.capacity());
		assert_eq!(NUM_MATCH, iterator.pair_c.capacity());
	}




//###############################################################################################//
//
//										Next Match
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
		assert!(iterator.indexing);
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
	}




//###############################################################################################//
//
//										Step
//
// 	fn step ( &mut self ) -> bool
//
//###############################################################################################//

	#[test]
	// If pair a/b/c has 0 elements,
	// - step() will return false.
	// - Index a/b/c will be 0.
	// - indexing will be true.
	fn test_step_indexing_0_elements ( )
	{
		let stars : Vec<Equatorial> = Vec::new();
		let angle = Radians(0.0);
		const NUM_MATCH : usize = 4;
		let mut iterator: StarTriangleIterator<NUM_MATCH> = StarTriangleIterator::new();
		iterator.begin(angle, &stars);

		check::<NUM_MATCH>(&mut iterator); 				// a: 0, b: 0, c: 0
		iterator.pair_a.push_back(SearchResult{result: StarPair(0,0), error: 0.0});
		check::<NUM_MATCH>(&mut iterator);				// a: 1, b: 0, c: 0
		iterator.pair_b.push_back(SearchResult{result: StarPair(0,0), error: 0.0});
		check::<NUM_MATCH>(&mut iterator); 				// a: 1, b: 1, c: 0
		iterator.pair_a.pop_back();
		check::<NUM_MATCH>(&mut iterator); 				// a: 0, b: 1, c: 0
		iterator.pair_c.push_back(SearchResult{result: StarPair(0,0), error: 0.0});
		check::<NUM_MATCH>(&mut iterator); 				// a: 0, b: 1, c: 1
		iterator.pair_b.pop_back();
		check::<NUM_MATCH>(&mut iterator); 				// a: 0, b: 0, c: 1
		iterator.pair_a.push_back(SearchResult{result: StarPair(0,0), error: 0.0});
		check::<NUM_MATCH>(&mut iterator); 				// a: 1, b: 0, c: 1

		fn check <const N : usize> ( iter: &mut StarTriangleIterator<N> )
		{
			iter.indexing = false;
			assert!(!iter.step());
			assert!(iter.indexing);
			assert_eq!(0, iter.index_a);
			assert_eq!(0, iter.index_b);
			assert_eq!(0, iter.index_c);
		}
	}


	#[test]
	// step() should follow the sequence:
	// index_a: 0..pair_a.size()
	// index_b: 0..pair_b.size() [on completion of a].
	// index_c: 0..pair_c.size() [on completion of b].
	fn test_step_indexing ( )
	{
		let stars : Vec<Equatorial> = Vec::new();
		let angle = Radians(0.0);
		const NUM_MATCH : usize = 4;
		let mut iterator: StarTriangleIterator<NUM_MATCH> = StarTriangleIterator::new();
		iterator.begin(angle, &stars);

		let _ = iterator.pair_a.push_back(SearchResult{result: StarPair(0,0), error: 0.0});
		let _ = iterator.pair_a.push_back(SearchResult{result: StarPair(0,1), error: 0.0});
		let _ = iterator.pair_b.push_back(SearchResult{result: StarPair(1,0), error: 0.0});
		let _ = iterator.pair_c.push_back(SearchResult{result: StarPair(2,0), error: 0.0});
		let _ = iterator.pair_c.push_back(SearchResult{result: StarPair(2,1), error: 0.0});
		let _ = iterator.pair_c.push_back(SearchResult{result: StarPair(2,2), error: 0.0});
		let _ = iterator.pair_c.push_back(SearchResult{result: StarPair(2,3), error: 0.0});

		assert!(iterator.step());
		assert_eq!((0, 0, 0), (iterator.index_a, iterator.index_b, iterator.index_c));
		assert!(iterator.step());
		assert_eq!((1, 0, 0), (iterator.index_a, iterator.index_b, iterator.index_c));
		assert!(iterator.step());
		assert_eq!((0, 0, 1), (iterator.index_a, iterator.index_b, iterator.index_c));
		assert!(iterator.step());
		assert_eq!((1, 0, 1), (iterator.index_a, iterator.index_b, iterator.index_c));
		assert!(iterator.step());
		assert_eq!((0, 0, 2), (iterator.index_a, iterator.index_b, iterator.index_c));
		assert!(iterator.step());
		assert_eq!((1, 0, 2), (iterator.index_a, iterator.index_b, iterator.index_c));
		assert!(iterator.step());
		assert_eq!((0, 0, 3), (iterator.index_a, iterator.index_b, iterator.index_c));
		assert!(iterator.step());
		assert_eq!((1, 0, 3), (iterator.index_a, iterator.index_b, iterator.index_c));
		assert!(!iterator.step());
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
		const NUM_MATCH : usize = 0;
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
		stars.push_back(Equatorial{ra: Radians(2.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(3.0), dec: Radians(0.0)});
		stars.push_back(Equatorial{ra: Radians(4.0), dec: Radians(0.0)});
		let angle = Radians(0.123);
		const NUM_MATCH : usize = 4;
		let mut iterator: StarTriangleIterator<NUM_MATCH> = StarTriangleIterator::new();
		iterator.begin(angle, &stars);

		// let mut database = MockDatabase::new();
		// database.expect_find_close_ref().times(3)
		// 	.returning(|angle, _, found| found.push_back(StarPair(0, angle.0 as usize)).expect(""))
		// 	.withf(|_, tolerance, _| return *tolerance == Radians(0.123) );

		let mut chunk = MockChunkIterator::new();
		chunk.expect_find_close_ref_region().times(3)
			.returning(|angle, _, found| 
				found.push_back(SearchResult{result: StarPair(0, angle.0 as usize), error: 0.0}).expect(""))
			.withf(|_, tolerance, _| return *tolerance == Radians(0.123) );
			
		// let mut count = 0;
		// chunk.expect_next().times(1).returning(move || {count += 1; return count < 2;});
		chunk.expect_next().times(1).returning(|| return false);
		chunk.expect_begin().times(1).returning(|| return);

		assert!(iterator.prep_new_kernel(&stars, &mut chunk));
		assert!(!iterator.indexing);
		assert_eq!(iterator.kernel.i, iterator.input.0);
		assert_eq!(iterator.kernel.j, iterator.input.1);
		assert_eq!(iterator.kernel.k, iterator.input.2);
		assert_eq!(StarPair(0, 0), iterator.pair_a.get(0).result); // (0,0) to (0,1)
		assert_eq!(StarPair(0, 2), iterator.pair_b.get(0).result); // (0,0) to (0,2)
		assert_eq!(StarPair(0, 1), iterator.pair_c.get(0).result); // (0,1) to (0,2)
	}







}
