//! This module handles interactions with the database and sample sets.  
//!  
//! Use this module to read and write to files as well as read the database.  
//! The database used is the hipparcos database and is sourced from github astronexus.  
use star_tracker_lib::util::units::Equatorial;
use star_tracker_lib::util::aliases::Decimal;


pub mod io;
pub mod star;


/// Easy file management.
pub struct Io ( );


#[derive(Debug, Clone)]
/// Serialized variable for reading the database.
pub struct Star
{
	// #[serde(rename = "mag")]
	pub mag  : Decimal,
	// #[serde(flatten)]
	pub pos  : Equatorial,
	// #[serde(rename = "spect")]
	pub spec : String ,

	/// The official name of the object.
	pub name : String,

	/// The alternate name of the object (if name does not exist).
	pub bf: String,

	/// The hip code of the star.
	pub hip: String
}


/// Variables for reading the Hipparcos database
struct HipparcosAccessorStruct  ();

#[allow(dead_code)]
impl HipparcosAccessorStruct
{
	/// The file name for the program to read in.
	const INPUT_IMAGE_NAME	: &'static str	= "image_in.png";
	/// The file name for the program to output results to.
	const OUTPUT_IMAGE_NAME	: &'static str	= "image_out.png";

	/// The location of the Hipparcos database in a CSV style setup.
	/// If the database is not already installed, it will install.
	const HYG_DATABASE_URL	: &'static str	= "https://github.com/astronexus/HYG-Database/raw/main/hyg/v3/hyg_v37.csv";


	/// The local path to the Hipparcos csv database.
	/// If the file is not at the given path, it will be downloaded.
	const HYG_DATABASE_PATH	: &'static str	= "database/";

	/// The file where the database is stored.
	/// If the file is not at the given path, it will be downloaded.
	const HYG_DATABASE_FILE : &'static str  = "hyg.csv";

	/// If Declination is in degrees format
	const HYG_DATABASE_DEC_DEGREES				: bool = true;
	/// If Right Ascension is in hours format
	const HYG_DATABASE_RA_HOURS					: bool = true;
	/// The column name for apparent magnitude for the HYG Database.
	const HYG_DATABASE_HEADER_MAGNITUDE 		: &'static str = "mag";
	/// The column name for right ascension for the HYG Database.
	const HYG_DATABASE_HEADER_RIGHT_ASCENSION	: &'static str = "ra";
	/// The column name for declination for the HYG Database.
	const HYG_DATABASE_HEADER_DECLINATION		: &'static str = "dec";
	/// The column name for specularity for the HYG Database.
	const HYG_DATABASE_HEADER_SPECULARITY		: &'static str = "spect";
	/// The column name for name for the HYG Database.
	const HYG_DATABASE_HEADER_NAME				: &'static str = "proper";
	/// The column name for hip code for the HYG Database.
	const HYG_DATABASE_HEADER_HIP				: &'static str = "hip";
	/// The column for the alternate name for the HYG Database.
	const HYG_DATABASE_HEADER_BF				: &'static str = "bf";
}