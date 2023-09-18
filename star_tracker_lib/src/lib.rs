//! # src
//! This is a functional star tracker library which use the Pyramid Tracking method.  
//! It has been designed to run on any platform as it is `no_std`.  
//! To run the algorithm, you will need to have access to `star_tracker_nix` which is a companion crate.  
//! On that crate you will have access to a set of binaries such as `demo` and `simulation` which will show the functionality of the algorithm.  
//! This can be run on a microcontroller, however that has not been setup yet so you will need to write a HAL.  
//!
//! This algorithm has thorough test harnesses as is fast.  
//! If you used this package, please hit the star :P.  
//!
//! P.S. If you are to use this in your project, you are declaring that you have thoroughly tested this code and accept the risks.  
//! I don't take any responsibility if your project breaks or anything, use at your own risk.  
//!
//! # How To Use
//! I have tested this using ubuntu and ubuntu wsl on windows and got it working.  
//! If you cloned the repository, you should have access to setup.sh, that should download all the stuff you need to run this.  
//! You may need to install other things and if your on a different platform, I have not tested it.  
//! 
//! `star_tracker_lib` does not have a way of constructing the database and you are required to use `star_tracker_nix` to create it.  
//! In `star_tracker_lib` there is `demo` look at that for an example onto how to get this working, or look below.  
//! Anyway, here is the code to run the star tracker once you have constructed the database.  
//!
//! ## STEP 1: Image Processing
//! ```
//! use star_tracker_lib::util::aliases::Decimal;
//! use star_tracker_lib::util::aliases::Byte;
//! use star_tracker_lib::util::units::Vector2;
//! use star_tracker_lib::util::units::Pixel;
//! use star_tracker_lib::util::list::List;
//! use star_tracker_lib::image_processing::BasicImage;
//! use star_tracker_lib::image_processing::Blob;
//! use star_tracker_lib::image_processing::ThresholdGrid;
//! use star_tracker_lib::image_processing::ThresholdPercent;
//!
//! // Read the image in however you want.
//! // In this case we are creating a black image.
//! // Lets just assume it is a beautiful star scape...
//! const img_width:  usize = 808;
//! const img_height: usize = 608;
//! let mut img: BasicImage<img_width, img_height> = BasicImage::new();
//! 
//! // Nilback or Sauvola thresholding.
//! // This threshold is a set of grid cells which all have their own threshold for the local area.
//! // In this case it is 250 (50x50) cells spanning ~12 pixels wide and tall.
//! const grid_size: usize = 50;
//! let overshoot  : Byte  = 50; // How much over the mean should be considered the cutoff.
//! let skip       : usize = 1;  // Only samples every second pixel. This makes the threshold 4 times faster. 
//! let thresh_grid: ThresholdGrid<grid_size, grid_size> = ThresholdGrid::new(&img, overshoot, skip);
//!
//! 
//! // This is a global threshold where you choose a percentage of pixels to cut off.
//! // The image generates a histogram and anything under 99.99% is considered background.
//! // This is less effective as it does not consider bloom.
//! let percent       : Decimal = 0.9999;
//! let thresh_percent: ThresholdPercent = ThresholdPercent::new(&img, percent);
//!
//! // To view the effects of the threshold, you can use:
//! // thresh.apply(&mut image);     // To remove the background.
//! // thresh.apply_bin(&mut image); // To make the image binary.
//! // This is not nessisary however and will slow the program down...
//!
//!
//! // Now that we have a threshold, we need to find the stars (blobs).
//! // The implemented blob detection algorithm is the grass fire method.
//! // This will delete the image so make a copy if you need.
//! //
//! // Since this is an embedded project, you need to provide a *stack* for it to store all the neighboring pixels.
//! // If you are using a computer, provide a Vec, otherwise if you have a limited size, use an arraylist.
//! // The size of the array list determines roughly how big the blob is.
//! // You may even want to use this if you are using a PC as it limits the blob size.
//! // 
//! // There is also a *min_size*, this specifies the min size the blob can be.
//! // The bigger the star, the more accurate it is as a centroid between the pixels can be calculated.
//! // Also single pixels can be hot pixels.
//! // When searching, this method will skip each min_size pixels to speed up the program.
//! // It is recommended to set *min_size* to 2.
//! 
//!	let mut stack : Vec<Pixel> = Vec::new(); // Infinite sized blobs, use array list for finite size.
//! let mut blobs : Vec<Blob>  = Vec::new(); // The output.
//! let blob_min_size = 2;                   // Blobs must be at least 2 pixels big.
//!
//! Blob::find_blobs(blob_min_size, &thresh_grid, &mut img, &mut stack, &mut blobs);
//!	blobs.sort_order(Blob::sort_descending_intensity); // Sort by intensity and/or size for the biggest stars first.
//!
//! // to convert this into a useful format, just do this:
//! let mut stars_2d : Vec<Vector2> = Vec::new();
//! Blob::to_vector2(&blobs, &mut stars_2d);
//! 
//! // You can feed the stars_2d into the next step.
//! // This is the exact sub-pixel position of the stars relative to the image sensor. 
//! ```
//!
//! ## STEP 2: Projection
//! ```
//! use star_tracker_lib::projection::IntrinsicParameters;
//! use star_tracker_lib::projection::ExtrinsicParameters;
//! use star_tracker_lib::projection::SpaceImage;
//! use star_tracker_lib::projection::SpaceCamera;
//! use star_tracker_lib::projection::SpaceWorld;
//!
//! use star_tracker_lib::util::units::Degrees;
//! use star_tracker_lib::util::units::Vector2;
//! use star_tracker_lib::util::units::Vector3;
//! use star_tracker_lib::util::units::Equatorial;
//! use star_tracker_lib::util::aliases::Decimal;
//!
//!
//! let fov = Degrees(20.0).to_radians();
//! let img_width: Decimal  = 100.0;
//! let img_height: Decimal = 100.0;
//!
//! // If you know the field of view, just use the pixel size as the sensor size...
//!	let sensor_horizontal = (img_width.powf(2.0) + img_height.powf(2.0)).sqrt();
//! let img_center = Vector2{x: img_width / 2.0, y: img_height / 2.0};
//!
//! // Creates the intrinsic parameters from the field of view.
//!	let intrinsic_projection = IntrinsicParameters::from_fov(fov, sensor_horizontal, img_center);
//!
//! // Creates the extrinsic parameters.
//! // Using *look_at* sets a point which is forward and a direction which is up.
//! // With this, you can specify what orientation the camera is relative to the spacecraft.
//!	let reference_forward = Vector3{x: 0.0, y: 0.0, z: 1.0}.to_equatorial();
//! let reference_up      = Vector3{x: 0.0, y: 1.0, z: 0.0}.to_equatorial();
//! let extrinsic_projection = ExtrinsicParameters::look_at(reference_forward, reference_up)
//! 	.expect("Ensure extrinsic projection up and forward are not the same value.");
//!
//! // Use a for loop here where you convert all of the stars_2d into 3d points.
//! // The points should be output as a vec or array list.
//! let point       : SpaceImage  = SpaceImage(Vector2{x: img_width / 2.0, y: img_height / 2.0}); // Center of image.
//! let camera_space: SpaceCamera = intrinsic_projection.from_image(point);                      // 3d local coordinates.
//! let world_space : SpaceWorld  = extrinsic_projection.from_image(camera_space);              // 3d satellite coords.
//! 
//!
//! ```
//! 
//! ## STEP 3: Tracking Mode
//! This requires the database code in *star_tracker_nix* to construct the database.
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
//! // Pretend these are image values which have been projected onto a unit sphere from the previous step.
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
//! let success = Constellation::find_all (
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
//!
//! // In a success do a loop and convert all the matches in output into vector 3 positions.
//! // output.input: the index of the observed stars that you projected from previously.
//! // output.output: the index of the stars in the database.
//! // Weight: just copy that across.
//! ```
//!
//! ## STEP 4: Attitude Determination
//! ```
//! use star_tracker_lib::attitude_determination::AttitudeDetermination;
//! use star_tracker_lib::attitude_determination::Quest;
//! use star_tracker_lib::util::units::Quaternion;
//! use star_tracker_lib::util::units::Vector3;
//! use star_tracker_lib::util::units::Match;
//!
//! // If you setup projection correctly, the points will be relative to the front of the spacecraft.
//! // In this situation, I decided that was z: +1.
//! let reference_forward = Vector3{x: 0.0, y: 0.0, z: 1.0};
//!
//! // Lets just assume these points are from a valid constellation from tracking_mode.
//! let found_stars = vec!
//! [
//!     // The weight is relative to the other points, the higher the weight the more reliable.
//! 	Match{input: Vector3{x: 0.0, y: 0.0, z: 1.0}, output: Vector3{x: 0.0, y: 0.0, z: 1.0}, weight: 1.0},
//! 	Match{input: Vector3{x: 0.0, y: 1.0, z: 0.0}, output: Vector3{x: 0.0, y: 1.0, z: 0.0}, weight: 1.0},
//! 	Match{input: Vector3{x: 1.0, y: 0.0, z: 0.0}, output: Vector3{x: 1.0, y: 0.0, z: 0.0}, weight: 1.0},
//! 	Match{input: Vector3{x: 0.0, y: 1.0, z: 1.0}, output: Vector3{x: 0.0, y: 1.0, z: 1.0}, weight: 1.0},
//! ];
//!
//!
//! let rotate_to_cam  : Quaternion = Quest::estimate(&found_stars, None); // Quaternion to rotate world space to camera space.
//!	let rotate_to_world: Quaternion = rotate_to_cam.conjugate();           // Quaternion to rotate camera space to world space.
//!	let world_center = rotate_to_world.rotate_point(reference_forward);    // Rotate spacecraft front to world coordinates
//! ```
//! 
//! All code in this library has been written Tom Creusot.  
	
	#![cfg_attr(any(test, feature = "nix"),  allow(unused_imports))] // Stops maths import error.
	#![feature(no_coverage)]                        // If a file should not be tested.
	#![feature(let_chains)]                         // Allows a while let loop better (constellation).
	#![feature(int_roundings)]                      // Allows div_ceil
	#![feature(generic_const_exprs)]                // Newer version of associated_type_defaults.
	#![feature(const_fn_floating_point_arithmetic)] // allows constant Degrees.as_radians() function.
	#![cfg_attr(not(any(test, feature = "nix")), no_std)]       // When not test, there must be no_std.
	
	#[cfg(all(feature = "bit_32", feature = "bit_64"))]
	compile_error!("To specify the size of a float, compile with either the feature bit_32 or bit_64 enabled.\nEnsure only one is enabled.");
	#[cfg(not(any(feature = "bit_32", feature = "bit_64")))]
	compile_error!("To specify the size of a float, compile with either the feature bit_32 or bit_64 enabled.\nEnsure only one is enabled.");
	
	
	extern crate libm;
	
	#[cfg(any(test, feature = "nix"))] 
	#[macro_use]
	extern crate core;    // std alternative.
	
	
	#[cfg(any(test, feature = "nix"))]  extern crate mockall; // Test Mocks.
	#[cfg(any(test, feature = "nix"))]  extern crate rand;    // Testing randomness.
	
	#[allow(dead_code)] pub mod util;
	#[allow(dead_code)] pub mod image_processing;
#[allow(dead_code)] pub mod attitude_determination;
#[allow(dead_code)] pub mod tracking_mode;
#[allow(dead_code)] pub mod projection;

/// `core_include` is All the no_std packages.  
///
/// This module is found in *lib.rs*.  
/// Since I am using no_std, some things don't carry over natively so this module automatically includes them.
pub mod core_include
{
	pub use core::result::Result;
	pub use core::result::Result::Ok;
	pub use core::result::Result::Err;

	pub use core::option::Option;
	pub use core::option::Option::Some;
	pub use core::option::Option::None;

	pub use core::clone::Clone;
	pub use core::marker::Copy;
	pub use core::marker::Sized;

	pub use core::cmp::PartialEq;
	pub use core::cmp::PartialOrd;
	pub use core::cmp::Eq;

	pub use core::ops::Range;
	pub use core::ops::RangeInclusive;

	pub use core::prelude::v1::derive;
	pub use core::fmt::Debug;
	
	
	pub use crate::util::Maths;
	
}