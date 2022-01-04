//! Implementation of StarTriangle
use super::TriangleConstruct;
use super::StarTriangle;
use super::StarPair;
use super::Match;

use crate::tracking_mode::database::Database;
use crate::tracking_mode::KernelIterator;

use crate::util::units::Cartesian3D;
use crate::util::units::Equatorial;

use crate::util::list::ArrayList;
use crate::util::list::List;

use crate::util::err::Errors;
use crate::util::err::Error;



use crate::config::TrackingModeConsts;

impl <T: 'static> TriangleConstruct <T> for StarTriangle<usize>
	// where T: TrackingModeConsts, [(); T::PAIRS_MAX]: Sized
	where T: TrackingModeConsts, ArrayList<(), {T::PAIRS_MAX}> : Sized
{
	/// Finds every triangle from the provided stars which matches the database.
	/// ***DOES NOT CHECK FOR SPECULARITY!!!***
	/// # Arguments
	/// * `stars` - The stars in the image.
	/// * `database` - The database to search through.
	/// * `triangles` -  The output.
	fn find_match_triangle ( 
								&mut self,
								stars: &dyn List<Equatorial>, 
								database: &dyn Database, 
								triangles: &mut dyn List<Match<StarTriangle<usize>>>
							)
	{
		let mut iter = KernelIterator::new( stars.size() );
		
		while iter.step()
		{
			// Find the sides of the triangle.
			let side_a = stars.get(iter.i).angle_distance(stars.get(iter.j)); // i, j
			let side_b = stars.get(iter.i).angle_distance(stars.get(iter.k)); // i, k
			let side_c = stars.get(iter.j).angle_distance(stars.get(iter.k)); // j, k
			
			// Search the database for each side.
			let mut matches_a : ArrayList<StarPair<usize>, {T::PAIRS_MAX}> = ArrayList::new();
			let mut matches_b : ArrayList<StarPair<usize>, {T::PAIRS_MAX}> = ArrayList::new();
			let mut matches_c : ArrayList<StarPair<usize>, {T::PAIRS_MAX}> = ArrayList::new();
			
			database.find_close_ref(side_a, &mut matches_a);
			database.find_close_ref(side_b, &mut matches_b);
			database.find_close_ref(side_c, &mut matches_c);
			
			// If each side has found a match, search every possibility
			if matches_a.size() > 0 && matches_b.size() > 0 && matches_c.size() > 0
			{
				for ii in 0..matches_a.size()
				{
					for jj in 0..matches_b.size()
					{
						for kk in 0..matches_c.size()
						{
							// (i, j) and (i, k)  (i)
							let same_ab=StarPair::find_same(&matches_a.get(ii),&matches_b.get(jj));
							// (i, j) and (j, k)  (j)
							let same_ac=StarPair::find_same(&matches_a.get(ii),&matches_c.get(kk));
							// (i, k) and (j, k)  (k)
							let same_bc=StarPair::find_same(&matches_b.get(jj),&matches_c.get(kk));
							
							// A triangle can be formed.
							if same_ab.is_some() && same_ac.is_some() && 
								same_bc.is_some( ) && !triangles.is_full()
							{
								let input  = StarTriangle(iter.i, iter.j, iter.k);
								let output = StarTriangle(
									same_ab.unwrap(), 
									same_ac.unwrap(), 
									same_bc.unwrap());
								let found = Match{input: input, output: output, weight: 1.0};
								triangles.push_back(found)
									.expect("array: Triangles, was just found to not be full.");
							}
						}
					}
				}
			}
		}
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
			return Err(Errors::NoMatch);
		}
		
		return Ok(StarTriangle(a.unwrap(), b.unwrap(), c.unwrap()));
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
			
			return Ok(StarTriangle(a, b, c));
		}
		
		return Err(Errors::NoMatch);
	}
}


