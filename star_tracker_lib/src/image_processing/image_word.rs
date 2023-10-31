//! Implementation for ImageWord.
use crate::core_include::*;

use crate::util::aliases::Decimal;
use crate::util::aliases::Byte;
use crate::util::units::Pixel;
use crate::util::err::Errors;
use crate::util::err::Error;
use crate::util::word::WordList;
use crate::util::word::WordSize;
use crate::image_processing::ImageWord;
use crate::image_processing::Image;

/// Images store pixels as Bytes (8 bit).  
/// Computers store variables in "words", this is based on the architecture of your machine.
/// An 8 bit microcontroller will store images efficiently as the word and pixel size are the same.  
/// However, if you have a 32 bit microcontroller, each pixel is 4x the size. For a 2d image, this will take up 16x the space.
///
/// This image stores a set of bytes in each word ensuring efficient storage.  
/// Unfortunately, it will be slightly slower to run.  
///
/// This macro stores the image in a single Vec which requires the "nix" or "test" flag.  
///
/// # Arguments
/// * `image_size`   - (Pixel) The pixel dimensions of the image.
/// * `word_size`    - (usize) The number of bits in each word (the architecture of your machine).
/// * `nibble_bits`  - (usize) The number of bits in each pixel (usually 8 bit).
///
/// # Example
/// ```
/// use star_tracker_lib::create_image_word_nix;
/// use star_tracker_lib::util::aliases::Byte;
/// use star_tracker_lib::util::units::Pixel;
/// use star_tracker_lib::util::word::WordList;
/// use star_tracker_lib::util::word::WordSize;
/// use star_tracker_lib::image_processing::ImageWord;
///
/// let word_size  = 32; // 32 bis
/// let pixel_size = 8;  // 8 bits. 
///	let size = Pixel{x: 10, y: 11};
///	let img_word  = create_image_word_nix!(size, word_size, pixel_size); // Heres your image, have fun.
/// ```
#[macro_export]
#[cfg(any(test, feature = "nix"))] 
macro_rules! create_image_word_nix {
	( $image_size:expr, $word_size:expr, $nibble_bits:expr ) => 
	{
		{
			let pixels_num  = ($image_size).x * ($image_size).y;
			let num_nibbles = WordList::nibbles_in_word($word_size, $nibble_bits);
			let array_size  = WordList::array_size($word_size, $nibble_bits, pixels_num);
			let word_size   = WordSize
			{
				word_size: $word_size, 
				nibbles_num: num_nibbles, 
				nibbles_size: $nibble_bits
			};
			
			ImageWord
			{
				img: &mut WordList
				{
					array: &mut vec![0; array_size], 
					size: word_size
				}, 
				size: $image_size
			}
		}
	}
}


/// Images store pixels as Bytes (8 bit).  
/// Computers store variables in "words", this is based on the architecture of your machine.
/// An 8 bit microcontroller will store images efficiently as the word and pixel size are the same.  
/// However, if you have a 32 bit microcontroller, each pixel is 4x the size. For a 2d image, this will take up 16x the space.
///   
/// This image stores a set of bytes in each word ensuring efficient storage.  
/// Unfortunately, it will be slightly slower to run.  
///   
/// This macro stores the image in a single fixed array and the size must be passed as a constant.  
///   
/// # Arguments
/// * `image_size` - (const Pixel) The pixel dimensions of the image.
/// * `word_bits`  - (usize) The number of bits in each word (the architecture of your machine).
/// * `nibble_bits`  - (usize) The number of bits in each pixel (usually 8 bit).
///
/// # Example
/// ```
/// use star_tracker_lib::create_image_word;
/// use star_tracker_lib::util::aliases::Byte;
/// use star_tracker_lib::util::units::Pixel;
/// use star_tracker_lib::util::word::WordList;
/// use star_tracker_lib::util::word::WordSize;
/// use star_tracker_lib::image_processing::ImageWord;
///
/// const pixel_size: usize = 8; // 8 bits. 
/// const word_size:  usize = 32;
///	const size: Pixel = Pixel{x: 10, y: 11};
///	let img_word  = create_image_word!(size, word_size, pixel_size); // Heres your image, have fun.
/// ```
#[macro_export]
macro_rules! create_image_word {
	( $image_size:expr, $word_bits:expr, $nibble_bits:expr ) => 
	{
		{
			const PIXELS_NUM : usize = ($image_size).x * ($image_size).y;
			const NUM_NIBBLES: usize = WordList::nibbles_in_word($word_bits, $nibble_bits);
			const ARRAY_SIZE : usize = WordList::array_size($word_bits, $nibble_bits, PIXELS_NUM);
			let create_image_word_size   = WordSize
			{
				word_size:    $word_bits, 
				nibbles_num:  NUM_NIBBLES, 
				nibbles_size: $nibble_bits
			};

			ImageWord
			{
				img: &mut WordList
				{
					array: &mut [0; ARRAY_SIZE], 
					size: create_image_word_size
				}, 
				size: $image_size
			}
		}
	}
}


