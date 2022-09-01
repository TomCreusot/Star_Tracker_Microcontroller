#![allow(unused_must_use)]
extern crate star_tracker;
extern crate image;
extern crate rand;

use rand::prelude::*;

use star_tracker::config::TrackingModeConstsStruct;
use star_tracker::config::TrackingModeConsts;
use star_tracker::config::AttitudeDeterminationConstsStruct;
// use star_tracker::config::AttitudeDeterminationConsts;
use star_tracker::config::NixConstsStruct;
use star_tracker::config::NixConsts;


use star_tracker::util::list::List;
use star_tracker::util::aliases::Decimal;
use star_tracker::util::aliases::Byte;
use star_tracker::util::aliases::UInt;

// use star_tracker::util::units::Quaternion;
// use star_tracker::util::units::AngleAxis;
use star_tracker::util::units::Vector3;
use star_tracker::util::units::Vector2;
use star_tracker::util::units::Degrees;
use star_tracker::util::units::Radians;
use star_tracker::util::units::Equatorial;
use star_tracker::util::units::Pixel;

use star_tracker::nix::NixImage;
use star_tracker::nix::Io;
use star_tracker::nix::Star;

use star_tracker::image_processing::Blob;
use star_tracker::image_processing::Image;

use star_tracker::tracking_mode::Constellation;
use star_tracker::tracking_mode::StarTriangleIterator;
// use star_tracker::tracking_mode::StarPyramid;
// use star_tracker::tracking_mode::StarTriangle;
use star_tracker::tracking_mode::Match;
use star_tracker::tracking_mode::StarPair;
use star_tracker::tracking_mode::database::RegionalDatabase;
// use star_tracker::tracking_mode::database::KVector;
// use star_tracker::tracking_mode::database::array_database;
// use star_tracker::tracking_mode::database::StarDatabaseElement;
use star_tracker::tracking_mode::database::DatabaseGenerator;

use star_tracker::projection::IntrinsicParameters;
use star_tracker::projection::ExtrinsicParameters;
use star_tracker::projection::SpaceWorld;
use star_tracker::projection::SpaceImage;


use star_tracker::attitude_determination::Quest;
use star_tracker::attitude_determination::AttitudeDetermination;

const FOV           : Radians = Degrees(40.0).as_radians();
const MAGNITUDE_MIN : Decimal = -10.0;
const MAGNITUDE_MAX	: Decimal = 4.0;
const BINS_NUM      : usize = 3000;

const BLOB_SIZE		: usize = 100;

const IMAGE_SIZE	: Pixel = Pixel{x: 1000, y: 1000};

