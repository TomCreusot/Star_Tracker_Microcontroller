//! Nix contains any functionality which should only function on a computer.
//! This may be due to using the heap, unnecessary code, interfacing with external crates, etc.
//!
//!
//!
//!
use crate::util::aliases::Decimal;
use crate::util::units::Equatorial;
use crate::util::units::Radians;
use crate::util::units::BitField;

use crate::tracking_mode::StarPair;
use crate::tracking_mode::database::KVector;

pub mod template;
pub mod star;
pub mod io;
pub mod cv_image;
pub mod nix_image;
pub mod simulated_image;
pub mod distribution;
pub mod database_generator;

pub use self::nix_image::NixImage;
pub use self::simulated_image::SimulatedImage;

/// File management (Excluding Images).
pub struct Io ( );

/// Wrapper of trait `image_processing::Image` for `extern crate RGBImage`.
// pub struct NixImage
// {
// 	pub img_rgb: RgbImage,
// }

/// Wrapper for opencv::core::Mat.
pub struct CVImage ( opencv::core::Mat );

#[derive(Debug, /*Deserialize,*/ Clone)]
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



/// Tool to help construct and analyse the database.
pub struct DatabaseGenerator
{
	// The pyramid database can only hold statics.
	pub k_vector      : Vec<usize>,
	// The pyramid database can only hold statics.
	pub pairs         : Vec<StarPair<usize>>,
	// The pyramid database can only hold statics.
	pub pairs_region : Vec<BitField>,
	// The pyramid database can only hold statics.
	pub catalogue     : Vec<Equatorial>,

	pub num_regions   : usize,

	fov : Radians,
	k_lookup: KVector,
}

pub struct Distribute ( );

pub struct Distribution
{
	pub avg: Radians,
	pub max: Radians,
	pub min: Radians,
	pub dev: Radians,
}



/// A template file
pub struct Template
{
	// The values to replace.
	keys   : Vec<String>,
	// The values to replace the keys by.
	values : Vec<String>,
}




/// A set of colors, which can be converted to a string or byte array.
#[derive(Debug, Copy, Clone)]
pub enum Color
{
	// Primary
	Red, Green, Blue,

	// Primary Offset
	Orange, Turquoise, Purple,

	// Primary Dark
	Maroon, GreenDark, Navy,

	// Primary Light
	Pink, Lime, Sky,

	// Other
	Yellow,	Lavender,

	// Gray
	White, GreyLight, Grey, GreyDark, Black,

	Custom ( u8, u8, u8 ),
}

impl Color
{
	// Returns a color value for the enum.
	pub fn get_color ( &self ) -> [u8; 3]
	{
		match self
		{
			Color::White		=> { return [255, 255, 255];	}
			Color::GreyLight	=> { return [200, 200, 200];	}
			Color::Grey			=> { return [125, 125, 125];	}
			Color::GreyDark		=> { return [90, 90, 90];		}
			Color::Black		=> { return [0, 0, 0];			}

			Color::Red			=> { return [255, 0, 0];		}
			Color::Green		=> { return [0, 128, 0];		}
			Color::Blue			=> { return [0, 0, 255];		}

			Color::Orange		=> { return [200, 100, 30];		}
			Color::Turquoise	=> { return [30, 200, 100];		}
			Color::Purple		=> { return [100, 30, 200];		}

			Color::Maroon		=> { return [100, 0, 0];		}
			Color::GreenDark	=> { return [0, 100, 0];		}
			Color::Navy			=> { return [0, 0, 150];		}

			Color::Pink			=> { return [255, 128, 128];	}
			Color::Lime			=> { return [128, 255, 128];	}
			Color::Sky			=> { return [128, 128, 255];	}

			Color::Yellow		=> { return [255, 255, 25];		}
			Color::Lavender		=> { return [220, 190, 255];	}

			Color::Custom(r,g,b)=> { return [*r, *g, *b];		}
		}
	}
}
