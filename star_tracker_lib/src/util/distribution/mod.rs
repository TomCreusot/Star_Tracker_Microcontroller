//! This module is used to distribute points evenly over a sphere + some stats.  
//!
//! This was created to help with simulated tests and generating a region based database.     
//! There is also a stats Distribution struct because why not.
//! 
//! ```
//! use star_tracker_lib::util::units::Degrees;
//! use star_tracker_lib::util::units::Radians;
//! use star_tracker_lib::util::units::Equatorial;
//! use star_tracker_lib::util::distribution::Distribute;
//! use star_tracker_lib::util::distribution::Distribution;
//!
//! // To generate (roughly) equally spaced points on the unit sphere separated by 10 degrees.
//! let angle:        Radians         = Degrees(5.0).as_radians();                 // Separation angle.
//! let num_points:   usize           = Distribute::angle_to_points(angle);         // Calculates the number of points to suit angle.
//! let points:       Vec<Equatorial> = Distribute::fibonacci_lattice (num_points); // Generates *num_points* equally spaced on unit sphere.
//!
//! // Check how separated the points are:
//! let separation: Distribution =  Distribute::separation(&points);
//! assert!((separation.avg - angle).abs() < 0.01);
//! assert!((separation.max - angle).abs() < 0.01);
//! assert!((separation.min - angle).abs() < 0.1); // The poles are not that equal...
//! assert!((separation.dev - angle).abs() < 0.1);
//!
//! // Check how separated the points will be from any location on the unit sphere:
//! let coverage: Distribution = Distribute::coverage(&points);
//! assert!((coverage.dev - angle / 2.0).abs() < 0.1);
//!
//!
//! // This also exists if you need it:
//! let angle_from_points: Radians = Distribute::points_to_angle(num_points);
//! assert!((angle - angle_from_points).abs() < 0.01);
//! ```
use crate::util::units::Radians;

pub mod distribute;

/// Distributes a set of points onto a sphere.
pub struct Distribute ( );


/// Just a useful stat struct.
pub struct Distribution
{
	pub avg: Radians,
	pub max: Radians,
	pub min: Radians,
	pub dev: Radians,
}

