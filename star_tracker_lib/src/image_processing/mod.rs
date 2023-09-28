//! `image_processing` is basic image manipulation/storage and machine vision.
//!
//! `image_processing` involves storing an image in a reduced size and performing basic computer vision on the image.
//! This includes a fast method of blob detection and thresholding to obtain stars.
//!
//! # Extracting Stars
//! ```
//! use star_tracker_lib::util::aliases::Decimal;
//! use star_tracker_lib::util::aliases::Byte;
//! use star_tracker_lib::util::units::Vector2;
//! use star_tracker_lib::util::units::Pixel;
//! use star_tracker_lib::util::list::List;
//! use star_tracker_lib::image_processing::BasicImage;
//! use star_tracker_lib::image_processing::Blob;
//! use star_tracker_lib::image_processing::ThresholdGrid;
//! use star_tracker_lib::image_processing::ThresholdPercent;
//!
//! // Read the image in however you want.
//! // In this case we are creating a black image.
//! // Lets just assume it is a beautiful star scape...
//! const img_width:  usize = 808;
//! const img_height: usize = 608;
//! let mut img: BasicImage<img_width, img_height> = BasicImage::new();
//! 
//! // Nilback or Sauvola thresholding.
//! // This threshold is a set of grid cells which all have their own threshold for the local area.
//! // In this case it is 250 (50x50) cells spanning ~12 pixels wide and tall.
//! const grid_size: usize = 50;
//! let overshoot  : Byte  = 50; // How much over the mean should be considered the cutoff.
//! let skip       : usize = 1;  // Only samples every second pixel. This makes the threshold 4 times faster. 
//! let thresh_grid: ThresholdGrid<grid_size, grid_size> = ThresholdGrid::new(&img, overshoot, skip);
//!
//! 
//! // This is a global threshold where you choose a percentage of pixels to cut off.
//! // The image generates a histogram and anything under 99.99% is considered background.
//! // This is less effective as it does not consider bloom.
//! let percent       : Decimal = 0.9999;
//! let thresh_percent: ThresholdPercent = ThresholdPercent::new(&img, percent);
//!
//! // To view the effects of the threshold, you can use:
//! // thresh.apply(&mut image);     // To remove the background.
//! // thresh.apply_bin(&mut image); // To make the image binary.
//! // This is not necessary however and will slow the program down...
//!
//!
//! // Now that we have a threshold, we need to find the stars (blobs).
//! // The implemented blob detection algorithm is the grass fire method.
//! // This will delete the image so make a copy if you need.
//! //
//! // Since this is an embedded project, you need to provide a *stack* for it to store all the neighboring pixels.
//! // If you are using a computer, provide a Vec, otherwise if you have a limited size, use an arraylist.
//! // The size of the array list determines roughly how big the blob is.
//! // You may even want to use this if you are using a PC as it limits the blob size.
//! // 
//! // There is also a *min_size*, this specifies the min size the blob can be.
//! // The bigger the star, the more accurate it is as a centroid between the pixels can be calculated.
//! // Also single pixels can be hot pixels.
//! // When searching, this method will skip each min_size pixels to speed up the program.
//! // It is recommended to set *min_size* to 2.
//! 
//!	let mut stack : Vec<Pixel> = Vec::new(); // Infinite sized blobs, use array list for finite size.
//! let mut blobs : Vec<Blob>  = Vec::new(); // The output.
//! let blob_min_size = 2;                   // Blobs must be at least 2 pixels big.
//!
//! Blob::find_blobs(blob_min_size, &thresh_grid, &mut img, &mut stack, &mut blobs);
//!	blobs.sort_order(Blob::sort_descending_intensity); // Sort by intensity and/or size for the biggest stars first.
//!
//! // to convert this into a useful format, just do this:
//! let mut stars_2d : Vec<Vector2> = Vec::new();
//! Blob::to_vector2(&blobs, &mut stars_2d);
//! ```

pub mod image;
pub mod basic_image;
pub mod c_image;
pub mod blob;
pub mod threshold;

use crate::core_include::*;

use crate::util::aliases::Byte;
use crate::util::aliases::UInt;
use crate::util::units::Pixel;
use crate::util::units::Vector2;

pub use crate::image_processing::image::Image;
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
/// use star_tracker_lib::image_processing::{Image, BasicImage};
/// use star_tracker_lib::util::aliases::{UInt, Byte};
/// use star_tracker_lib::util::list::ArrayList;
/// use star_tracker_lib::util::units::Pixel;
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
/// ```
pub struct BasicImage <const WIDTH : usize, const HEIGHT : usize>
{
	/// The image.
	img : [[Byte; WIDTH]; HEIGHT],
}



