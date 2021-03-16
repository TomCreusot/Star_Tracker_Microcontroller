//! # Pyramid Tracking Mode Algorithm
//! The pyramid method is one of the most reliable methods. It was developed by Mortari in 1997.
//! The method was modified a few times.
//!
//! ## Summary
//! The algorithm creates a kernal (a set of 3 stars) with a method to reduce the number of times a false positive star is used.
//! It then will compare the constellation with a k-vector table database.
//! If it succeeds, it will then find another star and varify it is also consistant.
//!
//! ### Creating the kernal
//! To select the stars, a looping algerithm must be chosen the proposed method is implemented in this algorithm.
//! This will access the index of 0-1-2, 2-3-4, 3-4-5, 1-2-4, 2-3-5, 1-2-5, 1-3-4, 2-4-5, 1-3-5, 1-4-5.
//! This method is done to reduce the times a specific star is used as it may be a false star.
//! This is then conpared to the database to find the closest solution.
//!
//! ### Pyramid
//! If a correct kernal is found, an extra test is performed where another star is picked, finding the distance from this star to the other stars, this is then compared with the database.
//!
//!
//! ### Database
//! The database contains 3 different components:
//! - k-vector:
//!   This is a lookup table which points to the location of specific points in a database, this is ascending.
//!   It will point to the position of a value equal to or less than the supplied value (using floating points, this should be less than only).
//!
//! | k-vector-input | k-vect-output|   | index | database  |
//! |:--------------:|:------------:|:-:|:-----:|:---------:|
//! |        2       |       0      |   |   0   |     3     |
//! |        4       |       0      |   |   1   |     5     |
//! |        6       |       2      |   |   2   |     6     |
//! |        8       |       4      |   |   3   |     7     |
//! |                |              |   |   4   |     8     |
//! |                |              |   |   5   |     9     |
//!
//! - List of star pairs:
//!   The list of star pairs is every
//!
//! - Star Catalog
//!
//! ## Features
//! This method features:
//! - An algorithm which can fit on a microcontroller.
//! - Contains a method to reduce the repetition of using a false star.
//! - Fast as uses K-Table.
//! - Checks if the triangle is flipped (false positive).
//! - Adds an extra star to varify correct identification opposing other triangle constilation based methods.
//!
//!
//! ## Useful links:
//! - [Original Paper](https://www.researchgate.net/publication/254199748_Lost-in-Space_Pyramid_Algorithm_for_Robust_Star_Pattern_Recognition)
//! - [Pseudo Interpritation](https://arxiv.org/pdf/1808.08686.pdf#page=7)
//! - [Good Explination](http://mtc-m21b.sid.inpe.br/col/sid.inpe.br/mtc-m21b/2017/08.10.22.44/doc/publicacao.pdf#page=105)

impl TrackingMode for Pyramid
{
    /// Creates unique sets of TrackingMode's from the location of the stars on an equatorial plane.
	/// These are then compared with the database and the accurate sets from the database will be returned.
	/// # Arguments
	/// * `stars` - The list of stars in order of magnitude (descending).
	/// * `sets` - The database elements to append to.
	///
	/// # Example
	/// ```
	/// panic!("NYI");
	/// ```
	fn find_sets ( stars : &List<Equatorial>, sets : &mut List<Self> )
    {
	    let n = sets.size();
	    for dj in 1..n - 2
	    {
	        for dk in 1..n - 1 - dj
	        {
	            for ii  in 0..n - dj - dk - 1
	            {
					let i = ii - 1
	                let j = ii + dj - 1;
	                let k = j + dk - 1;

                    let stars = (stars.get(i), stars.get(j), stars.get(k));

					// Find unique triangle (kernal) from star set?
					// > 1  ? Purge specular selection (check if it is flipped).
						// sign(𝐛𝑖 ⋅ (𝐛𝑗 × 𝐛𝑘)) = sign(𝐫𝐼 ⋅ (𝐫𝐽 × 𝐫𝐾)) where b and r are unit vectors.
					// == 0 ? Try new kernal
					// == 1 ? Continue

					// If set found:
						// Find another star if available
						// Ensure distances match between all stars.
						// SUCCESS



					// K-Vector
					//
					//
					//

                    // Has a potential match been found in the triangle.
                    if results.size() > 0
                    {
                        beta = // star not i j or k
                        T = Find
                    }

                }
            }
        }
    }



    /// Finds the closest match with the distance of all stars.
    /// # Arguments
    ///
    ///
    fn find_t ( stars : &[Equatorial] )
    {


    }




}
