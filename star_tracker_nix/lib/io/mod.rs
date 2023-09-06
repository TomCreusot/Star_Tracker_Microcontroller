use star_tracker_lib::util::units::Equatorial;
use star_tracker_lib::util::units::Vector2;
use star_tracker_lib::util::units::Radians;
use star_tracker_lib::util::aliases::Decimal;


pub mod io;
pub mod star;
pub mod sample;
pub mod template;


/// File management (Excluding Images).
pub struct Io ( );

/// A .png, corr.fits file and .log which all describe the image.
pub struct Sample
{
	pub dir:       String,
	pub file_cor:  String,
	pub file_log:  String,
	pub file_dark: String,
	pub file_img:  Vec<String>,
}

/// Provides stats about the error of a single star in an image to where it should be.
pub struct StarError
{

	/// The location of the centroid of the star in the image.
	pub image_px:  Vector2,
	/// Where the centroid of the star should be in the image if the lens was rectilinear.
	pub real_px:   Vector2,


	/// The location of the star relative to the star catalogue.
	/// This shows how far it was from the actual position `real_eq`.
	pub image_eq:  Equatorial,
	/// The location of the star in a star catalogue.
	pub real_eq:   Equatorial,

	/// The great angle distance between image_eq and real_eq.
	pub error_eq:  Radians,
	/// The distance between image_px and real px.
	pub error_px:  Decimal
}



#[derive(Debug, Clone)]
/// Use for CSV serialization
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




/// A template file
pub struct Template
{
	// The values to replace.
	keys   : Vec<String>,
	// The values to replace the keys by.
	values : Vec<String>,
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
	const HYG_DATABASE_URL	: &'static str	= "https://github.com/astronexus/HYG-Database/raw/master/hyg/v3/hyg_v35.csv";


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