//! Projection is a module which can convert between 2D camera space and 3D world space.
//! It is also in charge of removing distortion from images/simulating it.
//!
//! A good summary of functions used can be found in this [opencv guide](https://docs.opencv.org/3.4/d9/d0c/group__calib3d.html).

use util::units::Matrix;
use util::units::Vector3;
use util::units::Vector2;
use util::units::Pixel;



pub mod extrinsic_parameters;
pub mod intrinsic_parameters;


/// The transformation between 3D world and image pixel coordinates.
pub struct Transformation
{
	pub extrinsic : ExtrinsicParameters,
	pub intrinsic : IntrinsicParameters,
}

/// The extrinsic (world transformation) parameters.
/// These are rotations and translations to convert world coordinates into the camera coordinates.
pub struct ExtrinsicParameters
{
	/// The matrix in charge of rotating the point from world to camera.
	rotation    : Matrix<3,3>,
	/// The applied translation to move the point from world to camera coordinates.
	pub translation : Vector3,
}



/// The intrinsic (camera properties) parameters.
/// This is used to specify how points in 3D space will be projected onto the camera plane.
pub struct IntrinsicParameters
{
	/// Distance (in units) between image plane and projection center *camera_constant*.
	pub focal_length    : Vector2,

	// / The size (in units) of each pixel.
	// pub pixel_scale     : Vector2,

	/// The center point of the image.
	pub principle_point : Vector2,

	// / How skewed the image is.
	// pub shear           : Decimal,
}



impl Transformation
{
	pub fn to_image ( &self, point: Vector3 ) -> Vector2
	{
		return self.intrinsic.to_image(self.extrinsic.to_image(point));
	}
}


/// The planar space the coordinate is located in.
/// If you are trying to turn a 3d point into an image, start at World Coordinates.
/// If you are trying to turn an image into a 3d point, start at Pixel Coordinates.
enum Space
{
	/// A 3D point in a scene with no relation to the camera.
	World  (Vector3),
	/// A 3D point relative to the camera with +z being the direction the camera is looking.
	Camera (Vector3),
	/// A 2D point representing the position relative to the image sensor flat to the sensor.
	Image  (Vector2),
	/// A 2D point on the image plane which has been rounded to the nearest pixel (LOSSY).
	Pixel  (Pixel),
}
