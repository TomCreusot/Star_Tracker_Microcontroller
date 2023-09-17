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
use core::ops::Range;
use crate::core_include::*;

// use std::ops::RangeInclusive;
#[cfg(test)] use mockall::*;
#[cfg(test)] use mockall::predicate::*;

use super::StarPair;

use crate::util::linear_lookup::LinearLookup;
use crate::util::aliases::Decimal;
use crate::util::aliases::UInt;
use crate::util::units::Equatorial;
use crate::util::units::BitField;
use crate::util::units::Radians;
use crate::util::err::Error;

pub use crate::tracking_mode::database::database::Database as Database;
pub use crate::tracking_mode::database::chunk_iterator::ChunkIterator as ChunkIterator;

#[cfg(test)] 
pub use crate::tracking_mode::database::chunk_iterator::MockChunkIterator as MockChunkIterator;
#[cfg(test)] 
pub use crate::tracking_mode::database::database::MockDatabase as MockDatabase;

mod k_vector;
pub mod pyramid_database;
pub mod regional_database;
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


#[cfg_attr(test, automock)]
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


/// The compiled k-vector database, this will allow the construction of the database and the lookup of elements in the database.
#[derive(Clone, Copy)]
pub struct PyramidDatabase <'a>
{
	/// The field of view used when the database was constructed.
	pub fov:       Radians,
	
	/// The equation of the k_vector which points to the k_vector table `k_vector`.
	pub k_lookup:  KVector,
	
	/// The k_vector table pointed to from `k_lookup`.
	pub k_vector:  &'a dyn LinearLookup<usize>,
	
	/// Pairs of stars in order of separation.  
	/// The separation is not provided, use the `k_lookup`, `k_vector` to find stars with a specific separation.  
	/// This points to the `cataloge`, where each element in the star pair is an index of the catalogue.
	pub pairs:     &'a dyn LinearLookup<StarPair<usize>>,
	
	/// The stars location in the sky.
	/// Use the `k_lookup`, `k_vector`, `pairs` to find the pairs of stars.
	pub catalogue: &'a dyn LinearLookup<Equatorial>,
}


/// This database is required for the [ChunkIteratorRegional](crate::tracking_mode::database::ChunkIteratorRegional).  
/// A list specifying the chunk the star is located in is required as the iterator uses a bitfield.
#[derive(Clone, Copy)]
pub struct RegionalDatabase<'a>
{
	/// The field of view used when the database was constructed.
	pub fov:       Radians,
	
	/// The number of bits in the `catalogue_field` bit field.
	pub num_fields: UInt,
	
	
	/// The equation of the k_vector which points to the k_vector table `k_vector`.
	pub k_lookup:  KVector,

	/// The k_vector table pointed to from `k_lookup`.
	pub k_vector:  &'a dyn LinearLookup<usize>,

	/// Pairs of stars in order of separation.  
	/// The separation is not provided, use the `k_lookup`, `k_vector` to find stars with a specific separation.  
	/// This points to the `cataloge`, where each element in the star pair is an index of the catalogue.
	pub pairs:     &'a dyn LinearLookup<StarPair<usize>>,

	/// The stars location in the sky.
	/// Use the `k_lookup`, `k_vector`, `pairs` to find the pairs of stars.
	pub catalogue: &'a dyn LinearLookup<Equatorial>,

	/// Reflects the index of the `catalogue`.  
	/// This represents what *region/chunk* of the sky the field is located in.
	/// Use this to identify if a star is relevant.
	pub catalogue_field: &'a dyn LinearLookup<BitField>
}








/// A chunk iterator is a way of optimizing the database search process.  
/// When searching the database for star matches, the database will return matches from around the entire celestial sphere.  
/// To ensure that all the pairs are within the same camera frame, a chunk iterator is used to move between each chunk/region.  
/// This ensures that only stars within the field of view will be used.  
///  
/// Consider that checking for triangle matches is a triple loop O(n^3) while a chunk search is O(m).
/// By reducing the number of incorrect matches (n) by increasing the regions (m), there is a large performance uplift.  
/// This also creates a higher reliability where the difference in time between a match and a false positive are greater.  
///   
/// ChunkIteratorNone does not have a chunk and instead is just the raw database search.  
/// This is not recommended but may be useful for testing.
pub struct ChunkIteratorNone <'a>
{
	/// Next needs to be called once, if begin has been called last this is false, otherwise true.
	started: bool,
	/// The database to search.
	database: &'a dyn Database,
}


/// A chunk iterator is a way of optimizing the database search process.  
/// When searching the database for star matches, the database will return matches from around the entire celestial sphere.  
/// To ensure that all the pairs are within the same camera frame, a chunk iterator is used to move between each chunk/region.  
/// This ensures that only stars within the field of view will be used.  
///  
/// Consider that checking for triangle matches is a triple loop O(n^3) while a chunk search is O(m).
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
	index_ra:  usize,
	
	/// The max/min range for the current declination.
	dec:       Range<Radians>,
	/// The max/min range for the current right ascension.
	ra:        Range<Radians>,
	
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