/// If you have rust code wrapped inside a c binary, you can use this to access the array.
///
/// # Example (C code)
/// ``` C
/// 
/// static size_t width  = 5;
/// static size_t height = 5;
/// static unsigned char image[] = 
/// {
///  11, 12, 13, 14, 15,
///  21, 22, 23, 24, 25,
///  31, 32, 33, 34, 35,
///  41, 42, 43, 44, 45,
///  51, 52, 53, 54, 55
/// };
/// 
/// typedef struct CImage
/// {
/// 	size_t width;
/// 	size_t height;
/// 	unsigned char* img;
/// } CImage;
/// 
/// CImage get_image ( )
/// {
///     CImage img;
///     img.width  = width;
///     img.height = height;
///     img.img  = image;
///     return img;
/// } 
///
/// ```
///
/// # Example (Rust code)
/// ``` ignore
/// use star_tracker_lib::image_processing::CImage;
///
/// extern "C"
/// {
///  	// Ensure the rust code is compiled with the c code.
/// 	pub fn get_image ( ) -> CImage;
/// }
///
/// fn function ( )
/// {
/// 	unsafe{ let image = get_image(); }
/// }
/// ```
#[repr(C)]
pub struct CImage
{
	width:  usize,
	height: usize,
	img: *mut Byte,
}

//###############################################################################################//
//										---	Threshold ---
//###############################################################################################//

/// A trait to help specify if a pixel is the foreground (stars) or background (space) of the image.
pub trait Threshold
{
	/// Returns the threshold which is considered as the foreground (stars).
	fn foreground ( &self, point: Pixel ) -> Byte;
	
	/// Applies the threshold to the image (CONSUMES).  
	/// This is not needed for blob detection, just for viewing.
	fn apply ( &self, img: &mut dyn Image )
	{
		for xx in 0..img.width()
		{
			for yy in 0..img.height()
			{
				let pos = Pixel{x: xx, y: yy};
				if img.get(pos) < self.foreground(pos)
				{
					img.set(pos, 0);
				}
			}
		}
	}
	
	/// Applies the threshold to the image (CONSUMES).  
	/// This is not needed for blob detection, just for viewing.
	/// Sets background to 0, foreground to 255.
	fn apply_bin ( &self, img: &mut dyn Image )
	{
		for xx in 0..img.width()
		{
			for yy in 0..img.height()
			{
				let pos = Pixel{x: xx, y: yy};
				if img.get(pos) < self.foreground(pos)
				{
					img.set(pos, 0);
				}
				else
				{
					img.set(pos, 255);
				}
			}
		}
	}
}

/// A basic percent threshold (Worse Than ThresholdGrid).  
///
/// This will generate a number based on a percentage brightness of the image.
pub struct ThresholdPercent
{
	threshold: Byte,
}


/// Nilback or Sauvola thresholding (Better than PercentThreshold).  
///
/// This is the process of dividing the image into regions, each region has its own threshold.  
/// By having multiple regions, if one part of the image has disturbance, it will not effect the other.  
pub struct ThresholdGrid <const NUM_H: usize, const NUM_V: usize>
{
	/// The size of the image thresholded.
	size : Pixel,

	/// Each grid cell manages its own threshold.
	cells : BasicImage<{NUM_H}, {NUM_V}>,
}



//###############################################################################################//
//										---	Blob ---
//###############################################################################################//

