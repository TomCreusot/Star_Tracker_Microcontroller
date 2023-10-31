//! implementation of [The threshold trait](crate::image_processing::Threshold).
use crate::image_processing::Image;
use crate::image_processing::ThresholdPercent;
use crate::image_processing::ThresholdGrid;
use crate::image_processing::Threshold;
use crate::image_processing::ImageBasic;

use crate::util::aliases::Decimal;
use crate::util::aliases::Byte;
use crate::util::aliases::UInt;
use crate::util::units::Pixel;
use crate::util::err::Error;
use crate::util::maths::Maths;
//###############################################################################################//
//										--- Percent ---
//###############################################################################################//

impl ThresholdPercent
{
	/// Generates a threshold for the given image.
	/// # Arguments
	/// * `img`    - The image
	/// * `percent`- What percentage of the darkest pixels should be ignored? (0.9999).
	///
	/// # Returns
	/// Returns threshold with the given percent.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::image_processing::ThresholdPercent;
	/// use star_tracker_lib::image_processing::Threshold;
	/// use star_tracker_lib::image_processing::ImageBasic;
	/// use star_tracker_lib::image_processing::Image;
	/// use star_tracker_lib::util::units::Pixel;
	///	
	/// const WIDTH : usize = 3;
	/// const HEIGHT: usize = 3;
	/// let mut img_array = [[0; WIDTH]; HEIGHT];
	/// let mut img = ImageBasic::new(&mut img_array);
	///	img.set(Pixel{x: 0, y: 0}, 1); img.set(Pixel{x: 1, y: 0}, 4); img.set(Pixel{x: 2, y: 0}, 5);
	/// img.set(Pixel{x: 0, y: 1}, 2); img.set(Pixel{x: 1, y: 1}, 4); img.set(Pixel{x: 2, y: 1}, 6);
	/// img.set(Pixel{x: 0, y: 2}, 3); img.set(Pixel{x: 1, y: 2}, 4); img.set(Pixel{x: 2, y: 2}, 7);
	/// // Each pixel is 0.12%
	///
	/// let thresh = ThresholdPercent::new(&img, 0.11);
	/// assert_eq!(thresh.foreground(Pixel{x: 0, y: 0}), 1);
	/// let thresh = ThresholdPercent::new(&img, 0.12);
	/// assert_eq!(thresh.foreground(Pixel{x: 0, y: 0}), 2);
	/// let thresh = ThresholdPercent::new(&img, 0.24);
	/// assert_eq!(thresh.foreground(Pixel{x: 0, y: 0}), 3);
	/// 
	/// ```
	pub fn new ( img: &dyn Image, percent: Decimal ) -> Self
	{
		let mut histogram : [UInt; 255] = [0; 255];
		let _ = img.histogram(&mut histogram);
		return Self{threshold: Self::threshold(img, percent, &histogram)};
	}
	
	
	/// Finds the minimum bar of the histogram which satisfies the percentage of pixels.
	/// # Arguments
	/// * `percentage` - The minimum % of pixels to be foreground.
	/// * `histogram`  - The histogram to examine.
	///
	/// # Returns
	/// The minimum brightness value to be foreground.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::image_processing::ImageBasic;
	/// use star_tracker_lib::image_processing::Image;
	/// use star_tracker_lib::image_processing::ThresholdPercent;
	/// use star_tracker_lib::util::list::ArrayList;
	/// use star_tracker_lib::util::aliases::UInt;
	/// use star_tracker_lib::util::aliases::Byte;
	/// use star_tracker_lib::util::units::Pixel;
	///
	/// const WIDTH : usize = 16;
	/// const HEIGHT: usize = 16;
	/// let mut img_array = [[0; WIDTH]; HEIGHT];
	/// let img : ImageBasic<16, 16> = ImageBasic::new(&mut img_array);
	/// let hist :  [UInt; Byte::max_value() as usize + 1] = [1; Byte::MAX as usize + 1]; // [1, 1, ...]
	/// assert_eq!(ThresholdPercent::threshold(&img, 0.5, &hist), Byte::MAX / 2 + 1);
	/// assert_eq!(ThresholdPercent::threshold(&img, 0.0, &hist), 0);
	/// assert_eq!(ThresholdPercent::threshold(&img, 1.0, &hist), Byte::MAX);
	/// ```
	pub fn threshold ( img: &dyn Image, percentage : Decimal, histogram : &[UInt] ) -> Byte
	{
		let cutoff: UInt = (percentage * (img.width() * img.height()) as Decimal).ceil() as UInt;

		let mut count : UInt = 0;
		let mut i : UInt = 0;
		while count < cutoff && i < histogram.len() as UInt
		{
			count += histogram[i as usize];
			i+=1;
		}
		return ((i as Decimal * Byte::MAX as Decimal) / histogram.len() as Decimal).ceil() as Byte;
	}
}


