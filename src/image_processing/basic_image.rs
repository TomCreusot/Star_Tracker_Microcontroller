//! Implementation for [BasicImage](crate::image_processing::BasicImage).

use crate::util::aliases::Byte;
use crate::util::units::Pixel;
use crate::image_processing::BasicImage;
use crate::image_processing::Image;
impl <const WIDTH : usize, const HEIGHT : usize> BasicImage <WIDTH, HEIGHT>
{
//###############################################################################################//
//										---	Constructors ---
//###############################################################################################//

	/// Creates an image with the default value of 0.
	/// # Example
	/// ```
	/// use star_tracker::image_processing::{BasicImage, Image};
	/// use star_tracker::util::units::Pixel;
	/// let img : BasicImage<1, 1> = BasicImage::new();
	/// assert_eq!(img.get(Pixel{x: 0, y: 0}), 0);
	/// ```
	pub fn new ( ) -> BasicImage<WIDTH, HEIGHT>	{	BasicImage { img: [[0; WIDTH]; HEIGHT] }	}
}


impl <const WIDTH : usize, const HEIGHT : usize> Image for BasicImage <WIDTH, HEIGHT>
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
	/// use star_tracker::image_processing::{BasicImage, Image};
	/// use star_tracker::util::units::Pixel;
	///	let img : BasicImage<1, 1> = BasicImage::new();
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
	/// use star_tracker::image_processing::{BasicImage, Image};
	/// use star_tracker::util::units::Pixel;
	///	let mut  img : BasicImage<1, 1> = BasicImage::new();
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
	///
	/// # Example
	/// ```
	///	use star_tracker::image_processing::{BasicImage, Image};
	///	let img : BasicImage<10, 0> = BasicImage::new();
	/// assert_eq!(img.width(), 10);
	/// ```
	fn width ( &self ) -> usize	{	return WIDTH;	}


	/// Returns the height of the image.
	///	# Returns
	///	The height of the image.
	///
	/// # Example
	/// ```
	/// use star_tracker::image_processing::{BasicImage, Image};
	///	let img : BasicImage<0, 10> = BasicImage::new();
	/// assert_eq!(img.height(), 10);
	/// ```
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
	use image_processing::BasicImage;
	use image_processing::Image;
	
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
		let img : BasicImage<10, 10> = BasicImage::new();
		assert_eq!(0, img.get(Pixel{x: 0, y: 0}));
		assert_eq!(0, img.get(Pixel{x: 9, y: 9}));
	}


	#[test]
	#[should_panic]
	fn test_get_out_of_bounds ( )
	{
		let img : BasicImage<10, 10> = BasicImage::new();
		img.get(Pixel{x: 10, y: 10});
	}



//											~ set ~												 //
	#[test]
	fn test_set_in_bounds ( )
	{
		let mut img : BasicImage<10, 10> = BasicImage::new();
		img.set(Pixel{x: 0, y: 0}, 10);
		img.set(Pixel{x: 9, y: 9}, 11);
		assert_eq!(10, img.get(Pixel{x: 0, y: 0}));
		assert_eq!(11, img.get(Pixel{x: 9, y: 9}));
	}

	#[test]
	#[should_panic]
	fn test_set_out_of_bounds ( )
	{
		let mut img : BasicImage<10, 10> = BasicImage::new();
		img.set(Pixel{x: 10, y: 10}, 0);
	}


//											~ width ~											 //
	#[test]
	fn test_width ( )
	{
		let img : BasicImage<10, 11> = BasicImage::new();
		assert_eq!(10, img.width());
	}

//											~ height ~											 //
	#[test]
	fn test_height ( )
	{
		let img : BasicImage<10, 11> = BasicImage::new();
		assert_eq!(11, img.height());
	}


}
