//! `attitude_determination` is the process of **voting** for the most likely attitude, this is the final step in the process.  
//!
//! The following operations would have already been executed:  
//! * reading image.
//! * thresholding.
//! * finding blobs.
//! * finding unique star pairs.
//!   
//! This stage uses given sensor position and given actual position of any positioning system to estimate the *most likely* attitude.  
//! A rotation matrix/vector is generated by the determination system which can be applied to any local point and it will be converted to the world position.  
//! # [Triad](https://www.coursera.org/lecture/spacecraft-dynamics-kinematics/2-triad-method-definition-xYght)  
//! *+* Easy to implement.  
//! *-* Only works with 2 vectors.  
//! *-* Only considers 1.5 sensor values.  
//!  
//! This method only allows 1.5 measurements and is **not** ideal for a star tracker.  
//! The methodology is to get 2, 3D vector measurements in its local and global frame.  
//! The most accurate vector is assigned to an axis of the rotation matrix.  
//! The least accurate vector is then crossed with the most accurate vector to give another axis.  
//! The last axis is generated by crossing the previous 2 axis.  
//! As this only accounts for < 2 input vectors, it is not ideal for a star tracker.  
//!
//!
//! # [Davenports Q Method](https://www.coursera.org/lecture/spacecraft-dynamics-kinematics/4-devenports-q-method-BOjJ6)
//! *+* Can have infinite elements.  
//! *+* Does not have singularities (always works).  
//! *-* Very slow (calculates eigenvalues).  
//! *-* Old, was used a lot but not used now.  
//! *-* Confusing and lack of instructions.  
//!
//! This method is far more accurate then the triad method and allows infinite elements and considers every element with a given weighting.  
//! It is an old method which uses a set of complicated equations to calculate [Wahba's problem](https://www.coursera.org/lecture/spacecraft-dynamics-kinematics/3-wahbas-problem-definition-8hcFM). These equations end up requiring the calculation of an eigenvalue/eigenvector pair from a 4x4 matrix which makes this method undesirable over others.  
//!
//! # [QUEST](https://www.coursera.org/lecture/spacecraft-dynamics-kinematics/5-quest-AQZKX)
//! *+* Most common method today.  
//! *+* A lot faster than davenports method but similar logic.  
//! *-* Potential singularity (CRP coordinates). <- Can be solved.  
//! *-* Confusing and lack of instructions.  
//!
//! This method uses iteration to find the eigenvalue/eigenvectors for davenports method.  
//! Unfortuantly at some point, the coordinate system is converted to CRP which has a singularity.  
//!
//! # [ESOQ](https://scholarsmine.mst.edu/cgi/viewcontent.cgi?article=8724&context=masters_theses#page=90 )  
//! *+* No singularity  
//! *+* A lot faster than davenport but same logic.  
//! *!* Made by Mortari (he has done a lot of research into attitude determination, however it is hard to understand his reports).  
//! *-* Confusing and lack of instructions.  
//!
//!
//!
//! # [OLEA](https://www.coursera.org/lecture/spacecraft-dynamics-kinematics/6-1-example-of-olae-Jl6Y1)
//!
//!
//! # How To Use
//! ```
//! use star_tracker_lib::attitude_determination::AttitudeDetermination;
//! use star_tracker_lib::attitude_determination::Quest;
//! use star_tracker_lib::util::units::Quaternion;
//! use star_tracker_lib::util::units::Vector3;
//! use star_tracker_lib::util::units::Match;
//!
//! // If you setup projection correctly, the points will be relative to the front of the spacecraft.
//! // In this situation, I decided that was z: +1.
//! let reference_forward = Vector3{x: 0.0, y: 0.0, z: 1.0};
//!
//! // Lets just assume these points are from a valid constellation from tracking_mode.
//! let found_stars = vec!
//! [
//!     // The weight is relative to the other points, the higher the weight the more reliable.
//! 	Match{input: Vector3{x: 0.0, y: 0.0, z: 1.0}, output: Vector3{x: 0.0, y: 0.0, z: 1.0}, weight: 1.0},
//! 	Match{input: Vector3{x: 0.0, y: 1.0, z: 0.0}, output: Vector3{x: 0.0, y: 1.0, z: 0.0}, weight: 1.0},
//! 	Match{input: Vector3{x: 1.0, y: 0.0, z: 0.0}, output: Vector3{x: 1.0, y: 0.0, z: 0.0}, weight: 1.0},
//! 	Match{input: Vector3{x: 0.0, y: 1.0, z: 1.0}, output: Vector3{x: 0.0, y: 1.0, z: 1.0}, weight: 1.0},
//! ];
//!
//!
//! let rotate_to_cam  : Quaternion = Quest::estimate(&found_stars, None); // Quaternion to rotate world space to camera space.
//!	let rotate_to_world: Quaternion = rotate_to_cam.conjugate();           // Quaternion to rotate camera space to world space.
//!	let world_center = rotate_to_world.rotate_point(reference_forward);    // Rotate spacecraft front to world coordinates
//! ```

