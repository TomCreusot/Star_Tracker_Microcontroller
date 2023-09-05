//! This runs through all the images in samples and tries to identify the stars using the star tracker code.
//! If you want to understand how the code works, maybe start here?
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

use star_tracker_lib::image_processing::*;

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
	const ANGLE_TOLERANCE : Radians = Degrees(0.2).as_radians(); 	// Maximum inaccuracy.
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

	// If stars are this close, one is excluded.
	const DOUBLE_STAR_TOLERANCE : Radians = TrackingConstsTest::ANGLE_TOLERANCE;


	// Loose conditions
	const TIME_GOOD    : u128 = 100000; // ms until autofail.


	println!("Performing Database Construction");
	println!("\tReading database.");
	print!("\t");
	let mut stars : Vec<Star> = Vec::new();
	let mut rdr = Io::get_csv (
		NixConstsStruct::HYG_DATABASE_PATH,
		NixConstsStruct::HYG_DATABASE_FILE,
		NixConstsStruct::HYG_DATABASE_URL );

	let iter = rdr.deserialize();
	for record in iter
	{
		let star : Star = record.expect("Could not decode.");
		stars.push(star);
	}
	println!("\t Found: {} stars", stars.len());

	stars.sort(); // The magnitude must be sorted to get best results for `limit_regions`
	println!("\tLimiting Magnitude.");
	let stars_limit_mag    = DatabaseGenerator::limit_magnitude (&stars, MAGNITUDE_MIN, MAGNITUDE_MAX);
	println!("\tLimiting Double Stars.");
	let stars_limit_double = DatabaseGenerator::limit_double_stars(&stars_limit_mag, DOUBLE_STAR_TOLERANCE);
	println!("\tLimiting Similar.");
	// let stars_limit_sim    = DatabaseGenerator::limit_similar(&stars_limit_double, Degrees(0.0001).to_radians());

	println!("\tCreating Database.");

	let samples = star_tracker_nix::io::Sample::load_samples();

	for sample in samples
	{
		// The Diagonal field of view.
		let fov_file = sample.get_fov();
		let fov : Radians;
		if let Some(fov_) = fov_file { fov = fov_; }
		else                         { continue;   }

		println!("{}, fov: {}", sample.dir, fov.to_degrees());


		if sample.dir != "samples/16mm_2" {continue;}//fov < Degrees(20.0).to_radians()  && fov < Degrees(40.0).to_radians() { continue; }


		let region_size = fov / 2.0;
		let region_num = 8;

		for img in sample.file_img
		{
			println!("\t{}", img);

			// let mut img = CVImage(imread(&img, opencv::core::CV_8UC3 as i32 )
				// .expect("Could not find img.png in root."));
			let mut img = CVImage::read(&img);
			let dark_frame = CVImage::read(&sample.file_dark);//(imread(&sample.file_dark, opencv::core::CV_8UC3 as i32).expect(""));



			println!("\tLimiting Regions.");
			let stars_limit_reg    = DatabaseGenerator::limit_regions(&stars_limit_double,region_size, region_num);


			println!("NUM: {} {} {} {}", stars.len(), stars_limit_mag.len(), stars_limit_double.len(), stars_limit_reg.len());
			let gen : DatabaseGenerator = DatabaseGenerator::gen_database(&stars_limit_reg, fov, fov, TrackingConstsTest::ANGLE_TOLERANCE);
			// let gen : DatabaseGenerator = DatabaseGenerator::gen_database_regional(&stars_limit_reg, fov, TrackingConstsTest::ANGLE_TOLERANCE, fov/2.0);
			let database = gen.get_database();
			// let database = gen.get_database_regional();
			// let mut database_iterator = ChunkIteratorNone::new(&database);
			// let mut database_iterator = ChunkIteratorRegional::new(&database);
			// let mut database_iterator = ChunkIteratorEquatorial::new(&database, Degrees(45.0).as_radians(), 0.3);
			// let mut database_iterator = ChunkIteratorDeclination::new(&database, Degrees(21.0).as_radians(), 0.2, ChunkIteratorDeclination::randomise_none);
			let mut database_iterator = ChunkIteratorDeclination::new(&database, fov, 1.5, ChunkIteratorDeclination::randomise_parity);

			// let mut database_iterator = ChunkAreaSearch::from_point(&database, center, Degrees(35.954).as_radians());


			// Identify num bins.
			// let mut prev = 0;
			// println!("bin tolerance {}", database_iterator.get_database().get_k_lookup().gradient);
			// for i in 0..database_iterator.get_database().get_k_vector_size()
			// {
			// 	let diff = database_iterator.get_database().get_k_vector(i) - database_iterator.get_database().get_k_vector(prev);
			// 	println!("{}\t{:.3}\t{}", i, Radians(i as Decimal * database_iterator.get_database().get_k_lookup().gradient + database_iterator.get_database().get_k_lookup().intercept).to_degrees(), diff);
			// 	prev = i;
			// }


			println!("{} stars, {} pairs.", stars_limit_reg.len(), database.pairs.size());








//###############################################################################################//
//							--- Image Processing / Blob Detection ---
//###############################################################################################//

			println!("Performing Image Processing and Blob Detection");



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



			let thresh = ThresholdGrid::<100, 100>::new(&img, 50);
//			let thresh = ThresholdPercent::new(&img, 0.99999);
			let mut img_thresh = CVImage::duplicate(&img);
			thresh.apply(&mut img_thresh);

//			opencv::imgproc::adaptive_threshold(&img.0, &mut img_thresh.0, 0.0,
//				opencv::imgproc::ADAPTIVE_THRESH_GAUSSIAN_C, opencv::imgproc::THRESH_BINARY_INV, 50, thresh_val);
//
			for x in 0..img_thresh.width()
			{
				for y in 0..img_thresh.height()
				{
					if img_thresh.get(Pixel{x: x, y: y}) < 1 as star_tracker_lib::util::aliases::Byte
					{
						img.set(Pixel{x: x, y: y}, 0);
					}
					else
					{
						let val = 255;//img.get(Pixel{x: x, y: y}).saturating_mul(5);
						img_thresh.set(Pixel{x: x, y: y}, val);
					}
				}
			}

			imshow("w/", &img.0);
			imshow("thresh", &img_thresh.0);
			wait_key(0);

			// Find the blobs in the image.
			let mut stack : Vec<Pixel> = Vec::new(); // Infinite sized blobs.
			let mut blobs : Vec<Blob> = Vec::new();
			let blob_min_size = 4;
			Blob::find_blobs(blob_min_size, &thresh, &mut img_consumable, &mut stack, &mut blobs);
			// Blob::find_blobs::<100>(img_thresh, &mut img_consumable, &mut blobs);

			// Convert the blobs into positions.
			let mut stars_2d : Vec<Vector2> = Vec::new();
			blobs.sort_order(Blob::sort_descending_intensity);
			Blob::to_vector2(&blobs, &mut stars_2d);

			for i in 0..stars_2d.len()
			{
				println!(". {:.3}\t{:.3}", stars_2d[i].x + 1.0, stars_2d[i].y + 1.0);
			}



			println!("\tFound: {} stars", stars_2d.len());


		//##############################################################################################//
		//							--- Projection ---
		//##############################################################################################//

			println!("Performing Projection");
			// Construct the parameters for an inverse intrinsic projection.
			let img_horizontal = ((img.width() as Decimal).powf(2.0) + (img.height() as Decimal).powf(2.0)).sqrt();
			let intrinsic_projection = IntrinsicParameters::from_fov(fov, img_horizontal);




			// Not really necessary.
			// If the camera is not on the front of the spacecraft, specify what orientation the camera is.
			// In this case, the camera is pointing +z (relative to the spacecraft)
			// and the top pixel is in the direction of +y (relative to the space craft).
			let reference_forward = Vector3{x: 0.0, y: 0.0, z: 1.0}.to_equatorial();
			let reference_up      = Vector3{x: 0.0, y: 1.0, z: 0.0}.to_equatorial();
			let extrinsic_projection = ExtrinsicParameters::look_at(reference_forward, reference_up)
				.expect("Ensure extrinsic projection up and forward are not the same value.");

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
				let _ = found_stars.push_back( Match{ input:  input, output: output, weight: 1.0 } );

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
			// 	// if star.angle_distance(reference_forward.to_vector3()) < fov/ 2.0
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
