use super::DatabaseGenerator;

use util::aliases::Decimal;
use util::units::Radians;
use util::units::Equatorial;
use util::list::List;

use nix::Star;

use tracking_mode::StarPair;
use tracking_mode::database::PyramidDatabase;
use tracking_mode::database::StarDatabaseElement;
use tracking_mode::database::KVector;


impl DatabaseGenerator
{
	pub fn get_database ( &self ) -> PyramidDatabase
	{
		return PyramidDatabase {
			fov      : self.fov,
			k_lookup : self.k_lookup,
			k_vector : &self.k_vector,
			pairs    : &self.pairs,
			catalogue: &self.catalogue,
		};
	}


	/// Creates a database on the heap.
	/// The database is only valid while the lifetime of `k_vector`, `pairs` and `catalogue` exist.
	/// # Arguments
	/// * `stars` - The stars to be inserted into the database.
	/// * `fov` - The field of view of the sensor.
	/// * `num_bins` - The number of lookup bins to use, more bins: more memory, less bins: slower.
	/// * `k_vector` - To satisfy the lifetime variable of the database, leave this blank.
	/// * `pairs` - To satisfy the lifetime variable of the database, leave this blank.
	/// * `catalogue` - To satisfy the lifetime variable of the database, leave this blank.
	/// # Returns
	/// The database with the lifetime of all the passed in variables.
	pub fn gen_database ( stars: &Vec<Star>, fov: Radians, num_bins: usize ) -> Self
	{
		let mut pairs_unrefined = StarDatabaseElement::create_list(fov / 2.0, stars);
		pairs_unrefined.sort();

		let mut pairs : Vec<StarPair<usize>> = Vec::new();
		for i in 0..pairs_unrefined.len()
		{
			pairs.push(StarPair(pairs_unrefined[i].pair.0, pairs_unrefined[i].pair.1));
		}

		let k_lookup = KVector::new(num_bins, pairs_unrefined[0].dist.0 as Decimal,
											pairs_unrefined[pairs.len() - 1].dist.0 as Decimal);

		let k_vector = k_lookup.generate_bins(&pairs_unrefined).expect("Database too small.");
 		k_lookup.generate_bins(&pairs_unrefined).expect("Increase the cutoff magnitude.");

		let mut catalogue : Vec<Equatorial> = Vec::new();
		for i in 0..stars.size() { catalogue.push(stars[i].pos); }


		return Self
		{
			k_vector: k_vector,
			pairs: pairs,
			catalogue: catalogue,
			fov: fov,
			k_lookup: k_lookup
		};
	}


	/// Returns a set of stars which achieves the maximum star coverage with the least overlaps.
	/// This ensures that regions of lots of stars will not take up as much memory.
	///
	/// The algorithm works by slotting the brightest stars into the database.
	/// Each star will be given a number representing the number of neighboring stars.
	/// While there is not an ideal coverage reached for each star in the region, a star is added.
	/// The algorithm ends when all stars are observed.
	/// # Arguments
	/// * `stars` - The stars to add, you may want to limit the magnitude before inputting.
	/// * `region` - The number of stars in a region wont vastly exceed `stars_in_region`.
	///              Ideally this should be less than the fov of the lens due to distortion.
	/// * stars_in_region - The minimum stars that can fit in a region.
	///
	/// # Asserts
	/// 4 < stars_in_region:
	/// There must be at least 3 stars in a region for the triangle method to work.
	/// Ideally you need more than 4 to get proper accuracy and reliability.
	///
	/// # Returns
	/// A reduced set of stars with the same star coverage as the input set but smaller in size.
	pub fn limit_regions (
				  stars: &Vec<Star>, region_size: Radians, stars_in_region: usize ) -> Vec<Star>
	{
		assert!(3 < stars_in_region,
			"You should have at least 4 stars in a region to make the tracking algorithm work.");

		// The stars which will be in the database.
		let mut stars_added       : Vec<Star>  = Vec::new();
		let mut stars_added_count : Vec<usize> = Vec::new();

		// The brightest stars should be added first.
		// stars.sort();

		// The reference of stars in the region a star is to be added.
		let mut region : Vec<usize> = Vec::new();


		for to_add in stars
		{
			region.clear();         // The stars in the region.
			let mut lonely = false; // If a star in the region does not have enough neighbors.

			// Checks if the region is saturated.
			for i in 0..stars_added.len()
			{
				if to_add.pos.angle_distance(stars_added[i].pos) < region_size
				{
					region.push(i);
					lonely |= stars_added_count[i] < stars_in_region;
				}
			}

			// Add the star if there is not enough stars in a region.
			// OR a star in this region does not have enough stars in its region.
			// Every star in the region must add a neighbor to their count.
			if region.len() < stars_in_region || lonely
			{
				for i in 0..region.size()
				{
					stars_added_count[region[i]] += 1;
				}
				stars_added.push(to_add.clone());
				stars_added_count.push(region.len() + 1);
			}
		}
		return stars_added;
	}



