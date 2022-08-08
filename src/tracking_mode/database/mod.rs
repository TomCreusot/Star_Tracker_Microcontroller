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
use crate::util::list::List;

use crate::util::err::{/*Errors, */Error};

mod k_vector;
mod star_database_element;
#[cfg(not(feature = "setup"))]
pub mod array_database;
pub mod pyramid_database;

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
pub struct PyramidDatabase
{
	pub fov:       Radians,
	pub k_lookup:  KVector,
	pub k_vector:  &'static [usize],
	pub pairs:     &'static [StarPair<usize>],
	pub catalogue: &'static [Equatorial],
}


#[automock]
pub trait Database
{
	fn find_close_ref ( &self, find : Radians, tolerance: Radians, 
														found : &mut dyn List<StarPair<usize>> );
	fn find_star ( &self, index: usize ) -> Error<Equatorial>;
	fn trim_range ( &self, find: Radians, tolerance: Radians, range: Range<usize> )-> Range<usize>;
}
