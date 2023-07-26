extern crate fitsio;
extern crate opencv;

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
	let filename = "corr.fits";
	let f = FitsFile::open(filename);
	let mut f_2 = FitsFile::open(filename).expect("");
	
	let mut field_x : Vec<f64> = Vec::new();
	let mut field_y : Vec<f64> = Vec::new();
	let mut index_x : Vec<f64> = Vec::new();
	let mut index_y : Vec<f64> = Vec::new();
	
	if let Ok(mut fits) = f
	{
		println!("FITS file successfully opened.");


		if let Ok(hdu) = fits.hdu(1)
		{
			field_x = hdu.read_col(&mut fits, "field_x").expect("");
			field_y = hdu.read_col(&mut fits, "field_y").expect("");
			index_x = hdu.read_col(&mut fits, "index_x").expect("");
			index_y = hdu.read_col(&mut fits, "index_y").expect("");
					
			// for col in hdu.columns(&mut fits)
			// {
			// 
			// 	 if let Column::Double{name, data} = col
			// 	 {
			// 	 	println!("{:?}", name);
			// 	 }
			// }
		}
		
		let mut img = imread( "starlight_6mm_46.704fov.png", ImreadModes::IMREAD_COLOR as i32 ).expect("");
		for i in 0..field_x.len()
		{
			let circle_thickness = 1;
			let circle_radius    = 1;
			let mut circle_color = Scalar::new(0.0, 100.0, 0.0, 0.0); // Red color (BGR format)
			
			let mut obs  = Point::new(field_x[i].round() as i32, field_y[i].round() as i32);
			let mut real = Point::new(index_x[i].round() as i32, index_y[i].round() as i32);
			
			let _ = line(&mut img, real, obs, circle_color, circle_thickness, 1, 0);
			
			circle(&mut img, obs, circle_radius, circle_color, circle_thickness,1,0,
			).unwrap();
			circle(&mut img, real, circle_radius, circle_color, circle_thickness,1,0,
			).unwrap();
		}
		
		imshow("image", &img);
		wait_key(0);
		
		
		
	}
	else
	{
		println!("COULD NOT OPEN FITS!\nplease have a corr.fits file in star_tracker_nix");
	}
}