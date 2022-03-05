//! This is where you should set the config variables for the program to run.
//! Change the appropreate values to calibrate the software to your setup.
//! Most of these values are only relevent for a microcontroller.
//! The reason for the microcontroller variables is for memory, this can be ignored for a computer.
//! The following methodology is implemented by associated consts

#![allow(unused_imports)]
use crate::util::aliases::Decimal;
use crate::util::aliases::DECIMAL_PRECISION;
// use crate::util::aliases::UInt;
use crate::util::units::Radians;
// use crate::util::units::Degrees;
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

impl NixConsts for NixConstsStruct
{
/// The file name for the program to read in.
const INPUT_IMAGE_NAME	: &'static str	= "image_in.png";
/// The file name for the program to output results to.
const OUTPUT_IMAGE_NAME	: &'static str	= "image_out.png";

/// The location of the hipacaros database in a CSV style setup.
/// If the database is not already installed, it will install.
const HYG_DATABASE_URL	: &'static str	= "https://raw.githubusercontent.com/astronexus/HYG-Database/master/hygdata_v3.csv";


/// The local path to the hipacaros csv database.
/// If the file is not at the given path, it will be downloaded.
const HYG_DATABASE_PATH	: &'static str	= "database/hyg.csv";


/// If Declination is in degrees format
const HYG_DATABASE_DEC_DEGREES				: bool = true;
/// If Right Ascention is in hours format
const HYG_DATABASE_RA_HOURS					: bool = true;
/// The column name for apparent magnitude for the HYG Database.
const HYG_DATABASE_HEADER_MAGNITUDE 		: &'static str = "mag";
/// The column name for right ascention for the HYG Database.
const HYG_DATABASE_HEADER_RIGHT_ASCENTION	: &'static str = "ra";
/// The column name for declination for the HYG Database.
const HYG_DATABASE_HEADER_DECLINATION		: &'static str = "dec";
/// The column name for specularity for the HYG Database.
const HYG_DATABASE_HEADER_SPECULARITY		: &'static str = "spect";
}

//###############################################################################################//
//###############################################################################################//
//
//									Attitude Determination
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// - QUEST.
//
//
//###############################################################################################//
//###############################################################################################//

impl AttitudeDeterminationConsts for AttitudeDeterminationConstsStruct
{
/// For quest algorithm, to find the correct attitude, the neuton raphson method is used.  
/// This method will loop and slowely decrease the gap between the current and previous prediction.
/// Acheiving perfect precision comparing the 2 values will take up computation power.
/// By specifying a precision, the computational requirements are lowered.
const LAMBDA_PRECISION		:	Decimal		= 0.1;//DECIMAL_PRECISION * 10000000.0;//100000.0;
	
}



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

impl TrackingModeConstructConsts for TrackingModeConstructConstsStruct
{
/// The maximum magnitude to be stored in the database.
const MAGNITUDE_MAX			: Decimal					= 4.8;



/// The bins are a lookup table.
/// If there is not enough bins, there is a massive performance hit.
/// If there is too many bins, there is no speed benifit and it takes up memory.
/// Test this with multiple sizes to get the best for the database.
const BINS_NUM				: usize						= 4000;

/// The maximum field of view of the sensor.
/// To save memory, make this smaller.
const FOV					: Radians					= Radians(30.0 / 180.0 * M_PI);
// const FOV					: Radians					= Radians(0.17453292519); // 10 degrees
}



impl TrackingModeConsts for TrackingModeConstsStruct
{
/// When searching for values in the database, memory required must be forward declared.
/// This should be the maximum number of possible matches until it gives up.
const PAIRS_MAX			: usize					= 1000;

/// The number of triangles to find from the database. 
const TRIANGLES_MAX		: usize					= 1000;

/// When comparing constellation, triangles are used.
/// Specularity is the test to see if the triangle is flipped.
/// If the triangle is flipped, it is invalid.
/// HOWEVER if a triangles area is too small (i.e. a strait line or small), any inaccuracy could cause it to be considered flipped.
/// Use this to define the minimum specularity until the specularity is unimportant.
const SPECULARITY_MIN		: Decimal			= 0.0001;

/// This value should be the maximum inaccuracy between pixels.
/// When searching the database, the database will consider anything within this range as valid.
/// If the value is too large, some values may not fit (max: max_database_matches) and the actual value may not be added.
/// If the value is too small, the actual value may not be found.
const ANGLE_TOLERANCE		: Radians			= Radians(0.0001);
}





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
impl ImageProcessingConsts for ImageProcessingConstsStruct
{
/// The MAXIMUM width/height of the image.
const IMAGE_SIZE_MAX			: Pixel					= Pixel{ x: 1920, y: 1080 };

/// The MAXIMUM number of pixels in a star.
/// If this is too low, it will consider a star as multiple stars.
const BLOB_SIZE_MAX				: usize					= 50;
}










