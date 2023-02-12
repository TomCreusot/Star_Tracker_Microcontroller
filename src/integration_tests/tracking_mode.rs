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
// use util::units::Vector3;

use nix::Star;
use nix::Io;

// use tracking_mode::StarPair;
use tracking_mode::Constellation;
use tracking_mode::StarPyramid;
use tracking_mode::Specularity;
use tracking_mode::StarTriangleIterator;
use nix::DatabaseGenerator;
use nix::Distribute;
// use tracking_mode::database::PyramidDatabase;

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
	const ANGLE_TOLERANCE : Radians = Degrees(0.1).as_radians(); 	// Maximum inaccuracy.
	// const ANGLE_TOLERANCE : Radians = Degrees(0.1).as_radians(); // Maximum inaccuracy.
}

pub fn run ( )
{
	let mut rng = rand::thread_rng();

	// To reduce size of database.
	const MAGNITUDE_MIN: Decimal = -20.0;
	const MAGNITUDE_MAX: Decimal = 5.8;

	// The fov of each sample of the sky (make it close to region size to get a reliable test).
	const SAMPLE_FOV   : Radians = Degrees(5.0).as_radians();

	const REGION_SIZE  : Radians = Degrees(30.0).as_radians(); // An area smaller than FOV.
	const REGION_NUM   : usize   = 20; // Should not be more than 1 redundant star in a region.

	// To create the database.
	const NUM_BINS     : usize   = 2000; // Refer to `src/tracking_mode/database/mod.rs`.
	const FOV          : Radians = Degrees(30.0).as_radians();


	// Disrupt input.
	const VARIATION_MAG         : Decimal = 0.1; // The variation outside of the magnitude range.
	const VARIATION_POSITION    : Radians = Degrees(0.06).as_radians(); // Error.
	const FALSE_STARS 			: usize   = 4; // Maximum number of fake, random stars.
	const HIDDEN_STARS			: usize   = 0; // Maximum number of real stars to remove.

	const CAP_STARS             : usize   = 15; // Max stars in image.

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

	// let coverage_average     = DatabaseGenerator::sky_coverage(&stars, REGION_SIZE, REGION_NUM);
	// println!(" - {:0.2}% average coverage.", coverage_average * 100.0);
	// let coverage_average_mag = DatabaseGenerator::sky_coverage(&stars_limit_mag, REGION_SIZE, REGION_NUM);
	// println!(" - {:0.2}% average coverage magnitude reduced", coverage_average_mag * 100.0);
	// let coverage_average_reg = DatabaseGenerator::sky_coverage(&stars_limit_reg, REGION_SIZE, REGION_NUM);
	// println!(" - {:0.2}% average coverage region reduced", coverage_average_reg * 100.0);
	// println!();
	// let coverage_worst       = DatabaseGenerator::sky_coverage_worst_case(&stars, REGION_SIZE);
	// println!(" - {} worst coverage.", coverage_worst);
	// let coverage_worst_mag   = DatabaseGenerator::sky_coverage_worst_case(&stars_limit_mag, REGION_SIZE);
	// println!(" - {} worst coverage magnitude reduced", coverage_worst_mag);
	// let coverage_worst_reg   = DatabaseGenerator::sky_coverage_worst_case(&stars_limit_reg, REGION_SIZE);
	// println!(" - {} worst coverage region reduced", coverage_worst_reg);
	// println!();
	// let coverage_best       = DatabaseGenerator::sky_coverage_best_case(&stars, REGION_SIZE);
	// println!(" - {} best coverage.", coverage_best);
	// let coverage_best_mag   = DatabaseGenerator::sky_coverage_best_case(&stars_limit_mag, REGION_SIZE);
	// println!(" - {} best coverage magnitude reduced", coverage_best_mag);
	// let coverage_best_reg   = DatabaseGenerator::sky_coverage_best_case(&stars_limit_reg, REGION_SIZE);
	// println!(" - {} best coverage region reduced", coverage_best_reg);



	// The official database is based off static arrays to save memory and remove the heap.
	// When simulating the database, these variables must exist while the database exists.

	// let gen : DatabaseGenerator = DatabaseGenerator::gen_database(&stars_limit_reg, FOV, NUM_BINS);
	// let database = gen.get_database();
	let gen : DatabaseGenerator = DatabaseGenerator::gen_database_regions(&stars_limit_reg, FOV, REGION_SIZE, NUM_BINS);
	let mut database = gen.get_database_regional();

	// for i in 0..database.pairs.size()
	// {
	// 	println!("{}\t\t{}\t\t{:#032b}", database.catalogue.get(database.pairs.get(i).0), database.catalogue.get(database.pairs.get(i).1), database.pairs_region.get(i));
	// }
	// panic!("");

	

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

	let observation_points = Distribute::angle_to_points(SAMPLE_FOV);
	let observation = Distribute::fibonacci_latice(observation_points);
	println!("Performing Lost In Space");
	println!(" - {} orientations", observation_points);
	println!();
	println!("Test\t|\tStars\t|\tSeparation\t|\tTime\t\t|\tResult\t|\tLocation");

	let mut test_num           = 0;
	let mut num_pyramid        = 0;
	let mut num_triangle       = 0;
	let mut num_error          = 0;
	let mut num_error_pyramid  = 0;
	let mut num_error_triangle = 0;
	let mut avg_time_pyramid   = 0;
	let mut avg_time_triangle  = 0;
	let mut avg_time_error     = 0;
	
	// let center = Equatorial{ra: Degrees(30.0).to_radians(), dec: Degrees(50.0).to_radians()};
	for center in observation
	{
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
					Radians(rng.gen_range(-VARIATION_POSITION.0..VARIATION_POSITION.0));
				position.dec = position.dec +
					Radians(rng.gen_range(-VARIATION_POSITION.0..VARIATION_POSITION.0));

				let rotated = rotation.to_image(SpaceWorld(position.to_vector3())).0;
				observable.push(rotated.to_equatorial());
				// println!("{}   \t{}   \t{}", position.print_standard(), rotated.to_equatorial().print_standard(), position.angle_distance(rotated.to_equatorial()));
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
			let ninty_deg = Degrees(90.0).to_radians();
			let ra = Radians(rng.gen_range(0.0..1.0) * 6.28);
			let dec = Radians(rng.gen_range((ninty_deg.0 - FOV.0)..ninty_deg.0));
			let eq = Equatorial{ra: ra, dec: dec};
			observable.push(eq);
		}
		

		//
		// Actual Algorithm
		let timer : std::time::Instant = std::time::Instant::now();

		let constellation : Constellation = Constellation::find::<TrackingConstsTest>(
			&observable, &mut database,
			&mut StarTriangleIterator::<{TrackingConstsTest::PAIRS_MAX}>::new(),
			&mut StarPyramid(0,0,0,0), &mut Specularity::Ignore);

		let time = timer.elapsed();
		match print_result(test_num, time, constellation, center, rotation, observable)
		{
			Result::Pyramid      =>{num_pyramid        +=1; avg_time_pyramid  += time.as_millis();}
			Result::Triangle     =>{num_triangle       +=1; avg_time_triangle += time.as_millis();}
			Result::Error        =>{num_error          +=1; avg_time_error    += time.as_millis();}
			Result::ErrorPyramid =>{num_error_pyramid  +=1; avg_time_error    += time.as_millis();}
			Result::ErrorTriangle=>{num_error_triangle +=1; avg_time_error    += time.as_millis();}
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

	println!("{:.2} ms\t avg time pyramid." ,avg_time_pyramid as Decimal /num_pyramid as Decimal);
	println!("{:.2} ms\t avg time triangle.",avg_time_triangle as Decimal/num_triangle as Decimal);
	println!("{:.2} ms\t avg time error.",   avg_time_error as Decimal   /(num_error + num_error_pyramid + num_error_triangle) as Decimal);
	
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
	println!("VAR POS:        {}", VARIATION_POSITION.to_degrees());
	println!("FALSE STARS:    {}", FALSE_STARS);
	println!("HIDDEN STARS:   {}", HIDDEN_STARS);
	println!("");
	println!("STARS IN IMAGE: {}", CAP_STARS);
}










enum Result
{
	Pyramid,
	Triangle,
	Error,

	ErrorPyramid,
	ErrorTriangle,
}






fn print_result ( test_num: usize, time: std::time::Duration, constellation : Constellation, location: Equatorial, rotation: ExtrinsicParameters, stars: Vec<Equatorial> ) -> Result
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

	print!("{}\t|\t{}\t|\t{}{:4.0} ms  ", test_num, stars.len(), time_color, tm);
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

			let max_dist : Degrees = Degrees(0.1);
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
				let t_1 = print_true(v_1); let t_2 = print_true(v_2);
				let t_3 = print_true(v_3); let t_4 = print_true(v_4);
				print!("{}FAILED Pyramid\t{} {} {} {}", color_very_bad, t_1,t_2,t_3,t_4);
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

			let max_dist : Degrees = Degrees(0.1);
			let v_1 = dist_1 < max_dist;
			let v_2 = dist_2 < max_dist;
			let v_3 = dist_3 < max_dist;


			if v_1 && v_2 && v_3
			{
				print!("{}Triangle\t\t", color_ok);
				print!("T T T");
				result = Result::Triangle;
			}
			else
			{
				let t_1 = print_true(v_1); let t_2 = print_true(v_2); let t_3 = print_true(v_3);
				print!("{}FAILED Triangle\t{} {} {}", color_very_bad, t_1, t_2, t_3);
				result = Result::ErrorTriangle;
			}
		}
		Constellation::None => 				// No stars identified.
		{
			print!("{}FAILED\t\tF F F F", color_bad);
			result = Result::Error;
		}
	}
	println!("{}\t\t|\t{}", color_normal, location.print_standard());
	return result;
}

// Nice formatting for True
fn print_true ( val : bool ) -> String
{
	let color = if !val {"\x1B[41;1m"} else {"\x1B[0m\x1B[1;32m"};
	let text = if val { "T" } else { "F" };
	return format!("{}{}\x1B[0m", color, text);
}
