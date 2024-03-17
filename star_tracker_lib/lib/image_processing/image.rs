//! The trait Image.
use core_include::*;

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
	/// use star_tracker_lib::util::units::Pixel;
	/// use star_tracker_lib::image_processing::ImageBasic;
	/// use star_tracker_lib::image_processing::Image;
	///
	/// const WIDTH : usize = 3;
	/// const HEIGHT: usize = 3;
	/// let mut img_array = [[0; WIDTH]; HEIGHT];
	/// let mut img = ImageBasic::new(&mut img_array);
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
	/// use star_tracker_lib::image_processing::ImageBasic;
	/// use star_tracker_lib::image_processing::Image;
	/// use star_tracker_lib::util::aliases::UInt;
	/// use star_tracker_lib::util::aliases::Byte;
	/// use star_tracker_lib::util::units::Pixel;
	///
	/// const WIDTH : usize = 3;
	/// const HEIGHT: usize = 3;
	/// let mut img_array = [[0; WIDTH]; HEIGHT];
	/// let mut img = ImageBasic::new(&mut img_array);
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
			return Result::Err(Errors::InvalidSize);
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

		return Result::Ok(());
	}


	/// Copies an image over to this image.
	/// The is mainly for nix.
	/// This can be useful to duplicate an image before blob detection.
	/// # Arguments
	/// * `from` - The image to copy from.
	/// # Returns
	/// Error if invalid.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Pixel;
	/// use star_tracker_lib::image_processing::ImageBasic;
	/// use star_tracker_lib::image_processing::Image;
	///
	/// const WIDTH : usize = 2;
	/// const HEIGHT: usize = 2;
	/// let mut img_array = [[0; WIDTH]; HEIGHT];
	/// let mut from = ImageBasic::new(&mut img_array);
	/// from.set(Pixel{x: 0, y: 0}, 1);
	/// from.set(Pixel{x: 0, y: 1}, 2);
	/// from.set(Pixel{x: 1, y: 0}, 3);
	/// from.set(Pixel{x: 1, y: 1}, 4);
	///
	/// const WIDTH_2 : usize = 2;
	/// const HEIGHT_2: usize = 2;
	/// let mut img_array = [[0; WIDTH_2]; HEIGHT_2];
	/// let mut to = ImageBasic::new(&mut img_array);
	/// assert!(to.copy_from(&from).is_ok());
	/// assert_eq!(to.get(Pixel{x: 0, y: 0}), from.get(Pixel{x: 0, y: 0}));
	/// assert_eq!(to.get(Pixel{x: 0, y: 1}), from.get(Pixel{x: 0, y: 1}));
	/// assert_eq!(to.get(Pixel{x: 1, y: 0}), from.get(Pixel{x: 1, y: 0}));
	/// assert_eq!(to.get(Pixel{x: 1, y: 1}), from.get(Pixel{x: 1, y: 1}));
	/// ```
	fn copy_from ( &mut self, from: &dyn Image ) -> Error<()>
	{
		if self.width() != from.width() || self.height() != from.height()
		{
			return Result::Err(Errors::InvalidSize);
		}

		for xx in 0..self.width()
		{
			for yy in 0..self.height()
			{
				let px = Pixel{x: xx, y: yy};
				self.set(px, from.get(px));
			}
		}
		return Result::Ok(());
	}


	/// Returns 8 as it is 8 bit.
	fn bits ( &self ) -> usize { 8 }
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

	pub fn get_image <const W: usize, const H: usize> ( ) -> [[Byte; W]; H]
	{
		[[0;W]; H]
	}

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
// pub copy_from         ( &self, &mut dyn Image) -. Error<()>
// pub bits              ( &self ) -> usize
//
//###############################################################################################//
//										~ reset ~												 //
#[test]
fn test_reset ( )
{
	let mut arr = get_image();
	let mut img : ImageBasic<3, 3> = ImageBasic::new(&mut arr);
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
		let mut arr = get_image();
		let img : ImageBasic<10, 10> = ImageBasic::new(&mut arr);
		assert!(img.valid_pixel(Pixel{x: 0, y: 0}));
		assert!(img.valid_pixel(Pixel{x: 9, y: 9}));
		assert!(img.valid_pixel(Pixel{x: 0, y: 9}));
		assert!(img.valid_pixel(Pixel{x: 9, y: 0}));
	}

	#[test]
	#[cfg_attr(coverage, coverage(off))]
	fn test_valid_pixel_unsafe ( )
	{
		let mut arr = get_image();
		let img : ImageBasic<10, 10> = ImageBasic::new(&mut arr);
		assert!(!img.valid_pixel(Pixel{x: 10, y: 10}));
		assert!(!img.valid_pixel(Pixel{x: 0, y: 10}));
		assert!(!img.valid_pixel(Pixel{x: 0, y: 10}));
	}





