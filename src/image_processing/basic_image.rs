//! Implementation for BasicImage
use crate::util::aliases::*;
use super::{BasicImage, Image};
impl <const WIDTH : usize, const HEIGHT : usize> BasicImage <WIDTH, HEIGHT>
{
//###############################################################################################//
//										---	Constructors ---
//###############################################################################################//

	/// Creates an image with the default value of 0.
	/// # Example
	/// ```
	/// use star_tracker::image_processing::{BasicImage, Image};
	/// let img : BasicImage<1, 1> = BasicImage::new();
	/// assert_eq!(img.get(0, 0), 0);
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
	/// * `x` - The column.
	/// * `y` - The row.
	///
	///	# Example
	/// ```
	/// use star_tracker::image_processing::{BasicImage, Image};
	///	let img : BasicImage<1, 1> = BasicImage::new();
	/// assert_eq!(img.get(0, 0), 0);
	/// ```
	fn get ( &self, x : usize, y : usize ) -> Byte
	{
		assert!((x < self.width() && y < self.height()), "Out of bounds");
		return self.img[y][x].clone();
	}

	/// Sets the pixel value at the current position.
	/// # Arguments
	///	* `x`     - The column.
	/// * `y`     - The row.
	/// * `value` - The value to set.
	///
	///	# Example
	/// ```
	/// use star_tracker::image_processing::{BasicImage, Image};
	///	let mut  img : BasicImage<1, 1> = BasicImage::new();
	/// img.set(0, 0, 10);
	/// assert_eq!(img.get(0, 0), 10);
	/// ```
	fn set ( &mut self, x : usize, y : usize, value: Byte )
	{
		assert!((x < self.width() && y < self.height()), "Out of bounds");
		self.img[y][x] = value;
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


	/// True if the get/set will not cause a panic.
	/// # Arguments
	/// * `x` - The x position.
	/// * `y` - The y position.
	///
	/// # Returns
	/// True if access is safe.
	///
	/// # Example
	/// ```
	/// use star_tracker::image_processing::{BasicImage, Image};
	///	let img : BasicImage<10, 10> = BasicImage::new();
	/// assert!(img.valid_pixel(0, 0));
	/// assert!(img.valid_pixel(9, 9));
	/// assert!(img.valid_pixel(0, 9));
	/// assert!(img.valid_pixel(9, 0));
	/// assert!(!img.valid_pixel(10, 10)); // Would panic if get or set.
	/// ```
	fn valid_pixel ( &self, x : usize, y : usize ) -> bool 
	{ return x < self.width() && y < self.height()	} // unsigned variables cant be -'ve.


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
	use image_processing::basic_image::*;

//
//  get ( x: usize, y: usize ) -> Byte
//
	#[test]
	fn test_get_in_bounds ( )
	{
		let img : BasicImage<10, 10> = BasicImage::new();
		assert_eq!(0, img.get(0, 0));
		assert_eq!(0, img.get(9, 9));
	}


	#[test]
	#[should_panic = "Out of bounds"]
	fn test_get_out_of_bounds ( )
	{
		let img : BasicImage<10, 10> = BasicImage::new();
		img.get(10, 10);
	}



//
// set ( x: usize, y: usize, value: byte )
//

	#[test]
	fn test_set_in_bounds ( )
	{
		let mut img : BasicImage<10, 10> = BasicImage::new();
		img.set(0, 0, 10);
		img.set(9, 9, 11);
		assert_eq!(10, img.get(0, 0));
		assert_eq!(11, img.get(9, 9));
	}

	#[test]
	#[should_panic = "Out of bounds"]
	fn test_set_out_of_bounds ( )
	{
		let mut img : BasicImage<10, 10> = BasicImage::new();
		img.set(10, 10, 0);
	}


	//
	// width() -> usize
	//

	#[test]
	fn test_width ( )
	{
		let img : BasicImage<10, 11> = BasicImage::new();
		assert_eq!(10, img.width());
	}


	//
	// height() -> usize
	//

	#[test]
	fn test_height ( )
	{
		let img : BasicImage<10, 11> = BasicImage::new();
		assert_eq!(11, img.height());
	}


}
