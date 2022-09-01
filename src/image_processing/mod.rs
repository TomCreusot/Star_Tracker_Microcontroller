//! `image_processing` involves storing an image in a reduced size and performing basic computer vision on the image.
//! This includes a fast method of blob detection and thresholding to obtain stars.
//!
//!
//!
//!

pub mod basic_image;
pub mod blob;

use crate::util::aliases::Byte;
use crate::util::aliases::UInt;
use crate::util::aliases::Decimal;
use crate::util::units::{Pixel, Vector2};


//###############################################################################################//
//									---	Image ---
//###############################################################################################//

/// If a new image struct is to be considered in the future.
/// (This is ultimately just to stop requiring generic arguments as consts in generic is too new).
pub trait Image
{
	/// Returns the pixel value at the current position.
	/// # Arguments
	/// * `px` -  The pixel to modify..
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

	/// True if the get/set will not cause a panic.
	/// # Arguments
	/// * `px` - The pixel to modify.
	///
	/// # Returns
	/// True if access is safe.
	fn valid_pixel ( &self, px: Pixel ) -> bool
	{ return px.x < self.width() && px.y < self.height()	}

	/// Generates a brightness histogram of the image.
	/// # Arguments
	/// * `histogram` - The histogram to fill, must have a size > 0 && < 256.
	///
	/// # Example
	/// ```
	/// use star_tracker::image_processing::{BasicImage, Image};
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
	fn histogram ( &self, histogram : &mut [UInt] )
	{
		assert!(histogram.len() != 0 || histogram.len() as UInt <= Byte::MAX as UInt);
		let ratio : Decimal = (histogram.len() as Decimal) / (Byte::MAX as Decimal + 1.0);
		for y in 0..self.height()
		{
			for x in 0..self.width()
			{
				let bar : usize = ((self.get(Pixel{x, y}) as Decimal) * ratio) as usize;
				histogram[bar] += 1;
			}
		}
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
	/// use star_tracker::image_processing::{BasicImage, Image};
	/// use star_tracker::util::list::ArrayList;
	/// use star_tracker::util::aliases::UInt;
	/// use star_tracker::util::aliases::Byte;
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
}



//###############################################################################################//
//									---	Basic Image ---
//###############################################################################################//

/// This is a struct to store a raw image and can provide basic image thresholding.
///
/// # Example
/// The following example will:
/// * Create an image 10x5 in size.
/// * Set pixels (0,0) = 200, (1,1) = 100, (9,4) = 10.
/// * Create a histogram of the different pixel intensities.
/// * Provide a threshold value which isolates the background from the foreground.
/// ```
/// use star_tracker::image_processing::{Image, BasicImage};
/// use star_tracker::util::aliases::{UInt, Byte};
/// use star_tracker::util::list::ArrayList;
/// use star_tracker::util::units::Pixel;
///
/// const WIDTH : usize = 10;
/// const HEIGHT: usize = 5;
/// let mut img : BasicImage<WIDTH, HEIGHT> = BasicImage::new();     // Creates a black image.
/// assert_eq!(img.width(), WIDTH);     // The maximum width of the image.
/// assert_eq!(img.height(), HEIGHT);   // The maximum height of the image.
///
/// assert_eq!(img.get(Pixel{x: 9, y: 4}), 0);          // Every pixel defaults at 0.
/// img.set(Pixel{x: 9, y: 4}, 10);                     // Sets (9,4) to 10.
/// assert_eq!(img.get(Pixel{x: 9, y: 4}), 10);         // The pixel has been changed to 10.
/// img.set(Pixel{x: 1, y: 1}, 100);                    // Set pixel (x: 1, y: 1) to 100.
/// img.set(Pixel{x: 0, y: 0}, 200);                    // Set pixel (x: 0, y: 0) to 200.
/// img.set(Pixel{x: 0, y: 1}, 200);                    // Set pixel (x: 0, y: 1) to 200.
///
/// // Generate Histogram
/// let mut hist : [UInt; Byte::max_value() as usize + 1] = [0; Byte::MAX as usize + 1];   // Can be of any size from 0 to the max intensity.
/// img.histogram(&mut hist);
/// assert_eq!(hist[0], (HEIGHT * WIDTH) as UInt - 4);   // Everything defaults to 0.
/// assert_eq!(hist[10], 1);                             // intensity 10 has 1 pixel.
/// assert_eq!(hist[100], 1);                            // 1 pixel is at intensity of 100.
/// assert_eq!(hist[200], 2);                            // 2 pixels are at the intensity of 200.
///
/// // Thresholding
/// assert_eq!(img.percent_threshold(0.5, &hist), 1);      // Finds what is brighter than half the pixels.
/// assert_eq!(img.percent_threshold(0.0, &hist), 0);      // Finds what is brighter than 0 of the pixels.
/// assert_eq!(img.percent_threshold(1.0, &hist), 201);    // Finds what is brighter than all of the pixels.
/// ```
pub struct BasicImage <const WIDTH : usize, const HEIGHT : usize>
{
	/// The image
	img : [[Byte; WIDTH]; HEIGHT],
}


//###############################################################################################//
//										---	Blob ---
//###############################################################################################//

/// A set of connected pixels in the foreground representing a star.
/// This uses the grass fire method (iteratively find new pixels and consume them).
///
/// # Example
/// ```
/// use star_tracker::image_processing::{Blob, Image, BasicImage};
/// use star_tracker::util::{list::ArrayList, list::List, units::Vector2};
/// use star_tracker::util::units::Pixel;
///
/// let mut img : BasicImage<3, 3> = BasicImage::new();
/// img.set(Pixel{x: 0, y: 0}, 1); // 1 1 0
/// img.set(Pixel{x: 1, y: 0}, 1); // 1 0 0
/// img.set(Pixel{x: 0, y: 1}, 1); // 0 0 1
/// img.set(Pixel{x: 2, y: 2}, 1);
///
/// let mut lst : ArrayList<Blob, 9> = ArrayList::new();
/// const BLOB_SIZE : usize = 100; // Should be bigger than the size of a blob.
/// Blob::find_blobs::<BLOB_SIZE>(1, &mut img, &mut lst);// Consumes pixels and finds blobs.
/// assert_eq!(lst.get(0).intensity, 3);
/// assert_eq!(lst.get(0).centroid.x, 0.333333333333333333333);
/// assert_eq!(lst.get(0).centroid.y, 0.333333333333333333333);
///
/// assert_eq!(lst.get(1).intensity, 1);
/// assert_eq!(lst.get(1).centroid.x, 2.0);
/// assert_eq!(lst.get(1).centroid.y, 2.0);
///
/// assert_eq!(img.get(Pixel{x: 0, y: 0}), 0);
/// assert_eq!(img.get(Pixel{x: 1, y: 0}), 0);
/// assert_eq!(img.get(Pixel{x: 0, y: 1}), 0);
/// assert_eq!(img.get(Pixel{x: 2, y: 2}), 0);
/// ```
#[derive(Clone)]
pub struct Blob
{
	/// The combined intensity of all the pixels.
	pub intensity : UInt,
	/// The center weighted point.
	pub centroid : Vector2,
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
	use crate::util::aliases::{Byte, UInt};
	use image_processing::*;

	//
	// valid_pixel (x, y) -> bool
	//

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


	//
	// histogram ( histogram : &mut [Byte] )
	//

	#[test]
	fn test_histogram_256_bars ( )
	{
		let mut img : BasicImage<3, 3> = BasicImage::new();
		let mut hist : [UInt; Byte::max_value() as usize + 1] = [0; Byte::MAX as usize + 1];
		img.set(Pixel{x: 0, y: 0}, 10);
		img.set(Pixel{x: 2, y: 0}, 20);
		img.set(Pixel{x: 0, y: 2}, 32);
		img.set(Pixel{x: 0, y: 1}, 43);

		img.histogram(&mut hist);
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
		img.histogram(&mut hist);
		assert_eq!(hist[0], 8);
		assert_eq!(hist[1], 1);
	}

	#[test]
	fn test_histogram_1_bar ( )
	{
		let mut img : BasicImage<3, 3> = BasicImage::new();
		let mut hist : [UInt; 1] = [0; 1];
		img.set(Pixel{x: 2, y: 2}, Byte::MAX);
		img.histogram(&mut hist);
		assert_eq!(hist[0], 9);
	}




	//
	// percent_threshold ( percentage : Decimal, histogram : &[Byte] )
	//
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
}
