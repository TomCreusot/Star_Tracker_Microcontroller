//! # Tracking Mode Test
//! This is an integration test of the whole of the tracking_mode module.
//! This includes the construction, verification and searching of the database to find specific stars.
//! This also provides a step by step guide to use the tracking mode algorithm.

use rand::prelude::*;
// use rand::SeedableRng;
// use rand::distributions::Standard;
// use rand::distributions::Distribution;

// use util::aliases::M_PI;
use util::aliases::Decimal;
use util::units::Radians;
use util::units::Degrees;
use util::units::Equatorial;

use nix::Star;
use nix::Io;

// use tracking_mode::StarPair;
use tracking_mode::Constellation;
use tracking_mode::StarPyramid;
use tracking_mode::Specularity;
use tracking_mode::StarTriangleIterator;
use tracking_mode::database::RegionalIterator;
use tracking_mode::database::PyramidIterator;
use tracking_mode::database::BoundedEquatorialIterator;
use tracking_mode::database::BoundedDeclinationIterator;
use tracking_mode::database::RegionalCrunchIterator;
use nix::DatabaseGenerator;
use nix::Distribute;

use projection::ExtrinsicParameters;
use projection::SpaceWorld;

use config::NixConstsStruct;
use config::NixConsts;
use config::TrackingModeConsts;


