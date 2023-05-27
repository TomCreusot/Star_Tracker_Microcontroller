/// Implementation for Database

use crate::tracking_mode::database::KVector;
use crate::tracking_mode::StarPair;

use crate::util::units::Equatorial;
use crate::util::units::Radians;

use super::RegionalDatabase;
use super::Database;

// The new function is located in template.txt and array_database.
// To use new, ```use crate::tracking_mode::database::array_database;```
// To modify, go to template.txt, modify it and run database_generator.rs.

impl <'a> Database for RegionalDatabase <'a>
{
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
	
	
	/// Gets the field of view the database was created for.
	fn get_fov            ( &self ) -> Radians                 { return self.fov;              }
}



//###############################################################################################//
//###############################################################################################//
//
//										Unit Tests
//
//###############################################################################################//
//###############################################################################################//


#[cfg(test)]
#[allow(unused_must_use)]
mod test
{
	use crate::tracking_mode::StarPair;
	use crate::tracking_mode::database::RegionalDatabase;
	use crate::tracking_mode::database::Database;
	use crate::tracking_mode::database::KVector;
	
	use crate::util::units::Equatorial;
	use crate::util::units::Radians;
	use crate::util::units::BitField;
	
	
	// Tests using Regional Database
	static DEFAULT_K_VECTOR_BIN : [usize;5]          = [0, 2, 4, 5, 9];
	static DEFAULT_PAIRS: [StarPair<usize>; 9] = [
		StarPair(0, 0),
		StarPair(0, 1),
		StarPair(0, 2),
		StarPair(0, 3),
		StarPair(0, 4),
		StarPair(0, 5),
		StarPair(0, 6),
		StarPair(0, 7),
		StarPair(0, 8),
	];
	static DEFAULT_CATALOGUE : [Equatorial;9] =
	[
		Equatorial{ra: Radians(0.0), dec: Radians(0.0)},
		Equatorial{ra: Radians(0.1), dec: Radians(0.0)},
		Equatorial{ra: Radians(0.2), dec: Radians(0.0)},
		Equatorial{ra: Radians(0.3), dec: Radians(0.0)},
		Equatorial{ra: Radians(0.4), dec: Radians(0.0)},
		Equatorial{ra: Radians(0.5), dec: Radians(0.0)},
		Equatorial{ra: Radians(0.6), dec: Radians(0.0)},
		Equatorial{ra: Radians(0.7), dec: Radians(0.0)},
		Equatorial{ra: Radians(0.8), dec: Radians(0.0)},
	];
	
	static DEFAULT_CATALOGUE_FIELD : [BitField; 9] = 
	[
		BitField(0b0000),
		BitField(0b0001),
		BitField(0b0010),
		BitField(0b0011),
		BitField(0b0100),
		BitField(0b0101),
		BitField(0b0110),
		BitField(0b0111),
		BitField(0b1000),
	];

	// Uses the above values to create a database.
	fn create_database ( ) -> RegionalDatabase<'static>
	{
		let k_vector = KVector::new(DEFAULT_K_VECTOR_BIN.len(), 0.0, 0.8);
		return RegionalDatabase
		{
			fov: DEFAULT_CATALOGUE[8].angle_distance(DEFAULT_CATALOGUE[0]),
			num_fields: 4,
			k_lookup: k_vector,
			k_vector: &DEFAULT_K_VECTOR_BIN,
			pairs: &DEFAULT_PAIRS,
			catalogue: &DEFAULT_CATALOGUE,
			catalogue_field: &DEFAULT_CATALOGUE_FIELD
		};
	}


//###############################################################################################//
//
//										Trait Database
//
// pub fn get_pairs          ( &self,     usize ) -> StarPair<usize>
// pub fn get_pairs_size     ( &mut self        ) -> usize
// pub fn get_catalogue      ( &self,     usize ) -> Equatorial
// pub fn get_catalogue_size ( &self            ) -> usize
//
// pub fn get_k_vector       ( &self,     usize ) -> usize
// pub fn get_k_vector_size  ( &self            ) -> usize
// pub fn get_k_lookup       ( &self            ) -> KVector
//
// pub fn get_fov            ( &self            ) -> Radians
//
//###############################################################################################//
//										~ get_pairs ~											 //
	#[test]
	fn test_get_pairs ( )
	{
		let database = create_database();
		assert_eq!(database.get_pairs(3), DEFAULT_PAIRS[3]);
	}
	
	#[test] #[should_panic]
	fn test_get_pairs_fail ( )
	{
		let database = create_database();
		database.get_pairs(10);
	}



//										~ get_pairs_size ~										 //
	#[test]
	fn test_get_pairs_size ( )
	{
		let database = create_database();
		assert_eq!(database.get_pairs_size(), DEFAULT_PAIRS.len());
	}
	
//										~ get_catalogue ~										 //
	#[test]
	fn test_get_catalogue ( )
	{
		let database = create_database();
		assert_eq!(database.get_catalogue(3), DEFAULT_CATALOGUE[3]);
	}
	
	#[test] #[should_panic]
	fn test_get_catalogue_fail ( )
	{
		let database = create_database();
		database.get_catalogue(10);
	}
	
//										~ get_catalogue_size ~									 //
	#[test]
	fn test_get_catalogue_size ( )
	{
		let database = create_database();
		assert_eq!(database.get_catalogue_size(), DEFAULT_CATALOGUE.len());
	}
	



//										~ get_k_vector ~										 //
	#[test]
	fn test_get_k_vector ( )
	{
		let database = create_database();
		assert_eq!(database.get_k_vector(3), DEFAULT_K_VECTOR_BIN[3]);
	}
	
	#[test] #[should_panic]
	fn test_get_k_vector_fail ( )
	{
		let database = create_database();
		database.get_k_vector(10);
	}
	
//										~ get_k_vector_size ~									 //
	#[test]
	fn test_get_k_vector_size ( )
	{
		let database = create_database();
		assert_eq!(database.get_k_vector_size(), DEFAULT_K_VECTOR_BIN.len());
	}
	
	
//										~ get_k_lookup ~										 //
	#[test]
	fn test_get_k_lookup ( )
	{
		let database = create_database();
		assert_eq!(database.get_k_lookup().gradient, database.k_lookup.gradient);
	}
	
	
	
//										~ get_fov ~												 //
	#[test]
	fn test_get_fov ( )
	{
		let database = create_database();
		assert_eq!(database.get_fov(), database.fov);
	}
}