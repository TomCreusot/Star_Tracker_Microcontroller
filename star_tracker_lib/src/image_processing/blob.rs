//! Implementation of [Blob](crate::util::image_processing::Blob).
use core_include::*;

use crate::util::aliases::Decimal;
use crate::util::aliases::UInt;
use crate::util::aliases::Byte;
use crate::util::units::Pixel;
use crate::util::units::Vector2;
use crate::util::list::List;
use crate::util::list::ListIterator;
use crate::util::list::ArrayList;

use crate::image_processing::Image;
use crate::image_processing::Blob;


impl Blob
{
//###############################################################################################//
//									--- Constructor ---
//###############################################################################################//
	pub fn new ( ) -> Blob
	{
		return Blob { intensity: 0, centroid: Vector2{x: 0.0, y: 0.0} };
	}
	
	
//###############################################################################################//
//									--- Front End ---
//###############################################################################################//

	/// Finds all blobs in an image.
	/// # Generic Arguments
	/// * `SIZE_BLOB` - The max number of pixels in a blob.
	/// # Arguments
	/// * `config` - The properties associated with the size requriement of the blob.
	/// * `theshold` - The value which is the minimum intensity to satisfy being in the foreground.
	/// * `img` - The image to read and set to black.
	/// * `lst` - The list to append the blobs to.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::image_processing::{Blob, BasicImage, Image};
	/// use star_tracker_lib::util::units::Pixel;
	/// use star_tracker_lib::util::list::{List, ArrayList};
	/// let mut img : BasicImage<3, 3> = BasicImage::new();
	/// img.set(Pixel{x: 0, y: 0}, 1); // 1 1 0
	/// img.set(Pixel{x: 1, y: 0}, 1); // 1 0 0
	/// img.set(Pixel{x: 0, y: 1}, 1); // 0 0 1
	/// img.set(Pixel{x: 2, y: 2}, 1);
 	///
	/// let mut lst : ArrayList<Blob, 9> = ArrayList::new();
	/// const MAX_BLOB_SIZE : usize = 3;
	/// Blob::find_blobs::<MAX_BLOB_SIZE>(1, &mut img, &mut lst);
	/// assert_eq!(lst.get(0).intensity, 3);
	/// assert_eq!(lst.get(0).centroid.x, 0.333333333333333333333);
	/// assert_eq!(lst.get(0).centroid.y, 0.333333333333333333333);
	///
	/// assert_eq!(lst.get(1).intensity, 1);
	/// assert_eq!(lst.get(1).centroid.x, 2.0);
	/// assert_eq!(lst.get(1).centroid.y, 2.0);
	/// assert_eq!(lst.size(), 2);
	///
	/// // Image has been consumed.
	/// assert_eq!(img.get(Pixel{x: 0, y: 0}), 0);
	/// assert_eq!(img.get(Pixel{x: 1, y: 0}), 0);
	/// assert_eq!(img.get(Pixel{x: 0, y: 1}), 0);
	/// assert_eq!(img.get(Pixel{x: 2, y: 2}), 0);
	/// ```
	pub fn find_blobs <const CONFIG : usize> (
												threshold: Byte,
												img: &mut dyn Image,
												lst: &mut dyn List<Blob>
											)
	{
		for y in 0..img.height ( )
		{
			for x in 0..img.width ( )
			{
				if threshold <= img.get(Pixel{x: x, y: y})
				{
					lst.slot(
						Blob::spread_grass_fire::<CONFIG>(threshold, Pixel{x: x, y: y}, img),
						Blob::sort_descending_intensity);
				}
			}
		}
	}


