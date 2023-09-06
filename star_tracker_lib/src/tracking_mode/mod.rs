//! `tracking_mode` is the method of matching the stars found in the image to a database.
//! This program uses the pyramid method of star tracking.
//! # Pyramid Tracking Mode Algorithm
//! The pyramid method is one of the most reliable methods. It was developed by Mortari in 1997.
//! The method was modified a few times.
//!
//! ## Summary
//! The algorithm creates a kernal (a set of 3 stars) with a method to reduce the number of times a false positive star is used.
//! It then will compare the constellation with a k-vector table database.
//! If it succeeds, it will then find another star and verify it is also consistent.
//!
//! ### Creating the kernel
//! To select the stars, a looping algorithm must be chosen the proposed method is implemented in this algorithm.
//! This will access the index of 0-1-2, 2-3-4, 3-4-5, 1-2-4, 2-3-5, 1-2-5, 1-3-4, 2-4-5, 1-3-5, 1-4-5.
//! This method is done to reduce the times a specific star is used as it may be a false star.
//!
//! ### Using the kernel
//! By working with the database, a set of star 'pairs' are found.
//! These each pair contains the id/location of each star.
//! By finding the common star from 2 pairs, the star can be identified in the database and the sample.
//! The matched triangle will then have its specularity checked (check if it is flipped).
//!
//! ### Pyramid
//! If a correct kernel is found, an extra test is performed where another star is picked, finding the distance from this star to the other stars, this is then compared with the database.
//!
//! ### Database
//! Refer to [this file](database/mod).
//!
//!
//! ## Features
//! This method features:
//! - An algorithm which can fit on a microcontroller.
//! - Contains a method to reduce the repetition of using a false star.
//! - Fast as uses K-Table.
//! - Checks if the triangle is flipped (false positive).
//! - Adds an extra star to verify correct identification opposing other triangle constellation based methods.
//!
//!
//! ## Useful links:
//! - [Original Paper](https://www.researchgate.net/publication/254199748_Lost-in-Space_Pyramid_Algorithm_for_Robust_Star_Pattern_Recognition)
//! - [Pseudo Interpretation](https://arxiv.org/pdf/1808.08686.pdf#page=7)
//! - [Good Explanation](http://mtc-m21b.sid.inpe.br/col/sid.inpe.br/mtc-m21b/2017/08.10.22.44/doc/publicacao.pdf#page=105)

#[cfg(test)] use mockall::predicate::*;
#[cfg(test)] use mockall::*;

use crate::core_include::*;

use crate::tracking_mode::database::ChunkIterator;
use crate::tracking_mode::database::SearchResult;

use crate::util::units::Vector3;
use crate::util::units::Equatorial;
use crate::util::units::Match;
use crate::util::units::Radians;
use crate::util::list::List;
use crate::util::list::ArrayList;
use crate::util::err::Error;
use crate::util::aliases::Decimal;

pub mod kernel_iterator;
pub mod constellation;
pub mod pilot_finder;
pub mod star_pair;
pub mod star_triangle;
pub mod star_triangle_iterator;
pub mod specularity;
pub mod database;


//###############################################################################################//
//
//										Structs
//
//###############################################################################################//


/// A set of 2 stars in equatorial space, this represents a line / angle.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct StarPair<T>		( pub T, pub T );

/// A set of 3 stars in T space, this represents a triangle.
/// For lookup in the database, it is easier to use equatorial as it requires less space.
/// For equations, you must use cartesian3D.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StarTriangle<T>	( pub T, pub T, pub T );

/// A set of 4 stars in 3D space, this represents a pyramid.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct StarPyramid<T> ( pub T, pub T, pub T, pub T );


/// Use to search for a pilot star to match a star triangle.
/// This is needed to generate a constellation.
/// # To Use:
/// ```
/// #![macro_use]
/// use star_tracker_lib::pilot_finder_array;
/// use star_tracker_lib::pilot_finder_vec; // Only compiles with nix flag.
/// use star_tracker_lib::tracking_mode::PilotFinder;
/// use star_tracker_lib::tracking_mode::database::SearchResult;
/// use star_tracker_lib::util::list::ArrayList;
///
/// /// For Computer
/// let finder = pilot_finder_vec!(); // Only compiles with nix flag...
/// /// For Embed
/// const ARRAY_SIZE: usize = 100; // The number of database matches that can be stored.
/// let finder = pilot_finder_array!(ARRAY_SIZE);
/// ```
pub struct PilotFinder<'a>
{
	/// This variable should be private but rust has weird rules making it impossible with macros...
	pub sides_a: &'a mut dyn List<SearchResult>,
	/// This variable should be private but rust has weird rules making it impossible with macros...
	pub sides_b: &'a mut dyn List<SearchResult>,
	/// This variable should be private but rust has weird rules making it impossible with macros...
	pub sides_c: &'a mut dyn List<SearchResult>
}


