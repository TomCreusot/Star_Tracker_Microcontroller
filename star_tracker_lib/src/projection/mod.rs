//! `projection` is a module which can convert between 2D camera space and 3D world space.  
//!
//! It is also in charge of removing distortion from images/simulating it.  
//! 
//! # Parameters
//! * `Extrinsic Parameters` - Rotating and moving world space so that the point with (0, 0, +x) is in front of the camera.
//! * `Intrinsic Parameters` - Projecting the camera space onto the image plane with scale and translation.
//! 
//! # Spaces
//! * `SpaceWorld ` - The location of the objects in a 3D environment relative to \[0,0,0\], 0 rotation.
//! * `SpaceCamera` - The location of the objects in a 3D environment relative to the camera (forward is +z).
//! * `SpaceImage ` - The object projected onto a flat surface on the sensor.
//! * `SpacePixel`  - SpaceImage rounded to the nearest pixel.
//!
//!
//! # Image to 3d (satellite) coordinates.
//! ```
//! use star_tracker_lib::projection::IntrinsicParameters;
//! use star_tracker_lib::projection::ExtrinsicParameters;
//! use star_tracker_lib::projection::SpaceImage;
//! use star_tracker_lib::projection::SpaceCamera;
//! use star_tracker_lib::projection::SpaceWorld;
//!
//! use star_tracker_lib::util::units::Degrees;
//! use star_tracker_lib::util::units::Vector2;
//! use star_tracker_lib::util::units::Vector3;
//! use star_tracker_lib::util::units::Equatorial;
//! use star_tracker_lib::util::aliases::Decimal;
//!
//!
//! let fov = Degrees(20.0).to_radians();
//! let img_width: Decimal  = 100.0;
//! let img_height: Decimal = 100.0;
//!
//! // If you know the field of view, just use the pixel size as the sensor size...
//!	let sensor_horizontal = (img_width.powf(2.0) + img_height.powf(2.0)).sqrt();
//! let img_center = Vector2{x: img_width / 2.0, y: img_height / 2.0};
//!
//! // Creates the intrinsic parameters from the field of view.
//!	let intrinsic_projection = IntrinsicParameters::from_fov(fov, sensor_horizontal, img_center);
//!
//! // Creates the extrinsic parameters.
//! // Using *look_at* sets a point which is forward and a direction which is up.
//! // With this, you can specify what orientation the camera is relative to the spacecraft.
//!	let reference_forward = Vector3{x: 0.0, y: 0.0, z: 1.0}.to_equatorial();
//! let reference_up      = Vector3{x: 0.0, y: 1.0, z: 0.0}.to_equatorial();
//! let extrinsic_projection = ExtrinsicParameters::look_at(reference_forward, reference_up)
//! 	.expect("Ensure extrinsic projection up and forward are not the same value.");
//!
//! let point       : SpaceImage  = SpaceImage(Vector2{x: img_width / 2.0, y: img_height / 2.0}); // Center of image.
//! let camera_space: SpaceCamera = intrinsic_projection.from_image(point);          // 3d local coordinates.
//! let world_space : SpaceWorld  = extrinsic_projection.from_image(camera_space);  // 3d satellite coords.
//! assert_eq!(world_space.0, reference_forward.to_vector3()); 
//! ```
//!
//!
//! # 3d (satellite) to 2d coordinates (simulation image).
//! ```
//! use star_tracker_lib::projection::IntrinsicParameters;
//! use star_tracker_lib::projection::ExtrinsicParameters;
//! use star_tracker_lib::projection::SpaceImage;
//! use star_tracker_lib::projection::SpaceCamera;
//! use star_tracker_lib::projection::SpaceWorld;
//!
//! use star_tracker_lib::util::units::Degrees;
//! use star_tracker_lib::util::units::Vector2;
//! use star_tracker_lib::util::units::Vector3;
//! use star_tracker_lib::util::units::Equatorial;
//! use star_tracker_lib::util::aliases::Decimal;
//!
//!
//! let fov = Degrees(20.0).to_radians();
//! let img_width: Decimal  = 100.0;
//! let img_height: Decimal = 100.0;
//!
//! // If you know the field of view, just use the pixel size as the sensor size...
//!	let sensor_horizontal = (img_width.powf(2.0) + img_height.powf(2.0)).sqrt();
//! let img_center = Vector2{x: img_width / 2.0, y: img_height / 2.0};
//!
//! // Creates the intrinsic parameters from the field of view.
//!	let intrinsic_projection = IntrinsicParameters::from_fov(fov, sensor_horizontal, img_center);
//!
//! // Creates the extrinsic parameters.
//! // Using *look_at* sets a point which is forward and a direction which is up.
//! // With this, you can specify what orientation the camera is relative to the spacecraft.
//!	let reference_forward = Vector3{x: 0.0, y: 0.0, z: 1.0}.to_equatorial();
//! let reference_up      = Vector3{x: 0.0, y: 1.0, z: 0.0}.to_equatorial();
//! let extrinsic_projection = ExtrinsicParameters::look_at(reference_forward, reference_up)
//! 	.expect("Ensure extrinsic projection up and forward are not the same value.");
//!
//! let point       : SpaceWorld  = SpaceWorld(Equatorial::north().to_vector3());  // North.
//! let camera_space: SpaceCamera = extrinsic_projection.to_image(point);         // 3d local coordinates.
//! let image_space : SpaceImage  = intrinsic_projection.to_image(camera_space); // 2d image coords.
//! assert_eq!(image_space.0, Vector2{x: img_width / 2.0, y: img_height / 2.0}); 
//! ```
//! A good summary of functions used can be found in this [opencv guide](https://docs.opencv.org/3.4/d9/d0c/group__calib3d.html).

use crate::core_include::*;

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

