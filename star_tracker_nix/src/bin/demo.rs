#![allow(unused_imports)]
//! This runs through all the images in samples and tries to identify the stars using the star tracker code.
//! If you want to understand how the code works, maybe start here?
//!

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
use star_tracker_lib::util::aliases::Byte;
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
use star_tracker_lib::tracking_mode::ConstellationResult;
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

use star_tracker_nix::io::Star;
use star_tracker_nix::io::Io;
use star_tracker_nix::tracking_mode::DatabaseGenerator;
use star_tracker_nix::tracking_mode::AbandonSearchTimeoutFailure;
use star_tracker_nix::image_processing::CVImage;
use star_tracker_nix::image_processing::Color;
use star_tracker_nix::util::units::Formatted;

use star_tracker_lib::image_processing::ImageWord;
use star_tracker_lib::util::word::WordList;
use star_tracker_lib::util::word::WordSize;
use star_tracker_lib::util::err::Unsafe;




use std::env;

pub fn main ( )
{
	env::set_var("RUST_BACKTRACE", "1");
	println!(r#"
	
	
				===== Demo =====
This runs through all the images in samples and tries to identify the stars using the star tracker code.
If you want to understand how the code works, maybe start here?

This accepts command line arguments for inclusive selection of images:
reset; cargo run --bin demo 16mm_checker_2

	  
	"#);

	let exclusive_folders: Vec<String> = env::args().collect();
	let mut sky_map = CVImage::new(Pixel{x: 1280, y: 720});

	let angle_tolerance  = Degrees(0.05).as_radians();
	let magnitude_min    = -20.00;
	let magnitude_max    =   5.9;
	let double_star_tolerance = angle_tolerance;


	// Loose conditions
	let time_good: u128 = 5000; // ms until auto fail.
	let fails_good: usize = 500;  // How many triangles can be failed matches until auto fail.


	println!("Performing Database Construction");
	println!("\tReading database.");
	print!("\t");
	let mut stars : Vec<Star> = Io::get_csv_database();



	println!("\t Found: {} stars", stars.len());
	stars.sort(); // The magnitude must be sorted to get best results for `limit_regions`
	println!("\tLimiting Magnitude.");
	let stars_limit_mag    = DatabaseGenerator::limit_magnitude (&stars, magnitude_min, magnitude_max);
	println!("\tLimiting Double Stars.");
	let stars_limit_double = DatabaseGenerator::limit_double_stars(&stars_limit_mag, double_star_tolerance);
	println!("\tLimiting Similar.");
	// let stars_limit_sim    = DatabaseGenerator::limit_similar(&stars_limit_double, Degrees(0.0001).to_radians());
	
	// draw(magnitude_min, magnitude_max, stars_limit_mag, &mut sky_map);
	println!("\tCreating Database.");
	
	let mut fails_bands : Vec<Vec<String>> = Vec::new();
	for i in 0..1000 { fails_bands.push(Vec::new()); fails_bands[i] = Vec::new();}

	let mut num_fails = 0;
	let mut num_success = 0;
	let mut time_success = 0;
	let samples = star_tracker_nix::io::Sample::load_samples();
	
	for sample in samples
	{
		for image_index in 0..sample.file_img.len()
		{



			// Allows you to choose the folder images.
			let mut is_exclusive = false;
			for i in 1..exclusive_folders.len()
			{
				is_exclusive |= sample.file_img[image_index].contains(&exclusive_folders[i]);
			}
			if !is_exclusive { continue; }

			if sample.file_img.contains(&"sky-wipe".to_string()) { continue; }


			
			// The Diagonal field of view.
			let fov_file = sample.get_fov();
			let fov : Radians;
			if let Some(fov_) = fov_file { fov = fov_; }
			else                         { continue;   }
			println!("\n\n\n\n{}, fov: {}", sample.file_img[image_index], fov.to_degrees());
			
			let region_size = Degrees(10.0).to_radians();//fov / 2.0;
			let region_num = 8;
			
			let mut img = CVImage::read(&sample.file_img[image_index]);
			let dark_frame = CVImage::read(&sample.file_dark);
			
			let magnitude = DatabaseGenerator::recommended_magnitude(fov);
			
			let stars_limit_mag_2  = DatabaseGenerator::limit_magnitude (&stars_limit_double, magnitude_min, magnitude);
			let stars_limit_reg    = DatabaseGenerator::limit_regions(&stars_limit_mag_2, region_size, region_num);


			let gen : DatabaseGenerator = DatabaseGenerator::gen_database(&stars_limit_reg, fov, fov / 1.1, angle_tolerance);
			// let gen : DatabaseGenerator = DatabaseGenerator::gen_database_regional(&stars_limit_reg, fov, fov, TrackingConstsTest::ANGLE_TOLERANCE);
			let database = gen.get_database();
			// let database = gen.get_database_regional();
			// let mut database_iterator = ChunkIteratorNone::new(&database);
			// let mut database_iterator = ChunkIteratorRegional::new(&database);
			// let mut database_iterator = ChunkIteratorEquatorial::new(&database, Degrees(45.0).as_radians(), 0.3);
			// let mut database_iterator = ChunkIteratorDeclination::new(&database, Degrees(21.0).as_radians(), 0.2, ChunkIteratorDeclination::randomise_none);
			// let mut database_iterator = ChunkIteratorDeclination::new(&database, Degrees(10.0).as_radians(), 1.0, ChunkIteratorDeclination::randomise_parity);
			let mut database_iterator = ChunkIteratorDeclination::new(&database, Degrees(20.0).as_radians(), 1.2, ChunkIteratorDeclination::randomise_parity);

			// let mut database_iterator = ChunkAreaSearch::from_point(&database, center, Degrees(35.954).as_radians());
			// let mut database_iterator = 
			// ChunkAreaSearch::from_range(&database, Degrees, Degrees(35.954).as_radians());


			// let mut database_iterator = ChunkAreaSearch::from_range(&database, Degrees(0.0).to_radians()..Degrees(360.0).to_radians(), Degrees(-7.3).to_radians()-fov/1.2..Degrees(-7.3).to_radians()+fov/1.2);



//################################################################################################//
//							--- Image Processing / Blob Detection ---
//################################################################################################//

		imshow("BEFORE", &img.0);

		println!("Image Processing");
		// Removing Dark Frame from Image.
		for x in 0..img.width() {
			for y in 0..img.height() {
				let px = Pixel{x: x, y: y};
				if 10 < dark_frame.get(px) { img.set(px, 0); } } 
		}
		
		let mut img_actual = CVImage::new(Pixel{x: 808, y: 608});
		for x in 0..img_actual.width() {
			for y in 0..img_actual.height() {
				let px = Pixel{x: x, y: y};
				star_tracker_nix::image_processing::NixImage::set(&mut img_actual, px, Color::Black);
			}
		}

		let corrections = sample.get_corr();
		if let Some(corr) = corrections
		{
			for cor in corr
			{
				let mut color = Scalar::new(255.0, 255.0, 255.0, 0.0); // Red color (BGR format)
				let thickness = 1;
				let radius    = 10;
				let px_pt = Point::new(cor.image_px.x as i32, cor.image_px.y as i32);
				circle(&mut img_actual.0, px_pt, radius, color, thickness, 1, 0).unwrap();
			}
		}


		// The image to be examined will be consumed.
		// To have a visualization, you will need a copy.
		// I am using a word image as it stores the image better.
		// It is slightly slower though.
		let mut img_consumable = star_tracker_lib::create_image_word_nix!(Pixel{x: img.width(), y: img.height()}, 32, 8);
		let _ = img_consumable.copy_from(&img);


		// Create a threshold using a semi adaptive threshold.
		let timer = std::time::Instant::now();
		let thresh = ThresholdGrid::<50, 50>::new(&img, 20, 3);
		// let thresh = ThresholdPercent::new(&img, 0.9999);
		let time_thresh = timer.elapsed().as_millis();
		
		// Find the blobs in the image.
		let timer = std::time::Instant::now();
		let mut stack : Vec<Pixel> = Vec::new(); // Infinite sized blobs.
		let mut blobs : Vec<Blob>  = Vec::new();
		let blob_min_size = 2;
		Blob::find_blobs(blob_min_size, &thresh, &mut img_consumable, &mut stack, &mut blobs);

		// Convert the blobs into positions.
		let mut stars_2d : Vec<Vector2> = Vec::new();
		blobs.sort_order(Blob::sort_descending_intensity);
		Blob::to_vector2(&blobs, &mut stars_2d);
		let time_blob = timer.elapsed().as_millis();
		
		// Visualizes the stars.
		let mut img_thresh = CVImage::duplicate(&img);
		// thresh.apply_bin(&mut img_thresh);

		for i in 0..stars_2d.size()
		{
			let color = Scalar::new(0.0, 100.0, 255.0, 100.0); // Red color (BGR format)
			let thickness = 3;
			let radius    = 20;
			let px_pt = Point::new(stars_2d.get(i).x as i32, stars_2d.get(i).y as i32);
			circle(&mut img_thresh.0, px_pt, radius, color, thickness, 1, 0).unwrap();
		}

		// let _ = imshow("Thresholded", &img_thresh.0);
		println!("Found: {} stars in image.\n", stars_2d.len());


//################################################################################################//
//							--- Projection ---
//################################################################################################//

			let timer = std::time::Instant::now();

			// Construct the parameters for an inverse intrinsic projection.
			let sensor_horizontal = ((img.width() as Decimal).powf(2.0) + (img.height() as Decimal).powf(2.0)).sqrt();
			let img_center = Vector2{x: img.width() as Decimal / 2.0, y: img.height() as Decimal / 2.0};
			let intrinsic_projection = IntrinsicParameters::from_fov(fov, sensor_horizontal, img_center);




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
				let camera_space = intrinsic_projection.from_image(point);
				let world_space  = extrinsic_projection.from_image(camera_space);

				stars_3d.push(world_space.0.to_equatorial());
			}

			let time_project = timer.elapsed().as_millis();




//################################################################################################//
//							--- Tracking Mode ---
//################################################################################################//

			let timer = std::time::Instant::now();

			// Finds a `constellation` which is 4 stars forming a pyramid shape.
			// Refer to star_tracker_lib::tracking_mode::{mod, Constellation::find}
			let mut found_all : Vec<Match<usize>> = Vec::new();
			let success = Constellation::find (
				&stars_3d, &mut database_iterator,
				&mut StarTriangleIterator::<10000>::new(),
				&mut Specularity::default(),
				&mut AbandonSearchTimeoutFailure::new(Duration::from_millis(time_good as u64), fails_good),
				angle_tolerance,
				4..=4,
				&mut found_all
			);
			let time_tracking = timer.elapsed().as_millis();
			let mut found_stars : Vec<Match<Vector3>> = Vec::new();
			match success
			{
				ConstellationResult::ErrorNoTriangleMatch      { fails } => 
				{
					println!("FAILED: Could not match any stars; {} failures.", fails);
				}	
				
				ConstellationResult::ErrorAborted              { fails } =>
				println!("FAILED: Aborted due to AbandonSearch parameter; {} failures.", fails),
				
				ConstellationResult::ErrorInsufficientPyramids { fails } =>
				println!("FAILED: Not enough matched stars; {} failures.", fails),
				
				ConstellationResult::Success                   { fails } =>
				{	
				fails_bands[fails].push(sample.file_img[image_index].clone());
					println!("SUCCESS; with {} fails.", fails);
			}}
			
			if found_all.size() < 3
			{
				num_fails += 1;
			}

			if 0 < found_all.size()
			{
			
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
				let _ = 
				put_text(&mut img.0, &f_text, px_pt, f_face, f_scale, color, thickness, 8, false);
				
				println!("{}: {}\t{}", i, name,
					database_iterator.get_database().get_catalogue(found_all[i].output));
			}
			
			let timer = std::time::Instant::now();
			let rotate_to_cam  : Quaternion = Quest::estimate(&found_stars, None);
			let rotate_to_world: Quaternion = rotate_to_cam.conjugate();
			let world_center = rotate_to_world.rotate_point(reference_forward.to_vector3());
			let time_attitude = timer.elapsed().as_millis();

			println!();
			println!("Found Center: {}", world_center.to_equatorial().formatted());
			println!("Found Center: {}", world_center.to_equatorial());
			
			
			if 3 < found_all.size()
			{
				num_success += 1;
				draw_pt(world_center.to_equatorial(), Color::Red, 2, &mut sky_map);
				star_tracker_nix::image_processing::NixImage::save(&sky_map, "sky.png");
			}
			
			if found_all.size() == 3
			{
				num_fails += 1;
				// draw_pt(world_center.to_equatorial(), Color::Blue, 4, &mut sky_map);
			}

			
			if sample.file_log != "" && sample.file_img.size() == 1
			{
				if let Some(center) = sample.get_center()
				{
					println!("True Center: {}", center.formatted());
					println!("Error: {:.2} degrees", 
					world_center.to_equatorial().angle_distance(center).to_degrees().0);
				} 
			}


			for i in 0..fails_bands.len()
			{
				if fails_bands[i].len() > 0
				{
					println!("\n\n\n\n{}", i);
					println!("{:?}", fails_bands[i]);
				}
			}


			println!("");
			println!("Time thresholding:   {} ms", time_thresh);
			println!("Time blob detection: {} ms", time_blob);
			println!("Time projection:     {} ms", time_project);
			println!("Time tracking:       {} ms", time_tracking);
			println!("Time quest:          {} ms", time_attitude);


			println!("num fails:    {}", num_fails);
			println!("num success:  {}", num_success);
			println!("time success: {}", time_success);

			// let _ = imshow("Actual", &img_actual.0);
			// let _ = imshow("Sky", &sky_map.0);
			// let _ = imshow("Image", &img.0);
			let _ = wait_key(10).unwrap();
		}
	}
}

