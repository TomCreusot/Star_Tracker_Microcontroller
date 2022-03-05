//! Implementation for KVector
use std::ops::Range;
use std::fmt;

use super::StarDatabaseElement;
use super::KVectorSearch;
use super::KVector;

use crate::util::test::DECIMAL_PRECISION_TEST;
use crate::util::aliases::DECIMAL_PRECISION;
use crate::util::aliases::Decimal;
use crate::util::units::Radians;
use crate::util::list::List;
use crate::util::err::{Errors, Error};


impl KVector
{
	/// For Preprocessing.  
	/// Finds the values required to use a K-Vector with the given database.
	/// # Arguments
	/// * `num_bins` - The number of bins.
	/// * `sorted_database` - A database in increasing sorted order by the angular distance between the stars.
	/// # Returns
	/// A calibrated KVector to use for the current configuration.
	///
	/// # Example
	/// ```
	/// use star_tracker::tracking_mode::database::KVector;
	/// use star_tracker::tracking_mode::database::StarDatabaseElement;
	/// use star_tracker::util::test::DECIMAL_PRECISION_TEST;
	/// use star_tracker::util::test::TestEqual;
	/// use star_tracker::tracking_mode::StarPair;
	/// use star_tracker::util::aliases::Decimal;
	/// use star_tracker::util::units::Equatorial;
	/// use star_tracker::util::units::Radians;
	///
	/// let element_min = 1.23;
	/// let element_max = 10.0;
	/// let num_bins = 2;
	/// let k_vector = KVector::new(num_bins, element_min, element_max);
	/// 
	/// let machine_epsilon = DECIMAL_PRECISION_TEST;
	/// let gradient = (element_max - element_min + machine_epsilon * 2.0) / (num_bins as Decimal);
	/// let intercept = element_min - machine_epsilon;
	/// // assert!(k_vector.gradient.test_equal(&(gradient as Decimal)));
	/// // assert!(k_vector.intercept.test_equal(&(intercept as Decimal)));
	/// // assert_eq!(k_vector.min_value, Radians(element_min as Decimal));
	/// // assert_eq!(k_vector.max_value, Radians(element_max as Decimal));
	/// 
	/// let mut y = gradient * 0.0 + intercept;			// The lower bounds.
	/// 
	/// 
	/// assert!(y < element_min);						// Must be smaller than min value.
	/// assert!(element_min - y < 0.05);				// Must be close to the min value.
	/// 
	/// y = gradient * 1.0 + intercept;					// This is the middle bounds.
	/// assert!(element_min < y && y < element_max);	// Value must be greater than the smallest element.
	///
	/// // Must be in the center between the bounds.
	/// assert!((y - (element_max + element_min) / 2.0).abs() < 0.0001);
	/// 
	/// y = gradient * 2.0 + intercept;					// This is the upper bounds.
	/// assert!(element_max <= y);						// Must include all values.
	/// assert!(y - element_max < 0.05);				// Must be close to the max value.
	/// ```
	pub fn new ( num_bins: usize, min_value: Decimal, max_value: Decimal ) -> KVector
	{
		let e = DECIMAL_PRECISION;
		let grad : Decimal = (max_value - min_value + 2.0 * e) / (num_bins as Decimal);
		let int  : Decimal = min_value - e;
	
		return KVector {
			gradient:  			grad as Decimal, 
			intercept: 			int  as Decimal,
			min_value: Radians(	min_value as Decimal ),
			max_value: Radians(	max_value as Decimal ),
			num_bins:  			num_bins,
		};
	}
	
	
	/// Creates a vector containging the bounds for each bin.
	/// # Arguments
	/// * "sorted_database" - The database in sorted order to reference.
	/// # Example
	/// ```
	/// use star_tracker::tracking_mode::database::KVector;
	/// use star_tracker::tracking_mode::database::StarDatabaseElement;
	/// use star_tracker::tracking_mode::StarPair;
	/// use star_tracker::util::aliases::Decimal;
	/// use star_tracker::util::units::Equatorial;
	/// use star_tracker::util::units::Radians;
	///
	///	//             0    1    2    3    4    5    6    7    8    9     10    11    12    13    14
	///	let dec = vec![0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 3.0, 5.0, 6.0, 10.0, 11.0, 27.0, 33.0, 33.0, 34.0];
	///	let lst = convert_dec_to_star_database_element(dec.clone());
	///
	/// const NUM_BINS : usize = 5;		
	/// let mut kvec = KVector::new(NUM_BINS, dec[0] as Decimal, dec[14] as Decimal);
	/// let mut vec = kvec.generate_bins(&lst).expect("Should not fail");
	/// 
	/// // The vector specifies the bounds.
	/// // To use this, specify the lower index as inclusive and the next index as exclusive.
	/// // e.g. for an element between bin 1 and 2, it will be index 9 (inclusive) to 11 (exclusive).
	/// // 0
	/// assert_eq!(*vec.get(0).expect("?"), 0);		// value = 0
	/// 
	/// // 6.8
	/// assert_eq!(*vec.get(1).expect("?"), 9);		// value = 10
	/// 
	/// // 13.6
	/// assert_eq!(*vec.get(2).expect("?"), 11);	// value = 22
	/// 
	/// // 20.4
	/// assert_eq!(*vec.get(3).expect("?"), 11);	// value = 22, THERE IS NO ELEMENTS SO IT IS THE SAME AS 2.
	/// 
	/// // 34.0
	/// assert_eq!(*vec.get(4).expect("?"), 15);	// value = 34
	///
	///	fn convert_dec_to_star_database_element ( val: Vec<Decimal> ) -> Vec<StarDatabaseElement>
	/// {
	/// 	let mut vec : Vec<StarDatabaseElement> = Vec::with_capacity(val.len());
	/// 
	/// 	for i in 0..val.len()
	/// 	{
	///			let ptr_1 = 2;
	///			let ptr_2 = 2;
	/// 		let pair = StarPair(ptr_1, ptr_2);
	/// 		vec.push(StarDatabaseElement{pair: pair, dist: Radians(val[i])});
	/// 	}
	/// 	return vec;
	/// }
	/// ```
	pub fn generate_bins ( &self, sorted_database: &Vec<StarDatabaseElement> ) -> Error<Vec<usize>>
	{
		if sorted_database.size() < 3
		{
			return Err(Errors::InvalidSize);
		}
		
		let mut vec = Vec::with_capacity(self.num_bins);
		
		for ii in 0..self.num_bins - 1
		{
			// The value must be greater than or equal min and smaller than max.
			// let min_value = self.gradient * ii as Decimal + self.intercept;
			let max_value = self.gradient * (ii) as Decimal + self.intercept;
			
			let mut jj = 0;
			if 0 < ii
			{
				jj = vec.get(ii - 1);
			}
			while sorted_database[jj].dist.0 < max_value
			{
				jj+=1;
			}
			vec.push(jj);
		}
		vec.push(sorted_database.size());
		return Ok(vec);
	}
}














