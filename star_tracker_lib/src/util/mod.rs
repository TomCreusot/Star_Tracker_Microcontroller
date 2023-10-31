//! `util` is contains any tools which may be important for general use in this project.
//!    
//! Everything inside this module is expected to be used in multiple modules.  
//! 
//! This includes;
//! * Types of storage lists.
//! * Aliases of datatypes to decide precision.
//! * Coordinate systems and units that will be used.
//! * Types of errors.
//! * Methods for testing.
pub mod err;
pub mod list;
pub mod word;
pub mod units;
pub mod aliases;
pub mod linear_lookup;

// When not using std.
pub mod maths;

pub mod test;

#[cfg(any(test, feature = "nix"))] pub mod distribution;

/// When using core (no_std), basic maths functionality is removed.  
/// Use this maths library (from libm) to substitute the std maths commands when needed.  
pub use crate::util::maths::Maths as Maths;