struct TestConditions
{
	pub fov:         Radians,
	pub region_size: Radians,
	pub region_num:  usize,
	pub time_cap:    u128,
	pub reach_multiplier: Decimal,
}


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
	
	// To create the database.
	const NUM_BINS     : usize   = 2000; // Refer to `src/tracking_mode/database/mod.rs`.

	const MAGNITUDE: Decimal = 5.8;

	// The separation of the sample points, make this smaller than the FOV so you can test more edge cases.
	const SAMPLE_FOV   : Radians = Degrees(5.0).as_radians();

	// If stars are this close, one is excluded.
	const DOUBLE_STAR_TOLERANCE : Radians = Degrees(0.2).as_radians();


	// Loose conditions
	const TIME_GOOD    : u128 = 80; // ms until autofail.
	
	let file_path = "results/integration_tests/iterator/BoundedEquatorialIterator.csv";
	

	
	let mut file_str : Vec<String> = vec!(
		"Method,FOV,Sample Image FOV,Magnitude,Double Star Tolerance,Region Size,Stars In Region,Number Tests,SUCCESSFUL PYRAMID IDENTIFICATIONS,SUCCESSFUL TRIANGLE IDENTIFICATIONS,FAILED IDENTIFICATIONS,FALSE PYRAMID IDENTIFICATIONS,FALSE TRIANGLE IDENTIFICATIONS,AVERAGE TIME PYRAMID,AVERAGE TIME TRIANGLE,AVERAGE TIME ERROR,STANDARD DEVIATION PYRAMID".to_string());
	Io::write_file(file_path, &file_str);

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

	stars.sort(); // The magnitude must be sorted to get best results for `limit_regions`
	let stars_limit_mag    = DatabaseGenerator::limit_magnitude (&stars, -10.0, MAGNITUDE);
	let stars_limit_double = DatabaseGenerator::limit_double_stars(&stars_limit_mag, DOUBLE_STAR_TOLERANCE);
	
	const conditions : [TestConditions; 56] = [
	TestConditions{fov: Degrees(60.0).as_radians(), region_size: Degrees(30.0).as_radians(), region_num: 8, time_cap: 30, reach_multiplier: 2.0},
	TestConditions{fov: Degrees(60.0).as_radians(), region_size: Degrees(30.0).as_radians(), region_num: 8, time_cap: 30, reach_multiplier: 1.5},
	TestConditions{fov: Degrees(60.0).as_radians(), region_size: Degrees(30.0).as_radians(), region_num: 8, time_cap: 30, reach_multiplier: 1.3},
	TestConditions{fov: Degrees(60.0).as_radians(), region_size: Degrees(30.0).as_radians(), region_num: 8, time_cap: 30, reach_multiplier: 1.0},
	 
	TestConditions{fov: Degrees(60.0).as_radians(), region_size: Degrees(20.0).as_radians(), region_num: 8, time_cap: 30, reach_multiplier: 2.0},
	TestConditions{fov: Degrees(60.0).as_radians(), region_size: Degrees(20.0).as_radians(), region_num: 8, time_cap: 30, reach_multiplier: 1.5},
	TestConditions{fov: Degrees(60.0).as_radians(), region_size: Degrees(20.0).as_radians(), region_num: 8, time_cap: 30, reach_multiplier: 1.3},
	TestConditions{fov: Degrees(60.0).as_radians(), region_size: Degrees(20.0).as_radians(), region_num: 8, time_cap: 30, reach_multiplier: 1.0}, 
	
	 
	TestConditions{fov: Degrees(50.0).as_radians(), region_size: Degrees(20.0).as_radians(), region_num: 8, time_cap: 30, reach_multiplier: 2.0}, 
	TestConditions{fov: Degrees(50.0).as_radians(), region_size: Degrees(20.0).as_radians(), region_num: 8, time_cap: 30, reach_multiplier: 1.5}, 
	TestConditions{fov: Degrees(50.0).as_radians(), region_size: Degrees(20.0).as_radians(), region_num: 8, time_cap: 30, reach_multiplier: 0.3}, 
	TestConditions{fov: Degrees(50.0).as_radians(), region_size: Degrees(20.0).as_radians(), region_num: 8, time_cap: 30, reach_multiplier: 1.0}, 
	
	TestConditions{fov: Degrees(50.0).as_radians(), region_size: Degrees(15.0).as_radians(), region_num: 8, time_cap: 30, reach_multiplier: 2.0}, 
	TestConditions{fov: Degrees(50.0).as_radians(), region_size: Degrees(15.0).as_radians(), region_num: 8, time_cap: 30, reach_multiplier: 1.5}, 
	TestConditions{fov: Degrees(50.0).as_radians(), region_size: Degrees(15.0).as_radians(), region_num: 8, time_cap: 30, reach_multiplier: 0.3}, 
	TestConditions{fov: Degrees(50.0).as_radians(), region_size: Degrees(15.0).as_radians(), region_num: 8, time_cap: 30, reach_multiplier: 1.0}, 
	
	
	TestConditions{fov: Degrees(45.0).as_radians(), region_size: Degrees(20.0).as_radians(), region_num: 8, time_cap: 40, reach_multiplier: 2.0}, 
	TestConditions{fov: Degrees(45.0).as_radians(), region_size: Degrees(20.0).as_radians(), region_num: 8, time_cap: 40, reach_multiplier: 1.5}, 
	TestConditions{fov: Degrees(45.0).as_radians(), region_size: Degrees(20.0).as_radians(), region_num: 8, time_cap: 40, reach_multiplier: 1.3}, 
	TestConditions{fov: Degrees(45.0).as_radians(), region_size: Degrees(20.0).as_radians(), region_num: 8, time_cap: 40, reach_multiplier: 1.0},
	 
	TestConditions{fov: Degrees(45.0).as_radians(), region_size: Degrees(15.0).as_radians(), region_num: 8, time_cap: 40, reach_multiplier: 2.0}, 
	TestConditions{fov: Degrees(45.0).as_radians(), region_size: Degrees(15.0).as_radians(), region_num: 8, time_cap: 40, reach_multiplier: 1.5}, 
	TestConditions{fov: Degrees(45.0).as_radians(), region_size: Degrees(15.0).as_radians(), region_num: 8, time_cap: 40, reach_multiplier: 1.3}, 
	TestConditions{fov: Degrees(45.0).as_radians(), region_size: Degrees(15.0).as_radians(), region_num: 8, time_cap: 40, reach_multiplier: 1.0},
	 
	 
	TestConditions{fov: Degrees(35.0).as_radians(), region_size: Degrees(20.0).as_radians(), region_num: 8, time_cap: 50, reach_multiplier: 2.0}, 
	TestConditions{fov: Degrees(35.0).as_radians(), region_size: Degrees(20.0).as_radians(), region_num: 8, time_cap: 50, reach_multiplier: 1.5}, 
	TestConditions{fov: Degrees(35.0).as_radians(), region_size: Degrees(20.0).as_radians(), region_num: 8, time_cap: 50, reach_multiplier: 1.3}, 
	TestConditions{fov: Degrees(35.0).as_radians(), region_size: Degrees(20.0).as_radians(), region_num: 8, time_cap: 50, reach_multiplier: 1.0}, 
	
	TestConditions{fov: Degrees(35.0).as_radians(), region_size: Degrees(15.0).as_radians(), region_num: 8, time_cap: 50, reach_multiplier: 2.0}, 
	TestConditions{fov: Degrees(35.0).as_radians(), region_size: Degrees(15.0).as_radians(), region_num: 8, time_cap: 50, reach_multiplier: 1.5}, 
	TestConditions{fov: Degrees(35.0).as_radians(), region_size: Degrees(15.0).as_radians(), region_num: 8, time_cap: 50, reach_multiplier: 1.3}, 
	TestConditions{fov: Degrees(35.0).as_radians(), region_size: Degrees(15.0).as_radians(), region_num: 8, time_cap: 50, reach_multiplier: 1.0}, 
	
	
	TestConditions{fov: Degrees(30.0).as_radians(), region_size: Degrees(15.0).as_radians(), region_num: 8, time_cap: 60, reach_multiplier: 2.0}, 
	TestConditions{fov: Degrees(30.0).as_radians(), region_size: Degrees(15.0).as_radians(), region_num: 8, time_cap: 60, reach_multiplier: 1.5}, 
	TestConditions{fov: Degrees(30.0).as_radians(), region_size: Degrees(15.0).as_radians(), region_num: 8, time_cap: 60, reach_multiplier: 1.3}, 
	TestConditions{fov: Degrees(30.0).as_radians(), region_size: Degrees(15.0).as_radians(), region_num: 8, time_cap: 60, reach_multiplier: 1.0}, 
	
	TestConditions{fov: Degrees(30.0).as_radians(), region_size: Degrees(10.0).as_radians(), region_num: 8, time_cap: 60, reach_multiplier: 2.0}, 
	TestConditions{fov: Degrees(30.0).as_radians(), region_size: Degrees(10.0).as_radians(), region_num: 8, time_cap: 60, reach_multiplier: 1.5}, 
	TestConditions{fov: Degrees(30.0).as_radians(), region_size: Degrees(10.0).as_radians(), region_num: 8, time_cap: 60, reach_multiplier: 1.3}, 
	TestConditions{fov: Degrees(30.0).as_radians(), region_size: Degrees(10.0).as_radians(), region_num: 8, time_cap: 60, reach_multiplier: 1.0}, 
	
	
	TestConditions{fov: Degrees(20.0).as_radians(), region_size: Degrees(10.0).as_radians(), region_num: 8, time_cap: 60, reach_multiplier: 2.0}, 
	TestConditions{fov: Degrees(20.0).as_radians(), region_size: Degrees(10.0).as_radians(), region_num: 8, time_cap: 60, reach_multiplier:  1.5},
	TestConditions{fov: Degrees(20.0).as_radians(), region_size: Degrees(10.0).as_radians(), region_num: 8, time_cap: 60, reach_multiplier:  1.3},
	TestConditions{fov: Degrees(20.0).as_radians(), region_size: Degrees(10.0).as_radians(), region_num: 8, time_cap: 60, reach_multiplier: 1.0}, 
	
	TestConditions{fov: Degrees(20.0).as_radians(), region_size: Degrees(5.0).as_radians(), region_num: 8, time_cap: 60, reach_multiplier: 2.0}, 
	TestConditions{fov: Degrees(20.0).as_radians(), region_size: Degrees(5.0).as_radians(), region_num: 8, time_cap: 60, reach_multiplier:  1.5},
	TestConditions{fov: Degrees(20.0).as_radians(), region_size: Degrees(5.0).as_radians(), region_num: 8, time_cap: 60, reach_multiplier:  1.3},
	TestConditions{fov: Degrees(20.0).as_radians(), region_size: Degrees(5.0).as_radians(), region_num: 8, time_cap: 60, reach_multiplier: 1.0}, 
	
	TestConditions{fov: Degrees(15.0).as_radians(), region_size: Degrees(6.0).as_radians(), region_num: 8, time_cap: 60, reach_multiplier: 2.0},
	TestConditions{fov: Degrees(15.0).as_radians(), region_size: Degrees(6.0).as_radians(), region_num: 8, time_cap: 60, reach_multiplier: 1.3},
	TestConditions{fov: Degrees(15.0).as_radians(), region_size: Degrees(6.0).as_radians(), region_num: 8, time_cap: 60, reach_multiplier: 1.0},
	TestConditions{fov: Degrees(15.0).as_radians(), region_size: Degrees(6.0).as_radians(), region_num: 8, time_cap: 60, reach_multiplier: 1.0},
	TestConditions{fov: Degrees(15.0).as_radians(), region_size: Degrees(2.0).as_radians(), region_num: 8, time_cap: 60, reach_multiplier: 2.0},
	TestConditions{fov: Degrees(15.0).as_radians(), region_size: Degrees(2.0).as_radians(), region_num: 8, time_cap: 60, reach_multiplier: 1.3},
	TestConditions{fov: Degrees(15.0).as_radians(), region_size: Degrees(2.0).as_radians(), region_num: 8, time_cap: 60, reach_multiplier: 1.0},
	TestConditions{fov: Degrees(15.0).as_radians(), region_size: Degrees(2.0).as_radians(), region_num: 8, time_cap: 60, reach_multiplier: 1.0}];
	
	for condition in conditions
	{
		let fov = condition.fov;
		let region_size = condition.region_size;
		let region_num = condition.region_num;
		let stars_limit_reg    = DatabaseGenerator::limit_regions(&stars_limit_double, region_size, region_num);


	// The official database is based off static arrays to save memory and remove the heap.
	// When simulating the database, these variables must exist while the database exists.
	// Choose between the pyramid database (old) and the regional database (new) to get a comparison.

	// let gen : DatabaseGenerator = DatabaseGenerator::gen_database(&stars_limit_reg, FOV, NUM_BINS);
	// let mut database = gen.get_database_pyramid();
	
	let gen : DatabaseGenerator = DatabaseGenerator::gen_database_regions(&stars_limit_reg, fov, fov * condition.reach_multiplier, NUM_BINS);
	// let database = gen.get_database_pyramid();
	let database = gen.get_database_regional();
	
	// let gen : DatabaseGenerator = DatabaseGenerator::gen_database(&stars_limit_reg, FOV, NUM_BINS);



	// let mut database_iterator = PyramidIterator::new(&database);
	// let mut database_iterator = RegionalIterator::new(&database);
	let mut database_iterator = BoundedDeclinationIterator::new(&database, condition.reach_multiplier);
	// let mut database_iterator = BoundedEquatorialIterator::new(&database, condition.reach_multiplier);
	// let mut database_iterator = RegionalCrunchIterator::new(&database, condition.reach_multiplier);



//###############################################################################################//
//
//							---	Tracking ---
//
//
//
//###############################################################################################//

	let observation_points = Distribute::angle_to_points(SAMPLE_FOV);
	let observation = Distribute::fibonacci_latice(observation_points);

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
	let mut deviation_time_pyramid : Vec<u128> = Vec::new();



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
		while i < stars.len()
		{
			if  stars[i].mag < MAGNITUDE
			  && stars[i].pos.angle_distance(center) < fov / 2.0
			{
				let mut position = stars[i].pos;
				let rotated = rotation.to_image(SpaceWorld(position.to_vector3())).0;
				observable.push(rotated.to_equatorial());
			}
			i += 1;
		}


		//
		// Actual Algorithm
		let timer : std::time::Instant = std::time::Instant::now();

		let constellation : Constellation = Constellation::find::<TrackingConstsTest>(
			&observable, &mut database_iterator,
			&mut StarTriangleIterator::<{TrackingConstsTest::PAIRS_MAX}>::new(),
			&mut StarPyramid(0,0,0,0), &mut Specularity::Ignore);

		let time = timer.elapsed();
		match print_result(test_num, time, constellation, center, rotation, observable, TIME_GOOD)
		{
			Result::Pyramid      =>
			{
				num_pyramid        +=1; 
				avg_time_pyramid   += time.as_millis();
				deviation_time_pyramid.push(time.as_millis());
			}
			Result::Triangle     =>{num_triangle       +=1; avg_time_triangle += time.as_millis();}
			Result::Error        =>{num_error          +=1; avg_time_error    += time.as_millis();}
			Result::ErrorPyramid =>{num_error_pyramid  +=1; avg_time_error    += time.as_millis();}
			Result::ErrorTriangle=>{num_error_triangle +=1; avg_time_error    += time.as_millis();}
			Result::ErrorTime    =>{ num_error_time +=1; }
		}
		println!("examining {} fov: {}", fov.to_degrees(), test_num);
		test_num+=1;
	}
	
	let mut standard_dev = 0.0;
	for e in deviation_time_pyramid
	{
		let diff = e as Decimal - avg_time_pyramid as Decimal / test_num as Decimal;
		standard_dev = (diff.abs().powf(2.0) / test_num as Decimal).sqrt();
	}
	
	file_str.push(format!("{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},",
		"BoundedEquatorialIterator",
		fov.to_degrees(),
		SAMPLE_FOV.to_degrees(),
		MAGNITUDE,
		DOUBLE_STAR_TOLERANCE.to_degrees(),
		
		region_size.to_degrees(),
		region_num,
		condition.reach_multiplier,
		test_num,
		
		num_pyramid,
		num_triangle,
		num_error,
		num_error_pyramid,
		num_error_triangle,
		
		avg_time_pyramid  as Decimal / test_num as Decimal,
		avg_time_triangle as Decimal / test_num as Decimal,
		avg_time_error    as Decimal / test_num as Decimal,
		standard_dev      as Decimal / test_num as Decimal,
	));
	
	Io::write_file(file_path, &file_str);
	}
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








fn print_result ( test_num: usize, time: std::time::Duration, constellation : Constellation, location: Equatorial, rotation: ExtrinsicParameters, stars: Vec<Equatorial>, time_good: u128 ) -> Result
{
	let result : Result;
	
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
			if v_1 && v_2 && v_3 && v_4  { result = Result::Pyramid; }
			else                         { result = Result::ErrorPyramid; }
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

			if v_1 && v_2 && v_3   { result = Result::Triangle;     }
			else                   {result = Result::ErrorTriangle; }
		}
		Constellation::None => 				// No stars identified.
		{
			result = Result::Error;
		}
	}
	
	
	if time_good < time.as_millis() && result != Result::Error && result != Result::ErrorPyramid && result != Result::ErrorTriangle
	{
		return Result::ErrorTime;
	}
	
	
	return result;
}
