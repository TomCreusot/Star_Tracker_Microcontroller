use crate::image_processing::Image;
use crate::nix::NixImage;
use image::io::Reader as ImageReader;
use image::{RgbImage, Rgb};
use crate::util::aliases::Byte;
use crate::util::units::Pixel;
use std::env;


impl Image for NixImage
{
	/// Image impl for getting the value of a pixel.
	fn get ( &self, px: Pixel ) -> Byte
	{ return self.img_rgb.get_pixel(px.x as u32, px.y as u32).0[0]; }

	/// Image impl for setting the value of a pixel.
	fn set ( &mut self, px: Pixel, value : Byte )
	{
		let color = Rgb{0: [value, value, value]};
		return self.img_rgb.put_pixel(px.x as u32, px.y as u32, color);
	}


	/// Returns the width of the image.
	///	# Returns
	///	The width of the image.
	fn width ( &self ) -> usize { return self.img_rgb.width() as usize; }

	/// Returns the height of the image.
	///	# Returns
	///	The height of the image.
	fn height ( &self ) -> usize { return self.img_rgb.height() as usize; }
}




impl NixImage
{
	/// Copies the provided image.
	/// # Arguments
	/// * `img` - The image to copy.
	pub fn new ( image: &dyn Image ) -> Self
	{
		let mut img = RgbImage::new(image.width() as u32, image.height() as u32);
		for y in 0..image.height()
		{
			for x in 0..image.width()
			{
				let val = image.get(Pixel{x: x, y: y});
				let px_value = Rgb{0:[val, val, val]};
				img.put_pixel(x as u32, y as u32, px_value);
			}
		}
		return Self{img_rgb: img};
	}
	
	
	/// Draws the points onto the image of the specified color.
	/// # Arguments
	/// * `px` - The position.
	/// * `size` - The size of the crosshair.
	/// * `color` - The color to draw.
	/// * `img_rgb` - the image to draw on.

	pub fn draw_points ( &mut self, px: Pixel, size : u32, color : [u8; 3])
	{
		for yy in (px.y as u32 + 1)..std::cmp::min(px.y as u32 + size, self.img_rgb.height())
		{
			self.img_rgb.put_pixel(px.x as u32, yy, Rgb{0:color});
		}
		for xx in (px.x as u32 + 1)..std::cmp::min(px.x as u32 + size, self.img_rgb.width())
		{
			self.img_rgb.put_pixel(xx, px.y as u32, Rgb{0:color});
		}
	}



	/// Reads in an image as a gray image.
	/// # Arguments
	/// * `name` - The location from the cwd, name and extension.
	///
	/// # Returns
	/// A RGB image matching the specifications.
	pub fn read_image ( name : &str ) -> NixImage
	{
		let mut dir = env::current_dir().unwrap();
		dir.push(name);
		let rdr = ImageReader::open(dir);
		let rdr2 = rdr.unwrap();
		let decoded = rdr2.decode();

		return NixImage{img_rgb: decoded.unwrap().to_rgb8()};
	}


	/// Converts a RGB dynamic into an Image.
	/// # Arguments
	/// * `name` - The location from the cwd, name and extension.
	/// * `img` - The image to copy across.
	pub fn dynamic_to_image ( &self, img : &mut dyn Image )
	{
		for y in 0..self.img_rgb.height() as usize
		{
			for x in 0..self.img_rgb.width() as usize
			{
				let px = self.img_rgb.get_pixel(x as u32, y as u32);
				let px_value = ((px[0] as u32 + px[1] as u32 + px[2] as u32) / 3) as u8;
				img.set(Pixel{x: x as usize, y: y as usize}, px_value);
			}
		}
	}


	/// Converts an Image to a RGB dynamic.
	/// # Arguments
	/// * `img` - The image to copy across.
	///
	/// # Returns
	/// The image.
	pub fn image_to_dynamic ( &mut self, img : &dyn Image ) -> &RgbImage
	{
		self.img_rgb = NixImage::new(img).img_rgb;
		return &self.img_rgb;
	}
}