	/// Creates and expands the blob to fit any pixels in the foreground.
	/// # Generic Arguments
	/// * `BLOB_SIZE` - The number of pixels allowed in a blob (how much memory to use finding a single blob).
	/// # Arguments
	/// * `config` - The config for the max blob size.
	/// * `threshold` - The value which is the minimum intensity to satisfy being in the foreground.
	/// * `start` - The start point where the blob should begin.
	/// * `img` - The image to read and consume (set pixels to 0).
	///
	/// # Returns
	/// The blob.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::image_processing::{Blob, BasicImage, Image};
	/// use star_tracker_lib::util::{list::{List, ArrayList}, units::Pixel};
	/// let mut img : BasicImage<2, 2> = BasicImage::new();
	/// img.set(Pixel{x: 0, y: 0}, 1); // 1 1
	/// img.set(Pixel{x: 1, y: 0}, 1); // 1 0
	/// img.set(Pixel{x: 0, y: 1}, 1);
	///
	/// const MAX_BLOB_SIZE : usize = 3;
	/// let start : Pixel = Pixel{x: 0, y: 0};
	/// let blob = Blob::spread_grass_fire::<MAX_BLOB_SIZE>(1, start, &mut img);
	/// assert_eq!(blob.intensity, 3);
	///
	/// assert!((blob.centroid.x - 0.333).abs() < 0.01);
	/// assert!((blob.centroid.y - 0.333).abs() < 0.01);
	///
	/// assert_eq!(img.get(Pixel{x: 0, y: 0}), 0);
	/// assert_eq!(img.get(Pixel{x: 1, y: 0}), 0);
	/// assert_eq!(img.get(Pixel{x: 0, y: 1}), 0);
	/// ```
	pub fn spread_grass_fire <const CONFIG : usize> (
														threshold : Byte,
														start : Pixel,
														img : &mut dyn Image
													) -> Blob
	{
		let mut blob : Blob = Blob::new();
		let mut stack : ArrayList<Pixel, CONFIG> = ArrayList::new();
		let _ = stack.push_back(start).is_err();
		while let Result::Ok(cur) = stack.pop_back()
		{
			if img.get(cur) != 0 // If Reinserted in list.
			{
				// Add to list
				Blob::find_neighbours(threshold, &cur, img, &mut stack);

				// Recalculate Centroid and Intensity
				blob.centroid.x = Blob::find_centroid(
					blob.centroid.x, blob.intensity, cur.x as UInt, img.get(cur) as UInt);
				blob.centroid.y = Blob::find_centroid(
					blob.centroid.y, blob.intensity, cur.y as UInt, img.get(cur) as UInt);
				blob.intensity += img.get(cur) as UInt;

				// Set the pixel to black.
				img.set(cur, 0);
			}
		}
		return blob;
	}


//###############################################################################################//
//									--- Back End ---
//###############################################################################################//

	/// Finds all neighbouring pixels which are valid and within the theshold.
	/// # Generic Arguments
	/// * `SIZE_BLOB` - The number of pixels allowed in a blob (how much memory to use to find a single blob).
	///
	/// # Arguments
	/// * `threshold` - The value which is the minimum intensity to satisfy being in the foreground.
	/// * `pt` - The point to look around.
	/// * `img` - The image to examine.
	/// * `stack` - The stack to append to.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::image_processing::{Blob, BasicImage, Image};
	/// use star_tracker_lib::util::units::Pixel;
	/// use star_tracker_lib::util::list::{ArrayList, List};
	/// let threshold = 1;
	/// let pt : Pixel = Pixel{ x: 1, y: 1 };
	/// let mut img : BasicImage<3, 3> = BasicImage::new();
	/// let mut lst : ArrayList<Pixel, 4> = ArrayList::new();
	///
	/// img.set(Pixel{x: 1, y: 0}, 1); // 0, 1, 0
	/// img.set(Pixel{x: 0, y: 1}, 1); // 1, 0, 1
	/// img.set(Pixel{x: 1, y: 2}, 1); // 0, 1, 0
	/// img.set(Pixel{x: 2, y: 1}, 1);
	///
	/// Blob::find_neighbours(threshold, &pt, &img, &mut lst); // Right, Left, Up, Down
	/// assert_eq!(lst.size(), 4);
	///
	/// assert_eq!(lst.get(0).x, 2); // Right
	/// assert_eq!(lst.get(0).y, 1);
	///
	/// assert_eq!(lst.get(1).x, 0); // Left
	/// assert_eq!(lst.get(1).y, 1);
	///
	/// assert_eq!(lst.get(2).x, 1); // Up
	/// assert_eq!(lst.get(2).y, 0);
	///
	/// assert_eq!(lst.get(3).x, 1); // Down
	/// assert_eq!(lst.get(3).y, 2);
	/// ```
	pub fn find_neighbours(threshold: Byte, pt: &Pixel, img: &dyn Image,stack:&mut dyn List<Pixel>)
	{
		// 4 directional
		// Right
		let mut px = Pixel{x: pt.x + 1, y: pt.y};
		if img.valid_pixel(px) && threshold <= img.get(px)
		{
			if stack.push_back(px).is_err()
			{
				return; // stack is full.
			}
		}
		// Left
		if 0 < pt.x
		{
			px = Pixel{x: pt.x - 1, y: pt.y};
			if img.valid_pixel(px) && threshold <= img.get(px)
			{
				if stack.push_back(px).is_err()
				{
					return; // stack is full.
				}
			}
		}
		// Up
		if 0 < pt.y
		{
			px = Pixel{x: pt.x, y: pt.y - 1};
			if img.valid_pixel(px) && threshold <= img.get(px)
			{
				if stack.push_back(px).is_err()
				{
					return; // stack is full.
				}
			}
		}
		// Down
		px = Pixel{x: pt.x, y: pt.y + 1};
		if img.valid_pixel(px) && threshold <= img.get(px)
		{
			if stack.push_back(px).is_err()
			{
				return; // stack is full.
			}
		}
	}


