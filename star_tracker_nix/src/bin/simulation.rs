#![allow(unused_imports)]
#![macro_use]
//! This is an integration test of the whole of the tracking_mode module.
//! This includes the construction, verification and searching of the database to find specific stars.
//! This also provides a step by step guide to use the tracking mode algorithm.
//!

#[macro_use]
extern crate star_tracker_lib;
extern crate star_tracker_nix;
extern crate image;
extern crate rand;
extern crate opencv;

use rand::prelude::*;

use std::time::Duration;

use opencv::highgui::wait_key;
use opencv::highgui::imshow;

use star_tracker_lib::util::aliases::Decimal;
use star_tracker_lib::util::aliases::M_PI;
use star_tracker_lib::util::units::Radians;
use star_tracker_lib::util::units::Degrees;
use star_tracker_lib::util::units::Hours;
use star_tracker_lib::util::units::Match;
use star_tracker_lib::util::units::Equatorial;
use star_tracker_lib::util::units::Vector3;
use star_tracker_lib::util::units::Vector2;
use star_tracker_lib::util::units::Quaternion;
use star_tracker_lib::util::units::Pixel;
use star_tracker_lib::util::distribution::Distribute;
use star_tracker_lib::util::linear_lookup::LinearLookup;


use star_tracker_lib::tracking_mode::Constellation;
use star_tracker_lib::tracking_mode::StarPyramid;
use star_tracker_lib::tracking_mode::Specularity;
use star_tracker_lib::tracking_mode::StarTriangleIterator;
use star_tracker_lib::tracking_mode::AbandonSearch;
use star_tracker_lib::tracking_mode::AbandonSearchNone;
use star_tracker_lib::tracking_mode::PilotFinder;
use star_tracker_lib::tracking_mode::database::ChunkIterator;
use star_tracker_lib::tracking_mode::database::ChunkIteratorNone;
use star_tracker_lib::tracking_mode::database::ChunkIteratorRegional;
use star_tracker_lib::tracking_mode::database::ChunkIteratorEquatorial;
use star_tracker_lib::tracking_mode::database::ChunkIteratorDeclination;
use star_tracker_lib::tracking_mode::database::ChunkAreaSearch;
use star_tracker_lib::tracking_mode::database::SearchResult;

use star_tracker_lib::image_processing::Image;

use star_tracker_lib::projection::ExtrinsicParameters;
use star_tracker_lib::projection::SpaceWorld;
use star_tracker_lib::projection::SpaceImage;

use star_tracker_lib::attitude_determination::Quest;
use star_tracker_lib::attitude_determination::AttitudeDetermination;

use star_tracker_nix::io::Star;
use star_tracker_nix::io::Io;
use star_tracker_nix::tracking_mode::DatabaseGenerator;
use star_tracker_nix::tracking_mode::AbandonSearchTimeoutFailure;
use star_tracker_nix::image_processing::Color;
use star_tracker_nix::image_processing::CVImage;


