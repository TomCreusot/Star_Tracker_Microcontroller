//! # Tracking Mode Test
//! This is an integration test of the whole of the tracking_mode module.
//! This includes the construction, verification and searching of the database to find specific stars.
//! This also provides a step by step guide to use the tracking mode algorithm.

use rand::prelude::*;

// use util::aliases::M_PI;
use util::aliases::Decimal;
use util::units::Radians;
use util::units::Degrees;
use util::units::Equatorial;
use util::units::Vector3;

use nix::Star;
use nix::Io;

use tracking_mode::StarPair;
use tracking_mode::Constellation;
use tracking_mode::StarPyramid;
use tracking_mode::Specularity;
use tracking_mode::StarTriangleIterator;
use tracking_mode::database::DatabaseGenerator;
use tracking_mode::database::PyramidDatabase;

use projection::ExtrinsicParameters;
use projection::SpaceWorld;

use config::NixConstsStruct;
use config::NixConsts;
use config::TrackingModeConsts;


// Defines how the tracking algorithm should perform.
pub struct TrackingConstsTest ( );
impl TrackingModeConsts for TrackingConstsTest
{
	const PAIRS_MAX       : usize = 2000;							// Irrelevant, ensure big.
	const TRIANGLES_MAX   : usize = 2000;							// Irrelevant, ensure big.
	// const SPECULARITY_MIN : Decimal = 300.0;						// If the triangle is flipped.
	const SPECULARITY_MIN : Decimal = 0.0001;						// If the triangle is flipped.
	const ANGLE_TOLERANCE : Radians = Degrees(0.05).as_radians(); 	// Maximum inaccuracy.
	// const ANGLE_TOLERANCE : Radians = Degrees(0.1).as_radians(); 	// Maximum inaccuracy.
}

