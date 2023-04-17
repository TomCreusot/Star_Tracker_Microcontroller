//! Used for any database.
//! A set of functions may not be relevant to the required database, however, it is made universal.
use std::ops::Range;

use mockall::*;
use mockall::predicate::*;

use crate::util::list::List;
use crate::util::units::Equatorial;
use crate::util::units::Radians;
use crate::util::units::BitField;
// use crate::util::units::BitCompare;
use crate::util::err::Errors;
use crate::util::err::Error;

use crate::tracking_mode::StarPair;
use crate::tracking_mode::database::KVector;
use crate::tracking_mode::database::KVectorSearch;
use crate::tracking_mode::database::SearchResult;


#[automock]
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
		return Err(Errors::OutOfBounds);
	}


	/// Finds close matches to the provided angular separation and returns the star pair reference.
	/// The an element of the star pair reference can be inserted into `find_star` to get the actual location.
	/// # Arguments
	/// * `find` - The angular separation between the found stars to find in the database.
	/// * `region_compare` - The index of the star pair is input, this function returns true if the pair is valid.
	/// * `found` - The closest matches to the provided `find`.
	/// # Returns
	/// The fields at the end of each pair.
	/// Use this information so that when finding multiple pairs connected, they can span multiple regions.
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
				found.push_back(result).expect("database::find_close_ref: Already checked if found full?");
			}
		}
	}
	
	
	/// Finds all elements within the tolerance.
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


	/// Returns the number of regions in the database.
	/// If PyramidDatabase, 1 will be returned.
	fn num_regions ( &self ) -> usize;

	/// Gets what regions a pair occupies.
	/// The trait implementation specifies every region is satisfied incase the database does not use regions.
	fn get_region ( &self, _pair_index: usize ) -> BitField
	{
		return BitField::ALL; // Every bit.
	}


	/// Gets the star pair at the index in the array.
	/// Used for any trait implementations bellow.
	fn get_pairs          ( &self, index: usize ) -> StarPair<usize>;
	/// Gets the size of the star pairs array.
	/// Used for any trait implementations bellow.
	fn get_pairs_size     ( &self ) -> usize;

	/// Gets the catalogue array.
	/// Used for any trait implementations bellow.
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
		let mut start =if range.start < self.get_pairs_size()
			{ range.start }else{self.get_pairs_size()-1};
		let mut end   =if range.end   < self.get_pairs_size()
			{ range.end   }else{self.get_pairs_size()};

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


//###############################################################################################//
//
//										Trim Range
//
// fn trim_range ( &self, find: Radians, tolerance: Radians, range: Range<usize> )-> Range<usize>
//
//###############################################################################################//