impl Threshold for ThresholdPercent
{
	/// Returns the threshold which is considered as the foreground (stars).
	fn foreground ( &self, _point: Pixel ) -> Byte
	{
		return self.threshold;
	}
}



//###############################################################################################//
//										--- Grid ---
//###############################################################################################//

impl <const NUM_H: usize, const NUM_V: usize> ThresholdGrid <NUM_H, NUM_V>
{
	/// Generates a threshold for the given image.
	/// # Arguments
	/// * `img`      - The image
	/// * `overshoot`- From the average of the cell, should the threshold be higher or lower by how much.
	/// * `skip`     - Sample every *skip* pixel to speed up the algorithm.
	///
	/// # Returns
	/// Returns a threshold of every cell..
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::image_processing::ThresholdGrid;
	/// use star_tracker_lib::image_processing::Threshold;
	/// use star_tracker_lib::image_processing::ImageBasic;
	/// use star_tracker_lib::image_processing::Image;
	/// use star_tracker_lib::util::units::Pixel;
	///
	/// const WIDTH : usize = 99;
	/// const HEIGHT: usize = 99;
	/// let mut img_array = [[0; WIDTH]; HEIGHT];
	/// let img = ImageBasic::new(&mut img_array);
	///
	/// // Threshold will generate 9 cells which have a threshold value of the average pixel + 10.
	/// // Every second pixel will be skipped to make the algorithm run 4 times faster (2^2).
	/// let thresh = ThresholdGrid::<3,3>::new(&img, 10, 1);
	/// assert_eq!(thresh.foreground(Pixel{x: 0, y: 0}), 10); // 0 + 10
	/// 
	/// ```
	pub fn new ( img: &dyn Image, overshoot: Byte, skip: usize ) -> Self
	{
		let mut cells = [[0; NUM_H]; NUM_V];
		for col in 0..NUM_H
		{
			for row in 0..NUM_V
			{
				let cell = Pixel{x: col, y: row};
				let mut cell_val = 0.0;
				let mut count = 0.0;
				
				// Due to truncating
				let start_x = col * img.width() / NUM_H;
				let start_y = row * img.height() / NUM_V;
				let end_x   = (((col + 1) * img.width())  as Decimal / NUM_H as Decimal).round() as usize;
				let end_y   = (((row + 1) * img.height()) as Decimal / NUM_V as Decimal).round() as usize;
				
				for xx in ( start_x .. end_x ).step_by(skip + 1)
				{				
					for yy in ( start_y .. end_y ).step_by(skip + 1)
					{
						count += 1.0;
						cell_val += img.get(Pixel{x: xx, y: yy}) as Decimal;
					}
				}
				let mean = (cell_val / count).round() as Byte;
				cells[cell.y][cell.x] = mean.saturating_add(overshoot);
			}
		}
		return Self{size: Pixel{x: img.width(), y: img.height()}, cells: cells};
	}


	/// Gets the cell with the correct threshold.
	fn get_cell ( &self, pos: Pixel ) -> Pixel
	{
		return Pixel
		{
			x: pos.x * NUM_H / self.size.x, 
			y: pos.y * NUM_V / self.size.y
		};
	}

	fn get_cell_val ( &self, pos: Pixel ) -> Byte
	{
		return self.cells[pos.y][pos.x];
	}
}


