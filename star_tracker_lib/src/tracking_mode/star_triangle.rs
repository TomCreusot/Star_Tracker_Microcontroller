//! Implementation of StarTriangle
use crate::core_include::*;

use super::StarTriangle;
use super::StarPair;

use crate::tracking_mode::database::Database;

use crate::util::units::Vector3;
use crate::util::units::Equatorial;

use crate::util::list::List;
// use crate::util::list::ArrayList;

use crate::util::err::Errors;
use crate::util::err::Error;



impl StarTriangle<usize>
{

	/// Attempts to construct a StarTriangle from 3 StarPairs.
	/// A StarTriangle can be formed if the StarPairs have a total of 3 different values and loop.
	/// # Arguments
	/// * `pair_a` - The first star pair.
	/// * `pair_b` - The second star pair.
	/// * `pair_c` - The third star pair.
	pub fn construct_triangle (
					pair_a: StarPair<usize>,
					pair_b: StarPair<usize>,
					pair_c: StarPair<usize>,
				) -> Option<StarTriangle<usize>>
	{
		// (i, j) and (i, k)  (i)
		let same_ab = StarPair::find_same(pair_a, pair_b);
		
		if let Some(ab) = same_ab
		{
			// (i, j) and (j, k)  (j)
			let same_ac = StarPair::find_same(pair_a, pair_c);
			
			if let Some(ac) = same_ac
			{
				// (i, k) and (j, k)  (k)
				let same_bc = StarPair::find_same(pair_b, pair_c);

				// A triangle can be formed.
				if let Some(bc) = same_bc
				{
					if ab != ac && ab != bc && ac != bc
					{
						return Option::Some(StarTriangle(ab, ac, bc));
					}
				}
			}
		}
		return Option::None;
	}
}



impl StarTriangle<usize>
{
	/// Tries to convert self into an equatorial StarTriangle by finding the stars in the database.
	/// # Arguments
	/// * `database` - The database to reference.
	/// # Returns
	/// Err(()) if element cannot be found, database triangle if can be found.
	pub fn search_database ( &self, database: &dyn Database ) -> Error<StarTriangle<Equatorial>>
	{
		let a = database.find_star(self.0);
		let b = database.find_star(self.1);
		let c = database.find_star(self.2);

		if a.is_err() || b.is_err() || c.is_err()
		{
			return Result::Err(Errors::NoMatch);
		}

		return Result::Ok(StarTriangle(a.unwrap(), b.unwrap(), c.unwrap()));
	}


	/// Tries to convert self into an equatorial by finding the elements in the provided list.
	/// # Arguments
	/// * `list` - The list to lookup.
	/// # Returns
	/// Err(()) if the element cannot be found, list triangle if can be found.
	pub fn search_list ( &self, list: &dyn List<Equatorial> ) -> Error<StarTriangle<Equatorial>>
	{
		if self.0 < list.size() && self.1 < list.size() && self.2 < list.size()
		{
			let a = list.get(self.0);
			let b = list.get(self.1);
			let c = list.get(self.2);

			return Result::Ok(StarTriangle(a, b, c));
		}

		return Result::Err(Errors::NoMatch);
	}



	/// Finds if the triangle contains the particular star.
	/// # Arguments
	/// * `star` - The star index to be matched to self.
	/// # Returns
	/// True if the triangle contains the star.
	///
	/// ```
	/// use star_tracker_lib::tracking_mode::StarTriangle;
	///	let triangle = StarTriangle(0, 1, 2);
	/// assert!(triangle.has(0));
	/// assert!(triangle.has(1));
	/// assert!(triangle.has(2));
	/// ```
	pub fn has ( &self, star: usize ) -> bool
	{
		return self.0 == star || self.1 == star || self.2 == star;
	}
}


