#![allow(unused_imports)]
extern crate star_tracker_lib;
extern crate star_tracker_nix;
extern crate opencv;
extern crate open;

// use reqwest;

use std::time::Duration;

use opencv::imgcodecs::imread;
use opencv::imgcodecs::ImreadModes;

use opencv::core::Mat;
use opencv::core::Size;
use opencv::core::Scalar;
use opencv::core::Point;
use opencv::imgproc::circle;
use opencv::imgproc::line;
use opencv::imgproc::put_text;

use opencv::highgui::wait_key;
use opencv::highgui::imshow;

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
use star_tracker_lib::projection::SpaceCamera;
use star_tracker_lib::projection::SpaceWorld;

use star_tracker_lib::tracking_mode::Constellation;
use star_tracker_lib::tracking_mode::StarPyramid;
use star_tracker_lib::tracking_mode::StarPair;
use star_tracker_lib::tracking_mode::Specularity;
use star_tracker_lib::tracking_mode::StarTriangleIterator;
// use star_tracker_lib::tracking_mode::AbandonSearch;
use star_tracker_lib::tracking_mode::database::ChunkIterator;
use star_tracker_lib::tracking_mode::database::ChunkIteratorNone;
use star_tracker_lib::tracking_mode::database::ChunkIteratorRegional;
use star_tracker_lib::tracking_mode::database::ChunkAreaSearch;
use star_tracker_lib::tracking_mode::database::ChunkIteratorEquatorial;
use star_tracker_lib::tracking_mode::database::ChunkIteratorDeclination;

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
	const ANGLE_TOLERANCE : Radians = Degrees(0.3).as_radians(); 	// Maximum inaccuracy.
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
	const MAGNITUDE_MAX: Decimal = 3.3;


	// Region Reduction
	const REGION_SIZE  : Radians = Degrees(30.0).as_radians(); // An area smaller than FOV.
	const REGION_NUM   : usize   = 8;   // Should not be more than 1 redundant star in a region.
	
	// If stars are this close, one is excluded.
	const DOUBLE_STAR_TOLERANCE : Radians = TrackingConstsTest::ANGLE_TOLERANCE;

	// The Diagonal field of view.
	const FOV          : Radians = Degrees(46.704).as_radians();

	// Loose conditions
	const TIME_GOOD    : u128 = 100000; // ms until autofail.
	
	
	
	
	
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
		// if star.pos.angle_distance(center) < FOV / 2.0
		{
		stars.push(star);}
	}
	
	stars.sort(); // The magnitude must be sorted to get best results for `limit_regions`
	println!("\tLimiting Magnitude.");
	let stars_limit_mag    = DatabaseGenerator::limit_magnitude (&stars, MAGNITUDE_MIN, MAGNITUDE_MAX);
	println!("\tLimiting Double Stars.");
	let stars_limit_double = DatabaseGenerator::limit_double_stars(&stars_limit_mag, DOUBLE_STAR_TOLERANCE);
	println!("\tLimiting Similar.");
	// let stars_limit_sim    = DatabaseGenerator::limit_similar(&stars_limit_double, Degrees(0.0001).to_radians());
	println!("\tLimiting Regions.");
	let stars_limit_reg    = DatabaseGenerator::limit_regions(&stars_limit_double,REGION_SIZE, REGION_NUM);
	
	println!("\tCreating Database.");
	
	// let gen : DatabaseGenerator = DatabaseGenerator::gen_database(&stars_limit_reg, FOV, FOV, TrackingConstsTest::ANGLE_TOLERANCE);
	let gen : DatabaseGenerator = DatabaseGenerator::gen_database_regional(&stars_limit_reg, FOV, TrackingConstsTest::ANGLE_TOLERANCE, FOV/2.0);
	// let database = gen.get_database();
	println!("A");
	let database = gen.get_database_regional();
	println!("B");
	let mut database_iterator = ChunkIteratorNone::new(&database);
	// let mut database_iterator = ChunkIteratorRegional::new(&database);
	println!("C");
	// let mut database_iterator = ChunkIteratorEquatorial::new(&database, Degrees(45.0).as_radians(), 0.3);
	// let mut database_iterator = ChunkIteratorDeclination::new(&database, Degrees(21.0).as_radians(), 0.2, ChunkIteratorDeclination::randomise_none);
	// let mut database_iterator = ChunkIteratorDeclination::new(&database, FOV, 1.5, ChunkIteratorDeclination::randomise_parity);

	// let mut database_iterator = ChunkAreaSearch::from_point(&database, center, Degrees(35.954).as_radians());
	
	
	let mut prev = 0;
	println!("bin tolerance {}", database_iterator.get_database().get_k_lookup().gradient);
	for i in 0..database_iterator.get_database().get_k_vector_size()
	{
		let diff = database_iterator.get_database().get_k_vector(i) - database_iterator.get_database().get_k_vector(prev);
		println!("{}\t{:.3}\t{}", i, Radians(i as Decimal * database_iterator.get_database().get_k_lookup().gradient + database_iterator.get_database().get_k_lookup().intercept).to_degrees(), diff);
		prev = i;
	}
	
	
	println!("{} stars, {} pairs.", stars_limit_reg.len(), database.pairs.size());








