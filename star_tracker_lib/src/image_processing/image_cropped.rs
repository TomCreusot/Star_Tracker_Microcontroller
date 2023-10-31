//! Implementation for ImageBasic.
use crate::core_include::*;

use crate::util::aliases::Byte;
use crate::util::units::Pixel;
use crate::image_processing::ImageCropped;
use crate::image_processing::Image;




impl <'a> ImageCropped <'a>
{
//###############################################################################################//
//										---	Constructors ---
//###############################################################################################//

	/// This pretends an image is larger than it is.  
	/// It allows you to crop an image down while keeping the position of the pixels where they would be.  
	/// 1. Crop an image from the center.
	/// 2. Input the image into this class with the size.
	/// # Example
	/// ```
	/// use star_tracker_lib::image_processing::ImageCropped;
	/// use star_tracker_lib::image_processing::ImageBasic;
	/// use star_tracker_lib::image_processing::Image;
	/// use star_tracker_lib::util::units::Pixel;
	/// use star_tracker_lib::create_image_basic;
	/// 
	/// let mut img = create_image_basic!(400, 400);
	/// img.set(Pixel{x: 0, y: 0}, 10);
	///
	/// let mut img = ImageCropped::new(&mut img, Pixel{x: 808, y: 608});
	/// assert_eq!(img.get(Pixel{x: (808 / 4), y: (608 / 4)}), 10);
	/// ```
	pub fn new ( img: &'a mut dyn Image, size_fake: Pixel ) -> Self 
	{ 
		Self 
		{ 
			img: img, 
			size_fake: size_fake 
		} 
	}
}


impl <'a> Image for ImageCropped<'a>
{
//###############################################################################################//
//										---	Accessors ---
//###############################################################################################//

	/// Returns the pixel value at the current position.
	/// # Arguments
	/// * `pixel` - The pixel.
	fn get ( &self, pixel : Pixel ) -> Byte
	{
		let padding_x = self.size_fake.x.div_ceil(4);
		let padding_y = self.size_fake.y.div_ceil(4);

		if padding_x <= pixel.x && pixel.x < self.img.width()  + padding_x
		&& padding_y <= pixel.y && pixel.y < self.img.height() + padding_y
		{
			return self.img.get(Pixel{x: pixel.x - padding_x, y: pixel.y - padding_y});
		}
		else
		{
			return 0;
		}
	}
	
	/// Sets the pixel value at the current position.
	/// # Arguments
	///	* `pixel` - The pixel.
	/// * `value` - The value to set.
	fn set ( &mut self, pixel: Pixel, value: Byte )
	{
		let padding_x = self.size_fake.x.div_ceil(4);
		let padding_y = self.size_fake.y.div_ceil(4);

		if padding_x <= pixel.x && pixel.x < self.img.width()  + padding_x
		&& padding_y <= pixel.y && pixel.y < self.img.height() + padding_y
		{
			self.img.set(Pixel{x: pixel.x - padding_x, y: pixel.y - padding_y}, value);
		}
	}



	/// Returns the width of the image.
	///	# Returns
	///	The width of the image.
	fn width ( &self ) -> usize	    { return self.size_fake.x; }


