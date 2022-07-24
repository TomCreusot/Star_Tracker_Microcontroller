#![allow(unused_must_use)]
extern crate star_tracker;
extern crate fitsio;
extern crate fitsio_derive;
extern crate image;

use fitsio::FitsFile;                   
use fitsio::tables::FitsRow;    
use fitsio::hdu::FitsHdu;    
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
		
		
		
		
		
		
		
		
		
		println!("");
		
		for x in 0..img.width()
		{
			for y in 0..img.height()
			{
				let mut intensity = 0.0;
				let cur_pix = Pixel{x: x, y: y};
				
				let mut count = 0;
				let mut max_dist = 0.0;
				for i in 0..diff.len()
				{
					let cur = (diff[i].1 - cur_pix.into()).magnitude();
					if max_dist < cur
					{
						max_dist = cur;
					}
				}
				for i in 0..diff.len()
				{
					let dist = (diff[i].1 - cur_pix.into()).magnitude();
					intensity += (1.0 - dist / max_dist) * 1.0/diff[i].0;
						// intensity += (max_dist - (diff[i].1 - cur_pix.into()).magnitude()) * 1.0/diff[i].0;
					count += 1;
				}
				if count != 0
				{
					intensity /= count as f64;
				}
				intensity *= 500.0;
				// println!("{:?}", intensity);
				// print!("\x1B[2J \x1B[3J \x1B[H");
				img.set(cur_pix, intensity as Byte);
				// img.draw_points(cur_pix, 10, [0, 0, 255]);
				
			}
			println!("{} of {}", x, img.width());
		}

		img.img_rgb.save("results/lens/out/img_curve.png");
		
		
		
		
		
		
		
		
		
		
		
		
		
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

