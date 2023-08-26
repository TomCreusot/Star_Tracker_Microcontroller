//! implementation of [The threshold trait](crate::image_processing::Threshold).
use crate::image_processing::Image;
use crate::image_processing::ThresholdPercent;
use crate::image_processing::ThresholdGrid;
use crate::image_processing::Threshold;
use crate::image_processing::BasicImage;

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
	/// use star_tracker_lib::image_processing::BasicImage;
	/// use star_tracker_lib::image_processing::Image;
	/// use star_tracker_lib::util::units::Pixel;
	/// let mut img : BasicImage<3, 3> = BasicImage::new();
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
	/// use star_tracker_lib::image_processing::BasicImage;
	/// use star_tracker_lib::image_processing::Image;
	/// use star_tracker_lib::image_processing::ThresholdPercent;
	/// use star_tracker_lib::util::list::ArrayList;
	/// use star_tracker_lib::util::aliases::UInt;
	/// use star_tracker_lib::util::aliases::Byte;
	/// use star_tracker_lib::util::units::Pixel;
	///
	/// let img : BasicImage<16, 16> = BasicImage::new();
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
	pub fn new ( img: &dyn Image, overshoot: Byte ) -> Self
	{
		let mut cells : BasicImage<NUM_H, NUM_V> = BasicImage::new();
		
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
				
				for xx in start_x .. end_x
				{				
					for yy in start_y .. end_y
					{
						count += 1.0;
						cell_val += img.get(Pixel{x: xx, y: yy}) as Decimal;
					}
				}
				let mean = (cell_val / count).round() as Byte;
				cells.set(cell, mean.saturating_add(overshoot));			
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
}


impl <const NUM_H: usize, const NUM_V: usize> Threshold for ThresholdGrid<NUM_H, NUM_V>
{
	/// Returns the threshold which is considered as the foreground (stars).
	fn foreground ( &self, point: Pixel ) -> Byte
	{
		return self.cells.get(self.get_cell(point));
	}
}






