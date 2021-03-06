//! Nix contains any functionality which should only function on a computer.  
//! This may be due to using the heap, unnessisary code, interfacing with external crates, etc.  
//! 
//!
//!
//!

use crate::util::{aliases::Decimal, coordinates::Equatorial};

pub mod image;
pub mod star;

/// Static class containing communication between the external crate `image` and `image_processing/image`.
pub struct NixImage;


/// A star position and its magnitude.
pub struct Star
{
	pub position: Equatorial,
	pub magnitude: Decimal,
}
