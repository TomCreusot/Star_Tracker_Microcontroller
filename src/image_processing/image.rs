//! The trait Image.
use crate::util::units::Pixel;
use crate::util::aliases::Decimal;
use crate::util::aliases::Byte;
use crate::util::aliases::UInt;

use crate::util::err::Errors;
use crate::util::err::Error;
/// Image accessors, investigation and modification.  
///
/// Allows for external image packages to be wrapped and new image methods to be investigated.
/// (This is ultimately just to stop requiring generic arguments as consts in generic is too new).
pub trait Image
{
	/// Returns the pixel value at the current position.
	/// # Arguments
	/// * `px` -  The pixel to modify.
	fn get ( &self, px : Pixel ) -> Byte;

	/// Sets the pixel value at the current position.
	/// # Arguments
	///	* `px`     - The pixel to modify.
	/// * `value` - The value to set.
	fn set ( &mut self, px: Pixel, value: Byte );

	/// Returns the width of the image.
	///	# Returns
	///	The width of the image.
	fn width ( &self ) -> usize;

	/// Returns the height of the image.
	///	# Returns
	///	The height of the image.
	fn height ( &self ) -> usize;


	/// Sets all the pixels to 0.
	/// (Allows you to reuse the same image).
	/// # Example
	/// ```
	/// use star_tracker::util::units::Pixel;
	/// use star_tracker::image_processing::BasicImage; 
	/// use star_tracker::image_processing::Image;
	/// 
	/// let mut img : BasicImage<3, 3> = BasicImage::new();
	/// img.set(Pixel{x: 0, y: 0}, 10);
	/// img.set(Pixel{x: 2, y: 0}, 20);
	/// img.set(Pixel{x: 0, y: 2}, 32);
	/// img.set(Pixel{x: 0, y: 1}, 43);
	///
	/// img.reset(); // All the pixels are now 0.
	/// ```
	fn reset ( &mut self )
	{
		for xx in 0..self.width()
		{
			for yy in 0..self.height()
			{
				self.set(Pixel{x: xx, y: yy}, 0);
			}
		}
	}

	/// True if the get/set will not cause a panic.
	/// # Arguments
	/// * `px` - The pixel to modify.
	///
	/// # Returns
	/// True if access is safe.
	fn valid_pixel ( &self, px: Pixel ) -> bool
	{ return px.x < self.width() && px.y < self.height() }

