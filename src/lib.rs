//!
//! # src
//! This is the library.
//! It contains a set of modules required for the functionality of the star tracker.
//!
//!
//! ## Improvements
//! - [List Iterator](util::list::ListIterator) is underused.
//! 	- Problem
//!			- Using Indexing is less safe than an iterator.
//!		- Solution
//!			- Implement the created iterator in any iteration loops.
//!
//! - [Image Thresholding](image_processing::image::Image::percent_threshold) in image processing could use a better method.
//! 	- Problem
//! 		- Currently it must generate a histogram of the whole image (which takes time).
//! 		- It also is less effective if a region of the image experiances bloom from a bright object.
//! 	- Solution
//! 		- By taking a sample of points in a set of regions of the image, it will be faster.
//! 		- This allows you to use addaptive thresholding.
//! 		- This also allows you to cut out regions which are too bright.
//!
//! - [Blob Detection](image_processing::Blob::find_blobs) wipes across the image with each pixel and starts in the same place.
//! 	- Problem
//! 		- By wiping across the image, this takes time.
//! 		- Most stars will be more than one pixel wide if the lens is correctly fitted.
//! 		- The blob detection starts in the top left which is in a region which will experiance the most distortion.
//! 		- Any hot pixels in the area will always be picked up.
//! 	- Solution
//! 		- Use every second pixel, if no stars are found, use offset by 1.
//! 		- Start in the center and work out.
//!
//! - [Projection](projection) is not finished.
//! 	- Problem
//! 		- The projection only can do basic projection.
//! 		- It cannot distort/undistort.
//! 		- It cannot calibrate (remember to use infinite focus distance).
//! 	- Solution
//! 		- Finish it .
//!
//! - [Tracking Mode](tracking_mode::Constellation::find) can sometimes detect false positives.
//! 	- Problem
//! 		- When integration testing the tracking mode, false positives occur.
//! 		- This is far worse than a false negative.
//! 	- Solution
//! 		- If there is enough stars, instead of making a pyramid, make a 5 point pentahedron.
//!
//! - [Tracking Mode](tracking_mode::Constellation::find) has never created a triangle in integration tests.
//! 	- Problem
//! 		- Code may not be working properly.
//! 	- Solution
//! 		- Check if it should work.
//!
//! - [Regional Database](tracking_mode::database::RegionalDatabase) maxes out at 8, 32, 64, 128 values.
//! 	- Problem
//! 		- To increase the speed of the regional database, a bit field is used in the lookup.
//! 		- The bitfield has a maximum capacity which is quite low.
//! 		- For small field of views this is a problem.
//! 		- A point in the database may fit in multiple regions.
//! 		- Some hardware will only be 8 bit.
//! 		- Increasing the number of bits drastically increases the size of the database.
//! 	- Solution
//! 		- Find one.
//!
//! - [Attitude Determination](attitude_determination) voting method is confusing.
//! 	- Problem
//! 		- I have no idea how it works, it just seems to work with all the tests.
//! 	- Solution
//! 		- Get someone to verify it is correct.
//! 		- Ensure there is no singularities.
#![feature(generic_const_exprs)]				// Newer version of associated_type_defaults.

// #![feature(const_generics)]					// Allows constant values in generics.
// 
// #![feature(const_evaluatable_checked)]		// Allows generic struct variables.
// #![feature(associated_type_defaults)]		// Allows list to implement iterator.

#![feature(const_fn_floating_point_arithmetic)] // allows constant Degrees.as_radians() function.

extern crate curl;
extern crate csv;
extern crate serde;
extern crate rand;
extern crate mockall;
extern crate static_assertions;

extern crate opencv;

#[allow(dead_code)]
// #[feature(const_evaluatable_checked)]
pub mod util;
#[allow(dead_code)]
pub mod image_processing;
#[allow(dead_code)]
pub mod attitude_determination;

#[allow(dead_code)]
pub mod nix;

#[allow(dead_code)]
pub mod tracking_mode;

#[allow(dead_code)]
pub mod config;

pub mod integration_tests;

#[allow(dead_code)]
pub mod projection;
