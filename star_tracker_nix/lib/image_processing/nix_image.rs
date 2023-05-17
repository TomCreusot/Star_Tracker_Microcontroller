use crate::image_processing::Color;


use star_tracker_lib::image_processing::Image;

use star_tracker_lib::util::aliases::Decimal;
use star_tracker_lib::util::units::Pixel;
use star_tracker_lib::util::units::Vector2;
use star_tracker_lib::util::units::Vector3;

use star_tracker_lib::projection::SpaceWorld;
use star_tracker_lib::projection::SpaceCamera;
use star_tracker_lib::projection::SpaceImage;
use star_tracker_lib::projection::IntrinsicParameters;
use star_tracker_lib::projection::ExtrinsicParameters;

pub trait NixImage: Image
{
	/// Saves the image to the specified location.
	/// Creates directory if does not exist.
	/// # Arguments
	/// * `name` - The path, name and extension of the image.
	 fn save ( &self, name: &str );

	/// Gets the pixel as a coloured value.
	/// # Arguments
	/// * `px` - The pixel to get.
	fn get ( &self, px: Pixel ) -> Color;

	/// Sets the pixel as a coloured value.
	/// # Arguments
	/// * `px` - The pixel to set.
	/// * `value` - The color to set at the specified position.
	fn set ( &mut self, px: Pixel, value: Color );

	/// Sets all the pixels to the specified color.
	/// (Allows you to reuse the same image).
	fn reset_color ( &mut self, color: Color )
	{
		for xx in 0..self.width()
		{
			for yy in 0..self.height()
			{
				NixImage::set(self, Pixel{x: xx, y: yy}, color);
			}
		}
	}

	/// Draws marker onto the image, exludes the center pixel so you can analyse it.
	/// # Arguments
	/// * `px`		- The position.
	/// * `size`	- The size of the crosshair.
	/// * `color`	- The color to draw.
	/// * `img_rgb` - the image to draw on.
	fn draw_point ( &mut self, px: Pixel, size: usize, color: Color)
	{
		for yy in (px.y as usize + 1)..std::cmp::min(px.y as usize + size, self.height())
		{
			NixImage::set(self, Pixel{x: px.x, y: yy}, color);
		}
		for xx in (px.x as usize + 1)..std::cmp::min(px.x as usize + size, self.width())
		{
			NixImage::set(self, Pixel{x: xx, y: px.y}, color);
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
	 fn draw_star_projection (
		&mut self,
		star:      SpaceWorld,
		size:      Decimal,
		color:     Color,
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

			visible = self.draw_star(size, color, SpaceImage(center));
		}
		return visible;
	}


	/// Draws a star in Image Space
	/// # Arguments
	/// * `color` - The color of the star.
	/// * `pt` - The center of the image.
	/// # Returns
	/// True if atleast part of the image is visible.
	 fn draw_star ( &mut self, size: Decimal, color: Color, pt: SpaceImage ) -> bool
	{
		let color_ideal = color.get_color();
		let mut visible : bool = false;
		let mut xx = pt.0.x - size - 1.0;
		while xx < pt.0.x + size + 1.0
		{
			let mut yy = pt.0.y - size - 1.0;
			while yy < pt.0.y + size + 1.0
			{
				let point : Vector2 = Vector2{x: xx.round(), y: yy.round()};
				let dist : Decimal = (pt.0 - point).magnitude();
				let x : i32 = point.x as i32;
				let y : i32 = point.y as i32;
				if 0 < x && x < self.width() as i32 &&
					0 < y && y < self.height() as i32 &&
					dist < size
				{
					// Add the previous color to the new color.
					let px = Pixel{x: x as usize, y: y as usize};
					let intensity  = (size - dist) / size; // 0 to 1.
					let prev_color = NixImage::get(self, px).get_color();
					let curr_color =
						[(intensity * color_ideal[0] as Decimal) as u8,
						 (intensity * color_ideal[1] as Decimal) as u8,
						 (intensity * color_ideal[2] as Decimal) as u8];

					let c=[
						prev_color[0].saturating_add(curr_color[0]),
						prev_color[1].saturating_add(curr_color[1]),
						prev_color[2].saturating_add(curr_color[2])];
						NixImage::set(self, px, Color::Custom(c[0], c[1], c[2]));
					visible = true;
				}
				yy+=1.0;
			}
			xx+=1.0;
		}
		return visible;
	}


	/// Draws a line between 2 points in 3d.
	/// The line will be projected.
	/// This is a very basic method which draws a set of points along the path.
	/// # Arguments
	/// * `color` - The color of the line to draw.
	/// * `pt_1`  - The first point of the line.
	/// * `pt_2`  - The other point.
	/// * `intrinsic` - How the line should be projected onto the lens.
	/// * `extrinsic` - Where the camera is looking.
	 fn draw_3d_line (
		&mut self,
		color: Color,
		pt_1: SpaceWorld,
		pt_2: SpaceWorld,
		intrinsic: IntrinsicParameters,
		extrinsic: ExtrinsicParameters )
	{
		let difference = pt_2.0 - pt_1.0;
		let direction_result = difference.normalized();
		if let Ok(direction) = direction_result
		{
			let magnitude = difference.magnitude();

			let mut position = pt_1;

			while (position.0-pt_1.0).dot(difference) / magnitude < magnitude
			{
				let point = intrinsic.to_image(extrinsic.to_image(position)).0;
				let pixel = Pixel{x: point.x as usize, y: point.y as usize};
				if self.valid_pixel(pixel)
				{
					NixImage::set(self, pixel, color);
				}
				position.0 = position.0 + direction * 0.01;
			}
		}
	}

}
