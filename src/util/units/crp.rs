use crate::util::aliases::Decimal;
use super::CRP;
use super::Quaternion;
use super::Matrix;
use super::MatPos;

impl CRP
{
	pub fn new ( mat : &Matrix<3,1> ) -> CRP
	{
		return CRP{
			x: mat.get(MatPos{row: 0, col: 0}),
			y: mat.get(MatPos{row: 1, col: 0}),
			z: mat.get(MatPos{row: 2, col: 0}),
		};
	}
	
	
	/// Converts the CRP to a quaternion.
	///
	pub fn to_quaternion ( &self, gamma : Decimal ) -> Quaternion
	{
		let mag = ((self.x*self.x) + (self.y*self.y) + (self.z * self.z)).sqrt();
		
		let val : Decimal = 1.0 / (gamma*gamma + mag*mag).sqrt();
		let q = Quaternion{w: val * gamma, x: val * self.x, y: val * self.y, z: val * self.z};
		return q;
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
	use util::units::CRP;
	use util::units::Quaternion;
	use super::Matrix;
	use super::MatPos;
	use util::test::TestEqual;
	// Test values generated through matlab.
	
	#[test]
	fn test_new ( )
	{
		let mut mat3x1 : Matrix<3,1> = Matrix::new();
		mat3x1.set(MatPos{row: 0, col: 0}, 1.0);
		mat3x1.set(MatPos{row: 1, col: 0}, 2.0);
		mat3x1.set(MatPos{row: 2, col: 0}, 3.0);
		let a : CRP = CRP::new(&mat3x1);
		assert_eq!(a.x, 1.0);
		assert_eq!(a.y, 2.0);
		assert_eq!(a.z, 3.0);
	}
	
	#[test]
	fn test_to_quaternion ( )
	{
		let a : CRP = CRP{x: -0.0000947935, y: 0.000189587, z: -0.0000947935};
		let b : CRP = CRP{x: 0.0000931523, y: 0.000000111, z: -0.000085496};
		
		let mut compare = Quaternion{w: 0.999999973, x: -0.0000947935, y: 0.000189587, z: -0.0000947935};
		assert!(a.to_quaternion(1.0).test_close(&compare, 0.000001));
		
		compare = Quaternion{w: 0.999999992, x: 0.0000931523, y: 0.000000111, z: -0.000085496};
		assert!(b.to_quaternion(1.0).test_close(&compare, 0.000001));
	}

}