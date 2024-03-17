//! Used for any database.
//! A set of functions may not be relevant to the required database, however, it is made universal.
use crate::core_include::*;

#[cfg(test)] use mockall::*;
#[cfg(test)] use mockall::predicate::*;

use crate::util::list::List;
use crate::util::units::Equatorial;
use crate::util::units::Radians;
use crate::util::err::Errors;
use crate::util::err::Error;

use crate::tracking_mode::StarPair;
use crate::tracking_mode::database::KVector;
use crate::tracking_mode::database::KVectorSearch;
use crate::tracking_mode::database::SearchResult;


#[cfg_attr(test, automock)]
pub trait Database
{
	/// Finds the star with the provided index from the star pair database.
	/// # Arguments
	/// * `index` - The index of the star in the catalogue database.
	/// # Returns
	/// The actual position (usually J2000).
	fn find_star ( &self, index: usize ) -> Error<Equatorial>
	{
		if index < self.get_catalogue_size()
		{
			return Ok(self.get_catalogue(index));
		}
		return Result::Err(Errors::OutOfBounds);
	}


	/// Finds close matches to the provided angular separation and returns the star pair reference.
	/// The an element of the star pair reference can be inserted into `find_star` to get the actual location.
	/// # Arguments
	/// * `find` - The angular separation between the found stars to find in the database.
	/// * `found` - The closest matches to the provided `find`.
	fn find_close_ref ( &self, find: Radians, tolerance: Radians, found: &mut dyn List<SearchResult> )
	{
		let range = self.find_close_ref_range(find, tolerance);
		for i in range.clone()
		{
			let pair   = self.get_pairs(i);
			let error  = 1.0;//(if i < mid { i } else { range.end - i } - range.start) as Decimal / range.len() as Decimal;
			if !found.is_full()
			{
				let result = SearchResult{result: pair, error: error };
				let _ = found.push_back(result);
			}
		}
	}
	
	
	/// Finds the range in the pair list where pairs of the given distance will be.  
	/// This is different to `find_close_ref` as it allows you to manually add pairs to the list.  
	/// This helps if you need a range.  
	/// * `find` - The angular separation between the found stars to find in the database.
	/// * `found` - The closest matches to the provided `find`.
	fn find_close_ref_range ( &self, find: Radians, tolerance: Radians ) -> Range<usize>
	{
		let range_k_vec_wrapped = self.get_k_lookup().get_bins(find, tolerance);
		if range_k_vec_wrapped.is_ok()
		{
			let range_k_vec = range_k_vec_wrapped.unwrap();
			let mut end_range = range_k_vec.end;
			if self.get_k_vector_size() <= end_range
			{
				end_range -= 1; // sometimes the upper value is stored in the bin above.
			}
			let range = self.get_k_vector(range_k_vec.start)..self.get_k_vector(end_range);
			return self.trim_range(find, tolerance, range);
		}
		return 0..0;
	}


	/// Gets the star pair at the index in the array.
	/// Used for any trait implementations bellow.
	fn get_pairs          ( &self, index: usize ) -> StarPair<usize>;
	/// Gets the size of the star pairs array.
	/// Used for any trait implementations bellow.
	fn get_pairs_size     ( &self ) -> usize;

	/// Gets the catalogue array.
	/// Used for any trait implementations below.
	fn get_catalogue      ( &self, index: usize ) -> Equatorial;
	/// Gets the catalogue array.
	/// Used for any trait implementations bellow.
	fn get_catalogue_size ( &self ) -> usize;

	/// Gets the k_vector array.
	/// Used for any trait implementations bellow.
	fn get_k_vector       ( &self, index: usize ) -> usize;
	/// Gets the k_vector array.
	/// Used for any trait implementations bellow.
	fn get_k_vector_size  ( &self ) -> usize;

	/// Gets the k_vector lookup equation.
	/// Used for any trait implementations below.
	fn get_k_lookup       ( &self ) -> KVector;
	
	/// Gets the field of view the database was created for.
	fn get_fov            ( &self ) -> Radians;


