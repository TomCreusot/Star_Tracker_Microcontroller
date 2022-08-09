#![allow(unused_must_use)]
extern crate star_tracker;
extern crate image;
extern crate rand;

use rand::prelude::*;

use star_tracker::config::TrackingModeConstsStruct;
use star_tracker::config::TrackingModeConsts;
use star_tracker::config::AttitudeDeterminationConstsStruct;


use star_tracker::util::list::List;
use star_tracker::util::aliases::Decimal;

use star_tracker::util::units::Quaternion;
use star_tracker::util::units::AngleAxis;
use star_tracker::util::units::Vector3;
use star_tracker::util::units::Degrees;
use star_tracker::util::units::Radians;
use star_tracker::util::units::Equatorial;


use star_tracker::tracking_mode::Constellation;
use star_tracker::tracking_mode::StarTriangleIterator;
// use star_tracker::tracking_mode::StarPyramid;
// use star_tracker::tracking_mode::StarTriangle;
use star_tracker::tracking_mode::Match;
use star_tracker::tracking_mode::database::PyramidDatabase;
use star_tracker::tracking_mode::database::array_database;


use star_tracker::attitude_determination::Quest;
use star_tracker::attitude_determination::AttitudeDetermination;



fn main ( )
{
	println!("\n\nWARNING:\nTHIS CURRENTLY WILL SKIP THE IMAGE PROCESSING AND BLOB DETECTION.");

	// Creates a set of tests by sampling equaly spaced points on a unit sphere.
	let mut centers : [Equatorial; 100] = [Equatorial{ra: Radians(0.0), dec: Radians(0.0)};100];
	Equatorial::evenly_distribute(&mut centers);


	// return;
	// Runs each test.
	for i in 0..centers.len()
	{
		let center = centers[i];

		// Chooses the orientation of the camera relative to the spacecraft.
		// It is random in this case for testing.
		// let mut orientation = random_orientation();
		let orientation = AngleAxis{
			angle: random_angle(Degrees(0.0).to_radians(), Degrees(360.0).to_radians()),
			axis: center.to_vector3()}.to_quaternion();



		println!("Loop: {},\t position ra: {:?}, dec: {:?}",
											i, center.ra.to_degrees(), center.dec.to_degrees());
		println!("Reading array_database File\t\t...", );

		// Finds all stars in the database within the sample are of the test.
		let mut input = get_stars(center);


		println!("\tStars In Image: {}", input.len());

		// Simulates an actual scenario by providing corruption to the image.
		println!("Corrupting Image           \t\t...");
		dither(&mut input, 0.00001);			// Randomises the positions of the stars slightly.
		hide_stars(&mut input, 1);				// Pretends some stars were not identified.
		false_stars(&mut input, center, 2);		// Pretends some stars were identified that dont exist.
		rotate(&mut input, orientation);		// Rotates the stars to fit


		println!("\tStars: {}", input.len());

		//////*************************************************************************************
		// Distort equatorial coordinates to a 2d image.
		let mut center = Vector3{x: 4.0, y: 3.0, z: 2.0};
		center.normalize();
		// println!("{:?}", Distortion::equatorial_to_local_image(center, center));


		// Undistort the image into equatorial.
		//
		//
		//
		//////*************************************************************************************


		// Attempts to create a star pyramid.
		println!("Finding Constellation      \t\t...");
		let constellation : Constellation = Constellation::find::<TrackingModeConstsStruct>(
			&input,
			&PyramidDatabase::new(),
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

		if rotation != orientation
		{
			println!("\n\n\nERROR!*!*!!*!*!*!!*!**!!*!**!*!*!*!*!*!*!!*!*!*!*!*!*!*!*!*!*!*!*!*!*\n\n\n\n\n");
		}

		println!("\texpected: {:?}", orientation.to_angle_axis());
		println!("\tactual:   {:?}", rotation.to_angle_axis());
		println!("\texpected: {:?}", orientation);
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


/// Finds all stars in the region from the star_tracker::tracking_mode::database::array_database::CATALOGUE_DATABASE database.
///
/// # Arguments
/// * `pos` - The position of the center of the camera, anything outside the database FOV is excluded.
/// # Returns
/// Any stars in the database surrounding "pos".
pub fn get_stars ( pos : Equatorial ) -> Vec<Equatorial>
{
	let mut stars : Vec<Equatorial> = Vec::new();
	for i in 0..array_database::CATALOGUE_DATABASE.len()
	{
		let star = array_database::CATALOGUE_DATABASE[i];
		if star.angle_distance(pos) < array_database::FOV / 2.0
		{
			stars.push(star);
		}
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




// / Creates an image from the given stars.
// / # Arguments
// / * `stars` - The stars to draw.
// / # Returns
// / An image with the stars inserted.
// pub fn create_img ( center: Quaternion, stars: Vec<Equatorial> ) -> NixImage
// {

// }


//###############################################################################################//
//
//										Error Creation
//
//###############################################################################################//



/// Creates random noise to the position of each star.
///
/// # Arguments
/// * `disrupt` - The stars to create noise.
/// * `amount`  - 0 is not random, 1 is completely random.
pub fn dither ( disrupt: &mut dyn List<Equatorial>, amount: Decimal )
{
	let mut rng = rand::thread_rng();
	for i in 0..disrupt.size()
	{
		let mut cart = disrupt.get(i).to_vector3();
		cart.x += rng.gen::<Decimal>() * amount - amount / 2.0;
		cart.y += rng.gen::<Decimal>() * amount - amount / 2.0;
		cart.z += rng.gen::<Decimal>() * amount - amount / 2.0;
		cart.normalize();
		disrupt.set(i, cart.to_equatorial());
	}
}



/// Generates a random orientation to rotate the stars through.
pub fn random_orientation ( ) -> Quaternion
{
	let mut rng = rand::thread_rng();
	let angle = Degrees(rng.gen::<Decimal>() * 360.0).to_radians();
	let mut axis = Vector3{
		x: rng.gen::<Decimal>() - 0.5,
		y: rng.gen::<Decimal>() - 0.5,
		z: rng.gen::<Decimal>() - 0.5 };

	axis.normalize();

	let angle_axis = AngleAxis{angle: angle, axis: axis};
	return angle_axis.to_quaternion();
}


/// Generates a random angle between start and end.
pub fn random_angle ( start: Radians, end: Radians ) -> Radians
{
	let mut rng = rand::thread_rng();
	return Radians(rng.gen::<Decimal>()) * (start - end) + start;
}



/// Rotates all points by a provided quaternion.
///
/// # Arguments
/// * `rotate`   - The stars to rotate.
/// * `rotation` - The rotation to apply to each point.
pub fn rotate ( rotate: &mut dyn List<Equatorial>, rotation : Quaternion )
{
	for i in 0..rotate.size()
	{
		let mut cart = rotate.get(i).to_vector3();
		cart = rotation.rotate_point(cart);
		rotate.set(i, cart.to_equatorial());
	}
}



/// Randomly creates false stars to mislead the program.
/// This simulates noise, a hot pixel or a celestial object which should not be in the frame.
///
/// # Arguments
/// * `add` - The list to add false stars to.
/// * `pos` - The center of the image.
/// * `num` - The number of stars to manufacture.
pub fn false_stars ( add: &mut dyn List<Equatorial>, pos : Equatorial, num : u32 )
{
	let mut rng = rand::thread_rng();
	for _i in 0..num
	{
		let mut star;
		loop
		{
			star = Equatorial{
			   ra:  Degrees(rng.gen::<Decimal>() * 360.0).to_radians(),
			   dec: Degrees(rng.gen::<Decimal>() * 180.0 - 90.0).to_radians()};

			if pos.angle_distance(star) < array_database::FOV / 2.0
			{
				add.push_back(star);
				break;
			}
		}
	}
}


/// Randomly removes stars from the image.
/// The may be because of a stuck pixel or lack of sensitivity...
///
/// # Arguments
/// * `remove` - The list to remove stars from.
/// * `num`    - The number of stars to remove.
pub fn hide_stars ( remove: &mut Vec<Equatorial>, num: u32 )
{
	let mut rng = rand::thread_rng();
	for _i in 0..num
	{
		remove.remove(rng.gen_range(0..(remove.len() - 1)));
	}
}
