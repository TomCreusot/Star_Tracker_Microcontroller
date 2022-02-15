/// Implementation for StarDatabaseElement
use std::cmp::Ordering;

use crate::tracking_mode::StarPair;

use super::StarDatabaseElement;

use crate::util::units::Radians;
use crate::util::list::List;
use crate::util::aliases::DECIMAL_PRECISION;

use crate::nix::Star;



impl StarDatabaseElement
{
	/// Finds close stars, combines them into pointing pairs and adds them into an Vec.
	/// # Arguments
	/// * `fov` - The field of view of the sensor.
	/// * `stars` - The stars to reference and point to.
	///
	/// # Returns
	/// Star Pairs.
	///
	/// # Example
	/// ```
	/// use star_tracker::tracking_mode::database::StarDatabaseElement;
	/// use star_tracker::util::units::Degrees;
	/// use star_tracker::util::units::Equatorial;
	/// use star_tracker::nix::Star;
	///
	/// let v180 = Degrees(180.0).to_radians();
	/// let v90 = Degrees(90.0).to_radians();
	/// let vn90 = Degrees(-90.0).to_radians();
	/// let v45 = Degrees(45.0).to_radians();
	/// let v0 = Degrees(0.0).to_radians();
	/// let mut lst: Vec<Star> = Vec::with_capacity(5);
	/// lst.push(Star{pos: Equatorial{ ra: v180, dec: v90 }, mag: 0.0, spec: "".to_string()});	// 0 North
	/// lst.push(Star{pos: Equatorial{ ra: v0, dec: v90 },   mag: 0.0, spec: "".to_string()});	// 1 North
	/// lst.push(Star{pos: Equatorial{ ra: v0, dec: vn90 },  mag: 0.0, spec: "".to_string()});	// 2 South
	/// lst.push(Star{pos: Equatorial{ ra: v90, dec: v45 },  mag: 0.0, spec: "".to_string()});	// 3 45* Equator
	/// lst.push(Star{pos: Equatorial{ ra: v0, dec: v0 },    mag: 0.0, spec: "".to_string()});	// 4 Equator
	/// 
	/// let mut fov = Degrees(45.000001).to_radians();
	/// let mut out = StarDatabaseElement::create_list(fov, &lst);
	/// assert_eq!(out.len(), 3);
	/// assert_eq!(out[0].pair.0, 0);
	/// assert_eq!(out[0].pair.1, 1);
	/// 
	/// assert_eq!(out[1].pair.0, 0);
	/// assert_eq!(out[1].pair.1, 3);
	/// 
	/// assert_eq!(out[2].pair.0, 1);
	/// assert_eq!(out[2].pair.1, 3);
	/// ```

	pub fn create_list ( fov: Radians, stars: &dyn List<Star> ) -> Vec<StarDatabaseElement>
	{
		// Assumes size.
		let mut vec : Vec<StarDatabaseElement> = Vec::with_capacity(stars.size());
		
		for ii in 0..stars.size()
		{
			for jj in (ii + 1)..stars.size()
			{
				let dist = stars.get(ii).pos.angle_distance(stars.get(jj).pos);
				if dist.0 < fov.0 && ii != jj
				{
					let existed = false;
					if !existed
					{
						vec.push(StarDatabaseElement{dist: dist, pair: StarPair(ii, jj)});
					}
				}
			}
		}
		return vec;
	}
	
	
	
	/// Converts the list to star pairs.
	/// # Arguments
	/// * `stars` - The stars to be converted. 
	/// # Returns
	/// The star pair inside the object in a list form.
	pub fn to_star_pairs ( stars: &dyn List<StarDatabaseElement> ) -> Vec<StarPair<usize>>
	{
		let mut vec : Vec<StarPair<usize>> = Vec::with_capacity(stars.size());
		for i in 0..stars.size()
		{
			vec.push_back(stars.get(i).pair).expect("This should work.");
		}
		return vec;
	}
}



