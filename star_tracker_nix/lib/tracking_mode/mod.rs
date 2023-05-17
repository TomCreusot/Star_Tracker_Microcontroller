//! Nix contains any functionality which should only function on a computer.
//! This may be due to using the heap, unnecessary code, interfacing with external crates, etc.
use star_tracker_lib::util::units::Equatorial;
use star_tracker_lib::util::units::Radians;
use star_tracker_lib::util::err::Error;

use star_tracker_lib::tracking_mode::StarPair;
use star_tracker_lib::tracking_mode::database::KVector;

pub mod star_database_element;
pub mod database_generator;
pub mod k_vector;


pub trait KVectorGenerator
{
	fn generate_bins ( &self, sorted_database: &Vec<StarDatabaseElement> ) ->Error<Vec<usize>>;
	fn display ( &self ) -> String;
}

/// An element with all the details required to insert into the database.
#[derive(Clone, Copy, Debug)]
pub struct StarDatabaseElement
{
	/// The location of the stars (does not matter what order they are in).
	pub pair : StarPair<usize>,
	/// The angular separation between the stars.
	pub dist : Radians,
}



/// Tool to help construct and analyse the database.
pub struct DatabaseGenerator
{
	// The pyramid database can only hold statics.
	pub k_vector      : Vec<usize>,
	// The pyramid database can only hold statics.
	pub pairs         : Vec<StarPair<usize>>,
	// The pyramid database can only hold statics.
	pub catalogue     : Vec<Equatorial>,

	fov : Radians,
	k_lookup: KVector,
}