impl StarTriangle<Equatorial>
{
	/// Converts the values to cartesian coordinates.
	pub fn to_cartesian3 ( &self ) -> StarTriangle<Cartesian3D>
	{
		return StarTriangle
		(
			self.0.to_cartesian3(),
			self.1.to_cartesian3(),
			self.2.to_cartesian3()
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
	use crate::tracking_mode::TriangleConstruct;
	use crate::tracking_mode::StarTriangle;
	use crate::tracking_mode::StarPair;
	use crate::tracking_mode::Match;
	use crate::tracking_mode::database::MockDatabase;
	
	use crate::util::units::Equatorial;
	use crate::util::units::Radians;
	use crate::util::aliases::Decimal;
	use crate::util::list::ArrayList;
	use crate::util::list::List;
	use crate::util::err::Errors;

	use crate::config::TrackingModeConsts;


	/// A mock for the sizes for the arrays, the arrays are not expected to exceed this size.
	pub struct MockConfigBig ( );
	impl TrackingModeConsts for MockConfigBig
	{
		const PAIRS_MAX       : usize = 10;
		const TRIANGLES_MAX   : usize = 10;
		const SPECULARITY_MIN : Decimal = 300.0;
	}	
	


	//
	// fn find_match ( &List<Equatorial>, &Database, &mut List<StarTriangle<usize>> )
	//

	
	#[test]
	// If there is not enough stars, no triangles can be found.
	fn find_match_triangle_not_enough_stars ( )
	{
		let database = MockDatabase::new();
		let mut lst_star : Vec<Equatorial> = vec![];
		let mut lst_tri : Vec<Match<StarTriangle<usize>>> = Vec::new();
		
		let mut tri : StarTriangle<usize> = StarTriangle(0,0,0);
		// Size 0
		TriangleConstruct::<MockConfigBig>::
			find_match_triangle( &mut tri, &lst_star, &database, &mut lst_tri );
		assert_eq!(lst_tri.len(), 0);
		
		// Size 1
		lst_star.push(Equatorial{ra: Radians(0.0), dec: Radians(1.0)});
		TriangleConstruct::<MockConfigBig>::
			find_match_triangle( &mut tri, &lst_star, &database, &mut lst_tri );
		assert_eq!(lst_tri.len(), 0);

		// Size 2
		lst_star.push(Equatorial{ra: Radians(0.0), dec: Radians(1.0)});
		TriangleConstruct::<MockConfigBig>::
			find_match_triangle( &mut tri, &lst_star, &database, &mut lst_tri );
		assert_eq!(lst_tri.len(), 0);
	}
	
	
	
	#[test]
	// If there is enough stars to make a triangle, however database cant find the required angles.
	fn find_match_triangle_cant_find_database ( )
	{
		let mut database = MockDatabase::new();
		let eq = Equatorial{ra: Radians(1.2), dec: Radians(2.3)};
		let lst_star : Vec<Equatorial> = vec![eq, eq, eq];
		let mut lst_tri : Vec<Match<StarTriangle<usize>>> = Vec::new();
		
		let mut tri : StarTriangle<usize> = StarTriangle(0,0,0);
	
		database.expect_find_close_ref()
			.times(3)
			.returning(|_, _| ())
			.withf(|find, _| find.0 < 0.001);
		TriangleConstruct::<MockConfigBig>::
			find_match_triangle( &mut tri, &lst_star, &database, &mut lst_tri );
		assert_eq!(lst_tri.len(), 0);
	}
	
	
	
	
	
	
	#[test]
	// If a triangle can be formed and the database can find the required angles, but it cannot form a triangle.
	fn find_match_triangle_invalid_return_database ( )
	{
		let mut database = MockDatabase::new();
		let eq = Equatorial{ra: Radians(1.2), dec: Radians(2.3)};
		let lst_star : Vec<Equatorial> = vec![eq, eq, eq];
		let mut lst_tri : Vec<Match<StarTriangle<usize>>> = Vec::new();
		
		let mut tri : StarTriangle<usize> = StarTriangle(0,0,0);
		
		
		const STAR_PAIR_1 : StarPair<usize> = StarPair(0, 1);
		const STAR_PAIR_2 : StarPair<usize> = StarPair(1, 2);
		const STAR_PAIR_3 : StarPair<usize> = StarPair(3, 4); // needs to be 2,0
		const ALL : [StarPair<usize>; 3] = [STAR_PAIR_1, STAR_PAIR_2, STAR_PAIR_3];
		let mut db_idx = 0;
		database.expect_find_close_ref()
			.times(3)
			.returning(move |_, found| {found.push_back(ALL[db_idx]); db_idx+=1;})
			.withf(|find, _| find.0 < 0.001);
		TriangleConstruct::<MockConfigBig>::
			find_match_triangle( &mut tri, &lst_star, &database, &mut lst_tri );
		assert_eq!(lst_tri.len(), 0);
	}
	
	
	
	
	#[test]
	// Does it work?
	fn find_match_triangle_valid_single_element ( )
	{
		let mut database = MockDatabase::new();
		let eq = Equatorial{ra: Radians(1.2), dec: Radians(2.3)};
		let lst_star : Vec<Equatorial> = vec![eq, eq, eq];
		let mut lst_tri : Vec<Match<StarTriangle<usize>>> = Vec::with_capacity(10);
		
		let mut tri : StarTriangle<usize> = StarTriangle(0,0,0);
		
		
		const STAR_PAIR_1 : StarPair<usize> = StarPair(0, 1);
		const STAR_PAIR_2 : StarPair<usize> = StarPair(0, 2);
		const STAR_PAIR_3 : StarPair<usize> = StarPair(1, 2);
		const ALL : [StarPair<usize>; 3] = [STAR_PAIR_1, STAR_PAIR_2, STAR_PAIR_3];
		let mut db_idx = 0;
		database.expect_find_close_ref()
			.times(3)
			.returning(move |_, found| {found.push_back(ALL[db_idx]); db_idx+=1;})
			.withf(|find, _| find.0 < 0.001);
		TriangleConstruct::<MockConfigBig>::
			find_match_triangle( &mut tri, &lst_star, &database, &mut lst_tri );

		assert_eq!(lst_tri.len(), 1);
		
		assert_eq!(lst_tri[0].input.0, 0);
		assert_eq!(lst_tri[0].input.1, 1);
		assert_eq!(lst_tri[0].input.2, 2);
		
		assert_eq!(lst_tri[0].output.0, 0);
		assert_eq!(lst_tri[0].output.1, 1);
		assert_eq!(lst_tri[0].output.2, 2);
	}
	
	

	#[test]
	// If the array overflows, does it break?
	fn find_match_triangle_cant_fit_elements ( )
	{
		let mut database = MockDatabase::new();
		let eq = Equatorial{ra: Radians(1.2), dec: Radians(2.3)};
		let lst_star : Vec<Equatorial> = vec![eq, eq, eq];
		let mut lst_tri : ArrayList<Match<StarTriangle<usize>>, 10> = ArrayList::new();
	
		let mut tri : StarTriangle<usize> = StarTriangle(0,0,0);
	
		const STAR_PAIR_1 : StarPair<usize> = StarPair(0, 1);
		const STAR_PAIR_2 : StarPair<usize> = StarPair(0, 2);
		const STAR_PAIR_3 : StarPair<usize> = StarPair(1, 2);
		const ALL : [[StarPair<usize>; 5]; 3] = [
		[STAR_PAIR_1, STAR_PAIR_1, STAR_PAIR_1, STAR_PAIR_1, STAR_PAIR_1], 
		[STAR_PAIR_2, STAR_PAIR_2, STAR_PAIR_2, STAR_PAIR_2, STAR_PAIR_2], 
		[STAR_PAIR_3, STAR_PAIR_3, STAR_PAIR_3, STAR_PAIR_3, STAR_PAIR_3]];
		let mut db_idx = 0;
		database.expect_find_close_ref()
			.times(3)
			.returning(move |_, found| {
				for e in ALL[db_idx] {found.push_back(e);} db_idx+=1;
			})
			.withf(|find, _| find.0 < 0.001);
		TriangleConstruct::<MockConfigBig>::
			find_match_triangle( &mut tri, &lst_star, &database, &mut lst_tri );
	
		assert_eq!(lst_tri.size(), 10);
	}





















	//
	// fn search_database ( &self, database: &dyn Database ) -> Error<StarTriangle<Equatorial>>
	//
	
	#[test]
	fn test_search_database_invalid ( )
	{
		let triangle = StarTriangle(1, 1, 1);
		let mut database = MockDatabase::new();
		database.expect_find_star().times(3).returning(|_| Err(Errors::OutOfBounds));
		assert!(triangle.search_database(&database).is_err());
	}

	#[test]
	fn test_search_database (  )
	{
		let triangle = StarTriangle(1, 2, 3);
		let mut database = MockDatabase::new();
		database.expect_find_star().times(3)
			.returning(|i| Ok(Equatorial{ra: Radians(i as Decimal), dec: Radians(i as Decimal + 1.0)}));
		
		let wrapped = triangle.search_database(&database);
		let triangle_eq = wrapped.expect("Returned Err()");
		assert_eq!(triangle_eq.0.ra,  Radians(1.0));
		assert_eq!(triangle_eq.0.dec, Radians(2.0));
		
		assert_eq!(triangle_eq.1.ra,  Radians(2.0));
		assert_eq!(triangle_eq.1.dec, Radians(3.0));
		
		assert_eq!(triangle_eq.2.ra,  Radians(3.0));
		assert_eq!(triangle_eq.2.dec, Radians(4.0));
	}


	//
	// fn search_list ( &self, database: &dyn Database ) -> Error<StarTriangle<Equatorial>>
	//
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


	//
	// fn to_cartesian3 ( &self ) -> StarTriangle<Cartesian3D>
	//
	#[test]
	fn test_to_cartesian3 ( )
	{
		let eq = Equatorial{ra: Radians(0.2), dec: Radians(3.4)};
		let triangle = StarTriangle(eq, eq, eq);
		let cart = triangle.to_cartesian3();
		assert_eq!(cart.0, eq.to_cartesian3());
		assert_eq!(cart.1, eq.to_cartesian3());
		assert_eq!(cart.2, eq.to_cartesian3());
	}
}