	/// Finds the new centroid after the pixel is added to the blob on a specific axis.
	/// # Arguments
	/// * `b_pos` - The current blob centroid.
	/// * `b_intensity` - The magnitude of the pixels in the blob.
	/// * `p_pos` - The pixel position.
	/// * `p_intensity` - The magnitude of the pixel.
	///
	/// # Returns
	/// The new centroid on the given axis.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::image_processing::Blob;
	/// assert_eq!(Blob::find_centroid(0.0, 3, 1, 1), 0.25); // 2, 1
	/// assert_eq!(Blob::find_centroid(0.0, 9, 2, 1), 0.2); // 9, 0, 1
	/// ```
	pub fn find_centroid ( b_pos : Decimal, b_intensity : UInt,
						p_pos : UInt, p_intensity : UInt ) -> Decimal
	{
		return	(	(b_pos * b_intensity as Decimal) 			// Moment of current blob.
					+ (p_pos * p_intensity) as Decimal) 		// Moment of new pixel.
				/ (b_intensity + p_intensity) as Decimal 		// Intensity of new blob.
	}




	/// Converts the list of blobs to a list of points.
	/// # Arguments
	/// * `blobs` - The blobs to convert to points.
	/// * `points` - The centroid of the blobs.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::util::{aliases::Decimal, units::Vector2, list::{List, ArrayList}};
	/// use star_tracker_lib::image_processing::Blob;
	/// let mut blobs : ArrayList<Blob, 3> = ArrayList::new();
	/// blobs.push_back(Blob{intensity: 10, centroid: Vector2{x: 10.0, y: 10.0}});
	/// blobs.push_back(Blob{intensity: 5, centroid: Vector2{x: 5.0, y: 5.0}});
	/// blobs.push_back(Blob{intensity: 0, centroid: Vector2{x: 0.0, y: 0.0}});
	///
	/// let mut points : ArrayList<Vector2, 2> = ArrayList::new();
	/// Blob::to_vector2(&blobs, &mut points);
	/// assert_eq!(blobs.get(0).centroid, points.get(0));
	/// assert_eq!(blobs.get(1).centroid, points.get(1));
	/// ```
	pub fn to_vector2 ( blobs: &dyn List<Blob>, points: &mut dyn List<Vector2> )
	{
		let iterator : ListIterator<Blob> = ListIterator::new(blobs);

		for iter in iterator
		{
			if points.push_back(iter.centroid).is_err()
			{
				return; // points is full;
			}
		}
	}


	/// Sorts in order of intensity brightest first.
	/// # Arguments
	/// * `brighest` - The brightest value.
	/// * `dullest` - The least intense blob.
	///
	/// # Return
	/// True if in order.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::image_processing::Blob;
	/// use star_tracker_lib::util::units::Vector2;
	/// let brightest = Blob{intensity: 1, centroid: Vector2{x: 0.0, y: 0.0}};
	/// let dullest = Blob{intensity: 0, centroid: Vector2{x: 0.0, y: 0.0}};
	/// assert!(Blob::sort_descending_intensity(&brightest, &dullest));
	/// assert!(!Blob::sort_descending_intensity(&dullest, &brightest));
	/// ```
	pub fn sort_descending_intensity ( brighest : & Blob, dullest : & Blob ) -> bool
	{return dullest.intensity < brighest.intensity;}
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
	use crate::image_processing::Blob;
	
	use crate::util::list::ArrayList;
	use crate::util::list::List;
	use crate::util::test::DECIMAL_PRECISION_TEST;
	use crate::util::units::Vector2;
	use crate::util::units::Pixel;
	use crate::util::aliases::Decimal;

