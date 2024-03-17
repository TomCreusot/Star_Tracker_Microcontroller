//! This module contains any alias variables to make more sense for the project.  
//!
//! To change the architecture of the cpu between 32 and 64 bit, use the features flag:
//! * `bit_32` - Uses 32 bit float and 32 bit int/uint
//! * `bit_64` - Uses 64 bit float and 64 bit int/uint

/// All decimal numbers will use this precision.
#[cfg(feature = "bit_32")] use core::f32::consts::PI;
/// All decimal numbers will use this precision.
#[cfg(feature = "bit_32")] pub type Decimal = f32;
/// All decimal numbers will use this precision.
#[cfg(feature = "bit_64")] use core::f64::consts::PI;
/// All decimal numbers will use this precision.
#[cfg(feature = "bit_64")] pub type Decimal = f64;

/// All unsigned integers will use this precision.
#[cfg(feature = "bit_32")] pub type UInt = u32;
/// All unsigned integers will use this precision.
#[cfg(feature = "bit_64")] pub type UInt = u64;

/// All signed integers will use this precision.
#[cfg(feature = "bit_32")] pub type Int = i32;
/// All signed integers will use this precision.
#[cfg(feature = "bit_64")] pub type Int = i64;

/// The maximum intensity of a pixel.
pub type Byte = u8;



/// The machine precision of Decimal.  
#[cfg(feature = "bit_64")] pub const DECIMAL_PRECISION: Decimal = 2.22E-16 * 10.0;
/// The machine precision of Decimal.  
#[cfg(feature = "bit_32")] pub const DECIMAL_PRECISION: Decimal = 1.0E-7 * 10.0;


/// PI with the precision of `DECIMAL_PRECISION`.  
/// Use this to avoid error messages for incorrect precision.  
pub const M_PI: Decimal = PI;





