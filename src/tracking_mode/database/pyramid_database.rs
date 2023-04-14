/// Implementation for Database
use std::ops::Range;

use crate::tracking_mode::database::KVector;
use crate::tracking_mode::StarPair;

use crate::util::units::Equatorial;
use crate::util::units::Radians;
use crate::util::err::Errors;
use crate::util::err::Error;

use super::PyramidDatabase;
use super::Database;

// The new function is located in template.txt and array_database.
// To use new, ```use crate::tracking_mode::database::array_database;```
// To modify, go to template.txt, modify it and run database_generator.rs.

impl <'a> Database for PyramidDatabase <'a>
{
	/// Returns the number of regions in the database.
	/// If PyramidDatabase, 1 will be returned.
	fn num_regions ( &self ) -> usize  { return 1; }


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


impl <'a> PyramidDatabase <'a>
{
	/// Trims the range provided by the k-vector so that every value is within the tolerance.
	/// # Arguments
	/// * `find`      - The center point of the tolerance.
	/// * `tolerance` - The distance allowed from the center.
	/// * `range`     - The range to be trimmed.
	/// # Returns
	/// The a trimmed version of `range`.
	fn trim_range ( &self, find: Radians, tolerance: Radians, range: Range<usize> ) -> Range<usize>
	{
		let mut start =if range.start < self.pairs.size() { range.start }else{self.pairs.size()-1};
		let mut end   =if range.end   < self.pairs.size() { range.end   }else{self.pairs.size()};

		// lower bounds
		loop
		{
			let valid    = start < end && start < self.pairs.size() - 1;
			let distance = self.angle_distance(self.pairs.get(start));
			if !(distance.is_ok() && tolerance.0 < (find - distance.unwrap()).abs() && valid)
			{
				break;
			}

			start += 1;
		}

		// upper bounds
		loop
		{
			let distance = self.angle_distance(self.pairs.get(end - 1));
			if !(distance.is_ok() && tolerance.0 < (find - distance.unwrap()).abs() && start < end)
			{
				break;
			}
			end -= 1;
		}

		return Range{start: start, end: end};
	}



