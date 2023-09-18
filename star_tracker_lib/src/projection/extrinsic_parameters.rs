//! Implementation of ExtrinsicParameters
use crate::core_include::*;

use super::ExtrinsicParameters;
use super::SpaceWorld;
use super::SpaceCamera;

use crate::util::units::Matrix;
use crate::util::units::MatPos;
use crate::util::units::Equatorial;
use crate::util::units::Vector3;

use crate::util::err::Errors;
use crate::util::err::Error;

impl ExtrinsicParameters
{
	/// Performs a transformation to convert from world coordinates to camera coordinates.
	/// # Arguments
	/// * `point` - The point in world coordinates.
	/// # Returns
	/// The point in camera coordinates.
	pub fn to_image ( &self, point: SpaceWorld ) -> SpaceCamera
	{
		return SpaceCamera(self.rotation.multiply(point.0) + self.translation);
	}


	/// Performs a transformation to convert from camera coordinates to world coordinates.
	/// # Arguments
	/// * `point` - The point in camera coordinates.
	/// # Returns
	/// The point in world coordinates.
	pub fn from_image ( &self, point: SpaceCamera ) -> SpaceWorld
	{
		return SpaceWorld(self.rotation.transposed().multiply(point.0 - self.translation));
	}


	/// A way of converting a looking direction and an up direction into a rotation transformation.  
	/// There are two implementations; matrix and quaternion.  
	/// This is the [Matrix Version](https://www.geertarien.com/blog/2017/07/30/breakdown-of-the-lookAt-function-in-OpenGL/).  
	/// This is the [Quaternion Version](https://answers.unity.com/questions/467614/what-is-the-source-code-of-quaternionlookrotation.html).  
	/// Observing these, it appears that the quaternion version just uses the matrix version.  
	/// This is a lot of unnecessary computation, thus it was decided to use the conventional matrix.  
	/// This method implements the opengl `LookAt` function.  
	///
	/// The parameters are equatorial to reduce input checking.
	/// # Arguments
	/// * `forward` - The direction to look.
	/// * `up` - The upwards direction of the camera, Cannot be the same as forward.
	///
	/// # Returns
	/// An ExtrinsicParameter based on the looking direction of the camera.  
	/// If forward is equal to up, Errors::InvalidValue will be produced.   
	///
	/// # Asserts
	/// forward != up
	pub fn look_at ( forward: Equatorial, up: Equatorial ) -> Error<Self>
	{
		if forward == up
		{
			return Err(Errors::InvalidValue);
		}

		let mut matrix : Matrix<3, 3> = Matrix::new();

		// Matrix parameters.
		// Z axis goes through the center of the frame.
		let z_axis = forward.to_vector3(); 
		// X axis rotation applied as axis angle adjacent to z.
		let x_axis = up.to_vector3().cross(z_axis).normalized().expect("Already checked.");
		// Same logic as x.
		let y_axis = z_axis.cross(x_axis).normalized().expect("Already checked.");

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

		// matrix = matrix.transposed();

		// Translation
		// matrix.set(MatPos{row: 3, col: 0}, x_axis.dot(center));
		// matrix.set(MatPos{row: 3, col: 1}, y_axis.dot(center));
		// matrix.set(MatPos{row: 3, col: 2}, z_axis.dot(center));

		// Homogeneous identity
		// matrix.set(MatPos{row: 3, col: 3}, 1.0);
		return Result::Ok(Self{rotation: matrix, translation: Vector3{x: 0.0, y: 0.0, z: 0.0}});
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

	use crate::projection::ExtrinsicParameters;
	use crate::projection::SpaceWorld;
	use crate::projection::SpaceCamera;

	use crate::util::units::Equatorial;
	use crate::util::units::Vector3;
	use crate::util::units::Degrees;
	use crate::util::units::Radians;
	use crate::util::units::MatPos;
	use crate::util::units::Matrix;

	use crate::util::err::Errors;
	use crate::util::err::Error;
	use crate::util::test::TestEqual;

//###############################################################################################//
//
//										Rotations
//
// pub fn to_image   ( &self, SpaceWorld  ) -> SpaceCamera
// pub fn from_image ( &self, SpaceCamera ) -> SpaceWorld
//
//###############################################################################################//
//										~ to_image ~											 //
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

		let translation = Vector3{x: 0.1, y: 0.2, z: 0.3};
		let param = ExtrinsicParameters{rotation: rotation, translation};

		let initial  = SpaceWorld(Vector3{x: 2.0, y: 3.0, z: 4.0});
		let expected = SpaceCamera(Vector3{x: 4.1, y: 2.2, z: 3.3});
		assert_eq!(param.to_image(initial), expected);
	}

//										~ from_image ~											 //
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

		// let translation = Vector3{x: 0.0, y: 0.0, z: 0.0};
		let translation = Vector3{x: 0.1, y: 0.2, z: 0.3};
		let param = ExtrinsicParameters{rotation: rotation, translation};

		let initial  = SpaceWorld(Vector3{x: 2.0, y: 3.0, z: 4.0});
		let camera   = param.to_image(initial);
		assert_eq!(param.from_image(camera), initial);
	}







