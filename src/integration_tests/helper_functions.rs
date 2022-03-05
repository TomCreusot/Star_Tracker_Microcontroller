use rand::prelude::*;
use image::RgbImage;

use crate::tracking_mode::database::StarDatabaseElement;
use crate::tracking_mode::database::PyramidDatabase;
use crate::tracking_mode::database::Database;
use crate::tracking_mode::database::KVector;

use crate::tracking_mode::StarPair;


use crate::image_processing::BasicImage;
use crate::image_processing::Image;
use crate::image_processing::Blob;

use crate::nix::NixImage;
use crate::nix::Star;

use crate::util::aliases::Decimal;
use crate::util::aliases::Byte;

// use crate::util::units::Quaternion;
use crate::util::units::Equatorial;
use crate::util::units::Radians;
use crate::util::units::Degrees;
use crate::util::units::Pixel;
use crate::util::list::List;



//###############################################################################################//
//###############################################################################################//
//###############################################################################################//
//
//										Generate Image
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
// 	let pt = point.to_cartesian3();
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
	let mut img = NixImage{img_rgb: RgbImage::new(0,0)};
	img.image_to_dynamic(image);
	
	
	let mut histogram : [u32; HISTOGRAM_SIZE] = [0; HISTOGRAM_SIZE];
	img.histogram(&mut histogram);	
	let threshold = img.novel_threshold(thresh_percent, &histogram);

	
	let mut blobs : Vec<Blob> = Vec::new();
	Blob::find_blobs::<MAX_BLOB_SIZE>(threshold, &mut img, &mut blobs);
	
	
	return blobs;
}







//###############################################################################################//
//###############################################################################################//
//###############################################################################################//
//
//										Database Construction
//
//###############################################################################################//
//###############################################################################################//
//###############################################################################################//

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Used to generate a star catalogue (position of stars).
/// This is done by randomly generating a set of stars within the equatorial sphere.
///
/// The actual catalogue uses Equatorial instead of Star.
///
/// # Arguments
/// * `num_elements` - The size of the catalogue to generate.
///
/// # Returns
/// A catalogue of stars in the nix format.
/// Most of the variables are unused and the actual database is an array of Equatorial coordinates.
pub fn generate_catalogue ( num_elements: usize ) -> Vec<Star>
{
	let mut rng = rand::thread_rng();

	// The database of stars.
	let mut cat : Vec<Star> = Vec::new();


	// Randomly generate 100 stars on the equatorial sphere.
	for _i in 0..num_elements
	{
		let element = Equatorial{
			ra:  Degrees(rng.gen::<Decimal>() * 360.0).to_radians(),
			dec: Degrees(rng.gen::<Decimal>() * 180.0 - 90.0).to_radians()};

		let star = Star{mag: 0.0, spec: "".to_string(), pos: element};
		cat.push_back(star).expect("Vecs should not be full");
	}	
	return cat;
}



// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Generates the star pairs.
/// This is a set of combinations of stars which are within the field of view.
/// The star pairs are relevent to the Pyramid tracking mode.
///
/// In this case 360 degrees will be used as the field of view is not being tested in this test.
///
/// # Arguments
/// * `cat` - The star catalogue of stars on the celestial sphere, this is usualy generated from a csv star database.
///
/// # Returns
/// A star database element which is used to generate a K_Vector.
/// The actual star pair database is using the database StarPair.
pub fn generate_star_pair ( cat: &Vec<Star> ) -> Vec<StarDatabaseElement>
{
	let field_of_view = Degrees(360.0).to_radians();
	let mut pairs = StarDatabaseElement::create_list(field_of_view, cat);
	pairs.sort_by(StarDatabaseElement::cmp);
	return pairs;
}



// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Generates the k-vector equation.
/// The k-vector equation outputs a value which represents an index of the k-vector bins.
///
/// # Arguments
/// * `num_bins` - The number of bins for the k-vector.
/// * `pairs`    - The star pairs created in `generate_star_pair`.
///
/// # Returns
/// The k-vector equation to be used to reference the bins.
pub fn create_k_vector ( num_bins : usize, pairs : &Vec<StarDatabaseElement> ) -> KVector
{
	let k_vector = KVector::new(num_bins, 
			pairs[0].dist.0 as Decimal, 
			pairs[pairs.len() - 1].dist.0 as Decimal);
	return k_vector;
			
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Creates the k-vector bins.
/// The k-vector equation points to a specific bin with the input of an angular distance.
/// The bin points to a star pair in the star pair database which has a similar angular distance.
///
/// # Arguments
/// * `k_vector`   - The k-vector generated in `create_k_vector`.
/// * `pairs` - The vector passed into `create_k_vector`.
///
/// # Returns
/// The k-vector bins.
pub fn create_bins ( k_vector: KVector, pairs: &Vec<StarDatabaseElement>  ) -> Vec<usize>
{
	return k_vector.generate_bins(pairs).expect("not enough bins available.");
}









//###############################################################################################//
//###############################################################################################//
//###############################################################################################//
//
//										Searching Database
//
//###############################################################################################//
//###############################################################################################//
//###############################################################################################//


/// Generates a random angular distance between min and max.
pub fn gen_angle_distance ( min: Radians, max: Radians ) -> Radians
{
	let mut rng = rand::thread_rng();
	return Radians(rng.gen::<Decimal>()) * (max - min) + min;	
}

/// Generates a tolerance value.
pub fn gen_tolerance ( max: Radians ) -> Radians
{
	let mut rng = rand::thread_rng();
	return Radians(rng.gen::<Decimal>()) * max;	
}




// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// This will search the database for all matches of an angular distance within a tolerance.
/// The method is really basic and slow.
///
/// The method is to loop through each element of the catalogue to find values of the correct
/// distance and to add it to a list.
/// It is the most reliable method as it is simple.
pub fn find_basic_angles ( angle: Radians, tolerance: Radians, database: &PyramidDatabase )
																			-> Vec<StarPair<usize>>
{
	let mut matches : Vec<StarPair<usize>> = Vec::new();
	
	for ii in 0..database.catalogue.len()
	{
		for jj in (ii+1)..database.catalogue.len()
		{
			if (database.angle_distance(StarPair(ii, jj)).expect("?") - angle).abs() < tolerance.0
			{
				matches.push_back(StarPair(ii, jj)).expect("Vectors should have capacity?");
			}
		}
	}
	
	return matches;
}



pub fn find_k_vector_angles ( angle: Radians, tolerance: Radians, database: &PyramidDatabase )
																			-> Vec<StarPair<usize>>
{
	let mut matches : Vec<StarPair<usize>> = Vec::new();
	database.find_close_ref(angle, tolerance, &mut matches);
	return matches;
}




