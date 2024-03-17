//! # Database  
//! This library manages the creation of the k-vector database.  
//! Contained is the required functionality to create the database.  
//!
//! # Example
//! ```
//! extern crate star_tracker_lib;
//! use star_tracker_lib::util::units::Degrees;
//! use star_tracker_lib::util::units::Radians;
//!
//! use star_tracker_database::tracking_mode::DatabaseGenerator; 
//! use star_tracker_database::io::Star; 
//! use star_tracker_database::io::Io; 
//!
//! let fov                   = Degrees(20.0).to_radians(); // Diagonal Field Of View of Sensor.
//! let angle_tolerance       = Degrees(0.03).to_radians();
//! 
//! // Reduce the number of stars by their magnitude, they should not be too dull.
//! let magnitude_min         = -20.00; // Excludes sun
//! let magnitude_max         = DatabaseGenerator::recommended_magnitude(fov);
//! 
//! // Reduce the number of stars by removing stars which are too close to each other.
//! let double_star_tolerance = angle_tolerance * 2.0;
//! 
//! // Reduce the number of stars by how clustered their neighborhood is.
//! let region_num = 8;     // Max 8 stars in each region or neighborhood (works well).
//!     // The region or neighborhood size should be smaller than the region size incase the brightest stars are clustered.
//!     // Having the region size too big could leave holes in the sky coverage.
//! let region_size = fov / 2.0; 
//! 
//!
//! // Reduce the number of star pairs by limiting the maximum angle that can be formed.
//! // A star pair taking up the whole area of the lens will encounter distortion.
//! let max_angle = fov / 1.2; // Angles can be 80% of the lens field of view.
//! 
//! // 1. Reading the database.
//! //    The database is automatically downloaded and read from using the following commands.
//! let mut stars : Vec<Star> = Io::get_csv_database();
//! 
//! // 2. Sorting the database.
//! //    To speed up the search process, the stars MUST be sorted in order of magnitude.
//! stars.sort();
//! 
//! // 3. Star reduction.
//! //    Certain stars are not needed when forming the database.
//! //    Stars that are too dull are not necessary.
//! //    Stars next to other stars are not necessary.
//! //    If there are too many stars in a single part of the sky, some should be removed.
//! let stars_limit_mag    = DatabaseGenerator::limit_magnitude (&stars, magnitude_min, magnitude_max);
//! let stars_limit_double = DatabaseGenerator::limit_double_stars(&stars_limit_mag, double_star_tolerance);
//! let stars_limit_reg    = DatabaseGenerator::limit_regions(&stars_limit_double, region_size, region_num);
//! 
//! // 4. Create the database.
//! //    This creates the database.
//! let gen : DatabaseGenerator = DatabaseGenerator::gen_database(&stars_limit_reg, fov, max_angle, angle_tolerance);
//! let database = gen.get_database();
//!
//! // Put the database inside a chunk iterator and it can be used for the star tracker.
//! ```




#![feature(path_file_prefix)]

extern crate star_tracker_lib;
extern crate serde_json;
extern crate serde;
extern crate curl;
extern crate rand;

pub mod tracking_mode;
pub mod io;


pub mod distribution;