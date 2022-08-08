//! Implementation of ExtrinsicParameters
use super::ExtrinsicParameters;

use util::units::Matrix;
use util::units::MatPos;
// use util::units::Degrees;
// use util::units::Radians;
// use util::units::AngleAxis;
use util::units::Equatorial;
use util::units::Cartesian3D;

// use util::aliases::DECIMAL_PRECISION;

impl ExtrinsicParameters
{
	/// Performs a transformation to convert from world coordinates to camera coordinates.
	///
	///
	pub fn to_image ( &self, point: Cartesian3D ) -> Cartesian3D
	{
		return self.rotation.multiply(point) + self.translation;
	}
	
	
	/// Performs a transformation to convert from camera coordinates to world coordinates.
	///
	///
	pub fn from_image ( &self, point: Cartesian3D ) -> Cartesian3D
	{
		return self.rotation.transposed().multiply(point - self.translation);
	}
	
	
	/// A way of converting a looking direction and an up direction into a rotation transformation.
	/// There are two implementations; matrix and quaternion.
	/// This is the [Matrix Version](https://www.geertarien.com/blog/2017/07/30/breakdown-of-the-lookAt-function-in-OpenGL/).
	/// This is the [Quaternion Version](https://answers.unity.com/questions/467614/what-is-the-source-code-of-quaternionlookrotation.html).
	/// Observing these, it appears that the quaternion version just uses the matrix version.
	/// This is alot of unnecessary computation, thus it was decided to use the conventional matrix.
	/// This method implements the opengl `LookAt` function.
	///   
	/// The parameters are equatorial to reduce input checking.
	/// # Arguments
	/// * `forward` - The direction to look.
	/// * `up` - The upwards direction of the camera, Cannot be the same as forward.
	///
	/// # Returns
	/// An ExtrinsicParameter based on the looking direction of the camera. 
	/// 
	/// # Asserts
	/// forward != up
	pub fn look_at ( forward: Equatorial, up: Equatorial ) -> Self
	{
		assert_ne!(forward,up,"forward cannot be up as there is no way to know the orientation.");
		
		let mut matrix : Matrix<3, 3> = Matrix::new();

		// Matrix parameters.
		let z_axis = forward.to_cartesian3();           				// Z axis goes through the center of the frame.
		let x_axis = z_axis.cross(&up.to_cartesian3()).normalized();	// X axis rotation applied as axis angle adjacent to z.
		let y_axis = x_axis.cross(&z_axis).normalized();   				// Same logic as x.

		// Rotation
		matrix.set(MatPos{row: 0, col: 0}, x_axis.x);
		matrix.set(MatPos{row: 0, col: 1}, x_axis.y);
		matrix.set(MatPos{row: 0, col: 2}, x_axis.z);

		matrix.set(MatPos{row: 1, col: 0}, y_axis.x);
		matrix.set(MatPos{row: 1, col: 1}, y_axis.y);
		matrix.set(MatPos{row: 1, col: 2}, y_axis.z);

		matrix.set(MatPos{row: 2, col: 0}, z_axis.x);
		matrix.set(MatPos{row: 2, col: 1}, z_axis.y);
		matrix.set(MatPos{row: 2, col: 2}, z_axis.z);

		// Translation
		// matrix.set(MatPos{row: 3, col: 0}, x_axis.dot(&center));
		// matrix.set(MatPos{row: 3, col: 1}, y_axis.dot(&center));
		// matrix.set(MatPos{row: 3, col: 2}, z_axis.dot(&center));

		// Homogeneous identity
		// matrix.set(MatPos{row: 3, col: 3}, 1.0);
		return Self{rotation: matrix, translation: Cartesian3D{x: 0.0, y: 0.0, z: 0.0}};
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
	
	use projection::ExtrinsicParameters;
	use util::units::MatPos;
	use util::units::Matrix;
	use util::units::Degrees;
	use util::units::Radians;
	// use util::units::AngleAxis;
	use util::units::Equatorial;
	use util::units::Cartesian3D;

	// use util::aliases::M_PI;
	
	
	#[test]
	fn test_to_image ( )
	{
		let mut rotation : Matrix<3,3> = Matrix::new();
		// Rotates z onto x | x onto y | y onto z
		rotation.set(MatPos{row: 0, col: 0}, 0.0);
		rotation.set(MatPos{row: 0, col: 1}, 0.0);
		rotation.set(MatPos{row: 0, col: 2}, 1.0);
		
		rotation.set(MatPos{row: 1, col: 0}, 1.0);
		rotation.set(MatPos{row: 1, col: 1}, 0.0);
		rotation.set(MatPos{row: 1, col: 2}, 0.0);
		
		rotation.set(MatPos{row: 2, col: 0}, 0.0);
		rotation.set(MatPos{row: 2, col: 1}, 1.0);
		rotation.set(MatPos{row: 2, col: 2}, 0.0);
		
		let translation = Cartesian3D{x: 0.1, y: 0.2, z: 0.3};
		let param = ExtrinsicParameters{rotation: rotation, translation};
		
		let initial  = Cartesian3D{x: 2.0, y: 3.0, z: 4.0};
		let expected = Cartesian3D{x: 4.1, y: 2.2, z: 3.3};
		assert_eq!(param.to_image(initial), expected);
	}
	
	#[test]
	fn test_from_image ( )
	{
		let mut rotation : Matrix<3,3> = Matrix::new();
		// Rotates z onto x | x onto y | y onto z
		rotation.set(MatPos{row: 0, col: 0}, 0.0);
		rotation.set(MatPos{row: 0, col: 1}, 0.0);
		rotation.set(MatPos{row: 0, col: 2}, 1.0);
		
		rotation.set(MatPos{row: 1, col: 0}, 1.0);
		rotation.set(MatPos{row: 1, col: 1}, 0.0);
		rotation.set(MatPos{row: 1, col: 2}, 0.0);
		
		rotation.set(MatPos{row: 2, col: 0}, 0.0);
		rotation.set(MatPos{row: 2, col: 1}, 1.0);
		rotation.set(MatPos{row: 2, col: 2}, 0.0);
		
		// let translation = Cartesian3D{x: 0.0, y: 0.0, z: 0.0};
		let translation = Cartesian3D{x: 0.1, y: 0.2, z: 0.3};
		let param = ExtrinsicParameters{rotation: rotation, translation};
		
		let initial  = Cartesian3D{x: 2.0, y: 3.0, z: 4.0};
		let camera   = param.to_image(initial);
		assert_eq!(param.from_image(camera), initial);
	}
	
	
	
	
	
	
	
	
	
	
	#[test]
	fn test_look_at ( )
	{
		let mut rng = rand::thread_rng();
		for _ in 0..100
		{	
			let range_ra  = Equatorial::range_ra();
			let range_dec = Equatorial::range_dec();
	
			let start = Equatorial {
				ra:  Radians(rng.gen_range(range_ra.start().0..range_ra.end().0)),
				dec: Radians(rng.gen_range(range_dec.start().0..range_dec.end().0)) };
			
			let up = Equatorial {
				ra:  Radians(rng.gen_range(range_ra.start().0..range_ra.end().0)),
				dec: Radians(rng.gen_range(range_dec.start().0..range_dec.end().0)) };
			
			let rotation = ExtrinsicParameters::look_at(start, up).rotation;
			let start_to_end = rotation.multiply(start.to_cartesian3());
			assert_eq!(start_to_end, Cartesian3D{x: 0.0, y: 0.0, z: 1.0});
		}
	}
	
	
	#[test]
	fn test_look_at_default ( )
	{
		// Facing Up
		let mut a = Equatorial{ra: Degrees(10.0).to_radians(), dec: Degrees(30.0).to_radians()};
		let mut up = Equatorial{ra: Degrees(190.0).to_radians(), dec: Degrees(60.0).to_radians()};
		let mut rotation = ExtrinsicParameters::look_at(a, up).rotation;
		let mut start_to_end = rotation.multiply(a.to_cartesian3());
		assert_eq!(start_to_end, Cartesian3D{x: 0.0, y: 0.0, z: 1.0});
			
		// Facing Down
		a = Equatorial{ra: Degrees(-10.0).to_radians(), dec: Degrees(40.0).to_radians()};
		up = Equatorial{ra: Degrees(-10.0).to_radians(), dec: Degrees(50.0).to_radians()};
		rotation = ExtrinsicParameters::look_at(a, up).rotation;
		start_to_end = rotation.multiply(a.to_cartesian3());
		assert_eq!(start_to_end, Cartesian3D{x: 0.0, y: 0.0, z: 1.0});
	
		// Facing Down More
		a = Equatorial{ra: Degrees(20.0).to_radians(), dec: Degrees(-40.0).to_radians()};
		up = Equatorial{ra: Degrees(200.0).to_radians(), dec: Degrees(-50.0).to_radians()};
		rotation = ExtrinsicParameters::look_at(a, up).rotation;
		start_to_end = rotation.multiply(a.to_cartesian3());
		assert_eq!(start_to_end, Cartesian3D{x: 0.0, y: 0.0, z: 1.0});
	}
	
	#[test]
	#[should_panic]
	#[allow(unused_variables)]
	fn test_look_at_panic ( )
	{
		let a = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		ExtrinsicParameters::look_at(a,a);
	}
	
}