impl Ord for StarDatabaseElement
{
	/// Allows ordering with distnitude.
	fn cmp(&self, other: &Self) -> Ordering
	{
		if self.dist.0 > other.dist.0
		{
			return Ordering::Greater;
		}
		else if other.dist.0 > other.dist.0
		{
			return Ordering::Less;
		}
		else
		{
			return Ordering::Less;
		}
	}
}

impl Eq for StarDatabaseElement {}


impl PartialEq for StarDatabaseElement
{
	fn eq ( &self, other: &Self ) -> bool
	{
		return 
		((self.dist - other.dist).abs() < DECIMAL_PRECISION ) &&
		((self.pair.0 == other.pair.0 && self.pair.1 == other.pair.1) ||
		(self.pair.0 == other.pair.1 && self.pair.1 == other.pair.0));
	}
}

impl PartialOrd for StarDatabaseElement
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.dist.0 > other.dist.0// - DECIMAL_PRECISION
		{
			return Some(Ordering::Greater);
		}
		else if other.dist.0 > other.dist.0// + DECIMAL_PRECISION
		{
			return Some(Ordering::Less);
		}
		else
		{
			return Some(Ordering::Less);
		}
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
//###############################################################################################//
//										---	Database ---
//###############################################################################################//
	use tracking_mode::database::StarDatabaseElement;
	use tracking_mode::StarPair;
	
	use util::units::Equatorial;
	use util::units::Radians;
	use util::units::Degrees;
	
	use util::list::List;
	
	use nix::Star;


	fn construct_vec_star ( ) -> Vec<Star>
	{
		let v180 = Degrees(180.0).to_radians();
		let v90 = Degrees(90.0).to_radians();
		let vn90 = Degrees(-90.0).to_radians();
		let v45 = Degrees(45.0).to_radians();
		let v0 = Degrees(0.0).to_radians();
		let mut lst: Vec<Star> = Vec::with_capacity(5);
		lst.push(Star{pos: Equatorial{ ra: v180, dec: v90 }, mag: 0.0, spec: "".to_string()});	// 0 North
		lst.push(Star{pos: Equatorial{ ra: v0, dec: v90 },   mag: 0.0, spec: "".to_string()});	// 1 North
		lst.push(Star{pos: Equatorial{ ra: v0, dec: vn90 },  mag: 0.0, spec: "".to_string()});	// 2 South
		lst.push(Star{pos: Equatorial{ ra: v90, dec: v45 },  mag: 0.0, spec: "".to_string()});	// 3 45* Equator
		lst.push(Star{pos: Equatorial{ ra: v0, dec: v0 },    mag: 0.0, spec: "".to_string()});	// 4 Equator
		return lst;
	}



	//
	//  getters / setters
	//
	#[test]
	fn test_construct_database_basic_pairs_45_deg ( )
	{
		let lst = construct_vec_star();
		let mut fov = Degrees(0.0).to_radians();
		let mut out = StarDatabaseElement::create_list(fov, &lst);
		assert_eq!(out.len(), 0);
		
	 	fov = Degrees(1.0).to_radians();
		out = StarDatabaseElement::create_list(fov, &lst);
		assert_eq!(out.len(), 1);
		assert_eq!(out[0].pair.0, 0);
		assert_eq!(out[0].pair.1, 1);


	 	fov = Degrees(44.0).to_radians();
		out = StarDatabaseElement::create_list(fov, &lst);
		assert_eq!(out.len(), 1);
		
	 	fov = Degrees(45.000000000001).to_radians();
		out = StarDatabaseElement::create_list(fov, &lst);
		assert_eq!(out.len(), 3);
		assert_eq!(out[0].pair.0, 0);
		assert_eq!(out[0].pair.1, 1);
		
		assert_eq!(out[1].pair.0, 0);
		assert_eq!(out[1].pair.1, 3);
		
		assert_eq!(out[2].pair.0, 1);
		assert_eq!(out[2].pair.1, 3);
	}
	
	
	
	#[test]
	fn test_construct_database_basic_pairs_90_deg ( )
	{		
		let lst = construct_vec_star();
	 	let mut fov = Degrees(89.0).to_radians();
		let mut out = StarDatabaseElement::create_list(fov, &lst);
		assert_eq!(out.len(), 3);
		
		
		fov = Degrees(90.01).to_radians();
		out = StarDatabaseElement::create_list(fov, &lst);
		assert_eq!(out.len(), 7);
		assert_eq!(out[0].pair.0, 0);	// NORTH	NORTH2
		assert_eq!(out[0].pair.1, 1);
		
		assert_eq!(out[1].pair.0, 0);	// NORTH	45*
		assert_eq!(out[1].pair.1, 3);
		
		assert_eq!(out[2].pair.0, 0);	// NORTH	EQ
		assert_eq!(out[2].pair.1, 4);
		
		
		assert_eq!(out[3].pair.0, 1);	// NORTH2	45*
		assert_eq!(out[3].pair.1, 3);
		
		assert_eq!(out[4].pair.0, 1);	// NORTH2	EQ
		assert_eq!(out[4].pair.1, 4);
		
		
		assert_eq!(out[5].pair.0, 2);	// SOURTH	EQ
		assert_eq!(out[5].pair.1, 4);
	
	
		assert_eq!(out[6].pair.0, 3);	// 45*		EQ
		assert_eq!(out[6].pair.1, 4);
	}
	
	
	
	#[test]
	fn test_construct_database_basic_pairs_180_deg ( )
	{		
		let lst = construct_vec_star();
	 	let mut fov = Degrees(170.0).to_radians();
		let mut out = StarDatabaseElement::create_list(fov, &lst);
		assert_eq!(out.len(), 8);
		
		
		fov = Degrees(180.01).to_radians();
		out = StarDatabaseElement::create_list(fov, &lst);
		assert_eq!(out.len(), 10);
		assert_eq!(out[0].pair.0, 0);	// NORTH	NORTH2
		assert_eq!(out[0].pair.1, 1);
		
		assert_eq!(out[1].pair.0, 0);	// NORTH	SOUTH
		assert_eq!(out[1].pair.1, 2);
		
		assert_eq!(out[2].pair.0, 0);	// NORTH	45*
		assert_eq!(out[2].pair.1, 3);
		
		assert_eq!(out[3].pair.0, 0);	// NORTH	EQ
		assert_eq!(out[3].pair.1, 4);
		
		
		assert_eq!(out[4].pair.0, 1);	// NORTH2	SOUTH
		assert_eq!(out[4].pair.1, 2);
		
		assert_eq!(out[5].pair.0, 1);	// NORTH2	45*
		assert_eq!(out[5].pair.1, 3);
		
		assert_eq!(out[6].pair.0, 1);	// NORTH2	EQ
		assert_eq!(out[6].pair.1, 4);
		
		
		assert_eq!(out[7].pair.0, 2);	// SOURTH	45*
		assert_eq!(out[7].pair.1, 3);
		
		assert_eq!(out[8].pair.0, 2);	// SOURTH	EQ
		assert_eq!(out[8].pair.1, 4);
	
	
		assert_eq!(out[9].pair.0, 3);	// 45*		EQ
		assert_eq!(out[9].pair.1, 4);
	}
	
	
	
	
	
	
	
	
	
	#[test]
	fn test_to_star_pairs ( )
	{
		let mut vec : Vec<StarDatabaseElement> = Vec::new();
		vec.push_back(StarDatabaseElement{pair: StarPair(1, 0), dist: Radians(10.0)}).expect("");
		vec.push_back(StarDatabaseElement{pair: StarPair(0, 2), dist: Radians(20.0)}).expect("");
		vec.push_back(StarDatabaseElement{pair: StarPair(3, 0), dist: Radians(30.0)}).expect("");
		vec.push_back(StarDatabaseElement{pair: StarPair(0, 4), dist: Radians(40.0)}).expect("");
		
		let out = StarDatabaseElement::to_star_pairs(&vec);
		assert_eq!(out.get(0), StarPair(1, 0));
		assert_eq!(out.get(1), StarPair(0, 2));
		assert_eq!(out.get(2), StarPair(3, 0));
		assert_eq!(out.get(3), StarPair(0, 4));
	}

}
