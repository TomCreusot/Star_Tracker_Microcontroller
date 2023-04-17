//! The database for the pyramid method is a set of 2 lookup tables (star pairs, star catalogue) and an equation (k-vector)
//!
//! # [K-Vector](https://en.everybodywiki.com/K-Vector)
//! The k-vector is a fast lookup table which has a time complexity independent of the size of the database.
//! It uses an equation to locate specific *bins* in the database where the element can be found.
//!
//! ## Equations
//! **Variables**
//! * `m` - Gradient
//! * `q` - Intercept
//! * `y` - The input variable (angular distance).
//! ### Prepossessing
//! This is the process of creating the k-vector, this is only required once.
//! **Symbols**
//! * `D_max` - The maximum value of an element in the database.
//! * `D_min` - The minimum value of an element in the database.
//! * `n` - The number of elements in the database.
//! * `N` - The number of vector bins, a larger value will be faster, however it will cost more memory, ideal: N = n.
//! * `d` - (n - 1)
//! * `e` - Relative machine precision rust uses [binary32/64](https://en.wikipedia.org/wiki/Machine_epsilon).
//!   * `f32 = 1.19e-07`
//!   * `f64 = 2.22e-16`
//!
//! **Finding Gradient**
//! `m = ( D_max - D_min + 2 d e ) / ( N - 1 )`
//!
//! **Finding Intercept**
//! `q = D_min - d e`
//!
//! ### Searching
//! By inputting 2 equations, the bounds of the element can be found.
//! `k_a = floor(y - q) / m + 1`
//! `k_b = ceil(y - q) / m`
//! i is within k_a and k_b.
//!
//!
//!
//!
//!
//!
//!
//!
//!
//! # List of Pairs
//! * The list of star pairs is a lookup table containing 2 references to stars.
//! * The stars will have their angular distances calculated, this will be the order they are sorted in.
//! * The stars should only form a pair if they are within the same field of view.
//! * The index and position are not required to store.
//! e.g.
//!
//! |  index  |    a    |    b    |
//! |:-------:|:-------:|:-------:|
//! |    0    | *star a | *star b |
//! |    1 	  | *star c | *star b |
//! |    2    | *star c | *star a |
//! |   ...   |   ...	|   ...   |
//!
//!
//!
//!
//!
//!
//! # Star Catalogue
//! The star catalogue is just the positions of the stars.
//! This can be done using equatorial (less memory) or Cartesian coordinates (less time).
//! The equatorial is more beneficial for this project.
//! e.g.
//!
//! | index (*star) | ra  | dec |
//! |:-------------:|:---:|:---:|
//! | 0 (*star a)   |  1  | -1  |
//! | 1 (*star b)   |  2  |  1  |
//! | 2 (*star c)   |  2  | -1  |
//! | 3 (*star d)   |  3  |  0  |
//! |    ...        | ... | ... |
use std::ops::Range;
// use std::ops::RangeInclusive;
use mockall::*;
use mockall::predicate::*;

use super::StarPair;

use crate::util::units::Equatorial;
use crate::util::units::Radians;
use crate::util::aliases::Decimal;
use crate::util::linear_lookup::LinearLookup;
use crate::util::err::Error;

pub use crate::tracking_mode::database::database::Database as Database;
pub use crate::tracking_mode::database::chunk_iterator::ChunkIterator as ChunkIterator;
pub use crate::tracking_mode::database::database::MockDatabase as MockDatabase;

mod k_vector;
mod star_database_element;
#[cfg(not(feature = "setup"))]
pub mod array_database;
pub mod pyramid_database;
pub mod database;
pub mod chunk_iterator;
pub mod search_result;
/// The database equation which points to the star pair database.
///
///
///
#[derive(Copy, Clone)]
pub struct KVector
{
	/// The gradient for the k-vector bin equation.
	/// This also represents ~ 1/2 the tolerance or the distance between bins.
	pub gradient      : Decimal,
	/// The y intercept for the k-vector bin equation.
	pub intercept     : Decimal,

	/// The smallest value in the database.
	pub min_value     : Radians,

	/// The largest value in the database.
	pub max_value     : Radians,

	/// The number of kvector elements.
	pub num_bins      : usize,
}