	/// Generates a brightness histogram of the image.
	/// # Arguments
	/// * `histogram` - The histogram to fill, must have a size > 0 && < 256.
	///
	/// # Example
	/// ```
	/// use star_tracker::image_processing::BasicImage;
	/// use star_tracker::image_processing::Image;
	/// use star_tracker::util::aliases::UInt;
	/// use star_tracker::util::aliases::Byte;
	/// use star_tracker::util::units::Pixel;
	/// let mut img : BasicImage<3, 3> = BasicImage::new();
	/// let mut hist : [UInt; Byte::max_value() as usize + 1] = [0; Byte::MAX as usize + 1];
	/// img.set(Pixel{x: 0, y: 0}, 10);
	/// img.set(Pixel{x: 2, y: 0}, 20);
	/// img.set(Pixel{x: 0, y: 2}, 32);
	/// img.set(Pixel{x: 0, y: 1}, 43);
	///
	/// img.histogram(&mut hist);
	/// assert_eq!(hist[0], 5);
	/// assert_eq!(hist[10], 1);
	/// assert_eq!(hist[20], 1);
	/// assert_eq!(hist[32], 1);
	/// assert_eq!(hist[43], 1);
	/// ```
	fn histogram ( &self, histogram : &mut [UInt] ) -> Error<()>
	{
		if histogram.len() == 0 || (Byte::MAX as UInt + 1) < histogram.len() as UInt
		{
			return Err(Errors::InvalidSize);
		}
		
		
		let ratio : Decimal = (histogram.len() as Decimal) / (Byte::MAX as Decimal + 1.0);
		for y in 0..self.height()
		{
			for x in 0..self.width()
			{
				let bar : usize = ((self.get(Pixel{x, y}) as Decimal) * ratio) as usize;
				histogram[bar] += 1;
			}
		}
		
		return Ok(());
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
	/// use star_tracker::image_processing::BasicImage;
	/// use star_tracker::image_processing::Image;
	/// use star_tracker::util::list::ArrayList;
	/// use star_tracker::util::aliases::UInt;
	/// use star_tracker::util::aliases::Byte;
	///
	/// let img : BasicImage<16, 16> = BasicImage::new();
	/// let hist :  [UInt; Byte::max_value() as usize + 1] = [1; Byte::MAX as usize + 1]; // [1, 1, ...]
	/// assert_eq!(img.percent_threshold(0.5, &hist), Byte::MAX / 2 + 1);
	/// assert_eq!(img.percent_threshold(0.0, &hist), 0);
	/// assert_eq!(img.percent_threshold(1.0, &hist), Byte::MAX);
	/// ```
	fn percent_threshold ( &self, percentage : Decimal, histogram : &[UInt] ) -> Byte
	{
		let cutoff: UInt = (percentage * (self.width() * self.height()) as Decimal).ceil() as UInt;

		let mut count : UInt = 0;
		let mut i : UInt = 0;
		while count < cutoff && i < histogram.len() as UInt
		{
			count += histogram[i as usize];
			i+=1;
		}
		return ((i as Decimal * Byte::MAX as Decimal) / histogram.len() as Decimal).ceil() as Byte;
	}


	/// Copies an image over to another image.
	/// The is mainly for nix.
	/// This can be useful to duplicate an image before blob detection.
	/// # Arguments
	/// * `to` - The image to be overwritten.
	/// # Returns
	/// Error if invalid.
	///
	/// # Example
	/// ```
	/// use star_tracker::util::units::Pixel;
	/// use star_tracker::image_processing::BasicImage;
	/// use star_tracker::image_processing::Image;
	///
	/// let mut from: BasicImage<2, 2> = BasicImage::new();
	/// from.set(Pixel{x: 0, y: 0}, 1);
	/// from.set(Pixel{x: 0, y: 1}, 2);
	/// from.set(Pixel{x: 1, y: 0}, 3);
	/// from.set(Pixel{x: 1, y: 1}, 4);
	/// let mut to: BasicImage<2, 2> = BasicImage::new();
	/// assert!(from.copy_to(&mut to).is_ok());
	/// assert_eq!(to.get(Pixel{x: 0, y: 0}), from.get(Pixel{x: 0, y: 0}));
	/// assert_eq!(to.get(Pixel{x: 0, y: 1}), from.get(Pixel{x: 0, y: 1}));
	/// assert_eq!(to.get(Pixel{x: 1, y: 0}), from.get(Pixel{x: 1, y: 0}));
	/// assert_eq!(to.get(Pixel{x: 1, y: 1}), from.get(Pixel{x: 1, y: 1}));
	/// ```
	fn copy_to ( &self, to: &mut dyn Image ) -> Error<()>
	{
		if self.width() != to.width() || self.height() != to.height()
		{
			return Err(Errors::InvalidSize);
		}

		for xx in 0..self.width()
		{
			for yy in 0..self.height()
			{
				let px = Pixel{x: xx, y: yy};
				to.set(px, self.get(px));
			}
		}
		return Ok(());
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
	use image_processing::*;



//###############################################################################################//
//
//										Features
//
// NOT IMPLEMENTED pub fn get    ( &self, Pixel ) -> Byte
// NOT IMPLEMENTED pub fn set    ( &self, Pixel, Byte )
// NOT IMPLEMENTED pub fn width  ( &self ) -> usize
// NOT IMPLEMENTED pub fn height ( &self ) -> usize
// 
// pub fn reset          ( &mut self )
// pub fn valid_pixel    ( &self, Pixel ) -> bool
// pub histogram         ( &self, &mut [UInt] )
// pub percent_threshold ( &self, Decimal, &[UInt] ) -> Byte
// pub copy_to           ( &self, &mut dyn Image) -. Error<()>
//
//###############################################################################################//
//										~ reset ~												 //
#[test]
fn test_reset ( )
{
	let mut img : BasicImage<3, 3> = BasicImage::new();
	img.set(Pixel{x: 0, y: 0}, 10);
	img.set(Pixel{x: 2, y: 0}, 20);
	img.set(Pixel{x: 0, y: 2}, 32);
	img.set(Pixel{x: 0, y: 1}, 43);
	
	img.reset();
	for xx in 0..img.width()
	{
		for yy in 0..img.height()
		{
			assert_eq!(img.get(Pixel{x: xx, y: yy}), 0);
		}
	}
}




//										~ valid pixel ~											 //
	#[test]
	fn test_valid_pixel_safe ( )
	{
		let img : BasicImage<10, 10> = BasicImage::new();
		assert!(img.valid_pixel(Pixel{x: 0, y: 0}));
		assert!(img.valid_pixel(Pixel{x: 9, y: 9}));
		assert!(img.valid_pixel(Pixel{x: 0, y: 9}));
		assert!(img.valid_pixel(Pixel{x: 9, y: 0}));
	}
	#[test]
	fn test_valid_pixel_unsafe ( )
	{
		let img : BasicImage<10, 10> = BasicImage::new();
		assert!(!img.valid_pixel(Pixel{x: 10, y: 10}));
		assert!(!img.valid_pixel(Pixel{x: 0, y: 10}));
		assert!(!img.valid_pixel(Pixel{x: 0, y: 10}));
	}



	

//										~ histogram ~											 //
	#[test]
	fn test_histogram_256_bars ( )
	{
		let mut img : BasicImage<3, 3> = BasicImage::new();
		let mut hist : [UInt; Byte::max_value() as usize + 1] = [0; Byte::MAX as usize + 1];
		img.set(Pixel{x: 0, y: 0}, 10);
		img.set(Pixel{x: 2, y: 0}, 20);
		img.set(Pixel{x: 0, y: 2}, 32);
		img.set(Pixel{x: 0, y: 1}, 43);

		img.histogram(&mut hist).expect("This should be valid.");
		assert_eq!(hist[0], 5);
		assert_eq!(hist[10], 1);
		assert_eq!(hist[20], 1);
		assert_eq!(hist[32], 1);
		assert_eq!(hist[43], 1);
	}

	#[test]
	fn test_histogram_2_bar ( )
	{
		let mut img : BasicImage<3, 3> = BasicImage::new();
		let mut hist : [UInt; 2] = [0; 2];
		img.set(Pixel{x: 2, y: 2}, Byte::MAX / 2 + 1);
		img.histogram(&mut hist).expect("This should be valid.");
		assert_eq!(hist[0], 8);
		assert_eq!(hist[1], 1);
	}

	#[test]
	fn test_histogram_1_bar ( )
	{
		let mut img : BasicImage<3, 3> = BasicImage::new();
		let mut hist : [UInt; 1] = [0; 1];
		img.set(Pixel{x: 2, y: 2}, Byte::MAX);
		img.histogram(&mut hist).expect("This should be valid.");
		assert_eq!(hist[0], 9);
	}

	#[test]
	fn test_histogram_fail ( )
	{
		let img        : BasicImage<3, 3> = BasicImage::new();
		let mut hist_small : [UInt; 0] = [];
		let mut hist_large : [UInt; Byte::MAX as usize + 2] = [0; Byte::MAX as usize + 2];
		assert!(img.histogram(&mut hist_small).is_err());
		assert!(img.histogram(&mut hist_large).is_err());
	}




//										~ percent threshold ~									 //
	#[test]
	fn test_percent_threshold_1_bar_0_percent ( )
	{
		let img : BasicImage<3, 3> = BasicImage::new();
		let hist : [UInt; 1] = [9];
		assert_eq!(img.percent_threshold(0.0, &hist), 0);
	}

	#[test]
	fn test_percent_threshold_1_bar_1_percent ( )
	{
		let img : BasicImage<3, 3> = BasicImage::new();
		let hist : [UInt; 1] = [9];
		assert_eq!(img.percent_threshold(0.01, &hist), Byte::MAX);
	}

	#[test]
	fn test_perecent_threshold_2_bar_49_percent ( )
	{
		let img : BasicImage<2, 2> = BasicImage::new();
		let hist : [UInt; 2] = [2, 2];
		assert_eq!(img.percent_threshold(0.49, &hist), 128);
	}

	#[test]
	fn test_percent_threshold_2_bar_50_percent ( )
	{
		let img : BasicImage<2, 2> = BasicImage::new();
		let hist : [UInt; 2] = [2, 2];
		assert_eq!(img.percent_threshold(0.6, &hist), Byte::MAX);
	}

	#[test]
	fn test_percent_threshold_256_bar ( )
	{
		let img : BasicImage<16, 16> = BasicImage::new();
		let hist :  [UInt; Byte::max_value() as usize + 1] = [1; Byte::MAX as usize + 1]; // [1, 1, ...]
		assert_eq!(img.percent_threshold(0.5, &hist), Byte::MAX / 2 + 1);
		assert_eq!(img.percent_threshold(0.0, &hist), 0);
		assert_eq!(img.percent_threshold(1.0, &hist), Byte::MAX);
	}


//										~ copy_to ~												 //
	#[test]
	fn test_copy_to ( )
	{
		let mut from: BasicImage<2, 2> = BasicImage::new();
		from.set(Pixel{x: 0, y: 0}, 1);
		from.set(Pixel{x: 0, y: 1}, 2);
		from.set(Pixel{x: 1, y: 0}, 3);
		from.set(Pixel{x: 1, y: 1}, 4);
		let mut to: BasicImage<2, 2> = BasicImage::new();
		assert!(from.copy_to(&mut to).is_ok());
		assert_eq!(to.get(Pixel{x: 0, y: 0}), from.get(Pixel{x: 0, y: 0}));
		assert_eq!(to.get(Pixel{x: 0, y: 1}), from.get(Pixel{x: 0, y: 1}));
		assert_eq!(to.get(Pixel{x: 1, y: 0}), from.get(Pixel{x: 1, y: 0}));
		assert_eq!(to.get(Pixel{x: 1, y: 1}), from.get(Pixel{x: 1, y: 1}));
	}

	#[test]
	fn test_copy_to_error ( )
	{
		let mut from: BasicImage<2, 2> = BasicImage::new();
		let mut to  : BasicImage<2, 3> = BasicImage::new();
		assert!(from.copy_to(&mut to).is_err());
		assert!(to.copy_to(&mut from).is_err());
	}
}
