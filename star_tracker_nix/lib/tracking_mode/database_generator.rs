use super::DatabaseGenerator;

use star_tracker_lib::tracking_mode::StarPair;
use star_tracker_lib::tracking_mode::database::PyramidDatabase;
use star_tracker_lib::tracking_mode::database::RegionalDatabase;
use star_tracker_lib::tracking_mode::database::KVector;

use star_tracker_lib::util::distribution::Distribute;
use star_tracker_lib::util::aliases::Decimal;
use star_tracker_lib::util::aliases::UInt;
use star_tracker_lib::util::units::Equatorial;
use star_tracker_lib::util::units::BitField;
use star_tracker_lib::util::units::Radians;
use star_tracker_lib::util::list::List;

use crate::io::Star;
use crate::tracking_mode::StarDatabaseElement;
use crate::tracking_mode::KVectorGenerator;



impl DatabaseGenerator
{
	/// The required magnitude to ensure there are 4 stars in the image in any orientation with the specified field of view.
	pub fn recomended_magnitude ( fov: Radians ) -> Decimal
	{
		let magnitude_reduction = [
			6.69, // 10
			6.38, // 12
			5.75, // 14
			5.75, // 16
			5.44, // 18
			5.44, // 20
			5.12, // 22
			5.12, // 24
			5.12, // 26
			5.12, // 28
			4.81, // 30
			4.81, // 32
			4.50, // 34
			4.50, // 36
			4.19, // 38
			4.19, // 40
			4.19, // 42
			4.19, // 44
			4.19, // 46
			4.19, // 48
			3.56, // 50
			3.56, // 52
			3.56, // 54
			3.56, // 56
			3.56, // 58
			3.25, // 60
			3.25, // 62
			3.25, // 64
			3.25, // 66
			3.25, // 68
			3.25, // 70
			3.25, // 72
			3.25, // 74
			3.25, // 76
			2.94, // 78
			2.94, // 80
			2.94, // 82
			2.62, // 84
			2.62, // 86
			2.62, // 88
		];
		return magnitude_reduction[((fov.to_degrees().0 - 10.0) / 2.0).round() as usize];
	}









	/// Returns the default PyramidDatabase.  
	/// Call gen_database before this.
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

	
	/// Returns a regional database.  
	/// Ensure to call gen_database_regional before this.  
	/// gen_database will be sufficient to create this database.
	pub fn get_database_regional ( &self ) -> RegionalDatabase
	{
		assert_ne!(self.num_fields, 0, 
			"To use this database you muse use gen_database_regional instead of gen_database.");
		return RegionalDatabase {
			fov       : self.fov,
			num_fields: self.num_fields, 
			k_lookup  : self.k_lookup,
			k_vector  : &self.k_vector,
			pairs     : &self.pairs,
			catalogue : &self.catalogue,
			catalogue_field: &self.catalogue_field,
		};
	}
	