/// The result from the star triangle iterator.
// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct IterationResult
// {
// 	/// The location of the stars in the found list.
// 	pub input:  StarTriangle<usize>,
// 	/// The location of the stars in the database.
// 	pub output:  StarTriangle<usize>,
// 	/// The lower this number, the more likely it is valid.
// 	pub error:   Decimal,
// }

/// By finding every potential StarTriangle before testing their valid is bad for performance.
/// At 3 stars, there is one triangle.
/// At 4 stars, there is 4 triangles.
/// At 5 stars, there is 10... (exponential).
/// By only checking one at a time, it is likely that only 1 triangle needs to be generated.
pub struct StarTriangleIterator <const N_MAX_MATCHES: usize>
{
	/// The iterator for comparing star pairs.
	/// Ideal as does not priorities the first star pair found.
	kernel: KernelIterator,

	/// The values searched by the kernel.
	input:  StarTriangle<usize>,

	/// After calling next, this is the result.
	/// Used for finding the pilot.
	expected_triangle: Option<Match<StarTriangle<usize>>>,

	/// All found elements from the database when searched by the star pairs constructing input.
	pair_a: ArrayList<SearchResult, {N_MAX_MATCHES}>,
	/// All found elements from the database when searched by the star pairs constructing input.
	pair_b: ArrayList<SearchResult, {N_MAX_MATCHES}>,
	/// All found elements from the database when searched by the star pairs constructing input.
	pair_c: ArrayList<SearchResult, {N_MAX_MATCHES}>,

	/// All found elements for the current pilot star and star_a.
	pair_p_a: ArrayList<SearchResult, {N_MAX_MATCHES}>,
	/// All found elements for the current pilot star and star_b.
	pair_p_b: ArrayList<SearchResult, {N_MAX_MATCHES}>,
	/// All found elements for the current pilot star and star_c.
	pair_p_c: ArrayList<SearchResult, {N_MAX_MATCHES}>,

	/// The current search index of pair_a.
	index_a: isize,
	/// The current search index of pair_b.
	index_b: usize,
	/// The current search index of pair_c.
	index_c: usize,

	/// The current index of the pilot star.
	index_p: isize,
	/// The current index of the pilot star.
	index_p_a: isize,
	/// The current index of the pilot star.
	index_p_b: usize,
	/// The current index of the pilot star.
	index_p_c: usize,

	angle_tolerance: Radians,
}


/// An iterator which reduces the chances of getting the same star twice.
/// This is useful in the case that the first stars are invalid, it will take less processing time to get to useful stars.
/// For a size of 6, the sequence would be:
/// 0-1-2, 2-3-4, 3-4-5, 1-2-4, 2-3-5, 1-2-5, 1-3-4, 2-4-5, 1-3-5, 1-4-5.
///
/// # Example
/// ```
/// use star_tracker_lib::tracking_mode::KernelIterator;
/// let mut iter = KernelIterator::new(10);
/// while iter.step()
/// {
/// 	assert!(iter.i != 1000);
/// 	assert!(iter.j != 1000);
/// 	assert!(iter.k != 1000);
/// }
/// ```
pub struct KernelIterator
{
	/// The first star index to use.
	pub i : usize,
	/// The second index to use.
	pub j : usize,
	/// The third index to use.
	pub k : usize,

	/// The number of elements to iterate through.
	size  : usize,
	/// 0 to (n - dj - dk - 1)
	di : usize,
	/// 1 to (n - 1 - dj)
	dj : usize,
	/// 1 to (n - 2)
	dk : usize
}



//###############################################################################################//
//
//										Enums
//
//###############################################################################################//

