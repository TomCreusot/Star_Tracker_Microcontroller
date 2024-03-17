//! Implementation of ChunkIterator

#[cfg(test)] use mockall::predicate::*;
#[cfg(test)] use mockall::*;

use crate::core_include::*;

use crate::tracking_mode::database::Database;
use crate::tracking_mode::database::SearchResult;
use crate::tracking_mode::database::ChunkAreaSearch;
use crate::tracking_mode::database::ChunkIteratorNone;
use crate::tracking_mode::database::ChunkIteratorEquatorial;
use crate::tracking_mode::database::ChunkIteratorDeclination;

use crate::util::aliases::Decimal;
use crate::util::aliases::M_PI;
use crate::util::units::Equatorial;
use crate::util::units::Radians;
use crate::util::units::Degrees;
use crate::util::list::List;

use crate::util::Maths;


#[cfg_attr(test, automock)]
pub trait ChunkIterator
{
	/// Resets the iterator.  
	/// Call this to return to the start, then call next.
	fn begin ( &mut self );
	
	/// Steps to the next region.
	/// This must be called to begin.
	/// If there is no more regions, next will return false.
	fn next ( &mut self ) -> bool;
	
	/// Finds close matches to the provided angular separation and returns the star pair reference.
	/// The an element of the star pair reference can be inserted into `find_star` to get the actual location.
	/// # Arguments
	/// * `find` - The angular separation between the found stars to find in the database.
	/// * `found` - The closest matches to the provided `find`.
	/// # Returns
	/// The fields at the end of each pair.
	/// Use this information so that when finding multiple pairs connected, they can span multiple regions.
	fn find_close_ref_region ( &self, find : Radians, tolerance: Radians, found : &mut dyn List<SearchResult> )
	{
		let range = self.get_database().find_close_ref_range(find, tolerance);
		for i in range.clone()
		{
			let pair   = self.get_database().get_pairs(i);
			let error  = 1.0;
			// let error  =(if i < mid { i } else { range.end - i } - range.start) as Decimal / range.len() as Decimal;
			if !found.is_full() && self.same_region(i)
			{
				let result = SearchResult{result: pair, error: error };
				let _ = found.push_back(result);
			}
		}
	}

	/// Returns the database this iterator holds.
	fn get_database ( &self ) -> &dyn Database;
	
	/// Returns true if the pair at the provided index fits the current region.
	fn same_region ( &self, pair_index: usize ) -> bool;
	
}


impl <'a> ChunkIteratorNone <'a>
{
	/// Constructor.  
	/// Feed the database and then call next.
	pub fn new ( database: &'a dyn Database ) -> Self 
	{ return Self{database: database, started: false};  }
}

impl <'a> ChunkIterator for ChunkIteratorNone <'a>
{
	/// Resets the iterator.  
	/// Call this to return to the start, then call next.
	fn begin ( &mut self ) { self.started = false; }
	
	/// ChunkIterator does not have regions, next returns true then always false after begin.
	fn next ( &mut self ) -> bool 
	{
		let result = self.started; 
		self.started = true; 
		return !result;
	}
	
	/// Returns the database this iterator holds.
	fn get_database ( &self ) -> &dyn Database { return self.database; }
	
	/// Returns true as there is only one region.
	fn same_region ( &self, _pair_index: usize ) -> bool { return true; }
}





















impl <'a> ChunkIteratorDeclination <'a>
{
	
	
	/// Creates a new iterator.  
	/// Use `begin` to start iterating then use `next` to step.
	/// Ensure step is smaller than the field of view to ensure full coverage.
	/// # Arguments
	/// * `step` -  
	///            How far the iterator jumps between iterations, this is the separation between the centers of chunks,  
	///            Make this proportional to the database diagonal field of view.
	///
	/// * `size_multiplier` - 
	///            The percentage of overlap between chunks, 0 means the chunks have no overlap.
	///            For the best results, use a number greater than 0 as it will ensure that all stars are covered.
	///
	/// * `randomiser` -
	///            fn randomiser ( current step, number of steps ) -> index of chunk (must cover all chunks starting from 0 to n - 1).
	///            Use `ChunkIteratorDeclination::randomise_parity`.  
	///            This is here becasue the chunks are designed to overlap.  
	///            Because they overlap, it is more likely that neigbouring chunks will produce the same result.  
	///            To avoid testing the same area twice in a row, te randomiser function should input the step index and the number of steps and output the chunk to look at.  
	///            ChunkIteratorDeclination::randomise_parity will do all the even chunks followed by the odd chunks ensuring that the order does not overlap. 

	pub fn new ( 
		database: &'a dyn Database, 
		step: Radians, size_addition: Decimal, 
		randomiser: fn (usize, usize) -> usize ) -> Self
	{
		let true_step = Self::widen_integer_step(step);
		let num =  (Degrees(180.0).as_radians() / true_step).0.floor() as usize + 1;
		return Self
		{
			database:              database,
			randomiser:            randomiser,
			index:                 0,
			num:                   num,
			dec:                   Radians(0.0)..Radians(0.0),
			chunk_step:            true_step,
			chunk_size_multiplier: 1.0 + size_addition
		}
	}
	
	
	/// Generates a randomised version of the index.  
	/// This allows you to skip neigbouring bands so overlapping stars are ignored till the end.  
	/// The randomisation is to use every odd band followed by every even band.
	pub fn randomise_parity ( index: usize, num_elements: usize ) -> usize
	{
		if index < num_elements.div_ceil(2)
		{
			return index * 2;
		}
		
		// Odd numbers start at ceil(num / 2)
		return (index - num_elements.div_ceil(2)) * 2 + 1;
	}
	
	
	
	/// Provides no randomisation, index 1 = chunk 1...
	pub fn randomise_none ( index: usize, _num_elements: usize ) -> usize
	{
		return index;
	}
	
	/// Widens the step size so that there is an integer number of steps of equal distance.
	/// # Arguments
	/// * `step` - widens the step
	pub fn widen_integer_step ( step: Radians ) -> Radians
	{
		// Ensures that if 90.0001 is given, it will round to 90 instead of 180
		let overflow = Degrees(0.2).as_radians(); 
		
		// NAN is created if exceeding 180.0.
		if Degrees(180.0).as_radians() + overflow < step
		{
			return Degrees(360.0).to_radians();
		}
		
		// Below 180 deg
		let diviser = Degrees(180.0).as_radians() / step.0;
		return Degrees(180.0).as_radians() / (diviser + overflow).0.floor();
	}
}


impl <'a> ChunkIterator for ChunkIteratorDeclination <'a>
{
	
	/// Resets this iterator.
	/// Call this to return to the start, then call next.
	fn begin ( &mut self )
	{
		self.index = 0;
		self.dec = Radians(0.0)..Radians(0.0);
	}
	
	/// Steps to the next region.
	/// This must be called after begin.
	/// If there is no more regions, next will return false.
	fn next ( &mut self ) -> bool
	{
		if self.num <= self.index { return false; }
		let actual_index = (self.randomiser)(self.index, self.num);
		// let actual_index = self.index;
		let half_step = self.chunk_step * self.chunk_size_multiplier / 2.0;
		let dec       = Radians(actual_index as Decimal * self.chunk_step.0) - Degrees(90.0).as_radians();
		self.dec = dec - half_step .. dec + half_step;
		self.index += 1;
		return true;
	}
	
	fn get_database ( &self ) -> &dyn Database { return self.database; }
	
	
	fn same_region ( &self, pair_index: usize ) -> bool
	{
		let pair = self.database.get_pairs(pair_index);
		let p1 = self.database.find_star(pair.0);
		let p2 = self.database.find_star(pair.1);
		
		if p1.is_err() || p2.is_err()
		{
			return false;
		}
		
		let valid_dec = 
			(self.dec.start <= p1.unwrap().dec && p1.unwrap().dec <= self.dec.end) ||
			(self.dec.start <= p2.unwrap().dec && p2.unwrap().dec <= self.dec.end);

		return valid_dec;
	}
	
	
}






impl <'a> ChunkIteratorEquatorial <'a>
{	
	/// Creates a new iterator.  
	/// Use `begin` to start iterating then use `next` to step.
	/// Ensure step is smaller than the field of view to ensure full coverage.
	/// # Arguments
	/// * `step` -  
	///            How far the iterator jumps between iterations, this is the separation between the centers of chunks,  
	///            Make this proportional to the database diagonal field of view.
	///
	/// * `size_multiplier` - 
	///            The percentage of overlap between chunks, 0 means the chunks have no overlap.
	///            For the best results, use a number greater than 0 as it will ensure that all stars are covered.

	pub fn new ( database: &'a dyn Database, step: Radians, chunk_size_multiplier: Decimal ) -> Self 
	{
		return Self
		{
			index_dec:             0,
			index_ra:              0,
			database:              database,
			dec:                   Radians(0.0)..Radians(0.0),
			ra:                    Radians(0.0)..Radians(0.0),
			num_dec:               (Degrees(180.0).to_radians() / step).0.ceil() as usize,
			num_ra:                0,
			chunk_step:            step,
			chunk_size_multiplier: 1.0 + chunk_size_multiplier,
		}
	}
}

impl <'a> ChunkIterator for ChunkIteratorEquatorial <'a>
{
	/// Resets the iterator.  
	/// Call this to return to the start, then call next.
	fn begin ( &mut self ) 
	{
		self.index_dec   = 0;
		self.index_ra    = 0;
		self.dec         = Radians(0.0)..Radians(0.0);
		self.ra          = Radians(0.0)..Radians(0.0);
		self.num_ra      = 0;
	}
	