	/// Removes stars outside the designated bounds.
	/// # Arguments
	/// * `stars` - A list of stars with random magnitudes.
	/// * `min_magnitude` - The max magnitude of the star.
	/// * `max_magnitude` - The max magnitude of the star.
	/// # Returns
	/// All the stars that are less than the `cutoff_magnitude`.
	/// # Equatorial
	/// use util::units::Radians;
	/// use util::units::Equatorial;
	/// use nix::Star;
	/// use tracking_mode::DatabaseGenerator;
	/// let eq : Equatorial = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
	/// let mut stars : Vec<Star> = Vec::new();
	/// stars.push(Star{mag: -30.0,   pos: eq, spec: "".to_string(), name: "1".to_string()});
	/// stars.push(Star{mag: -20.0,   pos: eq, spec: "".to_string(), name: "2".to_string()});
	/// stars.push(Star{mag: -10.0,   pos: eq, spec: "".to_string(), name: "3".to_string()});
	/// stars.push(Star{mag: 0.0,     pos: eq, spec: "".to_string(), name: "4".to_string()});
	/// stars.push(Star{mag: 1.0,     pos: eq, spec: "".to_string(), name: "5".to_string()});
	/// stars.push(Star{mag: 1.99999, pos: eq, spec: "".to_string(), name: "6".to_string()});
	/// stars.push(Star{mag: 2.0,     pos: eq, spec: "".to_string(), name: "7".to_string()});
	///
	/// let out = DatabaseGenerator::limit_magnitude(&stars, -11.0, 2.0);
	/// assert_eq!(out.len(), 4);
	/// assert_eq!(out[0].name, "3");
	/// assert_eq!(out[1].name, "4");
	/// assert_eq!(out[2].name, "5");
	/// assert_eq!(out[3].name, "6");
	pub fn limit_magnitude ( stars: &dyn List<Star>,
								min_magnitude: Decimal,
								max_magnitude: Decimal ) -> Vec<Star>
	{
		let mut s : Vec<Star> = Vec::new();
		for i in 0..stars.size()
		{
			if stars.get(i).mag < max_magnitude && min_magnitude < stars.get(i).mag
			{
				s.push(stars.get(i));
			}
		}
		return s;
	}



	/// Finds the percentage of sky covered by the specified number of stars.
	/// This is done by finding the percentage of regions in the sky which are satisfied.
	/// # Arguments
	/// * `stars` - The stars which are observed in the sky by the sensor.
	/// * `region` - The accurate part of the sensor, use a value less than the vertical fov.
	/// * `stars_in_region` - The number of stars required in to qualify as covered in a region.
	/// # Returns
	/// A decimal percentage.
	pub fn sky_coverage ( stars: &dyn List<Star>, region: Radians, stars_in_region : usize )
		-> Decimal
	{
		let mut coverage : Decimal = 0.0;
		let num_points : usize = Equatorial::evenly_distribute_points(region);
		let comparison_points : Vec<Equatorial> = Equatorial::evenly_distribute(num_points);

		for ii in 0..comparison_points.len()
		{
			let mut count = 0;
			for jj in 0..stars.size()
			{
				if comparison_points[ii].angle_distance(stars.get(jj).pos) < region
				{
					count += 1;
				}
			}

			if stars_in_region <= count
			{
				coverage += 1.0;
			}
		}
		coverage /= comparison_points.len() as Decimal;
		return coverage;
	}


	/// Searches for the worst case scenario in the sky where there is the least amount of stars.
	/// This is a very unlikely situation.
	/// # Arguments
	/// * `stars` - The stars which are observed in the sky by the sensor.
	/// * `region` - The accurate part of the sensor, use a value less than the vertical fov.
	/// # Returns
	/// The number of stars in the most lonely part of the sky.
	pub fn sky_coverage_worst_case ( stars: &dyn List<Star>, region: Radians ) -> usize
	{
		let mut worst_case : usize = usize::MAX;
		let num_points : usize = Equatorial::evenly_distribute_points(region);
		let comparison_points : Vec<Equatorial> = Equatorial::evenly_distribute(num_points);

		for ii in 0..comparison_points.len()
		{
			let mut count = 0;
			for jj in 0..stars.size()
			{
				if comparison_points[ii].angle_distance(stars.get(jj).pos) < region
				{
					count += 1;
				}
			}
			if count < worst_case
			{
				worst_case = count;
			}
		}
		return worst_case;
	}