pub fn run ( )
{
	let mut rng = rand::thread_rng();

	// To reduce size of database.
	const MAGNITUDE_MIN: Decimal = -20.0;
	const MAGNITUDE_MAX: Decimal = 5.8;

	const REGION_SIZE  : Radians = Degrees(6.0).as_radians(); // An area smaller than FOV.
	const REGION_NUM   : usize   = 5; // Should not be more than 1 redundant star in a region.

	// To create the database.
	const NUM_BINS     : usize   = 2000; // Refer to `src/tracking_mode/database/mod.rs`.
	const FOV          : Radians = Degrees(20.0).as_radians();


	// Disrupt input.
	const VARIATION_MAG         : Decimal = 0.5; // The variation outside of the magnitude range.
	const VARIATION_POSITION    : Radians = Degrees(0.00001).as_radians(); // Error.
	const FALSE_STARS 			: usize   = 3; // Maximum number of fake, random stars.
	const HIDDEN_STARS			: usize   = 3; // Maximum number of real stars to remove.

	const CAP_STARS             : usize   = 10; // Max stars in image.

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

	println!("Reducing Database");

	stars.sort(); // The magnitude must be sorted to get best results for `limit_regions`

	let stars_limit_mag =DatabaseGenerator::limit_magnitude (&stars, MAGNITUDE_MIN, MAGNITUDE_MAX);
	let stars_limit_reg=DatabaseGenerator::limit_regions(&stars_limit_mag,REGION_SIZE, REGION_NUM);

	println!(" - {} stars total.", stars.len());
	println!(" - {} stars when magnitude reduced.", stars_limit_mag.len());
	println!(" - {} stars when region reduced.", stars_limit_reg.len());
	println!();

	let coverage_average     = DatabaseGenerator::sky_coverage(&stars, REGION_SIZE, REGION_NUM);
	println!(" - {:0.2}% average coverage.", coverage_average * 100.0);
	let coverage_average_mag = DatabaseGenerator::sky_coverage(&stars_limit_mag, REGION_SIZE, REGION_NUM);
	println!(" - {:0.2}% average coverage magnitude reduced", coverage_average_mag * 100.0);
	let coverage_average_reg = DatabaseGenerator::sky_coverage(&stars_limit_reg, REGION_SIZE, REGION_NUM);
	println!(" - {:0.2}% average coverage region reduced", coverage_average_reg * 100.0);
	println!();
	let coverage_worst       = DatabaseGenerator::sky_coverage_worst_case(&stars, REGION_SIZE);
	println!(" - {} worst coverage.", coverage_worst);
	let coverage_worst_mag   = DatabaseGenerator::sky_coverage_worst_case(&stars_limit_mag, REGION_SIZE);
	println!(" - {} worst coverage magnitude reduced", coverage_worst_mag);
	let coverage_worst_reg   = DatabaseGenerator::sky_coverage_worst_case(&stars_limit_reg, REGION_SIZE);
	println!(" - {} worst coverage region reduced", coverage_worst_reg);
	println!();
	let coverage_best       = DatabaseGenerator::sky_coverage_best_case(&stars, REGION_SIZE);
	println!(" - {} best coverage.", coverage_best);
	let coverage_best_mag   = DatabaseGenerator::sky_coverage_best_case(&stars_limit_mag, REGION_SIZE);
	println!(" - {} best coverage magnitude reduced", coverage_best_mag);
	let coverage_best_reg   = DatabaseGenerator::sky_coverage_best_case(&stars_limit_reg, REGION_SIZE);
	println!(" - {} best coverage region reduced", coverage_best_reg);



	// The official database is based off static arrays to save memory and remove the heap.
	// When simulating the database, these variables must exist while the database exists.

	let gen : DatabaseGenerator = DatabaseGenerator::gen_database(&stars_limit_reg, FOV, NUM_BINS);
	let database = gen.get_database();

	println!();
	println!("Created database");
	println!(" - {} angles generated.", database.pairs.size());




//###############################################################################################//
//
//							---	Tracking ---
//
//
//
//###############################################################################################//

	let observation_points = Equatorial::evenly_distribute_points(REGION_SIZE);
	let observation = Equatorial::evenly_distribute(observation_points);
	println!("Performing Lost In Space");
	println!(" - {} orientations", observation_points);
	println!();
	println!("Test\t|\tTime\t\t|\tResult");

	let mut test_num           = 0;
	let mut num_pyramid        = 0;
	let mut num_triangle       = 0;
	let mut num_error          = 0;
	let mut num_error_pyramid  = 0;
	let mut num_error_triangle = 0;
	let mut avg_time = 0;
	for center in observation
	{
		let mut observable : Vec<Equatorial> = Vec::new();

		// Uses look_at to set center to +z.
		// This is how the image will present the information to the algorithm.
		let up = Equatorial{ra: Radians(0.0), dec: Degrees(90.0).to_radians()};
		let rotation = ExtrinsicParameters::look_at(center, up);

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
					Radians(rng.gen_range(-VARIATION_POSITION.0..VARIATION_POSITION.0));
				position.dec = position.dec +
					Radians(rng.gen_range(-VARIATION_POSITION.0..VARIATION_POSITION.0));

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
			let magnitude = 1.41421356; // divide by this just in-case ra and dec are max.
			let ra = Radians(rng.gen_range(-1.0..1.0) * FOV.0 / magnitude);
			let dec = Radians(rng.gen_range(-1.0..1.0) * FOV.0 / magnitude);
			let eq = Equatorial{ra: ra, dec: dec};
			observable.push(eq);
		}

		//
		// Actual Algorithm
		let timer : std::time::Instant = std::time::Instant::now();

		let mut constellation : Constellation = Constellation::find::<TrackingConstsTest>(
			&observable, &database,
			&mut StarTriangleIterator::<{TrackingConstsTest::PAIRS_MAX}>::new(),
			&mut StarPyramid(0,0,0,0), &mut Specularity::Ignore);

		let time = timer.elapsed();
		avg_time += time.as_millis();
		match print_result(test_num, time, constellation, rotation)
		{
			Result::Pyramid       => {num_pyramid        +=1;}
			Result::Triangle      => {num_triangle       +=1;}
			Result::Error         => {num_error          +=1;}
			Result::ErrorPyramid  => {num_error_pyramid  +=1;}
			Result::ErrorTriangle => {num_error_triangle +=1;}
		}
		test_num+=1;
	}

	println!("\n\n");
	println!("{}\t tests.", test_num);
	println!("{}\t pyramids identified.", num_pyramid);
	println!("{}\t triangles identified.", num_triangle);
	println!("{}\t failures.", num_error);
	println!("{}\t falsely identified pyramids.", num_error_pyramid);
	println!("{}\t falsely identified triangles.", num_error_triangle);

	println!("{} ms\t average time.", avg_time as Decimal / test_num as Decimal);
}










