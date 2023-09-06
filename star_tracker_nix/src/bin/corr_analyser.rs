#![allow(unused_imports)]
//! This program is used to identify how a lens is distorted.  
//! This will show a set of 3 images;  
//!
//! - Original
//!   The original image with no modifications.
//!
//! - Error
//!   Two circles will be surrounding each star, 
//!   The green circle is where the star is observed.
//!   The red circle is where the star should be.
//!   The larger the circle, the more distortion.
//!   There is also a big red circle, the `center of mass` this is where the least distortion is calculated.
//!
//! - Distortion
//!   This is a colorful image showing how the lens is warped.
//!   The `center of mass` of the lens is calculated, this is where the distortion is least.
//!   Each pixel considers its angle from the `center of mass`, and will visually represent this.
//!   There is a border around the image, this is the color that the pixels should ideally be at that location.
//!

extern crate fitsio;
extern crate opencv;
extern crate star_tracker_nix;
extern crate star_tracker_lib;

use fitsio::FitsFile;
use fitsio::tables::Column;

use opencv::imgcodecs::imread;
use opencv::imgcodecs::ImreadModes;

use opencv::core::Mat;
use opencv::core::Size;
use opencv::core::Scalar;
use opencv::core::Point;
use opencv::core::Vec3b;
use opencv::prelude::MatTraitConst;
use opencv::imgproc::circle;
use opencv::imgproc::line;
use opencv::imgproc::put_text;
use opencv::core::Point3_;
use opencv::core::Vector;
use opencv::prelude::MatTraitConstManual;
use opencv::core::prelude::MatTrait;
use opencv::imgproc::cvt_color;
use opencv::imgproc::COLOR_HLS2BGR;


use opencv::highgui::wait_key;
use opencv::highgui::imshow;

use star_tracker_lib::util::aliases::Decimal;
use star_tracker_lib::util::units::Degrees;
use star_tracker_lib::util::units::Radians;
use star_tracker_lib::util::units::Equatorial;
use star_tracker_lib::util::units::Vector2;

use star_tracker_nix::io::Sample;
use star_tracker_nix::io::StarError;