//###############################################################################################//
//
//
//
//
//
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! //
//
//										DONT TOUCH!
//
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! //
//
//
//
//
//
//###############################################################################################//
// This method uses associated constants

pub struct ImageProcessingConstsStruct();
pub struct NixConstsStruct();
pub struct TrackingModeConstructConstsStruct();
pub struct TrackingModeConstsStruct();
pub struct AttitudeDeterminationConstsStruct();


/// These constants are required for the image processing part of the application.
pub trait ImageProcessingConsts
{
	/// The MAXIMUM width/height of the image.
	const IMAGE_SIZE_MAX			: Pixel;

	/// The MAXIMUM number of pixels in a star.
	/// If this is too low, it will consider a star as multiple stars.
	const BLOB_SIZE_MAX				: usize;
}


/// When running a nix application, these are the constants that may be required.
pub trait NixConsts
{
	/// The file name for the program to read in.
	const INPUT_IMAGE_NAME	: &'static str;
	/// The file name for the program to output results to.
	const OUTPUT_IMAGE_NAME	: &'static str;

	/// The location of the hipacaros database in a CSV style setup.
	/// If the database is not already installed, it will install.
	const HYG_DATABASE_URL	: &'static str;


	/// The local path to the hipacaros csv database.
	/// If the file is not at the given path, it will be downloaded.
	const HYG_DATABASE_PATH	: &'static str;


	/// If Declination is in degrees format
	const HYG_DATABASE_DEC_DEGREES				: bool;
	/// If Right Ascention is in hours format
	const HYG_DATABASE_RA_HOURS					: bool;
	/// The column name for apparent magnitude for the HYG Database.
	const HYG_DATABASE_HEADER_MAGNITUDE 		: &'static str;
	/// The column name for right ascention for the HYG Database.
	const HYG_DATABASE_HEADER_RIGHT_ASCENTION	: &'static str;
	/// The column name for declination for the HYG Database.
	const HYG_DATABASE_HEADER_DECLINATION		: &'static str;
	/// The column name for specularity for the HYG Database.
	const HYG_DATABASE_HEADER_SPECULARITY		: &'static str;
}


/// When generating the database for the tracking mode, these are the constants required.
pub trait TrackingModeConstructConsts
{
	/// The maximum magnitude to be stored in the database.
	const MAGNITUDE_MAX	: Decimal	= 1.0;//5.5;


	/// Bins / Database Size, Max: 1, Min: 0.00001.
	/// The more bins, the more memory but faster lookup times.
	/// Less bins will result in less memory requirements but multiple comparisons will need to be made.
	const BINS_NUM		: usize		= 1000;

	/// The maximum field of view of the sensor.
	/// To save memory, make this smaller.
	const FOV			: Radians	= Radians(M_PI / 10.0);//5.0);
}



/// When running the tracking mode reconition software, these are the constants required.
pub trait TrackingModeConsts
{
	/// When searching for values in the database, memory required must be forward declared.
	/// This should be the maximum number of possible matches until it gives up.
	const PAIRS_MAX			: usize			= 0;

	/// The number of triangles to find from the database. 
	const TRIANGLES_MAX		: usize			= 0;

	/// When comparing constellation, triangles are used.
	/// Specularity is the test to see if the triangle is flipped.
	/// If the triangle is flipped, it is invalid.
	/// HOWEVER if a triangles area is too small (i.e. a strait line or small), any inaccuracy could cause it to be considered flipped.
	/// Use this to define the minimum specularity until the specularity is unimportant.
	const SPECULARITY_MIN	: Decimal 		= 0.0;
	
	/// This value should be the maximum inaccuracy between pixels.
	/// When searching the database, the database will consider anything within this range as valid.
	/// If the value is too large, some values may not fit (max: max_database_matches) and the actual value may not be added.
	/// If the value is too small, the actual value may not be found.
	const ANGLE_TOLERANCE	: Radians		= Radians(0.0);
}


/// When performing attitude determination
pub trait AttitudeDeterminationConsts
{
/// For quest algorithm, to find the correct attitude, the neuton raphson method is used.  
/// This method will loop and slowely decrease the gap between the current and previous prediction.
/// Acheiving perfect precision comparing the 2 values will take up computation power.
/// By specifying a precision, the computational requirements are lowered.
const LAMBDA_PRECISION		:	Decimal;
	
}