/// A set of connected pixels in the foreground representing a star.
///
/// This uses the grass fire method (iteratively find new pixels and consume them).
///
/// # Example
/// ```
/// use star_tracker_lib::image_processing::Blob;
/// use star_tracker_lib::image_processing::BasicImage;
/// use star_tracker_lib::image_processing::Image;
/// use star_tracker_lib::image_processing::ThresholdPercent;			
/// use star_tracker_lib::util::units::Pixel;
/// use star_tracker_lib::util::list::{List, ArrayList};
/// let mut img : BasicImage<3, 3> = BasicImage::new();
/// img.set(Pixel{x: 0, y: 0}, 1); // 1 1 0
/// img.set(Pixel{x: 1, y: 0}, 1); // 1 0 0
/// img.set(Pixel{x: 0, y: 1}, 1); // 0 0 1
/// img.set(Pixel{x: 2, y: 2}, 1);
///
/// // lst is all the blobs in the image.
/// let mut lst : ArrayList<Blob, 9> = ArrayList::new();
///
/// // stack is the memory allocation allowed for when finding the blob.
/// // If you have limited space, use an array list with the size equal to the max number of pixels in a blob.
/// // If you have unlimited space or a heap, use Vec.
/// const MAX_BLOB_SIZE : usize = 3;
/// let mut stack_on_stack: ArrayList<Pixel, MAX_BLOB_SIZE> = ArrayList::new();
/// // let mut stack_on_heap : Vec<Pixel> = Vec::new();
///
/// // You must have a threshold for the image to know what is foreground.
/// let thresh = ThresholdPercent::new(&img, 0.5); // Look at ThresholdGrid for a better threshold.
///
/// // The smallest size a blob is allowed.
/// let min_size = 2; 
///
/// Blob::find_blobs(min_size, &thresh, &mut img, &mut stack_on_stack, &mut lst);
/// assert_eq!(lst.get(0).intensity, 3);
/// assert_eq!(lst.get(0).centroid.x, 0.333333333333333333333);
/// assert_eq!(lst.get(0).centroid.y, 0.333333333333333333333);
///
/// assert_eq!(lst.size(), 1);
///
/// // Image has been consumed.
/// assert_eq!(img.get(Pixel{x: 0, y: 0}), 0);
/// assert_eq!(img.get(Pixel{x: 1, y: 0}), 0);
/// assert_eq!(img.get(Pixel{x: 0, y: 1}), 0);
/// assert_eq!(img.get(Pixel{x: 2, y: 2}), 0);
/// ```
#[derive(Clone)]
pub struct Blob
{
	/// The number of pixels used.
	pub size: usize,
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
#[allow(unused_must_use)]
mod test
{
	use crate::image_processing::BasicImage;
	use crate::image_processing::Image;
	use crate::image_processing::ThresholdPercent;
	use crate::image_processing::Threshold;
	
	use crate::util::units::Pixel;

	#[test]
	fn test_threshold_apply (  )
	{
		let thresh = ThresholdPercent{threshold: 10};

		let mut img : BasicImage<3, 3> = BasicImage::new();
		img.set(Pixel{x: 0, y: 0}, 9);  // 9,  10, 9
		img.set(Pixel{x: 1, y: 0}, 10); // 10, 9,  10
		img.set(Pixel{x: 2, y: 0}, 9);  // 9,  10, 9

		img.set(Pixel{x: 0, y: 1}, 10);
		img.set(Pixel{x: 1, y: 1}, 9);
		img.set(Pixel{x: 2, y: 1}, 10);
		
		img.set(Pixel{x: 0, y: 2} , 9);
		img.set(Pixel{x: 1, y: 2} , 10);
		img.set(Pixel{x: 2, y: 2} , 9);

		thresh.apply(&mut img);

		assert_eq!(img.get(Pixel{x: 0, y: 0}), 0);
		assert_eq!(img.get(Pixel{x: 1, y: 0}), 10);
		assert_eq!(img.get(Pixel{x: 2, y: 0}), 0);

		assert_eq!(img.get(Pixel{x: 0, y: 1}), 10);
		assert_eq!(img.get(Pixel{x: 1, y: 1}), 0);
		assert_eq!(img.get(Pixel{x: 2, y: 1}), 10);

		assert_eq!(img.get(Pixel{x: 0, y: 2}), 0);
		assert_eq!(img.get(Pixel{x: 1, y: 2}), 10);
		assert_eq!(img.get(Pixel{x: 2, y: 2}), 0);
	}

	#[test]
	fn test_threshold_apply_bin (  )
	{
		let thresh = ThresholdPercent{threshold: 10};

		let mut img : BasicImage<3, 3> = BasicImage::new();
		img.set(Pixel{x: 0, y: 0}, 9);  // 9,  10, 9
		img.set(Pixel{x: 1, y: 0}, 10); // 10, 9,  10
		img.set(Pixel{x: 2, y: 0}, 9);  // 9,  10, 9

		img.set(Pixel{x: 0, y: 1}, 10);
		img.set(Pixel{x: 1, y: 1}, 9);
		img.set(Pixel{x: 2, y: 1}, 10);
		
		img.set(Pixel{x: 0, y: 2} , 9);
		img.set(Pixel{x: 1, y: 2} , 10);
		img.set(Pixel{x: 2, y: 2} , 9);

		thresh.apply_bin(&mut img);

		assert_eq!(img.get(Pixel{x: 0, y: 0}), 0);
		assert_eq!(img.get(Pixel{x: 1, y: 0}), 255);
		assert_eq!(img.get(Pixel{x: 2, y: 0}), 0);

		assert_eq!(img.get(Pixel{x: 0, y: 1}), 255);
		assert_eq!(img.get(Pixel{x: 1, y: 1}), 0);
		assert_eq!(img.get(Pixel{x: 2, y: 1}), 255);

		assert_eq!(img.get(Pixel{x: 0, y: 2}), 0);
		assert_eq!(img.get(Pixel{x: 1, y: 2}), 255);
		assert_eq!(img.get(Pixel{x: 2, y: 2}), 0);
	}
}