use crate::util::units::Match;
use crate::util::units::Vector3;
use crate::util::units::Quaternion;
use crate::util::aliases::Decimal;
use crate::util::list::List;

pub mod wahba;
pub mod quest;

pub trait AttitudeDetermination
{
	/// Finds the most likely pointing direction from the given (observed, reference) positions.
	/// # Arguments
	/// * `positions` - The (input: observed, output: reference, weighting: __).  
	/// 	The weighting is just a ratio, it does not matter the size, just how it relates to other weightings.
	/// * `lambda` - 
	/// 	For quest algorithm, to find the correct attitude, the neuton raphson method is used.  
	/// 	This method will loop and slowly decrease the gap between the current and previous prediction.  
	/// 	Achieving perfect precision comparing the 2 values will take up computation power.  
	/// 	By specifying a precision, the computational requirements are lowered.  
	/// 	To use a default estimate value, you can provide None and it will use LAMBDA_PRECISION.  
	/// # Returns
	/// A quaternion which rotates output to input.  
	/// Use Quaternion.conjugate() to get a rotation from input to output. 
	fn estimate ( positions: &dyn List<Match<Vector3>>, lambda: Option<Decimal> ) -> Quaternion;
}

/// Formula required for any Wahba based methods.  
/// Does not solve the problem.
pub struct Wahba ( );


/// Implements AttitudeDetermination through the quest method.   
/// Use `estimate` to estimate the attitude of the camera.
pub struct Quest( );

/// For quest algorithm, to find the correct attitude, the neuton raphson method is used.  
/// This method will loop and slowly decrease the gap between the current and previous prediction.  
/// Achieving perfect precision comparing the 2 values will take up computation power.  
/// By specifying a precision, the computational requirements are lowered.  
const LAMBDA_PRECISION: Decimal = 0.1;



//###############################################################################################//
//###############################################################################################//
//
//										Unit Tests
//
//###############################################################################################//
//###############################################################################################//

//
#[cfg(test)]
mod test
{
	use attitude_determination::Quest;
	use attitude_determination::AttitudeDetermination;

	use util::units::Vector3;
	use util::units::Quaternion;
	use util::units::AngleAxis;
	use util::units::Radians;
	use util::units::Degrees;
	use util::units::Match;
	use util::aliases::Decimal;
	use util::aliases::DECIMAL_PRECISION;
	use util::test::TestEqual;

	use util::list::ArrayList;
	use util::list::List;

	use rand::prelude::*;



	// Generates a set of coordinate pairs.  
	// The input coordinate is a random point on a unit sphere.  
	// The output coordinate is that point rotated using `rotation` with an error of `variation`.  
	// When run, the input to output should be close to correct but with some system noise.  
	//
	// The weight has a random variation specified by `var_weight`, if this is set to 1, 
	// it means one weight may be double the size, 2 is 2/3, 3 is 3/4...  
	fn random_coordinates <const N : usize> (
			rotation : AngleAxis, variation : AngleAxis, var_weight : Decimal
		) -> ArrayList<Match<Vector3>, N>
	{
		let mut rng = rand::thread_rng();
		let mut coords : ArrayList<Match<Vector3>, N> = ArrayList::new();
		while !coords.is_full()
		{
			let mut input = Vector3
			{ x: rng.gen::<Decimal>(), y: rng.gen::<Decimal>(), z: rng.gen::<Decimal>() };
			input.normalize().expect("Error if 0,0,0");

			let angle = rotation.angle + Radians(rng.gen::<Decimal>() - 0.5) * variation.angle;
			let mut axis = rotation.axis;
			axis.x += variation.axis.x * (rng.gen::<Decimal>() - 0.5);
			axis.y += variation.axis.y * (rng.gen::<Decimal>() - 0.5);
			axis.z += variation.axis.z * (rng.gen::<Decimal>() - 0.5);
			axis.normalize().expect("Error if 0,0,0");
			let output = (AngleAxis{angle: angle, axis: axis}.to_quaternion()).rotate_point(input);
			let weight = 1.0 + var_weight * rng.gen::<Decimal>();
			let element : Match<Vector3> = Match{input: input, output: output, weight: weight};

			coords.push_back(element).expect("already did error check?");
		}
		return coords;
	}


