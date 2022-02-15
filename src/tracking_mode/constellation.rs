//! Implementation of Constellation.
// use crate::util::units::Cartesian3D;
use super::SpecularityConstruct;
use super::TriangleConstruct;
use super::PyramidConstruct;
use super::Constellation;
use super::StarTriangle;
use super::StarPyramid;
use super::Match;
use super::Specularity;
use crate::tracking_mode::database::Database;

use crate::util::units::Equatorial;
use crate::util::list::List;
use crate::util::list::ArrayList;
use crate::util::list::ListIterator;
use crate::util::err::Error;

use crate::config::TrackingModeConsts;

impl Constellation
{
/// Wrapper to simplify the imports.
/// Essentialy new()
pub fn find_constellation <T: TrackingModeConsts> ( 
	stars : &dyn List<Equatorial>, database : &dyn Database ) -> Constellation
	
	where T: 'static + TrackingModeConsts, 
	ArrayList<(), {T::PAIRS_MAX}> : Sized, 
	ArrayList<(), {T::TRIANGLES_MAX}> : Sized
{
	let mut gen_tri : StarTriangle<usize> = StarTriangle(0, 0, 0);
	let mut gen_pyr : StarPyramid<usize> = StarPyramid(0, 0, 0, 0);
	let mut gen_spec = Specularity::Ignore;
	
	
	return Constellation::new::<T>(stars, database, &mut gen_tri, &mut gen_pyr, &mut gen_spec);
}


/// Creates unique sets of TrackingMode's from the location of the stars on an equatorial plane.
/// These are then compared with the database and the accurate sets from the database will be returned.
/// # Arguments
/// * `stars` - The list of stars in order of magnitude (descending).
///
/// # Returns
/// The triangle of the image and the triangle of the database.
pub fn new <T: TrackingModeConsts> ( 
									stars    : &dyn List<Equatorial>, 
									database : &dyn Database, 
									gen_tri  : &mut dyn TriangleConstruct<T>,
									gen_pyr  : &mut dyn PyramidConstruct<T>,
									gen_spec : &mut dyn SpecularityConstruct<T>
								) -> Constellation
	where T: 'static + TrackingModeConsts, 
	ArrayList<(), {T::PAIRS_MAX}> : Sized, 
	ArrayList<(), {T::TRIANGLES_MAX}> : Sized
// where T: TrackingModeConsts, [(); T::PAIRS_MAX]: Sized
{
	// Not enough stars, instant fail.
	if stars.size() < 3
	{
		return Constellation::None;
	}
	// Enough to make a triangle constellation.
	else if stars.size() >= 3
	{
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~	 Attempt to find stars matches.	~~~~~~~~~~~~~~~~~~~~
		let mut triangles: 
			ArrayList<Match<StarTriangle<usize>>,{T::TRIANGLES_MAX}> = ArrayList::new();
		gen_tri.find_match_triangle ( stars, database, &mut triangles );

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~	 Loops through all matches.		~~~~~~~~~~~~~~~~~~~~
		let iterator: ListIterator<Match<StarTriangle<usize>>> = ListIterator::new(&triangles);
		for iter in iterator
		{
			let input : Error<StarTriangle<Equatorial>> = iter.input.search_list(stars);
			let output : Error<StarTriangle<Equatorial>> = iter.output.search_database(database);
			// ~~~~~~~~~~~~~~~~~~~~~~~~~	 A valid match was found.		~~~~~~~~~~~~~~~~~~~~
			if input.is_ok() && output.is_ok()
			{
				let input = input.unwrap();
				let output = output.unwrap();
				
				// ~~~~~~~~~~~~~~~~~~~~~	 The speculariy in/out match	~~~~~~~~~~~~~~~~~~~~
				if gen_spec.same(&input.to_cartesian3(), &output.to_cartesian3())
				{
					// ~~~~~~~~~~~~~~~~~	Only a triangle can be formed.	~~~~~~~~~~~~~~~~~~~~
					if stars.size() == 3
					{ 
						return Constellation::Triangle(
							Match{input: input, output: output, weight: 1.0});
					}
					
					// ~~~~~~~~~~~~~~~~~	Pyramid can be formed.			~~~~~~~~~~~~~~~~~~~~
					else if 3 < stars.size()
					{ 
						let result = gen_pyr.find_pilot(stars, database, iter.input, iter.output);
						// ~~~~~~~~~~~~~	A match is found.				~~~~~~~~~~~~~~~~~~~~
						if let Ok(found) = result
						{
							// ~~~~~~~~~	Get the star from the database.	~~~~~~~~~~~~~~~~~~~~
							if found.input < stars.size()
							{
								let pil_in = stars.get(found.input);
								let pil_out = database.find_star(found.output);
								
								if let Ok(out) = pil_out
								{
									let pyr_in  = StarPyramid(input.0, input.1, input.2, pil_in);
									let pyr_out = StarPyramid(output.0, output.1, output.2, out);
									return Constellation::Pyramid(
										Match{input: pyr_in, output: pyr_out, weight: 1.0});
								}
							}
						}
					}
				}
			}
		}
	}
	return Constellation::None;
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
	use std::mem;

	use super::Constellation;
	use super::StarTriangle;
	use super::Match;
	use crate::tracking_mode::MockSpecularityConstruct;
	use crate::tracking_mode::database::MockDatabase;
	use crate::tracking_mode::MockTriangleConstruct;
	use crate::tracking_mode::MockPyramidConstruct;
	
	use crate::util::units::Equatorial;
	use crate::util::units::Radians;
	use crate::util::aliases::Decimal;
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
	
	
	
	fn compare_triangle_eq ( 
		input: StarTriangle<Equatorial>, 
		output: StarTriangle<Equatorial> )
	{
		assert!((output.0.ra  - input.0.ra).0.abs()  < 0.01);
		assert!((output.0.dec - input.0.dec).0.abs() < 0.01);
		assert!((output.1.ra  - input.1.ra).0.abs()  < 0.01);
		assert!((output.1.dec - input.1.dec).0.abs() < 0.01);
		assert!((output.2.ra  - input.2.ra).0.abs()  < 0.01);
		assert!((output.2.dec - input.2.dec).0.abs() < 0.01);	
	}
	

	
	// use super::StarTriangle;
	
	#[test]
	// If there is less than 3 stars input, None should be returned.
	fn test_new_invalid_number_stars ( )
	{
		let a = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let mut stars : Vec<Equatorial> = vec![];
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_p = MockPyramidConstruct::new();
		let mut mock_s = MockSpecularityConstruct::new();
		let mock_d = MockDatabase::new();
		
		let mut tri=Constellation::new::<MockConfigBig>(&stars, &mock_d, &mut mock_t, &mut mock_p, &mut mock_s);
		assert_eq! ( mem::discriminant(&tri),  mem::discriminant(&Constellation::None) );
		
		stars = vec![a];
		tri=Constellation::new::<MockConfigBig>(&stars, &mock_d, &mut mock_t, &mut mock_p, &mut mock_s);
		assert_eq! ( mem::discriminant(&tri), mem::discriminant(&Constellation::None) );
		
		stars = vec![a, a];
		tri=Constellation::new::<MockConfigBig>(&stars, &mock_d, &mut mock_t, &mut mock_p, &mut mock_s);
		assert_eq! ( mem::discriminant(&tri), mem::discriminant(&Constellation::None) );
	}
	



	#[test]
	// If there is 3 stars and a triangle cannot be formed, None should be returned.
	fn test_new_invalid_3_stars ( )
	{
		let a = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let stars : Vec<Equatorial> = vec![a, a, a];
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_p = MockPyramidConstruct::new();
		let mut mock_s = MockSpecularityConstruct::new();
		let mock_d = MockDatabase::new();
		mock_t.expect_find_match_triangle().times(1).returning(|_, _, _| ());
		
		let tri = Constellation::new::<MockConfigBig>(&stars, &mock_d, &mut mock_t, &mut mock_p, &mut mock_s);
		assert_eq! ( mem::discriminant(&tri), mem::discriminant(&Constellation::None) );
					 
		// assert_eq!(mock_t.calls, 1);
	}



	#[test]
	// If there is 3 stars and a triangle can be formed, a triangle should be returned.
	fn test_new_valid_3_stars ( )
	{
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_p = MockPyramidConstruct::new();
		let mut mock_d = MockDatabase::new();
		let mut mock_s = MockSpecularityConstruct::new();
		
		
		// These are points from the image referenced by TRI_IN
		let a = Equatorial{ra: Radians(1.0), dec: Radians(0.1)};
		let b = Equatorial{ra: Radians(2.0), dec: Radians(0.2)};
		let c = Equatorial{ra: Radians(3.0), dec: Radians(0.3)};
		let stars : Vec<Equatorial> = vec![a, b, c];
		
		// The software function will call find_match_triangle;
		// It will return the triangle from the image (TRI_IN) with the matched database triangle (TRI_OUT).
		// This will then be converted to the equatorial coordinates from the reference locations.
		static TRI_IN  : StarTriangle<usize> = StarTriangle( 0, 1, 2 );
		static TRI_OUT : StarTriangle<usize> = StarTriangle( 0, 1, 2 );
		static MATCH_TRI : Match<StarTriangle<usize>> = 
			Match{input: TRI_IN, output: TRI_OUT, weight: 1.0};
		
		mock_t.expect_find_match_triangle().times(1).returning(|_, _, out| out.push_back(MATCH_TRI).expect(""));
		mock_s.expect_same().times(1).returning(|_, _| return true); // The triangles are the correct orientation.
		
		// mock_g.expect_find_match_triangle().times(1)
		// 		.returning(|_, _, triangles| (*triangles).push_back(MATCH_TRI));
		
		// The output for converting the TRI_OUT to equatorial (searching database).
		static STAR_OUT : [Equatorial ; 3] =
			[Equatorial{ra: Radians(111.2), dec: Radians(222.3)},
			Equatorial{ra: Radians(11.2), dec: Radians(22.3)},
			Equatorial{ra: Radians(1.2), dec: Radians(2.3)}];
		mock_d.expect_find_star().times(3).returning(|i| Ok(STAR_OUT[i]));
		
		let tri = &Constellation::new::<MockConfigBig>(&stars, &mock_d, &mut mock_t, &mut mock_p, &mut mock_s);
		if let Constellation::Triangle(t) = tri
		{
			compare_triangle_eq(t.input, StarTriangle(stars[0], stars[1], stars[2]));
			compare_triangle_eq(t.output, StarTriangle(STAR_OUT[0], STAR_OUT[1], STAR_OUT[2]));
			// assert_eq!(mock_t.calls, 1);
		}
		else
		{
			panic!("Invalid Triangle: {:?}", tri);
		}
	}


	#[test]
	// If there is 3 stars and a triangle can be formed, but the triangle is flipped, a triangle should not be returned.
	fn test_new_invalid_specular_3_stars ( )
	{
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_p = MockPyramidConstruct::new();
		let mut mock_d = MockDatabase::new();
		let mut mock_s = MockSpecularityConstruct::new();
		
		
		// These are points from the image referenced by TRI_IN
		let a = Equatorial{ra: Radians(1.0), dec: Radians(0.1)};
		let b = Equatorial{ra: Radians(2.0), dec: Radians(0.2)};
		let c = Equatorial{ra: Radians(3.0), dec: Radians(0.3)};
		let stars : Vec<Equatorial> = vec![a, b, c];
		
		// The software function will call find_match_triangle;
		// It will return the triangle from the image (TRI_IN) with the matched database triangle (TRI_OUT).
		// This will then be converted to the equatorial coordinates from the reference locations.
		static TRI_IN  : StarTriangle<usize> = StarTriangle( 0, 1, 2 );
		static TRI_OUT : StarTriangle<usize> = StarTriangle( 0, 1, 2 );
		static MATCH_TRI : Match<StarTriangle<usize>> = 
			Match{input: TRI_IN, output: TRI_OUT, weight: 1.0};
		
		mock_t.expect_find_match_triangle().times(1).returning(|_,_,out| out.push_back(MATCH_TRI).expect(""));
		mock_s.expect_same().times(1).returning(|_, _| return false); // The triangles are the correct orientation.
		
		// mock_g.expect_find_match_triangle().times(1)
		// 		.returning(|_, _, triangles| (*triangles).push_back(MATCH_TRI));
		
		// The output for converting the TRI_OUT to equatorial (searching database).
		static STAR_OUT : [Equatorial ; 3] =
			[Equatorial{ra: Radians(111.2), dec: Radians(222.3)},
			Equatorial{ra: Radians(11.2), dec: Radians(22.3)},
			Equatorial{ra: Radians(1.2), dec: Radians(2.3)}];
		mock_d.expect_find_star().times(3).returning(|i| Ok(STAR_OUT[i]));
		
		let tri = &Constellation::new::<MockConfigBig>(&stars, &mock_d, &mut mock_t, &mut mock_p, &mut mock_s);
		if let Constellation::Triangle(_t) = tri
		{
			panic!("Triangle was formed");
		}
	}











	#[test]
	// If there is 4 stars and a triangle cannot be formed, None should be returned.
	fn test_new_invalid_4_stars ( )
	{
		let a = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let stars : Vec<Equatorial> = vec![a, a, a, a];
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_p = MockPyramidConstruct::new();
		let mock_d = MockDatabase::new();
		let mut mock_s = MockSpecularityConstruct::new();
		
		mock_t.expect_find_match_triangle().times(1).returning(|_, _, _| ());
		
		let tri = Constellation::new::<MockConfigBig>(&stars, &mock_d, &mut mock_t, &mut mock_p, &mut mock_s);
		assert_eq! ( mem::discriminant(&tri), mem::discriminant(&Constellation::None) );
	}




	#[test]
	// If there is 4 stars and a triangle is found but not specular, None should be returned.
	fn test_new_invalid_specular_4_stars ( )
	{
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_p = MockPyramidConstruct::new();
		let mut mock_d = MockDatabase::new();
		let mut mock_s = MockSpecularityConstruct::new();
		
		
		// These are points from the image referenced by TRI_IN
		let a = Equatorial{ra: Radians(1.0), dec: Radians(0.1)};
		let b = Equatorial{ra: Radians(2.0), dec: Radians(0.2)};
		let c = Equatorial{ra: Radians(3.0), dec: Radians(0.3)};
		let d = Equatorial{ra: Radians(4.0), dec: Radians(0.4)};
		let stars : Vec<Equatorial> = vec![a, b, c, d];
		
		// The software function will call find_match_triangle;
		// It will return the triangle from the image (TRI_IN) with the matched database triangle (TRI_OUT).
		// This will then be converted to the equatorial coordinates from the reference locations.
		static TRI_IN  : StarTriangle<usize> = StarTriangle( 0, 1, 2 );
		static TRI_OUT : StarTriangle<usize> = StarTriangle( 0, 1, 2 );
		static MATCH_TRI : Match<StarTriangle<usize>> = 
			Match{input: TRI_IN, output: TRI_OUT, weight: 1.0};
		
		mock_t.expect_find_match_triangle().times(1).returning(|_,_,out| out.push_back(MATCH_TRI).expect(""));
		mock_s.expect_same().times(1).returning(|_, _| return false); // The triangles are the correct orientation.
		// The output for converting the TRI_OUT to equatorial (searching database).
		static STAR_OUT : [Equatorial ; 3] =
			[Equatorial{ra: Radians(111.2), dec: Radians(222.3)},
			Equatorial{ra: Radians(11.2), dec: Radians(22.3)},
			Equatorial{ra: Radians(1.2), dec: Radians(2.3)}];
		mock_d.expect_find_star().times(3).returning(|i| Ok(STAR_OUT[i]));
		
		let tri = &Constellation::new::<MockConfigBig>(&stars, &mock_d, &mut mock_t, &mut mock_p, &mut mock_s);
		if let Constellation::Triangle(_t) = tri
		{
			panic!("Triangle was formed");
		}
	}
	


	#[test]
	// If there is 4 stars and a triangle is found and is specular, Pyramid should be returned.
	fn test_new_valid_4_stars ( )
	{
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_p = MockPyramidConstruct::new();
		let mut mock_d = MockDatabase::new();
		let mut mock_s = MockSpecularityConstruct::new();
		
		// These are points from the image referenced by TRI_IN
		let a = Equatorial{ra: Radians(1.0), dec: Radians(0.1)};
		let b = Equatorial{ra: Radians(2.0), dec: Radians(0.2)};
		let c = Equatorial{ra: Radians(3.0), dec: Radians(0.3)};
		let d = Equatorial{ra: Radians(4.0), dec: Radians(0.4)};
		let stars : Vec<Equatorial> = vec![a, b, c, d];
		
		// The software function will call find_match_triangle;
		// It will return the triangle from the image (TRI_IN) with the matched database triangle (TRI_OUT).
		// This will then be converted to the equatorial coordinates from the reference locations.
		static TRI_IN  : StarTriangle<usize> = StarTriangle( 0, 1, 2 );
		static TRI_OUT : StarTriangle<usize> = StarTriangle( 0, 1, 2 );
		static MATCH_TRI : Match<StarTriangle<usize>> = 
			Match{input: TRI_IN, output: TRI_OUT, weight: 1.0};
		
		mock_t.expect_find_match_triangle().times(1).returning(|_,_,out| out.push_back(MATCH_TRI).expect(""));
		mock_s.expect_same().times(1).returning(|_,_| return true); // The triangles are the correct orientation.
		// The output for converting the TRI_OUT to equatorial (searching database).
		static STAR_OUT : [Equatorial ; 4] =
			[Equatorial{ra: Radians(111.2), dec: Radians(222.3)},
			Equatorial{ra: Radians(11.2), dec: Radians(22.3)},
			Equatorial{ra: Radians(1.2), dec: Radians(2.3)},
			Equatorial{ra: Radians(0.2), dec: Radians(0.3)}];
		mock_d.expect_find_star().times(4).returning(|i| Ok(STAR_OUT[i]));
		
		mock_p.expect_find_pilot()
			.times(1)
			.returning(|_, _, _, _|return Ok(Match{input:3, output:3, weight: 1.0}));
		
		let pyr = &Constellation::new::<MockConfigBig>(&stars, &mock_d, &mut mock_t, &mut mock_p, &mut mock_s);
		if let Constellation::Pyramid(p) = pyr
		{
			assert_eq!(p.input.0, a);
			assert_eq!(p.input.1, b);
			assert_eq!(p.input.2, c);
			assert_eq!(p.input.3, d);
			
			assert_eq!(p.output.0.ra.0, 111.2);
			assert_eq!(p.output.1.ra.0, 11.2);
			assert_eq!(p.output.2.ra.0, 1.2);
			assert_eq!(p.output.3.ra.0, 0.2);
			
			assert_eq!(p.output.0.dec.0, 222.3);
			assert_eq!(p.output.1.dec.0, 22.3);
			assert_eq!(p.output.2.dec.0, 2.3);
			assert_eq!(p.output.3.dec.0, 0.3);
		}
		else
		{
			panic!("FAILED TO FORM PYRAMID: {:?}", pyr);
		}
	}


	#[test]
	// If there is more than 4 stars,
	// If any match is found, it should be returned, even if one fails specularity.
	fn test_new_valid_pyramid_iterate_specularity_stars (  )
	{
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_p = MockPyramidConstruct::new();
		let mut mock_d = MockDatabase::new();
		let mut mock_s = MockSpecularityConstruct::new();
		
		// These are points from the image referenced by TRI_IN
		let a = Equatorial{ra: Radians(1.0), dec: Radians(0.1)};
		let b = Equatorial{ra: Radians(2.0), dec: Radians(0.2)};
		let c = Equatorial{ra: Radians(3.0), dec: Radians(0.3)};
		let d = Equatorial{ra: Radians(4.0), dec: Radians(0.4)};
		let e = Equatorial{ra: Radians(5.0), dec: Radians(0.5)};
		let stars : Vec<Equatorial> = vec![a, b, c, d, e];
		
		// The software function will call find_match_triangle;
		// It will return the triangle from the image (TRI_IN) with the matched database triangle (TRI_OUT).
		// This will then be converted to the equatorial coordinates from the reference locations.
		static TRI_IN  : StarTriangle<usize> = StarTriangle( 0, 1, 2 );
		static TRI_OUT : StarTriangle<usize> = StarTriangle( 0, 1, 2 );
		static MATCH_TRI : Match<StarTriangle<usize>> = 
			Match{input: TRI_IN, output: TRI_OUT, weight: 1.0};
		
		mock_t.expect_find_match_triangle().times(1).returning(|_,_,out| {out.push_back(MATCH_TRI); out.push_back(MATCH_TRI);});
		
		let specularity = [false, true];
		let mut i = 0;
		mock_s.expect_same().times(2).returning(move |_,_| {let spec = specularity[i]; i+=1; return spec});
		// The output for converting the TRI_OUT to equatorial (searching database).
		static STAR_OUT : [Equatorial ; 4] =
			[Equatorial{ra: Radians(111.2), dec: Radians(222.3)},
			Equatorial{ra: Radians(11.2), dec: Radians(22.3)},
			Equatorial{ra: Radians(1.2), dec: Radians(2.3)},
			Equatorial{ra: Radians(0.2), dec: Radians(0.3)}];
		mock_d.expect_find_star().times(7).returning(|i| Ok(STAR_OUT[i]));
		
		mock_p.expect_find_pilot()
			.times(1)
			.returning(|_, _, _, _|return Ok(Match{input:3, output:3, weight: 1.0}));
		
		let pyr = &Constellation::new::<MockConfigBig>(&stars, &mock_d, &mut mock_t, &mut mock_p, &mut mock_s);
		if let Constellation::Pyramid(p) = pyr
		{
			assert_eq!(p.input.0, a);
			assert_eq!(p.input.1, b);
			assert_eq!(p.input.2, c);
			assert_eq!(p.input.3, d);
			
			assert_eq!(p.output.0.ra.0, 111.2);
			assert_eq!(p.output.1.ra.0, 11.2);
			assert_eq!(p.output.2.ra.0, 1.2);
			assert_eq!(p.output.3.ra.0, 0.2);
		}
		else
		{
			panic!("FAILED TO FORM PYRAMID: {:?}", pyr);
		}
	}




	#[test]
	// If there is more than 4 stars,
	// If any match is found, it should be returned, even if one fails specularity.
	fn test_new_valid_pyramid_iterate_find_pilot_stars (  )
	{
		let mut mock_t = MockTriangleConstruct::new();
		let mut mock_p = MockPyramidConstruct::new();
		let mut mock_d = MockDatabase::new();
		let mut mock_s = MockSpecularityConstruct::new();
		
		// These are points from the image referenced by TRI_IN
		let a = Equatorial{ra: Radians(1.0), dec: Radians(0.1)};
		let b = Equatorial{ra: Radians(2.0), dec: Radians(0.2)};
		let c = Equatorial{ra: Radians(3.0), dec: Radians(0.3)};
		let d = Equatorial{ra: Radians(4.0), dec: Radians(0.4)};
		let e = Equatorial{ra: Radians(5.0), dec: Radians(0.5)};
		let stars : Vec<Equatorial> = vec![a, b, c, d, e];
		
		// The software function will call find_match_triangle;
		// It will return the triangle from the image (TRI_IN) with the matched database triangle (TRI_OUT).
		// This will then be converted to the equatorial coordinates from the reference locations.
		static TRI_IN  : StarTriangle<usize> = StarTriangle( 0, 1, 2 );
		static TRI_OUT : StarTriangle<usize> = StarTriangle( 0, 1, 2 );
		static MATCH_TRI : Match<StarTriangle<usize>> = 
			Match{input: TRI_IN, output: TRI_OUT, weight: 1.0};
		
		mock_t.expect_find_match_triangle()
			.times(1)
			.returning(|_,_,out| {out.push_back(MATCH_TRI); out.push_back(MATCH_TRI);});
		

		mock_s.expect_same().times(2).returning(|_,_| return true);
		// The output for converting the TRI_OUT to equatorial (searching database).
		static STAR_OUT : [Equatorial ; 4] =
			[Equatorial{ra: Radians(111.2), dec: Radians(222.3)},
			Equatorial{ra: Radians(11.2), dec: Radians(22.3)},
			Equatorial{ra: Radians(1.2), dec: Radians(2.3)},
			Equatorial{ra: Radians(0.2), dec: Radians(0.3)}];
		mock_d.expect_find_star().times(7).returning(|i| Ok(STAR_OUT[i]));
		
		let pyramid = [Err(Errors::NoMatch), Ok(Match{input:3, output:3, weight: 1.0})];
		let mut i = 0;
		mock_p.expect_find_pilot()
			.times(2)
			.returning(move |_, _, _, _| {i+=1; return pyramid[i - 1]});
		
		let pyr = &Constellation::new::<MockConfigBig>(&stars, &mock_d, &mut mock_t, &mut mock_p, &mut mock_s);
		if let Constellation::Pyramid(p) = pyr
		{
			assert_eq!(p.input.0, a);
			assert_eq!(p.input.1, b);
			assert_eq!(p.input.2, c);
			assert_eq!(p.input.3, d);
			
			assert_eq!(p.output.0.ra.0, 111.2);
			assert_eq!(p.output.1.ra.0, 11.2);
			assert_eq!(p.output.2.ra.0, 1.2);
			assert_eq!(p.output.3.ra.0, 0.2);
		}
		else
		{
			panic!("FAILED TO FORM PYRAMID: {:?}", pyr);
		}
	}
}