	/// Finds the angular distance between a star pair referencing the catalogue.
	/// # Arguments
	/// * `pair` - The pair to find the distance from.
	/// # Returns
	/// The angular distance between the pair.
	pub fn angle_distance ( &self, pair: StarPair<usize> ) -> Error<Radians>
	{
		if pair.0 < self.catalogue.size() && pair.1 < self.catalogue.size()
		{
			return Ok(self.catalogue.get(pair.0).angle_distance(self.catalogue.get(pair.1)));
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
	// 
	// use crate::tracking_mode::StarPair;
	// // use crate::tracking_mode::database::MockKVectorSearch;
	// use crate::tracking_mode::database::PyramidDatabase;
	// use crate::tracking_mode::database::Database;
	// use crate::tracking_mode::database::KVector;
	//
	// use crate::util::units::Equatorial;
	// use crate::util::units::Radians;
	// use crate::util::list::ArrayList;
	// use crate::util::list::List;
	// // use crate::util::test::TestEqual;
	// use crate::util::err::Errors;
	// // use crate::util::err::Error;
	// use crate::util::aliases::DECIMAL_PRECISION;
	//
	//
	// static DEFAULT_K_VECTOR_BIN : [usize;5]          = [0, 2, 4, 5, 9];
	// static DEFAULT_PAIRS: [StarPair<usize>; 9] = [
	// 	StarPair(0, 0),
	// 	StarPair(0, 1),
	// 	StarPair(0, 2),
	// 	StarPair(0, 3),
	// 	StarPair(0, 4),
	// 	StarPair(0, 5),
	// 	StarPair(0, 6),
	// 	StarPair(0, 7),
	// 	StarPair(0, 8),
	// ];
	// static DEFAULT_CATALOGUE : [Equatorial;9] =
	// [
	// 	Equatorial{ra: Radians(0.0), dec: Radians(0.0)},
	// 	Equatorial{ra: Radians(0.1), dec: Radians(0.0)},
	// 	Equatorial{ra: Radians(0.2), dec: Radians(0.0)},
	// 	Equatorial{ra: Radians(0.3), dec: Radians(0.0)},
	// 	Equatorial{ra: Radians(0.4), dec: Radians(0.0)},
	// 	Equatorial{ra: Radians(0.5), dec: Radians(0.0)},
	// 	Equatorial{ra: Radians(0.6), dec: Radians(0.0)},
	// 	Equatorial{ra: Radians(0.7), dec: Radians(0.0)},
	// 	Equatorial{ra: Radians(0.8), dec: Radians(0.0)},
	// ];
	//
	// // Uses the above values to create a database.
	// fn create_database ( ) -> PyramidDatabase<'static>
	// {
	// 	let k_vector = KVector::new(DEFAULT_K_VECTOR_BIN.len(), 0.0, 0.8);
	// 	return PyramidDatabase
	// 	{
	// 		fov: DEFAULT_CATALOGUE[8].angle_distance(DEFAULT_CATALOGUE[0]),
	// 		k_lookup: k_vector,
	// 		k_vector: &DEFAULT_K_VECTOR_BIN,
	// 		pairs: &DEFAULT_PAIRS,
	// 		catalogue: &DEFAULT_CATALOGUE
	// 	};
	// }


//###############################################################################################//
//
//										Find Close Ref
//
// fn find_close_ref (
// 			&self, find : Radians, tolerance: Radians, found : &mut dyn List<StarPair<usize>> )
//
//###############################################################################################//

// #[test]
// // If the k_vector cannot find anything in range, it will immediatly end without checking.
// fn test_find_close_ref_invalid_angle ( )
// {
// 	let database = create_database();
//
// 	let find      = Radians(0.9);
// 	let tolerance = Radians(0.01);
// 	let mut found : Vec<StarPair<usize>> = Vec::new();
// 	database.find_close_ref(find, tolerance, &mut found);
//
// 	assert_eq!(found.size(), 0);
// }
//
//
// #[test]
// // If the arraylist is too small to fit all values, it should try to fit as many as possible.
// fn test_find_close_ref_too_small ( )
// {
// 	let database = create_database();
//
// 	let find      = Radians(0.0);
// 	let tolerance = Radians(10.0);
// 	let mut found : ArrayList<StarPair<usize>, 2> = ArrayList::new();
// 	database.find_close_ref(find, tolerance, &mut found);
//
// 	assert_eq!(2, found.size());
// 	assert_eq!(StarPair(0, 0),found.get(0));
// 	assert_eq!(StarPair(0, 1),found.get(1));
// }
//
//
// #[test]
// // If the list is big enough and the inputs are correct, the correct values should be returned.
// fn test_find_close_ref_valid ( )
// {
// 	let database = create_database();
//
// 	let mut find      = Radians(0.1);
// 	let mut tolerance = Radians(0.01);
// 	let mut found : ArrayList<StarPair<usize>, 10> = ArrayList::new();
// 	database.find_close_ref(find, tolerance, &mut found);
// 	assert_eq!(1, found.size());
// 	assert_eq!(StarPair(0, 1), found.get(0));
//
// 	find      = Radians(0.1);
// 	tolerance = Radians(0.1 + DECIMAL_PRECISION);
// 	found = ArrayList::new();
// 	database.find_close_ref(find, tolerance, &mut found);
// 	assert_eq!(3, found.size());
// 	assert_eq!(StarPair(0, 0), found.get(0));
// 	assert_eq!(StarPair(0, 1), found.get(1));
// 	assert_eq!(StarPair(0, 2), found.get(2));
//
// 	find      = Radians(0.7);
// 	tolerance = Radians(0.1 + DECIMAL_PRECISION);
// 	found = ArrayList::new();
// 	database.find_close_ref(find, tolerance, &mut found);
// 	assert_eq!(3, found.size());
// 	assert_eq!(StarPair(0, 6), found.get(0));
// 	assert_eq!(StarPair(0, 7), found.get(1));
// 	assert_eq!(StarPair(0, 8), found.get(2));
//
//
// 	find      = Radians(0.1);
// 	tolerance = Radians(2.0);
// 	found = ArrayList::new();
// 	database.find_close_ref(find, tolerance, &mut found);
// 	assert_eq!(9, found.size());
//
//
// 	assert_eq!(StarPair(0, 0), found.get(0));
// 	assert_eq!(StarPair(0, 1), found.get(1));
// 	assert_eq!(StarPair(0, 2), found.get(2));
// 	assert_eq!(StarPair(0, 3), found.get(3));
// 	assert_eq!(StarPair(0, 4), found.get(4));
// 	assert_eq!(StarPair(0, 5), found.get(5));
// 	assert_eq!(StarPair(0, 6), found.get(6));
// 	assert_eq!(StarPair(0, 7), found.get(7));
// 	assert_eq!(StarPair(0, 8), found.get(8));
// }
//
//
// //###############################################################################################//
// //
// //										Find Star
// //
// // fn find_star ( &self, index: usize ) -> Error<Equatorial>
// //
// //###############################################################################################//
//
// #[test]
// // If a star is within the bounds of the catalogue, the corresponding element should be returned.
// fn test_find_star_exists ( )
// {
// 	let database = create_database();
// 	assert_eq!(DEFAULT_CATALOGUE[0], database.find_star(0).expect("Not out of bounds"));
// 	assert_eq!(DEFAULT_CATALOGUE[1], database.find_star(1).expect("Not out of bounds"));
// 	assert_eq!(DEFAULT_CATALOGUE[2], database.find_star(2).expect("Not out of bounds"));
// 	assert_eq!(DEFAULT_CATALOGUE[3], database.find_star(3).expect("Not out of bounds"));
// 	assert_eq!(DEFAULT_CATALOGUE[4], database.find_star(4).expect("Not out of bounds"));
// 	assert_eq!(DEFAULT_CATALOGUE[5], database.find_star(5).expect("Not out of bounds"));
// 	assert_eq!(DEFAULT_CATALOGUE[6], database.find_star(6).expect("Not out of bounds"));
// 	assert_eq!(DEFAULT_CATALOGUE[7], database.find_star(7).expect("Not out of bounds"));
// 	assert_eq!(DEFAULT_CATALOGUE[8], database.find_star(8).expect("Not out of bounds"));
// }
//
// #[test]
// // If a star is outside the bounds of the catalogue, an error should be returned.
// fn test_find_star_invalid ( )
// {
// 	let database = create_database();
// 	assert!(database.find_star(9).is_err());
// }
}