//###############################################################################################//
//							--- Image Processing / Blob Detection ---
//###############################################################################################//

	println!("Performing Image Processing and Blob Detection");
	
	// Read image as grayscale.
	let mut img = CVImage(imread( "8mm_filterless.png", ImreadModes::IMREAD_COLOR as i32 )
	// let mut img = CVImage(imread( "starlight_6mm_46.704fov.png", ImreadModes::IMREAD_COLOR as i32 )
		.expect("Could not find img.png in root."));
		
	let dark_frame = CVImage(imread("dark_frame.png", ImreadModes::IMREAD_COLOR as i32).expect(""));
		
	for x in 0..img.width()
	{
		for y in 0..img.height()
		{
			let px = Pixel{x: x, y: y};
			if 10 < dark_frame.get(px)
			{
				img.set(px, 0);
			}
		}
	}
		
	let mut img_consumable = CVImage::new(Pixel{x: img.width(), y: img.height()});

	// Copy the image
	img.copy_to(&mut img_consumable).unwrap();
	
	
	
	// Threshold image with a percent threshold.
	let mut histogram : [UInt; 255] = [0; 255]; 
	let _ = img_consumable.histogram(&mut histogram);
	let thresh = img_consumable.percent_threshold(0.9999, &histogram);
	
	// Find the blobs in the image.
	let mut blobs : Vec<Blob> = Vec::new();
	Blob::find_blobs::<100>(thresh, &mut img_consumable, &mut blobs);
	
	// Convert the blobs into positions.
	let mut stars_2d : Vec<Vector2> = Vec::new();
	blobs.sort_order(Blob::sort_descending_intensity);
	Blob::to_vector2(&blobs, &mut stars_2d);
	
	for i in 0..stars_2d.len()
	{
		println!(". {:.3}\t{:.3}", stars_2d[i].x + 1.0, stars_2d[i].y + 1.0);
	}
	


	
	// stars_2d.clear();
	// true
	// stars_2d.push_back(Vector2{x: 385.28, y: 417.47});
	// stars_2d.push_back(Vector2{x: 418.27, y: 188.79});
	// stars_2d.push_back(Vector2{x: 327.87, y: 174.26});
	// stars_2d.push_back(Vector2{x: 584.51, y: 533.64});
	// stars_2d.push_back(Vector2{x: 366.37, y: 355.79});
	// stars_2d.push_back(Vector2{x: 359.86, y: 215.18});
	// stars_2d.push_back(Vector2{x: 188.58, y: 473.47});
	// stars_2d.push_back(Vector2{x: 470.86, y: 521.33});
	// stars_2d.push_back(Vector2{x: 501.06, y: 226.09});
	// stars_2d.push_back(Vector2{x: 644.89, y: 351.34});
	// stars_2d.push_back(Vector2{x: 258.77, y: 314.78});
	// stars_2d.push_back(Vector2{x: 512.61, y: 436.91});
	// stars_2d.push_back(Vector2{x: 671.74, y: 412.82});
	// stars_2d.push_back(Vector2{x: 444.62, y: 406.12});
	// stars_2d.push_back(Vector2{x: 360.76, y: 149.24});
	// stars_2d.push_back(Vector2{x: 483.45, y: 235.38});
	// stars_2d.push_back(Vector2{x: 282.90, y: 521.21});
	// stars_2d.push_back(Vector2{x: 670.31, y: 438.23});
	// stars_2d.push_back(Vector2{x: 381.58, y: 168.50});
	// stars_2d.push_back(Vector2{x: 233.82, y: 539.51});
	// stars_2d.push_back(Vector2{x: 134.63, y: 535.99});
	// stars_2d.push_back(Vector2{x: 320.04, y: 222.28});
	// stars_2d.push_back(Vector2{x: 492.76, y: 474.76});
	// stars_2d.push_back(Vector2{x: 491.93, y: 195.12});
	// stars_2d.push_back(Vector2{x: 492.85, y: 547.38});
	// stars_2d.push_back(Vector2{x: 217.41, y: 455.87});
	// stars_2d.push_back(Vector2{x: 711.48, y: 319.91});
	// stars_2d.push_back(Vector2{x: 529.78, y: 263.36});
	// stars_2d.push_back(Vector2{x: 607.65, y: 579.98});
	// stars_2d.push_back(Vector2{x: 492.66, y: 143.45});
	// stars_2d.push_back(Vector2{x: 424.50, y: 285.88});
	// stars_2d.push_back(Vector2{x: 27.95 , y: 513.03});
	// stars_2d.push_back(Vector2{x: 372.32, y: 281.57});
	// stars_2d.push_back(Vector2{x: 380.04, y: 493.08});
	// stars_2d.push_back(Vector2{x: 545.61, y: 230.27});
	// stars_2d.push_back(Vector2{x: 449.46, y: 131.27});
	// stars_2d.push_back(Vector2{x: 519.07, y:  32.08});	
	
	
	
	
	

	
	
	
	
	
	
	



	// span