//###############################################################################################//
//
//										Constructors
//
// pub fn look_at ( Equatoria, Equaorial ) -> Error<Self>
//
//###############################################################################################//

	#[no_coverage]
	#[test]
	// Checks if the forward vector is always the center of the output.
	fn test_look_at_center ( ) -> Error<()>
	{
		// Facing Up
		let mut forward = 
			Equatorial{ra: Degrees(10.0).to_radians(), dec: Degrees(30.0).to_radians()};
		let mut up = 
			Equatorial{ra: Degrees(190.0).to_radians(), dec: Degrees(60.0).to_radians()};
		let mut rotation = 
			ExtrinsicParameters::look_at(forward, up)?.rotation;
		let mut start_to_end = rotation.multiply(forward.to_vector3());

		start_to_end.test_close(&Vector3{x: 0.0, y: 0.0, z: 1.0}, 0.001); // !

		// Facing Down
		forward = Equatorial{ra: Degrees(-10.0).to_radians(), dec: Degrees(40.0).to_radians()};
		up = Equatorial{ra: Degrees(-10.0).to_radians(), dec: Degrees(50.0).to_radians()};
		rotation = ExtrinsicParameters::look_at(forward, up)?.rotation;
		start_to_end = rotation.multiply(forward.to_vector3());

		start_to_end.test_close(&Vector3{x: 0.0, y: 0.0, z: 1.0}, 0.001); // !

		// Facing Down More
		forward = Equatorial{ra: Degrees(20.0).to_radians(), dec: Degrees(-40.0).to_radians()};
		up = Equatorial{ra: Degrees(200.0).to_radians(), dec: Degrees(-50.0).to_radians()};
		rotation = ExtrinsicParameters::look_at(forward, up)?.rotation;
		start_to_end = rotation.multiply(forward.to_vector3());
		
		start_to_end.test_close(&Vector3{x: 0.0, y: 0.0, z: 1.0}, 0.001); // !

		// Facing random directions.
		let mut rng = rand::thread_rng();
		let range_ra  = Equatorial::range_ra();
		let range_dec = Equatorial::range_dec();
		for _ in 0..100
		{
			let forward = Equatorial {
				ra:  Radians(rng.gen_range(range_ra.start().0..range_ra.end().0)),
				dec: Radians(rng.gen_range(range_dec.start().0..range_dec.end().0)) };
			let up = Equatorial {
				ra:  Radians(rng.gen_range(range_ra.start().0..range_ra.end().0)),
				dec: Radians(rng.gen_range(range_dec.start().0..range_dec.end().0)) };
			rotation = ExtrinsicParameters::look_at(forward, up)?.rotation;
			
			rotation.multiply(forward.to_vector3()).test_close(
				&Vector3{x: 0.0, y: 0.0, z: 1.0}, 0.001); // !
		}

		return Ok(());
	}


	#[no_coverage]
	#[test]
	// Ensures the output is not flipped.
	// * A point on the axis adjacent to forward and up must be on the correct side after the transform.
	fn test_look_at_specularity_adjacent_axis ( ) -> Error<()>
	{
		let mut rng = rand::thread_rng();
		let range_ra  = Equatorial::range_ra();
		let range_dec = Equatorial::range_dec();

		// Adjacent Axis
		for _ in 0..100
		{
			let forward = Equatorial {
				ra:  Radians(rng.gen_range(range_ra.start().0..range_ra.end().0)),
				dec: Radians(rng.gen_range(range_dec.start().0..range_dec.end().0)) };
			let up = Equatorial {
				ra:  Radians(rng.gen_range(range_ra.start().0..range_ra.end().0)),
				dec: Radians(rng.gen_range(range_dec.start().0..range_dec.end().0)) };

			let rotation = 
				ExtrinsicParameters::look_at(forward, up).expect("look_at failed").rotation;
			let left = forward.to_vector3().cross(up.to_vector3()).normalized()
				.expect("cross failed");
			// [0 0 1] x [0 1 0] = [-1 0 0]
			rotation.multiply(left).test_close(&Vector3{x: -1.0, y: 0.0, z: 0.0}, 0.001); // !
		}
		return Ok(());
	}


	#[no_coverage]
	#[test]
	// Ensures the output is not flipped.
	// * A point on the plane of forward and up must be on the correct point after the transform.
	fn test_look_at_specularity_same_plane ( ) -> Error<()>
	{
		let mut rng = rand::thread_rng();
		let range_ra  = Equatorial::range_ra();
		let range_dec = Equatorial::range_dec();

		// Same Plane
		for _ in 0..100
		{
			let forward = Equatorial {
				ra:  Radians(rng.gen_range(range_ra.start().0..range_ra.end().0)),
				dec: Radians(rng.gen_range(range_dec.start().0..range_dec.end().0)) };
			let up = Equatorial {
				ra:  Radians(rng.gen_range(range_ra.start().0..range_ra.end().0)),
				dec: Radians(rng.gen_range(range_dec.start().0..range_dec.end().0)) };

			let rotation = 
				ExtrinsicParameters::look_at(forward, up).expect("inputs are not equal").rotation;
			// forward cross up = -(perpendicular vector).
			// (perpendicular vector) cross forward = (proper up).
			// proper up is in the direction of up but with the magnitude of the projected component of up.
			// find the normalized vector.
			let proper_up =forward.to_vector3().cross(up.to_vector3()).cross(forward.to_vector3());
			
			rotation.multiply(proper_up).normalized()?.test_close(
				&Vector3{x: 0.0, y: 1.0, z: 0.0}, 0.001);
		}
		return Ok(());
	}


	#[no_coverage]
	#[test]
	// Errors::InvalidValue should be returned if the inputs are equal.
	fn test_look_at_error ( )
	{
		let a = Equatorial{ra: Radians(0.0), dec: Radians(0.0)};
		assert_eq!(ExtrinsicParameters::look_at(a,a), Err(Errors::InvalidValue));
	}

}
