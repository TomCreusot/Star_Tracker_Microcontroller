//! The integration_tests binary is for running all the integration_tests setup in the star_tracker_nix library.  
//! To run, use cargo run --bin integration_tests.

#![allow(unused_must_use)]
extern crate star_tracker_lib;
extern crate star_tracker_nix;
extern crate image;
extern crate rand;

fn main ( )
{
	star_tracker_nix::integration_tests::run();
}