// . 384.488    417.896
// . 365.778    355.310
// . 417.412    188.315
// . 328.828    175.047
// . 359.617    213.770
// . 584.172    531.420
// . 21.920     401.637
// . 193.394    471.022
// . 259.852    313.956
// . 206.072    174.061
// . 171.766    356.026
// . 116.147    471.011
// . 147.145    528.025
// . 470.923    522.000
// . 500.996    225.192
// . 514.989    437.923
// . 282.097    520.037

	// reg
// . 384.518	417.813
// . 365.885	355.300
// . 417.277	188.409
// . 328.931	175.018
// . 359.501	213.630
// . 584.000	531.394
// . 22.000 	401.576
// . 193.385	471.000
// . 206.000	174.000
// . 260.000	314.000
// . 172.000	356.000
// . 116.000	471.000
// . 147.000	528.000
// . 471.000	522.000
// . 515.000	438.000
// . 282.000	520.000
// . 501.000	225.000











	// // image
	// stars_2d.push(Vector2{x: 384.49, y: 417.95});
	// stars_2d.push(Vector2{x: 365.70, y: 355.44});
	// stars_2d.push(Vector2{x: 417.36, y: 188.42});
	// stars_2d.push(Vector2{x: 328.72, y: 175.23});
	// stars_2d.push(Vector2{x: 359.59, y: 214.02});
	// stars_2d.push(Vector2{x: 584.21, y: 531.37});
	// 
	// stars_2d.push(Vector2{x: 27.92 , y: 514.33});
	// 
	// stars_2d.push(Vector2{x: 193.46, y: 471.28});
	// 
	// 
	// stars_2d.push(Vector2{x: 259.75, y: 314.02});
	// 
	// 
	// 
	// 
	// stars_2d.push(Vector2{x: 133.75, y: 533.82});
	// stars_2d.push(Vector2{x: 471.12, y: 521.99});
	// stars_2d.push(Vector2{x: 515.16, y: 438.27});
	// stars_2d.push(Vector2{x: 282.01, y: 520.55});
	// stars_2d.push(Vector2{x: 500.91, y: 225.36});
	// 
	// 
	// 
	// stars_2d.push(Vector2{x: 219.60, y: 455.64});
	// stars_2d.push(Vector2{x: 236.00, y: 535.69});
	// stars_2d.push(Vector2{x: 360.46, y: 150.91});
	// stars_2d.push(Vector2{x: 380.89, y: 169.55});
	// stars_2d.push(Vector2{x: 319.99, y: 221.31});
	// stars_2d.push(Vector2{x: 371.60, y: 282.00});
	// stars_2d.push(Vector2{x: 378.70, y: 495.44});
	// stars_2d.push(Vector2{x: 494.03, y: 475.63});
	// stars_2d.push(Vector2{x: 491.03, y: 195.48});
	// stars_2d.push(Vector2{x: 492.62, y: 547.48});
	// stars_2d.push(Vector2{x: 447.39, y: 134.81});
	// stars_2d.push(Vector2{x: 489.24, y: 148.10});
	// stars_2d.push(Vector2{x: 483.48, y: 234.38});
	// stars_2d.push(Vector2{x: 424.37, y: 284.29});
	// stars_2d.push(Vector2{x: 445.73, y: 407.17});
	// stars_2d.push(Vector2{x: 545.29, y: 231.02});
	// stars_2d.push(Vector2{x: 645.25, y: 351.77});
	// stars_2d.push(Vector2{x: 669.57, y: 438.13});
	// stars_2d.push(Vector2{x: 671.07, y: 412.63});
	// stars_2d.push(Vector2{x: 520.04, y: 31.77});	
	// stars_2d.push(Vector2{x: 530.39, y: 262.95});
	// stars_2d.push(Vector2{x: 604.32, y: 573.43});
	// stars_2d.push(Vector2{x: 705.47, y: 322.38});
	// 
	
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
	let reference_forward = Vector3{x: 0.0, y: 0.0, z: 1.0}.to_equatorial();
	let reference_up      = Vector3{x: 0.0, y: 1.0, z: 0.0}.to_equatorial();
	let extrinsic_projection = ExtrinsicParameters::look_at(reference_forward, reference_up)
		.expect("Ensure entrinsic projection up and forward are not the same value.");
	
	// Read the blob positions and convert them to 3d equatorial space.
	let mut stars_3d : Vec<Equatorial> = Vec::new();
	for i in 0..stars_2d.len()
	{
		let point = SpaceImage(stars_2d[i]);
		let camera_space = intrinsic_projection.from_image(point);        // 3d relative to camera.
		let world_space  = extrinsic_projection.from_image(camera_space); // 3d relative to body.
		
		stars_3d.push(world_space.0.to_equatorial());
	}
	
	// stars_3d.clear();
	// 
	// stars_3d.push_back(Equatorial{ra: Degrees(219.92).to_radians(), dec: Degrees(-60.84).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(186.65).to_radians(), dec: Degrees(-63.10).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(187.79).to_radians(), dec: Degrees(-57.11).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(252.17).to_radians(), dec: Degrees(-69.03).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(210.96).to_radians(), dec: Degrees(-60.37).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(191.93).to_radians(), dec: Degrees(-59.69).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(220.48).to_radians(), dec: Degrees(-47.39).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(238.79).to_radians(), dec: Degrees(-63.43).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(189.30).to_radians(), dec: Degrees(-69.14).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(221.97).to_radians(), dec: Degrees(-79.04).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(204.97).to_radians(), dec: Degrees(-53.47).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(229.73).to_radians(), dec: Degrees(-68.68).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(245.09).to_radians(), dec: Degrees(-78.70).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(220.63).to_radians(), dec: Degrees(-64.97).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(183.79).to_radians(), dec: Degrees(-58.75).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(191.57).to_radians(), dec: Degrees(-68.11).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(228.07).to_radians(), dec: Degrees(-52.10).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(250.77).to_radians(), dec: Degrees(-77.52).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(185.34).to_radians(), dec: Degrees(-60.40).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(227.98).to_radians(), dec: Degrees(-48.74).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(224.63).to_radians(), dec: Degrees(-43.13).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(193.65).to_radians(), dec: Degrees(-57.18).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(234.18).to_radians(), dec: Degrees(-66.32).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(184.39).to_radians(), dec: Degrees(-67.96).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(243.86).to_radians(), dec: Degrees(-63.69).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(219.47).to_radians(), dec: Degrees(-49.43).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(216.73).to_radians(), dec: Degrees(-83.67).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(195.57).to_radians(), dec: Degrees(-71.55).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(260.50).to_radians(), dec: Degrees(-67.77).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(176.40).to_radians(), dec: Degrees(-66.73).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(201.00).to_radians(), dec: Degrees(-64.54).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(220.49).to_radians(), dec: Degrees(-37.79).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(200.66).to_radians(), dec: Degrees(-60.99).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(229.38).to_radians(), dec: Degrees(-58.80).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(188.12).to_radians(), dec: Degrees(-72.13).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(177.42).to_radians(), dec: Degrees(-63.79).to_radians()});
	// stars_3d.push_back(Equatorial{ra: Degrees(160.74).to_radians(), dec: Degrees(-64.39).to_radians()});
	