	/// Returns the height of the image.
	///	# Returns
	///	The height of the image.
	fn height ( &self ) -> usize	{ return self.size_fake.y; }
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
	use crate::create_image_basic;
	use crate::util::units::Pixel;
	use image_processing::ImageBasic;
	use image_processing::ImageCropped;
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
	fn test_get_even_even ( )
	{
		let mut img = create_image_basic!(4, 4);
	
		for x in 0..img.width() { for y in 0..img.height() {
			img.set(Pixel{x: x, y: y}, ((x + 1) * 10 + y + 1) as Byte);
		} }

		let img = ImageCropped::new(&mut img, Pixel{x: 10, y: 10});

		// 0 1 2 3 4 5 6 7 8 9
		// x x x . . . . x x x
		for x in 0..10 { assert_eq!(img.get(Pixel{x: x, y: 0}), 0); }
		for x in 0..10 { assert_eq!(img.get(Pixel{x: x, y: 1}), 0); }
		for x in 0..10 { assert_eq!(img.get(Pixel{x: x, y: 2}), 0); }
		for x in 0..10 { assert_eq!(img.get(Pixel{x: x, y: 7}), 0); }
		for x in 0..10 { assert_eq!(img.get(Pixel{x: x, y: 8}), 0); }
		for x in 0..10 { assert_eq!(img.get(Pixel{x: x, y: 9}), 0); }

		for y in 3..6  { assert_eq!(img.get(Pixel{x: 0, y: y}), 0); }
		for y in 3..6  { assert_eq!(img.get(Pixel{x: 1, y: y}), 0); }
		for y in 3..6  { assert_eq!(img.get(Pixel{x: 2, y: y}), 0); }
		for y in 3..6  { assert_eq!(img.get(Pixel{x: 7, y: y}), 0); }
		for y in 3..6  { assert_eq!(img.get(Pixel{x: 8, y: y}), 0); }
		for y in 3..6  { assert_eq!(img.get(Pixel{x: 9, y: y}), 0); }

		assert_eq!(img.get(Pixel{x: 3, y: 3}), 11);
		assert_eq!(img.get(Pixel{x: 4, y: 3}), 21);
		assert_eq!(img.get(Pixel{x: 5, y: 3}), 31);
		assert_eq!(img.get(Pixel{x: 6, y: 3}), 41);

		assert_eq!(img.get(Pixel{x: 3, y: 4}), 12);
		assert_eq!(img.get(Pixel{x: 4, y: 4}), 22);
		assert_eq!(img.get(Pixel{x: 5, y: 4}), 32);
		assert_eq!(img.get(Pixel{x: 6, y: 4}), 42);
		
		assert_eq!(img.get(Pixel{x: 3, y: 5}), 13);
		assert_eq!(img.get(Pixel{x: 4, y: 5}), 23);
		assert_eq!(img.get(Pixel{x: 5, y: 5}), 33);
		assert_eq!(img.get(Pixel{x: 6, y: 5}), 43);
		
		assert_eq!(img.get(Pixel{x: 3, y: 6}), 14);
		assert_eq!(img.get(Pixel{x: 4, y: 6}), 24);
		assert_eq!(img.get(Pixel{x: 5, y: 6}), 34);
		assert_eq!(img.get(Pixel{x: 6, y: 6}), 44);
	}
	

