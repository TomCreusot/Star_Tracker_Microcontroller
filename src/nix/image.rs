use crate::image_processing::Image; 
use crate::nix::NixImage;
// use crate::util::{aliases::Decimal, coordinates::Cartesian2D}; 
use image::io::Reader as ImageReader;
use image::{RgbImage, Rgb};
use std::env;

impl NixImage
{
	/// Draws the points onto the image of the specified color.
	/// # Arguments
	/// * `x` - The x position.
	/// * `y` - The y position.
	/// * `size` - The size of the crosshair.
	/// * `color` - The color to draw.
	/// * `img_rgb` - the image to draw on.
	///
	pub fn draw_points ( x: u32, y: u32, size : u32, color : [u8; 3], img_rgb : &mut RgbImage )
	{
		for yy in (y + 1)..std::cmp::min(y+size, img_rgb.height())
		{
			img_rgb.put_pixel(x, yy, Rgb{0:color});
		}
		for xx in (x + 1)..std::cmp::min(x+size, img_rgb.width())
		{
			img_rgb.put_pixel(xx, y, Rgb{0:color});
		}
	}
	
	/// Reads in an image as a gray image.
	/// # Arguments
	/// * `name` - The location from the cwd, name and extension.
	///
	/// # Returns
	/// A RGB image matching the specifications.
	pub fn read_image ( name : &str ) -> RgbImage
	{
		let mut dir = env::current_dir().unwrap();
		dir.push(name);
		let rdr = ImageReader::open(dir);
		let rdr2 = rdr.unwrap();
		let decoded = rdr2.decode();
		return decoded.unwrap().to_rgb8();
	}


	/// Converts a RGB dynamic into an Image.
	/// # Arguments
	/// * `name` - The location from the cwd, name and extension.
	/// * `img` - The image to copy across.	
	pub fn dynamic_to_image ( img_rgb : &RgbImage, img : &mut dyn Image )
	{
		for y in 0..img_rgb.height() as usize
		{
			for x in 0..img_rgb.width() as usize
			{
				let px = img_rgb.get_pixel(x as u32, y as u32);
				let px_value = ((px[0] as u32 + px[1] as u32 + px[2] as u32) / 3) as u8;
				img.set(x as usize, y as usize, px_value);
			}
		}
	}


	/// Converts an Image to a RGB dynamic.
	/// # Arguments
	/// * `img` - The image to copy across.
	///
	/// # Returns
	/// The image.
	pub fn image_to_dynamic ( img : &dyn Image ) -> RgbImage
	{
		let mut rgb_img = RgbImage::new(img.width() as u32, img.height() as u32);
		
		for y in 0..img.height()
		{
			for x in 0..img.width()
			{
				let px_value = Rgb{0:[img.get(x, y), img.get(x, y), img.get(x, y)]};
				rgb_img.put_pixel(x as u32, y as u32, px_value);
			}
		}
		return rgb_img;
	}
}
