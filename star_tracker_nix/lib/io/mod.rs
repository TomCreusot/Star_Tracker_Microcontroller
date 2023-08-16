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
