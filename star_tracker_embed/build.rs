extern crate serde_json;

use std::env;
use std::fs::File;
use std::io::Write;

use serde_json::json;
use serde_json::Value;

fn main ( )
{
	let file_json = "database.json";
	let file_rust = "src/database/database.rs";
//	let text_json = ; // ...
	
	
//	let root : Value  = serde_json::from_str(&text_json).expect("Invalid Json File Contense");
//	let magnitude_max = &root["magnitude_max"];
//	let bins_num      = &root["bins_num"];
//	let fov           = &root["fov"];	
	
	let file = r#"
		/// This is the database for the star tracker.
		/// It is generated from a template file in src/config/template.txt.
		/// It was generated with a:
		/// * fov of $(FOV)
		/// * magnitude of $(MAGNITUDE)
		/// * number of bins of $(BIN_SIZE)
	
		use star_tracker_lib::util::units::Radians;
		use star_tracker_lib::util::units::Equatorial;
		use star_tracker_lib::util::aliases::Decimal;
		use star_tracker_lib::tracking_mode::database::KVector;
		use star_tracker_lib::tracking_mode::database::PyramidDatabase;
		use star_tracker_lib::tracking_mode::StarPair;
	
	
	
	
		// This is here instead of pyramid_database.rs to fix dependancy issues.
		impl <'a> PyramidDatabase <'a>
		{
			/// Constructs a new pyramid database using the pregenerated databases (run database_generator).
			pub fn new ( ) -> Self
			{
				return Self
				{
					fov: 		FOV,
					k_lookup:	K_LOOKUP_DATABASE,
					k_vector:	&K_VECTOR_DATABASE,
					pairs:		&STAR_PAIR_DATABASE,
					catalogue:	&CATALOGUE_DATABASE,
				};
			}
		}

		/// The field of view of the sensor in the current configuration.
		pub static FOV : Radians = Radians($(FOV));


		/// The maximum magnitude viewable by the sensor in the current configuration.
		pub static MAGNITUDE : Decimal = $(MAGNITUDE);

		/// The KVector Equation
		pub static K_LOOKUP_DATABASE : KVector = $(K_LOOKUP);

		/// The KVector Database (Bins pointing to the general area of the star pairs).
		pub static K_VECTOR_DATABASE : [usize; $(K_VECTOR_SIZE)] =
		[
			$(K_VECTOR_ELEMENTS)
		];

		/// The Star Pair Database (Location of A in catalogue, Location of B in catalogue).
		pub static STAR_PAIR_DATABASE : [StarPair<usize>; $(STAR_PAIR_SIZE)] =
		[
		$(STAR_PAIR_ELEMENTS)
		];

		/// The Catalogue Database (Position of stars).
		pub static CATALOGUE_DATABASE : [Equatorial; $(CATALOGUE_SIZE)] =
		[
		$(CATALOGUE_ELEMENTS)
		];
	"#;
	
	let mut output = File::create(&file_rust).unwrap();
	output.write_all(file.as_bytes()).unwrap();
}
	