/// A chunk iterator is a way of optimizing the database search process.  
/// When searching the database for star matches, the database will return matches from around the entire celestial sphere.  
/// To ensure that all the pairs are within the same camera frame, a chunk iterator is used to move between each chunk/region.  
/// This ensures that only stars within the field of view will be used.  
/// 
/// Consider that checking for triangle matches is a triple loop O(n^3) while a chunk search is O(m).
/// By reducing the number of incorrect matches (n) by increasing the regions (m), there is a large performance uplift.  
/// This also creates a higher reliability where the difference in time between a match and a false positive are greater.  
///    
/// For ChunkIteratorEquatorial, chunks are determined by a bounded area between two declination bands.  
/// These bands are generated by having 2 caps of 1/2 *chunk_step* at the poles and a distance of *chunk_step* approaching the equator.  
/// There is also a *chunk_size_multiplier*, this allows the bands to overlap slightly as a percent of the step.  
/// `
///               *
///             -----          1/2 chunk_step
///           ----------       chunk_step
///        ---------------     chunk_step
///        ---------------     chunk_step
///           ----------       chunk_step
///             -----          1/2 chunk_step
///               *
///
/// `
///
pub struct ChunkIteratorDeclination <'a>
{
	/// The database to search.
	database: &'a dyn Database,
	
	/// fn randomiser ( current step, number of steps ) -> index of chunk (must cover all chunks starting from 0 to n - 1).
	/// Use `ChunkIteratorDeclination::randomise_parity`.  
	/// This is here because the chunks are designed to overlap.  
	/// Because they overlap, it is more likely that neighboring chunks will produce the same result.  
	/// To avoid testing the same area twice in a row, te randomiser function should input the step index and the number of steps and output the chunk to look at.  
	/// ChunkIteratorDeclination::randomise_parity will do all the even chunks followed by the odd chunks ensuring that the order does not overlap. 
	randomiser: fn (usize, usize) -> usize,
	
	/// The current step.
	index: usize,
	/// The number of bands in the celestial sphere.
	num:   usize,
	
	/// The max/min range for the current declination.
	dec: Range<Radians>,
	
	/// The distance between declination centers.  
	/// The wider this value, the less chunks there are.
	chunk_step: Radians,
	/// Multiplied by chunk step to give the range coverage of the chunk.  
	/// The wider this value, the more coverage but less performance.  
	/// `chunk_size_multiplier` must be greater than 1.
	chunk_size_multiplier: Decimal
}




/// A chunk iterator is a way of optimising the database search process.  
/// When searching the database for star matches, the database will return matches from around the entire celestial sphere.  
/// To ensure that all the pairs are within the same camera frame, a chunk iterator is used to move between each chunk/region.  
/// This ensures that only stars within the field of view will be used.  
/// 
/// Consider that checking for triangle matches is a triple loop O(n^3) while a chunk search is O(m).
/// By reducing the number of incorrect matches (n) by increasing the regions (m), there is a large performance uplift.  
/// This also creates a higher reliability where the difference in time between a match and a false positive are greater.  
///   
/// For `ChunkIteratorRegional`, a special database (`RegionalDatabase`) must be used.  
/// This database has a list containing a bitfield specifying a region where each star is located.  
/// When this iterator is used, it will check if either star in the star pair are within the search region.  
/// If the stars are outside of the region, they will be excluded.  
///  
/// These regions are a set of equal spaced circles with some overlap.  
///  
/// Due to the additional size of the database, it is recommended to use a different iterator if you are lacking space...
pub struct ChunkIteratorRegional <'a>
{
	/// The database to search.
	database: &'a RegionalDatabase<'a>,

	/// This incriments from 0 representing what bit to investigate.
	index: UInt,
	
	/// Has the iterator started iterating.  
	/// Due to the design of the other iterators, next must be called before the iterator can begin.  
	/// Therefore index should start from -1.  
	/// Since the bitfield uses a uint, this variable must specify if it is the first iteration.
	started: bool,
	
}




/// Like ChunkIteratorEquatorial except it does not iterate.  
/// This is used so that you can search a small reagion of the sky.  
/// The usecase is if you have a general idea of the location of the attitude but not good enough.
/// For areas that go past 360 degrees, either use a negative min bounds or a value over 360 for max.
pub struct ChunkAreaSearch <'a>
{
	/// The database to search.
	database: &'a dyn Database,
	
	/// The max/min range for the current declination.
	dec: Range<Radians>,
	
	/// The max/min range for the current right ascension.
	ra: Range<Radians>,
	
	/// Has begin been called without next?
	started: bool,
}