	/// Steps to the next region.
	/// This must be called to begin.
	/// If there is no more regions, next will return false.
	fn next ( &mut self ) -> bool 
	{	
		
		// If there has not been enough chunks to wrap around the declination band yet.
		if self.index_ra < self.num_ra
		{
			let step = Degrees(360.0).as_radians() / self.num_ra as Decimal;
			let size = step * self.chunk_size_multiplier;
			
			// Move to next right ascension node.
			let center = step * (self.index_ra as Decimal) + step / 2.0;
			self.ra = center - size / 2.0 .. center + size / 2.0; 
			
			
			self.index_ra += 1;
			return true;
		}
		else if self.index_dec == 0
		{
			let up = Degrees(90.0).as_radians();
			let overshoot = self.chunk_step * (self.chunk_size_multiplier - 1.0) / 2.0;
			self.dec = -up.. self.chunk_step -up + overshoot;
			self.num_ra = 0;
			self.ra = Radians(0.0) .. Degrees(360.0).as_radians();
			self.index_ra   = 0;
			self.index_dec += 1;
			return true;
			
		}
		else if self.index_dec == self.num_dec - 1
		{
			let up = Degrees(90.0).as_radians();
			let overshoot = self.chunk_step * (self.chunk_size_multiplier - 1.0) / 2.0;
			self.dec = up -self.chunk_step-overshoot..up;
			self.num_ra = 0;
			self.ra = Radians(0.0) .. Degrees(360.0).as_radians();
			self.index_ra   = 0;
			self.index_dec += 1;
			return true;
		}
		// If a declination band is completed.
		else if self.index_dec < self.num_dec
		{
			let dec_size = self.chunk_step * self.chunk_size_multiplier;
			let center = self.chunk_step / 2.0 + self.chunk_step * 
				(self.index_dec as Decimal)-Degrees(90.0).as_radians();
			self.dec = center - dec_size / 2.0 .. center + dec_size / 2.0;
			
			// eq_dec_band is the closest point of the camera shot to the equator.
			let mut eq_dec_band = Radians(center.abs()) - dec_size / 2.0;
			if eq_dec_band.0 < 0.0 { eq_dec_band.0 = 0.0; }
			
			// Find the field of view of the declination band (angle to revolve around it from [0,0,0]).
			// This is coincidently cos(declination) * 360 degrees.
			let angle =  Radians(eq_dec_band.cos() * 2.0 * M_PI);
			
			// Number of photos = field of view of declination band / field of view of the sensor
			self.num_ra = (angle / self.chunk_step).0.ceil() as usize;
			self.index_ra = 0;
			
			
			self.index_dec += 1;
			// Move to the first right ascension band or to the next declination.
			return self.next();
		}
		
		return false;
	}
	
	/// Returns the database this iterator holds.	
	fn get_database ( &self ) -> &dyn Database { return self.database; }
	
	/// Returns true if the pair at the provided index fits the current region.
	fn same_region ( &self, pair_index: usize ) -> bool
	{
		let pair = self.database.get_pairs(pair_index);
		let p1 = self.database.find_star(pair.0);
		let p2 = self.database.find_star(pair.1);
		
		if p1.is_err() || p2.is_err()
		{
			return false;
		}
		let valid_dec = 
			(self.dec.start <= p1.unwrap().dec && p1.unwrap().dec <= self.dec.end) ||
			(self.dec.start <= p2.unwrap().dec && p2.unwrap().dec <= self.dec.end);

		let mut valid_ra = 
			(self.ra.start <= p1.unwrap().ra && p1.unwrap().ra <= self.ra.end) ||
			(self.ra.start <= p2.unwrap().ra && p2.unwrap().ra <= self.ra.end);
			
		if Degrees(360.0).as_radians() < self.ra.end
		{
			valid_ra |= p1.unwrap().ra <= self.ra.end - Degrees(360.0).as_radians();
			valid_ra |= p2.unwrap().ra <= self.ra.end - Degrees(360.0).as_radians();
		}
		else if self.ra.start < Radians(0.0)
		{
			valid_ra |= self.ra.start + Degrees(360.0).as_radians() <= p1.unwrap().ra;
			valid_ra |= self.ra.start + Degrees(360.0).as_radians() <= p2.unwrap().ra;
		}

		return valid_dec && valid_ra;
	}
}















impl <'a> ChunkAreaSearch <'a>
{
	/// Uses the given ranges as the search area.
	pub fn from_range ( database: &'a dyn Database, 
		bounds_ra: Range<Radians>, bounds_dec: Range<Radians> ) -> Self
	{
		return Self { database: database, ra: bounds_ra, dec: bounds_dec, started: false };
	}
	
	/// Constructs the search area as 1/2 fov from the center point on the ra and dec axis.
	pub fn from_point ( database: &'a dyn Database, center: Equatorial, fov: Radians ) -> Self
	{
		let range_ra  = center.ra  - fov / 2.0 .. center.ra  + fov / 2.0;
		let range_dec = center.dec - fov / 2.0 .. center.dec + fov / 2.0;
		return Self { database: database, ra: range_ra, dec: range_dec, started: false };
	}
}



impl<'a> ChunkIterator for ChunkAreaSearch <'a>
{
	/// Allows you to use `next`.  
	fn begin ( &mut self ) { self.started = false; }
	
	/// Returns true then always false until begin is returned again.
	fn next ( &mut self ) -> bool 
	{
		let val = self.started;
		self.started = true; 
		return !val; 
	}

	/// Returns the database this iterator holds.
	fn get_database ( &self ) -> &dyn Database { return self.database; }


