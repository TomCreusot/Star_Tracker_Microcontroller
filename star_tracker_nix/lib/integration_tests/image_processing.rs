//! # Image Processing Test
//! This is an integration test of the image_processing module.
//! This is a step by step guide to generate an image of stars and find the centroid of the generated star.
//!
//! # Summary
//! The image_processing module is made up of Image and Blob.
//! ## Image
//! Image can be image_processing::BasicImage or nix::NixImage.
//! * `BasicImage` is a constant sized image which just stores basic grayscale information.
//! * `NixImage` can be read from an image file, has useful tools and color.
//!
//! ## Blob
//! Blob performs feature detection on the image to extract the stars.
//! This is done by the grassfire method and a basic percentage based threshold done in Image.

use rand::prelude::*;

use star_tracker_lib::image_processing::BasicImage;
use star_tracker_lib::image_processing::Image;
use star_tracker_lib::image_processing::Blob;

use star_tracker_lib::util::aliases::Byte;
use star_tracker_lib::util::aliases::UInt;
use star_tracker_lib::util::aliases::Decimal;
use star_tracker_lib::util::units::Pixel;

use crate::image_processing::NixImage;
use crate::image_processing::CVImage;


pub fn run ( )
{
	let mut image = gen_image();
	insert_noise(&mut image, 10, 140);

	// Generating a set of positions for the stars so the algorithm can determine the # success.
	let mut stars : Vec<Pixel> = Vec::new();
	for _i in 0..100
	{
		let rand = random_point(Pixel{x: 0, y: 0}, Pixel{x: image.width(), y: image.height()});
		stars.push(rand);

		insert_blob ( &mut image, rand, 200, Pixel{x: 2, y: 2}, Pixel{x: 0, y: 0}, 	50, Pixel{x: 3, y: 3} );
	}

	insert_lens_flare(&mut image, Pixel{x: 0, y: 0}, 150, 2000.0);
	insert_lens_flare(&mut image, Pixel{x: 1000, y: 1000}, 200, 100.0);

	let blobs = get_blobs::<255, 200>(&image, 0.9999);
	let mut num_correct = 0; // Identified star and is real.
	let mut num_false = 0;   // Identified star which is incorrect.
	let mut rgb = CVImage::duplicate(&image);
	rgb.save("results/integration_tests/image_processing/hidden.png");
	for ii in 0..blobs.len()
	{
		let point = Pixel{x: blobs[ii].centroid.x.round() as usize,
				y: blobs[ii].centroid.y.round() as usize};
		let size = 5;
		let color = crate::image_processing::Color::Yellow;
		rgb.draw_point(point, size, color);

		let mut found = false;
		for jj in 0..stars.len()
		{
			let correct_x =  (point.x as f32 - stars[jj].x as f32).abs() < 2.1;
			let correct_y =  (point.y as f32 - stars[jj].y as f32).abs() < 2.1;
			if correct_x && correct_y
			{
				num_correct += 1;
				found = true;
			}
		}
		if !found
		{
			num_false += 1;
		}

	}

	println!("This is a stress test, it is unlikely that this will happen in real life.");
	println!("The stars are difficult to see on the image.");
	println!("To find the image, go to:");
	println!(" - results/integration_tests/image_processing/hidden.png for an unaltered image.");
	println!(" - results/integration_tests/image_processing/found.png to show all found stars.");
	println!(" - results/integration_tests/image_processing/shown.png to show actual stars.");
	println!("  found {} blobs", blobs.len());
	println!("  {} correctly identified (within 2 pixels).", num_correct);
	println!("  {} falsely identified.", num_false);
	println!("  {} missed.", stars.len() - num_correct);

	rgb.save("results/integration_tests/image_processing/found.png");

	rgb = CVImage::duplicate(&image);
	for i in 0..stars.len()
	{
		rgb.draw_point(stars[i], 5, crate::image_processing::Color::Orange);
	}
	rgb.save("results/integration_tests/image_processing/shown.png");
}



//###############################################################################################//
//###############################################################################################//
//###############################################################################################//
//
//									Helper functions
//
//###############################################################################################//
//###############################################################################################//
//###############################################################################################//


/// Generates a 1000x1000 image and sets all pixels to black.
/// # Returns
/// A 1000x1000 basic image.
pub fn gen_image ( ) -> BasicImage<1000,1000>
{
	return BasicImage::new();
}


/// Returns a randomly generated point.
/// # Arguments
/// * `min` - The INCLUSIVE minimum pixel in x and y.
/// * `max` - The EXCLUSIVE minimum pixel in x and y.
/// # Returns
/// A number equal to or above min but smaller than max.
pub fn random_point ( min: Pixel, max: Pixel ) -> Pixel
{
	let mut rng = rand::thread_rng();
	return Pixel{x: rng.gen_range(min.x..max.x), y: rng.gen_range(min.y..max.y)};
}


/// Adds noise to all pixels.
/// # Arguments
/// * `image` - The image to manipulate.
/// * `floor` - The lowest noise value.
/// * `ceil`  - The highest noise value.
pub fn insert_noise ( image: &mut dyn Image, floor: Byte, ceil: Byte )
{
	let mut rng = rand::thread_rng();
	for xx in 0..image.width()
	{
		for yy in 0..image.height()
		{
			let mut val : Byte = image.get(Pixel{x: xx, y: yy});
			val = val.saturating_add(rng.gen_range(floor..ceil));
			image.set(Pixel{x: xx, y: yy}, val);
		}
	}
}


