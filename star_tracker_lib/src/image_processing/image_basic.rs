//! Implementation for ImageBasic.
use crate::core_include::*;

use crate::util::aliases::Byte;
use crate::util::units::Pixel;
use crate::image_processing::ImageBasic;
use crate::image_processing::Image;



/// Creates a black image of the given size.
/// # Arguments
/// * `x` - The horizontal size of the x image (Constant).
/// * `y` - The vertical size of the image (Constant).
/// 
/// # Returns
/// A basic image of the given size.
///
///
/// # Example
/// ```
/// use star_tracker_lib::image_processing::ImageBasic;
/// use star_tracker_lib::image_processing::Image;
/// use star_tracker_lib::create_image_basic;
/// 
/// const WIDTH:  usize = 10;
/// const HEIGHT: usize = 11;
/// let img = create_image_basic!(WIDTH, HEIGHT);
///
/// assert_eq!(img.width(),  WIDTH);
/// assert_eq!(img.height(), HEIGHT);
/// ```
#[macro_export]
macro_rules! create_image_basic {
	($x:expr, $y:expr) => 
	{
		ImageBasic{img: &mut [[0; $x]; $y]}
	}
}



impl <'a, const WIDTH: usize, const HEIGHT: usize> ImageBasic <'a, WIDTH, HEIGHT>
{
//###############################################################################################//
//										---	Constructors ---
//###############################################################################################//

	/// Imports an image from a multidimensional array.  
	/// Can be used if an image is stored elsewhere.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::image_processing::{ImageBasic, Image};
	/// use star_tracker_lib::util::units::Pixel;
	/// use star_tracker_lib::create_image_basic;
	/// 
	/// const WIDTH:  usize = 1;
	/// const HEIGHT: usize = 10;
	/// 
	/// let mut arr = [[0; WIDTH]; HEIGHT];
	/// let img = ImageBasic::new(&mut arr);
	/// assert_eq!(img.get(Pixel{x: 0, y: 0}), 0);
	/// ```
	pub fn new ( img: &'a mut [[Byte; WIDTH]; HEIGHT] ) -> Self { ImageBasic { img: img } }
}


impl <'a, const WIDTH : usize, const HEIGHT : usize> Image for ImageBasic <'a, WIDTH, HEIGHT>
{
//###############################################################################################//
//										---	Accessors ---
//###############################################################################################//

	/// Returns the pixel value at the current position.
	/// # Arguments
	/// * `pixel` - The pixel.
	///
	///	# Example
	/// ```
	/// use star_tracker_lib::image_processing::{ImageBasic, Image};
	/// use star_tracker_lib::util::units::Pixel;
	///
	/// use star_tracker_lib::create_image_basic;
	/// 
	/// const WIDTH:  usize = 1;
	/// const HEIGHT: usize = 1;
	/// let img = create_image_basic!(WIDTH, HEIGHT);
	/// assert_eq!(img.get(Pixel{x: 0, y: 0}), 0);
	/// ```
	fn get ( &self, pixel : Pixel ) -> Byte
	{
		return self.img[pixel.y][pixel.x].clone();
	}

	/// Sets the pixel value at the current position.
	/// # Arguments
	///	* `pixel` - The pixel.
	/// * `value` - The value to set.
	///
	///	# Example
	/// ```
	/// use star_tracker_lib::image_processing::{ImageBasic, Image};
	/// use star_tracker_lib::util::units::Pixel;	
	/// use star_tracker_lib::create_image_basic;
	/// 
	/// const WIDTH:  usize = 1;
	/// const HEIGHT: usize = 1;
	/// let mut img = create_image_basic!(WIDTH, HEIGHT);
	/// img.set(Pixel{x: 0, y: 0}, 10);
	/// assert_eq!(img.get(Pixel{x: 0, y: 0}), 10);
	/// ```
	fn set ( &mut self, pixel: Pixel, value: Byte )
	{
		self.img[pixel.y][pixel.x] = value;
	}



	/// Returns the width of the image.
	///	# Returns
	///	The width of the image.
	fn width ( &self ) -> usize	{	return WIDTH;	}


	/// Returns the height of the image.
	///	# Returns
	///	The height of the image.
	fn height ( &self ) -> usize	{	return HEIGHT;	}
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
	use crate::util::units::Pixel;
	use image_processing::ImageBasic;
	use image_processing::Image;
	use image_processing::Byte;


//###############################################################################################//
//
//										Basic Image
//
// pub fn get         ( &self, x: usize, y: usize ) -> Byte 
// pub fn set         ( &self, x: usize, y: usize, Byte ) 
// pub fn width       ( &self ) -> usize
// pub fn height      ( &self ) -> usize
// pub fn valid_pixel ( &self, Pixel ) -> bool
//
//###############################################################################################//
//											~ get ~												 //
	#[test]
	fn test_get_in_bounds ( )
	{
		let img = create_image_basic!(10, 10);
		assert_eq!(0, img.get(Pixel{x: 0, y: 0}));
		assert_eq!(0, img.get(Pixel{x: 9, y: 9}));
	}
	
	
	#[test]
	#[should_panic]
	fn test_get_out_of_bounds ( )
	{
		let img = create_image_basic!(10, 10);
		img.get(Pixel{x: 10, y: 10});
	}
	
	
	
	//											~ set ~												 //
	#[test]
	fn test_set_in_bounds ( )
	{
		let mut img = create_image_basic!(10, 10);
		img.set(Pixel{x: 0, y: 0}, 10);
		img.set(Pixel{x: 9, y: 9}, 11);
		assert_eq!(10, img.get(Pixel{x: 0, y: 0}));
		assert_eq!(11, img.get(Pixel{x: 9, y: 9}));
	}
	
	#[test]
	#[should_panic]
	fn test_set_out_of_bounds ( )
	{
		let mut img = create_image_basic!(10, 10);
		img.set(Pixel{x: 10, y: 10}, 0);
	}
	
	
	//											~ width ~											 //
	#[test]
	fn test_width ( )
	{
		let img = create_image_basic!(9, 10);
		assert_eq!(9, img.width());
	}
	
	//											~ height ~											 //
	#[test]
	fn test_height ( )
	{
		let img = create_image_basic!(10, 11);
		assert_eq!(11, img.height());
	}


}
