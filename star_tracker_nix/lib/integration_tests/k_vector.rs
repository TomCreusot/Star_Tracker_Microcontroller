//! # K-Vector Test
//! This is an integration test for the K Vector.
//! Read this for a step by step guide into the K Vector implementation for the following files:
//! - k_vector
//! - star_database_element
//!
//! # Summary
//! K_Vector is described in [database/mod.rs](src/tracking_mode/database/mod.rs).
//! The database is separated into 3 sections:
//! * K-Vector
//! * StarPairs
//! * Catalogue
//!
//! The `K-Vector` is an equation which points to a set of `bins`.
//! The `Bins` are pointers to the `StarPairs` database and will.
//! This allows instant searches of pairs of stars with the specified angular distance.
//! 
//! The `StarPairs` are a pair of references to the Catalogue.
//! The reason for using this is to optimise the pyramid algorithm.
//!
//! The `Catalogue` stores the equatorial positions of each star.
//!
//!
//! # This test will:
//! - Generate a K-Vector Database (catalogue, star pairs, k-vector, k-vector equation).
//! - A random angle will be requested to be found.
//! - The K-Vector will search for simmilar angles.
//! - The angles will be compared to ensure they are close to the given range.



use star_tracker_lib::tracking_mode::database::PyramidDatabase;
use star_tracker_lib::tracking_mode::database::SearchResult;
use star_tracker_lib::tracking_mode::database::KVector;

use star_tracker_lib::tracking_mode::StarPair;

use star_tracker_lib::util::units::Equatorial;
use star_tracker_lib::util::units::Radians;
use star_tracker_lib::util::units::Degrees;
use star_tracker_lib::util::list::List;

use crate::integration_tests::helper_functions::*;
use crate::tracking_mode::StarDatabaseElement;
use crate::io::Star;

static mut K_VECTOR_BINS : Vec<usize>           = Vec::new();
static mut CATALOGUE     : Vec<Equatorial>      = Vec::new();
static mut STAR_PAIRS    : Vec<StarPair<usize>> = Vec::new();

pub fn run ( )
{
	let num_stars = 100;
	let tolerance = Radians(0.01); // The tolerance determines the number of bins.
	
	// used in the construction of the database.
	let gen_catalogue  : Vec<Star>                = generate_catalogue(num_stars);
	let gen_star_pairs : Vec<StarDatabaseElement> = generate_star_pair(&gen_catalogue);

	let database : PyramidDatabase;
	let k_vector : KVector = create_k_vector(num_bins, &gen_star_pairs);
	// Statics should be constant so this is unsafe.
	unsafe
	{
		K_VECTOR_BINS  = create_bins(k_vector, &gen_star_pairs);
		CATALOGUE      = Star::to_equatorial(&gen_catalogue);
		STAR_PAIRS     = StarDatabaseElement::to_star_pairs(&gen_star_pairs);

		database = PyramidDatabase {
			fov:       Degrees(360.0).to_radians(),
			k_lookup:  k_vector,
			k_vector:  &K_VECTOR_BINS,
			pairs:     &STAR_PAIRS,
			catalogue: &CATALOGUE,
		};
	}

	// Testing is done by generating a set of random angle values.
	// The values are then searched through a varaiety of methods.
	// All methods must succeed.
	let number_tests = 1000;
	
	let mut elapsed_basic    = std::time::Duration::ZERO;
	let mut elapsed_k_vector = std::time::Duration::ZERO;
	for _ in 0..number_tests
	{
		// Generate a random angle and tolerance to search for.
		let angle       : Radians = gen_angle_distance(k_vector.min_value, k_vector.max_value);
		let tolerance   : Radians = gen_tolerance(Radians(0.01));
		
		
		// By searching each element in the database, this will eventualy find every element.
		// It is good to test against as it will be correct.
		let mut timer = std::time::Instant::now();
		let basic_angles          : Vec<StarPair<usize>> 
										= find_basic_angles(angle, tolerance, &database);
										// = find_basic_angles(k_vector.max_value, tolerance, &database);
										// = find_basic_angles(k_vector.min_value, tolerance, &database);
		let time_basic_angles = timer.elapsed();
		
		// This is the correct method where an advanced method is used to trim the excess stars.
		timer = std::time::Instant::now();
		let k_vector_angles       : Vec<SearchResult> 
										= find_k_vector_angles(angle, tolerance, &database);
										// = find_k_vector_angles(k_vector.max_value, tolerance, &database);
										// = find_k_vector_angles(k_vector.min_value, tolerance, &database);
		let time_k_vector_angles = timer.elapsed();
						
						
		elapsed_basic += time_basic_angles;
		elapsed_k_vector += time_k_vector_angles;
																	
		for ii in 0..basic_angles.size()
		{
			let mut contains = false;
			for jj in 0..k_vector_angles.size()
			{
				contains |= StarPair::are_same(&basic_angles[ii], &k_vector_angles[jj].result);
			}
			assert!(contains, "k_vector_angles is missing some valid StarPairs.");
		}
		for jj in 0..k_vector_angles.size()
		{
			let mut contains = false;
			for ii in 0..basic_angles.size()
			{
				contains |= StarPair::are_same(&basic_angles[jj], &k_vector_angles[ii].result);
			}
			assert!(contains, "k_vector_angles has extra invalid StarPairs.");
		}
	}
	
	elapsed_basic /= number_tests;
	elapsed_k_vector /= number_tests;
	println!("time basic search method:    {:?}\ntime k_vector search method: {:?}", 
		elapsed_basic, elapsed_k_vector);
	
}