	#[no_coverage]
	fn assert_close ( a: Decimal, b: Decimal )
	{
		if (a - b).abs() > DECIMAL_PRECISION_TEST
		{
			assert!(false, "\n\nassert_close failed: \n\tleft: `{}`\n\tright: `{}`\n\n", a, b);
		}
	}


//###############################################################################################//
//
//										Front End
//
// pub fn new      ( ) -> Self
// pub fn find_blobs        <const usize> ( Byte, &mut dyn Image, &mut dyn List<Blob> )
// pub fn spread_grass_fire <const usize> ( Byte, Pixel, &mut dyn Image )
//
//###############################################################################################//
//										~ new ~													 //

	#[test]
	fn test_new ( )
	{
		let blob : Blob = Blob::new();
		assert_eq!(blob.intensity, 0);
		assert_eq!(blob.centroid.x, 0.0);
		assert_eq!(blob.centroid.y, 0.0);
	}

//										~ find_blobs ~											 //
	#[test]
	// An error should not occure if there are no blobs.
	fn test_find_blobs_empty ( )
	{
		let mut img : BasicImage<3, 3> = BasicImage::new();
		let mut lst : ArrayList<Blob, 9> = ArrayList::new();

		const CONFIG : usize = 0;
		Blob::find_blobs::<CONFIG>(1, &mut img, &mut lst);

		assert_eq!(lst.size(), 0);
	}

	#[test]
	// The higher intensity should be prioritised.
	// This will remove noise from the image and ensure a vast spread over the whole image.
	fn test_find_blobs_exceeds_list ( )
	{
		let mut img : BasicImage<3, 3> = BasicImage::new();
		img.set(Pixel{x: 0, y: 0}, 1); // 1, 0, 2
		img.set(Pixel{x: 2, y: 0}, 2); // 0, 0, 0
		img.set(Pixel{x: 0, y: 2}, 5); // 5, 0, 10
		img.set(Pixel{x: 2, y: 2} , 10);
		let mut lst : ArrayList<Blob, 2> = ArrayList::new();

		const CONFIG : usize = 2;
		Blob::find_blobs::<CONFIG>(1, &mut img, &mut lst);

		assert_eq!(lst.size(), 2);
		assert_eq!(lst.get(0).intensity, 10);
		assert_eq!(lst.get(1).intensity, 5);
	}


	#[test]
	// Blobs should not merge on diagonals.
	fn test_find_blobs_diagonal ( )
	{
		let mut img : BasicImage<3, 3> = BasicImage::new();
		img.set(Pixel{x: 0, y: 0}, 1); // 1 1 0
		img.set(Pixel{x: 1, y: 0}, 1); // 1 0 0
		img.set(Pixel{x: 0, y: 1}, 1); // 0 0 1
		img.set(Pixel{x: 1, y: 2}, 1);

		let mut lst : ArrayList<Blob, 9> = ArrayList::new();

		const CONFIG : usize = 2;
		Blob::find_blobs::<CONFIG>(1, &mut img, &mut lst);
		assert_eq!(lst.get(0).intensity, 3);
		assert_close(lst.get(0).centroid.x, 1.0/3.0);
		assert_close(lst.get(0).centroid.y, 1.0/3.0);

		assert_eq!(lst.get(1).intensity, 1);
		assert_eq!(lst.get(1).centroid.x, 1.0);
		assert_eq!(lst.get(1).centroid.y, 2.0);

		assert_eq!(lst.size(), 2);

		assert_eq!(img.get(Pixel{x: 0, y: 0}), 0);
		assert_eq!(img.get(Pixel{x: 1, y: 0}), 0);
		assert_eq!(img.get(Pixel{x: 0, y: 1}), 0);
		assert_eq!(img.get(Pixel{x: 2, y: 2}), 0);
	}


