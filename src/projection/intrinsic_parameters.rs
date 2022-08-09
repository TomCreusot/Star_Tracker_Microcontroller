//! Implementation of IntrinsicParameters
use super::IntrinsicParameters;

use util::units::Matrix;
use util::units::MatPos;
use util::units::Radians;
use util::units::Vector2;
use util::units::Vector3;

use util::aliases::Decimal;

impl IntrinsicParameters
{
	pub fn to_image ( &self, point: Vector3 ) -> Vector2
	{
		let mut matrix : Matrix<3,3> = Matrix::identity();
		matrix.set(MatPos{row: 0, col: 0}, self.focal_length.x);
		matrix.set(MatPos{row: 1, col: 1}, self.focal_length.y);

		matrix.set(MatPos{row: 0, col: 2}, self.principle_point.x);
		matrix.set(MatPos{row: 1, col: 2}, self.principle_point.y);

		let out_3d = matrix.multiply(point);
		let out_2d = out_3d / out_3d.z;

		return Vector2{x: out_2d.x, y: out_2d.y};
	}


	// pub fn from_image ( &self, point: Vector2 ) -> Vector3
	// {
	// 	let x = (point - self.principle_point.x) / self.focal_length;
	// 	let y = (point - self.principle_point.y) / self.focal_length;
	// 	return Vector3{x: x, y: y, z}
	// }


	/// Generates the intrinsic parameters from the field of view.
	/// This is only useful when simulating the shot as many distortion effects are ignored.
	///
	/// # Arguments
	/// * `fov` - The field of view. This must be measured in the same direction as the image size.
	/// * `img_size` - The size of the sensor in the same direction as the fov, if simulated, use units of pixels.
	pub fn from_fov ( fov: Radians, img_size: Decimal ) -> Self
	{
		// Sensor Height / Field Of View * Distance To Object Of Interest (1 as unit sphere).
		let focal_length = img_size / fov.0 * 1.0;
		let focal_lengths = Vector2{x: focal_length, y: focal_length};
		let principle_point = Vector2{x: img_size / 2.0, y: img_size / 2.0};
		return Self{focal_length: focal_lengths, principle_point: principle_point};
	}
}
