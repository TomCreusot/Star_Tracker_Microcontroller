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

use rand::prelude::*;

use crate::tracking_mode::database::StarDatabaseElement;
use crate::tracking_mode::database::PyramidDatabase;
use crate::tracking_mode::database::Database;
use crate::tracking_mode::database::KVector;

use crate::tracking_mode::StarPair;

use crate::util::units::Equatorial;
use crate::util::aliases::Decimal;
use crate::util::units::Radians;
use crate::util::units::Degrees;
use crate::util::list::List;

use crate::nix::Star;

static mut K_VECTOR_BINS : Vec<usize>           = Vec::new();
static mut CATALOGUE     : Vec<Equatorial>      = Vec::new();
static mut STAR_PAIRS    : Vec<StarPair<usize>> = Vec::new();

pub fn run ( )
{
	println!("\n\n\n~~~~~~~~~~~~~~~~~~~~~~~~~\n K-VECTOR TEST\n~~~~~~~~~~~\n");
	
	let num_stars = 100;
	let num_bins = 1000; // If the bins are too small, there is a massive performance hit.
	
	// used in the construction of the database.
	let gen_catalogue  : Vec<Star>                = generate_catalogue(num_stars);
	let gen_star_pairs : Vec<StarDatabaseElement> = generate_star_pair(&gen_catalogue);


	let k_vector       : KVector              = create_k_vector(num_bins, &gen_star_pairs);

	let database : PyramidDatabase;
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
	for _i in 0..number_tests
	{
		// Generate a random angle and tolerance to search for.
		let angle       : Radians = gen_angle_distance(k_vector.min_value, k_vector.max_value);
		let tolerance   : Radians = gen_tolerance(Radians(0.01));
		let mut timer = std::time::Instant::now();
		
		// By searching each element in the database, this will eventualy find every element.
		// It is good to test against as it will be correct.
		timer = std::time::Instant::now();
		let basic_angles          : Vec<StarPair<usize>> 
										= find_basic_angles(angle, tolerance, &database);
										// = find_basic_angles(k_vector.max_value, tolerance, &database);
										// = find_basic_angles(k_vector.min_value, tolerance, &database);
		let time_basic_angles = timer.elapsed();
		
		// This is the correct method where an advanced method is used to trim the excess stars.
		timer = std::time::Instant::now();
		let k_vector_angles       : Vec<StarPair<usize>> 
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
				contains |= StarPair::are_same(&basic_angles[ii], &k_vector_angles[jj]);
			}
			assert!(contains, "k_vector_angles is missing some valid StarPairs.");
		}
		for jj in 0..k_vector_angles.size()
		{
			let mut contains = false;
			for ii in 0..basic_angles.size()
			{
				contains |= StarPair::are_same(&basic_angles[jj], &k_vector_angles[ii]);
			}
			assert!(contains, "k_vector_angles has extra invalid StarPairs.");
		}
	}
	
	elapsed_basic /= number_tests;
	elapsed_k_vector /= number_tests;
	println!("time basic search method:    {:?}\ntime k_vector search method: {:?}", 
		elapsed_basic, elapsed_k_vector);
	
}



//###############################################################################################//
//###############################################################################################//
//###############################################################################################//
//
//										Database Construction
//
//###############################################################################################//
//###############################################################################################//
//###############################################################################################//

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Used to generate a star catalogue (position of stars).
/// This is done by randomly generating a set of stars within the equatorial sphere.
///
/// The actual catalogue uses Equatorial instead of Star.
///
/// # Arguments
/// * `num_elements` - The size of the catalogue to generate.
///
/// # Returns
/// A catalogue of stars in the nix format.
/// Most of the variables are unused and the actual database is an array of Equatorial coordinates.
fn generate_catalogue ( num_elements: usize ) -> Vec<Star>
{
	let mut rng = rand::thread_rng();

	// The database of stars.
	let mut cat : Vec<Star> = Vec::new();


	// Randomly generate 100 stars on the equatorial sphere.
	for _i in 0..num_elements
	{
		let element = Equatorial{
			ra:  Degrees(rng.gen::<Decimal>() * 360.0).to_radians(),
			dec: Degrees(rng.gen::<Decimal>() * 180.0 - 90.0).to_radians()};

		let star = Star{mag: 0.0, spec: "".to_string(), pos: element};
		cat.push_back(star).expect("Vecs should not be full");
	}	
	return cat;
}



// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Generates the star pairs.
/// This is a set of combinations of stars which are within the field of view.
/// The star pairs are relevent to the Pyramid tracking mode.
///
/// In this case 360 degrees will be used as the field of view is not being tested in this test.
///
/// # Arguments
/// * `cat` - The star catalogue of stars on the celestial sphere, this is usualy generated from a csv star database.
///
/// # Returns
/// A star database element which is used to generate a K_Vector.
/// The actual star pair database is using the database StarPair.
fn generate_star_pair ( cat: &Vec<Star> ) -> Vec<StarDatabaseElement>
{
	let field_of_view = Degrees(360.0).to_radians();
	let mut pairs = StarDatabaseElement::create_list(field_of_view, cat);
	pairs.sort_by(StarDatabaseElement::cmp);
	return pairs;
}



// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Generates the k-vector equation.
/// The k-vector equation outputs a value which represents an index of the k-vector bins.
///
/// # Arguments
/// * `num_bins` - The number of bins for the k-vector.
/// * `pairs`    - The star pairs created in `generate_star_pair`.
///
/// # Returns
/// The k-vector equation to be used to reference the bins.
fn create_k_vector ( num_bins : usize, pairs : &Vec<StarDatabaseElement> ) -> KVector
{
	let k_vector = KVector::new(num_bins, 
			pairs[0].dist.0 as Decimal, 
			pairs[pairs.len() - 1].dist.0 as Decimal);
	return k_vector;
			
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Creates the k-vector bins.
/// The k-vector equation points to a specific bin with the input of an angular distance.
/// The bin points to a star pair in the star pair database which has a similar angular distance.
///
/// # Arguments
/// * `k_vector`   - The k-vector generated in `create_k_vector`.
/// * `pairs` - The vector passed into `create_k_vector`.
///
/// # Returns
/// The k-vector bins.
fn create_bins ( k_vector: KVector, pairs: &Vec<StarDatabaseElement>  ) -> Vec<usize>
{
	return k_vector.generate_bins(pairs).expect("not enough bins available.");
}









//###############################################################################################//
//###############################################################################################//
//###############################################################################################//
//
//										Searching Database
//
//###############################################################################################//
//###############################################################################################//
//###############################################################################################//


/// Generates a random angular distance between min and max.
fn gen_angle_distance ( min: Radians, max: Radians ) -> Radians
{
	let mut rng = rand::thread_rng();
	return Radians(rng.gen::<Decimal>()) * (max - min) + min;	
}

/// Generates a tolerance value.
fn gen_tolerance ( max: Radians ) -> Radians
{
	let mut rng = rand::thread_rng();
	return Radians(rng.gen::<Decimal>()) * max;	
}




// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// This will search the database for all matches of an angular distance within a tolerance.
/// The method is really basic and slow.
///
/// The method is to loop through each element of the catalogue to find values of the correct
/// distance and to add it to a list.
/// It is the most reliable method as it is simple.
fn find_basic_angles ( angle: Radians, tolerance: Radians, database: &PyramidDatabase )
																			-> Vec<StarPair<usize>>
{
	let mut matches : Vec<StarPair<usize>> = Vec::new();
	
	for ii in 0..database.catalogue.len()
	{
		for jj in (ii+1)..database.catalogue.len()
		{
			if (database.angle_distance(StarPair(ii, jj)) - angle).abs() < tolerance.0
			{
				matches.push_back(StarPair(ii, jj)).expect("Vectors should have capacity?");
			}
		}
	}
	
	return matches;
}



fn find_k_vector_angles ( angle: Radians, tolerance: Radians, database: &PyramidDatabase )
																			-> Vec<StarPair<usize>>
{
	let mut matches : Vec<StarPair<usize>> = Vec::new();
	database.find_close_ref(angle, tolerance, &mut matches);
	return matches;
}