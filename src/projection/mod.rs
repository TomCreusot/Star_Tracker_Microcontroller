//! Projection is a module which can convert between 2D camera space and 3D world space.
//! It is also in charge of removing distortion from images/simulating it. 
//!
//! A good summary of functions used can be found in this [opencv guide](https://docs.opencv.org/3.4/d9/d0c/group__calib3d.html).

use util::units::Matrix;
use util::units::Cartesian3D;
use util::units::PixelWeighted;



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
	pub translation : Cartesian3D,
}



/// The intrinsic (camera properties) parameters.
/// This is used to specify how points in 3D space will be projected onto the camera plane.
pub struct IntrinsicParameters
{
	/// Distance (in units) between image plane and projection center *camera_constant*.
	pub focal_length    : PixelWeighted,

	// / The size (in units) of each pixel.
	// pub pixel_scale     : PixelWeighted,

	/// The center point of the image.
	pub principle_point : PixelWeighted,

	// / How skewed the image is.
	// pub shear           : Decimal,
}



impl Transformation
{
	pub fn to_image ( &self, point: Cartesian3D ) -> PixelWeighted
	{
		return self.intrinsic.to_image(self.extrinsic.to_image(point));
	}
}