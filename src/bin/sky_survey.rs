//! In the worst case scenario, there may be a hole in the cluster of stars.
//! By finding at specific scenarios what the 
//!
extern crate star_tracker;

use star_tracker::config::NixConstsStruct;
use star_tracker::config::NixConsts;

use star_tracker::util::units::Equatorial;
// use star_tracker::util::units::Radians;
use star_tracker::util::aliases::Decimal;
use star_tracker::util::aliases::M_PI;

use std::fmt;

// use star_tracker::config::TrackingModeConstructConstsStruct;
// use star_tracker::config::TrackingModeConstructConsts;
// use star_tracker::config::TrackingModeConstsStruct;
// use star_tracker::config::TrackingModeConsts;

use star_tracker::nix::Io;
use star_tracker::nix::Star;


fn main (  )
{
	println!("\n\n\nRUNNING");
	
	let mut rdr = Io::get_csv (
		NixConstsStruct::HYG_DATABASE_PATH,
		NixConstsStruct::HYG_DATABASE_FILE,
		NixConstsStruct::HYG_DATABASE_URL );

	let comparison_points : Vec<Equatorial> = Equatorial::evenly_distribute(1000);
	let mut database_stars : Vec<Star> = Vec::with_capacity(1000);
	let mut magnitude_stars : Vec<Equatorial> = Vec::with_capacity(1000);

	const MAGNITUDE_RANGE : LinearRange = LinearRange{min: -1.0, max: 7.0, num: 35};
	const FOV_RANGE : LinearRange		= LinearRange{min: 0.0 * M_PI / 180.0, max: 50.0 * M_PI / 180.0, num: 50};
	
	let mut smallest_num_stars : Vec<Vec<u32>> = vec![vec![0; MAGNITUDE_RANGE.num]; FOV_RANGE.num];
	let mut percent_above_3 : Vec<Vec<Decimal>> = vec![vec![0.0; MAGNITUDE_RANGE.num]; FOV_RANGE.num];
	let mut percent_above_4 : Vec<Vec<Decimal>> = vec![vec![0.0; MAGNITUDE_RANGE.num]; FOV_RANGE.num];
	let mut percent_above_5 : Vec<Vec<Decimal>> = vec![vec![0.0; MAGNITUDE_RANGE.num]; FOV_RANGE.num];
	
	
	
	
	let iter = rdr.deserialize();
	for record in iter
	{
		let record : Star = record.expect("Could not decode.");
		if record.mag < MAGNITUDE_RANGE.max
		{
			database_stars.push(record);
		}
	}	
	
	println!("read from database");
	
	// Save cursor position.
	// print!("\x1b7");
	
	// Loop through database and find every element bellow a certain magnitude, add to list.
	for m in 0..MAGNITUDE_RANGE.num
	{
		for e in &database_stars
		{
			if e.mag < MAGNITUDE_RANGE.get(m)
			{
				magnitude_stars.push(e.pos);
			}
		}
		
		find_least_stars(&mut smallest_num_stars, m, FOV_RANGE, &comparison_points, &magnitude_stars);
		find_percent_stars(&mut percent_above_3, m, FOV_RANGE, &comparison_points, &magnitude_stars, 3);
		find_percent_stars(&mut percent_above_4, m, FOV_RANGE, &comparison_points, &magnitude_stars, 4);
		find_percent_stars(&mut percent_above_5, m, FOV_RANGE, &comparison_points, &magnitude_stars, 5);
		
		// print!("\x1B[2J \x1B[1;1H");
		print!("\x1B[2J \x1B[3J \x1B[H");
		// Move cursor back to top.
		// print!("\x1b8");
		// print!("\x1b0J");

		let header_color = "\x1B[1;31m";
		let norm_color = "\x1B[0m";
		println!("{}magnitude: {} out of {}", header_color, MAGNITUDE_RANGE.get(m), MAGNITUDE_RANGE.max);
		println!("\nLeast number of stars");
		println!("{}", norm_color);
		print_data(&smallest_num_stars, MAGNITUDE_RANGE, FOV_RANGE);
		
		println!("\n{}Percentage above 3 stars", header_color);
		println!("{}", norm_color);
		print_data(&percent_above_3, MAGNITUDE_RANGE, FOV_RANGE);
		
		println!("\n{}Percentage above 4 stars", header_color);
		println!("{}", norm_color);
		print_data(&percent_above_4, MAGNITUDE_RANGE, FOV_RANGE);
		
		println!("\n{}Percentage above 5 stars", header_color);
		println!("{}", norm_color);
		print_data(&percent_above_5, MAGNITUDE_RANGE, FOV_RANGE);


		magnitude_stars.clear();
	}
	
	
	

}




/// Finds the smallest number of stars that can be seen with a specific field of view.
fn find_least_stars ( num_stars: &mut Vec<Vec<u32>>, magnitude_idx: usize, fov_range : LinearRange, points: &[Equatorial], stars: &Vec<Equatorial> )
{
	for f in 0..fov_range.num
	{
		let mut smallest = 1000000;
		for even in points
		{
			let mut current = 0;
			for actual in stars
			{
				if even.angle_distance(*actual).0 < fov_range.get(f) / 2.0
				{
					current+=1;
				}
			}
			
			if current < smallest
			{
				smallest = current;
			}
		}
		num_stars[f][magnitude_idx] = smallest;
	}	
}


fn find_percent_stars ( percent: &mut Vec<Vec<Decimal>>, magnitude_idx: usize, fov_range : LinearRange, points: &[Equatorial], stars: &Vec<Equatorial>, num_above: usize )
{
	for f in 0..fov_range.num
	{
		let mut paired_count = 0;
		for even in points
		{
			let mut siblings = 0;
			for actual in stars
			{
				if even.angle_distance(*actual).0 < fov_range.get(f) / 2.0
				{
					siblings += 1;
				}
			}
			
			if siblings > num_above
			{
				paired_count += 1;
			}
		}
		percent[f][magnitude_idx] = paired_count as Decimal / points.len() as Decimal * 100.0;
	}	
}




fn print_data <T> ( values: &Vec<Vec<T>>, mag_range : LinearRange, fov_range: LinearRange) where T: fmt::Display
{
	// Construct headers
	print!("Field Of View Rad\t");
	print!("Field Of View Deg\t");
	for m in 0..mag_range.num
	{
		print!("Mag @ {:.2} \t", mag_range.get(m));
	}
	println!("");
	
	// Main
	for cc in 0..values.len()
	{
		print!("{:0.2}\t", fov_range.get(cc));
		print!("{:0.2}\t", fov_range.get(cc) * 180.0 / M_PI);
		for rr in 0..values[0].len()
		{
			print!("{:0.0}\t", values[cc][rr]);
		}
		println!("");
	}
}





// An iteratorable set of numbers.
struct LinearRange 
{
	pub max : Decimal, // The maximum value to go up to.
	pub min : Decimal, // The minimum value.
	pub num : usize, // The number of steps.

}

impl LinearRange
{
	// Returns the next value in the sequence
	fn get ( &self, iteration: usize ) -> Decimal
	{
		iteration as Decimal * (self.max - self.min) / self.num as Decimal
	}
}