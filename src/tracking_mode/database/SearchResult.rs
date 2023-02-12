//! Implementation of [SearchResult](crate::tracking_mode::database::SearchResult)

use crate::tracking_mode::database::SearchResult;
use crate::tracking_mode::database::SearchSave;
use crate::tracking_mode::StarPair;

use crate::util::units::BitField;
use crate::util::units::Decimal;

// impl SearchSave for SearchResult
// {
// 	/// Call to return the constructed requested type.
// 	fn Save ( result: StarPair<usize> , region: BitField, error: Decimal ) -> Self
// 	{
// 		return Self{result: result, region: region, error: error};
// 	}
// }