pub fn main ( )
{
	std::env::set_var("RUST_BACKTRACE", "1");
	println!(r#"
	
	
				===== Simulation =====
This is an integration test of the whole of the tracking_mode module.
This includes the construction, verification and searching of the database to find specific stars.
This also provides a step by step guide to use the tracking mode algorithm.

	  
	"#);

	
	let mut rng = rand::thread_rng();
	
	const ANGLE_TOLERANCE : Radians = Degrees(0.04).as_radians();
	const MAGNITUDE_MIN : Decimal = -20.0;
	const MAGNITUDE_MAX : Decimal =  6.0;
	
	// The separation of the sample points, make this smaller than the FOV so you can test more edge cases.
	const SAMPLE_FOV   : Radians = Degrees(5.0).as_radians();
	
	
	// Region Reduction
	const REGION_SIZE  : Radians = Degrees(10.0).as_radians(); // An area smaller than FOV.
	const REGION_NUM   : usize   = 8;   // Should not be more than 1 redundant star in a region.
	
	// If stars are this close, one is excluded.
	const DOUBLE_STAR_TOLERANCE : Radians = ANGLE_TOLERANCE;//Degrees(0.2).as_radians();
	
	// To create the database.
	const FOV          : Radians = Degrees(18.0).as_radians();
	
	
	// Disrupt input.
	const VARIATION_MAG         : Decimal = 0.1;//0.1; // The variation outside of the magnitude range.
	const FALSE_STARS           : usize   = 0;//4; // Maximum number of fake, random stars.
	const HIDDEN_STARS          : usize   = 0; // Maximum number of real stars to remove.
	const VARIATION_POSITION_STD_DEV : Radians = Degrees(0.014).as_radians(); // Standard Deviation of Error.
	const VARIATION_POSITION_MEAN    : Radians = Degrees(0.012).as_radians(); // Mean of Error.
	
	const CAP_STARS             : usize   = 20; // Max stars in image.
	
	
	// Loose conditions
	const TIME_GOOD             : u128 = 500; // ms until autofail.
	const FAILURE_GOOD          : usize = 300; // # of triangle failures until a failure.
	let ERROR_GOOD            : Radians = Degrees(1.0).to_radians(); // The acceptable error of the result
	
	//###############################################################################################//
	//
	//							---	Constructing Database ---
	// The database must first be constructed:
	// * The main star csv database will be read, this is used as the star distribution is accurate.
	//
	// * The stars will then cut stars outside the magnitude, this includes the sun and dark stars.
	//
	// * The stars will then have the region cut, this is because some regions of the sky will have
	// far more stars than others, buy removing the redundant stars, the database shrinks in size.
	//
	// * Using the shortcut: `DatabaseGenerator::gen_database`, the database can be generated easily.
	//###############################################################################################//
	
	println!("Reading CSV Database");
	let mut stars : Vec<Star> = Io::get_csv_database();
	
	println!("Reducing Database");
	
	stars.sort(); // The magnitude must be sorted to get best results for `limit_regions`
	
	println!("* Magnitude Reduction");
	let stars_limit_mag    = DatabaseGenerator::limit_magnitude (&stars, MAGNITUDE_MIN, MAGNITUDE_MAX);
	println!("* Double Star Reduction");
	let stars_limit_double = DatabaseGenerator::limit_double_stars(&stars_limit_mag, DOUBLE_STAR_TOLERANCE);
	println!("* Region Reduction");
	let stars_limit_reg = DatabaseGenerator::limit_regions(&stars_limit_double,REGION_SIZE, REGION_NUM);
	
	println!("* Drawing Map");
	let mut sky_map = CVImage::new(Pixel{x: 1280, y: 720});
	// draw(-1000.0, 4.0, stars_limit_mag.clone(), &mut sky_map);
	
	println!(" - {} stars total.", stars.len());
	println!(" - {} stars when magnitude reduced.", stars_limit_mag.len());
	println!(" - {} stars when double star reduced.", stars_limit_double.len());
	println!(" - {} stars when region reduced.", stars_limit_reg.len());

	// Set to true to provide coverage report
	if false
	{
		println!("\n");
		let coverage_average     = DatabaseGenerator::sky_coverage(&stars, REGION_SIZE, REGION_NUM);
		println!(" - {:0.2}% average coverage.", coverage_average * 100.0);
		let coverage_average_mag = DatabaseGenerator::sky_coverage(&stars_limit_mag, REGION_SIZE, REGION_NUM);
		println!(" - {:0.2}% average coverage magnitude reduced", coverage_average_mag * 100.0);
		let coverage_average_db  = DatabaseGenerator::sky_coverage(&stars_limit_double, REGION_SIZE, REGION_NUM);
		println!(" - {:0.2}% average coverage double reduced", coverage_average_db * 100.0);
		let coverage_average_reg = DatabaseGenerator::sky_coverage(&stars_limit_reg, REGION_SIZE, REGION_NUM);
		println!(" - {:0.2}% average coverage region reduced", coverage_average_reg * 100.0);
		println!();
		let coverage_worst     = DatabaseGenerator::sky_coverage_worst_case(&stars, REGION_SIZE);
		println!(" - {} worst coverage.", coverage_worst);
		let coverage_worst_mag = DatabaseGenerator::sky_coverage_worst_case(&stars_limit_mag, REGION_SIZE);
		println!(" - {} worst coverage magnitude reduced", coverage_worst_mag);
		let coverage_worst_db  = DatabaseGenerator::sky_coverage_worst_case(&stars_limit_double, REGION_SIZE);
		println!(" - {} worst coverage double reduced", coverage_worst_db);
		let coverage_worst_reg = DatabaseGenerator::sky_coverage_worst_case(&stars_limit_reg, REGION_SIZE);
		println!(" - {} worst coverage region reduced", coverage_worst_reg);
		println!();
		let coverage_best      = DatabaseGenerator::sky_coverage_best_case(&stars, REGION_SIZE);
		println!(" - {} best coverage.", coverage_best);
		let coverage_best_mag  = DatabaseGenerator::sky_coverage_best_case(&stars_limit_mag, REGION_SIZE);
		println!(" - {} best coverage magnitude reduced", coverage_best_mag);
		let coverage_best_db   = DatabaseGenerator::sky_coverage_best_case(&stars_limit_double, REGION_SIZE);
		println!(" - {} best coverage double reduced", coverage_best_db);
		let coverage_best_reg  = DatabaseGenerator::sky_coverage_best_case(&stars_limit_reg, REGION_SIZE);
		println!(" - {} best coverage region reduced", coverage_best_reg);
	}

	// let gen_2 : DatabaseGenerator = DatabaseGenerator::gen_database(&stars_limit_mag, FOV, FOV, ANGLE_TOLERANCE);
	// let gen_2 : DatabaseGenerator = DatabaseGenerator::gen_database(&stars_limit_mag, FOV, FOV/1.5, ANGLE_TOLERANCE);
	// let database_2 = gen_2.get_database();
	// println!(" - {} angles generated (without).", database_2.pairs.size());


	// let gen : DatabaseGenerator = DatabaseGenerator::gen_database(&stars_limit_mag, FOV, FOV / 1.3, ANGLE_TOLERANCE);
	// let gen : DatabaseGenerator = DatabaseGenerator::gen_database_regional(&stars_limit_reg, FOV, FOV/1.3, ANGLE_TOLERANCE);
	let gen : DatabaseGenerator = DatabaseGenerator::gen_database_regional(&stars_limit_reg, FOV, FOV, ANGLE_TOLERANCE);
	// let database = gen.get_database();
	let database = gen.get_database_regional();


	let mut database_iterator = ChunkIteratorNone::new(&database);

	// database
	// let mut database_iterator = ChunkIteratorRegional::new(&database);


	// database			distance between chunks		percent overlap between chunks
	// let mut database_iterator = ChunkIteratorEquatorial::new(&database, Degrees(50.0).as_radians(), 0.2);

	// database			distance between chunks		percent overlap between chunks (0 is when the boundaries are touching, 1 is when they meet half way.) 
	// let mut database_iterator = ChunkIteratorDeclination::new(&database, Degrees(10.0).as_radians(), 1.0, ChunkIteratorDeclination::randomise_none);
	// let mut database_iterator = ChunkIteratorDeclination::new(&database, Degrees(10.0).as_radians(), 1.0, ChunkIteratorDeclination::randomise_parity);


	// let mut last = 0;
	// for i in 0..database.k_vector.size()
	// {
	// 	println!("{}", database.k_vector.get(i) - last);
	// 	last = database.k_vector.get(i);
	// }

	// for i in 0..stars_limit_mag
	// {
	// 	let close
	// 	for j in 0..stars_limit_mag
	// 	{

	// 	}
	// }

	println!();
	println!("Created database");
	println!(" - {} angles generated.", database.pairs.size());
	println!(" - {} bins", database.k_vector.size());




//###############################################################################################//
//
//							---	Tracking ---
//
//
//
//###############################################################################################//

	let observation_points = Distribute::angle_to_points(SAMPLE_FOV);
	let observation = Distribute::fibonacci_lattice(observation_points);
	println!("Performing Lost In Space");
	println!(" - {} orientations", observation_points);
	println!();
	println!("Test\t|\tStars\t|\tSeparation\t|\tTime\t\t|\tResult\t|\tLocation");

	let mut test_num           = 0;
	let mut num_pyramid        = 0;
	let mut num_triangle       = 0;
	let mut num_error          = 0;
	let mut num_error_time     = 0;
	let mut num_error_pyramid  = 0;
	let mut num_error_triangle = 0;
	let mut avg_time_pyramid   = 0;
	let mut avg_time_triangle  = 0;
	let mut avg_time_error     = 0;
	let mut avg_time_cap       = 0;
	let mut avg_time_false_positive= 0;

	// let center = Equatorial{ra: Degrees(30.0).to_radians(), dec: Degrees(50.0).to_radians()};
	for center in observation
	{
		// let mut database_iterator = ChunkAreaSearch::from_point(&database, center, FOV * 1.1 + Degrees(5.0).to_radians());


		let mut observable : Vec<Equatorial> = Vec::new();

		// Uses look_at to set center to +z.
		// This is how the image will present the information to the algorithm.
		let up = Equatorial{ra: Radians(0.0), dec: Degrees(90.0).to_radians()};
		let rotation =
			ExtrinsicParameters::look_at(center, up).expect("Up and forward can be the same");

		let mut i = 0;
		// Gets stars in observed center, uses main database to add some variation.
		while i < stars.len() && observable.len() < CAP_STARS
		{
			let rand_min_mag = MAGNITUDE_MIN - rng.gen_range(-VARIATION_MAG..VARIATION_MAG);
			let rand_max_mag = MAGNITUDE_MAX + rng.gen_range(-VARIATION_MAG..VARIATION_MAG);

			if  rand_min_mag < stars[i].mag && stars[i].mag < rand_max_mag
			  && stars[i].pos.angle_distance(center) < FOV / 2.0
			{
				let mut position = stars[i].pos;
				position.ra  = position.ra +
					Radians(gen_random_sd(&mut rng, VARIATION_POSITION_MEAN.0, VARIATION_POSITION_STD_DEV.0));
				position.dec = position.dec +
					Radians(gen_random_sd(&mut rng, VARIATION_POSITION_MEAN.0, VARIATION_POSITION_STD_DEV.0));

				let rotated = rotation.to_image(SpaceWorld(position.to_vector3())).0;
				observable.push(rotated.to_equatorial());
			}
			i += 1;
		}


		// Hides stars
		for _i in 0..HIDDEN_STARS
		{
			observable.remove(rng.gen_range(0..observable.len() - 1));
		}


		// Adds fake stars.
		for _i in 0..FALSE_STARS
		{
			let ninety_deg = Degrees(90.0).to_radians();
			let ra = Radians(rng.gen_range(0.0..1.0) * 6.28);
			let dec = Radians(rng.gen_range((ninety_deg.0 - FOV.0)..ninety_deg.0));
			let eq = Equatorial{ra: ra, dec: dec};
			observable.push(eq);
		}

		//
		// Actual Algorithm
		let timer : std::time::Instant = std::time::Instant::now();
		// 
		let mut found_all : Vec<Match<usize>> = Vec::new();

		let success = Constellation::find(
			&observable, &mut database_iterator,
			&mut StarTriangleIterator::<1000>::new(),
			&mut Specularity::default(),
			&mut AbandonSearchTimeoutFailure::new(Duration::from_millis(TIME_GOOD as u64), FAILURE_GOOD),
			// &mut AbandonSearchNone(),
			ANGLE_TOLERANCE,
			4..=4,
			&mut found_all,
		);
		
		// let success = Constellation::old_find(
		// 	&observable, &mut database_iterator,
		// 	&mut StarTriangleIterator::<1000>::new(),
		// 	&mut Specularity::default(),
		// 	&mut AbandonSearchNone(),//TimeoutFailure::new(Duration::from_millis(TIME_GOOD as u64), FAILURE_GOOD),
		// 	ANGLE_TOLERANCE,
		// 	4..=4,
		// 	&mut found_all,
		// 	&mut pilot_finder_vec!()
		// );
		
		let time = timer.elapsed();
		match print_result(test_num, time, found_all, center, rotation, observable, database.catalogue, TIME_GOOD, ERROR_GOOD)
		{
			Result::Pyramid       =>{
				num_pyramid        +=1; avg_time_pyramid  += time.as_millis(); 
				draw_pt(center, Color::Blue, 4, &mut sky_map);
			}
			Result::Triangle      =>{
				num_triangle       +=1; avg_time_triangle += time.as_millis();
				draw_pt(center, Color::Yellow, 4, &mut sky_map);
			}
			Result::Error         =>{
				num_error          +=1; avg_time_error    += time.as_millis();
				draw_pt(center, Color::Yellow, 4, &mut sky_map);
			
			}
			Result::ErrorPyramid  =>{num_error_pyramid  +=1; avg_time_false_positive += time.as_millis();
				draw_pt(center, Color::Red, 4, &mut sky_map);
			}
			Result::ErrorTriangle =>{num_error_triangle +=1; avg_time_error    += time.as_millis();
				draw_pt(center, Color::Yellow, 4, &mut sky_map);
			}
			Result::ErrorTime     =>{num_error_time +=1; avg_time_cap          += time.as_millis();
				draw_pt(center, Color::Yellow, 4, &mut sky_map);
			
			}
		}
		test_num+=1;
		
		
		// let _ = imshow("Sky", &sky_map.0);
	}


	println!("\n\n");
	println!("{}\t tests.", test_num);
	println!("{}\t pyramids identified.", num_pyramid);
	println!("{}\t triangles identified.", num_triangle);
	println!("{}\t failures.", num_error);
	println!("{}\t falsely identified pyramids.", num_error_pyramid);
	println!("{}\t falsely identified triangles.", num_error_triangle);
	println!("{}\t exceeded time requirement and was valid.", num_error_time);

	println!("{:.2} ms\t avg time pyramid." ,avg_time_pyramid as Decimal /num_pyramid as Decimal);
	println!("{:.2} ms\t avg time triangle.",avg_time_triangle as Decimal/num_triangle as Decimal);
	println!("{:.2} ms\t avg time error.",   avg_time_error as Decimal   /(num_error + num_error_triangle) as Decimal);
	println!("{:.2} ms\t avg time cap.",     avg_time_cap as Decimal   /(num_error_time as Decimal));
	println!("{:.2} ms\t avg time false_positive.",   avg_time_false_positive as Decimal /(num_error_pyramid as Decimal));

	println!("");
	println!("");
	println!("------------- Config  -------------");
	println!("MAGNITUDE:      {} to {}", MAGNITUDE_MIN, MAGNITUDE_MAX);
	println!("FOV:            {}", FOV.to_degrees());
	println!("FOV REGION:     {}", REGION_SIZE.to_degrees());
	println!("");
	println!("STARS/REGION:   {}", REGION_NUM);
	println!("");
	println!("VAR MAG:        {}", VARIATION_MAG);
	println!("VAR POS:        {}", VARIATION_POSITION_MEAN.to_degrees());
	println!("VAR POS:        {}", VARIATION_POSITION_STD_DEV.to_degrees());
	println!("FALSE STARS:    {}", FALSE_STARS);
	println!("HIDDEN STARS:   {}", HIDDEN_STARS);
	println!("");
	println!("STARS IN IMAGE: {}", CAP_STARS);

	// star_tracker_nix::image_processing::NixImage::save(&sky_map, "sky_sim.png");
	// let _ = wait_key(0);

}








































#[derive(PartialEq)]
enum Result
{
	Pyramid,
	Triangle,
	Error,

	ErrorPyramid,
	ErrorTriangle,
	ErrorTime,
}








fn print_result ( 
	test_num: usize, 
	time: std::time::Duration, 
	constellation : Vec<Match<usize>>, 
	location: Equatorial, 
	rotation: ExtrinsicParameters, 
	stars_image: Vec<Equatorial>, stars_database: &dyn LinearLookup<Equatorial>,
	time_good: u128, allowed_error: Radians ) -> Result
{
	let result : Result;

	let color_normal   = "\x1B[0m";
	let color_bad      = "\x1B[1;31m";
	let color_very_bad = "\x1B[41;1m";
	let color_ok       = "\x1B[1;34m";
	let color_good     = "\x1B[1;32m";

	let tm = time.as_millis();
	let time_color = if tm < time_good { color_good } else { color_bad };

	print!("{}\t|\t{}\t|\t{}{:4.0} ms  ", test_num, stars_image.len(), time_color, tm);
	print!("{}\t|\t", color_normal);
	
	let mut found_stars : Vec<Match<Vector3>> = Vec::new();
	for i in 0..constellation.size()
	{
		let input  = stars_image[constellation[i].input].to_vector3();
		let output = stars_database.get(constellation[i].output).to_vector3();
		let _ = found_stars.push( Match{ input:  input, output: output, weight: 1.0 } );
	}
	match constellation.len()
	{
		4 =>	// Success (4 stars identified or more)
		{
			let rotate_to_cam  : Quaternion = Quest::estimate(&found_stars, None);
			let rotate_to_world: Quaternion = rotate_to_cam.conjugate();
			let world_center = rotate_to_world.rotate_point(Vector3{x: 0.0, y: 0.0, z: 1.0});
			let error = world_center.angle_distance(location.to_vector3());
			if error < allowed_error
			{
				print!("{}{:20} | {}", color_good, "Pyramid", error.to_degrees());
				result = Result::Pyramid;
			}
			else
			{
				print!("{}{:20} | {}", color_very_bad, "FAILED: Pyramid", error.to_degrees());
				result = Result::ErrorPyramid;
			}
		}
		3 =>	// Mild Success (3 stars identified)
		{
			let rotate_to_cam  : Quaternion = Quest::estimate(&found_stars, None);
			let rotate_to_world: Quaternion = rotate_to_cam.conjugate();
			let world_center = rotate_to_world.rotate_point(Vector3{x: 0.0, y: 0.0, z: 1.0});
			let error = world_center.angle_distance(location.to_vector3());
			if error < allowed_error
			{
				print!("{}{:20} | {}", color_ok, "TRIANGLE", error.to_degrees());
				result = Result::Triangle;
			}
			else
			{
				print!("{}{:20} | {}", color_bad, "FAILED: TRIANGLE", error.to_degrees());
				result = Result::ErrorTriangle;
			}
		}
		0 => 				// No stars identified.
		{
			print!("{}{:20} |     ", color_bad, "FAILED");
			result = Result::Error;
		}
		
		_ => panic!("There should only be these options"),
	}
	print!("{}\t\t|\t{}", color_normal, print_standard_equatorial(location));
	
	
	if time_good < tm && result != Result::Error && result != Result::ErrorPyramid && result != Result::ErrorTriangle
	{
		println!("FAILED FROM TIMEOUT: {} ms", tm);
		return Result::ErrorTime;
	}
	else
	{
		println!("");
	}

	
	
	return result;
}

// Nice formatting for True
fn print_true ( val : bool ) -> String
{
	let color = if !val {"\x1B[41;1m"} else {"\x1B[0m\x1B[1;32m"};
	let text = if val { "T" } else { "F" };
	return format!("{}{}\x1B[0m", color, text);
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



/// Uses Box Muller technique to generate a randomly generated normally distributed number.
pub fn gen_random_sd ( rng: &mut impl Rng, mean: Decimal, std_dev: Decimal ) -> Decimal
{
	let u1: Decimal = rng.gen();
	let u2: Decimal = rng.gen();

	let z0 = (-2.0 * u1.ln() as Decimal).sqrt() * (2.0 * M_PI * u2).cos();
	return mean + std_dev * z0;
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