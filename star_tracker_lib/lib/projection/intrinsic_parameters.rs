//! Implementation of [IntrinsicParameters](crate::projection::intrinsic_parameters)
use super::IntrinsicParameters;
use super::SpaceCamera;
use super::SpaceImage;

use util::units::Matrix;
use util::units::MatPos;
use util::units::Radians;
use util::units::Vector2;
use util::units::Vector3;

use util::aliases::Decimal;

impl IntrinsicParameters
{
	/// Converts a 3d camera space to a 2d image space with the provided intrinsic parameters.  
	/// Lossy as depth is sacrificed.
	pub fn to_image ( &self, point: SpaceCamera ) -> SpaceImage
	{
		let mut matrix : Matrix<3,3> = Matrix::identity();
		matrix.set(MatPos{row: 0, col: 0}, self.focal_length.x);
		matrix.set(MatPos{row: 1, col: 1}, self.focal_length.y);

		matrix.set(MatPos{row: 0, col: 2}, self.principle_point.x);
		matrix.set(MatPos{row: 1, col: 2}, self.principle_point.y);

		let out_3d = matrix.multiply(point.0);
		let out_2d = out_3d / out_3d.z;

		return SpaceImage(Vector2{x: out_2d.x, y: out_2d.y});
	}


	/// Converts a 2d image space to a 3d camera space with the provided intrinsic parameters.  
	/// As depth is not known, z will always be +1.
	pub fn from_image ( &self, point: SpaceImage ) -> SpaceCamera
	{
		let x = (point.0.x - self.principle_point.x) / self.focal_length.x;
		let y = (point.0.y - self.principle_point.y) / self.focal_length.y;
		return SpaceCamera(Vector3{x: x, y: y, z: 1.0});
	}


	/// Generates the intrinsic parameters from the field of view.
	/// This is only useful when simulating the shot as many distortion effects are ignored.
	///
	/// # Arguments
	/// * `fov` - The field of view (in any dimension).   
	/// This must be measured in the same direction as the image size.
	/// * `img_size` - The size of the sensor in the same direction as the fov, if simulated, use units of pixels.
	/// * `principle_point - The center of the image.
	pub fn from_fov ( fov: Radians, sensor_size: Decimal, principle_point: Vector2 ) -> Self
	{
		// Focal length = image size * working distance / fov (in the same units as working_dist).
		let unit_fov     = (fov).sin();       // The width as a unit vector.
		let working_dist = (fov / 2.0).cos(); // The distance of the distance from the plane.
		let focal_length = sensor_size / unit_fov * working_dist;
		let focal_lengths = Vector2{x: focal_length, y: focal_length};
		return Self{focal_length: focal_lengths, principle_point: principle_point};
	}
}



//###############################################################################################//
//###############################################################################################//
//
//										Unit Tests
//
//###############################################################################################//
//###############################################################################################//

#[cfg(test)]
mod test
{
	use rand::prelude::*;

	use crate::projection::IntrinsicParameters;
	use crate::projection::SpaceImage;
	use crate::projection::SpaceCamera;

	use crate::util::units::Vector3;
	use crate::util::units::Vector2;
	use crate::util::units::Degrees;

	use crate::util::test::TestEqual;

//###############################################################################################//
//
//										Rotations
//
// pub fn to_image   ( &self, SpaceCamera ) -> SpaceImage
// pub fn from_image ( &self, SpaceImage  ) -> SpaceCamera
//
//###############################################################################################//
//										~ to_image ~											 //
	#[test]
	// With a movement of the principle_point but a default focal length should have the same scale but different origin.
	fn test_to_image_principle ( )
	{
		let focal_length    = Vector2{x: 1.0,  y: 1.0};
		let principle_point = Vector2{x: 11.1, y: 22.2};
		let param = 
			IntrinsicParameters{focal_length: focal_length, principle_point: principle_point};

		let initial  = SpaceCamera(Vector3{x: 2.0, y: 3.0, z: 1.0});
		let expected = SpaceImage(Vector2{x: 2.0 + 11.1, y: 3.0 + 22.2});
		assert_eq!(param.to_image(initial), expected);
	}