impl KVectorSearch for KVector
{
	/// Gets the index of where the value is located in the star pair list.  
	/// This may include the neigbouring bins as it is on the edge of the bin.  
	/// i.e.  
	/// If the bin tolerance is 10:    
	/// [1: (0 to 10), 2: (10 to 20), 3: (20 to 30)],
	/// If you enter 19, you will receive 2 and 3.
	/// If you enter 15, you will receive 1, 2 and 3.
	/// # Arguments
	/// * `value` - The value of the angular interstar distance.
	///
	/// # Example
	/// ```
	/// use star_tracker::util::units::Radians;
	/// use star_tracker::util::aliases::Decimal;
	/// use star_tracker::tracking_mode::database::KVector;
	/// //           0    1    2    3    4  
	/// let dec = vec![4.0, 5.0, 6.0, 7.0, 8.0];
	/// // let lst = convert_dec_to_star_database_element(dec.clone());
	/// const NUM_BINS : usize = 4;
	/// 
	/// let kvec = KVector::new(NUM_BINS, dec[0].clone() as Decimal, dec[4] as Decimal);
	/// // Use the ranges in vec to find the elements in the star pair database.
	/// ```
	fn get_bins ( &self, value: Radians, angle_tolerance: Radians ) -> Error<Range<usize>>
	{
		if value.0 < self.min_value.0
		{
			return Err(Errors::InvalidValue);
		}
		else if self.max_value.0 < value.0
		{
			return Err(Errors::InvalidValue);
		}
		
		let tolerance = self.gradient / 2.0 + DECIMAL_PRECISION as Decimal + angle_tolerance.0;
		let mut high = (value.0 - self.intercept + tolerance) / self.gradient;
		let mut low =  (value.0 - self.intercept - tolerance) / self.gradient;
		
		low = low.floor();
		high = high.ceil();
		
		if high > self.num_bins as Decimal
		{
			high = self.num_bins as Decimal
		}
		return Ok(low as usize .. high as usize);
	}
}







