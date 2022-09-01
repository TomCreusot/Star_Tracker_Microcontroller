//! Implementation of RegionalDatabase
/// Implementation for Database
// use std::ops::Range;

use crate::tracking_mode::database::KVector;
use crate::tracking_mode::StarPair;

use crate::util::units::Equatorial;
// use crate::util::err::Errors;
// use crate::util::err::Error;

use super::RegionalDatabase;
use super::Database;

impl <'a> Database for RegionalDatabase <'a>
{
	/// Returns the number of regions in the database.
	/// If PyramidDatabase, 1 will be returned.
	fn num_regions ( &self ) -> usize    {   return self.num_regions;  }


	/// For find_close_ref.
	/// Checks if the region is the same as the pair.
	/// # Argument
	/// * `index_pairs` - The index of the star pair to investigate.
	fn is_correct_region ( &self, index_pairs: usize ) -> bool
	{
		return self.pairs_region.get(index_pairs) & (1 << self.region_selected ) != 0;
	}

	/// Moves the database to the next database.
	/// Useful for RegionalDatabase.
	/// Not usefull for the standard PyramidDatabase.
	/// # Returns
	/// False if selected region returns to 0.
	fn increment_region ( &mut self ) -> bool
	{
		if self.region_selected < self.num_regions()
		{
			self.region_selected += 1;
			return true;
		}
		self.region_selected = 0;
		return false;
	}


	/// Gets the star pair at the index in the array.
	/// Used for any trait implementations bellow.
	fn get_pairs          ( &self, i: usize )->StarPair<usize> { return self.pairs.get(i);     }
	/// Gets the size of the star pairs array.
	/// Used for any trait implementations bellow.
	fn get_pairs_size     ( &self ) -> usize                   { return self.pairs.size();     }

	/// Gets the catalogue array.
	/// Used for any trait implementations bellow.
	fn get_catalogue      ( &self, i: usize ) -> Equatorial    {return self.catalogue.get(i);  }
	/// Gets the catalogue array.
	/// Used for any trait implementations bellow.
	fn get_catalogue_size ( &self ) -> usize                   { return self.catalogue.size(); }

	/// Gets the k_vector array.
	/// Used for any trait implementations bellow.
	fn get_k_vector       ( &self, i: usize ) -> usize         { return self.k_vector.get(i);  }
	/// Gets the k_vector array.
	/// Used for any trait implementations bellow.
	fn get_k_vector_size  ( &self ) -> usize                   { return self.k_vector.size();  }
	/// Gets the k_vector lookup equation.
	/// Used for any trait implementations bellow.
	fn get_k_lookup  ( &self ) -> KVector                      { return self.k_lookup;         }
}
