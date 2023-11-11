//! Implementation for BasicImage.
use crate::core_include::*;

use crate::util::aliases::Byte;
use crate::util::units::Pixel;
use crate::image_processing::RefImage;
use crate::image_processing::Image;
impl <'a, const WIDTH : usize, const HEIGHT : usize> RefImage <'a, WIDTH, HEIGHT>
{
//###############################################################################################//
//										---	Constructors ---
//###############################################################################################//

	/// Borrows the image.
	pub fn new ( img: &'a mut [[Byte; WIDTH]; HEIGHT] ) -> Self	{ Self { img: img } }
}


impl <'a, const WIDTH : usize, const HEIGHT : usize> Image for RefImage <'a, WIDTH, HEIGHT>
{
//###############################################################################################//
//										---	Accessors ---
//###############################################################################################//

	/// Returns the pixel value at the current position.
	/// # Arguments
	/// * `pixel` - The pixel.
	fn get ( &self, pixel : Pixel ) -> Byte
	{
		return self.img[pixel.y][pixel.x].clone();
	}

	/// Sets the pixel value at the current position.
	/// # Arguments
	///	* `pixel` - The pixel.
	/// * `value` - The value to set.
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
	use crate::util::aliases::Byte;
	use crate::util::units::Pixel;
	use image_processing::RefImage;
	use image_processing::Image;
	
	fn image ( ) -> [[Byte; 3]; 3]
	{
		[
		[11, 12, 13],
		[21, 22, 23],
		[31, 32, 33]
		]
	}

//###############################################################################################//
//
//										Ref Image
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
		let mut im = image();
		let img = RefImage::new(&mut im);
		assert_eq!(11, img.get(Pixel{x: 0, y: 0}));
		assert_eq!(33, img.get(Pixel{x: 2, y: 2}));
	}
	
	
	#[test]
	#[should_panic]
	fn test_get_out_of_bounds ( )
	{
		let mut im = image();
		let img = RefImage::new(&mut im);
		img.get(Pixel{x: 4, y: 4});
	}
	
	
	
	//											~ set ~												 //
	#[test]
	fn test_set_in_bounds ( )
	{
		let mut im = image();
		let mut img = RefImage::new(&mut im);
		img.set(Pixel{x: 0, y: 0}, 10);
		img.set(Pixel{x: 2, y: 2}, 11);
		assert_eq!(10, img.get(Pixel{x: 0, y: 0}));
		assert_eq!(11, img.get(Pixel{x: 2, y: 2}));
	}
	
	#[test]
	#[should_panic]
	fn test_set_out_of_bounds ( )
	{
		let mut im = image();
		let mut img = RefImage::new(&mut im);
		img.set(Pixel{x: 3, y: 3}, 0);
	}
	
	
	//											~ width ~											 //
	#[test]
	fn test_width ( )
	{
		let mut im = image();
		let img = RefImage::new(&mut im);
		assert_eq!(3, img.width());
	}
	
	//											~ height ~											 //
	#[test]
	fn test_height ( )
	{
		let mut im = image();
		let img = RefImage::new(&mut im);
		assert_eq!(3, img.height());
	}


}