impl <const NUM_H: usize, const NUM_V: usize> Threshold for ThresholdGrid<NUM_H, NUM_V>
{
	/// Returns the threshold which is considered as the foreground (stars).
	fn foreground ( &self, point: Pixel ) -> Byte
	{
		let cell = self.get_cell(point);
		return self.cells[cell.y][cell.x]; 
	}
}









//###############################################################################################//
//###############################################################################################//
//
//										Unit Tests
//
//###############################################################################################//
//###############################################################################################//
#[cfg(test)]
mod test
{
	use crate::util::aliases::Byte;
	use crate::util::aliases::UInt;
	use crate::util::units::Pixel;
	use crate::image_processing::*;


	pub fn get_image <const W: usize, const H: usize> ( ) -> [[Byte; W]; H]
	{
		[[0;W]; H]
	}

//###############################################################################################//
//
//										Threshold Percent
//
// pub new        ( &dyn Image, Decimal ) -> ThresholdPercent
// pub threshold  ( &dyn Image, Decimal, [UInt])
// pub foreground ( &self, Pixel, Byte  ) -> bool
//
//###############################################################################################//
//										~ new ~													 //
	#[test]
	fn test_percent_new_1 ( )
	{
		let mut arr = get_image();
		let mut img : ImageBasic<100, 100> = ImageBasic::new(&mut arr);
		
		for xx in 0..100
		{
			for yy in 0..100
			{
				img.set(Pixel{x: xx, y: yy}, xx as Byte);
			}
		}

		let thresh = ThresholdPercent::new(&img, 0.5);
		assert_eq!(thresh.threshold, 49);
		let thresh = ThresholdPercent::new(&img, 0.2);
		assert_eq!(thresh.threshold, 19);
		let thresh = ThresholdPercent::new(&img, 0.8);
		assert_eq!(thresh.threshold, 79);
	}
	
	#[test]
	fn test_percent_new_2 ( )
	{
		let mut arr = get_image();
		let mut img : ImageBasic<101, 101> = ImageBasic::new(&mut arr);
		
		for xx in 0..101
		{
			for yy in 0..101
			{
				img.set(Pixel{x: xx, y: yy}, yy as Byte);
			}
		}

		let thresh = ThresholdPercent::new(&img, 0.5);
		assert_eq!(thresh.threshold, 50);
		let thresh = ThresholdPercent::new(&img, 0.2);
		assert_eq!(thresh.threshold, 20);
		let thresh = ThresholdPercent::new(&img, 0.8);
		assert_eq!(thresh.threshold, 80);
	}
	
	
	
	

//										~ percent threshold ~									 //
	#[test]
	fn test_percent_threshold_1_bar_0_percent ( )
	{
		let mut arr = get_image();
		let img : ImageBasic<3, 3> = ImageBasic::new(&mut arr);
		let hist : [UInt; 1] = [9];
		assert_eq!(ThresholdPercent::threshold(&img, 0.0, &hist), 0);
	}

	#[test]
	fn test_percent_threshold_1_bar_1_percent ( )
	{
		let mut arr = get_image();
		let img : ImageBasic<3, 3> = ImageBasic::new(&mut arr);
		let hist : [UInt; 1] = [9];
		assert_eq!(ThresholdPercent::threshold(&img, 0.01, &hist), Byte::MAX);
	}

	#[test]
	fn test_percent_threshold_2_bar_49_percent ( )
	{
		let mut arr = get_image();
		let img : ImageBasic<2, 2> = ImageBasic::new(&mut arr);
		let hist : [UInt; 2] = [2, 2];
		assert_eq!(ThresholdPercent::threshold(&img, 0.49, &hist), 128);
	}

	#[test]
	fn test_percent_threshold_2_bar_50_percent ( )
	{
		let mut arr = get_image();
		let img : ImageBasic<2, 2> = ImageBasic::new(&mut arr);
		let hist : [UInt; 2] = [2, 2];
		assert_eq!(ThresholdPercent::threshold(&img, 0.6, &hist), Byte::MAX);
	}

