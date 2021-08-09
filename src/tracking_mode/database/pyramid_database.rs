/// Implementation for Database
use super::Database;
use super::PyramidDatabase;

use crate::util::list::List;
// use crate::util::aliases::Decimal;
use crate::util::units::Radians;
use crate::util::units::Equatorial;

use crate::tracking_mode::StarPair;

use crate::tracking_mode::database::array_database::*;

impl PyramidDatabase
{
	/// Constructs a new database using the pregenerated databases (run database_generator).
	fn new ( ) -> Self
	{
		return Self 
		{ 
			fov: 		FOV, 
			k_lookup:	K_LOOKUP_DATABASE, 
			k_vector:	&K_VECTOR_DATABASE, 
			pairs:		&STAR_PAIR_DATABASE, 
			catalogue:	&CATALOGUE_DATABASE,
		};
	}
}
	
	
	
impl Database for PyramidDatabase
{
	/// Finds close matches to the provided angular separation and returns the star pair reference.  
	/// The an element of the star pair reference can be inserted into `find_star` to get the actual location.  
	/// # Arguments
	/// * `find` - The angular separation between the found stars to find in the database.
	/// * `found` - The closest matches to the provided `find`.
	/// # Example
	/// ```
	/// panic!("NYI");
	/// ```
	fn find_close_ref ( &self, find : Radians, found : &mut dyn List<StarPair<usize>> )
	{
		let k_range_wrapped = self.k_lookup.get_bins(find);
		if let Ok(k_range) = k_range_wrapped
		{
			let k_vect = &self.k_vector[k_range];
			
			// assume the closest element is in the center of the range.
			let mid = k_vect.len() / 2; // The middle index or middle index -1 for odds.
			
			for i in 0..mid
			{
				if !found.is_full() 
				{
					found.push_back(self.pairs[k_vect[mid + i]]);
				}
				if !found.is_full() 
				{
					found.push_back(self.pairs[k_vect[mid - i]]);
				}
			}
			if !found.is_full() && mid % 2 == 0
			{
				found.push_back(self.pairs[k_vect[k_vect.len() - 1]]);
			}
		}
	}
	
	
	
	
	/// Finds the star with the provided index from the star pair database.
	/// # Arguments
	/// * `index` - The index of the star in the catalogue database.
	/// # Returns
	/// The actual position (usualy J2000).
	/// # Example
	/// ```
	/// panic!("NYI");
	/// ```
	fn find_star ( &self, index: usize ) -> Result<Equatorial, ()>
	{
		return Ok(self.catalogue[index]);
	}	
	
	/// Returns the field of view of the database compiled.
	fn get_fov ( &self ) -> Radians
	{
		return self.fov;
	}
}



//###############################################################################################//
//###############################################################################################//
//
//										Unit Tests
//
//###############################################################################################//
//###############################################################################################//


#[cfg(test)]
mod test
{
	// use crate::tracking_mode::database::StarDatabaseElement;
	// use crate::tracking_mode::database::Database;
	// use crate::tracking_mode::database::KVector;
	// use crate::tracking_mode::database::*;
	
	// use crate::util::list::List;
	// use crate::util::aliases::decimal_precision;
	// use crate::util::aliases::Decimal;
	// use crate::util::units::Radians;


	// fn sample_database ( ) -> Database
	// {
	// 	return Database
	// 	{
	// 		fov: 0.0, // Ignore
	// 		k_lookup: KVector::new( ),
	// 
	// 	}
	// }

	#[test]
	fn test_new ( )
	{
	/*	let database = Database::new();
		assert!( (database.fov - array_database::FOV).0.abs() < 0.000001 );
		assert_eq!(database.k_lookup as *const _, array_database::K_LOOKUP_DATABASE as *const _); 
		assert_eq!(database.k_vector as *const _, array_database::K_VECTOR_DATABASE as *const _);
		assert_eq!(database.pairs as *const _, array_database::STAR_PAIR_DATABASE as *const _); 
		assert_eq!(database.catalogue as *const _, array_database::CATALOGUE_DATABASE as *const _);
*/	}
	
	
	#[test]
	fn test_find_star ( )
	{
		
	}
	
	#[test]
	fn find_close_ref ( )
	{
		
	}
	
	// 
	// #[test]
	// fn test_find_star ( )
	// {
	// 	let database = Database{fov: 0.0, k_lookup}
	// }


	#[test]
	fn test_get_fov ( )
	{
		
	}

}