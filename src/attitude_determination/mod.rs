//! `attitude_determination` is the final step in the process.
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

use util::units::Cartesian3D;
use util::units::Quaternion;
use util::list::List;
use tracking_mode::Match;
use config::AttitudeDeterminationConsts;

pub mod wahba;
pub mod quest;

pub trait AttitudeDetermination
{
	/// Finds the most likely pointing direction from the given (observed, reference) positions.
	/// # Arguments
	/// * `positions` - The (input: observed, output: reference, weighting: __).
	/// The weighting is just a ratio, it does not matter the size, just how it relates to other weightings.
	fn estimate <T: 'static> ( positions: &dyn List<Match<Cartesian3D>> ) -> Quaternion
		where T: AttitudeDeterminationConsts;
}

/// Formula required for any Wahba based methods.
/// Does not solve the problem.
pub struct Wahba ( );


/// Implements AttitudeDetermination through the quest method.
pub struct Quest( );


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
	use config::AttitudeDeterminationConsts;
	use attitude_determination::AttitudeDetermination;
	use attitude_determination::Quest;

	use util::units::Cartesian3D;
	// use util::units::Quaternion;
	use util::units::AngleAxis;
	use util::units::Radians;
	use util::units::Degrees;
	use util::aliases::Decimal;
	use util::aliases::DECIMAL_PRECISION;
	use util::test::TestEqual;

	use util::list::ArrayList;
	use util::list::List;

	use tracking_mode::Match;

	use rand::prelude::*;





	pub struct ConstQuest ( );
	impl AttitudeDeterminationConsts for ConstQuest
	{
		const LAMBDA_PRECISION : Decimal = DECIMAL_PRECISION * 100000.0;
	}


	fn random_coordinates <const N : usize> (
			rotation : AngleAxis, variation : AngleAxis, var_weight : Decimal
		) -> ArrayList<Match<Cartesian3D>, N>
	{
		let mut rng = rand::thread_rng();
		let mut coords : ArrayList<Match<Cartesian3D>, N> = ArrayList::new();
		while !coords.is_full()
		{
			let mut input = Cartesian3D
			{ x: rng.gen::<Decimal>(), y: rng.gen::<Decimal>(), z: rng.gen::<Decimal>() };
			input.normalize();

			let angle = rotation.angle + Radians(rng.gen::<Decimal>() - 0.5) * variation.angle;
			let mut axis = rotation.axis;
			axis.x += variation.axis.x * (rng.gen::<Decimal>() - 0.5);
			axis.y += variation.axis.y * (rng.gen::<Decimal>() - 0.5);
			axis.z += variation.axis.z * (rng.gen::<Decimal>() - 0.5);
			axis.normalize();
			let output = (AngleAxis{angle: angle, axis: axis}.to_quaternion()).rotate_point(input);
			let weight = var_weight + rng.gen::<Decimal>();
			let element : Match<Cartesian3D> = Match{input: input, output: output, weight: weight};

			coords.push_back(element).expect("HELLO");
		}
		return coords;
	}


