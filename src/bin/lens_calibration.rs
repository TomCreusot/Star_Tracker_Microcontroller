#![allow(unused_must_use)]
extern crate star_tracker;
extern crate fitsio;
extern crate fitsio_derive;
extern crate image;

use fitsio::FitsFile;                   
use fitsio::tables::FitsRow;    
// use fitsio::hdu::FitsHdu;    
use fitsio::hdu::HduInfo;    
use fitsio_derive::FitsRow;

use star_tracker::image_processing::Image;
use star_tracker::util::units::Pixel;
use star_tracker::util::aliases::Byte;
use star_tracker::util::units::Cartesian3D;
use star_tracker::nix::NixImage;       



fn main ( )
{
	//
	// Reads the fits file.
	//
	println!("Reading corr.fits ...");
	let file_name_corr= "results/lens/corr_files/corr.fits";
	let mut file_corr = FitsFile::open(file_name_corr).expect("Could not read file 'corr.fits'.");
	let hdu_corr      = file_corr.hdu(1).expect("No table in  corr.fits goto nova.astronomy.net."); 
	
	//
	// Reads the img file.
	//
	println!("Reading image.jpg ...");
	let file_name_img = "results/lens/corr_files/image.jpg";
	let mut img       = NixImage::read_image(file_name_img);
	println!("Image\t|\trows: {},\tcolumns: {}", img.width(), img.height());

	if let HduInfo::TableInfo { ref column_descriptions, ref num_rows, .. } = hdu_corr.info
	{
		// Find the difference between actual and expected in the corr.fits.
		// diff is the distance between the actual and expected and the position in the image.
		let mut diff : Vec<(f64, Cartesian3D)> = Vec::new();
		let mut min = 10.0;
		let mut max = 0.0;
	    println!("Table\t|\trows: {},\tcolumns: {}", num_rows, column_descriptions.len());
		
		//
		// Find difference actual and expected in fits.
		// The actual is
		// The expected is
		//
		for r in 0..*num_rows
		{
			let actual = Cartesian3D
				{
					x: hdu_corr.read_cell_value::<f64>(&mut file_corr,"field_x", r).expect(""),
					y: hdu_corr.read_cell_value::<f64>(&mut file_corr,"field_y", r).expect(""),
					z: 0.0
				};
			let expected = Cartesian3D
				{
					x: hdu_corr.read_cell_value::<f64>(&mut file_corr,"index_x", r).expect(""),
					y: hdu_corr.read_cell_value::<f64>(&mut file_corr,"index_y", r).expect(""),
					z: 0.0
				};
			let difference = (actual - expected).magnitude();
			// if ( difference < 1.5 )
			{
			max = if max < difference { difference } else { max };
			min = if difference < min { difference } else { min };
			// println!("{:.2}", difference);
			diff.push((difference, actual));
			img.draw_points(Pixel{x: actual.x as usize, y: actual.y as usize}, 10, [0, 255, 0]);
			img.draw_points(Pixel{x: expected.x as usize,y: expected.y as usize}, 10, [0, 0, 255]);
			}
		}
		
		
		
		
		
		
		
		
		
		println!("Creating displacement map...");
		
		// Find variance in difference.
		let mut avg_diff : f64 = 0.0;
		// let mut max_diff : f64 = 0.0;
		let mut min_diff : f64 = f64::INFINITY;
		
		for i in 0..diff.len()
		{
			avg_diff = diff[i].0;
			// max_diff = if max_diff < diff[i].0 { diff[i].0 } else { max_diff };
			min_diff = if min_diff < diff[i].0 { min_diff } else { diff[i].0 };
		}
		avg_diff /= diff.len() as f64;
		// let var_diff = max_diff - min_diff;
		
		
		// Create displacement map by:
		// * Setting all pixels to 50%.
		// * Finding the closest N measurements.
		// * Adding the intensity of each measurement to the pixel - the average displacement.
		// * A dark patch is when the measurement is below the average displacement.
		// * A light patch is when the measurement is above the average displacement.
		// Pixels will be skipped by *step* to speed up the process.
		let max_dist : f64 = 100.0;
		let step = 1;
		let mut x = 0;
		let mut y = 0;
		while x < img.width()
		{
			while y < img.height()
			{
				let loops = 20;					// The number of measurements to consider.
				let pixel = Pixel{x: x, y: y};
				let mut influence = avg_diff;
				let mut lowest_all_val = 0.0;			// The previous closest distance.
				let mut lowest_cur_val = f64::INFINITY;	// The current distance.
				let mut lowest_cur_idx = 0;				// The index of the current distance.
				
				// Loops through and finds the closest n measurements to the pixel.
				for _i in 0..loops
				{
					for d in 0..diff.len()
					{
						let dist = (diff[d].1 - pixel.into()).magnitude();
						if lowest_all_val < dist && dist < lowest_cur_val
						{
							lowest_cur_idx = d;
							lowest_cur_val = dist;
						}
					}
					influence += diff[lowest_cur_idx].0 - avg_diff;
					lowest_all_val = lowest_cur_val;
					lowest_cur_val = f64::INFINITY;
				}
				influence /= loops as f64;

				let intensity = influence * 255.0 / max_dist;
				
				// Applies to all pixels which will be skipped.
				let mut xx = x;
				let mut yy = y;
				while xx < step + x && xx < img.width()
				{
					while yy < step + y && yy < img.height()
					{
						img.set(Pixel{x: xx, y: yy}, intensity as Byte);
						yy+=1;
					}
					yy = y;
					xx+=1;
				}
				// img.draw_points(cur_pix, 10, [0, 0, 255]);

				y += step;
			}
			y = 0;
			x += step;
			println!("{} of {}", x, img.width());
		}
		img.img_rgb.save("results/lens/out/img_curve.png");
		
		
		
		
		
		
		
		
		
		
		// Draws on the other image.
		for r in 0..*num_rows
		{
			let actual = Cartesian3D
				{
					x: hdu_corr.read_cell_value::<f64>(&mut file_corr,"field_x", r).expect(""),
					y: hdu_corr.read_cell_value::<f64>(&mut file_corr,"field_y", r).expect(""),
					z: 0.0
				};
			let expected = Cartesian3D
				{
					x: hdu_corr.read_cell_value::<f64>(&mut file_corr,"index_x", r).expect(""),
					y: hdu_corr.read_cell_value::<f64>(&mut file_corr,"index_y", r).expect(""),
					z: 0.0
				};
			let difference = (actual - expected).magnitude();
			img.draw_points(Pixel{x: actual.x as usize, y: actual.y as usize}, 3, [0, 255, 0]);
			img.draw_points(Pixel{x: expected.x as usize, y: expected.y as usize}, 3, [255, 0, 255]);
			img.draw_points(Pixel{x: expected.x as usize, y: expected.y as usize}, (difference * 5.0) as u32, [255, 0, 255]);
		}
		img.img_rgb.save("results/lens/out/img_curve.png");
		
	}
		
	
	
	
	
}


#[derive(Default, FitsRow)]
struct Row {                            
	
	#[fitsio(colname = "field_x")]
	field_x : f64,
	
	#[fitsio(colname = "field_y")]        
	field_y : f64,              
	
	
	
	#[fitsio(colname = "field_ra")]        
	field_ra : f64
	,              
	#[fitsio(colname = "field_dec")]        
	field_dec : f64,              
	
	
	
	#[fitsio(colname = "index_x")]        
	index_x : f64
	,              
	#[fitsio(colname = "index_y")]        
	index_y : f64,              
	
	
}                                       

