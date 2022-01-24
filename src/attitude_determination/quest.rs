//! The following equations are essential to the implementation of any algorithms using Whaba's method.

use super::Quest;
use super::Wahba;
use super::AttitudeDetermination;

use tracking_mode::Match;

// use util::aliases::Decimal;

use util::list::List;

use util::units::CRP;
use util::units::Matrix;
// use util::units::MatPos;
use util::units::Cartesian3D;
use util::units::Quaternion;

use config::AttitudeDeterminationConsts;


impl AttitudeDetermination for Quest
{
	fn estimate <T: 'static> ( positions: &dyn List<Match<Cartesian3D>> ) -> Quaternion
		where T: AttitudeDeterminationConsts
	{
		// Create K matrix (Davenport).
		let b = Wahba::find_b(positions).transpose();
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
		let b = sigma * sigma + (z.transpose() * z).to_decimal();
		let c = s.determinate() + (z.transpose() * s * z).to_decimal(); 
		let d = (z.transpose() * s * s * z).to_decimal();
		
		// Use the neuton raphson method to solve for lambda.
		// This would be:
		// ``` lambda_n = det[lambda_n-1 * I_4x4 - K] / d/dx(det[lambda_n-1 * I_4x4 - K]) ```
		// Luckly there is a simpler solution with the polynomial.
		while T::LAMBDA_PRECISION <= (lambda - last_lambda).abs()
		{
			last_lambda = lambda;
			
			let f = lambda.powf(4.0) - (a + b)*lambda.powf(2.0) - c * lambda + (a*b + c*sigma - d);
			let f_prime = 4.0*lambda.powf(3.0) - 2.0 * (a + b) * lambda - c;
			lambda = lambda - f/f_prime;
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

