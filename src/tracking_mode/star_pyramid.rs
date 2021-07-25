//! Implementation of StarPyramid.

struct StarTriangle ( Equatorial, Equatorial, Equatorial );
struct StarPair ( Equatorial, Equatorial );


impl TrackingMode for StarPyramid
{
    /// Creates unique sets of TrackingMode's from the location of the stars on an equatorial plane.
	/// These are then compared with the database and the accurate sets from the database will be returned.
	/// # Arguments
	/// * `stars` - The list of stars in order of magnitude (descending).
	///
	/// # Returns
	/// The triangle of the image and the triangle of the database.
	///
	/// # Example
	/// ```
	/// panic!("NYI");
	/// ```
	fn new ( stars : &List<Equatorial> ) -> StarPyramid
    {
	    let iter = KernelIterator(stars.size());

		while iter.step()
		{
			// 1. Assemble the triangle.
			// 2. Search in database and ensure the triangle is correct.
	        let stars = StarTriangle(stars.get(iter.i), stars.get(iter.j), stars.get(iter.k));
			let validStars = verify_triangle ( stars );

			// If star was found.
			if ( !validStars.is_none() )
			{
				if ( stars.size() < 4 )
				{
					// Cant run pyramid check as not enough stars.
					return (stars, validStars.unwrap());
				}
				else
				{
					// Find another star and varify it is correct.
					for x in stars.size()
					{
						if x != iter.i && x != iter.j && x != iter.k
						{
							
						}
					}
				}
			}
		}
    }
}