impl <'a> Image for ImageWord <'a>
{
//###############################################################################################//
//										---	Accessors ---
//###############################################################################################//

	/// Returns the pixel value at the current position.
	/// # Arguments
	/// * `pixel` - The pixel.
	fn get ( &self, pixel : Pixel ) -> Byte
	{
		unsafe
		{
			return self.img.get(pixel.y * self.size.x + pixel.x) as Byte;
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
			self.img.set(pixel.y * self.size.x + pixel.x, value as usize);
		}
	}



	/// Returns the width of the image.
	fn width ( &self ) -> usize	 { return self.size.x;  }


	/// Returns the height of the image.
	fn height ( &self ) -> usize { return self.size.y; }


	/// Same as Image::copy_from but ensures the bits fit inside the nibble.  
	/// If the nibble is less than a byte, it will not work properly.  
	/// To shrink the pixels, 
	fn copy_from ( &mut self, from: &dyn Image ) -> Error<()>
	{
		if self.width() != from.width() || self.height() != from.height()
		{
			return Result::Err(Errors::InvalidSize);
		}
		let diff = self.bits() as isize - from.bits() as isize;
		let left = 0 < diff;
		let diff = diff.abs();
		for xx in 0..self.width()
		{
			for yy in 0..self.height()
			{
				let px = Pixel{x: xx, y: yy};
				if left
				{
					self.set(px, (from.get(px) << diff) as Byte);
				}
				else
				{
					self.set(px, (from.get(px) >> diff) as Byte);
				}
			}
		}
		return Result::Ok(());
	} 


	/// Returns the number of bits in each nibble.  
	fn bits ( &self ) -> usize { self.img.size.nibbles_size }
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
	use crate::util::err::Errors;
	use crate::util::err::Error;
	use crate::util::word::WordList;
	use crate::util::word::WordSize;
	use image_processing::ImageWord;
	use image_processing::ImageBasic;
	use image_processing::Image;
	use image_processing::Byte;

	use crate::create_image_basic;

//###############################################################################################//
//
//										Word Image
//
// macro create_image_word_nix
// macro create_image_word
// 
// pub fn new         (WordList, Pixel) -> Self
// pub fn get         ( &self, x: usize, y: usize ) -> Byte 
// pub fn set         ( &self, x: usize, y: usize, Byte ) 
// pub fn width       ( &self ) -> usize
// pub fn height      ( &self ) -> usize
//
// pub fn copy_from   ( &mut self, &dyn Image ) -> Error<()>
// pub fn bits        ( &self ) -> usize
//
//###############################################################################################//


//											~ test_create_vec ~									 //

	#[test]
	fn test_create_nix_0x0 ( )
	{
		let size = Pixel{x: 0, y: 0};
		let img_word  = create_image_word_nix!(size, 32, 8);
		assert_eq!(img_word.size, size);
		assert_eq!(img_word.img.array.size(), 0);
	}

	#[test]
	fn test_create_nix_1x1 ( )
	{
		let size = Pixel{x: 1, y: 1};
		let img_word  = create_image_word_nix!(size, 32, 8);
		assert_eq!(img_word.size, size);
		assert_eq!(img_word.img.array.size(), 1);
	}

