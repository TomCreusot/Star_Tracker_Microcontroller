//! Coordinates manages the different types of coordinate methods;
//! ``` ignore
//! Cartesian2D<T>
//!
//! Cartesian3D<T>
//!
//! Equatorial
//! 	get_ra_hour           ( ) -> Decimal             // Gets right ascension in hour format.
//! 	get_dec_degrees       ( ) -> Decimal             // Gets declination in degrees.
//! 	set_ra_hour           ( Decimal )                // Sets right ascension from hour format.
//! 	set_dec_degrees       ( Decimal )                // Sets declination from degrees.
//! 	get_phi               ( ) -> Decimal             // Gets the declination starting from z = +1 as 0 degrees (spherical coordinates).
//! 	great_circle_distance ( Equatorial ) -> Decimal  // Finds the curved distance along a unit sphere.
//! 	planar_distance       ( Equatorial ) -> Decimal  // Finds the distance between the 2 cartesian points on a unit sphere.
//!
//!
//! Quaternion
//!
use crate::util::aliases::Decimal;

pub mod equatorial;
pub mod quaternion;

//###############################################################################################//
//										---	Cartesian2D ---
//###############################################################################################//
/// Linear Cartesian Coordinates.  
/// These can be used for any flat plane including images.  
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Cartesian2D <T>
{
	pub x : T,
	pub y : T,
}

//###############################################################################################//
//										---	Cartesian3D ---
//###############################################################################################//
/// A 3d point.  
/// This is to represent equatorial coordinates for doing angle calculations.  
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Cartesian3D <T>
{
	pub x : T,
	pub y : T,
	pub z : T,
}

//###############################################################################################//
//										---	Equatorial ---
//###############################################################################################//
/// Equatorial Coordinates are coordinates which define a direction along a sphere.  
/// Right Ascension (ra) is defined as the angle around the equator from 0 to 2PI (Don't use 0h to 24h).  
/// Declination (dec) is defined as the angle from the equator north or south from 0 to PI (Don't use -90 to 90 deg).  
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Equatorial
{
	/// Right Ascention (along the equator).
	pub ra : Decimal,
	/// Declination (from pole to pole).
	pub dec : Decimal,
}


//###############################################################################################//
//										---	Quaternion ---
//###############################################################################################//
/// Represents a 3D rotation without singularity.
/// This method uses the LEFT HAND COORDINATE SYSTEM.
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Quaternion
{
	pub w : Decimal, 
	pub x : Decimal, 
	pub y : Decimal, 
	pub z : Decimal,
}
