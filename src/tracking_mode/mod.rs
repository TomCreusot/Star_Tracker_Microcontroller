//! `tracking_mode` is the method of matching the stars found in the image to a database.
//! This program uses the pyramid method of star tracking.
//! # Pyramid Tracking Mode Algorithm
//! The pyramid method is one of the most reliable methods. It was developed by Mortari in 1997.
//! The method was modified a few times.
//!
//! ## Summary
//! The algorithm creates a kernal (a set of 3 stars) with a method to reduce the number of times a false positive star is used.
//! It then will compare the constellation with a k-vector table database.
//! If it succeeds, it will then find another star and varify it is also consistant.
//!
//! ### Creating the kernal
//! To select the stars, a looping algerithm must be chosen the proposed method is implemented in this algorithm.
//! This will access the index of 0-1-2, 2-3-4, 3-4-5, 1-2-4, 2-3-5, 1-2-5, 1-3-4, 2-4-5, 1-3-5, 1-4-5.
//! This method is done to reduce the times a specific star is used as it may be a false star.
//!
//! ### Using the kernal
//! By working with the database, a set of star 'pairs' are found.
//! These each pair contains the id/location of each star.
//! By finding the common star from 2 pairs, the star can be identified in the database and the sample.
//! The matched triangle will then have its specularity checked (check if it is flipped).
//!
//! ### Pyramid
//! If a correct kernal is found, an extra test is performed where another star is picked, finding the distance from this star to the other stars, this is then compared with the database.
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
//! - Adds an extra star to varify correct identification opposing other triangle constilation based methods.
//!
//!
//! ## Useful links:
//! - [Original Paper](https://www.researchgate.net/publication/254199748_Lost-in-Space_Pyramid_Algorithm_for_Robust_Star_Pattern_Recognition)
//! - [Pseudo Interpritation](https://arxiv.org/pdf/1808.08686.pdf#page=7)
//! - [Good Explination](http://mtc-m21b.sid.inpe.br/col/sid.inpe.br/mtc-m21b/2017/08.10.22.44/doc/publicacao.pdf#page=105)

use crate::util::units::Cartesian3D;
use crate::util::aliases::UInt;

pub mod kernel_iterator;
// pub mod star_pyramid;
pub mod star_pair;
pub mod star_triangle;
pub mod database;




/// A set of 2 stars in equatorial space, this represents a line / angle.
#[derive(Clone, Copy)]
pub struct StarPair<T>			( pub T, pub T );

/// A set of 3 stars in T space, this represents a triangle.
/// For lookup in the database, it is easier to use equatorial as it requires less space.
/// For equations, you must use cartesian3D.
pub struct StarTriangle<T>	( pub T, pub T, pub T );

/// A set of 4 stars in 3D space, this represents a pyramid.
pub struct StarPyramid		( pub Cartesian3D, pub Cartesian3D, pub Cartesian3D );


/// An iterator which reduces the chances of getting the same star twice.
/// This is useful in the case that the first stars are invalid, it will take less processing time to get to useful stars.
/// For a size of 6, the sequence would be:
/// 0-1-2, 2-3-4, 3-4-5, 1-2-4, 2-3-5, 1-2-5, 1-3-4, 2-4-5, 1-3-5, 1-4-5.
///
/// # Example
/// ```
/// use star_tracker::tracking_mode::KernelIterator;
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
	pub i : UInt,
	/// The second index to use.
	pub j : UInt,
	/// The third index to use.
	pub k : UInt,

	/// The number of elements to iterate through.
	size  : UInt,
	/// 0 to (n - dj - dk - 1)
	di : UInt,
	/// 1 to (n - 1 - dj)
	dj : UInt,
	/// 1 to (n - 2)
	dk : UInt
}