	#[test]
	fn test_create_nix_multiple_x ( )
	{
		let size = Pixel{x: 4, y: 1};
		let img_word  = create_image_word_nix!(size, 32, 8);
		assert_eq!(img_word.size, size);
		assert_eq!(img_word.img.array.size(), 1);
	}

	#[test]
	fn test_create_nix_multiple_y ( )
	{
		let size = Pixel{x: 1, y: 4};
		let img_word  = create_image_word_nix!(size, 32, 8);
		assert_eq!(img_word.size, size);
		assert_eq!(img_word.img.array.size(), 1);
	}


	#[test]
	fn test_create_nix ( )
	{
		let size = Pixel{x: 10, y: 11};
		let img_word  = create_image_word_nix!(size, 32, 8);

		let nibs_per_word  = 4.0;
		let nibs_num_total = (size.x * size.y) as f32;
		assert_eq!(img_word.size, size);
		assert_eq!(img_word.img.array.size(), (nibs_num_total / nibs_per_word).ceil() as usize);
	}

	#[test]
	fn test_create_nix_64_bit ( )
	{
		let size = Pixel{x: 10, y: 11};
		let img_word  = create_image_word_nix!(size, 64, 8);

		let nibs_per_word  = 8.0;
		let nibs_num_total = (size.x * size.y) as f32;
		assert_eq!(img_word.size, size);
		assert_eq!(img_word.img.array.size(), (nibs_num_total / nibs_per_word).ceil() as usize);
	}


	#[test]
	fn test_create_nix_64_bit_array_size ( )
	{
		let size = Pixel{x: 100, y: 110};
		let img_word  = create_image_word_nix!(size, 64, 8);
		assert_eq!(img_word.img.array.size(), 100*110/8);
	}



//											~ test_create ~										 //

	#[test]
	fn test_create_0x0 ( )
	{
		const SIZE: Pixel = Pixel{x: 0, y: 0};
		let img_word  = create_image_word!(SIZE, 32, 8);
		assert_eq!(img_word.size, SIZE);
		assert_eq!(img_word.img.array.size(), 0);
	}
	
	#[test]
	fn test_create_1x1 ( )
	{
		const SIZE: Pixel = Pixel{x: 1, y: 1};
		let img_word  = create_image_word!(SIZE, 32, 8);
		assert_eq!(img_word.size, SIZE);
		assert_eq!(img_word.img.array.size(), 1);
	}
	
	#[test]
	fn test_create_multiple_x ( )
	{
		const SIZE: Pixel = Pixel{x: 4, y: 1};
		let img_word  = create_image_word_nix!(SIZE, 32, 8);
		assert_eq!(img_word.size, SIZE);
		assert_eq!(img_word.img.array.size(), 1);
	}
	
	#[test]
	fn test_create_multiple_y ( )
	{
		const SIZE: Pixel = Pixel{x: 1, y: 4};
		let img_word  = create_image_word!(SIZE, 32, 8);
		assert_eq!(img_word.size, SIZE);
		assert_eq!(img_word.img.array.size(), 1);
	}
	
	
	#[test]
	fn test_create ( )
	{
		const SIZE: Pixel = Pixel{x: 10, y: 11};
		let img_word  = create_image_word!(SIZE, 32, 8);
		
		let nibs_per_word  = 4.0;
		let nibs_num_total = (SIZE.x * SIZE.y) as f32;
		assert_eq!(img_word.size, SIZE);
		assert_eq!(img_word.img.array.size(), (nibs_num_total / nibs_per_word).ceil() as usize);
	}
	
