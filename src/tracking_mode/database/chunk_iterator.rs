//! Implementation of DatabaseIterator

use crate::tracking_mode::database::Database;
use crate::tracking_mode::database::SearchResult;
use crate::tracking_mode::database::ChunkIteratorNone;
use crate::tracking_mode::database::ChunkIteratorEquatorial;

use crate::util::list::List;
use crate::util::aliases::Decimal;
use crate::util::aliases::M_PI;
use crate::util::units::Radians;
use crate::util::units::Degrees;


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
			let error  = 1.0;//(if i < mid { i } else { range.end - i } - range.start) as Decimal / range.len() as Decimal;
			if !found.is_full() && self.same_region(i)
			{
				let result = SearchResult{result: pair, error: error };
				found.push_back(result).expect("database_iterator::find_close_ref_region: Already checked if found full?");
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
	{ return Self{database: database};  }
}

impl <'a> ChunkIterator for ChunkIteratorNone <'a>
{
	/// Resets the iterator.  
	/// Call this to return to the start, then call next.
	fn begin ( &mut self ) { }
	
	/// ChunkIterator does not have regions, next returns false.
	fn next ( &mut self ) -> bool { return false; }
	
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
	/// * `size` - 
	///            A number above 1, this multiplies by the step distance to be the range of each chunk.
	///            The higher the number, the more coverage and lower performance.

	pub fn new ( database: &'a dyn Database, step: Radians, chunk_size_multiplier: Decimal ) -> Self 
	{
		assert!(1.0 <= chunk_size_multiplier, "chunk_size_multiplier must be larger than 1.");
		println!("{} {}", step.to_degrees(), chunk_size_multiplier);
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
	
	
	fn next_ra ( &mut self )
	{
		let step = Degrees(360.0).as_radians() / self.num_ra as Decimal;
		let size = step * self.chunk_size_multiplier;
		
		// Move to next right ascension node.
		let center = step * (self.index_ra as Decimal) + step / 2.0;
		self.ra = center - size / 2.0 .. center + size / 2.0; 
		
		
		self.index_ra += 1;
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
		// println!("{} | {:.2} .. {:.2}    {:.2} .. {:.2}", self.index_ra, self.dec.start.to_degrees().0, self.dec.end.to_degrees(), self.ra.start.to_degrees(), self.ra.end.to_degrees());
		let dec_size = self.chunk_step * self.chunk_size_multiplier;
		
		// If there has not been enough chunks to wrap around the declination band yet.
		if self.index_ra < self.num_ra
		{
			self.next_ra();
			return true;
		}
		else if self.index_dec == 0
		{
			self.dec    = -Degrees(90.0).as_radians()..self.chunk_step/2.0-Degrees(90.0).as_radians();
			self.num_ra = 0;
			self.ra = Radians(0.0) .. Degrees(360.0).as_radians();
			self.index_ra   = 0;
			self.index_dec += 1;
			return true;
			
		}
		else if self.index_dec == self.num_dec
		{
			self.dec    = Degrees(90.0).as_radians() - self.chunk_step / 2.0..Degrees(90.0).as_radians();
			self.num_ra = 0;
			self.ra = Radians(0.0) .. Degrees(360.0).as_radians();
			self.index_ra   = 0;
			self.index_dec += 1;
			return true;
		}
		// If a declination band is completed.
		else if self.index_dec < self.num_dec
		{
			let center = self.chunk_step * (self.index_dec as Decimal)-Degrees(90.0).as_radians();
			self.dec = center - dec_size / 2.0 .. center + dec_size / 2.0;

			// The field of view of the declination band must be calculated.
			// This can be done by
			// ```
			// eq_dec_band = closest_to_zero(dec.start, dec.end) // The widest circle (most coverage).
			// circumference_celestial_sphere = 1 // sudo circumference as using ratio.
			// circumference_declination_band = cos(eq_dec_band) // Technicaly radius (1 to 0).
			//
			// ratio        = circumference_declination_band / circumference_celestial_sphere
			// num_ra_bands = ratio * 2pi / ra_chunk_size.
			// ```
			// This can be simplified to:
			// ```
			// num_ra_bands = cos(eq_dec_band) * 2pi / ra_chunk_size
			// ```
			
			// eq_dec_band is the closest point of the camera shot to the equator.
			let mut eq_dec_band = Radians(center.abs()) - dec_size / 2.0;
			if eq_dec_band.0 < 0.0 { eq_dec_band.0 = 0.0; }
			
			// Find the field of view of the declination band (angle to revolve around it from [0,0,0]).
			let angle =  Radians(eq_dec_band.cos() * 2.0 * M_PI);
			
			// Number of photos = field of view of declination band / field of view of the sensor
			self.num_ra = (angle / self.chunk_step).0.ceil() as usize;
			
			
			// Move to next dec band and back to first ra band.
			self.index_ra = 0;
			self.index_dec += 1;
		
			// Move to the first right ascension band.
			return self.next();
		}
		
		return false;
	}
	
	/// Returns the database this iterator holds.	
	fn get_database ( &self ) -> &dyn Database { return self.database; }
	
	/// Returns true if the pair at the provided index fits the current region.
	fn same_region ( &self, pair_index: usize ) -> bool
	{
		let p1 = self.database.find_star(self.database.get_pairs(pair_index).0);
		let p2 = self.database.find_star(self.database.get_pairs(pair_index).1);
		
		if p1.is_err() || p2.is_err()
		{
			return false;
		}
		let valid_dec = 
			(self.dec.start < p1.unwrap().dec && p1.unwrap().dec < self.dec.end) ||
			(self.dec.start < p2.unwrap().dec && p2.unwrap().dec < self.dec.end);

		let valid_ra = 
			(self.ra.start < p1.unwrap().ra && p1.unwrap().ra < self.ra.end) ||
			(self.ra.start < p2.unwrap().ra && p2.unwrap().ra < self.ra.end);
			

		return valid_dec && valid_ra;
	}
}