	/// Creates a database on the heap.
	/// The database is only valid while the lifetime of `k_vector`, `pairs` and `catalogue` exist.
	/// # Arguments
	/// * `stars`     - The stars to be inserted into the database.
	/// * `fov`       - The field of view of the sensor, .
	/// * `chunk_size`- The max distance between the stars until they are considered not linked.
	/// * `tolerance` - The allowed error of a star pair until it is not considered a match.
	/// # Returns
	/// The database with the lifetime of all the passed in variables.
	pub fn gen_database ( 
		stars: &Vec<Star>, 
		fov: Radians, 
		chunk_size : Radians, 
		tolerance: Radians ) -> Self
	{
		let mut pairs_unrefined = StarDatabaseElement::create_list(chunk_size, stars);
		pairs_unrefined.sort();

		let mut pairs : Vec<StarPair<usize>> = Vec::new();
		for i in 0..pairs_unrefined.len()
		{
			pairs.push(StarPair(pairs_unrefined[i].pair.0, pairs_unrefined[i].pair.1));
		}
		
		let num_bins = KVector::ideal_bins(&pairs_unrefined, tolerance);
		let k_lookup = KVector::new(num_bins, pairs_unrefined[0].dist.0 as Decimal,
											pairs_unrefined[pairs.len() - 1].dist.0 as Decimal);

		let k_vector = k_lookup.generate_bins(&pairs_unrefined).expect("Database too small.");
 		k_lookup.generate_bins(&pairs_unrefined).expect("Increase the cutoff magnitude.");

		let mut catalogue : Vec<Equatorial> = Vec::new();
		for i in 0..stars.size() { catalogue.push(stars[i].pos); }


		return Self
		{
			k_vector:        k_vector,
			pairs:           pairs,
			catalogue:       catalogue,
			fov:             fov,
			k_lookup:        k_lookup,
			catalogue_field: Vec::new(),
			num_fields:      0,
		};
	}
	
	

	
	/// Creates a database on the heap.
	/// The database is only valid while the lifetime of `k_vector`, `pairs` and `catalogue` exist.  
	/// This generates the regional database for the regional chunk iterator.  
	/// You can also use this for PyramidDatabase as it is just some extra fields.  
	/// # Arguments
	/// * `stars`     - The stars to be inserted into the database.
	/// * `fov`       - The field of view of the sensor, .
	/// * `tolerance` - The allowed error of a star pair until it is not considered a match.
	/// * `chunk_size`- The size of a chunk in the ChunkRegionalIterator.
	/// # Returns
	/// The database with the lifetime of all the passed in variables.
	pub fn gen_database_regional ( 
		stars: &Vec<Star>, fov: Radians, chunk_size: Radians, tolerance: Radians ) -> Self
	{
		let mut points_num = Distribute::angle_to_points(fov);

		if (BitField::FIELDS as usize) < points_num
		{	// There are more points then the bitfield, that would not work.
			points_num = BitField::FIELDS as usize;
		}
		let chunks = Distribute::fibonacci_lattice(points_num);
		// let angle = Distribute::points_to_angle(points_num);
		
		let mut database = Self::gen_database ( stars, fov, chunk_size, tolerance );
		let mut catalogue_field = Vec::with_capacity(database.catalogue.len()); 
		for star in &database.catalogue
		{
			let mut field = BitField(0);
			for chunk in 0..chunks.len()
			{
				field.set(chunk, star.angle_distance(chunks[chunk]) < chunk_size);
				
			}
			catalogue_field.push(field);
		}
		
		database.catalogue_field = catalogue_field;
		database.num_fields = points_num as UInt;
		return database;
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
	/// // Creates a list of stars between -11 and 2.0 magnitude.
	/// let limited = DatabaseGenerator::limit_magnitude(&stars, -11.0, 2.0);
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


	/// Removes any stars which are in the same location such as double stars.
	/// # Arguments
	/// * `stars` - The stars to observe.
	/// * `tolerance` - How close until the stars are considered double stars.
	///
	/// # Returns
	/// A reduced list of stars excluding the double stars.
	pub fn limit_double_stars ( stars: &dyn List<Star>, tolerance: Radians ) -> Vec<Star>
	{
		let mut stars_added : Vec<Star> = Vec::with_capacity(stars.size());
		for i in 0..stars.size()
		{
			stars_added.push(stars.get(i));
		}

		for ii in 0..stars_added.size()
		{
			let mut jj = ii + 1;
			while jj < stars_added.size()
			{
				if stars_added[ii].pos.angle_distance(stars_added[jj].pos) < tolerance
				{
					stars_added.remove(jj);
					jj-=1;
				}
				jj+=1;
			}
		}
		return stars_added;
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
		let mut coverage: Decimal = 0.0;
		let num_points: usize = Distribute::angle_to_points(region);
		let comparison_points : Vec<Equatorial> = Distribute::fibonacci_lattice(num_points);

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
		let num_points : usize = Distribute::angle_to_points(region);
		let comparison_points : Vec<Equatorial> = Distribute::fibonacci_lattice(num_points);

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
		let num_points : usize = Distribute::angle_to_points(region);
		let comparison_points : Vec<Equatorial> = Distribute::fibonacci_lattice(num_points);

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





// 
// //###############################################################################################//
// //###############################################################################################//
// //
// //										Unit Tests
// //
// //###############################################################################################//
// //###############################################################################################//
// 
// #[cfg(test)]
// mod test
// {
// 	use crate::util::units::Equatorial;
// 	use crate::util::units::Radians;
// 	use crate::util::units::Degrees;
// 
// 	use crate::nix::DatabaseGenerator;
// 
// 	use nix::Star;
// 
// 	// #[test]
// 	// // Testing the kvector involves creating a kvector and testing its accuracy.
// 	// fn test_gen_database ( )
// 	// {
// 	// 	// panic!("NYI");
// 	// 
// 	// 	// let mut rng = rand::thread_rng();
// 	// 	// let mut stars : Vec<Star> = Vec::new();
// 	// 	//
// 	// 	// let fov = Degrees(30.0).to_radians();
// 	// 	// let num_bins = 100;
// 	// 	// for _i in 0..100
// 	// 	// {
// 	// 	// 	let str = "".to_string();
// 	// 	// 	let eq = Equatorial{ra: Degrees(rng.gen::<Decimal>() * 360.0).to_radians(),
// 	// 	// 		dec: Degrees(rng.gen::<Decimal>() * 180.0 - 90.0).to_radians()};
// 	// 	// 	stars.push(Star{mag: 0.0, pos: eq, spec: str.clone(), name: str.clone()});
// 	// 	// }
// 	// 	//
// 	// 	// let gen = DatabaseGenerator::gen_database(&stars, fov, num_bins);
// 	// 	// let db = gen.get_database();
// 	// 	//
// 	// 	// let mut count = 0;
// 	// 	// for ii in 0..stars.len()
// 	// 	// {
// 	// 	// 	for jj in ii+1..stars.len()
// 	// 	// 	{
// 	// 	// 		let dist = stars[ii].pos.angle_distance(stars[jj].pos);
// 	// 	// 		if dist < fov / 2.0 && ii != jj
// 	// 	// 		{
// 	// 	// 			count += 1;
// 	// 	// 			let mut found : Vec<StarPair<usize>> = Vec::new();
// 	// 	// 			db.find_close_ref(dist, Radians(0.001), &mut found);
// 	// 	//
// 	// 	// 			let mut valid = false;
// 	// 	// 			for kk in 0..found.len()
// 	// 	// 			{
// 	// 	// 				let star_1 = db.find_star(found[kk].0).expect("?");
// 	// 	// 				let star_2 = db.find_star(found[kk].1).expect("?");
// 	// 	// 				valid |= (star_1.angle_distance(star_2) - dist).abs() < 0.001;
// 	// 	// 			}
// 	// 	// 			assert!(valid)
// 	// 	// 		}
// 	// 	// 	}
// 	// 	// }
// 	// 	// assert_eq!(count, db.pairs.size());
// 	// 	// assert_eq!(stars.len(), db.catalogue.size());
// 	// }
// 
// 
// 	// limit_regions ( stars, region size, # in region ) -> Vec
// 	#[test]
// 	fn test_limit_regions ( )
// 	{
// 		let mut stars : Vec<Star> = Vec::new();
// 
// 		let mut eq = Equatorial{ra: Degrees(0.0).to_radians(), dec: Degrees(0.0).to_radians()};
// 		stars.push(Star{mag: 0.0, pos: eq, spec: "".to_string(), name: "0".to_string()});
// 
// 		eq = Equatorial{ra: Degrees(0.0).to_radians(), dec: Degrees(1.0).to_radians()};
// 		stars.push(Star{mag: 0.0, pos: eq, spec: "".to_string(), name: "1".to_string()});
// 
// 		eq = Equatorial{ra: Degrees(0.0).to_radians(), dec: Degrees(-1.0).to_radians()};
// 		stars.push(Star{mag: 0.0, pos: eq, spec: "".to_string(), name: "2".to_string()});
// 
// 		eq = Equatorial{ra: Degrees(1.0).to_radians(), dec: Degrees(0.0).to_radians()};
// 		stars.push(Star{mag: 0.0, pos: eq, spec: "".to_string(), name: "3".to_string()});
// 
// 		eq = Equatorial{ra: Degrees(-1.0).to_radians(), dec: Degrees(0.0).to_radians()};
// 		stars.push(Star{mag: 0.0, pos: eq, spec: "".to_string(), name: "4".to_string()});
// 
// 		// Should be cut.
// 		eq = Equatorial{ra: Degrees(2.0).to_radians(), dec: Degrees(0.0).to_radians()};
// 		stars.push(Star{mag: 0.0, pos: eq, spec: "".to_string(), name: "5".to_string()});
// 
// 		eq = Equatorial{ra: Degrees(-2.0).to_radians(), dec: Degrees(0.0).to_radians()};
// 		stars.push(Star{mag: 0.0, pos: eq, spec: "".to_string(), name: "6".to_string()});
// 
// 		eq = Equatorial{ra: Degrees(0.0).to_radians(), dec: Degrees(2.0).to_radians()};
// 		stars.push(Star{mag: 0.0, pos: eq, spec: "".to_string(), name: "7".to_string()});
// 
// 		eq = Equatorial{ra: Degrees(0.0).to_radians(), dec: Degrees(-2.0).to_radians()};
// 		stars.push(Star{mag: 0.0, pos: eq, spec: "".to_string(), name: "8".to_string()});
// 
// 		// Since previous were not added, should not be cut.
// 		eq = Equatorial{ra: Degrees(0.0).to_radians(), dec: Degrees(4.0).to_radians()};
// 		stars.push(Star{mag: 0.0, pos: eq, spec: "".to_string(), name: "9".to_string()});
// 
// 		eq = Equatorial{ra: Degrees(4.0).to_radians(), dec: Degrees(0.0).to_radians()};
// 		stars.push(Star{mag: 0.0, pos: eq, spec: "".to_string(), name: "10".to_string()});
// 
// 		let limited = DatabaseGenerator::limit_regions(&stars, Degrees(3.0).to_radians(), 4);
// 		// assert_eq!(limited.len(), 4);
// 		assert_eq!(limited[0].name, "0".to_string());
// 		assert_eq!(limited[1].name, "1".to_string());
// 		assert_eq!(limited[2].name, "2".to_string());
// 		assert_eq!(limited[3].name, "3".to_string());
// 		assert_eq!(limited[4].name, "9".to_string());
// 		assert_eq!(limited[5].name, "10".to_string());
// 	}
// 
// 	#[test] // num < 4
// 	#[should_panic]
// 	fn test_limit_regions_panic ( )
// 	{
// 		let mut stars: Vec<Star> = Vec::new();
// 		stars.push(Star{mag: 0.0, pos: Equatorial{ra: Radians(0.0), dec: Radians(0.0)},
// 			spec: "".to_string(), name: "".to_string()});
// 		stars.push(Star{mag: 0.0, pos: Equatorial{ra: Radians(0.0), dec: Radians(0.0)},
// 			spec: "".to_string(), name: "".to_string()});
// 		stars.push(Star{mag: 0.0, pos: Equatorial{ra: Radians(0.0), dec: Radians(0.0)},
// 			spec: "".to_string(), name: "".to_string()});
// 
// 		DatabaseGenerator::limit_regions(&stars, Radians(0.0), 3);
// 	}
// 
// 
// 
// 	#[test]
// 	fn test_limit_magnitude ( )
// 	{
// 		let eq : Equatorial = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
// 		let mut stars : Vec<Star> = Vec::new();
// 		stars.push(Star{mag: -30.0,   pos: eq, spec: "".to_string(), name: "1".to_string()});
// 		stars.push(Star{mag: -20.0,   pos: eq, spec: "".to_string(), name: "2".to_string()});
// 		stars.push(Star{mag: -10.0,   pos: eq, spec: "".to_string(), name: "3".to_string()});
// 		stars.push(Star{mag: 0.0,     pos: eq, spec: "".to_string(), name: "4".to_string()});
// 		stars.push(Star{mag: 1.0,     pos: eq, spec: "".to_string(), name: "5".to_string()});
// 		stars.push(Star{mag: 1.99999, pos: eq, spec: "".to_string(), name: "6".to_string()});
// 		stars.push(Star{mag: 2.0,     pos: eq, spec: "".to_string(), name: "7".to_string()});
// 
// 		let out = DatabaseGenerator::limit_magnitude(&stars, -11.0, 2.0);
// 		assert_eq!(out.len(), 4);
// 		assert_eq!(out[0].name, "3");
// 		assert_eq!(out[1].name, "4");
// 		assert_eq!(out[2].name, "5");
// 		assert_eq!(out[3].name, "6");
// 	}
// 
// 	#[test]
// 	// Should not crash.
// 	fn test_limit_magnitude_invalid_bounds ( )
// 	{
// 		let eq : Equatorial = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
// 		let mut stars : Vec<Star> = Vec::new();
// 		stars.push(Star{mag: -30.0,   pos: eq, spec: "".to_string(), name: "1".to_string()});
// 		stars.push(Star{mag: -20.0,   pos: eq, spec: "".to_string(), name: "2".to_string()});
// 		stars.push(Star{mag: -10.0,   pos: eq, spec: "".to_string(), name: "3".to_string()});
// 
// 		let out = DatabaseGenerator::limit_magnitude(&stars, 100.0, -100.0);
// 		assert_eq!(out.len(), 0);
// 	}
// 
// }
