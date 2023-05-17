//! Implementation of [SearchResult](crate::tracking_mode::database::SearchResult)
use crate::core_include::*;

use crate::tracking_mode::database::SearchResult;
use crate::tracking_mode::StarPair;

use crate::util::list::List;

// impl SearchSave for SearchResult
// {
// 	/// Call to return the constructed requested type.
// 	fn Save ( result: StarPair<usize> , region: BitField, error: Decimal ) -> Self
// 	{
// 		return Self{result: result, region: region, error: error};
// 	}
// }


impl SearchResult
{
	
	/// Checks if the star pair has an element.
	/// # Arguments
	/// * `pair_a` - The first pair.
	/// * `pair_b` - The second pair.
	/// # Returns
	/// The simmilar item.
	/// ```
	pub fn has_same_star ( pair_a: &SearchResult, pair_b: &SearchResult ) -> bool
	{
		return StarPair::has_same(&pair_a.result, &pair_b.result);
	}
	
	/// Finds the index of a star pair containing both values if it exists.
	/// # Arguments
	/// * `find` - A set of 2 stars that must be found (any order).
	/// * `list` - The list to search through.
	/// # Returns
	/// None if there is not a match or the first valid result.
	pub fn index_of_pair ( find: StarPair<usize>, list: &dyn List<Self> ) -> Option<usize>
	{
		let f = find;
		for i in 0..list.size()
		{
			let p = list.get(i).result;
			if (p.0 == f.0 && p.1 == f.1) || (p.0 == f.1 && p.1 == f.0)
			{
				return Option::Some(i);
			}
		}
		return Option::None;
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
	use crate::tracking_mode::database::SearchResult;
	use crate::tracking_mode::StarPair;

//###############################################################################################//
//
//										Features
//
// pub fn has_same_star ( &SearchResult, &SearchResult )     -> bool
// pub fn index_of_pair ( StarPair<usize>, &dyn List<Self> ) -> Option<usize>
//
//###############################################################################################//
//										~ has_same_star ~										 //
	#[test]
	fn test_has_same_correct ( )
	{
		let pair_a = SearchResult{result: StarPair(1, 3), error: 1.0};
		let pair_b = SearchResult{result: StarPair(1, 2), error: 2.0};
		assert!(SearchResult::has_same_star(&pair_a, &pair_b));
	}

	#[test]
	fn test_has_same_invalid ( )
	{
		let pair_a = SearchResult{result: StarPair(1, 3), error: 1.0};
		let pair_b = SearchResult{result: StarPair(2, 2), error: 2.0};
		assert!(!SearchResult::has_same_star(&pair_a, &pair_b));
	}
	
	
//										~ index_of_pair ~										 //
	
	#[test]
	fn test_index_of_pair_valid ( )
	{
		let search = StarPair(0, 1);
		let options = vec![
			SearchResult{result: StarPair(0, 2), error: 1.2},
			SearchResult{result: StarPair(1, 0), error: 1.3},
		];
		assert_eq!(SearchResult::index_of_pair(search, &options), Some(1));
	}
	
	#[test]
	fn test_index_of_pair_invalid ( )
	{
		let search = StarPair(0, 1);
		let options = vec![
			SearchResult{result: StarPair(0, 2), error: 1.2},
			SearchResult{result: StarPair(1, 1), error: 1.3},
			SearchResult{result: StarPair(0, 0), error: 1.4},
		];
		assert_eq!(SearchResult::index_of_pair(search, &options), None);
	}

}