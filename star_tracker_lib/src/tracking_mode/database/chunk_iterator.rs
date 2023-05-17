//! Implementation of DatabaseIterator

#[cfg(test)] use mockall::predicate::*;
#[cfg(test)] use mockall::*;

use crate::core_include::*;

use crate::tracking_mode::database::Database;
use crate::tracking_mode::database::SearchResult;
use crate::tracking_mode::database::ChunkIteratorNone;
use crate::tracking_mode::database::ChunkIteratorEquatorial;

use crate::util::list::List;
use crate::util::aliases::Decimal;
use crate::util::aliases::M_PI;
use crate::util::units::Radians;
use crate::util::units::Degrees;

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





impl <'a> ChunkIteratorEquatorial <'a>
{
	/// Creates a new iterator.  
	/// Use `next` to start iterating.  
	/// To ensure full coverage, make `step` smaller than `size`, the larger size is, the more overlap between chunks and the more coverage.  
	/// If `size` is too big compared to `step`, the chunks will become too large and performance will drop.
	/// # Arguments
	/// * `step` -  
	///            How far the iterator jumps between iterations, this is the separation between the centers of chunks,  
	///            Make this proportional to the database diagonal field of view.
	///
	/// * `chunk_size_multiplier` - 
	///            A number above 1, this multiplies by the step distance to be the range of each chunk.
	///            The higher the number, the more coverage and lower performance.

	pub fn new ( database: &'a dyn Database, step: Radians, chunk_size_multiplier: Decimal ) -> Self 
	{
		assert!(1.0 <= chunk_size_multiplier, "chunk_size_multiplier must be larger than 1.");
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
			chunk_size_multiplier: chunk_size_multiplier,
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
			// This is coinsidently cos(declination) * 360 degrees.
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
	use crate::tracking_mode::database::ChunkIteratorNone;
	use crate::tracking_mode::database::ChunkIteratorEquatorial;
	
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
//										~ new ~											 //
	#[test]
	fn test_none_new ( )
	{
		let mut db = MockDatabase::new();
		db.expect_find_star().times(1).returning(|_| return Ok(Equatorial::zero()));
		let iter = ChunkIteratorNone::new(&db);
		let _ = iter.database.find_star(0);
	}
	
//										~ begin ~											 //
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

//										~ next ~											 //
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
//										ChunkIteratorEquatorial
//
// pub fn new   ( &'a dyn Database ) -> Self;
//
// pub fn begin ( &mut self );
// pub fn next  ( &mut self );
// pub fn get_database ( &self );
// pub fn same_region  ( &self, usize );
//
//###############################################################################################//
// //										~ new ~											 //
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
		let chunk_size_multiplier = 1.0;
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
		let chunk_size_multiplier = 1.0;
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
		let chunk_size_multiplier = 1.0;
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
	#[no_coverage]
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
		let chunk_size_multiplier = 1.0;
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
		let chunk_size_multiplier = 2.0;
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
		let chunk_size_multiplier = 2.0;
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
		let chunk_size_multiplier = 2.0;
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
		let chunk_size_multiplier = 2.0;
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
		let chunk_size_multiplier = 2.0;
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
		let chunk_size_multiplier = 2.0;
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
		let chunk_size_multiplier = 2.0;
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
		let chunk_size_multiplier = 2.0;
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
		let chunk_size_multiplier = 2.0;
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
		let chunk_size_multiplier = 2.0;
		let mut db = MockDatabase::new();
		db.expect_get_pairs().times(1).returning(|_| return StarPair(0, 1));
		db.expect_find_star().times(2).returning(|_| return Err(Errors::NoMatch));
			
		let mut iter = ChunkIteratorEquatorial::new(&db, step, chunk_size_multiplier);
		iter.begin();
		iter.next(); // pole
		iter.next(); // dec: ~0, 6 ra bands...1
		assert!(!iter.same_region(1), "Outside range");
	}
	
}