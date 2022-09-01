//! Implementation of ExtrinsicParameters
use super::ExtrinsicParameters;
use super::SpaceWorld;
use super::SpaceCamera;

use util::units::Matrix;
use util::units::MatPos;
use util::units::Equatorial;
use util::units::Vector3;

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
	///
	/// # Asserts
	/// forward != up
	pub fn look_at ( forward: Equatorial, up: Equatorial ) -> Self
	{
		assert_ne!(forward,up,"forward cannot be up as there is no way to know the orientation.");

		let mut matrix : Matrix<3, 3> = Matrix::new();

		// Matrix parameters.
		let z_axis = forward.to_vector3();		      			// Z axis goes through the center of the frame.
		let x_axis = up.to_vector3().cross(z_axis).normalized();// X axis rotation applied as axis angle adjacent to z.
		let y_axis = z_axis.cross(x_axis).normalized();   		// Same logic as x.

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
		return Self{rotation: matrix, translation: Vector3{x: 0.0, y: 0.0, z: 0.0}};
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
	use projection::SpaceWorld;
	use projection::SpaceCamera;
	use util::units::MatPos;
	use util::units::Matrix;
	use util::units::Degrees;
	use util::units::Radians;
	// use util::units::AngleAxis;
	use util::units::Equatorial;
	use util::units::Vector3;

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

		let translation = Vector3{x: 0.1, y: 0.2, z: 0.3};
		let param = ExtrinsicParameters{rotation: rotation, translation};

		let initial  = SpaceWorld(Vector3{x: 2.0, y: 3.0, z: 4.0});
		let expected = SpaceCamera(Vector3{x: 4.1, y: 2.2, z: 3.3});
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

		// let translation = Vector3{x: 0.0, y: 0.0, z: 0.0};
		let translation = Vector3{x: 0.1, y: 0.2, z: 0.3};
		let param = ExtrinsicParameters{rotation: rotation, translation};

		let initial  = SpaceWorld(Vector3{x: 2.0, y: 3.0, z: 4.0});
		let camera   = param.to_image(initial);
		assert_eq!(param.from_image(camera), initial);
	}









	#[test]
	// Checks if the forward vector is always the center of the output.
	fn test_look_at_center ( )
	{
		// Facing Up
		let mut forward = Equatorial{ra: Degrees(10.0).to_radians(), dec: Degrees(30.0).to_radians()};
		let mut up = Equatorial{ra: Degrees(190.0).to_radians(), dec: Degrees(60.0).to_radians()};
		let mut rotation = ExtrinsicParameters::look_at(forward, up).rotation;
		let mut start_to_end = rotation.multiply(forward.to_vector3());
		assert_eq!(start_to_end, Vector3{x: 0.0, y: 0.0, z: 1.0});

		// Facing Down
		forward = Equatorial{ra: Degrees(-10.0).to_radians(), dec: Degrees(40.0).to_radians()};
		up = Equatorial{ra: Degrees(-10.0).to_radians(), dec: Degrees(50.0).to_radians()};
		rotation = ExtrinsicParameters::look_at(forward, up).rotation;
		start_to_end = rotation.multiply(forward.to_vector3());
		assert_eq!(start_to_end, Vector3{x: 0.0, y: 0.0, z: 1.0});

		// Facing Down More
		forward = Equatorial{ra: Degrees(20.0).to_radians(), dec: Degrees(-40.0).to_radians()};
		up = Equatorial{ra: Degrees(200.0).to_radians(), dec: Degrees(-50.0).to_radians()};
		rotation = ExtrinsicParameters::look_at(forward, up).rotation;
		start_to_end = rotation.multiply(forward.to_vector3());
		assert_eq!(start_to_end, Vector3{x: 0.0, y: 0.0, z: 1.0});

		// Facing random directions.
		let mut rng = rand::thread_rng();
		let range_ra  = Equatorial::range_ra();
		let range_dec = Equatorial::range_dec();
		for _i in 0..100
		{
			let forward = Equatorial {
				ra:  Radians(rng.gen_range(range_ra.start().0..range_ra.end().0)),
				dec: Radians(rng.gen_range(range_dec.start().0..range_dec.end().0)) };
			let up = Equatorial {
				ra:  Radians(rng.gen_range(range_ra.start().0..range_ra.end().0)),
				dec: Radians(rng.gen_range(range_dec.start().0..range_dec.end().0)) };
			rotation = ExtrinsicParameters::look_at(forward, up).rotation;
			assert_eq!(rotation.multiply(forward.to_vector3()), Vector3{x: 0.0, y: 0.0, z: 1.0});
		}
	}


	#[test]
	// Ensures the output is not flipped.
	// * A point on the axis adjacent to forward and up must be on the correct side after the transform.
	fn test_look_at_specularity_adjacent_axis ( )
	{
		let mut rng = rand::thread_rng();
		let range_ra  = Equatorial::range_ra();
		let range_dec = Equatorial::range_dec();

		// Adjacent Axis
		for _i in 0..100
		{
			let forward = Equatorial {
				ra:  Radians(rng.gen_range(range_ra.start().0..range_ra.end().0)),
				dec: Radians(rng.gen_range(range_dec.start().0..range_dec.end().0)) };
			let up = Equatorial {
				ra:  Radians(rng.gen_range(range_ra.start().0..range_ra.end().0)),
				dec: Radians(rng.gen_range(range_dec.start().0..range_dec.end().0)) };

			let rotation = ExtrinsicParameters::look_at(forward, up).rotation;
			let left = forward.to_vector3().cross(up.to_vector3()).normalized();
			// [0 0 1] x [0 1 0] = [-1 0 0]
			assert_eq!(rotation.multiply(left), Vector3{x: -1.0, y: 0.0, z: 0.0});
		}
	}


	#[test]
	// Ensures the output is not flipped.
	// * A point on the plane of forward and up must be on the correct point after the transform.
	fn test_look_at_specularity_same_plane ( )
	{
		let mut rng = rand::thread_rng();
		let range_ra  = Equatorial::range_ra();
		let range_dec = Equatorial::range_dec();

		// Same Plane
		for _i in 0..100
		{
			let forward = Equatorial {
				ra:  Radians(rng.gen_range(range_ra.start().0..range_ra.end().0)),
				dec: Radians(rng.gen_range(range_dec.start().0..range_dec.end().0)) };
			let up = Equatorial {
				ra:  Radians(rng.gen_range(range_ra.start().0..range_ra.end().0)),
				dec: Radians(rng.gen_range(range_dec.start().0..range_dec.end().0)) };

			let rotation = ExtrinsicParameters::look_at(forward, up).rotation;
			// forward cross up = -(perpendicular vector).
			// (perpendicular vector) cross forward = (proper up).
			// proper up is in the direction of up but with the magnitude of the projected component of up.
			// find the normalized vector.
			let proper_up =forward.to_vector3().cross(up.to_vector3()).cross(forward.to_vector3());
			assert_eq!(rotation.multiply(proper_up).normalized(), Vector3{x: 0.0, y: 1.0, z: 0.0});
		}
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