use rand::prelude::*;

use crate::image_processing::Image;
use crate::nix::NixImage;
use image::io::Reader as ImageReader;
use image::{RgbImage, Rgb};
use crate::util::aliases::{Byte, Decimal};
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


	/// Draws the star field on the image centered around `center`, rotated around `rotation`.
	/// All stars outside the `fov` will be excluded.
	///
	/// # Arguments
	/// * `center` - The center of the image.
	/// * `rotation` - The angle of the viewing frame.
	/// * `fov` - The field of view of the viewing frame.
	/// * `intensity_modifer` - The intensity of a magnitude 1 star.
	/// * `intensity_variance` - The variance in brighness (random).
	/// * `blur_modifier` - 1 is 1 pixel, 2 is 4 pixels, etc...
	/// * `blur_variance` - The variance in circularity and size (random).
	/// * `position_variance` - The maximum distance from the actual point where the star should be.
	/// * `stars` - The stars to draw.
/*
	pub fn draw_stars ( center : Equatorial, rotation : Quaternion, fov : Decimal,
						intensity_modifer: Decimal, intensity_variance: Decimal,
						blur_modifier : Decimal, blur_variance : Decimal,
						stars: & Vec<Star> )
	{
		for e in stars
		{
			let pos = rotation.rotate_point(position.to_cartesian3()).from_cartesian3();
			let intensity = variate_scalar(intensity_modifer, intensity_variance) / star.magnitude;
			let blur = variate_scalar(blur_modifer, blur_variance);
			draw_star ( , intensity, blur, self.img_rgb);
		}
	}*/


	/// Variates a decimal by a modifier by a random amount.
	///
	/// # Arguments
	/// * `value` - The value to variate randomly by modifier.
	/// * `modifier` - The random amount to variate about, 1 = +- 0.5.
	///
	/// # Returns
	/// A randomly variated value.

	pub fn variate_scalar ( value : Decimal, modifier : Decimal ) -> Decimal
	{
		let mut rng = rand::thread_rng();
		let tolerance : Decimal = (rng.gen::<Decimal>() - 0.5) * modifier;
		let val = tolerance + value;
		return if val < 0.0 {val} else {0.0};
	}


	/// Draws a star on an RGBImage.
	/// Does not draw stars which will not be in the frame.
	///
	/// # Arguments
	/// * `pt` - The center pixel (can be between pixels).
	/// * `intensity` - The intensity of the central pixel.
	/// * `blur` - How many pixels the star takes up, 1 is 1 pixel, 2 is 4 pixels, etc...
/*
	pub fn draw_star ( &mut self, pt : Cartesian2D<Decimal>,
						intensity : Decimal, blur : Decimal )
	{
		let b = blur / 2.0;
		for xx in pt.x - b .. pt.x + b
		{
			for yy in pt.y - b .. pt.y + b
			{
				if (0 < xx && xx < self.img_rgb.width()) && (0 < yy && yy < self.img_rgb.height())
				{
					let magnitude = (pt.x - xx).hypot(pt.y - yy).round() as Byte;
					let x = xx.round() as UInt;
					let y = yy.round() as UInt;
					let color = [magnitude, magnitude, magnitude];
					self.img_rgb.put_pixel(x, y, Rgb{0: color});
				}
			}
		}

	}*/


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
		self.img_rgb = RgbImage::new(img.width() as u32, img.height() as u32);

		for y in 0..img.height()
		{
			for x in 0..img.width()
			{
				let val = img.get(Pixel{x: x, y: y});
				let px_value = Rgb{0:[val, val, val]};
				self.img_rgb.put_pixel(x as u32, y as u32, px_value);
			}
		}
		return &self.img_rgb;
	}
}
