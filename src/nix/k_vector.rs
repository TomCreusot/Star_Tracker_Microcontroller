use std::fmt;

use crate::tracking_mode::database::KVector;
use crate::nix::StarDatabaseElement;

use crate::util::err::Errors;
use crate::util::err::Error;

use crate::util::aliases::Decimal;
use crate::util::units::Radians;
use crate::util::test::DECIMAL_PRECISION_TEST;

use crate::util::list::List;





impl KVector
{

	
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


impl fmt::Display for KVector
{
	/// USED FOR DATABASE GENERATION
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











// 
// //###############################################################################################//
// //
// //										Generate Bins
// //
// // fn generate_bins ( &self, sorted_database: &Vec<StarDatabaseElement> ) -> Error<Vec<usize>>
// //
// //
// //###############################################################################################//
// 
// 
// 
// 	fn convert_dec_to_star_database_element ( val: Vec<Decimal> ) -> Vec<StarDatabaseElement>
// 	{
// 		let mut vec : Vec<StarDatabaseElement> = Vec::with_capacity(val.size());
// 
// 		for i in 0..val.size()
// 		{
// 			let pair = StarPair(0, 0);
// 			vec.push(StarDatabaseElement{pair: pair.clone(), dist: Radians(val[i])});
// 		}
// 		return vec;
// 	}
// 
// 
// 
// 
// 	fn test_generate_bins_failure ( )
// 	{
// 		let dec_0 = vec![];
// 		let dec_1 = vec![0.0];
// 		let dec_2 = vec![0.0, 0.0];
// 		let dec_3 = vec![0.0, 0.0, 0.0];
// 
// 		let lst_0 = convert_dec_to_star_database_element(dec_0);
// 		let lst_1 = convert_dec_to_star_database_element(dec_1);
// 		let lst_2 = convert_dec_to_star_database_element(dec_2);
// 		let lst_3 = convert_dec_to_star_database_element(dec_3);
// 
// 		let kvec = KVector::new(0, 0.0, 0.0);
// 		kvec.generate_bins(&lst_0).expect_err("Should fail.");
// 		kvec.generate_bins(&lst_1).expect_err("Should fail.");
// 		kvec.generate_bins(&lst_2).expect_err("Should fail.");
// 		kvec.generate_bins(&lst_3).expect("Should NOT fail.");
// 	}
// 
// 
// 
// 	#[test]
// 	fn test_generate_bins_combined_bins ( )
// 	{
// 		//             0    1    2    3    4    5    6    7    8    9     10    11    12    13    14
// 		let dec = vec![0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 3.0, 5.0, 6.0, 10.0, 11.0, 27.0, 33.0, 33.0, 34.0];
// 		let lst = convert_dec_to_star_database_element(dec.clone());
// 		const NUM_BINS_2 : usize = 5;
// 
// 		let kvec = KVector::new(NUM_BINS_2, dec[0] as Decimal, dec[14] as Decimal);
// 		let vec = kvec.generate_bins(&lst).expect("Should not fail");
// 
// 		assert_eq!(vec.size(), NUM_BINS_2);
// 
// 		// The vector specifies the bounds.
// 		// To use this, specify the lower index as inclusive and the next index as exclusive.
// 		// e.g. for an element between bin 1 and 2, it will be index 9 (inclusive) to 11 (exclusive).
// 		// 0
// 		assert_eq!(vec.get(0), 0);		// value = 0
// 
// 		// 6.8
// 		assert_eq!(vec.get(1), 9);		// value = 10
// 
// 		// 13.6
// 		assert_eq!(vec.get(2), 11);		// value = 22
// 
// 		// 20.4
// 		assert_eq!(vec.get(3), 11);		// value = 22, THERE IS NO ELEMENTS SO IT IS THE SAME AS 2.
// 
// 		// 27.2
// 		assert_eq!(vec.get(4), 15);		// value = 34		
// 	}
// 
// 
// 
// 	#[test]
// 	fn test_generate_bins_same_bins_as_elements ( )
// 	{
// 		//             0    1    2    3    4    5    6    7     8     9     10     11     12     13     14
// 		let dec = vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 9.0, 10.0, 16.0, 33.0, 100.0, 190.0, 210.0, 211.0, 212.0];
// 		let lst = convert_dec_to_star_database_element(dec.clone());
// 		const NUM_BINS : usize = 15;
// 
// 		let kvec = KVector::new(NUM_BINS, dec[0].clone() as Decimal, dec[14] as Decimal);
// 		let vec : Vec<usize> = kvec.generate_bins(&lst).expect("Should not fail");
// 
// 		assert_eq!(vec.size(), NUM_BINS);
// 
// 		for i in 0..vec.size()
// 		{
// 			println!("{}", vec.get(i));
// 		}
// 
// 		// 2
// 		assert_eq!(vec.get(0), 0);		// value = 2
// 		// ~14.13
// 		assert_eq!(vec.get(1), 8);		// value = 16
// 		// ~28.27
// 		assert_eq!(vec.get(2), 9);		// value = 33
// 		// ~42.4
// 		assert_eq!(vec.get(3), 10);		// value = 100
// 		// ~56.5
// 		assert_eq!(vec.get(4), 10);		// value = 100
// 		// ~70.7
// 		assert_eq!(vec.get(5), 10);		// value = 100
// 		// ~84.8
// 		assert_eq!(vec.get(6), 10);		// value = 100
// 		// ~98.9
// 		assert_eq!(vec.get(7), 10);		// value = 100
// 		// ~113.0
// 		assert_eq!(vec.get(8), 11);		// value = 190
// 		// ~127,2
// 		assert_eq!(vec.get(9), 11);		// value = 190
// 		// ~141.3
// 		assert_eq!(vec.get(10), 11);		// value = 190
// 		// ~155.5
// 		assert_eq!(vec.get(11), 11);		// value = 190
// 		// ~169.6
// 		assert_eq!(vec.get(12), 11);		// value = 190
// 		// ~183.7
// 		assert_eq!(vec.get(13), 11);		// value = 190
// 		// ~212
// 		assert_eq!(vec.get(14), 15);		// value = 212
// 	}
// 
// 
// 
// 
// }