	#[test]
	fn test_percent_threshold_256_bar ( )
	{
		let mut arr = get_image();
		let img : ImageBasic<16, 16> = ImageBasic::new(&mut arr);
		let hist :  [UInt; Byte::max_value() as usize + 1] = [1; Byte::MAX as usize + 1]; // [1, 1, ...]
		assert_eq!(ThresholdPercent::threshold(&img, 0.5, &hist), Byte::MAX / 2 + 1);
		assert_eq!(ThresholdPercent::threshold(&img, 0.0, &hist), 0);
		assert_eq!(ThresholdPercent::threshold(&img, 1.0, &hist), Byte::MAX);
	}

	
	
	
	
	
	
//										~ foreground ~											 //

	#[test]
	fn test_percent_foreground ( )
	{
		let thresh = ThresholdPercent{threshold: 123};
		assert_eq!(thresh.foreground(Pixel{x: 0, y: 0}), 123);
		assert_eq!(thresh.foreground(Pixel {x: 1000, y: 1000}), 123);
	}
	
	
	
//###############################################################################################//
//
//										Threshold Grid
//
// pub new        ( &dyn Image,   Byte, usize )  -> ThresholdGrid
// pub foreground ( &self, Pixel, Byte  ) -> bool
//
//###############################################################################################//
//										~ new ~													 //
	#[test]
	// With a single cell, a signal threshold should be used over the whole image.
	fn test_grid_new_single_cell ( )
	{
		let mut arr = get_image();
		let mut img : ImageBasic<10, 10> = ImageBasic::new(&mut arr);
		
		for xx in 0..10
		{
			for yy in 0..10
			{
				img.set(Pixel{x: xx, y: yy}, xx as Byte);
			}
		}

		let thresh = ThresholdGrid::<1, 1>::new(&img, 0, 0);
		assert_eq!(thresh.get_cell_val(Pixel{x: 0, y: 0}), 5);
		let thresh = ThresholdGrid::<1, 1>::new(&img, 3, 0);
		assert_eq!(thresh.get_cell_val(Pixel{x: 0, y: 0}), 3+5);
	}
	
	#[test]
	fn test_grid_new_four_cell ( )
	{
		let mut arr = get_image();
		let mut img : ImageBasic<4, 4> = ImageBasic::new(& mut arr);
		img.set(Pixel{x: 0, y: 0}, 12);
		img.set(Pixel{x: 3, y: 0}, 16);
		img.set(Pixel{x: 0, y: 3}, 20);
		img.set(Pixel{x: 3, y: 3}, 24);
		
		let thresh = ThresholdGrid::<2, 2>::new(&img, 0, 0);
		assert_eq!(thresh.get_cell_val(Pixel{x: 0, y: 0}), 12 / 4);
		assert_eq!(thresh.get_cell_val(Pixel{x: 1, y: 0}), 16 / 4);
		assert_eq!(thresh.get_cell_val(Pixel{x: 0, y: 1}), 20 / 4);
		assert_eq!(thresh.get_cell_val(Pixel{x: 1, y: 1}), 24 / 4);
	}
	
		#[test]
	fn test_grid_new_odd_cells ( )
	{
		let mut arr = get_image();
		let mut img : ImageBasic<3, 3> = ImageBasic::new(&mut arr);
		img.set(Pixel{x:0,y:0}, 0);   img.set(Pixel{x:1,y:0}, 50);  img.set(Pixel{x:2,y:0}, 100);
		img.set(Pixel{x:0,y:1}, 50);  img.set(Pixel{x:1,y:1}, 50);  img.set(Pixel{x:2,y:1}, 150);
		img.set(Pixel{x:0,y:2}, 100); img.set(Pixel{x:1,y:2}, 150); img.set(Pixel{x:2,y:2}, 200);
		
		// Added 1 to all results due to truncation
		let thresh = ThresholdGrid::<2, 2>::new(&img, 0, 0);
		assert_eq!(thresh.get_cell_val(Pixel{x: 0, y: 0}) as usize, (0  + 50  + 50  + 50 ) / 4 + 1);
		assert_eq!(thresh.get_cell_val(Pixel{x: 1, y: 0}) as usize, (50 + 50  + 100 + 150) / 4 + 1);
		assert_eq!(thresh.get_cell_val(Pixel{x: 0, y: 1}) as usize, (50 + 50  + 100 + 150) / 4 + 1);
		assert_eq!(thresh.get_cell_val(Pixel{x: 1, y: 1}) as usize, (50 + 150 + 150 + 200) / 4 + 1);
	}
	
	
	#[test]
	fn test_grid_new_excessive_overshoot ( )
	{
		let mut arr = get_image();
		let mut img : ImageBasic<1, 1> = ImageBasic::new(&mut arr);
		img.set(Pixel{x:0,y:0}, 10);
		
		// Added 1 to all results due to truncation
		let thresh = ThresholdGrid::<1, 1>::new(&img, 254, 0);
		assert_eq!(thresh.get_cell_val(Pixel{x: 0, y: 0}) as usize, 255);
	}
	