	/// Trims the range provided by the k-vector so that every value is within the tolerance.
	/// # Arguments
	/// * `find`      - The center point of the tolerance.
	/// * `tolerance` - The distance allowed from the center.
	/// * `range`     - The range to be trimmed.
	/// # Returns
	/// The a trimmed version of `range`.
	fn trim_range ( &self, find: Radians, tolerance: Radians, range: Range<usize> ) -> Range<usize>
	{
		// let mut start = if range.start < self.get_pairs_size()
		// 	{ range.start } else { self.get_pairs_size() - 1 };
		// 
		// let mut end   = if range.end   < self.get_pairs_size()
		// 	{ range.end   }else { self.get_pairs_size() };

		let mut start = core::cmp::min(range.start, self.get_pairs_size() - 1);
		let mut end   = core::cmp::min(range.end,   self.get_pairs_size());

		// lower bounds
		loop
		{
			let valid    = start < end && start < self.get_pairs_size() - 1;
			let distance = self.angle_distance(self.get_pairs(start));
			if !(distance.is_ok() && tolerance.0 < (find - distance.unwrap()).abs() && valid)
			{
				break;
			}

			start += 1;
		}

		// upper bounds
		loop
		{
			let distance = self.angle_distance(self.get_pairs(end - 1));
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
	fn angle_distance ( &self, pair: StarPair<usize> ) -> Error<Radians>
	{
		if pair.0 < self.get_catalogue_size() && pair.1 < self.get_catalogue_size()
		{
			let pair_1 = self.get_catalogue(pair.0);
			let pair_2 = self.get_catalogue(pair.1);
			return Ok(pair_1.angle_distance(pair_2));
		}
		else
		{
			return Result::Err(Errors::OutOfBounds);
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

	use crate::tracking_mode::StarPair;
	// use crate::tracking_mode::database::MockKVectorSearch;
	use crate::tracking_mode::database::PyramidDatabase;
	use crate::tracking_mode::database::SearchResult;
	use crate::tracking_mode::database::Database;
	use crate::tracking_mode::database::KVector;
	
	use crate::util::units::Equatorial;
	use crate::util::units::Radians;
	use crate::util::list::ArrayList;
	use crate::util::list::List;
	use crate::util::err::Errors;

	// Tests using Pyramid Database
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
	fn create_database ( ) -> PyramidDatabase<'static>
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



// ###############################################################################################//
// 
// 										Find Close Ref
// 
// fn find_close_ref ( &self, find: Radians, tolerance: Radians, found: &mut dyn List<SearchResult> )
// 
// ###############################################################################################//

	#[test]
	fn test_find_close_ref_range ( )
	{
		let database = create_database();
		let mut found = Vec::new();
		let mut find      = Radians(0.5);
		let mut tolerance = Radians(0.199);
		
		database.find_close_ref(find, tolerance, &mut found);
		assert_eq!(found.len(), 3);
		assert_eq!(found[0].result, StarPair(0, 4));
		assert_eq!(found[1].result, StarPair(0, 5));
		assert_eq!(found[2].result, StarPair(0, 6));
		
		found.clear();
		find      = Radians(0.5);
		tolerance = Radians(0.2001);
		database.find_close_ref(find, tolerance, &mut found);
		assert_eq!(found.len(), 5);
		assert_eq!(found[0].result, StarPair(0, 3));
		assert_eq!(found[1].result, StarPair(0, 4));
		assert_eq!(found[2].result, StarPair(0, 5));
		assert_eq!(found[3].result, StarPair(0, 6));
		assert_eq!(found[4].result, StarPair(0, 7));
		
		found.clear();
		find      = Radians(0.3);
		tolerance = Radians(0.01);
		database.find_close_ref(find, tolerance, &mut found);
		assert_eq!(found.len(), 1);
		assert_eq!(found[0].result, StarPair(0, 3));
	}
	
	
	
	
	#[test]
	// If the input has an invalid range or there is nothing to find, nothing should be added.
	fn test_find_close_ref_invalid ( )
	{
		let database = create_database();
		let mut found = Vec::new();
		let mut find      = Radians(-0.2);
		let mut tolerance = Radians(0.19);
		
		database.find_close_ref(find, tolerance, &mut found);
		assert_eq!(found.len(), 0);
		
		find      = Radians(0.5);
		tolerance = Radians(-0.1);
		database.find_close_ref(find, tolerance, &mut found);
		assert_eq!(found.len(), 0);
		
		found.clear();
		find      = Radians(0.9);
		tolerance = Radians(0.09);
		database.find_close_ref(find, tolerance, &mut found);
		assert_eq!(found.len(), 0);
	}
	
	
	#[test]
	// If there are too many elements in the bounds, it should not panic.
	fn test_find_close_ref_range_too_many ( )
	{
		let database = create_database();
		let mut found : ArrayList<SearchResult, 3> = ArrayList::new();
		let find      = Radians(0.5);
		let tolerance = Radians(10.0);
		
		database.find_close_ref(find, tolerance, &mut found);
		assert_eq!(found.size(), 3);
		assert_eq!(found.get(0).result, StarPair(0,0));
		assert_eq!(found.get(1).result, StarPair(0,1));
		assert_eq!(found.get(2).result, StarPair(0,2));
		
	}
	
	
	
	


// ###############################################################################################//
// 
// 										Find Close Ref Range
// 
// fn find_close_ref_range ( &self, find: Radians, tolerance: Radians ) -> Range<usize>
// 
// ###############################################################################################//

	#[test]
	fn test_find_close_ref_range_valid ( )
	{
		let database = create_database();
		let mut find      = Radians(0.5);
		let mut tolerance = Radians(0.199);
		assert_eq!(database.find_close_ref_range(find, tolerance), 4..7);
		
		find      = Radians(0.5);
		tolerance = Radians(0.2001);
		assert_eq!(database.find_close_ref_range(find, tolerance), 3..8);
		
		find      = Radians(0.8);
		tolerance = Radians(0.2);
		assert_eq!(database.find_close_ref_range(find, tolerance), 7..9);
		
		find      = Radians(0.3);
		tolerance = Radians(0.01);
		assert_eq!(database.find_close_ref_range(find, tolerance), 3..4);
	}
	
	#[test]
	// If the provided value is outside the bounds, 
	// it should safely return 0..0 if out of range or 8..8 nothing in range.
	fn test_find_close_ref_range_invalid ( )
	{
		let database = create_database();
		let mut find      = Radians(-0.2);
		let mut tolerance = Radians(0.19);
		assert_eq!(database.find_close_ref_range(find, tolerance), 0..0);
		
		find      = Radians(0.5);
		tolerance = Radians(-0.1);
		assert_eq!(database.find_close_ref_range(find, tolerance), 8..8);
		
		find      = Radians(0.9);
		tolerance = Radians(0.09);
		assert_eq!(database.find_close_ref_range(find, tolerance), 0..0);
	}



// ###############################################################################################//
// 
// 										Trim Range
// 
// fn trim_range ( &self, find: Radians, tolerance: Radians, range: Range<usize> )-> Range<usize>
// 
// ###############################################################################################//

	#[test]
	// The output of trim range should always be within the tolerance.
	fn test_trim_range ( )
	{
		let database      = create_database();
		let find          = Radians(0.3);
		let range         = 0..9;
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
	
	#[test]
	fn test_trim_range_out_of_bounds ( )
	{
		let database      = create_database();
		let find          = Radians(0.3);
		let tolerance = Radians(0.100001);
		let mut range     = 0..10;
		assert_eq!(2..5, database.trim_range(find, tolerance, range.clone()));
		
		range = 10..10;
		assert_eq!(8..8, database.trim_range(find, tolerance, range.clone()));
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
		
		// Not in the star pair database (0r, 2r), (0r, 3r).
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


	
//###############################################################################################//
//
//										Find Star
//
// pub fn angle_distance ( &self, index: usize ) -> Error<Equatorial> (OutOfBounds)
//
//###############################################################################################//
	
	#[test]
	fn test_find_star_valid ( )
	{
		let database = create_database();
		assert_eq!(database.find_star(DEFAULT_CATALOGUE.len() -1), 
					Ok(DEFAULT_CATALOGUE[DEFAULT_CATALOGUE.len() -1]));
	}
	
	#[test]
	fn test_find_star_invalid ( )
	{
		let database = create_database();
		assert_eq!(database.find_star(DEFAULT_CATALOGUE.len()), Err(Errors::OutOfBounds));
	}
}
