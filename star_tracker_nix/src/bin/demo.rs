extern crate star_tracker_lib;
extern crate star_tracker_nix;
extern crate opencv;

use std::time::Duration;

use opencv::imgcodecs::imread;
use opencv::imgcodecs::ImreadModes;


use star_tracker_lib::util::aliases::Decimal;
use star_tracker_lib::util::aliases::UInt;
use star_tracker_lib::util::units::Quaternion;
use star_tracker_lib::util::units::Equatorial;
use star_tracker_lib::util::units::Vector3;
use star_tracker_lib::util::units::Vector2;
use star_tracker_lib::util::units::Radians;
use star_tracker_lib::util::units::Degrees;
use star_tracker_lib::util::units::Hours;
use star_tracker_lib::util::units::Pixel;
use star_tracker_lib::util::units::Match;

use star_tracker_lib::util::list::List;

use star_tracker_lib::image_processing::Image;
use star_tracker_lib::image_processing::Blob;

use star_tracker_lib::projection::IntrinsicParameters;
use star_tracker_lib::projection::ExtrinsicParameters;
use star_tracker_lib::projection::SpaceImage;

use star_tracker_lib::tracking_mode::Constellation;
use star_tracker_lib::tracking_mode::StarPyramid;
use star_tracker_lib::tracking_mode::Specularity;
use star_tracker_lib::tracking_mode::StarTriangleIterator;
// use star_tracker_lib::tracking_mode::AbandonSearch;
use star_tracker_lib::tracking_mode::database::ChunkIteratorNone;
use star_tracker_lib::tracking_mode::database::ChunkIteratorRegional;
use star_tracker_lib::tracking_mode::database::ChunkAreaSearch;
// use star_tracker_lib::tracking_mode::database::ChunkIteratorEquatorial;
// use star_tracker_lib::tracking_mode::database::ChunkIteratorDeclination;

use star_tracker_lib::attitude_determination::AttitudeDetermination;
use star_tracker_lib::attitude_determination::Quest;

use star_tracker_lib::config::NixConstsStruct;
use star_tracker_lib::config::NixConsts;
use star_tracker_lib::config::TrackingModeConsts;
use star_tracker_lib::config::ImageProcessingConsts;
use star_tracker_lib::config::AttitudeDeterminationConsts;

use star_tracker_nix::io::Star;
use star_tracker_nix::io::Io;
use star_tracker_nix::tracking_mode::DatabaseGenerator;
use star_tracker_nix::tracking_mode::SearchTimeout;
use star_tracker_nix::image_processing::CVImage;



// Defines how the tracking algorithm should perform.
pub struct TrackingConstsTest ( );
impl TrackingModeConsts for TrackingConstsTest
{
	const PAIRS_MAX       : usize = 2000;							// Irrelevant, ensure big.
	const TRIANGLES_MAX   : usize = 2000;							// Irrelevant, ensure big.
	const SPECULARITY_MIN : Decimal = 0.0001;						// If the triangle is flipped.
	const ANGLE_TOLERANCE : Radians = Degrees(0.1).as_radians(); 	// Maximum inaccuracy.
}

// Defines how blob detection will work.
pub struct ImageProcessingConstsTest ( );
impl ImageProcessingConsts for ImageProcessingConstsTest
{
	/// Size of the image, not needed for this as we are using opencv image.
	const IMAGE_SIZE_MAX : Pixel = Pixel{ x: 0, y: 0 };
	
	/// The MAXIMUM number of pixels in a star.
	/// If this is too low, it will consider a star as multiple stars.
	const BLOB_SIZE_MAX  : usize = 50;
}



pub struct AttitudeDeterminationConstsTest ( );
impl AttitudeDeterminationConsts for AttitudeDeterminationConstsTest
{
/// For quest algorithm, to find the correct attitude, the neuton raphson method is used.  
/// This method will loop and slowly decrease the gap between the current and previous prediction.  
/// Achieving perfect precision comparing the 2 values will take up computation power.  
/// By specifying a precision, the computational requirements are lowered.  
const LAMBDA_PRECISION		:	Decimal		= 0.1;//DECIMAL_PRECISION * 10000000.0;//100000.0;
	
}



