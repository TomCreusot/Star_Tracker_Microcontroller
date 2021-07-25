//! Nix contains any functionality which should only function on a computer.
//! This may be due to using the heap, unnessisary code, interfacing with external crates, etc.
//!
//!
//!
//!
use image::RgbImage;
use crate::util::{aliases::Decimal, units::Equatorial};

pub mod nix_image;
pub mod template;
pub mod star;
pub mod io;

/// File management (Excluding Images).
pub struct Io ( );

/// Wrapper of trait `image_processing::Image` for `extern crate RGBImage`.
pub struct NixImage
{
	pub img_rgb: RgbImage,
}


/// A star position and its magnitude.
// #[derive(Clone)]
// pub struct Star
// {
// 	pub position: Equatorial,
// 	pub magnitude: Decimal,
// }


/*
// #[derive(Deserialize, Debug)]
#[derive(Debug)]
/// Used by extern csv library.
pub struct Star
{
	pub mag  : Decimal,
	pub pos  : Equatorial,
	pub spec : String,
}

*/
#[derive(Debug, /*Deserialize,*/ Clone)]
/// Use for CSV serialization 
pub struct Star
{
	// #[serde(rename = "mag")]
	pub mag  : Decimal,
	// #[serde(flatten)]
	pub pos  : Equatorial,
	// #[serde(rename = "spect")]
	pub spec : String 
}



/// A template file 
pub struct Template
{
	// The values to replace.
	keys   : Vec<String>,
	// The values to replace the keys by. 
	values : Vec<String>,
}