	/// Returns true if the pair at the provided index fits the current chunk.
	fn same_region ( &self, pair_index: usize ) -> bool
	{
		let pair = self.database.get_pairs(pair_index);
		let p1 = self.database.find_star(pair.0);
		let p2 = self.database.find_star(pair.1);
		
		if p1.is_err() || p2.is_err()
		{
			return false;
		}
		let valid_dec = 
			(self.dec.start <= p1.unwrap().dec && p1.unwrap().dec <= self.dec.end) ||
			(self.dec.start <= p2.unwrap().dec && p2.unwrap().dec <= self.dec.end);

		let mut valid_ra = 
			(self.ra.start <= p1.unwrap().ra && p1.unwrap().ra <= self.ra.end) ||
			(self.ra.start <= p2.unwrap().ra && p2.unwrap().ra <= self.ra.end);
			
		if Degrees(360.0).as_radians() < self.ra.end
		{
			valid_ra |= p1.unwrap().ra <= self.ra.end - Degrees(360.0).as_radians();
			valid_ra |= p2.unwrap().ra <= self.ra.end - Degrees(360.0).as_radians();
		}
		else if self.ra.start < Radians(0.0)
		{
			valid_ra |= self.ra.start + Degrees(360.0).as_radians() <= p1.unwrap().ra;
			valid_ra |= self.ra.start + Degrees(360.0).as_radians() <= p2.unwrap().ra;
		}

		return valid_dec && valid_ra;
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
	use crate::tracking_mode::StarPair;
	
	use crate::tracking_mode::database::ChunkIterator;
	use crate::tracking_mode::database::ChunkAreaSearch;
	use crate::tracking_mode::database::ChunkIteratorNone;
	use crate::tracking_mode::database::ChunkIteratorEquatorial;
	use crate::tracking_mode::database::ChunkIteratorDeclination;
	
	use crate::tracking_mode::database::MockDatabase;

	use crate::util::units::Equatorial;
	use crate::util::units::Radians;
	use crate::util::units::Degrees;
	
	use crate::util::aliases::Decimal;
	use crate::util::test::TestEqual;
	use crate::util::err::Errors;



//###############################################################################################//
//
//										ChunkIteratorNone
//
// pub fn new   ( &'a dyn Database ) -> Self;
// pub fn begin ( &mut self );
// pub fn next  ( &mut self );
// pub fn get_database ( &self );
// pub fn same_region  ( &self, usize );
// pub fn find_close_ref_region ( &self, find : Radians, 
//                           tolerance: Radians, found : &mut dyn List<SearchResult> )
//
//###############################################################################################//
//										~ new ~													 //
	#[test]
	fn test_none_new ( )
	{
		let mut db = MockDatabase::new();
		db.expect_find_star().times(1).returning(|_| return Ok(Equatorial::zero()));
		let iter = ChunkIteratorNone::new(&db);
		let _ = iter.database.find_star(0);
	}
	
//										~ begin ~												 //
	#[test]
	// no calls to the database should occur, there is no other variables.
	fn test_none_begin ( )
	{
		let db = MockDatabase::new();
		let mut iter = ChunkIteratorNone::new(&db);
		iter.begin();
		assert_eq!(iter.started, false);
		iter.started = true;
		iter.begin();
		assert_eq!(iter.started, false);
	}

//										~ next ~												 //
	#[test]
	fn test_none_next ( )
	{
		let db = MockDatabase::new();
		let mut iter = ChunkIteratorNone::new(&db);
		iter.begin();
		assert!(iter.next(), "Must return true after begin is called.");
		assert!(!iter.next(), "Must return false if next was called previously.");
		iter.begin();
		assert!(iter.next(), "Must return true after begin is called.");
		assert!(!iter.next(), "Must return false if next was called previously.");
	}
	
	
//										~ get_database ~										 //
	#[test]
	fn test_none_get_database ( )
	{
		let mut db = MockDatabase::new();
		db.expect_find_star().times(1).returning(|_| return Ok(Equatorial::zero()));
		let iter = ChunkIteratorNone::new(&db);
		let _ = iter.get_database().find_star(0);
	}
	
//										~ same_region ~											 //
	#[test]
	// must return true.
	fn test_none_same_region ( )
	{
		let db = MockDatabase::new();
		let mut iter = ChunkIteratorNone::new(&db);
		iter.begin();
		assert!(iter.same_region(0), "Must always be true.");
		iter.next();
		assert!(iter.same_region(1), "Must always be true.");
		iter.next();
		assert!(iter.same_region(2), "Must always be true.");
	}
	

//							~ find_close_ref_region ~											 //
	// The following stars are in completely different regions, this should be accepted by None.
	#[test]
	fn test_none_find_close_ref_region ( )
	{
		let mut db = MockDatabase::new();
		let find      = Radians(0.1);
		let tolerance = Radians(0.2);

		db.expect_find_close_ref_range()
			.times(1)
			.returning(|_,_| 5..6)
			.withf(move |find_, tolerance_| return find == *find_ && tolerance == *tolerance_);
		
		// Not used as same_pair does nothing.
		// db.expect_find_star()
		// 	.times(2)
		// 	.returning ( move |index|
		// 		{
		// 			return Ok(stars[index - 4]);
		// 		});
		
		db.expect_get_pairs().times(1)
			.returning ( move |_|
			{
				return StarPair(4, 5);
			});
				
		let mut iter = ChunkIteratorNone::new(&db);
		let mut found = Vec::new();
		iter.begin();
		iter.next();
		iter.find_close_ref_region(find, tolerance, &mut found);
		assert_eq!(found[0].result, StarPair(4, 5));
	}


//###############################################################################################//
//
//										ChunkIteratorDeclination
//
// pub fn new   ( &'a dyn Database, Radians, Decimal ) -> Self;
// pub fn randomise_parity   ( index: usize, num_elements: usize ) -> usize;
// pub fn widen_integer_step   ( Radians ) -> Radians;
//
// pub fn begin ( &mut self );
// pub fn next  ( &mut self );
// pub fn get_database ( &self );
// pub fn same_region  ( &self, usize );
//
//###############################################################################################//
//											~ new ~												 //
	#[test]
	// the step size should be rounded to 90 if it is between 60 and 90.
	fn test_declination_new_single_band_close_ceil ( )
	{
		let db = MockDatabase::new();
		let iterator = ChunkIteratorDeclination::new(&db, Degrees(88.0).as_radians(), 0.0, ChunkIteratorDeclination::randomise_parity);
		assert_eq!(iterator.chunk_size_multiplier, 1.0); // Its a multiplier
		assert_eq!(iterator.num, 3);
		assert_eq!(iterator.dec.start, Radians(0.0));
		assert_eq!(iterator.dec.end, Radians(0.0));
		Degrees(90.0).as_radians().test_equal(&iterator.chunk_step);
	}
	#[test]
	// the step size should be rounded to 60 if it is between 36 and 60.
	fn test_declination_new_single_band_close_floor ( )
	{
		let db = MockDatabase::new();
		let iterator = ChunkIteratorDeclination::new(&db, Degrees(45.1).as_radians(), 1.0, ChunkIteratorDeclination::randomise_parity);
		assert_eq!(iterator.chunk_size_multiplier, 2.0); // Its a multiplier
		assert_eq!(iterator.num, 4);
		assert_eq!(iterator.dec.start, Radians(0.0));
		assert_eq!(iterator.dec.end, Radians(0.0));
		Degrees(60.0).as_radians().test_equal(&iterator.chunk_step);
	}
	
//										~ randomise_parity ~										 //
	#[test]
	// Should work with an even number of elements.
	fn test_randomise_parity_even ( )
	{
		assert_eq!(ChunkIteratorDeclination::randomise_parity(0, 10), 0);
		assert_eq!(ChunkIteratorDeclination::randomise_parity(1, 10), 2);
		assert_eq!(ChunkIteratorDeclination::randomise_parity(2, 10), 4);
		assert_eq!(ChunkIteratorDeclination::randomise_parity(3, 10), 6);
		assert_eq!(ChunkIteratorDeclination::randomise_parity(4, 10), 8);
		
		assert_eq!(ChunkIteratorDeclination::randomise_parity(5, 10), 1);
		assert_eq!(ChunkIteratorDeclination::randomise_parity(6, 10), 3);
		assert_eq!(ChunkIteratorDeclination::randomise_parity(7, 10), 5);
		assert_eq!(ChunkIteratorDeclination::randomise_parity(8, 10), 7);
		assert_eq!(ChunkIteratorDeclination::randomise_parity(9, 10), 9);
	}
	
	#[test]
	// Should work with an odd number of elements. 
	fn test_randomise_parity_odd ( )
	{
		assert_eq!(ChunkIteratorDeclination::randomise_parity(0, 9), 0);
		assert_eq!(ChunkIteratorDeclination::randomise_parity(1, 9), 2);
		assert_eq!(ChunkIteratorDeclination::randomise_parity(2, 9), 4);
		assert_eq!(ChunkIteratorDeclination::randomise_parity(3, 9), 6);
		assert_eq!(ChunkIteratorDeclination::randomise_parity(4, 9), 8);
		
		assert_eq!(ChunkIteratorDeclination::randomise_parity(5, 9), 1);
		assert_eq!(ChunkIteratorDeclination::randomise_parity(6, 9), 3);
		assert_eq!(ChunkIteratorDeclination::randomise_parity(7, 9), 5);
		assert_eq!(ChunkIteratorDeclination::randomise_parity(8, 9), 7);
	}

	#[test]
	// Should be in order. 
	fn test_randomise_parity_two_elements ( )
	{
		assert_eq!(ChunkIteratorDeclination::randomise_parity(0, 2), 0);
		assert_eq!(ChunkIteratorDeclination::randomise_parity(1, 2), 1);
	}
	
	#[test]
	// Should be in not fail. 
	fn test_randomise_parity_1_element ( )
	{
		assert_eq!(ChunkIteratorDeclination::randomise_parity(0, 1), 0);
	}

//									widen_integer_step ~										 //
	#[test]
	// If the size is 180, 180 should be returned.
	fn test_widen_integer_max_size ( )
	{
		assert_eq!(ChunkIteratorDeclination::widen_integer_step(Degrees(180.0).as_radians()), Degrees(180.0).as_radians());
	}

	#[test]
	// If the angle is already an integer of 180, it should not be modified.
	fn test_widen_integer_already_integer ( )
	{
		assert_eq!(ChunkIteratorDeclination::widen_integer_step(Degrees(360.0).as_radians()),     Degrees(360.0).as_radians());
		assert_eq!(ChunkIteratorDeclination::widen_integer_step(Degrees(180.0).as_radians()),     Degrees(180.0).as_radians());
		assert_eq!(ChunkIteratorDeclination::widen_integer_step(Degrees(90.0).as_radians()),      Degrees(90.0).as_radians());
		assert_eq!(ChunkIteratorDeclination::widen_integer_step(Degrees(60.0).as_radians()),      Degrees(60.0).as_radians());
		assert_eq!(ChunkIteratorDeclination::widen_integer_step(Degrees(36.0).as_radians()),      Degrees(36.0).as_radians());
		ChunkIteratorDeclination::widen_integer_step(Degrees(25.7142).as_radians()).test_equal(&Degrees(25.7142).as_radians());
	}

	#[test]
	// Should always round up.
	fn test_widen_integer_already_needs_to_round ( )
	{
		assert_eq!(ChunkIteratorDeclination::widen_integer_step(Degrees(360.1).as_radians()), Degrees(360.0).as_radians());
		assert_eq!(ChunkIteratorDeclination::widen_integer_step(Degrees(180.3).as_radians()), Degrees(360.0).as_radians());
		assert_eq!(ChunkIteratorDeclination::widen_integer_step(Degrees(180.1).as_radians()), Degrees(180.0).as_radians());
		assert_eq!(ChunkIteratorDeclination::widen_integer_step(Degrees(179.0).as_radians()), Degrees(180.0).as_radians());
		assert_eq!(ChunkIteratorDeclination::widen_integer_step(Degrees(90.3).as_radians()),  Degrees(180.0).as_radians());
		assert_eq!(ChunkIteratorDeclination::widen_integer_step(Degrees(90.1).as_radians()),  Degrees(90.0).as_radians());
		assert_eq!(ChunkIteratorDeclination::widen_integer_step(Degrees(56.0).as_radians()),  Degrees(60.0).as_radians());
		assert_eq!(ChunkIteratorDeclination::widen_integer_step(Degrees(45.3).as_radians()),  Degrees(60.0).as_radians());
		assert_eq!(ChunkIteratorDeclination::widen_integer_step(Degrees(45.0).as_radians()),  Degrees(45.0).as_radians());
	}





//										~ begin ~												 //
	#[test]
	fn test_declination_begin ( )
	{
		let db = MockDatabase::new(); 
		let mut iterator = ChunkIteratorDeclination::new(&db, Degrees(180.0).as_radians(), 1.0, ChunkIteratorDeclination::randomise_parity);
		assert_eq!(iterator.index, 0);
		assert_eq!(iterator.num,   2);
		assert_eq!(iterator.dec.start, Radians(0.0));
		assert_eq!(iterator.dec.end, Radians(0.0));
		assert_eq!(iterator.chunk_step, Degrees(180.0).as_radians());
		assert_eq!(iterator.chunk_size_multiplier, 2.0);
		iterator.next();
		iterator.begin();
		assert_eq!(iterator.index, 0);
		assert_eq!(iterator.num,   2);
		assert_eq!(iterator.dec.start, Radians(0.0));
		assert_eq!(iterator.dec.end, Radians(0.0));
		assert_eq!(iterator.chunk_step, Degrees(180.0).as_radians());
		assert_eq!(iterator.chunk_size_multiplier, 2.0);
	}
	







//										~ next ~												 //
	#[test]
	// each pole is 1/2 step, by having double the range, there is only one chunk
	fn test_declination_next_single ( )
	{
		let db = MockDatabase::new(); 
		let mut iterator = ChunkIteratorDeclination::new(
			&db, Degrees(360.0).as_radians(), 0.0, ChunkIteratorDeclination::randomise_none
		);
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(-270.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(90.00).as_radians());
		assert!(!iterator.next());
		
		iterator.begin();
		iterator.chunk_size_multiplier = 2.0;
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(-450.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees( 360.0 - 90.0).as_radians());
		assert!(!iterator.next());
	}
	
	#[test]
	// each pole is 1/2 step, by having the range, there should be two poles
	fn test_declination_next_two ( )
	{
		let db = MockDatabase::new(); 
		let mut iterator = ChunkIteratorDeclination::new(
			&db, Degrees(180.0).as_radians(),0.0, ChunkIteratorDeclination::randomise_none
		);
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(-180.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(00.00).as_radians());
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(00.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(180.0).as_radians());
		assert!(!iterator.next());
		
		iterator.begin();
		iterator.chunk_size_multiplier = 2.0;
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(-270.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(  90.0).as_radians());
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees( -90.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees( 270.0).as_radians());
		assert!(!iterator.next());
	}
	
	#[test]
	// each pole is 1/2 step, by having half the range, there should be two poles (90 deg)
	// and an extra band for the extra 90 degrees.
	fn test_declination_next_three ( )
	{
		let db = MockDatabase::new(); 
		let mut iterator = ChunkIteratorDeclination::new(
			&db, Degrees(90.0).as_radians(), 0.0, ChunkIteratorDeclination::randomise_none
		);
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(-135.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(-45.0).as_radians());
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(-45.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees( 45.0).as_radians());
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(45.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(135.0).as_radians());
		assert!(!iterator.next());

		iterator.begin();
		iterator.chunk_size_multiplier = 2.0;
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(-180.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(  -0.0).as_radians());
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(-90.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees( 90.0).as_radians());
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(  0.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(180.0).as_radians());
		assert!(!iterator.next());
	}
	
	#[test]
	// each pole is 1/2 step, by having 1/3 te range, there should be two poles (60 deg)
	// and an extra 2 bands for the extra 2/3, 120 degrees.
	fn test_declination_next_four ( )
	{
		let db = MockDatabase::new(); 
		let mut iterator = ChunkIteratorDeclination::new(
			&db, Degrees(60.0).as_radians(), 0.0, ChunkIteratorDeclination::randomise_none
		);
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(-120.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(-60.0).as_radians());
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(-60.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(00.00).as_radians());
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(00.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(60.0).as_radians());
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(60.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(120.0).as_radians());
		assert!(!iterator.next());
		
		iterator.begin();
		iterator.chunk_size_multiplier = 2.0;
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(-150.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(-30.0).as_radians());
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(-90.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees( 30.00).as_radians());
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(-30.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees( 90.0).as_radians());
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees( 30.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(150.0).as_radians());
		assert!(!iterator.next());
	}
	
	
	
	#[test]
	// each pole is 1/2 step, by having 1/4 te range, there should be two poles (36 deg)
	// and an extra 3 bands for the extra 3/4, 108 degrees.
	fn test_declination_next_five ( )
	{
		let db = MockDatabase::new(); 
		let mut iterator = ChunkIteratorDeclination::new(
			&db, Degrees(45.0).as_radians(), 0.0, ChunkIteratorDeclination::randomise_none
		);
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(-112.5).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(-67.5).as_radians());
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(-67.5).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(-22.5).as_radians());
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(-22.5).as_radians());
		assert_eq!(iterator.dec.end,   Degrees( 22.5).as_radians());
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(22.5).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(67.5).as_radians());
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(67.5).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(112.5).as_radians());
		assert!(!iterator.next());
	}
	
	#[test]
	// each pole is 1/2 step, by having 1/4 te range, there should be two poles (36 deg)
	// and an extra 3 bands for the extra 3/4, 108 degrees.
	fn test_declination_next_six ( )
	{
		let db = MockDatabase::new(); 
		let mut iterator = ChunkIteratorDeclination::new(
			&db, Degrees(36.0).as_radians(), 0.0, ChunkIteratorDeclination::randomise_none
		);
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(-108.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(-72.00).as_radians());
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(-72.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(-36.0).as_radians());
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(-36.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(-00.0).as_radians());
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(00.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(36.0).as_radians());
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(36.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(72.0).as_radians());
		assert!(iterator.next());
		assert_eq!(iterator.dec.start, Degrees(72.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(108.0).as_radians());
		assert!(!iterator.next());
	}
	
	
	#[test]
	// Same as six but using the randomiser to separate the chunks.
	// The randomiser goes through all the even chunks followed by all the odd chunks.
	fn test_declination_next_six_randomised ( )
	{
		let db = MockDatabase::new(); 
		let mut iterator = ChunkIteratorDeclination::new(
			&db, Degrees(36.0).as_radians(), 0.0, ChunkIteratorDeclination::randomise_parity
		);
		assert!(iterator.next()); // 0
		assert_eq!(iterator.dec.start, Degrees(-108.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(-72.00).as_radians());
		assert!(iterator.next()); // 2
		assert_eq!(iterator.dec.start, Degrees(-36.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(-00.0).as_radians());
		assert!(iterator.next()); // 4
		assert_eq!(iterator.dec.start, Degrees(36.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(72.0).as_radians());
		assert!(iterator.next()); // 1
		assert_eq!(iterator.dec.start, Degrees(-72.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(-36.0).as_radians());
		assert!(iterator.next()); // 3
		assert_eq!(iterator.dec.start, Degrees(00.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(36.0).as_radians());
		assert!(iterator.next()); // 5
		assert_eq!(iterator.dec.start, Degrees(72.0).as_radians());
		assert_eq!(iterator.dec.end,   Degrees(108.0).as_radians());
		assert!(!iterator.next());
	}
	





//										~ get_same_region ~										 //
	#[test]
	fn test_declination_same_region_inside_lower_bound ( )
	{
		let mut db = MockDatabase::new();
		db.expect_find_star().times(2).returning(|_| 
			return Ok(Equatorial{ra: Radians(0.0), dec: Degrees(-10.0).as_radians()}));
		db.expect_get_pairs().times(1).returning(|_| return StarPair(0,0));
			
		let mut iter = ChunkIteratorDeclination::new(&db, Radians(1.0), 1.0, ChunkIteratorDeclination::randomise_parity);
			
		iter.dec = Degrees(-10.0).as_radians() .. Degrees(0.0).as_radians();
		assert!(iter.same_region(0));
	}
	#[test]
	fn test_declination_same_region_inside_upper_bound ( )
	{
		let mut db = MockDatabase::new();
		db.expect_find_star().times(2).returning(|_| 
			return Ok(Equatorial{ra: Radians(0.0), dec: Degrees(20.0).as_radians()}));
		db.expect_get_pairs().times(1).returning(|_| return StarPair(0,0));
		let mut iter = ChunkIteratorDeclination::new(&db, Radians(1.0), 1.0, ChunkIteratorDeclination::randomise_parity);
			
		iter.dec = Degrees(-10.0).as_radians() .. Degrees(20.0).as_radians();
		assert!(iter.same_region(0));
	}
	
	
	
	#[test]
	fn test_declination_same_region_outside_lower_bound ( )
	{
		let mut db = MockDatabase::new();
		db.expect_find_star().times(2).returning(|_| 
			return Ok(Equatorial{ra: Radians(0.0), dec: Degrees(-10.01).as_radians()}));
		
		db.expect_get_pairs().times(1).returning(|_| return StarPair(0, 0));
		let mut iter = ChunkIteratorDeclination::new(&db, Radians(1.0), 1.0, ChunkIteratorDeclination::randomise_parity);
			
		iter.dec = Degrees(-10.0).as_radians() .. Degrees(0.0).as_radians();
		assert!(!iter.same_region(0));
	}
	#[test]
	fn test_declination_same_region_outside_upper_bound ( )
	{
		let mut db = MockDatabase::new();
		db.expect_find_star().times(2).returning(|_| 
			return Ok(Equatorial{ra: Radians(0.0), dec: Degrees(20.01).as_radians()}));
			
		db.expect_get_pairs().times(1).returning(|_| return StarPair(0, 0));
		
		let mut iter = ChunkIteratorDeclination::new(&db, Radians(1.0), 1.0, ChunkIteratorDeclination::randomise_parity);
			
		iter.dec = Degrees(-10.0).as_radians() .. Degrees(20.0).as_radians();
		assert!(!iter.same_region(0));
	}


	#[test]
	fn test_declination_same_region_halfin_upper_bound ( )
	{
		let mut db = MockDatabase::new();
		db.expect_find_star().times(2).returning(|index| 
			if index == 0
			{
				return Ok(Equatorial{ra: Radians(0.0), dec: Degrees(20.0).as_radians()});
			}
			else
			{
				return Ok(Equatorial{ra: Radians(0.0), dec: Degrees(20.01).as_radians()});
			});
			
		db.expect_get_pairs().times(1).returning(|_| return StarPair(0, 1));
		
		let mut iter = ChunkIteratorDeclination::new(&db, Radians(1.0), 1.0, ChunkIteratorDeclination::randomise_parity);
			
		iter.dec = Degrees(-10.0).as_radians() .. Degrees(20.0).as_radians();
		assert!(iter.same_region(0));
	}


	#[test]
	fn test_declination_same_region_halfin_lower_bound ( )
	{
		let mut db = MockDatabase::new();
		db.expect_find_star().times(2).returning(|index| 
			if index == 0
			{
				return Ok(Equatorial{ra: Radians(0.0), dec: Degrees(-10.0).as_radians()});
			}
			else
			{
				return Ok(Equatorial{ra: Radians(0.0), dec: Degrees(-10.01).as_radians()});
			});
			
		db.expect_get_pairs().times(1).returning(|_| return StarPair(0, 1));
		
		let mut iter = ChunkIteratorDeclination::new(&db, Radians(1.0), 1.0, ChunkIteratorDeclination::randomise_parity);
			
		iter.dec = Degrees(-10.0).as_radians() .. Degrees(20.0).as_radians();
		assert!(iter.same_region(0));
	}

	#[test]
	fn test_declination_same_region_invalid_first ( )
	{
		let mut db = MockDatabase::new();
		db.expect_find_star().times(2).returning(|index| 
			if index == 0
			{
				return Err(Errors::NoMatch);
			}
			else
			{
				return Ok(Equatorial{ra: Radians(0.0), dec: Degrees(-10.01).as_radians()});
			});
			
		db.expect_get_pairs().times(1).returning(|_| return StarPair(0, 1));
		
		let mut iter = ChunkIteratorDeclination::new(&db, Radians(1.0), 1.0, ChunkIteratorDeclination::randomise_parity);
			
		iter.dec = Degrees(-10.0).as_radians() .. Degrees(20.0).as_radians();
		assert!(!iter.same_region(0));
	}
	#[test]
	fn test_declination_same_region_invalid_second ( )
	{
		let mut db = MockDatabase::new();
		db.expect_find_star().times(2).returning(|index| 
			if index == 0
			{
				return Ok(Equatorial{ra: Radians(0.0), dec: Degrees(-10.01).as_radians()});
			}
			else
			{
				return Err(Errors::NoMatch);
			});
			
		db.expect_get_pairs().times(1).returning(|_| return StarPair(0, 1));
		
		let mut iter = ChunkIteratorDeclination::new(&db, Radians(1.0), 1.0, ChunkIteratorDeclination::randomise_parity);
			
		iter.dec = Degrees(-10.0).as_radians() .. Degrees(20.0).as_radians();
		assert!(!iter.same_region(0));
	}


	
	
//										~ get_database ~										 //
	#[test]
	fn test_declination_get_database ( )
	{
		let mut db = MockDatabase::new();
		db.expect_find_star().times(1).returning(|_| return Ok(Equatorial::zero()));
		let iter = ChunkIteratorDeclination::new(&db, Radians(1.0), 1.0, ChunkIteratorDeclination::randomise_parity);
		let _ = iter.get_database().find_star(0);
	}
	


	



//###############################################################################################//
//
//										ChunkIteratorEquatorial
//
// pub fn new   ( &'a dyn Database, Radians, Decimal ) -> Self;
//
// pub fn begin ( &mut self );
// pub fn next  ( &mut self );
// pub fn get_database ( &self );
// pub fn same_region  ( &self, usize );
//
//###############################################################################################//
// //										~ new ~												 //
// 	#[test]
// 	fn test_none_new ( )
// 	{
// 		let mut db = MockDatabase::new();
// 		db.expect_find_star().times(1).returning(|_| return Ok(Equatorial::zero()));
// 		let iter = ChunkIteratorNone::new(&db);
// 		let _ = iter.database.find_star(0);
// 	}
// 
// //										~ begin ~											 //
// 	#[test]
// 	// no calls to the database should occur, there is no other variables.
// 	fn test_none_begin ( )
// 	{
// 		let db = MockDatabase::new();
// 		let mut iter = ChunkIteratorNone::new(&db);
// 		iter.begin();
// 		assert_eq!(iter.started, false);
// 		iter.started = true;
// 		iter.begin();
// 		assert_eq!(iter.started, false);
// 	}
// 
//										~ next ~											 //
	#[test]
	fn test_equatorial_next_single_band ( )
	{
		let step = Degrees(180.0).as_radians();
		let chunk_size_multiplier = 0.0;
		let db = MockDatabase::new();
		let mut iter = ChunkIteratorEquatorial::new(&db, step, chunk_size_multiplier);
		iter.begin();
		
		assert!(iter.next(), "Should always step once.");
		iter.dec.start.assert_close(&Degrees(-90.1).as_radians(), 0.01);
		iter.dec.end.assert_close  (&Degrees(90.1).as_radians(),  0.01);
		
		iter.ra.start.assert_close (&Degrees(-0.1).as_radians(),  0.01);
		iter.ra.end.assert_close   (&Degrees(360.1).as_radians(), 0.01);
	
		assert!(!iter.next(), "Should only step once.");
	}


	#[test]
	fn test_equatorial_next_two_bands ( )
	{
		let step = Degrees(90.0).as_radians();
		let chunk_size_multiplier = 0.0;
		let db = MockDatabase::new();
		let mut iter = ChunkIteratorEquatorial::new(&db, step, chunk_size_multiplier);
		iter.begin();
		
		assert!(iter.next(), "Should always step once.");
		iter.dec.start.assert_close(&Degrees(-90.1).as_radians(),  0.01);
		iter.dec.end.assert_close(  &Degrees(0.1).as_radians(),    0.01);
		iter.ra.start.assert_close(  &Degrees(-0.1).as_radians(),  0.01);
		iter.ra.end.assert_close  (  &Degrees(360.1).as_radians(), 0.01);
		
		assert!(iter.next(), "Should step twice.");
		iter.dec.start.assert_close(&Degrees(-0.1).as_radians(),   0.01);
		iter.dec.end.assert_close(  &Degrees( 90.1).as_radians(),  0.01);
		iter.ra.start.assert_close(  &Degrees(-0.1).as_radians(),  0.01);
		iter.ra.end.assert_close  (  &Degrees(360.1).as_radians(), 0.01);
		
		assert!(!iter.next(), "Should only step twice.");
	}

	#[test]
	fn test_equatorial_next_three_bands ( )
	{
		let step = Degrees(60.0).as_radians();
		let chunk_size_multiplier = 0.0;
		let db = MockDatabase::new();
		let mut iter = ChunkIteratorEquatorial::new(&db, step, chunk_size_multiplier);
		iter.begin();
		
		assert!(iter.next(), "Should always step once.");
		iter.dec.start.assert_close (&Degrees(-90.0).as_radians(),  0.01);
		iter.dec.end.assert_close   (&Degrees(-30.0).as_radians(), 0.01);
		iter.ra.start.assert_close  (&Degrees(-0.0).as_radians(),  0.01);
		iter.ra.end.assert_close    (&Degrees(360.0).as_radians(), 0.01);
		
		// as at equator, will increment by step to 360 deg
		assert!(iter.next(), "Mid Band 1.");
		iter.dec.start.assert_close(&Degrees(-30.0).as_radians(),  0.01);
		iter.dec.end.assert_close  (&Degrees( 30.0).as_radians(),  0.01);
		iter.ra.start.assert_close (&Degrees(-0.0).as_radians(),   0.01);
		iter.ra.end.assert_close   (&Degrees(60.0).as_radians(),   0.01);
		
		assert!(iter.next(), "Mid Band 2.");
		iter.dec.start.assert_close(&Degrees(-30.0).as_radians(), 0.01);
		iter.dec.end.assert_close  (&Degrees( 30.0).as_radians(), 0.01);
		iter.ra.start.assert_close (&Degrees( 60.0).as_radians(), 0.01);
		iter.ra.end.assert_close   (&Degrees(120.0).as_radians(), 0.01);
		
		assert!(iter.next(), "Mid Band 3.");
		iter.dec.start.assert_close(&Degrees(-30.0).as_radians(), 0.01);
		iter.dec.end.assert_close  (&Degrees( 30.0).as_radians(), 0.01);
		iter.ra.start.assert_close (&Degrees(120.0).as_radians(), 0.01);
		iter.ra.end.assert_close   (&Degrees(180.0).as_radians(), 0.01);
		
		assert!(iter.next(), "Mid Band 4.");
		iter.dec.start.assert_close(&Degrees(-30.0).as_radians(), 0.01);
		iter.dec.end.assert_close  (&Degrees( 30.0).as_radians(), 0.01);
		iter.ra.start.assert_close (&Degrees(180.0).as_radians(), 0.01);
		iter.ra.end.assert_close   (&Degrees(240.0).as_radians(), 0.01);
		
		assert!(iter.next(), "Mid Band 5.");
		iter.dec.start.assert_close(&Degrees(-30.1).as_radians(), 0.01);
		iter.dec.end.assert_close  (&Degrees( 30.1).as_radians(), 0.01);
		iter.ra.start.assert_close (&Degrees(240.0).as_radians(), 0.01);
		iter.ra.end.assert_close   (&Degrees(300.0).as_radians(), 0.01);
		
		assert!(iter.next(), "Mid Band 6.");
		iter.dec.start.assert_close(&Degrees(-30.0).as_radians(), 0.01);
		iter.dec.end.assert_close  (&Degrees( 30.0).as_radians(), 0.01);
		iter.ra.start.assert_close (&Degrees(300.0).as_radians(), 0.01);
		iter.ra.end.assert_close   (&Degrees(360.0).as_radians(), 0.01);
		
		assert!(iter.next(), "Should step three times.");
		iter.dec.start.assert_close(&Degrees( 30.0).as_radians(), 0.01);
		iter.dec.end.assert_close  (&Degrees( 90.0).as_radians(), 0.01);
		iter.ra.start.assert_close (&Degrees( 0.0).as_radians(),  0.01);
		iter.ra.end.assert_close   (&Degrees(360.0).as_radians(), 0.01);
		
		assert!(!iter.next(), "Should only iterate 8 times.");
	}

	// Tests an entire ra band.
	#[coverage(off)]
	#[track_caller]
	fn test_band ( iter : &mut ChunkIteratorEquatorial, angle: Radians, 
			dec_min: Radians, dec_max: Radians, dec_eq: Radians )
	{
		let times = (dec_eq.cos() * 360.0 / angle.to_degrees().0).ceil() as usize;
		let angle = Degrees(360.0 / times as Decimal).to_radians();

		assert!(iter.next(), "Band: {} .. {}.", dec_min.to_degrees(), dec_max.to_degrees());
		iter.dec.start.assert_close(&dec_min, 0.01);
		iter.dec.end.assert_close  (&dec_max, 0.01);
		iter.ra.start.assert_close (&Degrees( -angle.to_degrees().0 * 
			(iter.chunk_size_multiplier - 1.0) / 2.0).as_radians(),  0.01);
		// iter.ra.end.assert_close   (&angle, 0.01);
		let mut last_ra = iter.ra.start  .. iter.ra.end;
		let dec     = iter.dec.start .. iter.dec.end;
		for _ in 0..(times - 1)
		{
			iter.next();
			assert_eq!(dec, iter.dec);
			iter.ra.start.assert_close(&(last_ra.end - angle * 
				(iter.chunk_size_multiplier - 1.0)), 0.01);
			iter.ra.end.assert_close(&(iter.ra.start + angle * (iter.chunk_size_multiplier)), 0.01);
			last_ra = iter.ra.start .. iter.ra.end;
		}
	}

	#[test]
	fn test_equatorial_next_five_bands ( )
	{
		let step = Degrees(20.0).as_radians();
		let chunk_size_multiplier = 0.0;
		let db = MockDatabase::new();
		let mut iter = ChunkIteratorEquatorial::new(&db, step, chunk_size_multiplier);
		iter.begin();
		
		assert!(iter.next(), "BAND 1");
		iter.dec.start.assert_close(&Degrees(-90.0).as_radians(), 0.01);
		iter.dec.end.assert_close  (&Degrees(-70.0).as_radians(), 0.01);
		iter.ra.start.assert_close (&Degrees( 0.0).as_radians(),  0.01);
		iter.ra.end.assert_close   (&Degrees(360.0).as_radians(), 0.01);
		
		// at 36 degrees, the angle steps are ceil((cos(50)*360)/20) = 12, 360 / 12 = 30 degrees.
		test_band(&mut iter, step, Degrees(-70.0).as_radians(), Degrees(-50.0).as_radians(), Degrees(-50.0).as_radians());
		test_band(&mut iter, step, Degrees(-50.0).as_radians(), Degrees(-30.0).as_radians(), Degrees(-30.0).as_radians());
		test_band(&mut iter, step, Degrees(-30.0).as_radians(), Degrees(-10.0).as_radians(), Degrees(-10.0).as_radians());
		test_band(&mut iter, step, Degrees(-10.0).as_radians(), Degrees(10.0).as_radians(), Degrees(0.0).as_radians());
		test_band(&mut iter, step, Degrees(10.0).as_radians(), Degrees(30.0).as_radians(), Degrees(10.0).as_radians());
		test_band(&mut iter, step, Degrees(30.0).as_radians(), Degrees(50.0).as_radians(), Degrees(30.0).as_radians());
		test_band(&mut iter, step, Degrees(50.0).as_radians(), Degrees(70.0).as_radians(), Degrees(50.0).as_radians());
		
		assert!(iter.next(), "BAND FINAL");
		iter.dec.start.assert_close(&Degrees(70.0).as_radians(), 0.01);
		iter.dec.end.assert_close  (&Degrees(90.0).as_radians(), 0.01);
		iter.ra.start.assert_close (&Degrees( 0.0).as_radians(),  0.01);
		iter.ra.end.assert_close   (&Degrees(360.0).as_radians(), 0.01);
		assert!(!iter.next(), "No more bands");
	}

	#[test]
	fn test_equatorial_next_five_bands_size_multiplier ( )
	{
		let step = Degrees(20.0).as_radians();
		let chunk_size_multiplier = 1.0;
		let db = MockDatabase::new();
		let mut iter = ChunkIteratorEquatorial::new(&db, step, chunk_size_multiplier);
		iter.begin();
		
		assert!(iter.next(), "BAND 1");
		iter.dec.start.assert_close(&Degrees(-90.0).as_radians(), 0.01);
		iter.dec.end.assert_close  (&Degrees(-60.0).as_radians(), 0.01); // -70
		iter.ra.start.assert_close (&Degrees( 0.0).as_radians(),  0.01);
		iter.ra.end.assert_close   (&Degrees(360.0).as_radians(), 0.01);
		
		// at 36 degrees, the angle steps are ceil((cos(50)*360)/20) = 12, 360 / 12 = 30 degrees.
		test_band(&mut iter, step, Degrees(-80.0).as_radians(), Degrees(-40.0).as_radians(), Degrees(-40.0).as_radians()); // -70..-50
		test_band(&mut iter, step, Degrees(-60.0).as_radians(), Degrees(-20.0).as_radians(), Degrees(-20.0).as_radians()); // -50..-30
		test_band(&mut iter, step, Degrees(-40.0).as_radians(), Degrees(-00.0).as_radians(), Degrees(-00.0).as_radians()); // -30..-10
		test_band(&mut iter, step, Degrees(-20.0).as_radians(), Degrees( 20.0).as_radians(), Degrees( 00.0).as_radians()); // -10.. 10
		test_band(&mut iter, step, Degrees( 00.0).as_radians(), Degrees( 40.0).as_radians(), Degrees( 00.0).as_radians()); //  10.. 30
		test_band(&mut iter, step, Degrees( 20.0).as_radians(), Degrees( 60.0).as_radians(), Degrees( 20.0).as_radians()); //  30.. 50
		test_band(&mut iter, step, Degrees( 40.0).as_radians(), Degrees( 80.0).as_radians(), Degrees( 40.0).as_radians()); //  50.. 70
		
		assert!(iter.next(), "BAND FINAL");
		iter.dec.start.assert_close(&Degrees(60.0).as_radians(), 0.01);
		iter.dec.end.assert_close  (&Degrees(90.0).as_radians(), 0.01);
		iter.ra.start.assert_close (&Degrees( 0.0).as_radians(),  0.01);
		iter.ra.end.assert_close   (&Degrees(360.0).as_radians(), 0.01);
		assert!(!iter.next(), "No more bands");
	}


//										~ get_database ~										 //
	#[test]
	fn test_equatorial_get_database ( )
	{
		let mut db = MockDatabase::new();
		db.expect_find_star().times(1).returning(|_| return Ok(Equatorial::zero()));
		let iter = ChunkIteratorEquatorial::new(&db, Radians(0.0), 1.0);
		let _ = iter.get_database().find_star(0);
	}



//										~ same_region ~											 //
	#[test]
	// on the boundry, the star should not be valid.
	fn test_equatorial_same_region_dec_invalid ( )
	{
		let step = Degrees(20.0).as_radians();
		let chunk_size_multiplier = 1.0;
		let mut db = MockDatabase::new();
		db.expect_get_pairs().times(1).returning(|_| return StarPair(0, 1));
		db.expect_find_star().times(2).returning(|_| 
			return Ok(Equatorial{ra: Radians(0.0), dec: Degrees(-59.9).as_radians()}));
			
		let mut iter = ChunkIteratorEquatorial::new(&db, step, chunk_size_multiplier);
		iter.begin();
		iter.next();
		
		assert!(!iter.same_region(1), "Outside range");
	}
	
	#[test]
	// inside the boundry, the star should not be valid.
	fn test_equatorial_same_region_dec_valid ( )
	{
		let step = Degrees(20.0).as_radians();
		let chunk_size_multiplier = 1.0;
		let mut db = MockDatabase::new();
		db.expect_get_pairs().times(1).returning(|_| return StarPair(0, 1));
		db.expect_find_star().times(2).returning(|_|
			return Ok(Equatorial{ra: Radians(0.0), dec: Degrees(-60.0).as_radians()}));
			
		let mut iter = ChunkIteratorEquatorial::new(&db, step, chunk_size_multiplier);
		iter.begin();
		iter.next();
		assert!(iter.same_region(1), "Inside range");
	}

	#[test]
	// Only one star in the pair needs to be in the region.
	fn test_equatorial_same_region_dec_valid_single ( )
	{
		let step = Degrees(20.0).as_radians();
		let chunk_size_multiplier = 1.0;
		let mut db = MockDatabase::new();
		db.expect_get_pairs().times(1).returning(|_| return StarPair(0, 1));
		db.expect_find_star().times(2).returning(|index|
			return Ok(Equatorial{ra: Radians(0.0), dec: Degrees(-61.0 - index as Decimal).as_radians()}));
			
		let mut iter = ChunkIteratorEquatorial::new(&db, step, chunk_size_multiplier);
		iter.begin();
		iter.next();
		assert!(iter.same_region(1), "Inside range");
	}
	
	#[test]
	// If the overshoot exceeds 360 degrees it should wrap inclusively to 0.
	fn test_equatorial_same_region_past_360_valid ( )
	{
		let step = Degrees(60.0).as_radians();
		let chunk_size_multiplier = 1.0;
		let mut db = MockDatabase::new();
		db.expect_get_pairs().times(1).returning(|_| return StarPair(0, 1));
		db.expect_find_star().times(2).returning(|_|
			return Ok(Equatorial{ra: Degrees(15.0).as_radians(), dec: Radians(0.0)}));
			
		let mut iter = ChunkIteratorEquatorial::new(&db, step, chunk_size_multiplier);
		iter.begin();
		iter.next(); // pole
		iter.next(); // dec: -60-60, 6 ra bands...1
		iter.next(); // dec: -60-60, 6 ra bands...2
		iter.next(); // dec: -60-60, 6 ra bands...3
		iter.next(); // dec: -60-60, 6 ra bands...4
		iter.next(); // dec: -60-60, 6 ra bands...5
		iter.next(); // dec: -60-60, 6 ra bands...6 <- Wrap
		assert!(iter.same_region(1), "Inside range");
	}
	
	#[test]
	// A single star in the right ascension region must be valid, the other can be anywhere.
	fn test_equatorial_same_region_single_ra_valid ( )
	{
		let step = Degrees(60.0).as_radians();
		let chunk_size_multiplier = 1.0;
		let mut db = MockDatabase::new();
		db.expect_get_pairs().times(1).returning(|_| return StarPair(0, 1));
		db.expect_find_star().times(2).returning(|index|
			return Ok(Equatorial{ra: Degrees(91.0 - index as Decimal).as_radians(), dec: Radians(0.0)}));
			
		let mut iter = ChunkIteratorEquatorial::new(&db, step, chunk_size_multiplier);
		iter.begin();
		iter.next(); // pole
		iter.next(); // dec: -60-60, 6 ra bands...1
		assert!(iter.same_region(1), "Inside range");
	}
	
	#[test]
	// If the overshoot exceeds 360 degrees it should wrap inclusively to 0 this exceeds the range.
	fn test_equatorial_same_region_past_360_invalid ( )
	{
		let step = Degrees(60.0).as_radians();
		let chunk_size_multiplier = 1.0;
		let mut db = MockDatabase::new();
		db.expect_get_pairs().times(1).returning(|_| return StarPair(0, 1));
		db.expect_find_star().times(2).returning(|_|
			return Ok(Equatorial{ra: Degrees(30.1).as_radians(), dec: Radians(0.0)}));
			
		let mut iter = ChunkIteratorEquatorial::new(&db, step, chunk_size_multiplier);
		iter.begin();
		iter.next(); // pole
		iter.next(); // dec: ~0, 6 ra bands...1
		iter.next(); // dec: ~0, 6 ra bands...2
		iter.next(); // dec: ~0, 6 ra bands...3
		iter.next(); // dec: ~0, 6 ra bands...4
		iter.next(); // dec: ~0, 6 ra bands...5
		iter.next(); // dec: ~0, 6 ra bands...6 <- Wrap
		assert!(!iter.same_region(1), "Outside range");
	}
	
	
	#[test]
	// If the overshoot is under 0 it should wrap inclusively to 360 this exceeds the range.
	fn test_equatorial_same_region_under_0_invalid ( )
	{
		let step = Degrees(60.0).as_radians();
		let chunk_size_multiplier = 1.0;
		let mut db = MockDatabase::new();
		db.expect_get_pairs().times(1).returning(|_| return StarPair(0, 1));
		db.expect_find_star().times(2).returning(|_|
			return Ok(Equatorial{ra: Degrees(329.9).as_radians(), dec: Radians(0.0)}));
			
		let mut iter = ChunkIteratorEquatorial::new(&db, step, chunk_size_multiplier);
		iter.begin();
		iter.next(); // pole
		iter.next(); // dec: ~0, 6 ra bands...1 <- wrap
		assert!(!iter.same_region(1), "Outside range");
	}
	
	#[test]
	// If the overshoot is under 0 it should wrap inclusively to 360 this exceeds the range.
	fn test_equatorial_same_region_under_0_valid ( )
	{
		let step = Degrees(60.0).as_radians();
		let chunk_size_multiplier = 1.0;
		let mut db = MockDatabase::new();
		db.expect_get_pairs().times(1).returning(|_| return StarPair(0, 1));
		db.expect_find_star().times(2).returning(|_|
			return Ok(Equatorial{ra: Degrees(330.0).as_radians(), dec: Radians(0.0)}));
			
		let mut iter = ChunkIteratorEquatorial::new(&db, step, chunk_size_multiplier);
		iter.begin();
		iter.next(); // pole
		iter.next(); // dec: ~0, 6 ra bands...1 <- wrap
		assert!(iter.same_region(1), "Inside range");
	}


	#[test]
	// If an invalid star pair is entered (which wont happen), false will be returned.
	// This is just to satisfy lcov
	fn test_equatorial_same_region_error ( )
	{
		let step = Degrees(60.0).as_radians();
		let chunk_size_multiplier = 1.0;
		let mut db = MockDatabase::new();
		db.expect_get_pairs().times(1).returning(|_| return StarPair(0, 1));
		db.expect_find_star().times(2).returning(|_| return Err(Errors::NoMatch));
			
		let mut iter = ChunkIteratorEquatorial::new(&db, step, chunk_size_multiplier);
		iter.begin();
		iter.next(); // pole
		iter.next(); // dec: ~0, 6 ra bands...1
		assert!(!iter.same_region(1), "Outside range");
	}



	
//###############################################################################################//
//
//										ChunkAreaSearch
//
// pub fn new   ( &'a dyn RegionalDatabase ) -> Self;
//
// pub fn begin ( &mut self );
// pub fn next  ( &mut self );
// pub fn get_database ( &self );
// pub fn same_region  ( &self, usize );
//
//###############################################################################################//
//										~ new ~													 //
	#[test]
	fn test_area_search_from_range ( )
	{
		let mut db = MockDatabase::new();
		db.expect_find_star().returning(|_| return Ok(Equatorial::zero()));
		
		let range_ra  = Radians(0.0) .. Radians(1.0);
		let range_dec = Radians(1.0) .. Radians(2.0);
		let iter = ChunkAreaSearch::from_range(&db, range_ra.clone(), range_dec.clone());
		
		iter.dec.start.test_close (&range_dec.start, 0.01);
		iter.dec.end.test_close   (&range_dec.end  , 0.01);
		iter.ra.start.test_close  (&range_ra.start , 0.01);
		iter.ra.end.test_close    (&range_ra.end   , 0.01);
		let _ = iter.database.find_star(0);
		assert_eq!(iter.started, false);
	}
	
	#[test]
	fn test_area_search_from_point ( )
	{
		let mut db = MockDatabase::new();
		db.expect_find_star().returning(|_| return Ok(Equatorial::zero()));
		
		let center = Equatorial{ra: Degrees(30.0).as_radians(), dec: Degrees(10.0).as_radians()};
		let iter = ChunkAreaSearch::from_point(&db, center, Degrees(20.0).as_radians());
		
		iter.dec.start.test_close (&Degrees(-10.0).as_radians(), 0.01);
		iter.dec.end.test_close   (&Degrees( 30.0).as_radians(), 0.01);
		iter.ra.start.test_close  (&Degrees( 10.0).as_radians(), 0.01);
		iter.ra.end.test_close    (&Degrees( 40.0).as_radians(), 0.01);
		let _ = iter.database.find_star(0);
		assert_eq!(iter.started, false);
	}
	
	
//										~ begin ~												 //
	#[test]
	fn test_area_search_begin ( )
	{
		let mut db = MockDatabase::new();
		db.expect_find_star().returning(|_| return Ok(Equatorial::zero()));
		
		let range_ra  = Radians(0.0) .. Radians(1.0);
		let range_dec = Radians(1.0) .. Radians(2.0);
		let mut iter = ChunkAreaSearch::from_range(&db, range_ra.clone(), range_dec.clone());
		iter.started = true; // should be false.
		
		iter.begin();
		iter.dec.start.test_close (&range_dec.start, 0.01);
		iter.dec.end.test_close   (&range_dec.end,   0.01);
		iter.ra.start.test_close  (&range_ra.start,  0.01);
		iter.ra.end.test_close    (&range_ra.end,    0.01);
		let _ = iter.database.find_star(0);
		assert_eq!(iter.started, false);
	}
	
//										~ next ~												 //
	#[test]
	fn test_area_search_next ( )
	{
		let db = MockDatabase::new();
		
		let range_ra  = Radians(0.0) .. Radians(1.0);
		let range_dec = Radians(1.0) .. Radians(2.0);
		
		let mut iter = ChunkAreaSearch::from_range(&db, range_ra.clone(), range_dec.clone());
		iter.begin();
		assert!(!iter.started);
		assert!(iter.next());
		assert!(iter.started);
		assert!(!iter.next());
		iter.dec.start.test_close (&range_dec.start, 0.01);
		iter.dec.end.test_close   (&range_dec.end,   0.01);
		iter.ra.start.test_close  (&range_ra.start,  0.01);
		iter.ra.end.test_close    (&range_ra.end,    0.01);
		assert_eq!(iter.started, true);
	}
	
	
//										~ get_database ~										 //
	#[test]
	fn test_area_search_get_database ( )
	{
		let mut db = MockDatabase::new();
		db.expect_find_star().returning(|_| return Ok(Equatorial::zero()));
		
		let range  = Radians(0.0) .. Radians(1.0);
		let iter = ChunkAreaSearch::from_range(&db, range.clone(), range.clone());
		
		let _ = iter.get_database().find_star(0);
	}
	
	
	
//										~ same_region ~											 //
	#[test]
	// If both stars are inside the region.
	fn test_area_search_same_region_both_inside ( )
	{
		let mut db = MockDatabase::new();
		db.expect_find_star().returning(|a| 
		{
			match a // Every 2 is the same.
			{
				0 => return Ok(Equatorial{ra: Degrees(15.0).to_radians(), dec: Degrees(0.0).to_radians()}), // Center
				1 => return Ok(Equatorial{ra: Degrees(0.0).to_radians(),  dec: Degrees(15.0).to_radians()}), // Top Left
				2 => return Ok(Equatorial{ra: Degrees(30.0).to_radians(), dec: Degrees(15.0).to_radians()}), // Top Right
				3 => return Ok(Equatorial{ra: Degrees(0.0).to_radians(),  dec: Degrees(-15.0).to_radians()}), // Bottom Left
				_ => return Ok(Equatorial{ra: Degrees(30.0).to_radians(), dec: Degrees(-15.0).to_radians()}), // Bottom Right
			}
		});
		
		let range_ra   = Degrees(0.0).to_radians()   .. Degrees(30.0).to_radians();
		let range_dec  = Degrees(-15.0).to_radians() .. Degrees(15.0).to_radians();
		
		db.expect_get_pairs().returning(|a| return StarPair(a, a));

		let iter = ChunkAreaSearch::from_range(&db, range_ra, range_dec);
		assert!(iter.same_region(0));
		assert!(iter.same_region(1));
		assert!(iter.same_region(2));
		assert!(iter.same_region(3));
		assert!(iter.same_region(4));
	}
	




	#[test]
	// If one star is within the region, it should return true.
	fn test_area_search_same_region_one_inside ( )
	{
		let mut db = MockDatabase::new();
		let mut i = 0;
		db.expect_find_star().returning(move |_| 
		{
			i+=1;
			match i
			{
				1 => return Ok(Equatorial{ra: Degrees(0.0).to_radians(),   dec: Degrees(16.0).to_radians()}), // Dec Above
				2 => return Ok(Equatorial{ra: Degrees(15.0).to_radians(),  dec: Degrees(0.0).to_radians()}),

				3 => return Ok(Equatorial{ra: Degrees(0.0).to_radians(),   dec: Degrees(-16.0).to_radians()}), // Dec Below
				4 => return Ok(Equatorial{ra: Degrees(15.0).to_radians(),  dec: Degrees(0.0).to_radians()}),
				
				5 => return Ok(Equatorial{ra: Degrees(359.0).to_radians(), dec: Degrees(0.0).to_radians()}), // Ra Left
				6 => return Ok(Equatorial{ra: Degrees(15.0).to_radians(),  dec: Degrees(0.0).to_radians()}),
				
				7 => return Ok(Equatorial{ra: Degrees(31.0).to_radians(),  dec: Degrees(0.0).to_radians()}), // Ra Right
				_ => return Ok(Equatorial{ra: Degrees(15.0).to_radians(),  dec: Degrees(0.0).to_radians()}),
			}
		});
		
		let range_ra   = Degrees(0.0).to_radians()   .. Degrees(30.0).to_radians();
		let range_dec  = Degrees(-15.0).to_radians() .. Degrees(15.0).to_radians();
		
		db.expect_get_pairs().returning(|a| return StarPair(a, a));

		let iter = ChunkAreaSearch::from_range(&db, range_ra, range_dec);
		assert!(iter.same_region(0));
		assert!(iter.same_region(1));
		assert!(iter.same_region(2));
		assert!(iter.same_region(3));
	}
	





	#[test]
	// If both stars are outside of the region, false will be returned.
	fn test_area_search_same_region_both_outside ( )
	{
		let mut db = MockDatabase::new();
		db.expect_find_star().returning(|a| 
			{
				match a // Every 2 is the same.
				{
					0 => return Ok(Equatorial{ra: Degrees(0.0).to_radians(),   dec: Degrees(16.0).to_radians()}),  // Top
					1 => return Ok(Equatorial{ra: Degrees(359.0).to_radians(), dec: Degrees(16.0).to_radians()}), // Top Left
					2 => return Ok(Equatorial{ra: Degrees(31.0).to_radians(),  dec: Degrees(16.0).to_radians()}), // Top Right

					3 => return Ok(Equatorial{ra: Degrees(359.0).to_radians(), dec: Degrees(0.0).to_radians()}),  // Left
					4 => return Ok(Equatorial{ra: Degrees(31.0).to_radians(),  dec: Degrees(0.0).to_radians()}),  // Right

					5 => return Ok(Equatorial{ra: Degrees(0.0).to_radians(),   dec: Degrees(-16.0).to_radians()}),  // Bottom
					6 => return Ok(Equatorial{ra: Degrees(359.0).to_radians(), dec: Degrees(-16.0).to_radians()}), // Bottom Left
					_ => return Ok(Equatorial{ra: Degrees(31.0).to_radians(),  dec: Degrees(-16.0).to_radians()}), // Bottom Right
				}
			});
		
		let range_ra   = Degrees(0.0).to_radians()   .. Degrees(30.0).to_radians();
		let range_dec  = Degrees(-15.0).to_radians() .. Degrees(15.0).to_radians();
		
		db.expect_get_pairs().returning(|a| return StarPair(a, a));

		let iter = ChunkAreaSearch::from_range(&db, range_ra, range_dec);
		assert_eq!(iter.same_region(0), false);
		assert_eq!(iter.same_region(1), false);
		assert_eq!(iter.same_region(2), false);
		assert_eq!(iter.same_region(3), false);
		assert_eq!(iter.same_region(4), false);
		assert_eq!(iter.same_region(5), false);
		assert_eq!(iter.same_region(6), false);
		assert_eq!(iter.same_region(7), false);
	}
	
	



	#[test]	
	#[cfg_attr(coverage, coverage(off))]
	// If the region wraps around, the stars on the inner side will be inside.
	fn test_area_search_same_region_inside_wrap ( )
	{
		let mut db = MockDatabase::new();
		db.expect_find_star().returning(|a| 
			{
				match a
				{
					// negative wrap
					0 => return Ok(Equatorial{ra: Degrees(-30.0).to_radians(), dec: Degrees(15.0).to_radians()}),  // Top Left
					1 => return Ok(Equatorial{ra: Degrees(-30.0).to_radians(), dec: Degrees(0.0).to_radians()}),   // Left
					2 => return Ok(Equatorial{ra: Degrees(-30.0).to_radians(), dec: Degrees(-15.0).to_radians()}), // Bottom Left
					
					// over wrap
					3 => return Ok(Equatorial{ra: Degrees(370.0).to_radians(), dec: Degrees(15.0).to_radians()}),  // Top Right
					4 => return Ok(Equatorial{ra: Degrees(370.0).to_radians(), dec: Degrees(0.0).to_radians()}),   // Right
					_ => return Ok(Equatorial{ra: Degrees(370.0).to_radians(), dec: Degrees(-15.0).to_radians()}), // Bottom Right
				}
			});	
			
		db.expect_get_pairs().returning(|a| return StarPair(a, a));
		
		let range_ra   = Degrees(-30.0).to_radians() .. Degrees(30.0).to_radians();
		let range_dec  = Degrees(-15.0).to_radians() .. Degrees(15.0).to_radians();
		let iter = ChunkAreaSearch::from_range(&db, range_ra, range_dec);
		assert!(iter.same_region(0));
		assert!(iter.same_region(1));
		assert!(iter.same_region(2));
		
		let range_ra   = Degrees(350.0).to_radians() .. Degrees(370.0).to_radians();
		let range_dec  = Degrees(-15.0).to_radians() .. Degrees(15.0).to_radians();
		let iter = ChunkAreaSearch::from_range(&db, range_ra, range_dec);
		assert!(iter.same_region(3));
		assert!(iter.same_region(4));
		assert!(iter.same_region(5));
	}



	#[test]
	// If the region wraps around, the stars on the outside side will be outside.
	fn test_area_search_same_region_outside_wrap ( )
	{
		let mut db = MockDatabase::new();
		db.expect_find_star().returning(|a| 
			{
				match a
				{
					// negative wrap
					0 => return Ok(Equatorial{ra: Degrees(-31.0).to_radians(), dec: Degrees(15.0).to_radians()}),  // Top Left
					1 => return Ok(Equatorial{ra: Degrees(-31.0).to_radians(), dec: Degrees(0.0).to_radians()}),   // Left
					2 => return Ok(Equatorial{ra: Degrees(-31.0).to_radians(), dec: Degrees(-15.0).to_radians()}), // Bottom Left
					
					// over wrap
					3 => return Ok(Equatorial{ra: Degrees(371.0).to_radians(), dec: Degrees(15.0).to_radians()}),  // Top Right
					4 => return Ok(Equatorial{ra: Degrees(371.0).to_radians(), dec: Degrees(0.0).to_radians()}),   // Right
					_ => return Ok(Equatorial{ra: Degrees(371.0).to_radians(), dec: Degrees(-15.0).to_radians()}), // Bottom Right
				}
			});	
			
		db.expect_get_pairs().returning(|a| return StarPair(a, a));
		
		let range_ra   = Degrees(-30.0).to_radians() .. Degrees(30.0).to_radians();
		let range_dec  = Degrees(-15.0).to_radians() .. Degrees(15.0).to_radians();
		let iter = ChunkAreaSearch::from_range(&db, range_ra, range_dec);
		assert_eq!(iter.same_region(0), false);
		assert_eq!(iter.same_region(1), false);
		assert_eq!(iter.same_region(2), false);
		
		let range_ra   = Degrees(350.0).to_radians() .. Degrees(370.0).to_radians();
		let range_dec  = Degrees(-15.0).to_radians() .. Degrees(15.0).to_radians();
		let iter = ChunkAreaSearch::from_range(&db, range_ra, range_dec);
		assert_eq!(iter.same_region(3), false);
		assert_eq!(iter.same_region(4), false);
		assert_eq!(iter.same_region(5), false);
	}
	
	



	
	#[test]
	// If an invalid star pair is entered (which wont happen), false will be returned.
	// This is just to satisfy lcov
	fn test_area_search_same_region_error ( )
	{
		let mut db = MockDatabase::new();
		let mut i = 0;
		db.expect_get_pairs().times(2).returning(|_| return StarPair(0, 1));
		db.expect_find_star().returning(move |_| 
		{
			i += 1;
			match i // Every 2 is the same.
			{
				1 => return Ok(Equatorial::north()),
				2 => return Err(crate::util::err::Errors::NoMatch),
				3 => return Err(crate::util::err::Errors::NoMatch),
				_ => return Ok(Equatorial::north()),
			}
		});
		
		let range_ra   = Degrees(330.0).to_radians() .. Degrees(30.0).to_radians();
		let range_dec  = Degrees(-15.0).to_radians() .. Degrees(15.0).to_radians();
		let iter = ChunkAreaSearch::from_range(&db, range_ra, range_dec);

		assert!(!iter.same_region(0), "Outside range");
		assert!(!iter.same_region(1), "Outside range");
	}
}
