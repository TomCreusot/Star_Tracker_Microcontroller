use crate::util::units::Radians;

pub mod distribute;

/// Distributes a set of points onto a sphere.
pub struct Distribute ( );


/// Just a useful stat struct.
pub struct Distribution
{
	pub avg: Radians,
	pub max: Radians,
	pub min: Radians,
	pub dev: Radians,
}

