use star_tracker_lib::util::units::Equatorial;
use star_tracker_lib::util::aliases::Decimal;

pub mod io;
pub mod star;
pub mod template;


/// File management (Excluding Images).
pub struct Io ( );


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

	pub name : String,
}




/// A template file
pub struct Template
{
	// The values to replace.
	keys   : Vec<String>,
	// The values to replace the keys by.
	values : Vec<String>,
}