	#[test]
	fn test_create_64_bit ( )
	{
		const SIZE: Pixel = Pixel{x: 10, y: 11};
		let img_word  = create_image_word!(SIZE, 64, 8);

		let nibs_per_word  = 8.0;
		let nibs_num_total = (SIZE.x * SIZE.y) as f32;
		assert_eq!(img_word.size, SIZE);
		assert_eq!(img_word.img.array.size(), (nibs_num_total / nibs_per_word).ceil() as usize);
	}


//											~ get ~												 //
	#[test]
	fn test_get_in_bounds ( )
	{
		let size = Pixel{x: 3, y: 2};
		let img  = create_image_word_nix!(size, 32, 8);	

		unsafe {
		// Setting each nibble in each word.
		img.img.set(0, 1);
		img.img.set(1, 2);
		img.img.set(2, 3);
		img.img.set(3, 4);
		img.img.set(4, 5);
		img.img.set(5, 6);
		}

		assert_eq!(img.get(Pixel{x: 0, y: 0}), 1);
		assert_eq!(img.get(Pixel{x: 1, y: 0}), 2);
		assert_eq!(img.get(Pixel{x: 2, y: 0}), 3);
		assert_eq!(img.get(Pixel{x: 0, y: 1}), 4);
		assert_eq!(img.get(Pixel{x: 1, y: 1}), 5);
		assert_eq!(img.get(Pixel{x: 2, y: 1}), 6);
	}


	#[test]
	fn test_get_in_bounds_4_bit ( )
	{
		let size = Pixel{x: 3, y: 2};
		let img  = create_image_word_nix!(size, 32, 4);	

		unsafe {
		// Setting each nibble in each word.
		img.img.set(0, 1);
		img.img.set(1, 2);
		img.img.set(2, 3);
		img.img.set(3, 4);
		img.img.set(4, 5);
		img.img.set(5, 6);
		}

		assert_eq!(img.get(Pixel{x: 0, y: 0}), 1);
		assert_eq!(img.get(Pixel{x: 1, y: 0}), 2);
		assert_eq!(img.get(Pixel{x: 2, y: 0}), 3);
		assert_eq!(img.get(Pixel{x: 0, y: 1}), 4);
		assert_eq!(img.get(Pixel{x: 1, y: 1}), 5);
		assert_eq!(img.get(Pixel{x: 2, y: 1}), 6);
	}


//											~ set ~												 //
	#[test]
	fn test_set_in_bounds ( )
	{
		let size = Pixel{x: 3, y: 2};
		let mut img  = create_image_word_nix!(size, 32, 8);	

		img.set(Pixel{x: 0, y: 0}, 1);
		img.set(Pixel{x: 1, y: 0}, 2);
		img.set(Pixel{x: 2, y: 0}, 3);
		img.set(Pixel{x: 0, y: 1}, 4);
		img.set(Pixel{x: 1, y: 1}, 5);
		img.set(Pixel{x: 2, y: 1}, 6);
		
		// Setting each nibble in each word.
		assert_eq!(img.img.array.get(0) >> (Byte::BITS as usize * 0) & Byte::MAX as usize, 1);
		assert_eq!(img.img.array.get(0) >> (Byte::BITS as usize * 1) & Byte::MAX as usize, 2);
		assert_eq!(img.img.array.get(0) >> (Byte::BITS as usize * 2) & Byte::MAX as usize, 3);
		assert_eq!(img.img.array.get(0) >> (Byte::BITS as usize * 3) & Byte::MAX as usize, 4);

		assert_eq!(img.img.array.get(1) >> (Byte::BITS as usize * 0) & Byte::MAX as usize, 5);
		assert_eq!(img.img.array.get(1) >> (Byte::BITS as usize * 1) & Byte::MAX as usize, 6);
	}