impl StarTriangle<Equatorial>
{
	/// Converts the values to cartesian coordinates.
	pub fn to_vector3 ( &self ) -> StarTriangle<Vector3>
	{
		return StarTriangle
		(
			self.0.to_vector3(),
			self.1.to_vector3(),
			self.2.to_vector3()
		);
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

	use crate::tracking_mode::StarTriangle;
	use crate::tracking_mode::StarPair;
	use crate::tracking_mode::database::MockDatabase;

	use crate::util::units::Equatorial;
	use crate::util::units::Radians;
	use crate::util::aliases::Decimal;
	use crate::util::err::Errors;


//###############################################################################################//
//
//										Construct Triangle
//										<usize>
// 	pub fn construct_triangle (
// 								pair_a: StarPair<usize>,
// 								pair_b: StarPair<usize>,
// 								pair_c: StarPair<usize>,
// 							) -> Option<StarTriangle<usize>>
//
//###############################################################################################//

	#[test]
	// Each provided star pair must have a link.
	fn test_construct_triangle_no_match ( )
	{
		let a = StarPair(0, 0);
		let b = StarPair(1, 1);
		let c = StarPair(2, 2);
		let d = StarPair(0, 1);

		// None are matching.
		assert_eq!(None, StarTriangle::construct_triangle(a, b, c));

		// 2 are matching.
		assert_eq!(None, StarTriangle::construct_triangle(a, b, d));
		assert_eq!(None, StarTriangle::construct_triangle(a, d, b));
		assert_eq!(None, StarTriangle::construct_triangle(d, a, b));
	}



	#[test]
	// If there is less than 3 stars in the provided 3 star pairs, it is invalid.
	fn test_construct_triangle_less_than_3_elements ( )
	{
		let a = StarPair(0, 0);
		let b = StarPair(0, 1);
		let c = StarPair(0, 2);

		// All star pairs have the same if every value is the same.
		assert_eq!(None, StarTriangle::construct_triangle(a, a, a));
		// All star pairs have the same if they all start with the same value.
		assert_eq!(None, StarTriangle::construct_triangle(a, b, c));
	}


	#[test]
	// If the input is valid, the StarTriangle provided must be in this sequence:
	// a-b, a-c, b-c where a-b is the star similar between a and b.
	fn test_construct_triangle ( )
	{
		let a = StarPair(0, 1);
		let b = StarPair(1, 2);
		let c = StarPair(2, 0);

		assert_eq!(Some(StarTriangle(1, 0, 2)), StarTriangle::construct_triangle(a, b, c));
		assert_eq!(Some(StarTriangle(0, 1, 2)), StarTriangle::construct_triangle(a, c, b));
		assert_eq!(Some(StarTriangle(1, 2, 0)), StarTriangle::construct_triangle(b, a, c));
		assert_eq!(Some(StarTriangle(2, 1, 0)), StarTriangle::construct_triangle(b, c, a));
		assert_eq!(Some(StarTriangle(0, 2, 1)), StarTriangle::construct_triangle(c, a, b));
		assert_eq!(Some(StarTriangle(2, 0, 1)), StarTriangle::construct_triangle(c, b, a));
	}



//###############################################################################################//
//
//										Search Database
//										<usize>
// fn search_database ( &self, database: &dyn Database )-> Error<StarTriangle<Equatorial>>
//
//###############################################################################################//


	#[test]
	// If the index is out of bounds, error should be produced.
	fn test_search_database_invalid ( )
	{
		let triangle = StarTriangle(1, 1, 1);
		let mut database = MockDatabase::new();
		database.expect_find_star().times(3).returning(|_| Err(Errors::OutOfBounds));
		assert!(triangle.search_database(&database).is_err());
	}

	#[test]
	//
	fn test_search_database ( )
	{
		let triangle = StarTriangle(1, 2, 3);
		let mut database = MockDatabase::new();
		database.expect_find_star().times(3)
			.returning(|i|
				Ok(Equatorial{ra: Radians(i as Decimal), dec: Radians(i as Decimal + 1.0)}));

		let wrapped = triangle.search_database(&database);
		let triangle_eq = wrapped.expect("Returned Err()");
		assert_eq!(triangle_eq.0.ra,  Radians(1.0));
		assert_eq!(triangle_eq.0.dec, Radians(2.0));

		assert_eq!(triangle_eq.1.ra,  Radians(2.0));
		assert_eq!(triangle_eq.1.dec, Radians(3.0));

		assert_eq!(triangle_eq.2.ra,  Radians(3.0));
		assert_eq!(triangle_eq.2.dec, Radians(4.0));
	}


//###############################################################################################//
//
//										Search List
//										<usize>
// fn search_list ( &self, database: &dyn Database ) -> Error<StarTriangle<Equatorial>>
//
//###############################################################################################//

	#[test]
	fn test_search_list_invalid ( )
	{
		let triangle = StarTriangle(1, 2, 3);
		let eq = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let mut lst : Vec<Equatorial> = vec![];

		assert!(triangle.search_list(&lst).is_err());
		lst.push(eq);
		assert!(triangle.search_list(&lst).is_err());
		lst.push(eq);
		assert!(triangle.search_list(&lst).is_err());
		lst.push(eq);
		assert!(triangle.search_list(&lst).is_err());
	}




//###############################################################################################//
//
//										Has
//										<usize>
// fn has ( &self, star: usize )-> bool
//
//###############################################################################################//

	#[test]
	// If the star triangle does have the specified star.
	fn test_has_true ( )
	{
		let triangle = StarTriangle(0, 1, 2);
		assert!(triangle.has(0));
		assert!(triangle.has(1));
		assert!(triangle.has(2));
	}

	#[test]
	// If the star triangle does not have the specified star.
	fn test_has_false ( )
	{
		let triangle = StarTriangle(1, 2, 3);
		assert!(!triangle.has(0));
		assert!(!triangle.has(4));
	}






//###############################################################################################//
//
//										To Cartesian3
//										<Equatorial>
// fn to_vector3 ( &self ) -> StarTriangle<Vector3>
//
//###############################################################################################//

	#[test]
	// A simple converter function.
	// What could go wrong?
	fn test_to_vector3 ( )
	{
		let eq = Equatorial{ra: Radians(0.2), dec: Radians(3.4)};
		let triangle = StarTriangle(eq, eq, eq);
		let cart = triangle.to_vector3();
		assert_eq!(cart.0, eq.to_vector3());
		assert_eq!(cart.1, eq.to_vector3());
		assert_eq!(cart.2, eq.to_vector3());
	}
}
