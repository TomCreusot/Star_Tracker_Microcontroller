//! Nix contains any functionality which should only function on a computer.
//! This may be due to using the heap, unnecessary code, interfacing with external crates, etc.
use star_tracker_lib::util::aliases::UInt;
use star_tracker_lib::util::units::Equatorial;
use star_tracker_lib::util::units::BitField;
use star_tracker_lib::util::units::Radians;
use star_tracker_lib::util::err::Error;

use star_tracker_lib::tracking_mode::StarPair;
use star_tracker_lib::tracking_mode::database::KVector;
// use star_tracker_lib::tracking_mode::database::Database;

pub mod star_database_element;
pub mod database_generator;
pub mod k_vector;
pub mod search_timeout;




/// Additional functionality provided to the KVector struct so it can generate a k_vector. 
pub trait KVectorGenerator
{
	fn ideal_bins ( sorted_database: &Vec<StarDatabaseElement>, tolerance: Radians ) -> usize;
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



/// A timer to abort a search.
pub struct SearchTimeout
{
	/// When the timere started.
	pub start_time : std::time::Instant,
	/// How long until giveup.
	pub timeout    : std::time::Duration,
}



/// Tool to help construct and analyse the database.  
/// This is used instead of implementing a trait for the databases as k_vector, k_pairs and catalogue all have lifetimes.
pub struct DatabaseGenerator
{
	/// The pyramid database can only hold statics.
	pub k_vector      : Vec<usize>,
	/// The pyramid database can only hold statics.
	pub pairs         : Vec<StarPair<usize>>,
	/// The pyramid database can only hold statics.
	pub catalogue     : Vec<Equatorial>,

	/// The field of view used when generating the database.
	/// This is the widest a star pair can be.
	fov : Radians,
	k_lookup: KVector,
	/// Used for regional database.
	catalogue_field: Vec<BitField>,
	/// Used for regional database.
	num_fields : UInt,
}

