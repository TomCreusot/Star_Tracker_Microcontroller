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

use crate::integration_tests::helper_functions::*;

// use crate::image_processing::BasicImage;
use crate::image_processing::Image;
// use crate::image_processing::Blob;
// 
use crate::util::units::Pixel;
// 
use crate::nix::NixImage;

// use crate::util::aliases::Decimal;
// use crate::util::aliases::Byte;

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
		
		// insert_blob(image, center intensity, spread, rand_center, rand_intensity, rand_spread )
		insert_blob ( &mut image,	rand,				200,		Pixel{x: 2, y: 2},
									Pixel{x: 0, y: 0}, 	50, 		Pixel{x: 3, y: 3} );
	}
					
	// insert_lens_flare ( image, center, intensity, spread )
	insert_lens_flare(&mut image, Pixel{x: 0, y: 0}, 150, 2000.0);
	insert_lens_flare(&mut image, Pixel{x: 1000, y: 1000}, 200, 100.0);
	
	
	// get_blobs <HISTOGRAM_SIZE, MAX_BLOB_SIZE> ( image, thresh_percent ) -> Vec<Blob>
	let blobs = get_blobs::<255, 200>(&image, 0.9999);
	let mut num_correct = 0; // Identified star and is real.
	let mut num_false = 0;   // Identified star which is incorrect.
	let mut rgb = NixImage::new(&image);
	rgb.img_rgb.save("results/integration_tests/image_processing/hidden.png").expect("");
	for ii in 0..blobs.len()
	{
		let point = Pixel{x: blobs[ii].centroid.x.round() as usize, 
				y: blobs[ii].centroid.y.round() as usize};
		let size = 5;
		let color = [255, 255, 0];
		rgb.draw_points(point, size, color);
		
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
	
	rgb.img_rgb.save("results/integration_tests/image_processing/found.png").expect("");
	
	rgb = NixImage::new(&image);
	for i in 0..stars.len()
	{
		rgb.draw_points(stars[i], 5, [100, 255, 0])
	}
	rgb.img_rgb.save("results/integration_tests/image_processing/shown.png").expect("");
}