	#[test]
	// Does the skip produce the same results?
	fn test_grid_new_skip_reproducible ( )
	{
		const GRID_SIZE : usize = 10;
		let mut arr = get_image();
		let mut img : ImageBasic<{3 * GRID_SIZE}, {3 * GRID_SIZE}> = ImageBasic::new(&mut arr);
		for x in 0..img.width() {
			for y in 0..img.height() {
				img.set(Pixel{x: x, y: y}, ((x / GRID_SIZE) + (y / GRID_SIZE * 5)) as Byte);
		} }
		
		// Added 1 to all results due to truncation
		let thresh = ThresholdGrid::<3, 3>::new(&img, 0, 1);
		assert_eq!(thresh.get_cell_val(Pixel{x: 0, y: 0}) as usize, 0);
		assert_eq!(thresh.get_cell_val(Pixel{x: 1, y: 0}) as usize, 1);
		assert_eq!(thresh.get_cell_val(Pixel{x: 2, y: 0}) as usize, 2);

		assert_eq!(thresh.get_cell_val(Pixel{x: 0, y: 1}) as usize, 5);
		assert_eq!(thresh.get_cell_val(Pixel{x: 1, y: 1}) as usize, 6);	
		assert_eq!(thresh.get_cell_val(Pixel{x: 2, y: 1}) as usize, 7);	

		assert_eq!(thresh.get_cell_val(Pixel{x: 0, y: 2}) as usize, 10);
		assert_eq!(thresh.get_cell_val(Pixel{x: 1, y: 2}) as usize, 11);	
		assert_eq!(thresh.get_cell_val(Pixel{x: 2, y: 2}) as usize, 12);	
	}
	