	/// Searches for the best case scenario in the sky where there is the most amount of stars.
	/// This is a very unlikely situation.
	/// If this number is too high, you should add limits to your database.
	/// # Arguments
	/// * `stars` - The stars which are observed in the sky by the sensor.
	/// * `region` - The accurate part of the sensor, use a value less than the vertical fov.
	/// # Returns
	/// The number of stars in the most lonely part of the sky.
	pub fn sky_coverage_best_case ( stars: &dyn List<Star>, region: Radians ) -> usize
	{
		let mut best_case : usize = 0;
		let num_points : usize = Equatorial::evenly_distribute_points(region);
		let comparison_points : Vec<Equatorial> = Equatorial::evenly_distribute(num_points);

		for ii in 0..comparison_points.len()
		{
			let mut count = 0;
			for jj in 0..stars.size()
			{
				if comparison_points[ii].angle_distance(stars.get(jj).pos) < region
				{
					count += 1;
				}
			}
			if count > best_case
			{
				best_case = count;
			}
		}
		return best_case;
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
	use rand::prelude::*;

	use util::units::Equatorial;
	use util::units::Radians;
	use util::units::Degrees;
	use util::aliases::Decimal;

	use tracking_mode::database::DatabaseGenerator;
	use tracking_mode::database::Database;
	use tracking_mode::StarPair;

	use nix::Star;

	#[test]
	// Testing the kvector involves creating a kvector and testing its accuracy.
	fn test_gen_database ( )
	{
		let mut rng = rand::thread_rng();
		let mut stars : Vec<Star> = Vec::new();

		let fov = Degrees(30.0).to_radians();
		let num_bins = 100;
		for _i in 0..100
		{
			let str = "".to_string();
			let eq = Equatorial{ra: Degrees(rng.gen::<Decimal>() * 360.0).to_radians(),
				dec: Degrees(rng.gen::<Decimal>() * 180.0 - 90.0).to_radians()};
			stars.push(Star{mag: 0.0, pos: eq, spec: str.clone(), name: str.clone()});
		}

		let gen = DatabaseGenerator::gen_database(&stars, fov, num_bins);
		let db = gen.get_database();

		let mut count = 0;
		for ii in 0..stars.len()
		{
			for jj in ii+1..stars.len()
			{
				let dist = stars[ii].pos.angle_distance(stars[jj].pos);
				if dist < fov / 2.0 && ii != jj
				{
					count += 1;
					let mut found : Vec<StarPair<usize>> = Vec::new();
					db.find_close_ref(dist, Radians(0.001), &mut found);

					let mut valid = false;
					for kk in 0..found.len()
					{
						let star_1 = db.find_star(found[kk].0).expect("?");
						let star_2 = db.find_star(found[kk].1).expect("?");
						valid |= (star_1.angle_distance(star_2) - dist).abs() < 0.001;
					}
					assert!(valid)
				}
			}
		}
		assert_eq!(count, db.pairs.size());
		assert_eq!(stars.len(), db.catalogue.size());
	}


	// limit_regions ( stars, region size, # in region ) -> Vec
	#[test]
	fn test_limit_regions ( )
	{
		let mut stars : Vec<Star> = Vec::new();

		let mut eq = Equatorial{ra: Degrees(0.0).to_radians(), dec: Degrees(0.0).to_radians()};
		stars.push(Star{mag: 0.0, pos: eq, spec: "".to_string(), name: "0".to_string()});

		eq = Equatorial{ra: Degrees(0.0).to_radians(), dec: Degrees(1.0).to_radians()};
		stars.push(Star{mag: 0.0, pos: eq, spec: "".to_string(), name: "1".to_string()});

		eq = Equatorial{ra: Degrees(0.0).to_radians(), dec: Degrees(-1.0).to_radians()};
		stars.push(Star{mag: 0.0, pos: eq, spec: "".to_string(), name: "2".to_string()});

		eq = Equatorial{ra: Degrees(1.0).to_radians(), dec: Degrees(0.0).to_radians()};
		stars.push(Star{mag: 0.0, pos: eq, spec: "".to_string(), name: "3".to_string()});

		eq = Equatorial{ra: Degrees(-1.0).to_radians(), dec: Degrees(0.0).to_radians()};
		stars.push(Star{mag: 0.0, pos: eq, spec: "".to_string(), name: "4".to_string()});

		// Should be cut.
		eq = Equatorial{ra: Degrees(2.0).to_radians(), dec: Degrees(0.0).to_radians()};
		stars.push(Star{mag: 0.0, pos: eq, spec: "".to_string(), name: "5".to_string()});

		eq = Equatorial{ra: Degrees(-2.0).to_radians(), dec: Degrees(0.0).to_radians()};
		stars.push(Star{mag: 0.0, pos: eq, spec: "".to_string(), name: "6".to_string()});

		eq = Equatorial{ra: Degrees(0.0).to_radians(), dec: Degrees(2.0).to_radians()};
		stars.push(Star{mag: 0.0, pos: eq, spec: "".to_string(), name: "7".to_string()});

		eq = Equatorial{ra: Degrees(0.0).to_radians(), dec: Degrees(-2.0).to_radians()};
		stars.push(Star{mag: 0.0, pos: eq, spec: "".to_string(), name: "8".to_string()});

		// Since previous were not added, should not be cut.
		eq = Equatorial{ra: Degrees(0.0).to_radians(), dec: Degrees(4.0).to_radians()};
		stars.push(Star{mag: 0.0, pos: eq, spec: "".to_string(), name: "9".to_string()});

		eq = Equatorial{ra: Degrees(4.0).to_radians(), dec: Degrees(0.0).to_radians()};
		stars.push(Star{mag: 0.0, pos: eq, spec: "".to_string(), name: "10".to_string()});

		let limited = DatabaseGenerator::limit_regions(&stars, Degrees(3.0).to_radians(), 4);
		// assert_eq!(limited.len(), 4);
		assert_eq!(limited[0].name, "0".to_string());
		assert_eq!(limited[1].name, "1".to_string());
		assert_eq!(limited[2].name, "2".to_string());
		assert_eq!(limited[3].name, "3".to_string());
		assert_eq!(limited[4].name, "9".to_string());
		assert_eq!(limited[5].name, "10".to_string());
	}

	#[test] // num < 4
	#[should_panic]
	fn test_limit_regions_panic ( )
	{
		let mut stars: Vec<Star> = Vec::new();
		stars.push(Star{mag: 0.0, pos: Equatorial{ra: Radians(0.0), dec: Radians(0.0)},
			spec: "".to_string(), name: "".to_string()});
		stars.push(Star{mag: 0.0, pos: Equatorial{ra: Radians(0.0), dec: Radians(0.0)},
			spec: "".to_string(), name: "".to_string()});
		stars.push(Star{mag: 0.0, pos: Equatorial{ra: Radians(0.0), dec: Radians(0.0)},
			spec: "".to_string(), name: "".to_string()});

		DatabaseGenerator::limit_regions(&stars, Radians(0.0), 3);
	}



	#[test]
	fn test_limit_magnitude ( )
	{
		let eq : Equatorial = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let mut stars : Vec<Star> = Vec::new();
		stars.push(Star{mag: -30.0,   pos: eq, spec: "".to_string(), name: "1".to_string()});
		stars.push(Star{mag: -20.0,   pos: eq, spec: "".to_string(), name: "2".to_string()});
		stars.push(Star{mag: -10.0,   pos: eq, spec: "".to_string(), name: "3".to_string()});
		stars.push(Star{mag: 0.0,     pos: eq, spec: "".to_string(), name: "4".to_string()});
		stars.push(Star{mag: 1.0,     pos: eq, spec: "".to_string(), name: "5".to_string()});
		stars.push(Star{mag: 1.99999, pos: eq, spec: "".to_string(), name: "6".to_string()});
		stars.push(Star{mag: 2.0,     pos: eq, spec: "".to_string(), name: "7".to_string()});

		let out = DatabaseGenerator::limit_magnitude(&stars, -11.0, 2.0);
		assert_eq!(out.len(), 4);
		assert_eq!(out[0].name, "3");
		assert_eq!(out[1].name, "4");
		assert_eq!(out[2].name, "5");
		assert_eq!(out[3].name, "6");
	}

	#[test]
	// Should not crash.
	fn test_limit_magnitude_invalid_bounds ( )
	{
		let eq : Equatorial = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		let mut stars : Vec<Star> = Vec::new();
		stars.push(Star{mag: -30.0,   pos: eq, spec: "".to_string(), name: "1".to_string()});
		stars.push(Star{mag: -20.0,   pos: eq, spec: "".to_string(), name: "2".to_string()});
		stars.push(Star{mag: -10.0,   pos: eq, spec: "".to_string(), name: "3".to_string()});

		let out = DatabaseGenerator::limit_magnitude(&stars, 100.0, -100.0);
		assert_eq!(out.len(), 0);
	}

}