impl fmt::Display for KVector
{
	fn fmt ( &self, format: &mut fmt::Formatter ) -> fmt::Result
	{
		let mut min = self.min_value;
		let mut max = self.max_value;
		if min < Radians(DECIMAL_PRECISION_TEST) { min = Radians(DECIMAL_PRECISION_TEST); }
		if max < Radians(DECIMAL_PRECISION_TEST) { max = Radians(DECIMAL_PRECISION_TEST); }
		let string = format!(
		"KVector{{gradient: {}, intercept: {}, min_value: {:?}, max_value: {:?}, num_bins: {}}}", 
		self.gradient, self.intercept, min, max, self.num_bins);
		
		format.write_str(&string)?;
		return Ok(());
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
mod test
{
	use crate::tracking_mode::database::StarDatabaseElement;
	use crate::tracking_mode::database::KVectorSearch;
	use crate::tracking_mode::database::KVector;
	
	use crate::tracking_mode::StarPair;
	
	use crate::util::aliases::DECIMAL_PRECISION;
	use crate::util::aliases::Decimal;
	use crate::util::test::TestEqual;
	use crate::util::units::Radians;
	use crate::util::list::List;





//###############################################################################################//
//
//										new
//
// fn new ( 
// 			num_bins: usize, 
//			min_value: Decimal, 
//			max_value: Decimal ) -> K_Vector
//
//###############################################################################################//

	#[test]
	fn test_new_two_elements ( )
	{
		let element_min = 1.23;
		let element_max = 10.0;
		let num_bins = 2;
		let k_vector = KVector::new(num_bins, element_min, element_max);
		
		let machine_epsilon = DECIMAL_PRECISION;
		let gradient = (element_max - element_min + machine_epsilon*2.0)/(num_bins as Decimal);
		let intercept = element_min - machine_epsilon;
		assert!(k_vector.gradient.test_equal(&(gradient as Decimal)));
		assert!(k_vector.intercept.test_equal(&(intercept as Decimal)));
		assert_eq!(k_vector.num_bins, num_bins);
		assert_eq!(k_vector.min_value, Radians(element_min as Decimal));
		assert_eq!(k_vector.max_value, Radians(element_max as Decimal));
		
		let mut y = gradient * 0.0 + intercept;			// The lower bounds.
		
		
		assert!(y < element_min);						// Must be smaller than min value.
		assert!(element_min.test_equal(&y));			// Must be close to the min value.
		
		y = gradient * 1.0 + intercept;					// This is the middle bounds (first bin).
		assert!(element_min < y && y < element_max);	// Value must be greater than the smallest element.
		assert!(y.test_equal(&((element_max + element_min) / 2.0)));// Must be in the center between the bounds.

		y = gradient * 2.0 + intercept;					// This is the upper bounds.
		assert!(element_max <= y);						// Must include all values.
		assert!(y.test_equal(&element_max));			// Must be close to the max value.
		
	}


	#[test]
	// As there is a large amount of bins, the final bin should be slightly under the max value.
	fn test_new_1000_elements ( )
	{
		let element_min = 1.23;
		let element_max = 10.0;
		let num_bins = 1000;
		let k_vector = KVector::new(num_bins, element_min, element_max);
		
		let machine_epsilon = DECIMAL_PRECISION;
		let gradient = (element_max - element_min + machine_epsilon*2.0)/(num_bins as Decimal);
		let intercept = element_min - machine_epsilon;
		assert!(k_vector.gradient.test_equal(&(gradient as Decimal)));
		assert!(k_vector.intercept.test_equal(&(intercept as Decimal)));
		
		let mut y = gradient * 0.0 + intercept;
		assert!(y < element_min);
		assert!(element_min.test_equal(&y));
	
		y = gradient * (num_bins as Decimal) + intercept;
		assert!(element_max <= y);					// Must include all values.
		assert!(y.test_close(&element_max, 0.01) );	// Must be close to the max value.
	}
	
	
	
	





//###############################################################################################//
//
//										Generate Bins
//
// fn generate_bins ( &self, sorted_database: &Vec<StarDatabaseElement> ) -> Error<Vec<usize>>
//
//
//###############################################################################################//



	fn convert_dec_to_star_database_element ( val: Vec<Decimal> ) -> Vec<StarDatabaseElement>
	{
		let mut vec : Vec<StarDatabaseElement> = Vec::with_capacity(val.size());
		
		for i in 0..val.size()
		{
			let pair = StarPair(0, 0);
			vec.push(StarDatabaseElement{pair: pair.clone(), dist: Radians(val[i])});
		}
		return vec;
	}




	fn test_generate_bins_failure ( )
	{
		let dec_0 = vec![];
		let dec_1 = vec![0.0];
		let dec_2 = vec![0.0, 0.0];
		let dec_3 = vec![0.0, 0.0, 0.0];
		
		let lst_0 = convert_dec_to_star_database_element(dec_0);
		let lst_1 = convert_dec_to_star_database_element(dec_1);
		let lst_2 = convert_dec_to_star_database_element(dec_2);
		let lst_3 = convert_dec_to_star_database_element(dec_3);
		
		let kvec = KVector::new(0, 0.0, 0.0);
		kvec.generate_bins(&lst_0).expect_err("Should fail.");
		kvec.generate_bins(&lst_1).expect_err("Should fail.");
		kvec.generate_bins(&lst_2).expect_err("Should fail.");
		kvec.generate_bins(&lst_3).expect("Should NOT fail.");
	}



	#[test]
	fn test_generate_bins_combined_bins ( )
	{
		//             0    1    2    3    4    5    6    7    8    9     10    11    12    13    14
		let dec = vec![0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 3.0, 5.0, 6.0, 10.0, 11.0, 27.0, 33.0, 33.0, 34.0];
		let lst = convert_dec_to_star_database_element(dec.clone());
		const NUM_BINS_2 : usize = 5;
		
		let kvec = KVector::new(NUM_BINS_2, dec[0] as Decimal, dec[14] as Decimal);
		let vec = kvec.generate_bins(&lst).expect("Should not fail");
		
		assert_eq!(vec.size(), NUM_BINS_2);
		
		// The vector specifies the bounds.
		// To use this, specify the lower index as inclusive and the next index as exclusive.
		// e.g. for an element between bin 1 and 2, it will be index 9 (inclusive) to 11 (exclusive).
		// 0
		assert_eq!(vec.get(0), 0);		// value = 0
		
		// 6.8
		assert_eq!(vec.get(1), 9);		// value = 10
		
		// 13.6
		assert_eq!(vec.get(2), 11);		// value = 22
		
		// 20.4
		assert_eq!(vec.get(3), 11);		// value = 22, THERE IS NO ELEMENTS SO IT IS THE SAME AS 2.

		// 27.2
		assert_eq!(vec.get(4), 15);		// value = 34		
	}
	
	
	
	#[test]
	fn test_generate_bins_same_bins_as_elements ( )
	{
		//             0    1    2    3    4    5    6    7     8     9     10     11     12     13     14
		let dec = vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 9.0, 10.0, 16.0, 33.0, 100.0, 190.0, 210.0, 211.0, 212.0];
		let lst = convert_dec_to_star_database_element(dec.clone());
		const NUM_BINS : usize = 15;
		
		let kvec = KVector::new(NUM_BINS, dec[0].clone() as Decimal, dec[14] as Decimal);
		let vec : Vec<usize> = kvec.generate_bins(&lst).expect("Should not fail");
		
		assert_eq!(vec.size(), NUM_BINS);
		
		for i in 0..vec.size()
		{
			println!("{}", vec.get(i));
		}
		
		// 2
		assert_eq!(vec.get(0), 0);		// value = 2
		// ~14.13
		assert_eq!(vec.get(1), 8);		// value = 16
		// ~28.27
		assert_eq!(vec.get(2), 9);		// value = 33
		// ~42.4
		assert_eq!(vec.get(3), 10);		// value = 100
		// ~56.5
		assert_eq!(vec.get(4), 10);		// value = 100
		// ~70.7
		assert_eq!(vec.get(5), 10);		// value = 100
		// ~84.8
		assert_eq!(vec.get(6), 10);		// value = 100
		// ~98.9
		assert_eq!(vec.get(7), 10);		// value = 100
		// ~113.0
		assert_eq!(vec.get(8), 11);		// value = 190
		// ~127,2
		assert_eq!(vec.get(9), 11);		// value = 190
		// ~141.3
		assert_eq!(vec.get(10), 11);		// value = 190
		// ~155.5
		assert_eq!(vec.get(11), 11);		// value = 190
		// ~169.6
		assert_eq!(vec.get(12), 11);		// value = 190
		// ~183.7
		assert_eq!(vec.get(13), 11);		// value = 190
		// ~212
		assert_eq!(vec.get(14), 15);		// value = 212
	}








//###############################################################################################//
//
//										Get Bins
//
// fn get_bins ( find: Radians ) -> Error<ops::RangeInclusive>
//
//###############################################################################################//

	#[test]
	fn test_get_bins_failure ( )
	{
		let kvec = KVector::new(10, 1.0, 10.0);
		kvec.get_bins ( Radians(0.999999),  Radians(0.0) ).expect_err("Should fail.");
		kvec.get_bins ( Radians(10.111111), Radians(0.0) ).expect_err("Should fail.");
				
		kvec.get_bins ( Radians(1.0000001),  Radians(0.0)).expect("Should pass.");
		kvec.get_bins ( Radians(9.99999999), Radians(0.0)).expect("Should pass.");
		
	}
	
	
	#[test]
	fn test_get_bins_tolerance ( )
	{
		//             0    1    2    3    4  
		let dec = vec![4.0, 5.0, 6.0, 7.0, 8.0];
		// let lst = convert_dec_to_star_database_element(dec.clone());
		const NUM_BINS : usize = 4;
		
		let kvec = KVector::new(NUM_BINS, dec[0].clone() as Decimal, dec[4] as Decimal);
		// let vec : Vec<usize> = kvec.generate_bins(lst);
		
		
		// Ranges will be in steps of the index above/bellow the tolerance / 2 + machine_precision
		// These steps will then be offset from the nearest integer by ~0.5.
		//
		// inclusive .. exclusive
		// 4.0-:   0 ..= 0 (Out of bounds)
		// 4.0 :   0 ..= 1
		// 4.0+:   0 ..= 1
		// 5.0-:   0 ..= 2
		// 5.0 :   0 ..= 2
		// 5.0+:   1 ..= 2
		// 6.0-:   1 ..= 3
		// 6.0 :   1 ..= 3
		// 6.0+:   2 ..= 3
		// 7.0-:   2 ..= 4
		// 7.0 :   2 ..= 4
		// 7.0+:   3 ..= 4
		// 8.0-:   3 ..= 4
		// 8.0 :   3 ..= 4
		// 8.0+:   4 ..= 4 (Out of bounds)
		
		// Multiplied decimal_precision by 3 as the number is so small that adding to it modifies the value.
		// let e = kvec.gradient / 2.0 - DECIMAL_PRECISION as Decimal * 3.0;
		let t_s  = Radians(0.0);
		let t_m  = Radians(0.4999999);
		let t_l  = Radians(0.5000001);
		let t_xl = Radians(0.9999999);
		let t_sl = Radians(1.0000001);
		
		// Should not crash due to low value.
		assert_eq!((0 .. 1), kvec.get_bins ( Radians(4.0), t_s).expect("Should pass"));
		assert_eq!((0 .. 1), kvec.get_bins ( Radians(4.0), t_m).expect("Should pass"));
		assert_eq!((0 .. 2), kvec.get_bins ( Radians(4.0), t_l).expect("Should pass"));
		assert_eq!((0 .. 2), kvec.get_bins ( Radians(4.0), t_xl).expect("Should pass"));
		assert_eq!((0 .. 2), kvec.get_bins ( Radians(4.0), t_sl).expect("Should pass"));
		
		// Should not crash due to large value.
		assert_eq!((3 .. 4), kvec.get_bins ( Radians(8.0), t_s).expect("Should pass"));
		assert_eq!((3 .. 4), kvec.get_bins ( Radians(8.0), t_m).expect("Should pass"));
		assert_eq!((2 .. 4), kvec.get_bins ( Radians(8.0), t_l).expect("Should pass"));
		assert_eq!((2 .. 4), kvec.get_bins ( Radians(8.0), t_xl).expect("Should pass"));
		assert_eq!((2 .. 4), kvec.get_bins ( Radians(8.0), t_sl).expect("Should pass"));
	
		
		assert_eq!((1 .. 3), kvec.get_bins ( Radians(6.0), t_s).expect("Should pass"));
		assert_eq!((1 .. 3), kvec.get_bins ( Radians(6.0), t_m).expect("Should pass"));
		assert_eq!((0 .. 4), kvec.get_bins ( Radians(6.0), t_l).expect("Should pass"));
	
	}
	
	
	#[test]
	fn test_get_bins ( )
	{
		//             0    1    2    3    4  
		let dec = vec![4.0, 5.0, 6.0, 7.0, 8.0];
		// let lst = convert_dec_to_star_database_element(dec.clone());
		const NUM_BINS : usize = 4;
		
		let kvec = KVector::new(NUM_BINS, dec[0].clone() as Decimal, dec[4] as Decimal);
		// let vec : Vec<usize> = kvec.generate_bins(lst);
		
		
		// Ranges will be in steps of the index above/bellow the tolerance / 2 + machine_precision
		// These steps will then be offset from the nearest integer by ~0.5.
		//
		// inclusive .. exclusive
		// 4.0-:   0 ..= 0 (Out of bounds)
		// 4.0 :   0 ..= 1
		// 4.0+:   0 ..= 1
		// 5.0-:   0 ..= 2
		// 5.0 :   0 ..= 2
		// 5.0+:   1 ..= 2
		// 6.0-:   1 ..= 3
		// 6.0 :   1 ..= 3
		// 6.0+:   2 ..= 3
		// 7.0-:   2 ..= 4
		// 7.0 :   2 ..= 4
		// 7.0+:   3 ..= 4
		// 8.0-:   3 ..= 4
		// 8.0 :   3 ..= 4
		// 8.0+:   4 ..= 4 (Out of bounds)
		
		let e = 0.01;
		let t = Radians(0.0);
		
		println!("y = {} x + {}", kvec.gradient, kvec.intercept);
		println!("4  {:?}", kvec.get_bins ( Radians(4.00000), t).expect("Should pass"));
		println!("4+ {:?}", kvec.get_bins ( Radians(4.0 + e), t).expect("Should pass"));
		println!("5- {:?}", kvec.get_bins ( Radians(5.0 - e), t).expect("Should pass"));
		println!("5  {:?}", kvec.get_bins ( Radians(5.00000), t).expect("Should pass"));
		println!("5+ {:?}", kvec.get_bins ( Radians(5.0 + e), t).expect("Should pass"));
		println!("6- {:?}", kvec.get_bins ( Radians(6.0 - e), t).expect("Should pass"));
		println!("6  {:?}", kvec.get_bins ( Radians(6.00000), t).expect("Should pass"));
		println!("6+ {:?}", kvec.get_bins ( Radians(6.0 + e), t).expect("Should pass"));
		println!("7- {:?}", kvec.get_bins ( Radians(7.0 - e), t).expect("Should pass"));
		println!("7  {:?}", kvec.get_bins ( Radians(7.00000), t).expect("Should pass"));
		println!("7+ {:?}", kvec.get_bins ( Radians(7.0 + e), t).expect("Should pass"));
		println!("8- {:?}", kvec.get_bins ( Radians(8.0 - e), t).expect("Should pass"));
		println!("8  {:?}", kvec.get_bins ( Radians(8.00000), t).expect("Should pass"));
		
		assert_eq!((0 .. 1), kvec.get_bins ( Radians(4.00000), t).expect("Should pass")); // 4.0
		assert_eq!((0 .. 1), kvec.get_bins ( Radians(4.0 + e), t).expect("Should pass")); // 4.0+
		
		assert_eq!((0 .. 2), kvec.get_bins ( Radians(5.0 - e), t).expect("Should pass")); // 5.0-
		assert_eq!((0 .. 2), kvec.get_bins ( Radians(5.00000), t).expect("Should pass")); // 5.0
		assert_eq!((0 .. 2), kvec.get_bins ( Radians(5.0 + e), t).expect("Should pass")); // 5.0+
		
		assert_eq!((1 .. 3), kvec.get_bins ( Radians(6.0 - e), t).expect("Should pass")); // 6.0-
		assert_eq!((1 .. 3), kvec.get_bins ( Radians(6.00000), t).expect("Should pass")); // 6.0
		assert_eq!((1 .. 3), kvec.get_bins ( Radians(6.0 + e), t).expect("Should pass")); // 6.0+
		
		assert_eq!((2 .. 4), kvec.get_bins ( Radians(7.0 - e), t).expect("Should pass")); // 7.0-
		assert_eq!((2 .. 4), kvec.get_bins ( Radians(7.00000), t).expect("Should pass")); // 7.0
		assert_eq!((2 .. 4), kvec.get_bins ( Radians(7.0 + e), t).expect("Should pass")); // 7.0+

		assert_eq!((3 .. 4), kvec.get_bins ( Radians(8.0 - e), t).expect("Should pass")); // 8.0-
		assert_eq!((3 .. 4), kvec.get_bins ( Radians(8.00000), t).expect("Should pass")); // 8.0
	}
}
