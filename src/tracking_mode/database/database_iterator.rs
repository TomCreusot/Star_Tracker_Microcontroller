//! Implementation of DatabaseIterator

use std::cmp;

use crate::tracking_mode::database::Database;
use crate::tracking_mode::database::SearchResult;
use crate::tracking_mode::database::PyramidIterator;
use crate::tracking_mode::database::BoundedDeclinationIterator;
use crate::tracking_mode::database::RegionalCrunchIterator;
use crate::tracking_mode::database::BoundedEquatorialIterator;
use crate::tracking_mode::database::RegionalIterator;
use crate::tracking_mode::database::RegionalDatabase;
use crate::tracking_mode::database::PyramidDatabase;

use crate::util::linear_lookup::LinearLookup;
use crate::util::list::List;
use crate::util::aliases::Decimal;
use crate::util::aliases::M_PI;
// use crate::util::units::Equatorial;
use crate::util::units::BitField;
use crate::util::units::BitCompare;
use crate::util::units::Radians;
use crate::util::units::Degrees;
use crate::util::units::Equatorial;


use crate::nix::Distribute;


pub trait DatabaseIterator
{
	/// Resets the iterator to start - 1.
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
				let result = SearchResult{result: pair, region: Some(self.get_region()), error: error };
				found.push_back(result).expect("database_iterator::find_close_ref_region: Already checked if found full?");
			}
		}
	}

	/// Returns the database this iterator holds.
	fn get_database ( &self ) -> &dyn Database;
	
	fn get_region ( &self ) -> usize;
	fn set_region ( &mut self, i: usize );
	
	/// Returns true if the pair at the provided index fits the current region.
	fn same_region ( &self, pair_index: usize ) -> bool;
	
}


impl <'a> PyramidIterator <'a>
{
	pub fn new ( database: &'a dyn Database ) -> Self 
	{ return Self{database: database};  }
}

impl <'a> DatabaseIterator for PyramidIterator <'a>
{
	
	fn begin ( &mut self ) { }
	
	/// PyramidIterator does not have regions, next returns false.
	fn next ( &mut self ) -> bool { return false; }
	
	fn get_database ( &self ) -> &dyn Database { return self.database; }
	
	fn get_region ( &self )   -> usize         { return 0; }
	fn set_region ( &mut self, _i: usize )      { }
	
	/// Returns true as there is only one region.
	fn same_region ( &self, _pair_index: usize ) -> bool { return true; }
}









impl <'a> RegionalCrunchIterator <'a>
{
	pub fn new ( database: &'a dyn Database, reach_multiplier: Decimal ) -> Self 
	{ 
		let count = crate::nix::Distribute::angle_to_points(database.get_fov());
		return Self
		{
			database: database, 
			latice: Distribute::fibonacci_latice(count),
			index: 0,
			reach_multiplier: reach_multiplier,
			started: true,
		};  
	}
}

impl <'a> DatabaseIterator for RegionalCrunchIterator <'a>
{
	
	fn begin ( &mut self ) { self.index = 0; self.started = true; }
	
	/// PyramidIterator does not have regions, next returns false.
	fn next ( &mut self ) -> bool 
	{ 
		if !self.started
		{
			self.index += 1;
		}
		else
		{
			self.started = false;
		}
		if self.index < self.latice.len()
		{
		println!("{}", self.latice[self.index]);
		}
		return self.index < self.latice.len(); 
	}
	
	fn get_database ( &self ) -> &dyn Database { return self.database; }
	
	fn get_region ( &self )   -> usize         { return self.index; }
	fn set_region ( &mut self, _i: usize )     { }
	
	/// Returns true as there is only one region.
	fn same_region ( &self, pair_index: usize ) -> bool 
	{ 
		let p1 = self.database.find_star(self.database.get_pairs(pair_index).0);
		let p2 = self.database.find_star(self.database.get_pairs(pair_index).1);
		
		if p1.is_err() || p2.is_err()
		{
			return false;
		}
		
		return p1.unwrap().angle_distance(self.latice[self.index]) < self.database.get_fov() * self.reach_multiplier ||
			p2.unwrap().angle_distance(self.latice[self.index]) < self.database.get_fov() * self.reach_multiplier;
	}
}














impl <'a> RegionalIterator <'a>
{
	pub fn new ( database: &'a RegionalDatabase<'a> ) -> Self 
	{ return Self{database: database, index: 0};  }
}

impl <'a> DatabaseIterator for RegionalIterator <'a>
{
	
	fn begin ( &mut self ) 
	{ 
		self.index = 0;
	}
	
	fn next ( &mut self ) -> bool 
	{ 
		if self.index < self.database.num_regions() - 1
		{
			self.index += 1;
			// println!("{}", self.index);
			
			return true;
		}
		return false;
	}
	
	fn get_database ( &self ) -> &dyn Database { return self.database; }
	fn get_region ( &self )   -> usize         { return self.index; }
	fn set_region ( &mut self, i: usize )      { self.index = i; }

	
	/// Returns true if the pair at the provided index fits the current region.
	fn same_region ( &self, pair_index: usize ) -> bool
	{
		let compare = BitCompare::Any(BitField(1 << self.index));
		return self.database.pairs_region.get(pair_index).compare(compare);
	}
}














impl <'a> BoundedDeclinationIterator <'a>
{
	pub fn new ( database: &'a dyn Database, region_multiplier: Decimal ) -> Self 
	{
		let region_size = database.get_fov() * region_multiplier;
		return Self
		{
			database:  database,
			interval:  region_size,
			dec:       Radians(0.0)..Radians(0.0),
			index:     0,
			num: (Degrees(180.0).to_radians() / region_size).0.ceil() as usize,
		}
	}
}

impl <'a> DatabaseIterator for BoundedDeclinationIterator <'a>
{
	fn begin ( &mut self ) 
	{
		self.dec = Radians(0.0)..Radians(0.0);
		self.index = 0;
	}
	
