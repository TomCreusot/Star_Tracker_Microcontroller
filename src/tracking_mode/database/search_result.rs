//! Implementation of [SearchResult](crate::tracking_mode::database::SearchResult)

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
	
	/// Checks if the star pair is the same in both instances.
	/// # Arguments
	/// * `pair_a` - The first pair.
	/// * `pair_b` - The second pair.
	/// # Returns
	/// The simmilar item.
	/// ```
	pub fn has_same_pair ( pair_a: &SearchResult, pair_b: &SearchResult ) -> bool
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
				return Some(i);
			}
		}
		return None;
	}
}