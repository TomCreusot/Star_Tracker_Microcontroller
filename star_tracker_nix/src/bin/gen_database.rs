//! This code is used to generate a database for embedded binaries.
//! Build files don't work as when no std is specified, the build file must comply.
//! 
//! This should be used in a bash script.
//! Call:
//! cargo run --bin gen_database {{path to flash folder}} {{path to config file}}
//! 
//! This will be under the module flash so ensure you have that in your program.
//! Also a database_config.json file is required.
//! 
//! The json config file must be structured as:
//! REQUIRED:
//! 	`fov_deg`:
//! 	The diagonal field of view of the sensor.
//! 	This has to be accurate, use nova.astrometry.net to get the best result.
//! 
//! 	`angle_tol_deg`: 
//! 	The error of pixels.
//! 	You can either use nova.astrometry.net corr.fits file with corr_analyser.
//! 	The error ideally should be equal or less than 0.1 degrees.
//! 	The bigger the tolerance, the longer the search takes.
//! 
//! OPTIONAL:
//! 	region_size_deg:
//! 		To reduce the database, the sky is divided up into regions.
//! 		Each region will have a limited amount of stars to ensure that there is enough stars at the poles and not too many in the milky way.
//! 		Default = fov / 2.
//! 
//! 	region_num_stars:
//! 		Once there is this many stars in a region, all duller stars in this region will be removed.
//! 		Default = 8.
//! 
//! 	chunk_size:
//! 		How far a star can be linked to another until they are too far away.
//! 		If this is too big, you are introducing error which will make the search longer.
//! 		If it is too small, you are limiting the amount of stars that can be matched.
//! 
//! 	magnitude_max:
//! 		The dullest star brightness allowed.
//! 		By having this too high, the database will be bigger, if the magnitude is too low, you wont have enough coverage.
//! 		The default value is calculated based on the input field of view, you probably should just use that.
//!

extern crate star_tracker_lib;
extern crate star_tracker_nix;


use star_tracker_lib::util::aliases::Decimal;
use star_tracker_lib::util::units::Radians;
use star_tracker_lib::util::units::Degrees;
use star_tracker_lib::util::list::List;

use star_tracker_nix::tracking_mode::DatabaseGenerator;
use star_tracker_nix::io::Star;
use star_tracker_nix::io::Io;

use std::env;
use std::fs::File;
use std::io::Read;