	#[test]
	// every second pixel is skipped when skip is set to 2.
	// If every second pixel is off, the threshold should still be the same as if they all were the same.
	fn test_grid_new_skip_every_other ( )
	{
		const GRID_SIZE : usize = 10;
		let mut arr = get_image();
		let mut img : ImageBasic<{3 * GRID_SIZE}, {3 * GRID_SIZE}> = ImageBasic::new(&mut arr);
		for x in 0..img.width() {
			for y in 0..img.height() {
				if x % 2 == 0 && y % 2 == 0 {
					img.set(Pixel{x: x, y: y}, ((x / GRID_SIZE) + (y / GRID_SIZE * 5)) as Byte);
		} } }

		// Added 1 to all results due to truncation
		let thresh = ThresholdGrid::<3, 3>::new(&img, 0, 1);
		assert_eq!(thresh.get_cell_val(Pixel{x: 0, y: 0}) as usize, 0);
		assert_eq!(thresh.get_cell_val(Pixel{x: 1, y: 0}) as usize, 1);
		assert_eq!(thresh.get_cell_val(Pixel{x: 2, y: 0}) as usize, 2);

		assert_eq!(thresh.get_cell_val(Pixel{x: 0, y: 1}) as usize, 5);
		assert_eq!(thresh.get_cell_val(Pixel{x: 1, y: 1}) as usize, 6);	
		assert_eq!(thresh.get_cell_val(Pixel{x: 2, y: 1}) as usize, 7);	

		assert_eq!(thresh.get_cell_val(Pixel{x: 0, y: 2}) as usize, 10);
		assert_eq!(thresh.get_cell_val(Pixel{x: 1, y: 2}) as usize, 11);	
		assert_eq!(thresh.get_cell_val(Pixel{x: 2, y: 2}) as usize, 12);	
	}

	
	
//										~ Foreground ~											 //
	#[test]
	fn test_grid_foreground_single_cell ( )
	{		
		let mut cells: [[Byte; 1]; 1] = [[0; 1]; 1];
		cells[0][0] = 10;
		
		let thresh = ThresholdGrid
		{
			size:  Pixel{x: 101, y: 101},
			cells: cells
		};
		
		assert_eq!(thresh.foreground(Pixel{x: 0, y: 0}),     10);
		assert_eq!(thresh.foreground(Pixel{x: 100, y: 100}), 10);
		assert_eq!(thresh.foreground(Pixel{x: 50, y: 50}),   10);
	}
	
	
	#[test]
	fn test_grid_foreground_double_cell ( )
	{
		let mut cells: [[Byte; 2]; 2] = [[0; 2]; 2];
		cells[0][0] = 10; cells[0][1] = 20;
		cells[1][0] = 30; cells[1][1] = 40;
	
		let thresh = ThresholdGrid
		{
			size:  Pixel{x: 100, y: 100},
			cells: cells
		};
		
		assert_eq!( thresh.foreground(Pixel{x: 0,  y:  0}), 10);
		assert_eq!( thresh.foreground(Pixel{x: 49, y: 49}), 10);
		
		assert_eq!( thresh.foreground(Pixel{x: 50, y:  0}), 20);
		assert_eq!( thresh.foreground(Pixel{x: 99, y: 49}), 20);
		
		assert_eq!( thresh.foreground(Pixel{x: 0,  y: 50}), 30);
		assert_eq!( thresh.foreground(Pixel{x: 49, y: 99}), 30);
	
		assert_eq!( thresh.foreground(Pixel{x: 50, y: 50}), 40);
		assert_eq!( thresh.foreground(Pixel{x: 99, y: 99}), 40);	
	}
	
	
	#[test]
	fn test_grid_odd_cells ( )
	{
		let mut cells: [[Byte; 3]; 3] = [[0;3];3];
		
		cells[0][0] = 11;   cells[0][1] = 21;    cells[0][2] = 31;
		cells[1][0] = 12;   cells[1][1] = 22;    cells[1][2] = 32;
		cells[2][0] = 13;   cells[2][1] = 23;    cells[2][2] = 33;

		let thresh = ThresholdGrid
		{
			size:  Pixel{x: 10, y: 10},
			cells: cells
		};
		
		// With an odd number of cells, the cutoff between the cells will be rounded down.
		// This will provide the top left cell with more cells.
		// 0 1 2 3 | 4 5 6 | 7 8 9
		assert_eq!( thresh.foreground(Pixel{x: 0, y: 0}), 11);
		assert_eq!( thresh.foreground(Pixel{x: 3, y: 3}), 11);
		
		assert_eq!( thresh.foreground(Pixel{x: 4, y: 0}), 21);
		assert_eq!( thresh.foreground(Pixel{x: 6, y: 3}), 21);
		
		assert_eq!( thresh.foreground(Pixel{x: 7, y: 0}), 31);
		assert_eq!( thresh.foreground(Pixel{x: 9, y: 3}), 31);
		
		
		assert_eq!( thresh.foreground(Pixel{x: 0, y: 4}), 12);
		assert_eq!( thresh.foreground(Pixel{x: 3, y: 6}), 12);
		
		assert_eq!( thresh.foreground(Pixel{x: 4, y: 4}), 22);
		assert_eq!( thresh.foreground(Pixel{x: 6, y: 6}), 22);
		
		assert_eq!( thresh.foreground(Pixel{x: 7, y: 4}), 32);
		assert_eq!( thresh.foreground(Pixel{x: 9, y: 6}), 32);
		
		
		assert_eq!( thresh.foreground(Pixel{x: 0, y: 7}), 13);
		assert_eq!( thresh.foreground(Pixel{x: 3, y: 9}), 13);
		
		assert_eq!( thresh.foreground(Pixel{x: 4, y: 7}), 23);
		assert_eq!( thresh.foreground(Pixel{x: 6, y: 9}), 23);
		
		assert_eq!( thresh.foreground(Pixel{x: 7, y: 7}), 33);
		assert_eq!( thresh.foreground(Pixel{x: 9, y: 9}), 33);
		
		
		}
}

