	fn next ( &mut self ) -> bool 
	{
		let center = (self.interval * self.index as Decimal) - Degrees(90.0).to_radians();
		
		self.dec = center - self.interval / 1.5 .. center + self.interval / 1.5;	
		
		let valid = self.index < self.num;
		// println!("{}\t{:.2} .. {:.2}", self.index, self.dec.start.to_degrees().0, self.dec.end.to_degrees().0);
		self.index += 1;
		return valid;
	}
	
	fn get_database ( &self ) -> &dyn Database { return self.database; }
	fn get_region ( &self )   -> usize         { return self.index; }
	fn set_region ( &mut self, _i: usize )     { panic!("NOT YET IMPLEMENTED"); }

	
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

		return valid_dec;
	}
}



























impl <'a> BoundedEquatorialIterator <'a>
{
	pub fn new ( database: &'a dyn Database, region_multiplier: Decimal ) -> Self 
	{
		let region_size = Equatorial{dec: database.get_fov() * region_multiplier, ra: database.get_fov() * region_multiplier};
		return Self
		{
			index_dec:    0,
			index_ra:     0,
			database:     database,
			interval_dec: region_size.dec,
			interval_ra:  Radians(0.0),
			dec:          Radians(0.0)..Radians(0.0),
			ra:           Radians(0.0)..Radians(0.0),
			num_dec:      (Degrees(180.0).to_radians() / region_size.dec).0.ceil() as usize,
			num_ra:       0,
			region_size:  region_size,
		}
	}
	
	
	fn get_region_count_latitude ( fov: Equatorial, declination_mid: Radians ) -> Radians
	{
		// Unit Sphere
		let circumference = 1.0;
	
		// The closest angle to the equator.
		// To ensure entire sky coverage, the region must overshoot by half the FOV.
		// The angle will be an entire FOV towards the equator from the target declination.
		// let slice_angle   = cmp::max((declination_mid.abs() - fov.dec.0), 0.0);
		let mut slice_angle = declination_mid;
		slice_angle.0 = slice_angle.0.abs();
	
		// The radius of the smaller circle sliced from the line of declination.
		let slice_radius        = slice_angle.cos();
	
		// The circumference of the slice.
		let slice_circumference = slice_radius;
	
		// The ratio between the celestial sphere and the slice circumference.
		// This will specify the angular FOV of the slice.
		let slice_ratio = slice_circumference / circumference;
	
		return Degrees(360.0).to_radians() * slice_ratio ;
	}
}

impl <'a> DatabaseIterator for BoundedEquatorialIterator <'a>
{
	fn begin ( &mut self ) 
	{
		self.index_dec   = 0;
		self.index_ra    = 0;
		self.interval_ra = Radians(0.0);
		self.dec         = Radians(0.0)..Radians(0.0);
		self.ra          = Radians(0.0)..Radians(0.0);
		self.num_ra      = 0;
	}
	
	fn next ( &mut self ) -> bool 
	{
		if self.index_ra < self.num_ra
		{
			// Move to next right ascension node.
			let center = self.interval_ra * (self.index_ra as Decimal) + self.interval_ra / 2.0;
			self.ra = center - self.interval_ra / 2.0 .. center + self.interval_ra / 2.0; 
			self.index_ra += 1;
			// println!("R {}\t{:.2}..{:.2}   \t   {}\t{:.2}..{:.2}", self.index_dec, self.dec.start.to_degrees().0, self.dec.end.to_degrees().0, self.index_ra, self.ra.start.to_degrees().0, self.ra.end.to_degrees().0);
			return true;
		}
		else if self.index_dec <= self.num_dec
		{
			// Move to next declination node and setup right ascension.
			let center = self.interval_dec * (self.index_dec as Decimal) - Degrees(90.0).as_radians();
			self.dec = center - self.interval_dec / 2.0 .. center + self.interval_dec / 2.0;
		
			let mut eq_dec = Radians(center.abs()) - self.region_size.dec / 2.0;
			if eq_dec.0 < 0.0 { eq_dec.0 = 0.0; }
			let angle =  Radians(eq_dec.cos() * 2.0 * M_PI);
			self.num_ra = (angle / self.region_size.ra).0.ceil() as usize;
			self.interval_ra = Degrees(360.0).as_radians() / self.num_ra as Decimal;
			self.index_ra = 0;
			self.index_dec += 1;
			
			// println!("angle {}", angle.to_degrees());
			// println!("D {}\t{:.2}..{:.2}   \t   {}\t{:.2}..{:.2}", self.index_dec, self.dec.start.to_degrees().0, self.dec.end.to_degrees().0, self.index_ra, self.ra.start.to_degrees().0, self.ra.end.to_degrees().0);
			return self.next();
		}
		
		
		return false;
	}
	
	
	fn get_database ( &self ) -> &dyn Database { return self.database; }
	fn get_region ( &self )   -> usize         { return self.index_dec; }
	fn set_region ( &mut self, _i: usize )      { panic!("NOT YET IMPLEMENTED"); }

	
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
	
	
	
 

	// 
	// 
	// 
	// 
	// fn get_region_count_longitude ( fov: Equatorial )
	// {
	// 	// fov and not fov / 2 as if there is only 2 regions, the poles need to overlap.
	// 	let not_poles = Degrees(90.0).to_radians() - fov.dec;
	// 	return (not_poles / fov.dec).ceil + 2;
	// }
}