pub fn main ( )
{
	// To reduce size of database.
	const MAGNITUDE_MIN: Decimal = -20.0;
	const MAGNITUDE_MAX: Decimal = 5.0;


	// Region Reduction
	const REGION_SIZE  : Radians = Degrees(15.0).as_radians(); // An area smaller than FOV.
	const REGION_NUM   : usize   = 10;   // Should not be more than 1 redundant star in a region.
	
	// If stars are this close, one is excluded.
	const DOUBLE_STAR_TOLERANCE : Radians = TrackingConstsTest::ANGLE_TOLERANCE;

	// The Diagonal field of view.
	const FOV          : Radians = Degrees(35.954).as_radians();

	// Loose conditions
	const TIME_GOOD    : u128 = 100000; // ms until autofail.
	
	
	
	
	
	
	
	let center = Equatorial{ra: Degrees(198.163).as_radians(), dec: Degrees(-62.350).as_radians()};
	
	
	
	println!("Performing Database Construction");
	println!("\tReading database.");
	let mut stars : Vec<Star> = Vec::new();
	let mut rdr = Io::get_csv (
		NixConstsStruct::HYG_DATABASE_PATH,
		NixConstsStruct::HYG_DATABASE_FILE,
		NixConstsStruct::HYG_DATABASE_URL );

	let iter = rdr.deserialize();
	for record in iter
	{
		let star : Star = record.expect("Could not decode.");
		if star.pos.angle_distance(center) < FOV / 2.0
		{
		stars.push(star);}
	}
	
	stars.sort(); // The magnitude must be sorted to get best results for `limit_regions`
	println!("\tLimiting Magnitude.");
	let stars_limit_mag    = DatabaseGenerator::limit_magnitude (&stars, MAGNITUDE_MIN, MAGNITUDE_MAX);
	println!("\tLimiting Double Stars.");
	let stars_limit_double = DatabaseGenerator::limit_double_stars(&stars_limit_mag, DOUBLE_STAR_TOLERANCE);
	println!("\tLimiting regions.");
	let stars_limit_reg    = DatabaseGenerator::limit_regions(&stars_limit_double,REGION_SIZE, REGION_NUM);
	
	let mut stars_limit_region = Vec::new();
	for i in 0..stars_limit_reg.len()
	{
		stars_limit_region.push(stars_limit_reg[i].clone());
	}
	
	println!("\tCreating Database.");
	let gen : DatabaseGenerator = DatabaseGenerator::gen_database(&stars_limit_mag, FOV, TrackingConstsTest::ANGLE_TOLERANCE);
	// let gen : DatabaseGenerator = DatabaseGenerator::gen_database_regional(&stars_limit_reg, FOV, TrackingConstsTest::ANGLE_TOLERANCE, Degrees(51.0).as_radians());
	let database = gen.get_database();
	// let database = gen.get_database_regional();
	// let mut database_iterator = ChunkIteratorNone::new(&database);
	// let mut database_iterator = ChunkIteratorRegional::new(&database);
	// let mut database_iterator = ChunkIteratorEquatorial::new(&database, Degrees(45.0).as_radians(), 0.3);
	// let mut database_iterator = ChunkIteratorDeclination::new(&database, Degrees(21.0).as_radians(), 0.2, ChunkIteratorDeclination::randomise_none);
	// let mut database_iterator = ChunkIteratorDeclination::new(&database, Degrees(50.0).as_radians(), 1.5, ChunkIteratorDeclination::randomise_parity);

	let mut database_iterator = ChunkAreaSearch::from_point(&database, center, Degrees(35.954).as_radians());
	
	println!("{}, {} stars, {} pairs.", stars_limit_mag.len(), stars_limit_reg.len(), database.pairs.size());








//###############################################################################################//
//							--- Image Processing / Blob Detection ---
//###############################################################################################//

	println!("Performing Image Processing and Blob Detection");
	
	// Read image as grayscale.
	let mut img = CVImage(imread( "image.png", ImreadModes::IMREAD_COLOR as i32 )
		.expect("Could not find img.png in root."));
	
	// Threshold image with a percent threshold.
	let mut histogram : [UInt; 255] = [0; 255]; 
	img.histogram(&mut histogram);
	let thresh = img.percent_threshold(0.9999, &histogram);
	
	// Find the blobs in the image.
	let mut blobs : Vec<Blob> = Vec::new();
	Blob::find_blobs::<50>(thresh, &mut img, &mut blobs);
	
	// Convert the blobs into positions.
	let mut stars_2d : Vec<Vector2> = Vec::new();
	blobs.sort_order(Blob::sort_descending_intensity);
	Blob::to_vector2(&blobs, &mut stars_2d);

	// stars_2d.clear();
	// stars_2d.push(Vector2{x: 698.764831543, y: 327.3709106445} );
	// stars_2d.push(Vector2{x: 234.1219024658, y: 423.197052002} );
	// stars_2d.push(Vector2{x: 263.6461486816, y: 255.3522644043});
	// stars_2d.push(Vector2{x: 579.2455444336, y: 360.4039916992});
	// stars_2d.push(Vector2{x: 91.27331542969, y: 567.9310302734});
	// stars_2d.push(Vector2{x: 741.1538085938, y: 79.40607452393});
	// stars_2d.push(Vector2{x: 112.1178665161, y: 197.8943939209});
	// stars_2d.push(Vector2{x: 409.044921875, y: 42.91330337524} );
	// stars_2d.push(Vector2{x: 310.5813293457, y: 365.9774169922});
	// stars_2d.push(Vector2{x: 495.5990905762, y: 560.0828857422});
	// stars_2d.push(Vector2{x: 189.8396759033, y: 361.6262512207});
	// stars_2d.push(Vector2{x: 337.1253967285, y: 98.32127380371});
	// stars_2d.push(Vector2{x: 680.013671875, y: 210.8746948242} );
	// stars_2d.push(Vector2{x: 185.6335144043, y: 116.7000274658});
	// stars_2d.push(Vector2{x: 653.3929443359, y: 465.044921875} );
	// stars_2d.push(Vector2{x: 278.9482727051, y: 115.9033050537});
	// stars_2d.push(Vector2{x: 225.0746459961, y: 323.8058166504});
	// stars_2d.push(Vector2{x: 121.0114822388, y: 260.1293334961});
	// stars_2d.push(Vector2{x: 119.9550933838, y: 528.7115478516});
	// stars_2d.push(Vector2{x: 429.044921875, y: 149.0449066162} );
	// stars_2d.push(Vector2{x: 793.6207275391, y: 213.9304199219});
	// stars_2d.push(Vector2{x: 353.7647094727, y: 131.8087158203});
	// stars_2d.push(Vector2{x: 323.0464782715, y: 440.6865844727});
	// stars_2d.push(Vector2{x: 551.044921875, y: 268.7954711914} );
	// stars_2d.push(Vector2{x: 347.9266052246, y: 13.91238212585});
	// stars_2d.push(Vector2{x: 161.0449066162, y: 197.0449066162});
	// stars_2d.push(Vector2{x: 431.3809204102, y: 180.837387085} );
	// stars_2d.push(Vector2{x: 219.0449066162, y: 195.8061523438});
	// stars_2d.push(Vector2{x: 287.955078125, y: 453.9551086426} );
	// stars_2d.push(Vector2{x: 83.04490661621, y: 351.8539733887});
	// stars_2d.push(Vector2{x: 247.0072021484, y: 223.6776123047});
	// stars_2d.push(Vector2{x: 91.95509338379, y: 377.955078125} );
	// stars_2d.push(Vector2{x: 739.044921875, y: 259.044921875}  );
	
	for e in &stars_2d
	{
		println!("{}", e);
	}
	
	
	println!("\tFound: {} stars", stars_2d.len());
	
	
//##############################################################################################//
//							--- Projection ---
//##############################################################################################//
	
	println!("Performing Projection");
	// Construct the parameters for an inverse intrinsic projection.
	let img_horizontal = ((img.width() as Decimal).powf(2.0) + (img.height() as Decimal).powf(2.0)).sqrt();
	let intrinsic_projection = IntrinsicParameters::from_fov(FOV, img_horizontal);

	
	// Not really nessisary.
	// If the camera is not on the front of the spacecraft, specify what orientation the camera is.
	// In this case, the camera is pointing +z (relative to the spacecraft) 
	// and the top pixel is in the direction of +y (relative to the space craft).
	let extrinsic_projection = ExtrinsicParameters::look_at(
		Vector3{x: 0.0, y: 0.0, z: 1.0}.to_equatorial(),  // forward
		Vector3{x: 0.0, y: 1.0, z: 0.0}.to_equatorial()   // up
	).expect("Ensure entrinsic projection up and forward are not the same value.");
	
	// Read the blob positions and convert them to 3d equatorial space.
	let mut stars_3d : Vec<Equatorial> = Vec::new();
	for i in 0..stars_2d.len()
	{
		let point = SpaceImage(stars_2d[i]);
		let camera_space = intrinsic_projection.from_image(point);        // 3d relative to camera.
		let world_space  = extrinsic_projection.from_image(camera_space); // 3d relative to body.
		
		stars_3d.push(world_space.0.to_equatorial());
	}
	
	
//##############################################################################################//
//							--- Tracking Mode ---
//##############################################################################################//

	println!("Performing Pyramid Identification");
	let timer = std::time::Instant::now();

	// Finds a `constellation` which is 4 stars forming a pyramid shape.
	// Refer to star_tracker_lib::tracking_mode::{mod, Constellation::find}
	let constellation : Constellation = Constellation::find::<TrackingConstsTest>(
		&stars_3d, &mut database_iterator,
		&mut StarTriangleIterator::<{TrackingConstsTest::PAIRS_MAX}>::new(),
		&mut StarPyramid(0,0,0,0), &mut Specularity::Ignore, 
		&SearchTimeout::start_timer(Duration::from_millis(TIME_GOOD as u64)));

	
	println!("\tTime taken: {}ms", timer.elapsed().as_millis());
	
	let mut found_stars : Vec<Match<Vector3>> = Vec::new();
	match constellation
	{
		Constellation::None          => 
		{ 
			println!("FAILED, no constellation could be found in the image.");
		}
		Constellation::Triangle(tri) => 
		{
			println!("Found triangle, (This is an unsafe result, don't trust it)...");
			found_stars.push(Match{input: tri.input.0.to_vector3(), output: tri.output.0.to_vector3(), weight: tri.weight});
			found_stars.push(Match{input: tri.input.1.to_vector3(), output: tri.output.1.to_vector3(), weight: tri.weight});
			found_stars.push(Match{input: tri.input.2.to_vector3(), output: tri.output.2.to_vector3(), weight: tri.weight});
		}
		Constellation::Pyramid(pyr)  => 
		{ 
			println!("Succesfully found pyramid..."); 
			found_stars.push(Match{input: pyr.input.0.to_vector3(), output: pyr.output.0.to_vector3(), weight: pyr.weight});
			found_stars.push(Match{input: pyr.input.1.to_vector3(), output: pyr.output.1.to_vector3(), weight: pyr.weight});
			found_stars.push(Match{input: pyr.input.2.to_vector3(), output: pyr.output.2.to_vector3(), weight: pyr.weight});
			found_stars.push(Match{input: pyr.input.3.to_vector3(), output: pyr.output.3.to_vector3(), weight: pyr.weight});
		}
	}
	
	if 0 < found_stars.len()
	{
		for i in 0..found_stars.len()
		{
			let star_in = found_stars[i].input.to_equatorial();
			let star_out = found_stars[i].output.to_equatorial();
			println!("{}\t\t{}\t{}", print_standard_equatorial(star_in), print_standard_equatorial(star_out), star_out);
		}
		
		let rotation : Quaternion = Quest::estimate::<AttitudeDeterminationConstsTest> ( &found_stars );
		println!("{}", print_standard_equatorial(rotation.rotate_point(Vector3{x: 0.0, y: 0.0, z: 1.0}).to_equatorial()));
	}
	

}





/// Prints in standard ra: hours, dec: degrees.
pub fn print_standard_equatorial ( to_print: Equatorial ) -> String
{
	let ra = hour_time_format(to_print.ra.to_hours());

	let dec_degrees = to_print.dec.to_degrees().0;
	
	// let mut dec_hour = self.dec.to_hours();
	// dec_hour = Hours(dec_hour.0.fract());
	// let dec_minutes = dec_hour.minutes();
	// let dec_seconds = dec_hour.seconds();
	let dec_minutes = (dec_degrees.fract() * 60.0).copysign(1.0); // arc minutes (1/60 degree).
	let dec_seconds = (dec_minutes.fract() * 60.0).copysign(1.0); // arc minutes (1/60 degree).
	let dec = format!("{:2.0}° {:2.0}' {:5.2}\"", dec_degrees, dec_minutes, dec_seconds);

	return format!("J200( {} | {} )", ra, dec);
}

// Prints hours in better format.
pub fn hour_time_format ( hours : Hours ) -> String
{
	return format!("{:2.0}h {:2.0}m {:5.2}s", hours.hours(), hours.minutes(), hours.seconds());
}