pub fn main ( )
{
	println!(r#"===== Corr Analyser =====
This program is used to identify how a lens is distorted.  
This will show a set of 3 images;  
	
	- Original
	  The original image with no modifications.
	
	- Error
	  Two circles will be surrounding each star, 
	  The green circle is where the star is observed.
	  The red circle is where the star should be.
	  The larger the circle, the more distortion.
	   There is also a big red circle, the `center of mass` this is where the least distortion is calculated.
	
	- Distortion
	  This is a colorful image showing how the lens is warped.
	  The `center of mass` of the lens is calculated, this is where the distortion is least.
	  Each pixel considers its angle from the `center of mass`, and will visually represent this.
	  There is a border around the image, this is the color that the pixels should ideally be at that location.


	  
	"#);

	let samples = Sample::load_samples();

	for sample in samples
	{
		let cor = sample.get_corr();
		let dir = sample.dir;
		let img_file = &sample.file_img[0];


		if let Some(cor) = cor
		{
			println!("{}", dir);
			// Draws error onto image
			let img_org      = imread( &img_file, ImreadModes::IMREAD_COLOR as i32 ).unwrap();
			let mut img_err  = imread( &img_file, ImreadModes::IMREAD_COLOR as i32 ).unwrap();
			let mut img_dist = imread( &img_file, ImreadModes::IMREAD_COLOR as i32 ).unwrap();

			let mut center_mass = Vector2{x: 0.0, y: 0.0};
			let mut avg_err_px = 0.0;
			let mut avg_err_eq = Radians(0.0);

			let mut st_dev_err_px = 0.0;
			let mut min_err_px = 0.0;
			let mut max_err_px = 0.0;
			let mut st_dev_err_eq = Radians(0.0);
			let mut min_err_eq = Radians(0.0);
			let mut max_err_eq = Radians(0.0);

			let circle_thickness = 1;
			let circle_radius    = 1;
			let color_real = Scalar::new(0.0,   0.0, 100.0, 0.0); // Red color (BGR format)
			let color_obs  = Scalar::new(0.0, 100.0,   0.0, 0.0); // Red color (BGR format)
			let color_line = Scalar::new(0.0, 100.0, 100.0, 0.0); // Red color (BGR format)

			// Calculate center of mass and averages.
			// Draws on error image showing where the stars should be.
			let mut total_error = 0.0;
			for i in 0..cor.len()
			{
				center_mass = center_mass + cor[i].image_px / cor[i].error_px;
				total_error += 1.0 / cor[i].error_px;

				avg_err_px += cor[i].error_px;
				avg_err_eq = avg_err_eq + cor[i].error_eq;

				let obs  = Point::new(cor[i].image_px.x.round() as i32, cor[i].image_px.y.round() as i32);
				let real = Point::new(cor[i].real_px.x.round()  as i32, cor[i].real_px.y.round()  as i32);

				let _= circle(&mut img_err, obs,  cor[i].error_px.round() as i32 * 2, color_obs,  circle_thickness,1,0);
				let _= circle(&mut img_err, real, circle_radius, color_real, circle_thickness,1,0);
				let _= line  (&mut img_err, real, obs, color_line, circle_thickness, 1, 0);
			}
			avg_err_px    /= cor.len() as f64;
			avg_err_eq.0  /= cor.len() as Decimal;
			center_mass = center_mass / total_error;

			let _=circle(&mut img_err, Point::new(center_mass.x as i32, center_mass.y as i32), 50, color_real, 10,1,0);


			// Calculate standard deviation.
			for i in 0..cor.len()
			{
				st_dev_err_px += (cor[i].error_px - avg_err_px).powf(2.0);
				st_dev_err_eq.0 += (cor[i].error_eq.0 - avg_err_eq.0).powf(2.0);
				max_err_px = if max_err_px < cor[i].error_px { cor[i].error_px }else{ max_err_px };
				max_err_eq = if max_err_eq < cor[i].error_eq { cor[i].error_eq }else{ max_err_eq };
				min_err_px = if cor[i].error_px < min_err_px { cor[i].error_px }else{ min_err_px };
				min_err_eq = if cor[i].error_eq < min_err_eq { cor[i].error_eq }else{ min_err_eq };
			}
			st_dev_err_px = (st_dev_err_px / cor.len() as f64).sqrt();
			st_dev_err_eq.0 = (st_dev_err_eq.0 / cor.len() as Decimal).sqrt();


			// Draws distortion
			for x in 0..img_dist.cols()
			{
				for y in 0..img_dist.rows()
				{
					let pos = Vector2{x: x as Decimal, y: y as Decimal};
					let mut total_offset = 0.0;
					let mut sum = Vector2{x: 0.0, y: 0.0};

					for i in 0..cor.len()
					{
						let offset = cor[i].image_px - pos;
						let err = (cor[i].image_px - cor[i].real_px) / offset.magnitude();
						sum = sum + err;
						total_offset += 1.0 / offset.magnitude();
					}
					sum = sum * total_offset;
					
			
					let intensity = 127.0;
					// let intensity = scalar_error * 127.0 / 1000.0;
					// let intensity = scalar_error * 5000.0 * 127.0;
					// println!("{}", intensity);
					let mut angle = sum.y.atan2(sum.x);
					let c = img_dist.at_2d_mut::<Point3_<u8>>(y as i32, x as i32).expect("?");
			
					let dir_center = Vector2{x: x as Decimal, y: y as Decimal} - center_mass; 
					
					// if (intensity * 10.0) as u32 % 100 < 10
					// {
					// 
					// 	c.x = 100; c.y = 255; c.z = 255;
					// }
					// else
					// {
						if (pos.x < 5.0 || img_err.cols() as Decimal - 6.0 < pos.x) 
						|| (pos.y < 5.0 || img_err.rows() as Decimal - 6.0 < pos.y)
						{
							angle = dir_center.y.atan2(dir_center.x);
						}
						c.z = ( -angle.sin() * intensity + intensity) as u8;
						c.x = (-(angle + 2.0*std::f64::consts::PI/3.0).sin() * intensity + intensity) as u8;
						c.y = (-(angle + 4.0*std::f64::consts::PI/3.0).sin() * intensity + intensity) as u8;
					// }
				}
			}

			println!("\nAVG PX: {:.4}", avg_err_px);
			println!("STD PX: {:.4}",   st_dev_err_px);
			println!("MAX PX: {:.4}",   max_err_px);
			println!("MIN PX: {:.4}\n", min_err_px);

			println!("AVG EQ: {:.6}", avg_err_eq.to_degrees());
			println!("STD EQ: {:.6}", st_dev_err_eq.to_degrees());
			println!("MAX EQ: {:.6}", max_err_eq.to_degrees());
			println!("MIN EQ: {:.6}", min_err_eq.to_degrees());

			println!("CENTER OF MASS: {}", center_mass);


			let _ = imshow("Original",   &img_org);
			let _ = imshow("Error",      &img_err);
			let _ = imshow("Distortion", &img_dist);
			let _ = wait_key(0);
			println!("\n\n\n\n\n\n");
		}
		else
		{
			println!("COULD NOT OPEN FITS: {}", dir);
		}
	}
}
