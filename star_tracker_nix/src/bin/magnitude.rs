//! This is used to calculate the minimum observable magnitude of each image in the samples folder.
//! It will use the corr.fits file to figure out the magnitudes.
//!

extern crate star_tracker_lib;
extern crate star_tracker_nix;

use star_tracker_lib::util::aliases::Decimal;
use star_tracker_lib::util::units::Radians;
use star_tracker_lib::util::units::Degrees;

use star_tracker_nix::io::Star;
use star_tracker_nix::io::Io;
use star_tracker_nix::tracking_mode::DatabaseGenerator;


pub fn main ( )
{
	println!(r#"
	
	
				===== Magnitude =====
This is used to calculate the minimum observable magnitude of each image in the samples folder.
It will use the corr.fits file to figure out the magnitudes.

	  
	"#);
	
	let exclusive_folders: Vec<String> = std::env::args().collect();

	// To reduce size of database.
	const MAGNITUDE_MIN: Decimal = -20.0;
	const MAGNITUDE_MAX: Decimal = 7.0;
	const DOUBLE_STAR_TOLERANCE: Radians = Degrees(0.01).as_radians();

	println!("Reading Database");
	println!("\t* Reading database.");
	print!("\t* ");
	let mut stars : Vec<Star> = Io::get_csv_database();
		
	println!("\t* Reducing Database.");
	stars.sort();
	let stars_limit_mag    = DatabaseGenerator::limit_magnitude (&stars, MAGNITUDE_MIN, MAGNITUDE_MAX);
	let stars_limit_double = 
		DatabaseGenerator::limit_double_stars(&stars_limit_mag, DOUBLE_STAR_TOLERANCE);

	let samples = star_tracker_nix::io::Sample::load_samples();
	
	println!("\t* Read Database");

	println!("{:90} | Magnitude Dullest | Magnitude Second Dullest", "File");
	for sample in samples
	{
		// Allows you to choose the folder images.
		let mut is_exclusive = false;
		for i in 1..exclusive_folders.len()
		{
			is_exclusive |= sample.dir.contains(&exclusive_folders[i]);
		}
		if !is_exclusive { continue; }


		if let Some(cor_values) = sample.get_corr()
		{
			let mut set = false;
			let mut dullest = 0;
			let mut dullest_second = 0;
			for i_c in 0..cor_values.len()
			{
				let point = cor_values[i_c].real_eq;
				
				for i_d in 0..stars_limit_double.len()
				{
					if point.angle_distance(stars_limit_double[i_d].pos) < DOUBLE_STAR_TOLERANCE
					{
						if !set || stars_limit_double[dullest].mag < stars_limit_double[i_d].mag
						{
							set = true;
							dullest_second = dullest;
							dullest = i_d;
						}
					}
				}
			}

			if set
			{
				println!("{:90}   {}\t\t {}", 
					sample.dir, stars_limit_mag[dullest].mag, stars_limit_mag[dullest_second].mag);
			}
			else
			{
				println!("ERROR on {}", sample.dir);
			}
		}
		println!("");
	}
}