	#[test]
	fn test_get_odd_even ( )
	{
		let mut img = create_image_basic!(4, 4);
		img.set(Pixel{x:0,y:0},11);img.set(Pixel{x:1,y:0},21);img.set(Pixel{x:2,y:0},31);img.set(Pixel{x:3,y:0},41);
		img.set(Pixel{x:0,y:1},12);img.set(Pixel{x:1,y:1},22);img.set(Pixel{x:2,y:1},32);img.set(Pixel{x:3,y:1},42);
		img.set(Pixel{x:0,y:2},13);img.set(Pixel{x:1,y:2},23);img.set(Pixel{x:2,y:2},33);img.set(Pixel{x:3,y:2},43);
		img.set(Pixel{x:0,y:3},14);img.set(Pixel{x:1,y:3},24);img.set(Pixel{x:2,y:3},34);img.set(Pixel{x:3,y:3},44);
		let img = ImageCropped::new(&mut img, Pixel{x: 7, y: 7});

		// 0 1 2 3 4 5 6 // OFF CENTER 
		// x x . . . . x 
		for x in 0..7 { assert_eq!(img.get(Pixel{x: x, y: 0}), 0); }
		for x in 0..7 { assert_eq!(img.get(Pixel{x: x, y: 1}), 0); }
		for x in 0..7 { assert_eq!(img.get(Pixel{x: x, y: 6}), 0); }

		for y in 0..7  { assert_eq!(img.get(Pixel{x: 0, y: y}), 0); }
		for y in 0..7  { assert_eq!(img.get(Pixel{x: 1, y: y}), 0); }
		for y in 0..7  { assert_eq!(img.get(Pixel{x: 6, y: y}), 0); }

		assert_eq!(img.get(Pixel{x: 2, y: 2}), 11);
		assert_eq!(img.get(Pixel{x: 3, y: 2}), 21);
		assert_eq!(img.get(Pixel{x: 4, y: 2}), 31);
		assert_eq!(img.get(Pixel{x: 5, y: 2}), 41);

		assert_eq!(img.get(Pixel{x: 2, y: 3}), 12);
		assert_eq!(img.get(Pixel{x: 3, y: 3}), 22);
		assert_eq!(img.get(Pixel{x: 4, y: 3}), 32);
		assert_eq!(img.get(Pixel{x: 5, y: 3}), 42);
		
		assert_eq!(img.get(Pixel{x: 2, y: 4}), 13);
		assert_eq!(img.get(Pixel{x: 3, y: 4}), 23);
		assert_eq!(img.get(Pixel{x: 4, y: 4}), 33);
		assert_eq!(img.get(Pixel{x: 5, y: 4}), 43);
		
		assert_eq!(img.get(Pixel{x: 2, y: 5}), 14);
		assert_eq!(img.get(Pixel{x: 3, y: 5}), 24);
		assert_eq!(img.get(Pixel{x: 4, y: 5}), 34);
		assert_eq!(img.get(Pixel{x: 5, y: 5}), 44);
	}


	
	#[test]
	// The pixels will be 1 up and left then it should be.
	fn test_get_even_odd ( )
	{
		let mut img = create_image_basic!(3, 3);
		img.set(Pixel{x:0,y:0},11);img.set(Pixel{x:1,y:0},21);img.set(Pixel{x:2,y:0},31);
		img.set(Pixel{x:0,y:1},12);img.set(Pixel{x:1,y:1},22);img.set(Pixel{x:2,y:1},32);
		img.set(Pixel{x:0,y:2},13);img.set(Pixel{x:1,y:2},23);img.set(Pixel{x:2,y:2},33);
		let img = ImageCropped::new(&mut img, Pixel{x: 10, y: 10});
		
		// 0 1 2 3 4 5 6 7 8 9 // OFF CENTER
		// x x x . . . x x x x
		for x in 0..10 { assert_eq!(img.get(Pixel{x: x, y: 0}), 0); }
		for x in 0..10 { assert_eq!(img.get(Pixel{x: x, y: 1}), 0); }
		for x in 0..10 { assert_eq!(img.get(Pixel{x: x, y: 2}), 0); }
		for x in 0..10 { assert_eq!(img.get(Pixel{x: x, y: 6}), 0); }
		for x in 0..10 { assert_eq!(img.get(Pixel{x: x, y: 7}), 0); }
		for x in 0..10 { assert_eq!(img.get(Pixel{x: x, y: 8}), 0); }
		for x in 0..10 { assert_eq!(img.get(Pixel{x: x, y: 9}), 0); }

		for y in 3..6  { assert_eq!(img.get(Pixel{x: 0, y: y}), 0); }
		for y in 3..6  { assert_eq!(img.get(Pixel{x: 1, y: y}), 0); }
		for y in 3..6  { assert_eq!(img.get(Pixel{x: 2, y: y}), 0); }
		for y in 3..6  { assert_eq!(img.get(Pixel{x: 6, y: y}), 0); }
		for y in 3..6  { assert_eq!(img.get(Pixel{x: 7, y: y}), 0); }
		for y in 3..6  { assert_eq!(img.get(Pixel{x: 8, y: y}), 0); }
		for y in 3..6  { assert_eq!(img.get(Pixel{x: 9, y: y}), 0); }

		assert_eq!(img.get(Pixel{x: 3, y: 3}), 11);
		assert_eq!(img.get(Pixel{x: 4, y: 3}), 21);
		assert_eq!(img.get(Pixel{x: 5, y: 3}), 31);

		assert_eq!(img.get(Pixel{x: 3, y: 4}), 12);
		assert_eq!(img.get(Pixel{x: 4, y: 4}), 22);
		assert_eq!(img.get(Pixel{x: 5, y: 4}), 32);
		
		assert_eq!(img.get(Pixel{x: 3, y: 5}), 13);
		assert_eq!(img.get(Pixel{x: 4, y: 5}), 23);
		assert_eq!(img.get(Pixel{x: 5, y: 5}), 33);
	}


