//! This module contains any alias variables to make more sense for the project.  
//! Change these for the circumstance of the project.  
//! For a 8 bit microcontroller dont use anything higher than 8 bits.  
//! For a 32 bit microcontroller dont use anything higher than 32 bits.    
//! For a 64 bit machine use the precision of the machine.  



/*
/// All decimal numbers will use this precision.
pub type Decimal = f32;
use std::f32::consts::PI;
/// The machine precision of Decimal.
pub const DECIMAL_PRECISION : 		Decimal = 1.19E-07 * 3.0; // f32
// */


//*
/// All decimal numbers will use this precision.
pub type Decimal = f64;
use std::f64::consts::PI;
/// PI to the correct precision.
/// The machine precision of Decimal.
pub const DECIMAL_PRECISION : 		Decimal = 2.22E-16 * 10.0; // f64
// */


pub const M_PI : Decimal = PI;





/// All unsigned integers will use this precision.
pub type UInt = u32;

/// The maximum intensity of a pixel.
pub type Byte = u8;

/// All signed integers will use this precision.
pub type Int = i32;