enum Result
{
	Pyramid,
	Triangle,
	Error,

	ErrorPyramid,
	ErrorTriangle,
}






fn print_result ( test_num: usize, time: std::time::Duration, constellation : Constellation, rotation: ExtrinsicParameters ) -> Result
{
	let result : Result;

	let color_normal   = "\x1B[0m";
	let color_bad      = "\x1B[1;31m";
	let color_very_bad = "\x1B[41;1m";
	let color_ok       = "\x1B[1;33m";
	let color_good     = "\x1B[1;32m";

	let time_good = 50;
	let time_ok   = 200;

	let tm = time.as_millis();
	let time_color = if tm < time_good { color_good } else
		{ if tm < time_ok { color_ok } else { color_bad } };

	print!("{}\t|\t{}{:4.0} ms  ", test_num, time_color, tm);
	print!("{}\t|\t", color_normal);

	match constellation
	{
		Constellation::Pyramid(pyramid) =>	// Success (4 stars identified or more)
		{
			let rotated_out_1 = rotation.to_image(SpaceWorld(pyramid.output.0.to_vector3()));
			let dist_1 = rotated_out_1.0.angle_distance(pyramid.input.0.to_vector3()).to_degrees();
			let rotated_out_2 = rotation.to_image(SpaceWorld(pyramid.output.1.to_vector3()));
			let dist_2 = rotated_out_2.0.angle_distance(pyramid.input.1.to_vector3()).to_degrees();
			let rotated_out_3 = rotation.to_image(SpaceWorld(pyramid.output.2.to_vector3()));
			let dist_3 = rotated_out_3.0.angle_distance(pyramid.input.2.to_vector3()).to_degrees();
			let rotated_out_4 = rotation.to_image(SpaceWorld(pyramid.output.3.to_vector3()));
			let dist_4 = rotated_out_4.0.angle_distance(pyramid.input.3.to_vector3()).to_degrees();

			let max_dist : Degrees = Degrees(0.01);
			let v_1 = dist_1 < max_dist;
			let v_2 = dist_2 < max_dist;
			let v_3 = dist_3 < max_dist;
			let v_4 = dist_4 < max_dist;
			if v_1 && v_2 && v_3 && v_4
			{
				print!("{}Pyramid\t\t", color_good);
				print!("T T T T");
				result = Result::Pyramid;
			}
			else
			{
				print!("{}FAILED Pyramid\t{} {} {} {}", color_very_bad, v_1,v_2,v_3,v_4);
				result = Result::ErrorPyramid;
			}
		}
		Constellation::Triangle(triangle) =>	// Mild Success (3 stars identified)
		{
			let rotated_out_1 = rotation.to_image(SpaceWorld(triangle.output.0.to_vector3()));
			let dist_1 = rotated_out_1.0.angle_distance(triangle.input.0.to_vector3()).to_degrees();
			let rotated_out_2 = rotation.to_image(SpaceWorld(triangle.output.1.to_vector3()));
			let dist_2 = rotated_out_2.0.angle_distance(triangle.input.1.to_vector3()).to_degrees();
			let rotated_out_3 = rotation.to_image(SpaceWorld(triangle.output.2.to_vector3()));
			let dist_3 = rotated_out_3.0.angle_distance(triangle.input.2.to_vector3()).to_degrees();

			let max_dist : Degrees = Degrees(0.01);
			let v_1 = dist_1 < max_dist;
			let v_2 = dist_2 < max_dist;
			let v_3 = dist_3 < max_dist;

			if v_1 && v_2 && v_3
			{
				print!("{}Triangle\t\t", color_ok);
				print!("T T T T");
				result = Result::Triangle;
			}
			else
			{
				print!("{}FAILED Triangle\t{} {} {}", color_very_bad, v_1, v_2, v_3);
				result = Result::ErrorTriangle;
			}
		}
		Constellation::None => 				// No stars identified.
		{
			print!("{}FAILED\t\tF F F F", color_bad);
			result = Result::Error;
		}
	}
	println!("{}", color_normal);
	return result;
}