//##############################################################################################//
//							--- Tracking Mode ---
//##############################################################################################//

	println!("Performing Pyramid Identification");
	let timer = std::time::Instant::now();

	// Finds a `constellation` which is 4 stars forming a pyramid shape.
	// Refer to star_tracker_lib::tracking_mode::{mod, Constellation::find}
	let mut found_all : Vec<Match<usize>> = Vec::new();
	Constellation::find_all::<TrackingConstsTest>(
		&stars_3d, &mut database_iterator,
		&mut StarTriangleIterator::<{TrackingConstsTest::PAIRS_MAX}>::new(),
		&mut Specularity::Ignore, 
		&SearchTimeout::start_timer(Duration::from_millis(TIME_GOOD as u64)),
		2,
		&mut found_all
	);

	let mut found_stars : Vec<Match<Vector3>> = Vec::new();
	
	println!("\tTime taken: {}ms", timer.elapsed().as_millis());
	
	for i in 0..found_all.size()
	{
		let input  = stars_3d[found_all[i].input].to_vector3();
		let output = database_iterator.get_database().get_catalogue(found_all[i].output).to_vector3();
		found_stars.push_back( Match{ input:  input, output: output, weight: 1.0 } );
		
		let mut name = &stars_limit_reg[found_all[i].output].name;
		if name.len() == 0 { name = &stars_limit_reg[found_all[i].output].bf; }
		if name.len() == 0 { name = &stars_limit_reg[found_all[i].output].hip; }

		let mut color = Scalar::new(0.0, 255.0, 255.0, 0.0); // Red color (BGR format)
		let thickness = 1;
		let radius    = 10;
		let px_loc = stars_2d[found_all[i].input];
		let px_pt = Point::new(px_loc.x as i32, px_loc.y as i32);
		circle(&mut img.0, px_pt, radius, color, thickness, 1, 0).unwrap();
		
		color = Scalar::new(0.0, 0.0, 255.0, 0.0); // Red color (BGR format)
		let f_text = format!("{}", i);
		let f_scale = 1.0;
		let f_face = opencv::imgproc::FONT_HERSHEY_SIMPLEX;
		put_text(&mut img.0, &f_text, px_pt, f_face, f_scale, color, thickness, 8, false).unwrap();
		
		println!("{}: {}\t{}", i, name,
			database_iterator.get_database().get_catalogue(found_all[i].output));
	}
	
	
	let rotate_to_cam  : Quaternion = Quest::estimate::<AttitudeDeterminationConstsTest> ( &found_stars );
	let rotate_to_world: Quaternion = rotate_to_cam.conjugate();
	let world_center = rotate_to_world.rotate_point(reference_forward.to_vector3());
	println!("Found Center: {}", print_standard_equatorial(world_center.to_equatorial()));
	
	
	
	println!("{:?}", rotate_to_cam.to_angle_axis());
	println!("{:?}", rotate_to_world.to_angle_axis());
		
	
	// for i in 0..stars_limit_reg.len()
	// {
	// 	let star = rotate_to_cam.rotate_point(stars_limit_reg[i].pos.to_vector3());
	// 
	// 	// if star.angle_distance(reference_forward.to_vector3()) < FOV/ 2.0
	// 	{
	// 		let center = Point::new(img.width() as i32 / 2, img.height() as i32 / 2); 
	// 		let mut star_world_space = SpaceWorld(star);
	// 		let mut star_camera_space = extrinsic_projection.to_image(star_world_space);
	// 		let mut star_pixel_space  = intrinsic_projection.to_image(star_camera_space);
	// 		let mut star_point_space  = Point::new(star_pixel_space.0.x as i32, star_pixel_space.0.y as i32);// + center;
	// 
	// 		let circle_thickness = 1;
	// 		let circle_radius    = (MAGNITUDE_MAX - stars_limit_reg[i].mag) as i32;
	// 
	// 		let mut circle_color = Scalar::new(0.0, 100.0, 0.0, 0.0); // Red color (BGR format)
	// 		circle(&mut img.0,star_point_space,circle_radius,circle_color,circle_thickness,1,0,
	// 		).unwrap();
	// 
	// 		if ( stars_limit_reg[i].name.eq("Hadar") )
	// 		{
	// 			circle_color.0[0] = 255.0;
	// 			circle_color.0[1] = 255.0;
	// 
	// 
	// 			let _ =line(&mut img.0, center, star_point_space, circle_color, circle_thickness, 1, 0);
	// 		}
	// 		let mut circle_color = Scalar::new(0.0, 100.0, 0.0, 0.0); // Red color (BGR format)
	// 		if ( stars_limit_reg[i].name.eq("Rigil Kentaurus") )
	// 		{
	// 			circle_color.0[0] = 255.0;
	// 			circle_color.0[1] = 0.0;
	// 			circle_color.0[2] = 255.0;
	// 
	// 			let _ = line(&mut img.0,center,star_point_space,circle_color,circle_thickness,1,0);
	// 		}
	// 
	// 		if ( stars_limit_reg[i].name.eq("Gacrux") || stars_limit_reg[i].name.eq("Acrux") || stars_limit_reg[i].name.eq("Mimosa") || stars_limit_reg[i].name.eq("Imai"))
	// 		{
	// 			circle_color.0[0] = 255.0;
	// 			circle_color.0[1] = 255.0;
	// 			circle_color.0[2] = 255.0;
	// 			circle(&mut img.0,star_point_space,circle_radius,circle_color,circle_thickness,1,0,
	// 			).unwrap();
	// 		}
	// 	}
	// }
		
	let _ = imshow("Image", &img.0);
	let _ = wait_key(0).unwrap();
	

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