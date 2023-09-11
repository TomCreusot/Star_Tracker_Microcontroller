//! Basic image manipulation/storage and machine vision.
//!
//! `image_processing` involves storing an image in a reduced size and performing basic computer vision on the image.
//! This includes a fast method of blob detection and thresholding to obtain stars.

pub mod image;
pub mod basic_image;
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
}

/// A basic percent threshold.
/// This will generate a number based on a percentage brightness of the image.
pub struct ThresholdPercent
{
	threshold: Byte,
}


// /// A varient of threshold which consists of a set of points which specify a local threshold.
// /// The median/mean/percent is calculated in the local area of each node.
// /// When the threshold of a pixel is requested, a linear function calculates the expected threshold of the pixel based on neiboring pixels.
// ///
// /// These points are equally spaced on the x and y axis being `NUM_H` wide and `NUM_V` high.
// //pub struct ThresholdNodal <const NUM_H: usize, const NUM_V: usize>
// //{
// //	/// Each node is used to represent a local threshold value and position.
// //	nodes : BasicImage<{NUM_H}, {NUM_V}>,
// //}

/// Nilback or Sauvola thresholding.
/// This is the process of dividing the image into regions, each region has its own threshold.
/// By having multiple regions, if one part of the image has disterbence, it will not effect the other.
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
/// let thresh = ThresholdPercent::new(&img, 0.5);
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
