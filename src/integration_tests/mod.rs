//! This folder is for integration tests.
//! Any complext testing or implementations go here.
//! To run an integration test, type:
//! ```cargo run --bin integration_tests"
pub mod helper_functions;

pub mod k_vector;
pub mod image_processing;

pub fn run()
{
	println!("\n\n\n~~~~~~~~~~~~~~~~~~~~~~~~~\n K-VECTOR TEST\n~~~~~~~~~~~\n");
	k_vector::run();
	
	
	println!("\n\n\n~~~~~~~~~~~~~~~~~~~~~~~~~\n IMAGE PROCESSING TEST\n~~~~~~~~~~~\n");
	image_processing::run();
}