	#[test]
	fn test_get_odd_odd ( )
	{
		let mut img = create_image_basic!(3, 3);
		img.set(Pixel{x:0,y:0},11);img.set(Pixel{x:1,y:0},21);img.set(Pixel{x:2,y:0},31);
		img.set(Pixel{x:0,y:1},12);img.set(Pixel{x:1,y:1},22);img.set(Pixel{x:2,y:1},32);
		img.set(Pixel{x:0,y:2},13);img.set(Pixel{x:1,y:2},23);img.set(Pixel{x:2,y:2},33);
		let img = ImageCropped::new(&mut img, Pixel{x: 7, y: 7});

		// 0 1 2 3 4 5 6
		// x x . . . x x
		for x in 0..10 { assert_eq!(img.get(Pixel{x: x, y: 0}), 0); }
		for x in 0..10 { assert_eq!(img.get(Pixel{x: x, y: 1}), 0); }
		for x in 0..10 { assert_eq!(img.get(Pixel{x: x, y: 5}), 0); }
		for x in 0..10 { assert_eq!(img.get(Pixel{x: x, y: 6}), 0); }

		for y in 3..6  { assert_eq!(img.get(Pixel{x: 0, y: y}), 0); }
		for y in 3..6  { assert_eq!(img.get(Pixel{x: 1, y: y}), 0); }
		for y in 3..6  { assert_eq!(img.get(Pixel{x: 8, y: y}), 0); }
		for y in 3..6  { assert_eq!(img.get(Pixel{x: 9, y: y}), 0); }

		assert_eq!(img.get(Pixel{x: 2, y: 2}), 11);
		assert_eq!(img.get(Pixel{x: 3, y: 2}), 21);
		assert_eq!(img.get(Pixel{x: 4, y: 2}), 31);

		assert_eq!(img.get(Pixel{x: 2, y: 3}), 12);
		assert_eq!(img.get(Pixel{x: 3, y: 3}), 22);
		assert_eq!(img.get(Pixel{x: 4, y: 3}), 32);
		
		assert_eq!(img.get(Pixel{x: 2, y: 4}), 13);
		assert_eq!(img.get(Pixel{x: 3, y: 4}), 23);
		assert_eq!(img.get(Pixel{x: 4, y: 4}), 33);
	}
	
	
	
	//											~ set ~												 //
	#[test]
	fn test_set_even_even ( )
	{
		let mut img = create_image_basic!(2, 2);
		let mut img = ImageCropped::new(&mut img, Pixel{x: 4, y: 4});
	
		for x in 0..4 { for y in 0..4 {
			img.set(Pixel{x: x, y: y}, (x * 10 + y) as Byte);
		} }

		// 0 1 2 3
		// x . . x
		for x in 0..4 { assert_eq!(img.get(Pixel{x: x, y: 0}), 0); }
		for x in 0..4 { assert_eq!(img.get(Pixel{x: x, y: 3}), 0); }

		for y in 1..2  { assert_eq!(img.get(Pixel{x: 0, y: y}), 0); }
		for y in 1..2  { assert_eq!(img.get(Pixel{x: 3, y: y}), 0); }

		assert_eq!(img.get(Pixel{x: 1, y: 1}), 11);
		assert_eq!(img.get(Pixel{x: 1, y: 2}), 12);
		assert_eq!(img.get(Pixel{x: 2, y: 1}), 21);
		assert_eq!(img.get(Pixel{x: 2, y: 2}), 22);
	}
	

