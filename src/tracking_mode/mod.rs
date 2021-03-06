//! `tracking_mode` is the method of matching the stars found in the image to a database.  
//! The main methods to do this use a set of 3 stars where a specific feature of the set is unique.  
//! The main methods of these are:
//! * `liebe` - Find the angle 
//!


trait TrackingMode
{
	/// Creates unique sets of TrackingMode's from the location of the stars on an equatorial plane.  
	/// The sets consider the earlier element in the list to be the pilot.
	/// # Arguments
	/// * `stars` - The list of stars in order of magnitude (descending).
	/// * `sets` - The TrackingMode's to append to.
	///
	/// # Example
	/// ```
	/// panic!("NYI");
	/// ```
	fn find_sets ( stars : List<Equatorial>, sets : &mut List<Self> )
	{
		'outer: for ii in 0..lst.size()
		{
			for jj in ii..lst.size()
			{
				for kk in jj..lst.size()
				{
					if ( sets.is_full() )
					{
						break 'outer;
					}
					sets.push_back(Self::new(lst.get(ii), lst.get(jj), lst.get(kk)));
				}
			}
		}
	}
	
	
	/// Creates a new tracking mode based on the stars input.
	/// # Arguments
	/// `a` - The pilot star.
	/// `b` - Not the pilot star.
	/// `c` - Some other non pilot star.
	///
	/// # Returns
	/// A TrackingMode of the specified type.
	fn new ( a : &Equatorial, b : &Equatorial, c : &Equatorial ) -> dyn TrackingMode;
	
	/// Finds how close the current set is from the sample.  
	/// Use to find if a database sample is close enough to be considered the same.
	/// # Arguments
	/// `tolerance` - The percentage the values can be off from each other until the output is negative.
	/// `other` - The database object to compare to.
	///
	/// # Returns
	/// -infinity to 1 where 1 is an exact match.
	fn eq ( &self, tolerance : Decimal, other : &dyn TrackingMode ) -> Decimal;
	
	/// Sets the 
	///
	///
	fn vote ( &mut dyn List<Equatorial> );
	
	
	#[cfg(generate_database)]
	fn generate_database_entry ( &self ) -> String;
}


[cfg(test)]
mod test
{
	struct MockTrackingMode { pub a : Equatorial, pub b: Equatorial, pub c: Equatorial };
	
	impl TrackingMode for MockTrackingMode
	{
		fn new ( a : &Equatorial, b : &Equatorial, c : &Equatorial ) -> dyn TrackingMode
		{
			return MockTrackingMode{a: a.clone(), b: b.clone(), c: c.clone()};
		}

		fn vote_single ( &self, tolerance : Decimal, other : &dyn TrackingMode ) -> Decimal
		{	return 0;	}

		/// Sets the
		///
		///
		fn vote ( &mut dyn List<Equatorial> );
	} 
	
}