	#[test]
	fn test_set_in_bounds_4_bit ( )
	{
		let size = Pixel{x: 3, y: 4};
		let mut img  = create_image_word_nix!(size, 32, 4);	

		img.set(Pixel{x: 0, y: 0}, 1);
		img.set(Pixel{x: 1, y: 0}, 2);
		img.set(Pixel{x: 2, y: 0}, 3);
		img.set(Pixel{x: 0, y: 1}, 4);
		img.set(Pixel{x: 1, y: 1}, 5);
		img.set(Pixel{x: 2, y: 1}, 6);
		img.set(Pixel{x: 0, y: 2}, 7);
		img.set(Pixel{x: 1, y: 2}, 8);
		img.set(Pixel{x: 2, y: 2}, 9);
		img.set(Pixel{x: 0, y: 3}, 10);
		img.set(Pixel{x: 1, y: 3}, 0b1111);
		img.set(Pixel{x: 2, y: 3}, 0b1111111);
		
		println!("{:b} {:b}", img.img.array.get(0), img.img.array.get(1));

		// Setting each nibble in each word.
		assert_eq!(img.img.array.get(0) >> 0  & 0b1111, 1);
		assert_eq!(img.img.array.get(0) >> 4  & 0b1111, 2);
		assert_eq!(img.img.array.get(0) >> 8  & 0b1111, 3);
		assert_eq!(img.img.array.get(0) >> 12 & 0b1111, 4);
		assert_eq!(img.img.array.get(0) >> 16 & 0b1111, 5);
		assert_eq!(img.img.array.get(0) >> 20 & 0b1111, 6);
		assert_eq!(img.img.array.get(0) >> 24 & 0b1111, 7);
		assert_eq!(img.img.array.get(0) >> 28 & 0b1111, 8);

		assert_eq!(img.img.array.get(1) >> 0  & 0b1111, 9);
		assert_eq!(img.img.array.get(1) >> 4  & 0b1111, 10);
		assert_eq!(img.img.array.get(1) >> 8  & 0b1111, 0b1111);
		assert_eq!(img.img.array.get(1) >> 12 & 0b1111, 0b00001111);
	}





//											~ width ~											 //
	
	#[test]
	fn test_width ( )
	{
		let size = Pixel{x: 100, y: 10};
		let img  = create_image_word_nix!(size, 32, 8);
		assert_eq!(100, img.width());
	}
	

//											~ height ~											 //
	#[test]
	fn test_height ( )
	{
		let size = Pixel{x: 100, y: 10};
		let img  = create_image_word_nix!(size, 32, 8);
		assert_eq!(10, img.height());
	}
	
	
	
	
//											~ copy_from ~										 //
	#[test]
	fn test_copy_from ( )
	{
		let size = Pixel{x: 5, y: 10};
		let mut img_1  = create_image_word_nix!(size, 32, 8); // 4 nibs
		let mut img_2  = create_image_word_nix!(size, 32, 6); // 5 nibs
		let mut img_3  = create_image_word_nix!(size, 32, 4); // 8 nibs

		img_1.set(Pixel{x: 0, y: 0}, 255);
		img_1.set(Pixel{x: 1, y: 1}, 32);
		assert_eq!(img_2.copy_from(&mut img_1), Ok(()));
		assert_eq!(img_2.get(Pixel{x: 0, y: 0}), 63);
		assert_eq!(img_2.get(Pixel{x: 1, y: 1}), 8); 
		
		assert_eq!(img_3.copy_from(&mut img_2), Ok(()));
		assert_eq!(img_3.get(Pixel{x: 0, y: 0}), 15);
		assert_eq!(img_3.get(Pixel{x: 1, y: 1}), 2);
		
		assert_eq!(img_1.copy_from(&mut img_3), Ok(()));
		assert_eq!(img_1.get(Pixel{x: 0, y: 0}), 0b11110000);
		assert_eq!(img_1.get(Pixel{x: 1, y: 1}), 0b00100000);
		
	}
	
	#[test]
	fn test_copy_from_error ( )
	{
		let mut img_1  = create_image_word_nix!(Pixel{x: 10, y: 5}, 32, 8);
		let mut img_2  = create_image_word_nix!(Pixel{x: 11, y: 5}, 32, 4);
		assert_eq!(img_1.copy_from(&img_2), Err(Errors::InvalidSize));
		assert_eq!(img_2.copy_from(&img_1), Err(Errors::InvalidSize));
	}
	



//											~ bits ~											 //
	#[test]
	fn test_bits ( )
	{
		let img = create_image_word_nix!(Pixel{x: 10, y: 59}, 32, 8);
		assert_eq!(img.bits(), 8);
		let img = create_image_word_nix!(Pixel{x: 23, y: 44}, 32, 5);
		assert_eq!(img.bits(), 5);
		let img = create_image_word_nix!(Pixel{x: 23, y: 44}, 32, 4);
		assert_eq!(img.bits(), 4);
		let img = create_image_word_nix!(Pixel{x: 23, y: 44}, 32, 2);
		assert_eq!(img.bits(), 2);
	}

}