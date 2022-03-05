/// Implementation for Database
use std::ops::Range;

use super::PyramidDatabase;
use super::Database;
use crate::tracking_mode::database::KVectorSearch;
use crate::tracking_mode::StarPair;

use crate::util::units::Equatorial;
use crate::util::units::Radians;
use crate::util::list::List;
use crate::util::err::Errors;
use crate::util::err::Error;

// The new function is located in template.txt and array_database.
// To use new, ```use crate::tracking_mode::database::array_database;```
// To modify, go to template.txt, modify it and run database_generator.rs.


impl Database for PyramidDatabase
{
	/// Finds close matches to the provided angular separation and returns the star pair reference.
	/// The an element of the star pair reference can be inserted into `find_star` to get the actual location.
	/// # Arguments
	/// * `find` - The angular separation between the found stars to find in the database.
	/// * `found` - The closest matches to the provided `find`.
	fn find_close_ref ( &self, find : Radians, tolerance: Radians, 
														found : &mut dyn List<StarPair<usize>> )
	{
		let range_k_vec_wrapped = self.k_lookup.get_bins(find, tolerance);
		if range_k_vec_wrapped.is_ok()
		{
			let range_k_vec = range_k_vec_wrapped.unwrap();
			let mut end_range = range_k_vec.end;
			if self.k_vector.len() <= end_range
			{
				end_range -= 1; // sometimes the upper value is stored in the bin above.
			} 
			let mut range = self.k_vector[range_k_vec.start]..self.k_vector[end_range];
			range = self.trim_range(find, tolerance, range);

			for i in range
			{
				if !found.is_full()
				{
					found.push_back(self.pairs[i]).expect("?");
				}
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

	
	
	/// Trims the range provided by the k-vector so that every value is within the tolerance.
	/// # Arguments
	/// * `find`      - The center point of the tolerance.
	/// * `tolerance` - The distance allowed from the center.
	/// * `range`     - The range to be trimmed.
	/// # Returns
	/// The a trimmed version of `range`.
	fn trim_range ( &self, find: Radians, tolerance: Radians, range: Range<usize> )	-> Range<usize>
	{
		let mut start =if range.start < self.pairs.len() { range.start } else {self.pairs.len()-1};
		let mut end   =if range.end   < self.pairs.len() { range.end   } else {self.pairs.len()};
		
		// lower bounds
		loop
		{
			let valid    = start < end && start < self.pairs.len() - 1;
			let distance = self.angle_distance(self.pairs[start]);
			if !(distance.is_ok() && tolerance.0 < (find - distance.unwrap()).abs() && valid)
			{
				break;
			}
			
			start += 1;
		}
		
		// upper bounds
		loop
		{
			let distance = self.angle_distance(self.pairs[end - 1]);
			if !(distance.is_ok() && tolerance.0 < (find - distance.unwrap()).abs() && start < end)
			{
				break;
			}
			end -= 1;
		}
		
		return Range{start: start, end: end};
	}
}
		

impl PyramidDatabase		
{
	/// Finds the angular distance between a star pair referencing the catalogue.
	/// # Arguments
	/// * `pair` - The pair to find the distance from.
	/// # Returns
	/// The angular distance between the pair.
	pub fn angle_distance ( &self, pair: StarPair<usize> ) -> Error<Radians>
	{
		if pair.0 < self.catalogue.len() && pair.1 < self.catalogue.len()
		{
			return Ok(self.catalogue[pair.0].angle_distance(self.catalogue[pair.1]));
		}
		else
		{
			return Err(Errors::OutOfBounds);
		}
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
	// use std::ops::Range;
	
	use crate::tracking_mode::StarPair;
	// use crate::tracking_mode::database::MockKVectorSearch;
	use crate::tracking_mode::database::PyramidDatabase;
	use crate::tracking_mode::database::Database;
	use crate::tracking_mode::database::KVector;

	use crate::util::units::Equatorial;
	use crate::util::units::Radians;
	use crate::util::list::ArrayList;
	use crate::util::list::List;
	// use crate::util::test::TestEqual;
	use crate::util::err::Errors;
	// use crate::util::err::Error;
	use crate::util::aliases::DECIMAL_PRECISION;
	

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
		
	// Uses the above values to create a database.
	fn create_database ( ) -> PyramidDatabase
	{
		let k_vector = KVector::new(DEFAULT_K_VECTOR_BIN.len(), 0.0, 0.8);	
		return PyramidDatabase
		{
			fov: DEFAULT_CATALOGUE[8].angle_distance(DEFAULT_CATALOGUE[0]),
			k_lookup: k_vector,
			k_vector: &DEFAULT_K_VECTOR_BIN,
			pairs: &DEFAULT_PAIRS,
			catalogue: &DEFAULT_CATALOGUE
		};
	}


//###############################################################################################//
//
//										Find Close Ref
//
// fn find_close_ref ( 
// 			&self, find : Radians, tolerance: Radians, found : &mut dyn List<StarPair<usize>> )
//
//###############################################################################################//

#[test]
// If the k_vector cannot find anything in range, it will immediatly end without checking.
fn test_find_close_ref_invalid_angle ( )
{
	let database = create_database();
	
	let find      = Radians(0.9);
	let tolerance = Radians(0.01);
	let mut found : Vec<StarPair<usize>> = Vec::new();
	database.find_close_ref(find, tolerance, &mut found);
	
	assert_eq!(found.len(), 0);
}


#[test]
// If the arraylist is too small to fit all values, it should try to fit as many as possible.
fn test_find_close_ref_too_small ( )
{
	let database = create_database();
	
	let find      = Radians(0.0);
	let tolerance = Radians(10.0);
	let mut found : ArrayList<StarPair<usize>, 2> = ArrayList::new();
	database.find_close_ref(find, tolerance, &mut found);
	
	assert_eq!(2, found.size());
	assert_eq!(StarPair(0, 0),found.get(0));
	assert_eq!(StarPair(0, 1),found.get(1));
}


#[test]
// If the list is big enough and the inputs are correct, the correct values should be returned.
fn test_find_close_ref_valid ( )
{
	let database = create_database();
	
	let mut find      = Radians(0.1);
	let mut tolerance = Radians(0.01);
	let mut found : ArrayList<StarPair<usize>, 10> = ArrayList::new();
	database.find_close_ref(find, tolerance, &mut found);
	assert_eq!(1, found.size());
	assert_eq!(StarPair(0, 1), found.get(0));
	
	find      = Radians(0.1);
	tolerance = Radians(0.1 + DECIMAL_PRECISION);
	found = ArrayList::new();
	database.find_close_ref(find, tolerance, &mut found);
	assert_eq!(3, found.size());
	assert_eq!(StarPair(0, 0), found.get(0));
	assert_eq!(StarPair(0, 1), found.get(1));
	assert_eq!(StarPair(0, 2), found.get(2));

	find      = Radians(0.7);
	tolerance = Radians(0.1 + DECIMAL_PRECISION);
	found = ArrayList::new();
	database.find_close_ref(find, tolerance, &mut found);
	assert_eq!(3, found.size());
	assert_eq!(StarPair(0, 6), found.get(0));
	assert_eq!(StarPair(0, 7), found.get(1));
	assert_eq!(StarPair(0, 8), found.get(2));
	
	
	find      = Radians(0.1);
	tolerance = Radians(2.0);
	found = ArrayList::new();
	database.find_close_ref(find, tolerance, &mut found);
	assert_eq!(9, found.size());
	
	
	assert_eq!(StarPair(0, 0), found.get(0));
	assert_eq!(StarPair(0, 1), found.get(1));
	assert_eq!(StarPair(0, 2), found.get(2));
	assert_eq!(StarPair(0, 3), found.get(3));
	assert_eq!(StarPair(0, 4), found.get(4));
	assert_eq!(StarPair(0, 5), found.get(5));
	assert_eq!(StarPair(0, 6), found.get(6));
	assert_eq!(StarPair(0, 7), found.get(7));
	assert_eq!(StarPair(0, 8), found.get(8));
}


//###############################################################################################//
//
//										Find Star
//
// fn find_star ( &self, index: usize ) -> Error<Equatorial>
//
//###############################################################################################//

#[test]
// If a star is within the bounds of the catalogue, the corresponding element should be returned.
fn test_find_star_exists ( )
{
	let database = create_database();
	assert_eq!(DEFAULT_CATALOGUE[0], database.find_star(0).expect("Not out of bounds"));
	assert_eq!(DEFAULT_CATALOGUE[1], database.find_star(1).expect("Not out of bounds"));
	assert_eq!(DEFAULT_CATALOGUE[2], database.find_star(2).expect("Not out of bounds"));
	assert_eq!(DEFAULT_CATALOGUE[3], database.find_star(3).expect("Not out of bounds"));
	assert_eq!(DEFAULT_CATALOGUE[4], database.find_star(4).expect("Not out of bounds"));
	assert_eq!(DEFAULT_CATALOGUE[5], database.find_star(5).expect("Not out of bounds"));
	assert_eq!(DEFAULT_CATALOGUE[6], database.find_star(6).expect("Not out of bounds"));
	assert_eq!(DEFAULT_CATALOGUE[7], database.find_star(7).expect("Not out of bounds"));
	assert_eq!(DEFAULT_CATALOGUE[8], database.find_star(8).expect("Not out of bounds"));
}

#[test]
// If a star is outside the bounds of the catalogue, an error should be returned.
fn test_find_star_invalid ( )
{
	let database = create_database();
	assert!(database.find_star(9).is_err());
}




//###############################################################################################//
//
//										Trim Range
//
// fn trim_range ( &self, find: Radians, tolerance: Radians, range: Range<usize> )-> Range<usize>
//
//###############################################################################################//

#[test]
// The output of trim range should always be within the tolerance.
fn test_trim_range ( )
{
	let database      = create_database();
	let find          = Radians(0.3);
	let range         = 0..10;
	let mut tolerance = Radians(-1.0);
	let mut output    = 8..8; // Lower bounds first, everything moves up.
	assert_eq!(output, database.trim_range(find, tolerance, range.clone()));
	
	tolerance = Radians(0.000001);
	output    = 3..4;
	assert_eq!(output, database.trim_range(find, tolerance, range.clone()));

	tolerance = Radians(0.100001);
	output    = 2..5;
	assert_eq!(output, database.trim_range(find, tolerance, range.clone()));
	
	tolerance = Radians(0.200001);
	output    = 1..6;
	assert_eq!(output, database.trim_range(find, tolerance, range.clone()));
	
	tolerance = Radians(0.300001);
	output    = 0..7;
	assert_eq!(output, database.trim_range(find, tolerance, range.clone()));
	
	tolerance = Radians(0.400001);
	output    = 0..8;
	assert_eq!(output, database.trim_range(find, tolerance, range.clone()));
	
	tolerance = Radians(0.500001);
	output    = 0..9;
	assert_eq!(output, database.trim_range(find, tolerance, range.clone()));
}




//###############################################################################################//
//
//										Angle Distance
//
// pub fn angle_distance ( &self, pair: StarPair<usize> ) -> Radians
//
//###############################################################################################//

#[test]
// If a StarPair is provided within the bounds of the catalogue database, the angle should match.
fn test_angle_distance_valid ( )
{
	let database = create_database();
	// From the star pair database.
	assert_eq!(Ok(Radians(0.0)), database.angle_distance(StarPair(0, 0)));
	assert_eq!(Ok(Radians(0.1)), database.angle_distance(StarPair(0, 1)));
	assert_eq!(Ok(Radians(0.2)), database.angle_distance(StarPair(0, 2)));
	assert_eq!(Ok(Radians(0.3)), database.angle_distance(StarPair(0, 3)));
	assert_eq!(Ok(Radians(0.4)), database.angle_distance(StarPair(0, 4)));
	assert_eq!(Ok(Radians(0.5)), database.angle_distance(StarPair(0, 5)));
	assert_eq!(Ok(Radians(0.6)), database.angle_distance(StarPair(0, 6)));
	assert_eq!(Ok(Radians(0.7)), database.angle_distance(StarPair(0, 7)));
	assert_eq!(Ok(Radians(0.8)), database.angle_distance(StarPair(0, 8)));

	// Not in the star pair database.
	assert_eq!(Ok(Radians(0.1)), database.angle_distance(StarPair(2, 3)));
}


#[test]
// If a StarPair is outside the bounds of the catalogue database, Errors::OutOfBounds is returned.
fn test_angle_distance_invalid ( )
{
	let database = create_database();
	assert_eq!(Err(Errors::OutOfBounds), database.angle_distance(StarPair(0, 9)));
	assert_eq!(Err(Errors::OutOfBounds), database.angle_distance(StarPair(9, 0)));
	assert_eq!(Err(Errors::OutOfBounds), database.angle_distance(StarPair(9, 9)));
	assert_eq!(Err(Errors::OutOfBounds), database.angle_distance(StarPair(100, 100)));
}

}