/*
	#[test]
	fn test_quest_matlab ( )
	{
		let mut input : ArrayList<Match<Cartesian3D>, 10> = ArrayList::new();
		input.push_back(Match{input: Cartesian3D{x: 1.0, y: 0.0, z: 0.3},
						output: Cartesian3D{x: 0.1, y: 1.0, z: 1.0}, weight: 1.0}).expect("HUH");
		input.push_back(Match{input: Cartesian3D{x: 1.0, y: 0.4, z: 0.0},
						output: Cartesian3D{x: 0.5, y: 1.0, z: 1.0}, weight: 1.0}).expect("HUH");
		input.push_back(Match{input: Cartesian3D{x: 1.0, y: 0.3, z: 0.0},
						output: Cartesian3D{x: 0.3, y: 1.0, z: 1.0}, weight: 1.0}).expect("HUH");
		input.push_back(Match{input: Cartesian3D{x: 1.0, y: 0.5, z: 0.0},
						output: Cartesian3D{x: 0.2, y: 1.0, z: 1.0}, weight: 1.0}).expect("HUH");

		for i in 0..input.size()
		{
			let mut m = input.get(i);
			m.input.normalize();
			m.output.normalize();
			input.set(i, m).expect("HUH");
		}

		let output_q  = Quest::estimate::<ConstQuest>(&input);
		let mut output_aa = output_q.to_angle_axis();

		output_aa.axis.normalize();

		let expected_q = Quaternion{
			w: 0.547549945381170,
			x: -0.461042818527695,
			y: -0.689219735118163,
			z: -0.112270804400634};

		println!("{:?} \t\t\t {:?}", output_q, expected_q);
		println!("{:?} \n{:?}", output_aa, expected_q.to_angle_axis());
		println!("{:?}", output_aa.axis.angle_distance(expected_q.to_angle_axis().axis).to_degrees());

		assert!(output_q.test_equal(&expected_q));
	}
*/

	#[test]
	fn test_quest_perfect_values ( )
	{
		let angle = Degrees(90.0).to_radians();
		let axis = Cartesian3D{x: 1.0, y: 0.0, z: 0.0};
		let mut angle_axis = AngleAxis{angle: angle, axis: axis};

		let angle_var = Degrees(0.0).to_radians();
		let axis_var = Cartesian3D{x: 0.0, y: 0.0, z: 0.0};
		let angle_axis_var = AngleAxis{angle: angle_var, axis: axis_var};

		let input = random_coordinates::<100>(angle_axis, angle_axis_var, 0.0);


		let rotation = Quest::estimate::<ConstQuest>(&input);
		angle_axis.angle = -angle_axis.angle;
		assert!(rotation.test_equal(&angle_axis.to_quaternion()));
	}


	#[test]
	fn test_quest_bad_weight ( )
	{
		let angle = Degrees(90.0).to_radians();
		let axis = Cartesian3D{x: 1.0, y: 0.0, z: 0.0};
		let angle_axis = AngleAxis{angle: angle, axis: axis};

		let angle_var = Degrees(0.0).to_radians();
		let axis_var = Cartesian3D{x: 0.0, y: 0.0, z: 0.0};
		let angle_axis_var = AngleAxis{angle: angle_var, axis: axis_var};

		let input = random_coordinates::<100>(angle_axis, angle_axis_var, 1000.0);


		let rotation = Quest::estimate::<ConstQuest>(&input);
		let ang_out = rotation.to_angle_axis();

		println!("{:?}", ang_out);
		assert!(rotation.dot(angle_axis.to_quaternion()).abs() < 0.01);
	}


	#[test]
	fn test_quest_bad_axis ( )
	{
		let angle = Degrees(90.0).to_radians();
		let axis = Cartesian3D{x: 1.0, y: 0.0, z: 0.0};
		let angle_axis = AngleAxis{angle: angle, axis: axis};

		let angle_var = Degrees(0.0).to_radians();
		let axis_var = Cartesian3D{x: 0.1, y: 0.1, z: 0.1};
		let angle_axis_var = AngleAxis{angle: angle_var, axis: axis_var};

		let input = random_coordinates::<100>(angle_axis, angle_axis_var, 0.0);


		let rotation = Quest::estimate::<ConstQuest>(&input);
		let ang_out = rotation.to_angle_axis();

		println!("{:?}", ang_out);
		// Rotation opposite provided angle axis.
		println!("{}", rotation.dot(angle_axis.to_quaternion()).abs());
		assert!(rotation.dot(angle_axis.to_quaternion()).abs() < 0.01);
	}



	#[test]
	fn test_quest_bad_angle ( )
	{
		let angle = Degrees(90.0).to_radians();
		let axis = Cartesian3D{x: 1.0, y: 0.0, z: 0.0};
		let angle_axis = AngleAxis{angle: angle, axis: axis};

		let angle_var = Degrees(10.0).to_radians();
		let axis_var = Cartesian3D{x: 0.0, y: 0.0, z: 0.0};
		let angle_axis_var = AngleAxis{angle: angle_var, axis: axis_var};

		let input = random_coordinates::<100>(angle_axis, angle_axis_var, 0.0);


		let rotation = Quest::estimate::<ConstQuest>(&input);
		let ang_out = rotation.to_angle_axis();

		println!("{:?}", ang_out);
		// Rotation opposite provided angle axis.
		println!("{}", rotation.dot(angle_axis.to_quaternion()).abs());
		assert!(rotation.dot(angle_axis.to_quaternion()).abs() < 0.01);
	}


	#[test]
	fn test_quest_bad_angle_weight_axis ( )
	{
		let angle = Degrees(90.0).to_radians();
		let axis = Cartesian3D{x: 1.0, y: 0.0, z: 0.0};
		let angle_axis = AngleAxis{angle: angle, axis: axis};

		let angle_var = Degrees(10.0).to_radians();
		let axis_var = Cartesian3D{x: 0.1, y: 0.1, z: 0.1};
		let angle_axis_var = AngleAxis{angle: angle_var, axis: axis_var};

		let input = random_coordinates::<100>(angle_axis, angle_axis_var, 1000.0);


		let rotation = Quest::estimate::<ConstQuest>(&input);
		let ang_out = rotation.to_angle_axis();

		println!("{:?}", ang_out);
		// Rotation opposite provided angle axis.
		println!("{}", rotation.dot(angle_axis.to_quaternion()).abs());
		assert!(rotation.dot(angle_axis.to_quaternion()).abs() < 0.01);
	}


}
