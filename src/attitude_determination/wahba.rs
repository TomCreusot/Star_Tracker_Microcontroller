//! The following equations are essential to the implementation of any algorithms using Wahba's method.

use super::Wahba;

use tracking_mode::Match;

use util::aliases::Decimal;

use util::list::List;

use util::units::Matrix;
use util::units::MatPos;
use util::units::Cartesian3D;



impl Wahba
{
/// Attitude Profile Matrix 
/// B = \sum^n_{i=1} precision_{i} * observed^{T} * reference
pub fn find_b ( positions: &dyn List<Match<Cartesian3D>> ) -> Matrix<3, 3>
{
	let mut b : Matrix<3, 3> = Matrix::new();

	for out_row in 0..3 // The out row is the 0: x, 1: y, 2: z of observed.
	{
		for out_col in 0..3 // The out column is the 0: x, 1: y, 2: z of the reference.
		{
			let mut output : Decimal = 0.0;
			for pos in 0..positions.size()	
			{
				let obs_v =
					positions.get(pos).output.to_matrix_row().get(MatPos{row:0, col: out_row});
				let ref_v =
					positions.get(pos).input.to_matrix_row().get(MatPos{row: 0, col: out_col});
				output += obs_v * ref_v * positions.get(pos).weight;
			}
			b.set(MatPos{row: out_row, col: out_col}, output);
		}
	}
	return b;
}

/// Z is the top of B subtract the bottom of B  flipped over the trace of the matrix.
pub fn find_z ( b: &Matrix<3,3> ) -> Matrix<3,1>
{
	let mut z : Matrix<3, 1> = Matrix::new();
	z.set(MatPos{row: 0, col: 0}, b.get(MatPos{row: 1, col: 2})- b.get(MatPos{row: 2, col: 1}));
	z.set(MatPos{row: 1, col: 0}, b.get(MatPos{row: 2, col: 0}) - b.get(MatPos{row: 0, col: 2}));
	z.set(MatPos{row: 2, col: 0}, b.get(MatPos{row: 0, col: 1}) - b.get(MatPos{row: 1, col: 0}));
	return z;
}

/// S is B multiplied by its transpose.
pub fn find_s ( b: &Matrix<3, 3> ) -> Matrix<3,3>
{
	return *b + b.transpose();
}


/// sigma is the trace of B.
pub fn find_sigma ( b: &Matrix<3,3> ) -> Decimal
{
	return b.trace();
}

/// An Eigenvector of the K matrix can be used to find the rotation matrix.
/// The eigenvector must be of the highest eigenvalue.
/// NOTE: SOMETIMES THE MATRIX IS FLIPPED SO SIGNMA IS AT THE BOTTOM RIGHT.
/// K = 
/// [
///  \sigma			Z(1)				Z(2)				Z(3)
///  Z(1)			S(0,0) - \sigma		S(0,1)				S(0,2)
///  Z(2)			S(1,0)				S(1,1) - \sigma		S(1,2)
///  Z(3)			S(2,0)				S(2,1)				S(2,2) - \sigma
/// ] 
pub fn find_k ( z: &Matrix<3, 1>, s: &Matrix<3, 3>, sigma: Decimal ) -> Matrix<4,4>
{
	let mut k : Matrix<4,4> = Matrix::new();
	k.insert(MatPos{row: 1, col: 0}, &z);
	k.insert(MatPos{row: 0, col: 1}, &z.transpose());
	k.insert(MatPos{row: 1, col: 1}, &s);
	
	k.set(MatPos{row: 0, col: 0}, k.get(MatPos{row: 0, col: 0}) + sigma);
	k.set(MatPos{row: 1, col: 1}, k.get(MatPos{row: 1, col: 1}) - sigma);
	k.set(MatPos{row: 2, col: 2}, k.get(MatPos{row: 2, col: 2}) - sigma);
	k.set(MatPos{row: 3, col: 3}, k.get(MatPos{row: 3, col: 3}) - sigma);
	
	return k;
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
#[allow(unused_must_use)]
mod test
{
	use attitude_determination::Wahba;
	use tracking_mode::Match;
	
	use util::aliases::Decimal;
	use util::units::MatPos;
	use util::units::Cartesian3D;
	use util::units::Matrix;
	
	use util::list::List;
	use util::test::TestEqual;
	use util::list::ArrayList;
	
	
	fn assert_close ( a : Decimal, b : Decimal )
	{
		if a.test_equal(&b)
		{
			assert_eq!(a, b);
		}
	}

	// The following tests were performed with a set of simulated results from matlab.
	
	fn gen_b ( ) -> Matrix<3,3>
	{
		let mut b : Matrix<3,3> = Matrix::new();
		b.set(MatPos{row: 0, col: 0}, 264.0);
		b.set(MatPos{row: 1, col: 0}, 291.0);
		b.set(MatPos{row: 2, col: 0}, 318.0);

		b.set(MatPos{row: 0, col: 1}, 277.5);
		b.set(MatPos{row: 1, col: 1}, 306.0);
		b.set(MatPos{row: 2, col: 1}, 334.5);
		
		b.set(MatPos{row: 0, col: 2}, 291.0);
		b.set(MatPos{row: 1, col: 2}, 321.0);
		b.set(MatPos{row: 2, col: 2}, 351.0);	
		return b;
	}
	
	
	//
	// fn find_b ( &List<Match<Cartesian3D>> ) -> Matrix<3,3>
	//
	
	
	#[test]
	fn test_find_b ( )
	{
		let mut pos : ArrayList<Match<Cartesian3D>, 5> = ArrayList::new();
		pos.push_back(Match{input: Cartesian3D{x: 10.0, y: 11.0, z: 12.0}, 
							output: Cartesian3D{x: 1.0, y: 2.0, z: 3.0}, weight: 0.1});
		pos.push_back(Match{input: Cartesian3D{x: 13.0, y: 14.0, z: 15.0}, 
							output: Cartesian3D{x: 4.0, y: 5.0, z: 6.0}, weight: 0.2});
		pos.push_back(Match{input: Cartesian3D{x: 16.0, y: 17.0, z: 18.0}, 
							output: Cartesian3D{x: 7.0, y: 8.0, z: 9.0}, weight: 0.3});
		pos.push_back(Match{input: Cartesian3D{x: 19.0, y: 20.0, z: 21.0}, 
							output: Cartesian3D{x: 10.0, y: 11.0, z: 12.0}, weight: 0.4});
		pos.push_back(Match{input: Cartesian3D{x: 22.0, y: 23.0, z: 24.0}, 
							output: Cartesian3D{x: 13.0, y: 14.0, z: 15.0}, weight: 0.5});
		
		let b : Matrix<3,3> = Wahba::find_b(&pos);
		assert_close(b.get(MatPos{row: 0, col: 0}), 264.0);
		assert_close(b.get(MatPos{row: 1, col: 0}), 291.0);
		assert_close(b.get(MatPos{row: 2, col: 0}), 318.0);
		
		assert_close(b.get(MatPos{row: 0, col: 1}), 277.5);
		assert_close(b.get(MatPos{row: 1, col: 1}), 306.0);
		assert_close(b.get(MatPos{row: 2, col: 1}), 334.5);
		
		assert_close(b.get(MatPos{row: 0, col: 2}), 291.0);
		assert_close(b.get(MatPos{row: 1, col: 2}), 321.0);
		assert_close(b.get(MatPos{row: 2, col: 2}), 351.0);
	}
	
	
	//
	// fn find_z ( &Matrix<3,3> ) -> Matrix<3,1>
	//
	
	#[test]
	fn test_find_z ( )
	{
		let z = Wahba::find_z(&gen_b());
		assert_close(z.get(MatPos{row: 0, col: 0}), -13.5);
		assert_close(z.get(MatPos{row: 1, col: 0}), 27.0);
		assert_close(z.get(MatPos{row: 2, col: 0}), -13.5);
	}
	
	//
	// fn find_s ( &Matrix<3,3> ) -> Matrix<3,3>
	//
	
	#[test]
	fn test_find_s ( )
	{
		let s = Wahba::find_s(&gen_b());
		assert_close(s.get(MatPos{row: 0, col: 0}), 528.0);
		assert_close(s.get(MatPos{row: 1, col: 0}), 568.5);
		assert_close(s.get(MatPos{row: 2, col: 0}), 609.0);
		
		assert_close(s.get(MatPos{row: 0, col: 1}), 568.5);
		assert_close(s.get(MatPos{row: 1, col: 1}), 612.0);
		assert_close(s.get(MatPos{row: 2, col: 1}), 655.5);
		
		assert_close(s.get(MatPos{row: 0, col: 2}), 609.0);
		assert_close(s.get(MatPos{row: 1, col: 2}), 655.5);
		assert_close(s.get(MatPos{row: 2, col: 2}), 702.0);
	}
	
	
	
	//
	// fn find_sigma ( &Matrix<3,3> ) -> Decimal
	//
	
	#[test]
	fn test_find_sigma ( )
	{
		assert_eq!(Wahba::find_sigma(&gen_b()), 921.0);
	}


	//
	// fn find_k ( z: &Matrix<3, 1>, s: &Matrix<3, 3>, sigma: Decimal ) -> Matrix<4,4>
	//
	
	#[test]
	fn test_find_k ( )
	{
		let b = gen_b();
		let z = Wahba::find_z(&b);
		let s = Wahba::find_s(&b);
		let sigma = Wahba::find_sigma(&b);
		let k = Wahba::find_k(&z, &s, sigma);
		assert_close(k.get(MatPos{row: 0, col: 0}), 921.0);
		assert_close(k.get(MatPos{row: 1, col: 0}), -13.5);
		assert_close(k.get(MatPos{row: 2, col: 0}), 27.0);
		assert_close(k.get(MatPos{row: 3, col: 0}), -13.5);
		
		assert_close(k.get(MatPos{row: 0, col: 1}), -13.5);
		assert_close(k.get(MatPos{row: 1, col: 1}), -393.0);
		assert_close(k.get(MatPos{row: 2, col: 1}), 568.5);
		assert_close(k.get(MatPos{row: 3, col: 1}), 609.0);
		
		assert_close(k.get(MatPos{row: 0, col: 2}), 27.0);
		assert_close(k.get(MatPos{row: 1, col: 2}), 568.5);
		assert_close(k.get(MatPos{row: 2, col: 2}), -309.0);
		assert_close(k.get(MatPos{row: 3, col: 2}), 655.5);
		
		assert_close(k.get(MatPos{row: 0, col: 3}), -13.5);
		assert_close(k.get(MatPos{row: 1, col: 3}), 609.0);
		assert_close(k.get(MatPos{row: 2, col: 3}), 655.5);
		assert_close(k.get(MatPos{row: 3, col: 3}), -219.0);
	}
}