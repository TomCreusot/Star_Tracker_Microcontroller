
struct Liebe
{
	pub position : Equatorial,
	pub angle : Decimal,
	pub side1 : Decimal,
	pub side2 : Decimal,
}


impl TrackingMode for Liebe
{
	/// Creates a new tracking mode based on the stars input.
	/// # Arguments
	/// `a` - The pilot star.
	/// `b` - Not the pilot star.
	/// `c` - Some other non pilot star.
	///
	/// # Returns
	/// A TrackingMode of the specified type.
	///
	/// # Example
	/// ```
	/// 
	/// ```
	fn new ( a : &Equatorial, b : &Equatorial, c : &Equatorial ) -> dyn TrackingMode
	{
		let angle1 = a.angle_distance(b);
		let angle2 = a.angle_distance(c);
		return Liebe
		{
			position: a.clone(),
			angle: (angle1 / angle2).atan(),
			side1: angle1(),
			side2: angle2()
		}
	}
	
	/// Finds how close the current set is from the sample.  
	/// Use to find if a database sample is close enough to be considered the same.
	/// # Arguments
	/// `tolerance` - The percentage the values can be off from each other until the output is negative.
	/// `other` - The database object to compare to.
	///
	/// # Returns
	/// -infinity to 1 where 1 is an exact match.
	fn eq ( &self, tolerance : Decimal, other : &dyn TrackingMode ) -> Decimal
	{
		
	}
	
	/// Sets the
	///
	///
	fn vote ( &mut dyn List<Equatorial> )
	{
		
	}
}




#[cfg(test)]
mod test
{
	#[test]
	fn new ( )
	{
		
	}
	
	
	
	
	
	
}
