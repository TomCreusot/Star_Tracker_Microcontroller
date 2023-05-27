use star_tracker_lib::tracking_mode::database::KVector;


use star_tracker_lib::util::err::Errors;
use star_tracker_lib::util::err::Error;

use star_tracker_lib::util::aliases::Decimal;
use star_tracker_lib::util::units::Radians;
use star_tracker_lib::util::test::DECIMAL_PRECISION_TEST;
use star_tracker_lib::util::list::List;


use crate::tracking_mode::StarDatabaseElement;
use crate::tracking_mode::KVectorGenerator;



impl KVectorGenerator for KVector
{
	/// Finds the ideal number of bins for the kvector.  
	/// Having not enough bins will result in having stars outside of the tolerance.  
	/// Having too many bins will require more memory.  
	/// Too larger: Excessive space.
	fn ideal_bins ( sorted_database: &Vec<StarDatabaseElement>, tolerance: Radians ) -> usize
	{
		let range = sorted_database[sorted_database.len() - 1].dist - sorted_database[0].dist;
		return (range / tolerance).0.ceil() as usize;
	}
	
	/// Creates a vector containging the bounds for each bin.
	/// # Arguments
	/// * "sorted_database" - The database in sorted order to reference.
	fn generate_bins ( &self, sorted_database: &Vec<StarDatabaseElement> ) -> Error<Vec<usize>>
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
	
	
	




	/// USED FOR DATABASE GENERATION
	fn display ( &self ) -> String
	{
		let mut min = self.min_value;
		let mut max = self.max_value;
		if min < Radians(DECIMAL_PRECISION_TEST) { min = Radians(DECIMAL_PRECISION_TEST); }
		if max < Radians(DECIMAL_PRECISION_TEST) { max = Radians(DECIMAL_PRECISION_TEST); }
		return format!(
			"KVector{{gradient: {}, intercept: {}, min_value: {:?}, max_value: {:?}, num_bins: {}}}", 
			self.gradient, self.intercept, min, max, self.num_bins);
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
pub mod test
{
	use star_tracker_lib::tracking_mode::database::KVector;
	use star_tracker_lib::tracking_mode::database::KVectorSearch;
	use star_tracker_lib::tracking_mode::StarPair;

	use star_tracker_lib::util::aliases::Decimal;
	use star_tracker_lib::util::units::Radians;
	use star_tracker_lib::util::list::List;

	use crate::tracking_mode::StarDatabaseElement;
	use crate::tracking_mode::KVectorGenerator;




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
	


//###############################################################################################//
//
//										KVectorGenerator
//
// fn ideal_bins ( sorted_database: &Vec<StarDatabaseElement>, tolerance: Radians ) -> usize
// fn generate_bins ( &self, sorted_database: &Vec<StarDatabaseElement> ) -> Error<Vec<usize>>
//
//
//###############################################################################################//
//									~ ideal_bins ~												 //
	#[test]
	fn test_ideal_bins ( )
	{
		let val = vec![1.0, 2.0, 3.0, 3.0, 4.0, 4.5, 5.0, 6.0, 6.0, 6.1, 6.2, 6.3, 10.0];
		let mut lst = convert_dec_to_star_database_element(val);
		assert_eq!(KVector::ideal_bins(&lst, Radians(1.0)), 9); lst[0].dist =Radians(1.1);
		assert_eq!(KVector::ideal_bins(&lst, Radians(1.0)), 9); lst[0].dist =Radians(0.9);
		assert_eq!(KVector::ideal_bins(&lst, Radians(1.0)), 10);
	}

	// #[test]
	// This is hard to test due to the randomness of bins...
	// fn test_ideal_bins_integration_test ( )
	// {
	// 	//              0    1     2   3    4    5    6    7   8     9   10   11    12
	// 	let val = vec![1.0, 2.0, 3.0, 3.0, 4.0, 4.5, 5.0, 6.0, 6.0, 6.1, 6.2, 6.3, 10.0];
	// 	let tolerance = Radians(1.0);
	// 	let lst = convert_dec_to_star_database_element(val);
	// 	let num_bins = KVector::ideal_bins(&lst, tolerance);
	// 
	// 	let k_vec = KVector::new(num_bins, 1.0, 10.0);
	// 	let bins =  k_vec.generate_bins(&lst).expect("");
	// 	assert_eq!(bins[k_vec.get_bins(Radians(1.0), tolerance).expect("").start],   0);
	// 	assert_eq!(bins[k_vec.get_bins(Radians(1.0), tolerance).expect("").end - 1], 1);
	// 	// 
	// 	assert_eq!(bins[k_vec.get_bins(Radians(1.5), tolerance).expect("").start],   0);
	// 	assert_eq!(bins[k_vec.get_bins(Radians(1.5), tolerance).expect("").end - 1], 1);
	// 
	// 	assert_eq!(bins[k_vec.get_bins(Radians(6.0), tolerance).expect("").start],   6);
	// 	assert_eq!(bins[k_vec.get_bins(Radians(6.0), tolerance).expect("").end - 1], 11);
	// }



//									~ generate_bins ~											 //



	#[test]
	fn test_generate_bins_failure ( )
	{
		let dec_0 = vec![];
		let dec_1 = vec![0.0];
		let dec_2 = vec![0.0, 0.0];

		let lst_0 = convert_dec_to_star_database_element(dec_0);
		let lst_1 = convert_dec_to_star_database_element(dec_1);
		let lst_2 = convert_dec_to_star_database_element(dec_2);

		let kvec = KVector::new(0, 0.0, 0.0);
		kvec.generate_bins(&lst_0).expect_err("Should fail.");
		kvec.generate_bins(&lst_1).expect_err("Should fail.");
		kvec.generate_bins(&lst_2).expect_err("Should fail.");
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




}