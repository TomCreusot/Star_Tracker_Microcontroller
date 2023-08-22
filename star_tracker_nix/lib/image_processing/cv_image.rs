use opencv::core::Mat;
use opencv::core::Vector;
use opencv::core::CV_8UC3;
use opencv::core::Point3_;
use opencv::core::prelude::MatTrait;
use opencv::core::prelude::MatTraitConst;
use opencv::prelude::MatTraitConstManual;

use opencv::highgui::imshow;
use opencv::highgui::wait_key;
use opencv::imgcodecs::imread;
use opencv::imgcodecs::imwrite;
use opencv::imgproc::cvt_color;
use opencv::highgui::destroy_all_windows;

use crate::image_processing::CVImage;
use crate::image_processing::Color;
use crate::image_processing::NixImage;

use star_tracker_lib::util::aliases::Byte;
use star_tracker_lib::util::units::Pixel;

use star_tracker_lib::image_processing::Image;

type Point3 = Point3_<Byte>;

impl CVImage
{
	/// Creates a new black image of specified size.
	/// # Arguments
	/// * `size` - The size of the image.
	pub fn new ( size: Pixel ) -> Self
	{
		return CVImage::new_color(size, Color::Black);
	}

	/// Creates a new image with specified size and color.
	/// # Arguments
	/// * `size` - The size of the image.
	/// * `color` - The color of each pixel.
	pub fn new_color ( size: Pixel, color: Color ) -> Self
	{
		unsafe // Declaring image is unsafe
		{
			let mat = Mat::new_rows_cols(size.y as i32, size.x as i32, CV_8UC3).expect("?");

			let mut img = Self(mat);
			for xx in 0..size.x
			{
				for yy in 0..size.y
				{
					NixImage::set(&mut img, Pixel{x: xx, y: yy}, color);
				}
			}
			return img;
		}
	}


	/// Reads an image from a file as an image.
	/// # Arguments
	/// * `path` - The path, name and extension of the file.
	/// # Returns
	/// An easy to use/implement image.
	pub fn read ( path : &str ) -> Self
	{
		unsafe // Declaring image is unsafe
		{
			let gray = imread(path, 0).expect("Invalid location.");
			let mut color: Mat = Mat::new_rows_cols(gray.mat_size()[0], gray.mat_size()[1], CV_8UC3).expect("?");
			cvt_color(&gray, &mut color, 8, 3).expect("Why not");
			return Self(color);
		}
	}


	/// Displays an image and waits for keyboard input or `time`.
	/// # Arguments
	/// * `name` - The title of the image.
	/// * `image` - The image to display.
	/// * `time` - The maximum time to delay until the code continues. (0 forever).
	pub fn show ( &self, name : &str, time: i32 )
	{
		imshow(name, &self.0).expect("?");
		wait_key(time).expect("?");
	}


	/// Closes all images which where shown.
	pub fn hide ( )
	{
		destroy_all_windows().expect("?");
	}


	/// Returns a copied image.
	/// # Arguments
	/// * `from` - The start image.
	/// # Returns
	/// An identical copy as a CVImage.
	pub fn duplicate ( from: &dyn Image ) -> Self
	{
		let mut img = Self::new(Pixel{x: from.width(), y: from.height()});
		from.copy_to(&mut img).expect("?");
		return img;
	}
}

// Grayscale Image
impl Image for CVImage
{
	/// Image impl for getting the value of a pixel.
	fn get ( &self, px: Pixel ) -> Byte
	{
		let color = NixImage::get(self, px).get_color();
		return ((color[0] as u32 + color[1] as u32 + color[2] as u32) / 3) as Byte;
	}

		/// Image impl for setting the value of a pixel.
	fn set ( &mut self, px: Pixel, value: Byte )
	{
		NixImage::set(self, px, Color::Custom(value, value, value))
	}


	/// Returns the width of the image.
	///	# Returns
	///	The width of the image.
	fn width ( &self ) -> usize { return self.0.size().expect("?").width as usize; }

	/// Returns the height of the image.
	///	# Returns
	///	The height of the image.
	fn height ( &self ) -> usize { return self.0.size().expect("?").height as usize; }
}





// Coloured Image
impl NixImage for CVImage
{
	/// Saves the image to the specified location.
	/// Creates directory if does not exist.
	/// # Arguments
	/// * `name` - The path, name and extension of the image.
	fn save ( &self, name : &str )
	{
		imwrite(name, &self.0, &Vector::new()).expect("Invalid location");
	}

	/// Gets the pixel as a coloured value.
	/// # Arguments
	/// * `px` - The pixel to get.
	fn get ( &self, px: Pixel ) -> Color
	{
		let c = self.0.at_2d::<Point3>(px.y as i32, px.x as i32).expect("?");
		return Color::Custom(c.z, c.y, c.x);
		}

	/// Sets the pixel as a coloured value.
	/// # Arguments
	/// * `px` - The pixel to set.
	/// * `value` - The color to set at the specified position.
	fn set ( &mut self, px: Pixel, value: Color )
	{
		let px_val = value.get_color();
		let c = self.0.at_2d_mut::<Point3>(px.y as i32, px.x as i32).expect("?");
		c.x = px_val[2];
		c.y = px_val[1];
		c.z = px_val[0];
	}
}










/// Copied from rust opencv source.
impl Clone for CVImage
{
	#[inline]
	fn clone(&self) -> Self
	{
		return Self(self.0.clone());
	}
}
