//! This contains the `estimate` method, use this to obtain the attitude of the camera.

use crate::attitude_determination::Quest;
use crate::attitude_determination::Wahba;
use crate::attitude_determination::AttitudeDetermination;


use crate::util::list::List;

use crate::util::units::CRP;
use crate::util::units::Match;
use crate::util::units::Matrix;
use crate::util::units::Vector3;
use crate::util::units::Quaternion;
use crate::util::aliases::Decimal;

use crate::attitude_determination::LAMBDA_PRECISION;

use crate::util::Maths;


impl AttitudeDetermination for Quest
{
	/// Call this to retrieve the attitude of the camera.  
	/// Finds the most likely pointing direction from the given (observed, reference) positions.
	/// # Arguments
	/// * `positions` - The (input: observed, output: reference, weighting: __).
	/// 	The weighting is just a ratio, it does not matter the size, just how it relates to other weightings.
	/// * `lambda_precision` - 
	/// 	For quest algorithm, to find the correct attitude, the neuton raphson method is used.
	/// 	This method will loop and slowly decrease the gap between the current and previous prediction.
	/// 	Achieving perfect precision comparing the 2 values will take up computation power.
	/// 	By specifying a precision, the computational requirements are lowered.
	/// 	To use a default estimate value, you can provide None and it will use LAMBDA_PRECISION.
	/// # Returns
	/// A quaternion which rotates output to input.
	/// Use Quaternion.conjugate() to get a rotation from input to output.
	fn estimate ( positions: &dyn List<Match<Vector3>>, mut lambda_precision: Option<Decimal> ) 
																					-> Quaternion
	{
		// Create K matrix (Davenport).
		let b = Wahba::find_b(positions).transposed();
		let z = Wahba::find_z(&b);
		let s = Wahba::find_s(&b);
		let sigma = Wahba::find_sigma(&b);

		// Initial Guess of lambda is the sum of the weights.
		let mut lambda = 0.0;
		let mut last_lambda = 0.0;
		for i in 0..positions.size()
		{
			lambda += positions.get(i).weight;
		}

		// Finding lambda uses a quatric polynomial.
		let a = sigma * sigma - s.adjoint().trace();
		let b = sigma * sigma + (z.transposed() * z).to_decimal();
		let c = s.determinate() + (z.transposed() * s * z).to_decimal();
		let d = (z.transposed() * s * s * z).to_decimal();

		let lambda_precision = *lambda_precision.get_or_insert(LAMBDA_PRECISION); 

		let mut i = 0; // An iteration count stops an infinite loop.
		while lambda_precision <= (lambda - last_lambda).abs() && i < 10
		{
			last_lambda = lambda;

			let f = lambda.powf(4.0) - (a + b)*lambda.powf(2.0) - c * lambda + (a*b + c*sigma - d);
			let f_prime = 4.0*lambda.powf(3.0) - 2.0 * (a + b) * lambda - c;
			lambda = lambda - f/f_prime;

			i += 1;
		}


		let alpha = lambda.powf(2.0) - sigma.powf(2.0) + s.adjoint().trace();
		let beta  = lambda - sigma;
		let gamma = (lambda + sigma) * alpha - s.determinate();


		let identity : Matrix<3,3> = Matrix::identity();
		let x = (alpha * identity + beta * s + s*s) * z;
		let crp = CRP::new(&x);
		let q_opt = crp.to_quaternion(gamma);
		return q_opt;
	}
}
