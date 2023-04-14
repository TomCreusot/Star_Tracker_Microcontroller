use rand::prelude::*;

use crate::tracking_mode::database::StarDatabaseElement;
use crate::tracking_mode::database::PyramidDatabase;
use crate::tracking_mode::database::SearchResult;
use crate::tracking_mode::database::Database;
use crate::tracking_mode::database::KVector;

use crate::tracking_mode::StarPair;

use crate::nix::Star;


use crate::util::aliases::Decimal;
use crate::util::units::Equatorial;
use crate::util::units::Radians;
use crate::util::units::Degrees;
use crate::util::list::List;

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
pub fn generate_catalogue ( num_elements: usize ) -> Vec<Star>
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

		let star = Star{mag: 0.0, spec: "".to_string(), pos: element, name: "".to_string()};
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
pub fn generate_star_pair ( cat: &Vec<Star> ) -> Vec<StarDatabaseElement>
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
pub fn create_k_vector ( num_bins : usize, pairs : &Vec<StarDatabaseElement> ) -> KVector
{
	let k_vector = KVector::new(num_bins,
			pairs[0].dist.0 as Decimal,
			pairs[pairs.size() - 1].dist.0 as Decimal);
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
pub fn create_bins ( k_vector: KVector, pairs: &Vec<StarDatabaseElement>  ) -> Vec<usize>
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
pub fn gen_angle_distance ( min: Radians, max: Radians ) -> Radians
{
	let mut rng = rand::thread_rng();
	return Radians(rng.gen::<Decimal>()) * (max - min) + min;
}

/// Generates a tolerance value.
pub fn gen_tolerance ( max: Radians ) -> Radians
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
pub fn find_basic_angles ( angle: Radians, tolerance: Radians, database: &PyramidDatabase )
																			-> Vec<StarPair<usize>>
{
	let mut matches : Vec<StarPair<usize>> = Vec::new();

	for ii in 0..database.catalogue.size()
	{
		for jj in (ii+1)..database.catalogue.size()
		{
			if (database.angle_distance(StarPair(ii, jj)).expect("?") - angle).abs() < tolerance.0
			{
				matches.push_back(StarPair(ii, jj)).expect("Vectors should have capacity?");
			}
		}
	}

	return matches;
}



pub fn find_k_vector_angles ( angle: Radians, tolerance: Radians, database: &PyramidDatabase )
																		-> Vec<SearchResult>
{
	let mut matches : Vec<SearchResult> = Vec::new();
	database.find_close_ref(angle, tolerance, &mut matches);
	return matches;
}
