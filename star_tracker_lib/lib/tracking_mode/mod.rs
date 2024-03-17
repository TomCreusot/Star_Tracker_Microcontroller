//! `tracking_mode` is the method of matching the stars found in the image to a database.  
//!  
//! This program uses the pyramid method of star tracking.  
//! # Pyramid Tracking Mode Algorithm  
//! The pyramid method is one of the most reliable methods. It was developed by Mortari in 1997.  
//! The method was modified a few times.  
//!
//! ## Summary
//! The algorithm creates a kernel (a set of 3 stars) with a method to reduce the number of times a false positive star is used.  
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
//!
//!
//!
//!
//! # How to Use
//! ``` ignore
//! use star_tracker_lib::tracking_mode::database::MockDatabase;
//! use star_tracker_lib::tracking_mode::database::Database;
//! use star_tracker_lib::tracking_mode::AbandonSearchFailures;
//! use star_tracker_lib::tracking_mode::StarTriangleIterator;
//! use star_tracker_lib::tracking_mode::ConstellationResult;
//! use star_tracker_lib::tracking_mode::AbandonSearch;
//! use star_tracker_lib::tracking_mode::Constellation;
//! use star_tracker_lib::tracking_mode::Specularity;
//!
//! use star_tracker_lib::util::units::Equatorial;
//! use star_tracker_lib::util::units::Degrees;
//! use star_tracker_lib::util::units::Vector3;
//! use star_tracker_lib::util::units::Match;
//! use star_tracker_lib::util::list::List;
//!
//! // To construct the database, use the tools in *star_tracker_nix*, look at demo.
//! let database_iterator = // database goes here.
//!
//!
//! // Pretend these are image values which have been projected onto a unit sphere.
//! let stars_3d: Vec<Equatorial> = vec![];
//!
//! // This is a way for increasing the speed of the algorithm.
//! // Just use an array size which is large and you can fit...
//! const SOME_LARGE_ARRAY_SIZE: usize = 1000;
//! let iterator = StarTriangleIterator::<SOME_LARGE_ARRAY_SIZE>::new();
//!
//! // The algorithm tries to create a triangle with a set of stars. 
//! // If the triangle is flipped, it is not a proper match.
//! // Specularity is a way of doing this.
//! // There is a min specularity where it is effectively a strait line.
//! // In this case, if the specularity is below this number, specularity is ignored.
//! // Specularity::default() has a good value, however if this isn't working for you, have a play around.
//! // Look in *Specularity* for more details.
//! let specularity = Specularity::default();
//!
//! // This will abort the program if it is taking too long.
//! // This abort uses the number of fails.
//! // The algorithm will try and construct a triangle with 3 stars, if it cannot, a fail occurs.
//! // This is set so if it cannot make a triangle 10 times in a row, it will abort.
//! // It is important to abort early as a success usually takes less than 5 times.
//! // The more fails, the more likely it will be incorrect and unreliable.
//! // You should also team this up with a timer, however this is not platform specific so make your own.
//! // There is a timer and fails counter abort in *star_tracker_nix* used by *demo*. 
//! let fails_good = 10;   // Unsuccessful matches.
//! let mut abort = AbandonSearchFailures::new(fails_good);
//!
//!
//! // How inaccurate is the lens?
//! // You will need to test the lens.
//! // Use nova.astrometry.net and get the corr file, put it through corr_analyzer in star_tracker_nix.
//! // Don't use an error above 0.1 degrees as it will likely fail.
//! let angle_tolerance = Degrees(0.08).to_radians();
//!
//!
//!
//! // The min and max (inclusive) required stars.
//! // If not enough stars can be formed into a constellation, the search will continue and will eventually fail.
//! // If there is too many stars in the constellation, the algorithm will end early on the upper bounds to save time.
//! // The pyramid method should use 4 stars, so it is set to 4.
//! let required_stars = 4..=4;
//!
//!
//! let mut output: Vec<Match<usize>> = Vec::new();
//!
//!
//! 	let success = Constellation::find_all (
//! 	&stars_3d, &mut database_iterator, &mut iterator,
//! 	&mut specularity, &mut abort, angle_tolerance, required_stars, &mut output 
//! );
//!
//!	match success
//! {
//! 	ConstellationResult::ErrorNoTriangleMatch      { fails } => 
//! 		println!("FAILED: Could not match any stars; {} failures.", fails),
//!
//! 	ConstellationResult::ErrorAborted              { fails } =>
//! 		println!("FAILED: Aborted due to AbandonSearch parameter; {} failures.", fails),
//!
//! 	ConstellationResult::ErrorInsufficientPyramids { fails } =>
//! 		println!("FAILED: Not enough matched stars; {} failures.", fails),
//!
//! 	ConstellationResult::Success                   { fails } =>
//! 		println!("SUCCESS; with {} fails.", fails),
//! }
//! ```

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
/// 
/// For lookup in the database, it is easier to use equatorial as it requires less space.
/// For equations, you must use cartesian3D.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StarTriangle<T>	( pub T, pub T, pub T );

