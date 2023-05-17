//! Basic image manipulation/storage and machine vision.
//!
//! `image_processing` involves storing an image in a reduced size and performing basic computer vision on the image.
//! This includes a fast method of blob detection and thresholding to obtain stars.

pub mod image;
pub mod basic_image;
pub mod blob;

use crate::core_include::*;

use crate::util::aliases::Byte;
use crate::util::aliases::UInt;
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
///
/// This uses the grass fire method (iteratively find new pixels and consume them).
///
/// # Example
/// ```
/// use star_tracker_lib::image_processing::{Blob, Image, BasicImage};
/// use star_tracker_lib::util::{list::ArrayList, list::List, units::Vector2};
/// use star_tracker_lib::util::units::Pixel;
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