star_tracker_nix::image_processing::NixImage::save(&sky_map, "sky.png");
let _ = wait_key(0);

}










fn draw ( mag_min: Decimal, mag_max: Decimal, stars: Vec<Star>, img: &mut CVImage )
{
	let px_p_equ = img.width()  as Decimal / Degrees(360.0).to_radians().0;
	let px_p_dec = img.height() as Decimal / Degrees(180.0).to_radians().0;
	for i in 0..stars.len()
	{
		let pos = Pixel{
			x: (px_p_equ * offset_ra(stars[i].pos.ra, Degrees(90.0).to_radians()).0)  as usize, 
			y: (px_p_dec * (stars[i].pos.dec + Degrees(90.0).to_radians()).0) as usize};
			
		let intensity_ratio = 1.0 - (stars[i].mag - mag_min) / (mag_max - mag_min);
		let intensity = (intensity_ratio * 5.0).powf(2.0);
		// println!("{} \t {}", stars[i].mag, intensity_ratio)
		let size_star    = 1.0 + intensity;
		let size_overlay = ((1.0 + intensity) * 7.0).powf(2.0);
		// let color = Color::Custom((intensity * 255.0) as Byte, 0, (255.0 - intensity * 255.0) as Byte);
		let color = Color::White;
		// println!("{:.2}\t{:.2}\t{:.2}\t{:.2}\t{:?}", stars[i].mag, intensity_ratio, intensity, size, color.get_color());
		let pos = SpaceImage(Vector2{x: pos.x as Decimal, y: pos.y as Decimal});
		// let prev_val = img.get(pos);
		// let new_val = prev_val.saturating_add(intensity);
		
		// img.set(pos, new_val);
		// img.overlay_circle(size_overlay, Color::Custom(1, 1, 2), pos);
		star_tracker_nix::image_processing::NixImage::draw_star(img, size_overlay, Color::Custom(2, 2, 2), pos);
		star_tracker_nix::image_processing::NixImage::draw_star(img, size_star, color, pos);
	}
}


#[inline]
fn draw_pt ( location: Equatorial, color: Color, size: usize, img: &mut CVImage )
{
	let px_p_equ = img.width()  as Decimal / Degrees(360.0).to_radians().0;
	let px_p_dec = img.height() as Decimal / Degrees(180.0).to_radians().0;

	let pos = Pixel{
		x: (px_p_equ * offset_ra(location.ra, Degrees(90.0).to_radians()).0)  as usize,  	
		y: (px_p_dec * (location.dec + Degrees(90.0).to_radians()).0) as usize};
	for xx in pos.x.saturating_sub(size)..pos.x+size
	{
		for yy in pos.y.saturating_sub(size)..pos.y+size
		{
			let pos = Pixel{x: xx, y: yy};
			if img.valid_pixel(pos)
			{
				star_tracker_nix::image_processing::NixImage::set(img, pos, color);
			}
		}	
	}
}

#[inline]
fn offset_ra ( val: Radians, offset: Radians ) -> Radians
{
	let mut val = val + offset;
	if Degrees(360.0).to_radians() < val
	{
		val = val - Degrees(360.0).to_radians();
	}
	return val;
}