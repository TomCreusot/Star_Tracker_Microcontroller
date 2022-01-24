//!
//!
//!
//!
#![feature(const_generics)]					// Allows constant values in generics.
// #![feature(adt_const_params)]				// Replaces const_generics.
// #![feature(generic_const_expr)]				// Replaces const_generics.

#![feature(const_evaluatable_checked)]		// Allows generic struct variables.
#![feature(associated_type_defaults)]		// Allows list to implement iterator.

// #![allow(unused_must_use)]

extern crate curl;
extern crate csv;
extern crate serde;
extern crate image;
extern crate rand;
extern crate mockall;




#[allow(dead_code)]
// #[feature(const_evaluatable_checked)]
pub mod util;
#[allow(dead_code)]
pub mod image_processing;
#[allow(dead_code)]
pub mod attitude_determination;

#[allow(dead_code)]
pub mod nix;

#[allow(dead_code)]
// #[feature(const_generics)]
// #[feature(const_evaluatable_checked)]
pub mod tracking_mode;

#[allow(dead_code)]
// #[feature(associated_consts)]
pub mod config;



