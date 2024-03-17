//! Implementation for ImageC.
use crate::util::aliases::Byte;
use crate::util::units::Pixel;
use crate::image_processing::ImageC;
use crate::image_processing::Image;
impl ImageC
{
//###############################################################################################//
//										---	Constructors ---
//###############################################################################################//

	/// When passing the c image to rust, use this.
	/// Or just call it from a c struct.
	pub fn new ( ptr: *mut Byte, size: Pixel ) -> ImageC	
	{	
		ImageC { img: ptr, width: size.x, height: size.y }
	}
}


impl Image for ImageC
{
//###############################################################################################//
//										---	Accessors ---
//###############################################################################################//

	/// Returns the pixel value at the current position.
	/// IF X EXCEEDS WIDTH, IT WILL OVERFLOW!
	/// # Arguments
	/// * `pixel` - The pixel.
	fn get ( &self, pixel : Pixel ) -> Byte
	{
		unsafe
		{
			*self.img.add(pixel.y * self.width + pixel.x)
		}
	}
	
	/// Sets the pixel value at the current position.
	/// # Arguments
	///	* `pixel` - The pixel.
	/// * `value` - The value to set.
	fn set ( &mut self, pixel: Pixel, value: Byte )
	{
		unsafe
		{
			*self.img.add(pixel.y * self.width + pixel.x) = value;
		}
	}



	/// Returns the width of the image.
	///	# Returns
	///	The width of the image.
	fn width ( &self ) -> usize	    {  return self.width;   }


	/// Returns the height of the image.
	///	# Returns
	///	The height of the image.
	fn height ( &self ) -> usize	{  return self.height;	 }
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
	use image_processing::ImageC;
	use image_processing::Image;
	
//###############################################################################################//
//
//										C Image
// pub fn new         ( *mut Byte, Pixel ) -> Self
// pub fn get         ( &self, x: usize, y: usize ) -> Byte 
// pub fn set         ( &self, x: usize, y: usize, Byte ) 
// pub fn width       ( &self ) -> usize
// pub fn height      ( &self ) -> usize
// pub fn valid_pixel ( &self, Pixel ) -> bool
//
//###############################################################################################//
//											~ new ~												 //
	#[test]
	fn test_new ( )
	{
		let mut image = [0, 1, 2, /* \n */10, 11, 12];
		let size  = Pixel{x: 3, y: 2};
		let img = ImageC::new(image.as_mut_ptr(), size);
		assert_eq!(img.width, 3);
		assert_eq!(img.height, 2);
		unsafe
		{
			assert_eq!(*img.img, 0);
			assert_eq!(*img.img.add(1), 1);
			assert_eq!(*img.img.add(3), 10);
		}
	}


//											~ get ~												 //
	#[test]
	fn test_get_in_bounds ( )
	{
		let mut image = [0, 1, 2, /* \n */10, 11, 12];
		let size  = Pixel{x: 3, y: 2};
		let img = ImageC::new(image.as_mut_ptr(), size);
		
		assert_eq!(img.get(Pixel{x: 0, y: 0}), 0);
		assert_eq!(img.get(Pixel{x: 1, y: 0}), 1);
		assert_eq!(img.get(Pixel{x: 2, y: 0}), 2);
		assert_eq!(img.get(Pixel{x: 0, y: 1}), 10);
		assert_eq!(img.get(Pixel{x: 1, y: 1}), 11);
		assert_eq!(img.get(Pixel{x: 2, y: 1}), 12);
	}

	#[test]
	// I guess this is a feature?
	// If x is larger than with, it will overflow to the next row.
	fn test_get_only_x ( )
	{
		let mut image = [0, 1, 2, /* \n */10, 11, 12];
		let size  = Pixel{x: 3, y: 2};
		let img = ImageC::new(image.as_mut_ptr(), size);
		
		assert_eq!(img.get(Pixel{x: 3, y: 0}), 10);
		assert_eq!(img.get(Pixel{x: 4, y: 0}), 11);
		assert_eq!(img.get(Pixel{x: 5, y: 0}), 12);
	}


//											~ set ~												 //
	#[test]
	fn test_set_in_bounds ( )
	{
		let mut image = [0, 0, 0, /* \n */0, 0, 0];
		let size  = Pixel{x: 3, y: 2};
		let mut img = ImageC::new(image.as_mut_ptr(), size);
		
		img.set(Pixel{x : 0, y: 0}, 1);  assert_eq!(img.get(Pixel{x: 0, y: 0}), 1);
		img.set(Pixel{x : 1, y: 0}, 2);  assert_eq!(img.get(Pixel{x: 1, y: 0}), 2);
		img.set(Pixel{x : 2, y: 0}, 3);  assert_eq!(img.get(Pixel{x: 2, y: 0}), 3);
		img.set(Pixel{x : 0, y: 1}, 11); assert_eq!(img.get(Pixel{x: 0, y: 1}), 11);
		img.set(Pixel{x : 1, y: 1}, 12); assert_eq!(img.get(Pixel{x: 1, y: 1}), 12);
		img.set(Pixel{x : 2, y: 1}, 13); assert_eq!(img.get(Pixel{x: 2, y: 1}), 13);
	}
	
	#[test]
	// I guess this is a feature?
	// If x is larger than with, it will overflow to the next row.
	fn test_set_only_x ( )
	{
		let mut image = [0, 1, 2, /* \n */10, 11, 12];
		let size  = Pixel{x: 3, y: 2};
		let mut img = ImageC::new(image.as_mut_ptr(), size);
		
		img.set(Pixel{x : 3, y: 0}, 1);  assert_eq!(img.get(Pixel{x: 0, y: 1}), 1);
		img.set(Pixel{x : 4, y: 0}, 2);  assert_eq!(img.get(Pixel{x: 1, y: 1}), 2);
		img.set(Pixel{x : 5, y: 0}, 3);  assert_eq!(img.get(Pixel{x: 2, y: 1}), 3);
	}

	
	
	//											~ width ~											 //
	#[test]
	fn test_width ( )
	{
		let mut image = [0, 1, 2, /* \n */10, 11, 12];
		let size  = Pixel{x: 3, y: 2};
		let img = ImageC::new(image.as_mut_ptr(), size);
		assert_eq!(img.width(), size.x);		
	}
	
	//											~ height ~											 //
	#[test]
	fn test_height ( )
	{
		let mut image = [0, 1, 2, /* \n */10, 11, 12];
		let size  = Pixel{x: 3, y: 2};
		let img = ImageC::new(image.as_mut_ptr(), size);
		assert_eq!(img.height(), size.y);		
	}


}