	#[test]
	// With a central principle_point but a varied focal length, the point should be scaled towareds/away from the origin.
	fn test_to_image_focal_length ( )
	{
		let focal_length    = Vector2{x: 5.0,  y: 6.0};
		let principle_point = Vector2{x: 0.0, y: 0.0};
		let param = 
			IntrinsicParameters{focal_length: focal_length, principle_point: principle_point};

		let initial  = SpaceCamera(Vector3{x: 2.0, y: 3.0, z: 2.0});
		let expected = SpaceImage(Vector2{x: 2.0 * 5.0 / 2.0, y: 3.0 * 6.0 / 2.0});
		assert_eq!(param.to_image(initial), expected);
	}
	
	
	
	#[test]
	// to_image() should be reversed with from_image() except everything will be projected to z=+1.
	fn test_from_image ( )
	{
		let mut rng = rand::thread_rng();
		for _ in 0..100
		{
			// let focal_length    = Vector2{x: rng.gen_range(0.1..9.0), y: rng.gen_range(0.1..9.0)};
			// let principle_point = Vector2{x: rng.gen_range(0.1..9.0), y: rng.gen_range(0.1..9.0)};
			let focal_length    = Vector2{x: 1.0, y: 1.0};
			let principle_point = Vector2{x: 0.0, y: 0.0};
			let param = 
				IntrinsicParameters{focal_length: focal_length, principle_point: principle_point};
			
			let mut point = SpaceCamera(Vector3{
				x: rng.gen_range(0.1..10.0), 
				y: rng.gen_range(0.1..10.0),
				z: rng.gen_range(0.1..10.0)});
			
			let projected = param.to_image(point);
			point.0 = point.0 / point.0.z; // Homogeneous (set everything to z=+1).
			assert_eq!(param.from_image(projected), point);
		}
	}




//###############################################################################################//
//
//										Constructors
//
// pub fn from_fov ( Radians, Decimal ) -> Self
//
//###############################################################################################//

	#[test]
	fn test_from_fov ( )
	{
		let fov  = Degrees(45.0).as_radians();
		let size = 10.0;
		
		let param = IntrinsicParameters::from_fov(fov, size, Vector2{x: 0.0, y: 0.0});
		
		// If the field of view of the camera is 45 deg and 10 units wide,
		// a point 45 deg from the center must be on the edge of the sensor.
		let sin_45 = Degrees(45.0 / 2.0).as_radians().sin();
		let initial  = SpaceCamera(Vector3{x: sin_45, y: sin_45, z: 1.0});
		let expected = SpaceImage(Vector2{x: 5.0, y: 5.0});
		
		println!("{}",sin_45);
		println!("{:?}", param);
		assert_eq!(param.to_image(initial), expected);
	}

	#[test]
	fn test_from_fov_principle_point ( )
	{
		let fov  = Degrees(45.0).as_radians();
		let size = 5.0;
		let principle_point = Vector2{x: 4.0 / 2.0, y: 3.0 / 2.0};
		// 3,4,5 triangle
		let param = IntrinsicParameters::from_fov(fov, size, principle_point);
		
		let initial  = SpaceImage(Vector2{x: 2.0, y: 1.5});
		let expected = SpaceCamera(Vector3{x: 0.0, y: 0.0, z: 1.0});
		assert_eq!(param.from_image(initial), expected);
		
		let initial  = SpaceImage(Vector2{x: 4.0, y: 3.0});
		param.from_image(initial).0.to_equatorial().dec.test_equal(&(fov / 2.0));
		
		let initial  = SpaceImage(Vector2{x: 0.0, y: 0.0});
		param.from_image(initial).0.to_equatorial().dec.test_equal(&(fov / 2.0));
	}
}