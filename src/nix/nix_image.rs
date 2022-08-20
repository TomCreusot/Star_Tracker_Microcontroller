use crate::image_processing::Image;
use crate::nix::NixImage;
use image::io::Reader as ImageReader;
use image::{RgbImage, Rgb};
use crate::util::aliases::Byte;
use crate::util::aliases::Decimal;
use crate::util::units::Pixel;
use crate::util::units::Vector2;
use crate::util::units::Vector3;
use std::env;

use projection::SpaceWorld;
use projection::SpaceCamera;
use projection::SpaceImage;
use projection::IntrinsicParameters;
use projection::ExtrinsicParameters;


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
	pub fn clone ( &self ) -> Self
	{
		let mut img = RgbImage::new(self.width() as u32, self.height() as u32);
		for y in 0..self.height()
		{
			for x in 0..self.width()
			{
				let val = self.get(Pixel{x: x, y: y});
				let px_value = Rgb{0:[val, val, val]};
				img.put_pixel(x as u32, y as u32, px_value);
			}
		}
		return Self{img_rgb: img};
	}

	/// Copies the provided image.
	/// # Arguments
	/// * `img` - The image to copy.
	pub fn duplicate ( image : &dyn Image ) -> Self
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


	/// Copies the provided image.
	/// # Arguments
	/// * `size` - The size of the image.
	pub fn new ( size : Pixel ) -> Self
	{
		let img = RgbImage::new(size.x as u32, size.y as u32);
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


	/// Projects a star onto the image if possible.
	/// Only draws the star if the star is within the bounds of the sensor.
	/// Only works with a field of view of or less than 180 degrees.
	///
	/// # Arguments
	/// * `star` - The star in world space to be projected.
	/// * `size` - The radius of the star.
	/// * `color`- The color.
	/// * `intrinsic` - The properties of the lens.
	/// * `extrinsic` - The camera rotation.
	///
	/// # Returns
	/// True if at least part of the star is visible.
	pub fn draw_star (
		&mut self,
		star:      SpaceWorld,
		size:      Decimal,
		color:     [u8;3],
		intrinsic: IntrinsicParameters,
		extrinsic: ExtrinsicParameters  ) -> bool
	{
		let mut visible : bool = false;
		let camera_space : SpaceCamera = extrinsic.to_image(star);

		// Stops any objects projected behind the image from appearing on the image.
		if 0.0 <= camera_space.0.dot(Vector3{x: 0.0, y: 0.0, z: 1.0})
		{
			let image_space : SpaceImage   = intrinsic.to_image(camera_space);
			let center      : Vector2      = image_space.0;
			let mut xx = center.x - size - 1.0;
			while xx < center.x + size + 1.0
			{
				let mut yy = center.y - size - 1.0;
				while yy < center.y + size + 1.0
				{
					let point : Vector2 = Vector2{x: xx.round(), y: yy.round()};
					let dist : Decimal = (center - point).magnitude();
					let x : i32 = point.x as i32;
					let y : i32 = point.y as i32;
					if 0 < x && x < self.img_rgb.width() as i32 &&
					   0 < y && y < self.img_rgb.height() as i32 &&
					   dist < size
					{
						// need to check that the image is getting brighter.
						let prev_color = self.img_rgb.get_pixel(x as u32, y as u32);
						let prev_intens = (prev_color[0] as Decimal +
										  prev_color[1] as Decimal +
										  prev_color[2] as Decimal) / 255.0;
						let curr_intens = (color[0] as Decimal +
										   color[1] as Decimal +
										   color[2] as Decimal) * (size - dist)/size / 255.0;
						if prev_intens < curr_intens
						{
							let c=[
								(color[0] as Decimal * curr_intens) as u8,
								(color[1] as Decimal * curr_intens) as u8,
								(color[2] as Decimal * curr_intens) as u8];
							self.img_rgb.put_pixel(x as u32, y as u32, Rgb{0:c});
						}
						visible = true;
					}
					yy+=1.0;
				}
				xx+=1.0;
			}
		}
		return visible;
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
		self.img_rgb = NixImage::duplicate(img).img_rgb;
		return &self.img_rgb;
	}
}