#[automock]
pub trait KVectorSearch
{
	/// Gets the index of where the value is located in the star pair list.
	/// This may include the neigbouring bins as it is on the edge of the bin.
	/// i.e.
	/// If the bin tolerance is 10:
	/// [1: (0 to 10), 2: (10 to 20), 3: (20 to 30)],
	/// If you enter 19, you will receive 2 and 3.
	/// If you enter 15, you will receive 1, 2 and 3.
	/// # Arguments
	/// * `value` - The value of the angular interstar distance.
	fn get_bins ( &self, value: Radians, tolerance: Radians ) -> Error<Range<usize>>;
}



/// The result for querying for a close match star pair.
/// Used in Database::find_close_ref.
#[derive(Debug, Copy, Clone)]
pub struct SearchResult
{
	/// The catalogue location for both stars.
	pub result:   StarPair<usize>,
	/// How reliable the search result is.
	/// The smaller the number the closer the result is to be true.
	pub error:    Decimal,
}


/// An element with all the details required to insert into the database.
#[derive(Clone, Copy, Debug)]
pub struct StarDatabaseElement
{
	/// The location of the stars (does not matter what order they are in).
	pub pair : StarPair<usize>,
	/// The angular separation between the stars.
	pub dist : Radians,
}


/// The compiled k-vector database, this will allow the construction of the database and the lookup of elements in the database.
#[derive(Clone, Copy)]
pub struct PyramidDatabase <'a>
{
	pub fov:       Radians,
	pub k_lookup:  KVector,
	pub k_vector:  &'a dyn LinearLookup<usize>,
	pub pairs:     &'a dyn LinearLookup<StarPair<usize>>,
	pub catalogue: &'a dyn LinearLookup<Equatorial>,
}

/// A chunk iterator is a way of optimising the database search process.  
/// When searching the database for star matches, the database will return matches from around the entire celestial sphere.  
/// To ensure that all the pairs are within the same camera frame, a chunk iterator is used to move between each chunk/region.  
/// This ensures that only stars within the field of view will be used.  
///  
/// Consider that checking for triangle matches is a tripple loop O(n^3) while a chunk search is O(m).
/// By reducing the number of incorrect matches (n) by increasing the regions (m), there is a large performance uplift.  
/// This also creates a higher reliability where the difference in time between a match and a false positive are greater.  
///   
/// ChunkIteratorNone does not have a chunk and instead is just the raw database search.  
/// This is not recomended but may be useful for testing.
pub struct ChunkIteratorNone <'a>
{
	/// The database to search.
	database: &'a dyn Database,
}


/// A chunk iterator is a way of optimising the database search process.  
/// When searching the database for star matches, the database will return matches from around the entire celestial sphere.  
/// To ensure that all the pairs are within the same camera frame, a chunk iterator is used to move between each chunk/region.  
/// This ensures that only stars within the field of view will be used.  
///  
/// Consider that checking for triangle matches is a tripple loop O(n^3) while a chunk search is O(m).
/// By reducing the number of incorrect matches (n) by increasing the regions (m), there is a large performance uplift.  
/// This also creates a higher reliability where the difference in time between a match and a false positive are greater.  
///  
/// For ChunkEquatorialIterator, chunks are determined by a bounded area between a min and max right ascension and declination.  
/// The declination bounds are separated by the vertical field of view of the sensor.  
/// The right ascension bounds are calculated by the number of photos are required to loop around the declination band closest to the equator.  
/// The regions can be expanded with the reach variable in the constructor if the field of view is too low (too many chunks).
pub struct ChunkIteratorEquatorial <'a>
{
	/// The database to search.
	database: &'a dyn Database,
	
	
	/// How many declination (vertical bands) have been covered.
	index_dec: usize,
	/// How many right ascension (horizontal bands) have been covered in the current declination band.
	index_ra: usize,
	
	/// The max/min range for the current declination.
	dec:      Range<Radians>,
	/// The max/min range for the current right ascension.
	ra:       Range<Radians>,
	
	/// The number of declination bands to be covered.
	num_dec:  usize,
	/// Number of right ascension perspectives in the current declination.
	num_ra:   usize,
	
	/// The distance between declination and right ascension band centers.  
	/// The wider this value, the less chunks there are.
	chunk_step: Radians,
	
	/// Multiplied by chunk step to give the range coverage of the chunk.  
	/// The wider this value, the more coverage but less performance.  
	/// `chunk_size_multiplier` must be greater than 1.
	chunk_size_multiplier: Decimal
}