	// 
	// #[test]
	// // Testing the result from this algorithm and a matlab algorithm.
	// fn test_quest_matlab ( )
	// {
	// 	let mut input : ArrayList<Match<Vector3>, 10> = ArrayList::new();
	// 	let _ = input.push_back(Match{input: Vector3{x: 1.0, y: 0.0, z: 0.3},
	// 					output: Vector3{x: 0.1, y: 1.0, z: 1.0}, weight: 1.0});
	// 	let _ = input.push_back(Match{input: Vector3{x: 1.0, y: 0.4, z: 0.0},
	// 					output: Vector3{x: 0.5, y: 1.0, z: 1.0}, weight: 1.0});
	// 	let _ = input.push_back(Match{input: Vector3{x: 1.0, y: 0.3, z: 0.0},
	// 					output: Vector3{x: 0.3, y: 1.0, z: 1.0}, weight: 1.0});
	// 	let _ = input.push_back(Match{input: Vector3{x: 1.0, y: 0.5, z: 0.0},
	// 					output: Vector3{x: 0.2, y: 1.0, z: 1.0}, weight: 1.0});
	// 
	// 	for i in 0..input.size()
	// 	{
	// 		let mut m = input.get(i);
	// 		m.input.normalize();
	// 		m.output.normalize();
	// 		input.set(i, m).expect("HUH");
	// 	}
	// 
	// 	let output_q  = Quest::estimate::<ConstQuest>(&input);
	// 	let mut output_aa = output_q.to_angle_axis();
	// 
	// 	output_aa.axis.normalize();
	// 
	// 	let expected_q = Quaternion{
	// 		w: 0.547549945381170,
	// 		x: -0.461042818527695,
	// 		y: -0.689219735118163,
	// 		z: -0.112270804400634};
	// 
	// 	println!("{:?} \t\t\t {:?}", output_q, expected_q);
	// 	println!("{:?} \n{:?}", output_aa, expected_q.to_angle_axis());
	// 	println!("{:?}", output_aa.axis.angle_distance(expected_q.to_angle_axis().axis).to_degrees());
	// 
	// 	assert!(output_q.test_equal(&expected_q));
	// }
	// 




	#[test]
	// If the input rotated by a constant is the output,
	// The found quaternion should be the opposite of the initial rotation.
	fn test_quest_perfect_values ( )
	{
		let angle = Degrees(90.0).to_radians();
		let axis = Vector3{x: 1.0, y: 0.0, z: 0.0};
		let mut angle_axis = AngleAxis{angle: angle, axis: axis};

		let angle_var = Degrees(0.0).to_radians();
		let axis_var = Vector3{x: 0.0, y: 0.0, z: 0.0};
		let angle_axis_var = AngleAxis{angle: angle_var, axis: axis_var};

		let input = random_coordinates::<100>(angle_axis, angle_axis_var, 0.0);


		let rotation = Quest::estimate(&input, None);
		angle_axis.angle = -angle_axis.angle;
		assert!(rotation.test_equal(&angle_axis.to_quaternion()));
	}


	#[test]
	// This uses perfect coordinate values, however the weighting is varied.
	// This should not create error as all the rotations are the same.
	fn test_quest_bad_weight ( )
	{
		let angle = Degrees(90.0).to_radians();
		let axis = Vector3{x: 1.0, y: 0.0, z: 0.0};
		let mut angle_axis = AngleAxis{angle: angle, axis: axis};
		
		let angle_var = Degrees(0.0).to_radians();
		let axis_var = Vector3{x: 0.0, y: 0.0, z: 0.0};
		let angle_axis_var = AngleAxis{angle: angle_var, axis: axis_var};
		
		let input = random_coordinates::<100>(angle_axis, angle_axis_var, 10.0);
		
		
		let rotation = Quest::estimate(&input, None);
		angle_axis.angle = -angle_axis.angle;
		assert!(rotation.test_equal(&angle_axis.to_quaternion()));
	}


