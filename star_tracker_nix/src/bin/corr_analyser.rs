#![allow(unused_imports)]
extern crate fitsio;
extern crate opencv;
extern crate star_tracker_nix;

use fitsio::FitsFile;
use fitsio::tables::Column;

use opencv::imgcodecs::imread;
use opencv::imgcodecs::ImreadModes;

use opencv::core::Mat;
use opencv::core::Size;
use opencv::core::Scalar;
use opencv::core::Point;
use opencv::imgproc::circle;
use opencv::imgproc::line;
use opencv::imgproc::put_text;


use opencv::highgui::wait_key;
use opencv::highgui::imshow;


pub fn main ( )
{
	
	let mut field_x : Vec<f64> = Vec::new();
	let mut field_y : Vec<f64> = Vec::new();
	let mut index_x : Vec<f64> = Vec::new();
	let mut index_y : Vec<f64> = Vec::new();

	let samples = star_tracker_nix::io::Io::iterate_samples();
	
	for sample in samples
	{
		let dir = sample.dir;
		let cor_file = sample.file_cor;
		let img_file = sample.file_img;
		
		let f = FitsFile::open(cor_file.clone());
		if let Ok(mut fits) = f
		{
			println!("Reading: {:?} \t\t\t {} {}", dir, cor_file, img_file);
			
			if let Ok(hdu) = fits.hdu(1)
			{
				field_y.clear();
				field_x.clear();
				index_x.clear();
				index_y.clear();
				field_x = hdu.read_col(&mut fits, "field_x").expect("");
				field_y = hdu.read_col(&mut fits, "field_y").expect("");
				index_x = hdu.read_col(&mut fits, "index_x").expect("");
				index_y = hdu.read_col(&mut fits, "index_y").expect("");
			}
		
			let img_org = imread( &img_file, ImreadModes::IMREAD_COLOR as i32 ).unwrap();
			let mut img_err = imread( &img_file, ImreadModes::IMREAD_COLOR as i32 ).unwrap();
			for i in 0..field_x.len()
			{
				let circle_thickness = 1;
				let circle_radius    = 1;
				let color_real = Scalar::new(0.0,   0.0, 100.0, 0.0); // Red color (BGR format)
				let color_obs  = Scalar::new(0.0, 100.0, 0.0,   0.0); // Red color (BGR format)
				let color_line = Scalar::new(0.0, 100.0, 100.0, 0.0); // Red color (BGR format)
		
				let obs  = Point::new(field_x[i].round() as i32, field_y[i].round() as i32);
				let real = Point::new(index_x[i].round() as i32, index_y[i].round() as i32);
		
				let _ = line(&mut img_err, real, obs, color_line, circle_thickness, 1, 0);
				let _= circle(&mut img_err, obs, circle_radius, color_obs, circle_thickness,1,0);
				let _=circle(&mut img_err, real, circle_radius, color_real, circle_thickness,1,0);
			}
		
			let _ = imshow("Original", &img_org);
			let _ = imshow("Error",    &img_err);
			let _ = wait_key(0);
		}
		else
		{
			println!("COULD NOT OPEN FITS: {}", dir);
		}
	}
}