/// Adds a lens flare over the whole image (any pixel brighter will remain at its brightness).
/// # Arguments
/// * `image`     - The image to adda a lens flare to.
/// * `flare`     - The origin of the flare (more ideal on outside).
/// * `intensity` - The intensity of the start pixel.
/// * `spread`    - How slowely it decays.
pub fn insert_lens_flare ( image: &mut dyn Image, center: Pixel, intensity: Byte, spread: Decimal )
{
	for xx in 0..image.width()
	{
		for yy in 0..image.height()
		{
			let distance_x = (center.x as Decimal - xx as Decimal).abs();
			let distance_y = (center.y as Decimal - yy as Decimal).abs();
			let distance = (distance_x * distance_x + distance_y * distance_y).sqrt();
			let mut val = (intensity as Decimal / (distance / spread + 1.0)) as Byte;
			val = std::cmp::max(val, image.get(Pixel{x: xx, y: yy}));
			image.set(Pixel{x: xx, y: yy}, val);
		}
	}
}


/// Converts an equatorial coordinate to a cartesian point on an image.
/// # Arguments
/// * `point`    - The point on the equatorial plane.
/// * `center`   - The orientation of the spacecraft.
/// * `fov`      - The field of view of the camera (top left to bottom right).
/// * `dimension`- The size of the image.
/// # Returns
/// The location.
// pub fn equatorial_to_pixel ( point: Equatorial, orientation: Quaternion, fov: Radians, dimensions: Pixel ) -> Pixel
// {
// 	let pt = point.to_vector3();
// 	// let
// }

/// Puts a roughly circular blob in the image with the given intensity, size, center + randomness.
/// # Arguments
/// * `image`          - The image to add a star to.
/// * `center`         - The position of the center of the star.
/// * `intensity`      - The brightness of the center of the image.
/// * `spread`         - The spread of the star (1 is a pixel either side range).
/// * `rand_center`    - The randomness of the center position (1 is a pixel verticaly and horizontaly).
/// * `rand_intensity` - The randomness of the brightness.
/// * `rand_spread`    - The randomness of the sprad of the star.
pub fn insert_blob ( image: &mut dyn Image,
				mut center: Pixel, intensity: Byte, spread: Pixel,
				rand_center: Pixel, rand_intensity: Byte, rand_spread: Pixel )
{
	let mut rng = rand::thread_rng();

	center.x += (rand_center.x as Decimal * rng.gen::<Decimal>()) as usize;
	center.y += (rand_center.y as Decimal * rng.gen::<Decimal>()) as usize;

	let mut start_x = center.x as i32 - spread.x as i32;
	let mut end_x   = center.x as i32 + spread.x as i32;
	let mut start_y = center.y as i32 - spread.y as i32;
	let mut end_y   = center.y as i32 + spread.y as i32;

	start_x += rng.gen_range(-(rand_spread.x as i32)..rand_spread.x as i32);
	end_x   += rng.gen_range(-(rand_spread.x as i32)..rand_spread.x as i32);
	start_y += rng.gen_range(-(rand_spread.y as i32)..rand_spread.y as i32);
	end_x   += rng.gen_range(-(rand_spread.y as i32)..rand_spread.y as i32);

	start_x = std::cmp::max(start_x, 0);
	end_x   = std::cmp::min(end_x, image.width() as i32 - 1);
	start_y = std::cmp::max(start_y, 0);
	end_y   = std::cmp::min(end_y, image.height() as i32 - 1);

	for xx in start_x..=end_x
	{
		for yy in start_y..=end_y
		{
			let distance_x = (center.x as Decimal - xx as Decimal).abs();
			let distance_y = (center.y as Decimal - yy as Decimal).abs();
			let distance = (distance_x * distance_x + distance_y * distance_y).sqrt();
			let spread_mag = ((spread.x * spread.x + spread.y * spread.y) as Decimal).sqrt();
			let drop_off = 1.0 / (distance / spread_mag + 1.0);

			let mut brightness = intensity as Decimal * drop_off;
			brightness += rng.gen::<Decimal>() * rand_intensity as Decimal - rand_intensity as Decimal;
			brightness = brightness.clamp(0.0, Byte::MAX as Decimal);
			image.set(Pixel{x: xx as usize, y: yy as usize}, brightness as Byte);
		}
	}
}


/// Thresholds, performs blob detection and finds the centroid.
/// Copies the image so the image is not consumed.
/// # Generic Arguments
/// * `HISTOGRAM_SIZE` - The number of columns in the image histogram.
/// * `MAX_BLOB_SIZE`  - The maximum number of pixels a blob can be in.
/// # Arguments
/// * `image`          - The image to perform blob detection on.
/// * `thresh_percent` - The value that pixels must be above to be a star.
pub fn get_blobs <const HISTOGRAM_SIZE: usize, const MAX_BLOB_SIZE : usize>
									( image: &dyn Image, thresh_percent: Decimal ) -> Vec<Blob>
{
	let mut img = CVImage::new(Pixel{x: image.width(), y: image.height()});
	image.copy_to(&mut img).expect("?");

	let mut histogram : [UInt; HISTOGRAM_SIZE] = [0; HISTOGRAM_SIZE];
	let _ = img.histogram(&mut histogram);
	let threshold = img.percent_threshold(thresh_percent, &histogram);


	let mut blobs : Vec<Blob> = Vec::new();
	Blob::find_blobs::<MAX_BLOB_SIZE>(threshold, &mut img, &mut blobs);


	return blobs;
}
