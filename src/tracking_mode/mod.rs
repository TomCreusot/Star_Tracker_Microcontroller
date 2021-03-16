//! `tracking_mode` is the method of matching the stars found in the image to a database.
//! The main methods to do this use a set of 3 stars where a specific feature of the set is unique.
//! The main methods of these are:
//! * `liebe` - Find the angle
//!


trait TrackingMode
{
	/// Creates unique sets of TrackingMode's from the location of the stars on an equatorial plane.
	/// These are then compared with the database and the accurate sets from the database will be returned.
	/// # Arguments
	/// * `stars` - The list of stars in order of magnitude (descending).
	/// * `sets` - The database elements to append to.
	fn find_sets ( stars : &List<Equatorial>, sets : &mut List<Self> );

	/// Finds how close the current set is from the sample.
	/// Use to find if a database sample is close enough to be considered the same.
	/// # Arguments
	/// `tolerance` - The percentage the values can be off from each other until the output is negative.
	/// `other` - The database object to compare to.
	///
	/// # Returns
	/// -infinity to 1 where 1 is an exact match.
	fn eq ( &self, tolerance : &Self, other : &dyn TrackingMode ) -> Decimal;

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
