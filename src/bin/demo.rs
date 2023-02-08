// #![allow(unused_must_use)]
// extern crate star_tracker;
// extern crate image;
// extern crate rand;
// extern crate opencv;
//
// use rand::prelude::*;
// use std::ops::Range;
//
// use star_tracker::config::TrackingModeConstsStruct;
// use star_tracker::config::AttitudeDeterminationConstsStruct;
// use star_tracker::config::TrackingModeConsts;
// use star_tracker::config::NixConstsStruct;
// use star_tracker::config::NixConsts;
//
// use star_tracker::util::aliases::Decimal;
// use star_tracker::util::aliases::Byte;
// use star_tracker::util::aliases::UInt;
// use star_tracker::util::list::List;
//
// use star_tracker::util::units::Equatorial;
// use star_tracker::util::units::Vector3;
// use star_tracker::util::units::Vector2;
// use star_tracker::util::units::Degrees;
// use star_tracker::util::units::Radians;
// use star_tracker::util::units::Pixel;
//
// use star_tracker::nix::DatabaseGenerator;
// use star_tracker::nix::Distribute;
// use star_tracker::nix::NixImage;
// use star_tracker::nix::CVImage;
// use star_tracker::nix::Color;
// use star_tracker::nix::Star;
// use star_tracker::nix::Io;
//
// use star_tracker::image_processing::Blob;
// use star_tracker::image_processing::Image;
//
// use star_tracker::tracking_mode::Constellation;
// use star_tracker::tracking_mode::StarTriangleIterator;
// use star_tracker::tracking_mode::Match;
//
// use star_tracker::projection::IntrinsicParameters;
// use star_tracker::projection::ExtrinsicParameters;
// use star_tracker::projection::SpaceWorld;
// use star_tracker::projection::SpaceImage;
// use star_tracker::projection::SpaceCamera;
//
//
// use star_tracker::attitude_determination::Quest;
// use star_tracker::attitude_determination::AttitudeDetermination;
//
//
// // Defines how the tracking algorithm should perform.
// pub struct TrackingConstsTest ( );
// impl TrackingModeConsts for TrackingConstsTest
// {
// 	const PAIRS_MAX       : usize = 2000;							// Irrelevant, ensure big.
// 	const TRIANGLES_MAX   : usize = 2000;							// Irrelevant, ensure big.
// 	// const SPECULARITY_MIN : Decimal = 300.0;						// If the triangle is flipped.
// 	const SPECULARITY_MIN : Decimal = 0.0001;						// If the triangle is flipped.
// 	const ANGLE_TOLERANCE : Radians = Degrees(0.1).as_radians(); 	// Maximum inaccuracy.
// }
//
//
// // Size of a blob on an image which should be considered as a star.
// const BLOB_SIZE		: usize = 100;
//
//
//
// fn main ( )
// {
// 	let mut rng = rand::thread_rng();
//
// 	// To reduce size of database.
// 	const MAGNITUDE_MIN: Decimal = -20.0;
// 	const MAGNITUDE_MAX: Decimal = 5.8;
//
// 	const REGION_SIZE  : Radians = Degrees(10.0).as_radians(); // An area smaller than FOV.
// 	const REGION_NUM   : usize   = 7; // Should not be more than 1 redundant star in a region.
//
// 	// To create the database.
// 	const NUM_BINS     : usize   = 2000; // Refer to `src/tracking_mode/database/mod.rs`.
// 	const FOV          : Radians = Degrees(20.0).as_radians();
//
//
// 	// Disrupt input.
// 	const IMAGE_SIZE		: Pixel = Pixel{x: 1000, y: 1000};
//
// 	const VAR_MAG 	     	: Range<Byte> = 150..200; // The variation outside of the magnitude range.
// 	const VAR_POS		    : Radians = Degrees(0.000001).as_radians(); // Error.
// 	const FALSE_STARS 		: usize   = 1;   // Maximum number of fake, random stars.
// 	const CAP_STARS         : usize   = 15;  // Max stars in image.
//
// 	const BLOOM_NUM         : usize   = 1;   // Number of bright artifacts in image.
// 	const BLOOM_SIZE		: Decimal = 5000.0;// The width of the bloom.
// 	const BLOOM_BRIGHTNESS	: Byte	  = 1; // The brightness of the bloom.
// 	const NOISE_BRIGHTNESS	: Byte    = 1; // The maximum brightness of noise.
//
// //###############################################################################################//
// //
// //							---	Constructing Database ---
// // The database must first be constructed:
// // * The main star csv database will be read, this is used as the star distribution is accurate.
// //
// // * The stars will then cut stars outside the magnitude, this includes the sun and dark stars.
// //
// // * The stars will then have the region cut, this is because some regions of the sky will have
// // far more stars than others, buy removing the redundant stars, the database shrinks in size.
// //
// // * Using the shortcut: `DatabaseGenerator::gen_database`, the database can be generated easily.
// //###############################################################################################//
//
// 	println!("Reading CSV Database");
// 	let mut stars : Vec<Star> = Vec::new();
// 	let mut rdr = Io::get_csv (
// 		NixConstsStruct::HYG_DATABASE_PATH,
// 		NixConstsStruct::HYG_DATABASE_FILE,
// 		NixConstsStruct::HYG_DATABASE_URL );
//
// 	let iter = rdr.deserialize();
// 	for record in iter
// 	{
// 		let star : Star = record.expect("Could not decode.");
// 		stars.push(star);
// 	}
//
// 	println!("Reducing Database");
//
// 	stars.sort(); // The magnitude must be sorted to get best results for `limit_regions`
//
// 	let stars_limit_mag =DatabaseGenerator::limit_magnitude (&stars, MAGNITUDE_MIN, MAGNITUDE_MAX);
// 	let stars_limit_reg=DatabaseGenerator::limit_regions(&stars_limit_mag,REGION_SIZE, REGION_NUM);
//
// 	println!(" - {} stars total.", stars.len());
// 	println!(" - {} stars when magnitude reduced.", stars_limit_mag.len());
// 	println!(" - {} stars when region reduced.", stars_limit_reg.len());
// 	println!();
//
// 	// let coverage_average     = DatabaseGenerator::sky_coverage(&stars, REGION_SIZE, REGION_NUM);
// 	// println!(" - {:0.2}% average coverage.", coverage_average * 100.0);
// 	let coverage_average_mag = DatabaseGenerator::sky_coverage(&stars_limit_mag, REGION_SIZE, REGION_NUM);
// 	println!(" - {:0.2}% average coverage magnitude reduced", coverage_average_mag * 100.0);
// 	let coverage_average_reg = DatabaseGenerator::sky_coverage(&stars_limit_reg, REGION_SIZE, REGION_NUM);
// 	println!(" - {:0.2}% average coverage region reduced", coverage_average_reg * 100.0);
// 	println!();
// 	// let coverage_worst       = DatabaseGenerator::sky_coverage_worst_case(&stars, REGION_SIZE);
// 	// println!(" - {} worst coverage.", coverage_worst);
// 	let coverage_worst_mag   = DatabaseGenerator::sky_coverage_worst_case(&stars_limit_mag, REGION_SIZE);
// 	println!(" - {} worst coverage magnitude reduced", coverage_worst_mag);
// 	let coverage_worst_reg   = DatabaseGenerator::sky_coverage_worst_case(&stars_limit_reg, REGION_SIZE);
// 	println!(" - {} worst coverage region reduced", coverage_worst_reg);
// 	println!();
// 	// let coverage_best       = DatabaseGenerator::sky_coverage_best_case(&stars, REGION_SIZE);
// 	// println!(" - {} best coverage.", coverage_best);
// 	let coverage_best_mag   = DatabaseGenerator::sky_coverage_best_case(&stars_limit_mag, REGION_SIZE);
// 	println!(" - {} best coverage magnitude reduced", coverage_best_mag);
// 	let coverage_best_reg   = DatabaseGenerator::sky_coverage_best_case(&stars_limit_reg, REGION_SIZE);
// 	println!(" - {} best coverage region reduced", coverage_best_reg);
//
//
//
// 	// The official database is based off static arrays to save memory and remove the heap.
// 	// When simulating the database, these variables must exist while the database exists.
//
// 	// let gen : DatabaseGenerator = DatabaseGenerator::gen_database(&stars_limit_reg, FOV, NUM_BINS);
// 	// let database = gen.get_database();
// 	let gen : DatabaseGenerator = DatabaseGenerator::gen_database_regions(&stars_limit_reg, FOV, NUM_BINS);
// 	let mut database = gen.get_database_regional();
//
//
// 	println!();
// 	println!("Created database");
// 	println!(" - {} angles generated.", database.pairs.size());
//
//
//
// //###############################################################################################//
// //							---	Test Runner ---
// //###############################################################################################//
// 	// Runs all tests.
// 	let observation_points = Distribute::angle_to_points(REGION_SIZE);
// 	let observation = Equatorial::fibonacci_latice(observation_points);
// 	println!("Performing Lost In Space");
// 	println!(" - {} orientations", observation_points);
// 	println!();
// 	// println!("Test\t|\tStars\t|\tSeparation\t|\tTime\t\t|\tResult\t|\tLocation");
//
// 	let mut num_pyramid        = 0;
// 	let mut num_triangle       = 0;
// 	let mut num_error          = 0;
// 	let mut num_error_pyramid  = 0;
// 	let mut num_error_triangle = 0;
// 	let mut avg_time = 0;
//
// 	let intrinsic = IntrinsicParameters::from_fov(FOV, IMAGE_SIZE.y as Decimal);
//
// 	for test_num in 0..observation.len()
// 	{
// //###############################################################################################//
// //
// //								--- Creating Image ---
// //
// // Creates an image from a specified location in the sky.
// // Creates noise and blooming.
// // Modifies the stars positions simulating lens distortion.
// //###############################################################################################//
// 		// The direction the camera is looking.
// 		let center : Equatorial = observation[test_num];
//
// 		// The input sample image.
// 		let mut image : CVImage = CVImage::new(IMAGE_SIZE);
//
// 		// The direction relative to the forward direction which is considered up.
// 		let up = random_direction();
//
// 		// Creates a camera rotation matrix which looks at a target and has an associated up direction.
// 		let extrinsic : ExtrinsicParameters = ExtrinsicParameters::look_at(center, up);
//
// 		// Sets the pixels to random values, represents a noisy camera.
// 		add_noise(&mut image, 0..NOISE_BRIGHTNESS);
//
// 		// Location in stars array of visible stars.
// 		let mut visible_stars : Vec<usize> = Vec::new();
// 		for star in 0..stars_limit_mag.len()
// 		{
// 			let mut point = stars_limit_mag[star].pos;
// 			let valid_pos = point.angle_distance(center) < FOV / 2.0 * 1.414213562; // within fov.
// 			let valid_num = true || visible_stars.len() < CAP_STARS; 					// max stars in image.
// 			if valid_pos && valid_num
// 			{
// 				// Simulate distortion.
// 				point.ra  = point.ra + Radians(rng.gen_range(-VAR_POS.0..VAR_POS.0));
// 				point.dec = point.dec + Radians(rng.gen_range(-VAR_POS.0..VAR_POS.0));
//
// 				let size = MAGNITUDE_MAX - stars_limit_mag[star].mag;
// 				let brightness = rng.gen_range(VAR_MAG);
// 				let color = Color::Custom(brightness, brightness, brightness);
// 				// Draw image.
// 				if image.draw_star_projection(SpaceWorld(point.to_vector3()),size,color,intrinsic,extrinsic)
// 				{
// 					visible_stars.push_back(star);
// 				}
// 			}
// 		}
//
// 		// Adds Bloom (sun/atmosphere/moon/damaged sensor).
// 		for i in 0..BLOOM_NUM	{	add_bloom(&mut image, BLOOM_SIZE, BLOOM_BRIGHTNESS);	}
//
// 		// Adds False Stars.
// 		for i in 0..FALSE_STARS
// 		{
// 			let mag = rng.gen_range(VAR_MAG);
// 			let size = rng.gen_range(0.0..10.0);
// 			let pos = Vector2{x: rng.gen_range(0..IMAGE_SIZE.x) as Decimal, y: rng.gen_range(0..IMAGE_SIZE.y) as Decimal};
// 			image.draw_star(size, Color::Custom(mag, mag, mag), SpaceImage(pos));
// 		}
//
//
// 		// let mut image_thresholded = CVImage::duplicate(&image);		// all the noise cut out.
// 		// let mut image_blob = CVImage::duplicate(&image);				// Shows where blobs are.
// 		// let mut image_tracking_mode = CVImage::duplicate(&image);	// Shows chosen stars.
//
// //###############################################################################################//
// //							---	Image Processing ---
// //###############################################################################################//
// 		let mut timer : std::time::Instant = std::time::Instant::now();
//
// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~|		Test Begin		|~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//
// 		let mut histogram : [UInt; 255] = [0; 255];
// 		image.histogram(&mut histogram);
//
// 		let threshold_percent = 0.9999;
// 		let threshold : Byte = image.percent_threshold(threshold_percent, &histogram);
//
// 		let mut blobs : Vec<Blob> = Vec::new();
// 		Blob::find_blobs::<BLOB_SIZE>(threshold, &mut image, &mut blobs);
//
// 		let mut points_vec2: Vec<Vector2> = Vec::new();
// 		Blob::to_vector2(&blobs, &mut points_vec2);
//
// 		for blob in 0..blobs.len()
// 		{
// 			let pt = Pixel{x: blobs[blob].centroid.x as usize, y: blobs[blob].centroid.y as usize};
// 		}
//
// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~|		Test End		|~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//
// 		// This code is for measuring performance.
// 		let time_image_processing = timer.elapsed();
//
// 		// Cuts all thresholded parts from the image
// 		// for xx in 0..image_thresholded.width()
// 		// {
// 		// 	for yy in 0..image_thresholded.height()
// 		// 	{
// 		// 		if Image::get(&image_thresholded, Pixel{x: xx, y: yy}) < threshold
// 		// 		{
// 		// 			Image::set(&mut image_thresholded, Pixel{x: xx, y: yy}, 0);
// 		// 		}
// 		// 	}
// 		// }
//
// 		for blob in 0..blobs.len()
// 		{
// 			let point = Pixel{x: blobs[blob].centroid.x as usize, y: blobs[blob].centroid.y as usize};
// 			// image_blob.draw_point(point, (blobs[blob].intensity / 255 / 10) as Byte);
// 		}
//
// //###############################################################################################//
// //							---	Projection ---
// //###############################################################################################//
// 		// Not Finnished
// 		timer = std::time::Instant::now();
//
// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~|		Test Begin		|~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//
// 		let mut points : Vec<Equatorial> = Vec::new();
// 		for i in 0..points_vec2.size()
// 		{
// 			points.push(intrinsic.from_image(SpaceImage(points_vec2[i])).0.to_equatorial());
// 		}
//
// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~|		Test End		|~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//
// 		// This code is for measuring performance.
// 		let time_projection = timer.elapsed();
//
// //###############################################################################################//
// //							---	Tracking Mode ---
// //###############################################################################################//
// 		// Attempts to create a star pyramid.
// 		timer = std::time::Instant::now();
//
// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~|		Test Begin		|~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//
// 		let constellation : Constellation = Constellation::find::<TrackingModeConstsStruct>(
// 			&points, &mut database,
// 			&mut StarTriangleIterator::<{TrackingModeConstsStruct::PAIRS_MAX}>::new(),
// 			&mut star_tracker::tracking_mode::StarPyramid(0,0,0,0),
// 			&mut star_tracker::tracking_mode::Specularity::Ignore);
//
//
// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~|		Test End		|~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//
// 		let time_tracking_mode = timer.elapsed();
// 		let mut time_rotation = timer.elapsed();
// 		let mut angle = Radians(Decimal::INFINITY);
//
// 		match constellation
// 		{
// 			Constellation::Pyramid(_) | Constellation::Triangle(_)  =>
// 			{
// 				let matched_stars : Vec<Match<Vector3>> = convert_constellation(constellation);
// 				// for m in 0..matched_stars.len()
// 				// {
// 				// 	let input_star = matched_stars[m].input;
// 				// 	let point = intrinsic.to_image(SpaceCamera(input_star));
// 				// 	// image_tracking_mode.draw_point(SpaceImage(point), Color::Yellow, 10);
// 				// }
//
// 				timer = std::time::Instant::now();
//
// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~|		Test Begin		|~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//
// 				let matched: Vec<Match<Vector3>> = convert_constellation(constellation);
//
// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~|		Test End		|~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~//
// 				let rotation = Quest::estimate::<AttitudeDeterminationConstsStruct>(&matched);
// 				let rotated = rotation.rotate_point(center.to_vector3());
// 				angle = rotated.angle_distance(Vector3{x: 0.0, y: 0.0, z: 1.0});
// 				time_rotation = timer.elapsed();
// 			}
// 			Constellation::None =>	{	}
// 		}
//
// 		println!("Test:            {}", test_num);
// 		println!("Angle:           {}", angle.to_degrees());
// 		println!();
// 		println!("Time Processing: {:?}", time_image_processing);
// 		println!("Time Tracking:   {:?}", time_tracking_mode);
// 		println!("Time Rotation:   {:?}", time_rotation);
// 		println!();
// 		println!("Stars In Image:  {:?}", visible_stars.len());
// 		println!("Constellation:   {}", if Constellation::None == constellation { "FAILED" } else { "Success" });
// 		println!("Blobs Detected:  {:?}", blobs.len());
// 		CVImage::hide();
//
// 		println!("\n\n\n\n\n\n\n");
// 	}
// }
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
// //###############################################################################################//
// //
// //										Required
// //
// //###############################################################################################//
//
//
// /// Gets all the stars from the database.
// pub fn get_stars ( ) -> Vec<Star>
// {
//
// 	let mut stars : Vec<Star> = Vec::new();
// 	let mut rdr = Io::get_csv (
// 		NixConstsStruct::HYG_DATABASE_PATH,
// 		NixConstsStruct::HYG_DATABASE_FILE,
// 		NixConstsStruct::HYG_DATABASE_URL );
//
// 	let iter = rdr.deserialize();
// 	for record in iter
// 	{
// 		let star : Star = record.expect("Could not decode.");
// 		stars.push(star);
// 	}
// 	return stars;
// }
//
//
// /// Converts a constellation into an Vec<Match<Vector3>>
// pub fn convert_constellation ( constellation : Constellation ) -> Vec<Match<Vector3>>
// {
// 	let mut vec : Vec<Match<Vector3>> = Vec::new();
//
// 	match constellation
// 	{
// 		Constellation::Pyramid(stars) =>
// 		{
// 			vec.push_back(Match{input: stars.input.0.to_vector3(), output: stars.output.0.to_vector3(), weight: stars.weight } );
// 			vec.push_back(Match{input: stars.input.1.to_vector3(), output: stars.output.1.to_vector3(), weight: stars.weight } );
// 			vec.push_back(Match{input: stars.input.2.to_vector3(), output: stars.output.2.to_vector3(), weight: stars.weight } );
// 			vec.push_back(Match{input: stars.input.3.to_vector3(), output: stars.output.3.to_vector3(), weight: stars.weight } );
// 		}
// 		Constellation::Triangle(stars) =>
// 		{
// 			vec.push_back(Match{input: stars.input.0.to_vector3(), output: stars.output.0.to_vector3(), weight: stars.weight } );
// 			vec.push_back(Match{input: stars.input.1.to_vector3(), output: stars.output.1.to_vector3(), weight: stars.weight } );
// 			vec.push_back(Match{input: stars.input.2.to_vector3(), output: stars.output.2.to_vector3(), weight: stars.weight } );
// 		}
// 		Constellation::None => { }
// 	}
// 	return vec;
// }
//
// //###############################################################################################//
// //
// //										Error Creation
// //
// //###############################################################################################//
//
// /// Generates a random direction.
// pub fn random_direction ( ) -> Equatorial
// {
// 	let mut rng = rand::thread_rng();
// 	let axis = Equatorial{
// 		ra:  Degrees(rng.gen::<Decimal>() * 360.0).to_radians(),
// 		dec: Degrees(rng.gen::<Decimal>() * 180.0 - 90.0).to_radians()};
//
// 	return axis;
// }
//
// /// Random num.
// pub fn rand_num ( ) -> Decimal
// {
// 	let mut rng = rand::thread_rng();
// 	return rng.gen::<Decimal>();
// }
//
// /// Adds a bloom object to the image.
// /// This could be atmosphere, the moon, the sun, a problem with the sensor...
// pub fn add_bloom (img: &mut dyn NixImage, size: Decimal, brightness: Byte )
// {
// 	let mut rng = rand::thread_rng();
// 	let mut point = Vector2{
// 		x: rng.gen_range(0.0..img.width() as Decimal),
// 		y: rng.gen_range(0.0..img.height() as Decimal)};
// 	img.draw_star(size, Color::Custom(brightness, brightness, brightness), SpaceImage(point));
// }
//
//
// pub fn add_star ( )
// {
//
// }
//
// // Adds random values to each pixel within the provided range.
// pub fn add_noise ( image: &mut dyn NixImage, range: Range<Byte> )
// {
// 	let mut rng = rand::thread_rng();
// 	for x in 0..image.width()
// 	{
// 		for y in 0..image.height()
// 		{
// 			let color = rng.gen_range(range.clone());
// 			NixImage::set(image, Pixel{x, y}, Color::Custom(color, color, color));
// 		}
// 	}
// }