// 	#[test]
// 	// The output of trim range should always be within the tolerance.
// 	fn test_trim_range ( )
// 	{
// 		// let database      = create_database();
// 		// let find          = Radians(0.3);
// 		// let range         = 0..10;
// 		// let mut tolerance = Radians(-1.0);
// 		// let mut output    = 8..8; // Lower bounds first, everything moves up.
// 		// assert_eq!(output, database.trim_range(find, tolerance, range.clone()));
// 		//
// 		// tolerance = Radians(0.000001);
// 		// output    = 3..4;
// 		// assert_eq!(output, database.trim_range(find, tolerance, range.clone()));
// 		//
// 		// tolerance = Radians(0.100001);
// 		// output    = 2..5;
// 		// assert_eq!(output, database.trim_range(find, tolerance, range.clone()));
// 		//
// 		// tolerance = Radians(0.200001);
// 		// output    = 1..6;
// 		// assert_eq!(output, database.trim_range(find, tolerance, range.clone()));
// 		//
// 		// tolerance = Radians(0.300001);
// 		// output    = 0..7;
// 		// assert_eq!(output, database.trim_range(find, tolerance, range.clone()));
// 		//
// 		// tolerance = Radians(0.400001);
// 		// output    = 0..8;
// 		// assert_eq!(output, database.trim_range(find, tolerance, range.clone()));
// 		//
// 		// tolerance = Radians(0.500001);
// 		// output    = 0..9;
// 		// assert_eq!(output, database.trim_range(find, tolerance, range.clone()));
//
//
// 		panic!("NYI");
// 	}
//
// 	#[test]
// 	fn test_trim_range_regions ( )
// 	{
// 		panic!("NYI");
// 	}
//
//
//
//
// //###############################################################################################//
// //
// //										Angle Distance
// //
// // pub fn angle_distance ( &self, pair: StarPair<usize> ) -> Radians
// //
// //###############################################################################################//
//
// 	#[test]
// 	// If a StarPair is provided within the bounds of the catalogue database, the angle should match.
// 	fn test_angle_distance_valid ( )
// 	{
// 		// let database = create_database();
// 		// // From the star pair database.
// 		// assert_eq!(Ok(Radians(0.0)), database.angle_distance(StarPair(0, 0)));
// 		// assert_eq!(Ok(Radians(0.1)), database.angle_distance(StarPair(0, 1)));
// 		// assert_eq!(Ok(Radians(0.2)), database.angle_distance(StarPair(0, 2)));
// 		// assert_eq!(Ok(Radians(0.3)), database.angle_distance(StarPair(0, 3)));
// 		// assert_eq!(Ok(Radians(0.4)), database.angle_distance(StarPair(0, 4)));
// 		// assert_eq!(Ok(Radians(0.5)), database.angle_distance(StarPair(0, 5)));
// 		// assert_eq!(Ok(Radians(0.6)), database.angle_distance(StarPair(0, 6)));
// 		// assert_eq!(Ok(Radians(0.7)), database.angle_distance(StarPair(0, 7)));
// 		// assert_eq!(Ok(Radians(0.8)), database.angle_distance(StarPair(0, 8)));
// 		//
// 		// // Not in the star pair database.
// 		// assert_eq!(Ok(Radians(0.1)), database.angle_distance(StarPair(2, 3)));
// 		panic!("NYI");
//
// 	}
//
//
// 	#[test]
// 	// If a StarPair is outside the bounds of the catalogue database, Errors::OutOfBounds is returned.
// 	fn test_angle_distance_invalid ( )
// 	{
// 		panic!("NYI");
//
// 		// let database = create_database();
// 		// assert_eq!(Err(Errors::OutOfBounds), database.angle_distance(StarPair(0, 9)));
// 		// assert_eq!(Err(Errors::OutOfBounds), database.angle_distance(StarPair(9, 0)));
// 		// assert_eq!(Err(Errors::OutOfBounds), database.angle_distance(StarPair(9, 9)));
// 		// assert_eq!(Err(Errors::OutOfBounds), database.angle_distance(StarPair(100, 100)));
// 	}
//
//
// 	#[test]
// 	// If a StarPair is provided within the bounds of the catalogue database, the angle should match.
// 	fn test_angle_distance_valid ( )
// 	{
// 		panic!("NYI");
//
// 		let database = create_database();
// 		// From the star pair database.
// 		assert_eq!(Ok(Radians(0.0)), database.angle_distance(StarPair(0, 0)));
// 		assert_eq!(Ok(Radians(0.1)), database.angle_distance(StarPair(0, 1)));
// 		assert_eq!(Ok(Radians(0.2)), database.angle_distance(StarPair(0, 2)));
// 		assert_eq!(Ok(Radians(0.3)), database.angle_distance(StarPair(0, 3)));
// 		assert_eq!(Ok(Radians(0.4)), database.angle_distance(StarPair(0, 4)));
// 		assert_eq!(Ok(Radians(0.5)), database.angle_distance(StarPair(0, 5)));
// 		assert_eq!(Ok(Radians(0.6)), database.angle_distance(StarPair(0, 6)));
// 		assert_eq!(Ok(Radians(0.7)), database.angle_distance(StarPair(0, 7)));
// 		assert_eq!(Ok(Radians(0.8)), database.angle_distance(StarPair(0, 8)));
//
// 		// Not in the star pair database.
// 		assert_eq!(Ok(Radians(0.1)), database.angle_distance(StarPair(2, 3)));
// 	}
//
//
// 	#[test]
// 	// If a StarPair is outside the bounds of the catalogue database, Errors::OutOfBounds is returned.
// 	fn test_angle_distance_invalid ( )
// 	{
// 		panic!("NYI");
//
// 		let database = create_database();
// 		assert_eq!(Err(Errors::OutOfBounds), database.angle_distance(StarPair(0, 9)));
// 		assert_eq!(Err(Errors::OutOfBounds), database.angle_distance(StarPair(9, 0)));
// 		assert_eq!(Err(Errors::OutOfBounds), database.angle_distance(StarPair(9, 9)));
// 		assert_eq!(Err(Errors::OutOfBounds), database.angle_distance(StarPair(100, 100)));
// 	}
}