//										~ histogram ~											 //
	#[test]
	fn test_histogram_256_bars ( )
	{
		let mut arr = get_image();
		let mut img : ImageBasic<3, 3> = ImageBasic::new(&mut arr);
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
		let mut arr = get_image();
		let mut img : ImageBasic<3, 3> = ImageBasic::new(&mut arr);
		let mut hist : [UInt; 2] = [0; 2];
		img.set(Pixel{x: 2, y: 2}, Byte::MAX / 2 + 1);
		img.histogram(&mut hist).expect("This should be valid.");
		assert_eq!(hist[0], 8);
		assert_eq!(hist[1], 1);
	}

	#[test]
	fn test_histogram_1_bar ( )
	{
		let mut arr = get_image();
		let mut img : ImageBasic<3, 3> = ImageBasic::new(&mut arr);
		let mut hist : [UInt; 1] = [0; 1];
		img.set(Pixel{x: 2, y: 2}, Byte::MAX);
		img.histogram(&mut hist).expect("This should be valid.");
		assert_eq!(hist[0], 9);
	}

	#[test]
	fn test_histogram_fail ( )
	{
		let mut arr = get_image();
		let img        : ImageBasic<3, 3> = ImageBasic::new(&mut arr);
		let mut hist_small : [UInt; 0] = [];
		let mut hist_large : [UInt; Byte::MAX as usize + 2] = [0; Byte::MAX as usize + 2];
		assert!(img.histogram(&mut hist_small).is_err());
		assert!(img.histogram(&mut hist_large).is_err());
	}



//										~ copy_from ~											 //
	#[test]
	fn test_copy_from ( )
	{
		let mut arr_1 = get_image();
		let mut arr_2 = get_image();
		let mut from: ImageBasic<2, 2> = ImageBasic::new(&mut arr_1);
		from.set(Pixel{x: 0, y: 0}, 1);
		from.set(Pixel{x: 0, y: 1}, 2);
		from.set(Pixel{x: 1, y: 0}, 3);
		from.set(Pixel{x: 1, y: 1}, 4);
		let mut to: ImageBasic<2, 2> = ImageBasic::new(&mut arr_2);
		assert!(to.copy_from(&from).is_ok());
		assert_eq!(to.get(Pixel{x: 0, y: 0}), from.get(Pixel{x: 0, y: 0}));
		assert_eq!(to.get(Pixel{x: 0, y: 1}), from.get(Pixel{x: 0, y: 1}));
		assert_eq!(to.get(Pixel{x: 1, y: 0}), from.get(Pixel{x: 1, y: 0}));
		assert_eq!(to.get(Pixel{x: 1, y: 1}), from.get(Pixel{x: 1, y: 1}));
	}

	#[test]
	fn test_copy_from_error ( )
	{
		let mut arr_1 = get_image();
		let mut arr_2 = get_image();
		let mut from: ImageBasic<2, 2> = ImageBasic::new(&mut arr_1);
		let mut to  : ImageBasic<2, 3> = ImageBasic::new(&mut arr_2);
		assert!(from.copy_from(&to).is_err());
		assert!(to.copy_from(&from).is_err());
	}
	
	
	//										~ bits ~												 //
	fn test_bits ( )
	{
		let mut arr = get_image();
		let img: ImageBasic<2, 2> = ImageBasic::new(&mut arr);
		assert_eq!(img.bits(), 8);

	}
}