/// A set of 4 stars in 3D space, this represents a pyramid.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct StarPyramid<T> ( pub T, pub T, pub T, pub T );


/// Use to search for a pilot star to match a star triangle.  
///
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

/// A way of speeding up the database search algorithm.  
///  
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


/// An iterator which reduces the chances of getting the same false star twice. 
/// 
/// This is useful in the case that the first stars are invalid, it will take less processing time to get to useful stars.  
/// For a size of 6, the sequence would be:  
/// 0-1-2,  
/// 2-3-4,   
/// 3-4-5,  
/// 1-2-4,  
/// 2-3-5,   
/// 1-2-5,   
/// 1-3-4,   
/// 2-4-5,   
/// 1-3-5,   
/// 1-4-5.   
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



/// A type of AbandonSearch 
/// 
/// where [should_abort](crate::tracking_mode::AbandonSearch::should_abort) calls are counted.    
/// If this number exceeds the max failure value, `should_abort` will be set to true.    
/// During a constellation search, if enough failures occur, the result is likely to be incorrect.    
///  
/// You should also combine this with a timer and not just rely on failures.  
/// A timer is not implemented as this is a no_std library.
pub struct AbandonSearchFailures
{
	/// The max number of failures allowed.
	max: usize,

	/// The number of failures that have occurred.
	count: usize,
}



//###############################################################################################//
//
//										Enums
//
//###############################################################################################//

/// The output from [find_all](crate::tracking_mode::Constellation::find_all).
///  
/// This is to provide details on how reliable the result was.  
#[derive(Debug)]
pub enum ConstellationResult
{
	/// No triangles were successfully identified.  
	/// This could be due to insufficient stars or no 3 stars were accurate enough to be considered.   
	/// `fails` refers to the number of times the algorithm was unable to identify a star triangle.  
	ErrorNoTriangleMatch      { fails: usize },

	/// The `abort: AbandonSearch` variable passed in has returned true to `should_abort`.   
	/// This is likely that the time or number of fails were exceeded.  
	/// `fails` refers to the number of times the algorithm was unable to identify a star triangle.  
	ErrorAborted              { fails: usize },

	/// A triangle was identified, however, the minimum number of matches were not reached.  
	/// The `num_stars` min value is recommended to be 4 for a reliable result.  
	/// If `num_stars` is too high or there is not enough stars, you may need to lower the size.  
	/// `fails` refers to the number of times the algorithm was unable to identify a star triangle.  
	ErrorInsufficientPyramids { fails: usize },
	
	/// The algorithm succeeded.
	/// It is likely that the result in `matches` is reliable.  
	/// `fails` refers to the number of times the algorithm was unable to identify a star triangle.  
	Success                   { fails: usize },
}

/// The return type for the star pyramid.  
///
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


/// Ensures the constellation is the correct orientation.  
///
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
///  
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
///
/// It is recommended to abandon the search if a certain time is exceeded as the longer a search takes the less likely it is correct.  
/// Also some searches can take far longer then others.  
#[cfg_attr(test, automock)]
pub trait AbandonSearch
{
	/// Returns true if the search should be abandon.  
	/// Returns false if the search should continue.  
	fn should_abort ( &mut self ) -> bool;
}



#[cfg_attr(test, automock)]
/// Use to compare regions.  
///
/// This may use the  
///
pub trait RegionCompare
{
	/// Returns if the input coordinates are within the same region.
	fn compare ( &self, compare_a: Equatorial, compare_b: Equatorial ) -> bool;
}


#[cfg_attr(test, automock)]
/// Used to find the pilot star for the found star triangle.
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
/// Used to iterate over star triangles until a match is found.
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
/// Used to check the specularity of a star triangle.
pub trait SpecularityConstruct
{
	/// Returns true if the triangle is the same orientation OR a triangle is IGNORE.
	fn same ( &mut self, a: &StarTriangle<Vector3>, b: &StarTriangle<Vector3> ) -> bool;
}
