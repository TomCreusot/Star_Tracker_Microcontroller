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


pub fn main ( )
{
	// To reduce size of database.
	const MAGNITUDE_MIN: Decimal = -20.0;
	const MAGNITUDE_MAX: Decimal = 4.0;


	// The Diagonal field of view.
	const FOV          : Radians = Degrees(69.45).as_radians();


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
	let stars_limit_mag    = DatabaseGenerator::limit_magnitude (&stars, MAGNITUDE_MIN, MAGNITUDE_MAX);
	
	
//##############################################################################################//
//							--- Projection ---
//##############################################################################################//
	// Read image as grayscale.
	let mut img = CVImage(imread( "starlight_6mm_46.704fov.png", ImreadModes::IMREAD_COLOR as i32 )
		.expect("Could not find img.png in root."));
		
	
	println!("Performing Projection");
	let img_horizontal = ((img.width() as Decimal).powf(2.0) + (img.height() as Decimal).powf(2.0)).sqrt();
	let intrinsic_projection = IntrinsicParameters::from_fov(FOV, img_horizontal);



	
	// Not really nessisary.
	// If the camera is not on the front of the spacecraft, specify what orientation the camera is.
	// In this case, the camera is pointing +z (relative to the spacecraft) 
	// and the top pixel is in the direction of +y (relative to the space craft).
	let reference_forward = Equatorial{ra: Hours(13.6).to_radians(), dec: Degrees(-62.5).to_radians()};
	let reference_up      = Vector3{x: 0.0, y: -1.0, z:-0.9}.to_equatorial();//Equatorial{ra: Hours(15.3).to_radians(), dec: Degrees(-60.9).to_radians()};
	let extrinsic_projection = ExtrinsicParameters::look_at(reference_forward, reference_up)
		.expect("Ensure entrinsic projection up and forward are not the same value.");
	
	
	for i in 0..stars_limit_mag.len()
	{
		let center = Point::new(img.width() as i32 / 2, img.height() as i32 / 2); 
		let star = stars_limit_mag[i].pos.to_vector3();
		let star_world_space = SpaceWorld(star);
		let star_camera_space = extrinsic_projection.to_image(star_world_space);
		let star_pixel_space  = intrinsic_projection.to_image(star_camera_space);
		let star_point_space  = Point::new(star_pixel_space.0.x as i32, star_pixel_space.0.y as i32) + center;

		if star.angle_distance(reference_forward.to_vector3()) < FOV/ 2.0
		{
			let circle_thickness = 1;
			let circle_radius    = (MAGNITUDE_MAX - stars_limit_mag[i].mag) as i32 + 3;
			
			let mut circle_color = Scalar::new(0.0, 100.0, 0.0, 0.0); // Red color (BGR format)
			circle(&mut img.0,star_point_space,circle_radius,circle_color,circle_thickness,1,0,
			).unwrap();
		}
	}
		
	let _ = imshow("Image", &img.0);
	let _ = wait_key(0).unwrap();
	

}
