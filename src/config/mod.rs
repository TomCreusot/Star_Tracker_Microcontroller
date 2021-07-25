//! This is where you should set the config variables for the program to run.
//! Change the appropreate values to calibrate the software to your setup.
//! Most of these values are only relevent for a microcontroller.
//! The reason for the microcontroller variables is for memory, this can be ignored for a computer.

use crate::util::aliases::Decimal;
use crate::util::aliases::UInt;
use crate::util::units::Radians;
use crate::util::units::Pixel;
use crate::util::aliases::M_PI;




//###############################################################################################//
//###############################################################################################//
//
//										Nix
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Nix (UNIX) is for variables which are not required/cannot be used on a microcontroller.
// As nix is for computers, it requires locations to write/read images etc.
//
//###############################################################################################//
//###############################################################################################//

/// The file name for the program to read in.
pub const INPUT_IMAGE_NAME	: &str	= "image_in.png";
/// The file name for the program to output results to.
pub const OUTPUT_IMAGE_NAME	: &str	= "image_out.png";

/// The location of the hipacaros database in a CSV style setup.
/// If the database is not already installed, it will install.
pub const HYG_DATABASE_URL	: &str	= "https://raw.githubusercontent.com/astronexus/HYG-Database/master/hygdata_v3.csv";


/// The local path to the hipacaros csv database.
/// If the file is not at the given path, it will be downloaded.
pub const HYG_DATABASE_PATH	: &str	= "database/hyg.csv";


/// If Declination is in degrees format
pub const HYG_DATABASE_DEC_DEGREES				: bool = true;
/// If Right Ascention is in hours format
pub const HYG_DATABASE_RA_HOURS					: bool = true;
/// The column name for apparent magnitude for the HYG Database.
pub const HYG_DATABASE_HEADER_MAGNITUDE 		: &str = "mag";
/// The column name for right ascention for the HYG Database.
pub const HYG_DATABASE_HEADER_RIGHT_ASCENTION	: &str = "ra";
/// The column name for declination for the HYG Database.
pub const HYG_DATABASE_HEADER_DECLINATION		: &str = "dec";
/// The column name for specularity for the HYG Database.
pub const HYG_DATABASE_HEADER_SPECULARITY		: &str = "spect";

//###############################################################################################//
//###############################################################################################//
//
//										Tracking Mode
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// - Pyramid tracking method.
// - Database.
//
//###############################################################################################//
//###############################################################################################//

//												//
//												//
//				CONSTRUCTING DATABASE			//
//												//
//												//
/// The maximum magnitude to be stored in the database.
pub const DATABASE_MAGNITUDE_MAX		: Decimal				= 2.2;//5.5;

/// This value should be the maximum inaccuracy between pixels.
/// When searching the database, the database will consider anything within this range as valid.
/// If the value is too large, some values may not fit (max: max_database_matches) and the actual value may not be added.
/// If the value is too small, the actual value may not be found.
pub const DATABASE_ANGLE_TOLERANCE		: Decimal				= 0.01;

/// Bins / Database Size, Max: 1, Min: 0.00001.
/// The more bins, the more memory but faster lookup times.
/// Less bins will result in less memory requirements but multiple comparisons will need to be made.
pub const DATABASE_BINS_NUM				: usize					= 100;

/// The maximum field of view of the sensor.
/// To save memory, make this smaller.
pub const DATABASE_FOV					: Radians				= Radians(M_PI / 5.0);


//													//
//													//
//						RUNTIME						//
//													//
//													//

/// When searching for values in the database, memory required must be forward declared.
/// This should be the maximum number of possible matches until it gives up.
pub const DATABASE_MATCHES_MAX			: UInt					= 10;


/// When comparing constellation, triangles are used.
/// Specularity is the test to see if the triangle is flipped.
/// If the triangle is flipped, it is invalid.
/// HOWEVER if a triangles area is too small (i.e. a strait line or small), any inaccuracy could cause it to be considered flipped.
/// Use this to define the minimum specularity until the specularity is unimportant.
pub const TRACKING_MODE_SPECULARITY_MIN	: Decimal				= 0.1;






//###############################################################################################//
//###############################################################################################//
//
//										Image Processing
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// - Reading an image.
// - Thresholding an image.
// - Performing blob detection.
// - Undistorting the points on the image.
// - Converting to equatorial.
//
//###############################################################################################//
//###############################################################################################//

/// The MAXIMUM width of the image.
pub const IMAGE_SIZE_MAX			: Pixel					= Pixel{ x: 1920, y: 1080 };

/// The MAXIMUM number of pixels in a star.
/// If this is too low, it will consider a star as multiple stars.
pub const BLOB_SIZE_MAX				: usize					= 50;
