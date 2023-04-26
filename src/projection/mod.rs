//! Projection is a module which can convert between 2D camera space and 3D world space.  
//!
//! It is also in charge of removing distortion from images/simulating it.  
//! 
//! # Parameters
//! * `Extrinsic Parameters` - Rotating and moving world space so that the point with (0, 0, +x) is in front of the camera.
//! * `Intrinsic Parameters` - Projecting the camera space onto the image plane with scale and translation.
//! 
//! # Spaces
//! * `SpaceWorld ` - The location of the objects in a 3D environment relative to [0,0,0], 0 rotation.
//! * `SpaceCamera` - The location of the objects in a 3D environment relative to the camera (forward is +z).
//! * `SpaceImage ` - The object projected onto a flat surface on the sensor.
//! * `SpacePixel`  - SpaceImage rounded to the nearest pixel.
//!
//! A good summary of functions used can be found in this [opencv guide](https://docs.opencv.org/3.4/d9/d0c/group__calib3d.html).

use util::units::Matrix;
use util::units::Vector3;
use util::units::Vector2;
use util::units::Pixel;



pub mod extrinsic_parameters;
pub mod intrinsic_parameters;

/// The extrinsic (world transformation) parameters.
/// These are rotations and translations to convert world coordinates into the camera coordinates.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ExtrinsicParameters
{
	/// The matrix in charge of rotating the point from world to camera.
	rotation    : Matrix<3,3>,
	/// The applied translation to move the point from world to camera coordinates.
	pub translation : Vector3,
}

/// The intrinsic (camera properties) parameters.
/// This is used to specify how points in 3D space will be projected onto the camera plane.
#[derive(Debug, Copy, Clone, PartialEq)]
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


/// A 3D point in a scene with no relation to the camera.
#[derive(PartialEq, Debug, Clone, Copy)] 
pub struct SpaceWorld  ( pub Vector3 );
/// A 3D point relative to the camera with +z being the direction the camera is looking.
#[derive(PartialEq, Debug, Clone, Copy)] 
pub struct SpaceCamera ( pub Vector3 );
/// A 2D point representing the position relative to the image sensor flat to the sensor.
#[derive(PartialEq, Debug, Clone, Copy)] 
pub struct SpaceImage  ( pub Vector2 );
/// A 2D point on the image plane which has been rounded to the nearest pixel (LOSSY).
#[derive(PartialEq, Debug, Clone, Copy)] 
pub struct SpacePixel  ( pub Pixel );


/// The planar space the coordinate is located in.
/// If you are trying to turn a 3d point into an image, start at World Coordinates.
/// If you are trying to turn an image into a 3d point, start at Pixel Coordinates.
enum Space
{
	/// A 3D point in a scene with no relation to the camera.
	World  (SpaceWorld),
	/// A 3D point relative to the camera with +z being the direction the camera is looking.
	Camera (SpaceCamera),
	/// A 2D point representing the position relative to the image sensor flat to the sensor.
	Image  (SpaceImage),
	/// A 2D point on the image plane which has been rounded to the nearest pixel (LOSSY).
	Pixel  (SpacePixel),
}