	#[test]
	// If there is some varyation in the rotation (angle axis-axis), there may be slight error,
	// However, it should not be substantial as the error will be averaged out.
	fn test_quest_bad_axis ( )
	{
		let angle = Degrees(90.0).to_radians();
		let axis = Vector3{x: 1.0, y: 0.0, z: 0.0};
		let angle_axis = AngleAxis{angle: angle, axis: axis};

		let angle_var = Degrees(0.0).to_radians();
		let axis_var = Vector3{x: 0.1, y: 0.1, z: 0.1};
		let angle_axis_var = AngleAxis{angle: angle_var, axis: axis_var};

		let input = random_coordinates::<100>(angle_axis, angle_axis_var, 0.0);


		let rotation = Quest::estimate(&input, None);
		
		// Rotation opposite provided angle axis.
		println!("{}", rotation.dot(angle_axis.to_quaternion()).abs());
		assert!(rotation.dot(angle_axis.to_quaternion()).abs() < 0.01);
	}



	#[test]
	// If there is some varyation in the rotation (angle axis-angle), there may be slight error,
	// However, it should not be substantial as the error will be averaged out.
	fn test_quest_bad_angle ( )
	{
		let angle = Degrees(90.0).to_radians();
		let axis = Vector3{x: 1.0, y: 0.0, z: 0.0};
		let angle_axis = AngleAxis{angle: angle, axis: axis};

		let angle_var = Degrees(10.0).to_radians();
		let axis_var = Vector3{x: 0.0, y: 0.0, z: 0.0};
		let angle_axis_var = AngleAxis{angle: angle_var, axis: axis_var};

		let input = random_coordinates::<100>(angle_axis, angle_axis_var, 0.0);


		let rotation = Quest::estimate(&input, None);
		let ang_out = rotation.to_angle_axis();

		println!("{:?}", ang_out);
		// Rotation opposite provided angle axis.
		println!("{}", rotation.dot(angle_axis.to_quaternion()).abs());
		assert!(rotation.dot(angle_axis.to_quaternion()).abs() < 0.01);
	}


	#[test]
	// If everything is wrong, there it should still be accurate with such a high sample space.
	fn test_quest_bad_angle_weight_axis ( )
	{
		let angle = Degrees(90.0).to_radians();
		let axis = Vector3{x: 1.0, y: 0.0, z: 0.0};
		let angle_axis = AngleAxis{angle: angle, axis: axis};

		let angle_var = Degrees(10.0).to_radians();
		let axis_var = Vector3{x: 0.1, y: 0.1, z: 0.1};
		let angle_axis_var = AngleAxis{angle: angle_var, axis: axis_var};

		let input = random_coordinates::<100>(angle_axis, angle_axis_var, 1000.0);


		let rotation = Quest::estimate(&input, None);
		let ang_out = rotation.to_angle_axis();

		println!("{:?}", ang_out);
		// Rotation opposite provided angle axis.
		println!("{}", rotation.dot(angle_axis.to_quaternion()).abs());
		assert!(rotation.dot(angle_axis.to_quaternion()).abs() < 0.01);
	}
	
	
	
	
	#[test]
	// The previous tests test that quaternions are
	fn test_quest_4_perfect_values ( )
	{
		
	}
	
	#[test]
	fn test_quaternion_reversibility ( )
	{
		let mut rng = rand::thread_rng();
		for _i in 0..100
		{
			let angle = Degrees(rng.gen::<Decimal>() * 360.0 * 2.0 - 360.0).to_radians();
			let axis = Vector3
			{ x: rng.gen::<Decimal>(), y: rng.gen::<Decimal>(), z: rng.gen::<Decimal>() }
			.normalized().expect("Error if 0,0,0");
			let angle_axis = AngleAxis{angle: angle, axis: axis};

			let rotation     = angle_axis.to_quaternion();
			let rotation_inv = rotation.conjugate();

			let coord = Vector3
			{ x: rng.gen::<Decimal>(), y: rng.gen::<Decimal>(), z: rng.gen::<Decimal>() }
				.normalized().expect("Error if 0,0,0");
			
			assert!(coord.test_equal(&rotation_inv.rotate_point(rotation.rotate_point(coord))));
		}
	}


}