pub fn main ( )
{
	println!(r#"
	
	
				===== Gen Database =====
This code is used to generate a database for embedded binaries.
Build files don't work as when no std is specified, the build file must comply.

This should be used in a bash script.
Call:
cargo run --bin gen_database {{path to flash folder}} {{path to config file}}

This will be under the module flash so ensure you have that in your program.
Also a database_config.json file is required.

The json config file must be structured as:
REQUIRED:
	`fov_deg`:
	The diagonal field of view of the sensor.
	This has to be accurate, use nova.astrometry.net to get the best result.

	`angle_tol_deg`: 
	The error of pixels.
	You can either use nova.astrometry.net corr.fits file with corr_analyser.
	The error ideally should be equal or less than 0.1 degrees.
	The bigger the tolerance, the longer the search takes.

OPTIONAL:
	region_size_deg:
		To reduce the database, the sky is divided up into regions.
		Each region will have a limited amount of stars to ensure that there is enough stars at the poles and not too many in the milky way.
		Default = fov / 2.

	region_num_stars:
		Once there is this many stars in a region, all duller stars in this region will be removed.
		Default = 8.

	chunk_size:
		How far a star can be linked to another until they are too far away.
		If this is too big, you are introducing error which will make the search longer.
		If it is too small, you are limiting the amount of stars that can be matched.

	magnitude_max:
		The dullest star brightness allowed.
		By having this too high, the database will be bigger, if the magnitude is too low, you wont have enough coverage.
		The default value is calculated based on the input field of view, you probably should just use that.
	"#);
	env::set_var("RUST_BACKTRACE", "1");
	let args: Vec<String> = env::args().collect();
	let mut file_config = File::open(Vec::get(&args, 2).clone())//.expect("Please provide a valid config file.").clone())
		.expect("Please provide a valid log file.");
	
	let mut json_str = String::new();
	file_config.read_to_string(&mut json_str).ok().expect("");
	let json: serde_json::Value = 
		serde_json::from_str(&json_str)
			.expect("INVALID LOG FILE");
			
	let fov            : Radians 
		= Degrees(json.get("fov_deg").expect("LOG FILE REQUIRES `fov_deg`")
			.as_f64().expect("INVALID TYPE IN LOG `fov`") as f64).to_radians();

	let angle_tolerance: Radians = 
		Degrees(json.get("angle_tol_deg").expect("LOG FILE REQUIRES `angle_tol_deg`")
			.as_f64().expect("INVALID TYPE IN LOG `angle_tol_deg")as f64).to_radians();
	
	
	let mut magnitude_max: Decimal = DatabaseGenerator::recommended_magnitude(fov);
	if let Some(val) = json.get("magnitude_max") { magnitude_max = val.as_f64().expect("INVALID TYPE IN LOG `magnitude_max") as Decimal; }

	let mut region_size: Radians = fov / 2.0;
	if let Some(val) = json.get("region_size") 
	{ region_size = Degrees(val.as_f64().expect("INVALID TYPE IN LOG `region_size") as Decimal).to_radians(); }
	
	let mut region_num: usize = 8;
	if let Some(val) = json.get("region_num") { region_num = val.as_i64().expect("INVALID TYPE IN LOG `region_num`") as usize; }
	std::mem::drop(file_config);
	
	let double_star_tolerance = angle_tolerance * 2.0;
	let magnitude_min    = -20.00; // Excludes sun


	println!("Performing Database Construction");
	println!("\tReading database.");
	print!("\t");
	let mut stars : Vec<Star> = Io::get_csv_database();
	println!("\t Found: {} stars", stars.len());
	stars.sort();

	println!("\tLimiting Magnitude.");
	let stars_limit_mag    = DatabaseGenerator::limit_magnitude (&stars, magnitude_min, magnitude_max);
	println!("\tLimiting Double Stars.");
	let stars_limit_double = DatabaseGenerator::limit_double_stars(&stars_limit_mag, double_star_tolerance);
	println!("\tLimiting Regions.");
	let stars_limit_reg    = DatabaseGenerator::limit_regions(&stars_limit_double, region_size, region_num);

	println!("\tCreating Database.");	

	let gen : DatabaseGenerator = DatabaseGenerator::gen_database_regional
		(&stars_limit_reg, fov, fov / 1.5, angle_tolerance);
	let database = gen.get_database_regional();


	let mut k_vector_str = String::with_capacity(database.k_vector.size() * 5);
	for i in 0..database.k_vector.size()
	{
		k_vector_str.push_str(&format!("\t{},\n", database.k_vector.get(i)).to_string());
	}

	let mut pairs_str = String::with_capacity(database.pairs.size() * 20);
	for i in 0..database.pairs.size()
	{
		pairs_str.push_str(&format!("\tStarPair({}, {}),\n", 
			database.pairs.get(i).0, database.pairs.get(i).1).to_string());
	}

	let mut catalog_str = String::with_capacity(database.catalogue.size() * 5);
	for i in 0..database.catalogue.size()
	{
		catalog_str.push_str(&format!("\tEquatorial{{ra: Radians({}), dec: Radians({})}},\n", 
			database.catalogue.get(i).ra.0, database.catalogue.get(i).dec.0).to_string());
	}

	let output = format!(r#"
//! This is a generated file from gen_database with the configuration of:
//! {}	field of view
//! {}	angle tolerance
//! {}	max magnitude
//! {}	region size
//! {}   	region number

use star_tracker_lib::util::units::Radians;
use star_tracker_lib::util::units::Equatorial;
use star_tracker_lib::tracking_mode::StarPair;
use star_tracker_lib::tracking_mode::database::KVector;
use star_tracker_lib::tracking_mode::database::PyramidDatabase;

pub const DATABASE: PyramidDatabase = PyramidDatabase
{{
	fov:       FOV,
	k_lookup:  K_LOOKUP,
	k_vector:  &K_VECTOR,
	pairs:     &PAIRS,
	catalogue: &CATALOGUE,
}};

pub const FOV: Radians = Radians({});

pub const K_LOOKUP: KVector = KVector
{{
	gradient:  {},
	intercept: {},
	min_value: Radians({}),
	max_value: Radians({}),
	num_bins:  {},
}};


pub const K_VECTOR: [usize; {}] = 
[
{}
];


pub const PAIRS: [StarPair<usize>; {}] = 
[
{}
];


pub const CATALOGUE: [Equatorial; {}] =
[
{}
];
	
	
	"#, 
	fov.to_degrees(), angle_tolerance.to_degrees(), magnitude_max, region_size.to_degrees(), region_num,
	database.fov.0,
	database.k_lookup.gradient, database.k_lookup.intercept, 
	database.k_lookup.min_value.0, database.k_lookup.max_value.0, database.k_lookup.num_bins,
	database.k_vector.size(),
	k_vector_str,
	database.pairs.size(),
	pairs_str,
	database.catalogue.size(),
	catalog_str
);


	star_tracker_nix::io::Io::write_to_file(args[1].as_str(), &output);

}