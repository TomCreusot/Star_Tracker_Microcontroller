/// Implementation for Database
use super::PyramidDatabase;
use super::Database;
use crate::tracking_mode::database::array_database::*;
use crate::tracking_mode::database::KVectorSearch;
use crate::tracking_mode::StarPair;

use crate::util::units::Equatorial;
use crate::util::units::Radians;
use crate::util::list::List;
use crate::util::err::Errors;
use crate::util::err::Error;



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
	fn find_close_ref ( &self, find : Radians, found : &mut dyn List<StarPair<usize>> )
	{
		// Error<ops::RangeInclusive<usize>>
		let k_range_wrapped = self.k_lookup.get_bins(find);
		
		// If the element is within the range.
		if let Ok(k_range) = k_range_wrapped
		{
			// Subing the range into the kvector bins outputs the location of the upper and lower bounds of the star pairs database.
			// The middle point is the most likely to be the angle specified.
			let lower_bounds : usize = self.k_vector[k_range.start().clone()];
			let upper_bounds : usize = self.k_vector[k_range.end().clone()];
			let length = upper_bounds - lower_bounds;
			let mid = (upper_bounds + lower_bounds) / 2;
			// Half the length as starting from the center and observing both sides
			for i in 0..length / 2 + 1
			{
				if !found.is_full() 
				{
					found.push_back(self.pairs[mid + i]).expect("found should not be full.");
				}
				if !found.is_full() && i != 0
				{
					found.push_back(self.pairs[mid - i]).expect("found should not be full.");
				}
			}
			// If the list is even, the final value will not be included. 
			if !found.is_full() && length % 2 != 0
			{
				found.push_back(self.pairs[upper_bounds - 1]).expect("found should not be full.");
			}
		}
	}
	
	
	
	
	/// Finds the star with the provided index from the star pair database.
	/// # Arguments
	/// * `index` - The index of the star in the catalogue database.
	/// # Returns
	/// The actual position (usualy J2000).
	fn find_star ( &self, index: usize ) -> Error<Equatorial>
	{
		if index < self.catalogue.len()
		{
			return Ok(self.catalogue[index]);
		}
		return Err(Errors::OutOfBounds);
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
#[allow(unused_must_use)]
mod test
{
	use crate::tracking_mode::StarPair;
	use crate::tracking_mode::database::*;
	
	use crate::util::units::Equatorial;
	use crate::util::units::Radians;
	use crate::util::list::ArrayList;
	use crate::util::test::TestEqual;
	
	static DEFAULT_K_VEC : [usize;5]          = [0, 2, 4, 6, 8];
	static DEFAULT_PAIRS: [StarPair<usize>; 8] = [
		StarPair(0, 1), // (0, 2) 
		StarPair(1, 2), // (0, 2)
		StarPair(2, 3), // (2, 4)
		StarPair(3, 4), // (2, 4)
		StarPair(4, 5), // (4, 6)
		StarPair(5, 6), // (4, 6)
		StarPair(6, 7), // (6, 8)
		StarPair(7, 8), // (6, 8)
	];
	static DEFAULT_CATALOGUE : [Equatorial;0] = [];	
	
	
	fn create_database ( ) -> PyramidDatabase
	{
		return PyramidDatabase
		{
			fov: Radians(0.0), 
			k_lookup: KVector::new(5, 0.0, 8.0), // Create a kvector designed around DEFAULT_K_VEC.
			k_vector: &DEFAULT_K_VEC, 
			pairs: &DEFAULT_PAIRS, 
			catalogue: &DEFAULT_CATALOGUE
		};
	}
	
	//
	// fn find_close_ref ( &self, Radians, List<StarPair<usize>> )
	// Unfortuantly structs struggle containing traits, this function was tested with the black box approach.
	//
	
	#[test]
	// If the k-vector cannot find the element within its range, nothing should happen.
	fn find_close_ref_out_of_range ( )
	{
		let database = create_database();

		let mut out : Vec<StarPair<usize>> = Vec::new();
		database.find_close_ref(Radians(100.0), &mut out);
		
		assert_eq!(out.len(), 0);
	}

	
	#[test]
	// If the output list cannot store enough elements, it should input as many as it can.
	fn find_close_ref_not_enough_space ( )
	{
		let database = create_database();

		let mut out : ArrayList<StarPair<usize>, 1> = ArrayList::new();
		database.find_close_ref(Radians(1.0), &mut out);
		// 1 fall under 0, 2 which is: (0,2) (2, 4) which is (0,1), (1, 2), |This one: (2, 3)|, (2, 4)
		assert_eq!(out.size(), 1);
		assert_eq!(out.get(0).0, 2);
		assert_eq!(out.get(0).1, 3);
	}


	#[test]
	// If the element to search is off the edges of the kvector and within a bin and enough space, 
	// they should be added in the appropreate order.
	fn find_close_ref_odd ( )
	{
		let database = create_database();

		let mut out : ArrayList<StarPair<usize>, 10> = ArrayList::new();
		database.find_close_ref(Radians(3.0), &mut out);
		// With a tolerance of 2 and a bin set of [0, 2, 4, 6, 8]
		// 3 is on the 4 side of [2,4] so the range will be [2, 4, 6]
		// The order starts at the middle or middle - 1 which is is:
		// mid     (i = 4): (4,5)
		// mid + 1 (i = 5): (5,6)
		// mid - 1 (i = 3): (3,4)
		// mid + 2 (i = 6): (6,7)
		// mid - 2 (i = 2): (2,3)
		
		assert_eq!(out.size(), 5);

		
		assert_eq!(out.get(0).0, 4);
		assert_eq!(out.get(0).1, 5);
		
		assert_eq!(out.get(1).0, 5);
		assert_eq!(out.get(1).1, 6);
		
		assert_eq!(out.get(2).0, 3);
		assert_eq!(out.get(2).1, 4);
		
		assert_eq!(out.get(3).0, 6);
		assert_eq!(out.get(3).1, 7);
		
		assert_eq!(out.get(4).0, 2);
		assert_eq!(out.get(4).1, 3);	
	}
	





	#[test]
	fn test_find_star ( )
	{
		let k_vector = KVector{
			gradient: 0.0, 
			intercept: 0.0, 
			min_value: Radians(0.0), 
			max_value: Radians(0.0),
			num_bins: 1};
		static K_VEC : [usize;0] = [];
		static PAIRS: [StarPair<usize>;0] = [];
		static CATALOGUE : [Equatorial;1] = [Equatorial{ra: Radians(0.9), dec: Radians(0.1)}];
		let database = PyramidDatabase{
			fov: Radians(0.3), 
			k_lookup: k_vector, 
			k_vector: &K_VEC, 
			pairs: &PAIRS, 
			catalogue: &CATALOGUE
		};
		assert!(database.find_star(0).expect("SHOULD BE OK").ra.0.test_close(&0.89999, 0.0001));
		assert!(database.find_star(0).expect("SHOULD BE OK").dec.0.test_close(&0.09999, 0.0001));
		
		assert!(database.find_star(1).is_err());
		assert!(database.find_star(1).is_err());
	}
	
	
	

	#[test]
	fn test_get_fov ( )
	{
		let k_vector = KVector{
			gradient: 0.0, 
			intercept: 0.0, 
			min_value: Radians(0.0), 
			max_value: Radians(0.0),
			num_bins: 1};
		static K_VEC : [usize;0] = [];
		static PAIRS: [StarPair<usize>;0] = [];
		static CATALOGUE : [Equatorial;0] = [];
		let database = PyramidDatabase{
			fov: Radians(0.3), 
			k_lookup: k_vector, 
			k_vector: &K_VEC, 
			pairs: &PAIRS, 
			catalogue: &CATALOGUE
		};
		assert!(database.get_fov().test_close(&Radians(0.29999), 0.0001));
	}

}