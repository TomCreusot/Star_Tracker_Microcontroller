extern crate star_tracker_lib;
extern crate star_tracker_database;


use star_tracker_lib::util::aliases::Decimal;
use star_tracker_lib::util::units::Radians;
use star_tracker_lib::util::units::Degrees;
use star_tracker_lib::util::list::List;

use star_tracker_database::tracking_mode::DatabaseGenerator;
use star_tracker_database::io::Star;
use star_tracker_database::io::Io;

use std::env;
use std::fs::File;
use std::io::Read;

pub fn main ( )
{
	// If there is an error, uncomment this to find it.
	// env::set_var("RUST_BACKTRACE", "1");

	// Command line arguments.
	// Removes the binary as not important and checks for the quiet flag.
	let mut args: Vec<String> = env::args().collect();
	let mut has_quiet = false;
	while args[0].contains("database_json") { args.remove(0); }
	for e in &args { has_quiet |= e == "-q"; }
	if !has_quiet
	{
	println!(r#"
	
	
				===== Database Json =====
This code is used to generate a rust database to be compiled inside a project for embedded binaries.
This should be used in a bash script.
Call:
```
cargo run --bin database_json {{path to output}} {{path to config file}}
# OR
cargo run --bin database_json {{path to output}} {{path to config file}} -q # For Quiet Mode
```

To run, you need to specify `path to output` which is where the output `.rs` should be placed.
You will also need a config `.json` file, which specifies how the database should be constructed.
A sample json file can be seen in `star_tracker_database/sample_database_config.json`
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
	}

	// If invalid arguments.
	if !(args.len() == 2 || args.len() == 3)
	{
		println!(
r#"
ERROR: Insufficient command line arguments.
Please enter the command line:
cargo run --bin gen_database{{path to output}} {{path to config file}}
"#
		);
		return;
	}


	

	// Reads the config json file and assigns values or defaults.
	let mut file_config = File::open(Vec::get(&args, 1).clone())//.expect("Please provide a valid config file.").clone())
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
	if let Some(val) = json.get("region_size_deg") 
	{ region_size = Degrees(val.as_f64().expect("INVALID TYPE IN LOG `region_size_deg") as Decimal).to_radians(); }
	
	let mut region_num: usize = 8;
	if let Some(val) = json.get("region_num_stars") { region_num = val.as_i64().expect("INVALID TYPE IN LOG `region_num_stars`") as usize; }
	
	std::mem::drop(file_config);

	










	// Generates Database
	let double_star_tolerance = angle_tolerance * 2.0;
	let magnitude_min    = -20.00; // Excludes sun


	println!("Performing Database Construction");
	println!("\tReading database.");
	print!("\t");
	let mut stars : Vec<Star> = Io::get_csv_database();
	println!("\t Found: {} stars", stars.len());
	stars.sort();

	println!("\tLimiting Magnitude.");
	println!("\t Max Magnitude {}", magnitude_max);
	let stars_limit_mag    = DatabaseGenerator::limit_magnitude (&stars, magnitude_min, magnitude_max);
	println!("\tLimiting Double Stars.");
	let stars_limit_double = DatabaseGenerator::limit_double_stars(&stars_limit_mag, double_star_tolerance);
	println!("\tLimiting Regions.");
	let stars_limit_reg    = DatabaseGenerator::limit_regions(&stars_limit_double, region_size, region_num);

	println!("\tCreating Database.");	

	let gen : DatabaseGenerator = DatabaseGenerator::gen_database(&stars_limit_reg, fov, fov / 1.3, angle_tolerance);
	let database = gen.get_database();












	// Converts the database to a string so it can be input into a file.
	let mut k_vector_str = String::with_capacity(database.k_vector.size() * 5);
	for i in 0..database.k_vector.size() - 1
	{
		k_vector_str.push_str(&format!("\t{},\n", database.k_vector.get(i)).to_string());
	}
    k_vector_str.push_str(&format!("\t{}", database.k_vector.get(database.k_vector.size() - 1)).to_string());


	let mut pairs_str = String::with_capacity(database.pairs.size() * 20);
	for i in 0..database.pairs.size() - 1
	{
		pairs_str.push_str(&format!("\t[{}, {}],\n", database.pairs.get(i).0, database.pairs.get(i).1).to_string());
	}
    pairs_str.push_str(&format!("\t[{}, {}]", database.pairs.get(database.pairs.size() - 1).0, database.pairs.get(database.pairs.size() - 1).1).to_string());




	let mut catalog_str = String::with_capacity(database.catalogue.size() * 5);
	for i in 0..database.catalogue.size()
	{
		catalog_str.push_str(&format!("\t{{\"Equatorial\": {{\"ra_radians\": {}, \"dec_radians\": {}}}}},\n", 
            database.catalogue.get(i).ra.0, database.catalogue.get(i).dec.0).to_string());
	}
    catalog_str.push_str(&format!("\t{{\"Equatorial\": {{\"ra_radians\": {}, \"dec_radians\": {}}}}}", 
        database.catalogue.get(database.catalogue.size() - 1).ra.0, 
        database.catalogue.get(database.catalogue.size() - 1).dec.0).to_string());




	// Calculates the expected size of the database.
	println!("K Vector:  {} elements      \t {} B at 32bit \t {} B at 64bit", database.k_vector.size(), database.k_vector.size() * 4, database.k_vector.size() * 8);
	println!("Star Pair: {} elements   \t {} B at 32bit \t {} B at 64bit", database.pairs.size(), database.pairs.size() * 8, database.pairs.size() * 16);
	println!("Catalogue: {} elements    \t {} B at 32bit \t {} B at 64bit", database.catalogue.size(), database.catalogue.size() * 8, database.catalogue.size() * 16);
	println!("\n                              \t~{} kB @ 32 bit \t\t ~{} kB @ 64 bit", 
		(database.k_vector.size() * 4 + database.pairs.size() * 8 + database.catalogue.size() * 8).div_ceil(1000),
		(database.k_vector.size() * 8 + database.pairs.size() * 16 + database.catalogue.size() * 16).div_ceil(1000),
	);





	// Outputs database. 
	let output = format!(r#"
{{
"angle_tolerance":   {},
"max_magnitude":     {},
"region_size":       {},
"region_number":     {},
"field_of_view_deg": {},
"field_of_view_rad": {},


"k_lookup": {{
    "gradient":  {},
    "intercept": {},
    "min_value_rad": {},
    "max_value_rad": {},
    "num_bins": {}
}},

"k_vector": [
{}
],

"pairs": [
{}
],

"catalogue": [
{}
]
}}	
	"#, 
	angle_tolerance.to_degrees().0, magnitude_max, region_size.to_degrees().0, region_num, fov.to_degrees().0, fov.0,
	database.k_lookup.gradient, database.k_lookup.intercept, 
	database.k_lookup.min_value.0, database.k_lookup.max_value.0, database.k_lookup.num_bins,
	
	k_vector_str,
	pairs_str,
	catalog_str
);


	star_tracker_database::io::Io::write_to_file(args[0].as_str(), &output);

}