fn main ( )
{

//###############################################################################################//
//							---	Setup ---
//###############################################################################################//
// Don't worry about this, it is only used in this test.

	// Creates a set of tests by sampling equally spaced points on a unit sphere.
	let points : usize = Equatorial::evenly_distribute_points(FOV / 2.0);
	let centers : Vec<Equatorial> =	Equatorial::evenly_distribute(points);

	// Creates a camera projection matrix with the given field of view.
	// The from_fov() method is only relevant for creating simulation images as lenses are complex.
	let intrinsic:IntrinsicParameters=IntrinsicParameters::from_fov(FOV, IMAGE_SIZE.y as Decimal);



	println!("Reading Star CSV Database\t\t...");
	let mut stars = get_stars();
	stars.sort();
	let stars_magnitude = DatabaseGenerator::limit_magnitude(&stars, MAGNITUDE_MIN, MAGNITUDE_MAX);
	let stars_region    = DatabaseGenerator::limit_regions(&stars_magnitude, FOV / 2.0, 5);
	println!("   * Stars Without Reduction:        {}", stars.size());
	println!("   * Stars With Magnitude Reduction: {}", stars_magnitude.size());
	println!("   * Stars With Region Reduction:    {}", stars_region.size());

	// Creates a database which is not the main database.
	// This makes the tests easier.
	println!("\nCreating Native Database \t\t...");
	let gen : DatabaseGenerator = DatabaseGenerator::gen_database(&stars_region, FOV, BINS_NUM);

	let mut database : RegionalDatabase = gen.get_database_regional();


	println!("   * k_vector size:                  {}", database.k_vector.size());
	println!("   * pairs size:                     {}", database.pairs.size());
	println!("   * catalogue size:                 {}", database.catalogue.size());



//###############################################################################################//
//							---	Test Runner ---
//###############################################################################################//
// Runs all tests.
	for i in 0..centers.len()
	{
		// The direction the camera is looking.
		let center : Equatorial = centers[i];
		println!("\n\n\n\nLoop: {},\t position ra: {}, dec: {}",
			i, center.ra.to_degrees(), center.dec.to_degrees());

		// The input sample image.
		let mut image : NixImage = NixImage::new(IMAGE_SIZE);

		// The center and up direction of the camera.
		let up = random_direction();

		// Creates a camera rotation matrix which looks at a target and has an associated up direction.
		let extrinsic : ExtrinsicParameters = ExtrinsicParameters::look_at(center, up);


		println!("Creating Image             \t\t...");
		let mut stars_in_image = 0;
		for star in &stars_magnitude
		{
			let point = star.pos;
			{
				if 	image.draw_star(SpaceWorld(point.to_vector3()),
					MAGNITUDE_MAX - star.mag, [100,100,255], intrinsic, extrinsic)
				{
					stars_in_image += 1;
				}
			}
		}
		println!("   * Stars In Image:                 {}", stars_in_image);
		let display_image = image.clone();
		display_image.img_rgb.save("results/demo/demo.png").expect("Could not save");


//###############################################################################################//
//							---	Image Processing ---
//###############################################################################################//
		println!("Performing Image Processing\t\t...");
		let mut histogram : [UInt; 255] = [0; 255];
		image.histogram(&mut histogram);

		let threshold_percent = 0.999;
		let threshold : Byte = image.percent_threshold(threshold_percent, &histogram);

		let mut blobs : Vec<Blob> = Vec::new();
		Blob::find_blobs::<BLOB_SIZE>(threshold, &mut image, &mut blobs);

		let mut points_vec2: Vec<Vector2> = Vec::new();
		Blob::to_vector2(&blobs, &mut points_vec2);

//###############################################################################################//
//							---	Projection ---
//###############################################################################################//
		let mut points : Vec<Equatorial> = Vec::new();
		for i in 0..points_vec2.size()
		{
			points.push(intrinsic.from_image(SpaceImage(points_vec2[i])).0.to_equatorial());
		}



//###############################################################################################//
//							---	Tracking Mode ---
//###############################################################################################//
		// Attempts to create a star pyramid.
		println!("Finding Constellation      \t\t...");
		let constellation : Constellation = Constellation::find::<TrackingModeConstsStruct>(
			&points,
			&mut database,
			&mut StarTriangleIterator::<{TrackingModeConstsStruct::PAIRS_MAX}>::new(),
			&mut star_tracker::tracking_mode::StarPyramid(0,0,0,0),
			&mut star_tracker::tracking_mode::Specularity::Ignore);

		match constellation
		{
			Constellation::Pyramid(_stars) =>	// Success (4 stars identified or more)
			{
				println!("\tFound Constellation Pyramid");
			}
			Constellation::Triangle(_stars) =>	// Mild Success (3 stars identified)
			{
				println!("\tFound Constellation Triangle");
			}
			Constellation::None => 				// No stars identified.
			{
				println!("\tFAILED... could not find matching constellation.");
				// return;
			}
		}


		// Combines the star vectors into a quaternion.
		println!("Rotation                   \t\t...");
		let matched_stars : Vec<Match<Vector3>> = convert_constellation(constellation);
		let mut rotation = Quest::estimate::<AttitudeDeterminationConstsStruct>(&matched_stars);

		let angle = rotation.to_angle_axis().axis.angle_distance(matched_stars[0].output).to_degrees();
		if angle < Degrees(90.0)
		{
			rotation.w = -rotation.w;
			rotation.y = -rotation.x;
			rotation.x = -rotation.y;
			rotation.z = -rotation.z;
		}

		// if rotation != orientation
		// {
		// 	println!("\n\n\nERROR!*!*!!*!*!*!!*!**!!*!**!*!*!*!*!*!*!!*!*!*!*!*!*!*!*!*!*!*!*!*!*\n\n\n\n\n");
		// }

		// println!("\texpected: {:?}", orientation.to_angle_axis());
		// println!("\tactual:   {:?}", rotation.to_angle_axis());
		// println!("\texpected: {:?}", orientation);
		println!("\tactual:   {:?}", rotation);
		if matched_stars.len() != 0
		{
			println!("*** separation:   {:?}", rotation.to_angle_axis().axis.angle_distance(matched_stars[0].output).to_degrees());
		}

		println!("\n\n\n\n\n\n\n");
	}
}

































//###############################################################################################//
//
//										Required
//
//###############################################################################################//


/// Gets all the stars from the database.
pub fn get_stars ( ) -> Vec<Star>
{

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
	return stars;
}


/// Converts a constellation into an Vec<Match<Vector3>>
pub fn convert_constellation ( constellation : Constellation ) -> Vec<Match<Vector3>>
{
	let mut vec : Vec<Match<Vector3>> = Vec::new();

	match constellation
	{
		Constellation::Pyramid(stars) =>
		{
			vec.push_back(Match{input: stars.input.0.to_vector3(), output: stars.output.0.to_vector3(), weight: stars.weight } );
			vec.push_back(Match{input: stars.input.1.to_vector3(), output: stars.output.1.to_vector3(), weight: stars.weight } );
			vec.push_back(Match{input: stars.input.2.to_vector3(), output: stars.output.2.to_vector3(), weight: stars.weight } );
			vec.push_back(Match{input: stars.input.3.to_vector3(), output: stars.output.3.to_vector3(), weight: stars.weight } );
		}
		Constellation::Triangle(stars) =>
		{
			vec.push_back(Match{input: stars.input.0.to_vector3(), output: stars.output.0.to_vector3(), weight: stars.weight } );
			vec.push_back(Match{input: stars.input.1.to_vector3(), output: stars.output.1.to_vector3(), weight: stars.weight } );
			vec.push_back(Match{input: stars.input.2.to_vector3(), output: stars.output.2.to_vector3(), weight: stars.weight } );
		}
		Constellation::None => { }
	}
	return vec;
}

//###############################################################################################//
//
//										Error Creation
//
//###############################################################################################//

/// Generates a random direction.
pub fn random_direction ( ) -> Equatorial
{
	let mut rng = rand::thread_rng();
	let axis = Equatorial{
		ra:  Degrees(rng.gen::<Decimal>() * 360.0).to_radians(),
		dec: Degrees(rng.gen::<Decimal>() * 180.0 - 90.0).to_radians()};

	return axis;
}

/// Random num.
pub fn rand_num ( ) -> Decimal
{
	let mut rng = rand::thread_rng();
	return rng.gen::<Decimal>();
}