	#[test]
	fn test_set_odd_even ( )
	{
		let mut img = create_image_basic!(2, 2);
		let mut img = ImageCropped::new(&mut img, Pixel{x: 5, y: 5});	
		
		for x in 0..5 { for y in 0..5 {
			img.set(Pixel{x: x, y: y}, (x * 10 + y) as Byte);
		}}
		
		// 0 1 2 3 4 // OFF CENTER 
		// x x . . x
		for x in 0..5 { assert_eq!(img.get(Pixel{x: x, y: 0}), 0); }
		for x in 0..5 { assert_eq!(img.get(Pixel{x: x, y: 1}), 0); }
		for x in 0..5 { assert_eq!(img.get(Pixel{x: x, y: 4}), 0); }

		for y in 0..5  { assert_eq!(img.get(Pixel{x: 0, y: y}), 0); }
		for y in 0..5  { assert_eq!(img.get(Pixel{x: 1, y: y}), 0); }
		for y in 0..5  { assert_eq!(img.get(Pixel{x: 4, y: y}), 0); }

		assert_eq!(img.get(Pixel{x: 2, y: 2}), 22);
		assert_eq!(img.get(Pixel{x: 2, y: 3}), 23);
		assert_eq!(img.get(Pixel{x: 3, y: 2}), 32);
		assert_eq!(img.get(Pixel{x: 3, y: 3}), 33);
	}


	
	#[test]
	// The pixels will be 1 up and left then it should be.
	fn test_set_even_odd ( )
	{
		let mut img = create_image_basic!(3, 3);
		let mut img = ImageCropped::new(&mut img, Pixel{x: 6, y: 6});

		for x in 0..6 { for y in 0..6 {
			img.set(Pixel{x: x, y: y}, (x * 10 + y) as Byte);
		}}

		// 0 1 2 3 4 5 // OFF CENTER
		// x x . . . x
		for x in 0..5 { assert_eq!(img.get(Pixel{x: x, y: 0}), 0); }
		for x in 0..5 { assert_eq!(img.get(Pixel{x: x, y: 1}), 0); }
		for x in 0..5 { assert_eq!(img.get(Pixel{x: x, y: 5}), 0); }

		for y in 1..4  { assert_eq!(img.get(Pixel{x: 0, y: y}), 0); }
		for y in 1..4  { assert_eq!(img.get(Pixel{x: 1, y: y}), 0); }
		for y in 1..4  { assert_eq!(img.get(Pixel{x: 5, y: y}), 0); }

		assert_eq!(img.get(Pixel{x: 2, y: 2}), 22);
		assert_eq!(img.get(Pixel{x: 3, y: 2}), 32);
		assert_eq!(img.get(Pixel{x: 4, y: 2}), 42);

		assert_eq!(img.get(Pixel{x: 2, y: 3}), 23);
		assert_eq!(img.get(Pixel{x: 3, y: 3}), 33);
		assert_eq!(img.get(Pixel{x: 4, y: 3}), 43);
		
		assert_eq!(img.get(Pixel{x: 2, y: 4}), 24);
		assert_eq!(img.get(Pixel{x: 3, y: 4}), 34);
		assert_eq!(img.get(Pixel{x: 4, y: 4}), 44);
	}


	#[test]
	fn test_set_odd_odd ( )
	{
		let mut img = create_image_basic!(3, 3);
		let mut img = ImageCropped::new(&mut img, Pixel{x: 7, y: 7});

		for x in 0..7 { for y in 0..7 {
			img.set(Pixel{x: x, y: y}, (x * 10 + y) as Byte);
		}}
		// 0 1 2 3 4 5 6
		// x x . . . x x
		for x in 0..10 { assert_eq!(img.get(Pixel{x: x, y: 0}), 0); }
		for x in 0..10 { assert_eq!(img.get(Pixel{x: x, y: 1}), 0); }
		for x in 0..10 { assert_eq!(img.get(Pixel{x: x, y: 5}), 0); }
		for x in 0..10 { assert_eq!(img.get(Pixel{x: x, y: 6}), 0); }

		for y in 0..10  { assert_eq!(img.get(Pixel{x: 0, y: y}), 0); }
		for y in 0..10  { assert_eq!(img.get(Pixel{x: 1, y: y}), 0); }
		for y in 0..10  { assert_eq!(img.get(Pixel{x: 8, y: y}), 0); }
		for y in 0..10  { assert_eq!(img.get(Pixel{x: 9, y: y}), 0); }

		assert_eq!(img.get(Pixel{x: 2, y: 2}), 22);
		assert_eq!(img.get(Pixel{x: 3, y: 2}), 32);
		assert_eq!(img.get(Pixel{x: 4, y: 2}), 42);

		assert_eq!(img.get(Pixel{x: 2, y: 3}), 23);
		assert_eq!(img.get(Pixel{x: 3, y: 3}), 33);
		assert_eq!(img.get(Pixel{x: 4, y: 3}), 43);
		
		assert_eq!(img.get(Pixel{x: 2, y: 4}), 24);
		assert_eq!(img.get(Pixel{x: 3, y: 4}), 34);
		assert_eq!(img.get(Pixel{x: 4, y: 4}), 44);
	}
	
	
//											~ width ~											 //
	#[test]
	fn test_width ( )
	{
		let mut img = create_image_basic!(9, 10);
		let img = ImageCropped::new(&mut img, Pixel{x: 100, y: 10});
		assert_eq!(100, img.width());
	}
	
//											~ height ~											 //
	#[test]
	fn test_height ( )
	{
		let mut img = create_image_basic!(10, 11);
		let img = ImageCropped::new(&mut img, Pixel{x: 100, y: 50});
		assert_eq!(50, img.height());
	}


}