//###############################################################################################//
//										--- Nodal ---
//###############################################################################################//
//
//impl ThresholdNodal<const NUM_H: usize, const NUM_V: usize>
//{
//	/// Generates a threshold for the given image.
//	/// Returns the threshold.
//	fn new ( img: &Image ) -> Self
//	{
//		let nodes = BasicImage::new();
//	
//		let node_reach = Pixel {
//			x: img.width()  / (NODES.x - 1),
//			y: img.height() / (NODES.y - 1),
//		};
//
//		// |1      |...|2     2|...|3  3  3|...|4 4 4 4|
//		let node_spacing = Pixel {
//			x: img.width()  / (NODES.x - 1),
//			y: img.height() / (NODES.y - 1),
//		};
//
//		for n_x in 0..NODES.x
//		{
//			for n_y in 0..NODES.y
//			{
//				let node = Pixel{x: n_x, y: n_y};
//				let min = Pixel{
//					x: node.x.saturating_sub(node_reach.x), 
//					y: node.y.saturating_sub(node_reach.y)};
//					
//				let max = Pixel{
//					x: std::cmp::min(node.x + node_reach.x, img.width()), 
//					y: std::cmp::min(node.y + node_reach.y, img.height())};
//				
//				let mut avg = 0.0;
//				for p_x in min.x .. max.x
//				{
//					for p_y in min.y .. max.y
//					{
//						let node = Pixel{x: p_x, y: p_y};			
//						avg += img.get(pos) as Decimal;
//					}
//				}
//				avg /= node_reach.x * node_reach.y;
//				nodes.get(node) = avg as Byte;
//			}
//		}
//		
//		return Ok(Self{nodes: nodes});
//	}
//}
//
//
//
//pub impl <const NUM_H: usize, const NUM_V: usize> Threshold for ThresholdNodal<NUM_H, NUM_V>
//{
//	/// Returns true if the pixel is in the foreground (stars).
//	fn foreground ( &self, point: Pixel, value: Byte ) -> bool
//	{
//		
//	}
//}






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
		let mut img : BasicImage<100, 100> = BasicImage::new();
		
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
		let mut img : BasicImage<101, 101> = BasicImage::new();
		
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
		let img : BasicImage<3, 3> = BasicImage::new();
		let hist : [UInt; 1] = [9];
		assert_eq!(ThresholdPercent::threshold(&img, 0.0, &hist), 0);
	}

	#[test]
	fn test_percent_threshold_1_bar_1_percent ( )
	{
		let img : BasicImage<3, 3> = BasicImage::new();
		let hist : [UInt; 1] = [9];
		assert_eq!(ThresholdPercent::threshold(&img, 0.01, &hist), Byte::MAX);
	}

	#[test]
	fn test_perecent_threshold_2_bar_49_percent ( )
	{
		let img : BasicImage<2, 2> = BasicImage::new();
		let hist : [UInt; 2] = [2, 2];
		assert_eq!(ThresholdPercent::threshold(&img, 0.49, &hist), 128);
	}

	#[test]
	fn test_percent_threshold_2_bar_50_percent ( )
	{
		let img : BasicImage<2, 2> = BasicImage::new();
		let hist : [UInt; 2] = [2, 2];
		assert_eq!(ThresholdPercent::threshold(&img, 0.6, &hist), Byte::MAX);
	}

	#[test]
	fn test_percent_threshold_256_bar ( )
	{
		let img : BasicImage<16, 16> = BasicImage::new();
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
// pub new        ( &dyn Image,   Byte )  -> ThresholdGrid
// pub foreground ( &self, Pixel, Byte  ) -> bool
//
//###############################################################################################//
//										~ new ~													 //
	#[test]
	// With a single cell, a signel threshold should be used over the whole image.
	fn test_grid_new_single_cell ( )
	{
		let mut img : BasicImage<10, 10> = BasicImage::new();
		
		for xx in 0..10
		{
			for yy in 0..10
			{
				img.set(Pixel{x: xx, y: yy}, xx as Byte);
			}
		}

		let thresh = ThresholdGrid::<1, 1>::new(&img, 0);
		assert_eq!(thresh.cells.get(Pixel{x: 0, y: 0}), 5);
		let thresh = ThresholdGrid::<1, 1>::new(&img, 3);
		assert_eq!(thresh.cells.get(Pixel{x: 0, y: 0}), 3+5);
	}
	
	#[test]
	fn test_grid_new_four_cell ( )
	{
		let mut img : BasicImage<4, 4> = BasicImage::new();
		img.set(Pixel{x: 0, y: 0}, 12);
		img.set(Pixel{x: 3, y: 0}, 16);
		img.set(Pixel{x: 0, y: 3}, 20);
		img.set(Pixel{x: 3, y: 3}, 24);
		
		let thresh = ThresholdGrid::<2, 2>::new(&img, 0);
		assert_eq!(thresh.cells.get(Pixel{x: 0, y: 0}), 12 / 4);
		assert_eq!(thresh.cells.get(Pixel{x: 1, y: 0}), 16 / 4);
		assert_eq!(thresh.cells.get(Pixel{x: 0, y: 1}), 20 / 4);
		assert_eq!(thresh.cells.get(Pixel{x: 1, y: 1}), 24 / 4);
	}
	
		#[test]
	fn test_grid_new_odd_cells ( )
	{
		let mut img : BasicImage<3, 3> = BasicImage::new();
		img.set(Pixel{x:0,y:0}, 0);   img.set(Pixel{x:1,y:0}, 50);  img.set(Pixel{x:2,y:0}, 100);
		img.set(Pixel{x:0,y:1}, 50);  img.set(Pixel{x:1,y:1}, 50);  img.set(Pixel{x:2,y:1}, 150);
		img.set(Pixel{x:0,y:2}, 100); img.set(Pixel{x:1,y:2}, 150); img.set(Pixel{x:2,y:2}, 200);
		
		// Added 1 to all results due to truncation
		let thresh = ThresholdGrid::<2, 2>::new(&img, 0);
		assert_eq!(thresh.cells.get(Pixel{x: 0, y: 0}) as usize, (0  + 50  + 50  + 50 ) / 4 + 1);
		assert_eq!(thresh.cells.get(Pixel{x: 1, y: 0}) as usize, (50 + 50  + 100 + 150) / 4 + 1);
		assert_eq!(thresh.cells.get(Pixel{x: 0, y: 1}) as usize, (50 + 50  + 100 + 150) / 4 + 1);
		assert_eq!(thresh.cells.get(Pixel{x: 1, y: 1}) as usize, (50 + 150 + 150 + 200) / 4 + 1);
	}
		
	
	
	
//										~ Foreground ~											 //
	#[test]
	fn test_grid_foreground_single_cell ( )
	{
		let mut cells: BasicImage<1, 1> = BasicImage::new();
		cells.set(Pixel{x: 0, y: 0}, 10);
	
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
		let mut cells: BasicImage<2, 2> = BasicImage::new();
		cells.set(Pixel{x: 0, y: 0}, 10);
		cells.set(Pixel{x: 1, y: 0}, 20);
		cells.set(Pixel{x: 0, y: 1}, 30);
		cells.set(Pixel{x: 1, y: 1}, 40);
	
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
		let mut cells: BasicImage<3, 3> = BasicImage::new();
		cells.set(Pixel{x:0,y:0}, 11); cells.set(Pixel{x:1,y:0}, 21); cells.set(Pixel{x:2,y:0}, 31);
		cells.set(Pixel{x:0,y:1}, 12); cells.set(Pixel{x:1,y:1}, 22); cells.set(Pixel{x:2,y:1}, 32);
		cells.set(Pixel{x:0,y:2}, 13); cells.set(Pixel{x:1,y:2}, 23); cells.set(Pixel{x:2,y:2}, 33);
	
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

















