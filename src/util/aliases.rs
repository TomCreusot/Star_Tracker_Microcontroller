//! This module contains any alias variables to make more sense for the project.  
//! Change these for the circumstance of the project.  
//! For a 8 bit microcontroller dont use anything higher than 8 bits.  
//! For a 32 bit microcontroller dont use anything higher than 32 bits.    
//! For a 64 bit machine use the precision of the machine.  



/// All decimal numbers will use this precision.
pub type Decimal = f32;


/// Returns the machine precision of Decimal (For K-Vector equation).
pub fn decimal_precision ( ) -> f64
{
	// Multiply by a few digits as adding a number may remove this.
	return 1.19E-07 * 3.0; // f32
	// return 2.22E-16; // f64
}


use std::f32::consts::PI;
// use std::f64::consts::PI;
/// PI to the correct precision.
pub const M_PI : Decimal = PI;





/// All unsigned integers will use this precision.
pub type UInt = u32;

/// The maximum intensity of a pixel.
pub type Byte = u8;

/// All signed integers will use this precision.
pub type Int = i32;