	#[test]
	// Blobs should not jump.
	fn test_find_blobs_separated ( )
	{
		let mut img : BasicImage<3, 3> = BasicImage::new();
		img.set(Pixel{x: 0, y: 0}, 1); // 1 1 0
		img.set(Pixel{x: 1, y: 0}, 1); // 1 0 0
		img.set(Pixel{x: 0, y: 1}, 1); // 0 0 1
		img.set(Pixel{x: 2, y: 2}, 1);

		let mut lst : ArrayList<Blob, 9> = ArrayList::new();
		const CONFIG : usize = 2;

		Blob::find_blobs::<CONFIG>(1, &mut img, &mut lst);
		assert_eq!(lst.get(0).intensity, 3);
		assert_close(lst.get(0).centroid.x, 1.0/3.0);
		assert_close(lst.get(0).centroid.y, 1.0/3.0);

		assert_eq!(lst.get(1).intensity, 1);
		assert_eq!(lst.get(1).centroid.x, 2.0);
		assert_eq!(lst.get(1).centroid.y, 2.0);

		assert_eq!(lst.size(), 2);

		assert_eq!(img.get(Pixel{x: 0, y: 0}), 0);
		assert_eq!(img.get(Pixel{x: 1, y: 0}), 0);
		assert_eq!(img.get(Pixel{x: 0, y: 1}), 0);
		assert_eq!(img.get(Pixel{x: 2, y: 2}), 0);
	}


//										~ spread_grass_fire ~									 //
	#[test]
	// Should safely end if too big.
	fn test_spread_grass_fire_blob_too_big ( )
	{
		const CONFIG : usize = 3; // Will detect 4.
		let mut img : BasicImage<3, 3> = BasicImage::new();
		img.set(Pixel{x: 0, y: 1}, 1); // 0 1 0
		img.set(Pixel{x: 1, y: 0}, 1); // 1 1 1
		img.set(Pixel{x: 2, y: 1}, 1); // 0 X 0
		img.set(Pixel{x: 1, y: 2}, 1); // <(X)
		img.set(Pixel{x: 1, y: 1}, 1); // middle
		let start : Pixel = Pixel{x: 1, y: 1};
		let blob = Blob::spread_grass_fire::<CONFIG>(1, start, &mut img);
		assert_eq!(blob.intensity, 4);
		assert_eq!(blob.centroid.x, 1.0);
		assert_eq!(blob.centroid.y, 0.75);

		assert_eq!(img.get(Pixel{x: 0, y: 1}), 0);
		assert_eq!(img.get(Pixel{x: 1, y: 0}), 0);
		assert_eq!(img.get(Pixel{x: 2, y: 1}), 0);
		assert_eq!(img.get(Pixel{x: 1, y: 2}), 1);
	}


	#[test]
	// Should safely end if too big.
	fn test_spread_grass_fire ( )
	{
		const CONFIG : usize = 3; // Will detect 3.
		let mut img : BasicImage<2, 2> = BasicImage::new();
		img.set(Pixel{x: 0, y: 0}, 1); // 1 1
		img.set(Pixel{x: 1, y: 0}, 1); // 1 0
		img.set(Pixel{x: 0, y: 1}, 1);

		let start : Pixel = Pixel{x: 0, y: 0};
		let blob = Blob::spread_grass_fire::<CONFIG>(1, start, &mut img);
		assert_eq!(blob.intensity, 3);
		assert_close(blob.centroid.x, 1.0/3.0);
		assert_close(blob.centroid.y, 1.0/3.0);

		assert_eq!(img.get(Pixel{x: 0, y: 0}), 0);
		assert_eq!(img.get(Pixel{x: 1, y: 0}), 0);
		assert_eq!(img.get(Pixel{x: 0, y: 1}), 0);
	}


//###############################################################################################//
//
//										Back End
//
// pub fn find_neighbours   ( Byte, &Pixel, &dyn Image, &mut dyn List<Pixel> )
// pub fn find_centroid     ()
//
//###############################################################################################//
//										~ find_neighbours ~										 //
	#[test]
	// The list should not panic if overfilled.
	fn test_find_neighbours ( )
	{
		let threshold = 1;
		let pt : Pixel = Pixel{ x: 1, y: 1 };
		let mut img : BasicImage<3, 3> = BasicImage::new();
		let mut lst : ArrayList<Pixel, 4> = ArrayList::new();

		img.set(Pixel{x: 1, y: 0}, 1); // 0, 1, 0
		img.set(Pixel{x: 0, y: 1}, 1); // 1, 0, 1
		img.set(Pixel{x: 1, y: 2}, 1); // 0, 1, 0
		img.set(Pixel{x: 2, y: 1}, 1);

		Blob::find_neighbours(threshold, &pt, &img, &mut lst); // Right, Left, Up, Down
		assert_eq!(lst.size(), 4);

		assert_eq!(lst.get(0).x, 2); // Right
		assert_eq!(lst.get(0).y, 1);

		assert_eq!(lst.get(1).x, 0); // Left
		assert_eq!(lst.get(1).y, 1);

		assert_eq!(lst.get(2).x, 1); // Up
		assert_eq!(lst.get(2).y, 0);

		assert_eq!(lst.get(3).x, 1); // Down
		assert_eq!(lst.get(3).y, 2);
	}


