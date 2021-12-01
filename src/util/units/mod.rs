/// Rust gives the ability to use struct tuples as units.
/// Using units will provide safety from inputting an incorrect value.
/// Try to use these where possible.
pub mod unit_ops;
pub mod angles;
pub mod cartesian3d;
pub mod quaternion;
pub mod equatorial;
pub mod matrix;
use serde::Deserialize;

use super::aliases::Decimal;


//###############################################################################################//
//										---	Angles ---
//###############################################################################################//
/// An angle in degrees.
#[derive(Debug, Copy, Clone)]//, PartialEq)]
pub struct Degrees ( pub Decimal );

/// An angle in radians.
#[derive(Debug, Copy, Clone)]//, PartialEq)]
#[derive(Deserialize)]
pub struct Radians ( pub Decimal );


/// An angle in 24 hours.
#[derive(Debug, Copy, Clone)]//, PartialEq)]
pub struct Hours ( pub Decimal );



//###############################################################################################//
//										---	Pixel ---
//###############################################################################################//

/// A position of a pixel.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pixel
{
	pub x: usize,
	pub y: usize,
}

/// A coordinate in 2d space not fixed to a grid.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PixelWeighted
{
	pub x: Decimal,
	pub y: Decimal,
}
//###############################################################################################//
//										---	Cartesian3D ---
//###############################################################################################//
/// A 3d point.
/// This is to represent equatorial coordinates for doing angle calculations.
#[derive(Debug, Copy, Clone)]
pub struct Cartesian3D
{
	pub x : Decimal,
	pub y : Decimal,
	pub z : Decimal,
}

//###############################################################################################//
//										---	Equatorial ---
//###############################################################################################//
/// Equatorial Coordinates are coordinates which define a direction along a sphere.
/// Right Ascension (ra) is defined as the angle around the equator from 0 to 2PI (Don't use 0h to 24h).
/// Declination (dec) is defined as the angle from -PI to PI (Don't use -90 to 90 deg).
#[derive(Debug, Copy, Clone)]
#[derive(Deserialize)]
pub struct Equatorial
{
	/// Right Ascention (along the equator).
	pub ra : Radians,
	/// Declination (from pole to pole).
	pub dec : Radians,
}


//###############################################################################################//
//										---	Quaternion ---
//###############################################################################################//
/// Represents a 3D rotation without singularity.
/// This method uses the LEFT HAND COORDINATE SYSTEM.
#[derive(Debug, Copy, Clone)]
pub struct Quaternion
{
	pub w : Decimal,
	pub x : Decimal,
	pub y : Decimal,
	pub z : Decimal,
}




//###############################################################################################//
//										---	Matrix ---
//###############################################################################################//
/// An n x m matrix.
/// W is the width.
/// H is the height.
#[derive(Copy, Clone)]
pub struct Matrix <const ROW : usize, const COLUMN : usize>//<const 'a, size : MatSize<'a>>//<const W : usize, const H : usize>
{
	/// The matrix ArrayList
	matrix : [[Decimal; COLUMN]; ROW],
	// matrix : [[Decimal; W]; H],
}



#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MatPos
{
	pub row: usize,
	pub col: usize,
}