/// The return type for the star pyramid.
/// Either there is no match or less than 3 stars	(None)
/// There is a match but only 3 supplied stars		(Triangle)
/// There is a match and more than 4 supplied stars	(Pyramid)
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Constellation
{
	Pyramid  ( Match<StarPyramid<Equatorial>>  ),
	Triangle ( Match<StarTriangle<Equatorial>> ),
	None
}


/// When comparing constellation, triangles are used.
/// Specularity is the test to see if the triangle is flipped.
/// If the triangle is flipped, it is invalid.
pub struct Specularity
{
	/// If a triangles area is too small (i.e. a strait line or small), any inaccuracy could cause it to be considered flipped.
	/// Use this to define the minimum specularity until the specularity is unimportant.
	specularity_min: Decimal,
}

/// A specularity test.
/// Ignore means the area was too small.
/// Valid means that the sign is a valid way of checking specularity in the current triangle.
#[derive(Debug)]
#[derive(PartialEq)]
pub enum SpecularityResult
{
	Ignore,
	Valid(bool)
}




//###############################################################################################//
//
//										Traits
//
//###############################################################################################//




/// Used to tell the search that it should give up.
/// It is recomended to abandon the search if a certain time is exceeded as the longer a search takes the less likely it is correct.
/// Also some searches can take far longer then others.
#[cfg_attr(test, automock)]
pub trait AbandonSearch
{
	/// Returns true if the search should be abandon.
	/// Returns false if the search should continue.
	fn should_abort ( &self ) -> bool;
}


#[cfg_attr(test, automock)]
/// Use to compare regions.
/// This may use the
///
pub trait RegionCompare
{
	/// Returns if the input coordinates are within the same region.
	fn compare ( &self, compare_a: Equatorial, compare_b: Equatorial ) -> bool;
}


#[cfg_attr(test, automock)]
pub trait PyramidConstruct
{
	/// Finds the pilot 
	/// # Arguments
	/// * `stars` - The stars from the image. 
	/// * `database` - The database to lookup.
	/// * `angle_tolerance` - How much error a star pair can have until it is not considered the same.
	/// 	Used for searching the database.
	/// * `input` - The star triangle from the input (what stars are being used).
	/// * `output` - The star triangle from the output in the same order as input.
	/// # Returns
	/// Ok(pilot) if valid.
	fn find_pilot (
				&mut self,
				stars : &dyn List<Equatorial>,
				database : &dyn ChunkIterator,
				angle_tolerance: Radians,
				input : StarTriangle<usize>,
				output : StarTriangle<usize>,
			) -> Error<Match<usize>>;
}



#[cfg_attr(test, automock)]
pub trait TriangleConstruct
{
	/// Call this to get the next StarTriangle observed/database pair.
	/// # Arguments
	/// * `stars` - The observed stars in the image.
	/// * `database` - The database of stars to search through.
	/// # Returns
	/// * None if there is no more available star triangles with the given parameters.
	/// * Some(Match{input: observed star triangle, output: database match}) if possible.
	fn next ( &mut self, stars: &dyn List<Equatorial>, database: &mut dyn ChunkIterator
		) -> Option<Match<StarTriangle<usize>>>;

	/// Iterates though suitable pilot stars for the given star triangle.
	/// Ensure the database iterator has not iterated since calling next.
	/// # Arguments
	/// * `stars`    - The observed stars in the image.
	/// * `database` - The database of stars to search through (That has not been iterated since next).
	/// # Returns
	/// * None if there is no more available pilot stars.
	/// * Some(Match{input: observed star triangle, output: database match}) if possible.
	fn next_pilot ( &mut self, stars: &dyn List<Equatorial>, database: &mut dyn ChunkIterator
	) -> Option<Match<usize>>;

	/// Prepares the StarTriangleIterator for iterating.
	/// # Arguments
	/// * `angle_tolerance` - When searching the database, the tolerance to use.
	/// * `stars` - The observed stars.
	fn begin ( &mut self, angle_tolerance: Radians, stars: &dyn List<Equatorial> );
}


#[cfg_attr(test, automock)]
pub trait SpecularityConstruct
{
	/// Returns true if the triangle is the same orientation OR a triangle is IGNORE.
	fn same ( &mut self, a: &StarTriangle<Vector3>, b: &StarTriangle<Vector3> ) -> bool;
}