	#[test]
	// find_neighbours should not panic if it is at the edge of the image.
	fn test_find_neighbours_edge ( )
	{
		let threshold = 1;
		let mut pt : Pixel = Pixel{ x: 0, y: 0 };
		let img : BasicImage<2, 2> = BasicImage::new();
		let mut lst : ArrayList<Pixel, 4> = ArrayList::new();

		Blob::find_neighbours(threshold, &pt, &img, &mut lst);
		pt.x = 1;
		Blob::find_neighbours(threshold, &pt, &img, &mut lst);
		pt.x = 0;
		pt.y = 1;
		Blob::find_neighbours(threshold, &pt, &img, &mut lst);
		pt.x = 1;
		Blob::find_neighbours(threshold, &pt, &img, &mut lst);
	}

	#[test]
	// The list should not panic if overfilled.
	fn test_find_neighbours_overfill ( )
	{
		let threshold = 1;
		let pt : Pixel = Pixel{ x: 1, y: 1 };
		let mut img : BasicImage<3, 3> = BasicImage::new();
		let mut lst : ArrayList<Pixel, 0> = ArrayList::new();

		img.set(Pixel{x: 1, y: 0}, 1); // up
		Blob::find_neighbours(threshold, &pt, &img, &mut lst);

		lst.clear();
		img.set(Pixel{x: 1, y: 0}, 0);
		img.set(Pixel{x: 1, y: 2}, 1); // down
		Blob::find_neighbours(threshold, &pt, &img, &mut lst);
		
		lst.clear();
		img.set(Pixel{x: 1, y: 2}, 0);
		img.set(Pixel{x: 0, y: 1}, 1); // left
		Blob::find_neighbours(threshold, &pt, &img, &mut lst);

		lst.clear();
		img.set(Pixel{x: 0, y: 1}, 0);
		img.set(Pixel{x: 2, y: 1}, 1); // right
		Blob::find_neighbours(threshold, &pt, &img, &mut lst);
	}




//										~ find_centroid ~										 //
	#[test]
	fn test_find_centroid_single_weight ( )
	{
		assert_eq!(Blob::find_centroid(0.0, 1, 1, 1), 0.5); // 1, 1
		assert_eq!(Blob::find_centroid(0.0, 1, 2, 1), 1.0); // 1, 0, 1
	}

	#[test]
	fn test_find_centroid_multi_weight ( )
	{
		assert_eq!(Blob::find_centroid(0.0, 3, 1, 1), 0.25); // 2, 1
		assert_eq!(Blob::find_centroid(0.0, 9, 2, 1), 0.2); // 9, 0, 1
	}


//										~ test_to_cartesian_2d ~								 //
	#[test]
	fn test_to_cartesian_2d ( )
	{
		let mut blobs : ArrayList<Blob, 3> = ArrayList::new();
		blobs.push_back(Blob{intensity: 10, centroid: Vector2{x: 10.0, y: 10.0}});
		blobs.push_back(Blob{intensity: 5, centroid: Vector2{x: 5.0, y: 5.0}});
		blobs.push_back(Blob{intensity: 0, centroid: Vector2{x: 0.0, y: 0.0}});

		let mut points : ArrayList<Vector2, 2> = ArrayList::new();
		Blob::to_vector2(&blobs, &mut points);
		assert_eq!(blobs.get(0).centroid, points.get(0));
		assert_eq!(blobs.get(1).centroid, points.get(1));
	}


//										~ sort_descending_intensity ~							 //
	#[test]
	fn test_sort_descending_intensity ( )
	{
		let brightest = Blob{intensity: 1, centroid: Vector2{x: 0.0, y: 0.0}};
		let dullest = Blob{intensity: 0, centroid: Vector2{x: 0.0, y: 0.0}};
		assert!(Blob::sort_descending_intensity(&brightest, &dullest));
		assert!(!Blob::sort_descending_intensity(&dullest, &brightest));
	}



}