//! Implementation for matrix.
use crate::util::aliases::Decimal;
use crate::util::units::MatrixPosition;
use crate::util::err::{Error, Errors};
use super::Matrix;

use std::fmt;
use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Div;

impl <const W : usize, const H : usize> Matrix <W, H>
{
	/// Default Constructor.
	/// Initializes matrix to 0.
	/// 
	/// # Example
	/// ```
	/// use star_tracker::util::units::Matrix;
	/// let mat3x4 : Matrix<3, 4> = Matrix::new();
	/// ```
	pub fn new ( ) -> Matrix<W, H>
	{
 		return Matrix { matrix: [[0.0; W]; H] };
	}
	
	/// Returns the width of the matrix.
	/// # Example
	/// ```
	/// use star_tracker::util::units::Matrix;
	/// let mat3x4 : Matrix<3, 4> = Matrix::new();
	/// assert_eq!(mat3x4.width(), 3);
	/// ```
	pub fn width  ( &self ) -> usize	{	return W;	}
	
	/// Returns the width of the matrix.
	/// # Example
	/// ```
	/// use star_tracker::util::units::Matrix;
	/// let mat3x4 : Matrix<3, 4> = Matrix::new();
	/// assert_eq!(mat3x4.height(), 4);
	/// ```
	pub fn height ( &self ) -> usize	{	return H;	}
	
	/// Returns the value at the given index.
	/// # Example
	/// ```
	/// use star_tracker::util::err::Errors;
	/// use star_tracker::util::units::Matrix;
	/// use star_tracker::util::units::MatrixPosition;
	/// let mut mat3x4 : Matrix<3, 4> = Matrix::new();
	/// assert!(mat3x4.set(MatrixPosition{x: 1, y: 2}, 3.12).is_ok());
	/// assert!(mat3x4.set(MatrixPosition{x: 2, y: 3}, 5.23).is_ok()); 
	/// assert_eq!(mat3x4.set(MatrixPosition{x: 4, y: 0}, 3.12), Err(Errors::OutOfBoundsX));
	/// assert_eq!(mat3x4.set(MatrixPosition{x: 0, y: 5}, 5.23), Err(Errors::OutOfBoundsY));
	/// assert_eq!(mat3x4.get(MatrixPosition{x: 1, y: 2}).expect(""), 3.12);
	/// assert_eq!(mat3x4.get(MatrixPosition{x: 2, y: 3}).expect(""), 5.23);
	/// assert_eq!(mat3x4.get(MatrixPosition{x: 0, y: 0}).expect(""), 0.0);
	/// assert_eq!(mat3x4.get(MatrixPosition{x: 4, y: 0}), Err(Errors::OutOfBoundsX));
	/// assert_eq!(mat3x4.get(MatrixPosition{x: 0, y: 5}), Err(Errors::OutOfBoundsY));
	/// ```
	pub fn get ( &self, pos: MatrixPosition ) -> Error<Decimal>
	{ 
		if pos.x >= self.width()
		{
			return Err(Errors::OutOfBoundsX);
		}
		if pos.y >= self.height()
		{
			return Err(Errors::OutOfBoundsY);
		}
		return Ok(self.get_fast(pos));
	}
	
	/// Returns the value at the given index.
	/// # Example
	/// ```
	/// use star_tracker::util::err::Errors;
	/// use star_tracker::util::units::Matrix;
	/// use star_tracker::util::units::MatrixPosition;
	/// let mut mat3x4 : Matrix<3, 4> = Matrix::new();
	/// assert!(mat3x4.set(MatrixPosition{x: 1, y: 2}, 3.12).is_ok());
	/// assert!(mat3x4.set(MatrixPosition{x: 2, y: 3}, 5.23).is_ok()); 
	/// assert_eq!(mat3x4.set(MatrixPosition{x: 4, y: 0}, 3.12), Err(Errors::OutOfBoundsX));
	/// assert_eq!(mat3x4.set(MatrixPosition{x: 0, y: 5}, 5.23), Err(Errors::OutOfBoundsY));
	/// assert_eq!(mat3x4.get(MatrixPosition{x: 1, y: 2}).expect(""), 3.12);
	/// assert_eq!(mat3x4.get(MatrixPosition{x: 2, y: 3}).expect(""), 5.23);
	/// assert_eq!(mat3x4.get(MatrixPosition{x: 0, y: 0}).expect(""), 0.0);
	/// ```
	pub fn set ( &mut self, pos: MatrixPosition, value: Decimal ) -> Error<()> 
	{ 
		if pos.x >= self.width()
		{
			return Err(Errors::OutOfBoundsX);
		}
		if pos.y >= self.height()
		{
			return Err(Errors::OutOfBoundsY);
		}
		self.set_fast(pos, value);
		return Ok(());
	}
	
	
	
	
	
	
	
	
	/// Unchecked accessor of matrix, does not check for bounds.
	fn get_fast ( &self, pos: MatrixPosition ) -> Decimal
	{
		return self.matrix[pos.y][pos.x];
	}

	/// Unchecked accessor of matrix, does not check for bounds.
	fn set_fast ( &mut self, pos: MatrixPosition, value: Decimal )
	{
		return self.matrix[pos.y][pos.x] = value;
	}
}













impl <const W: usize, const H: usize> fmt::Display for Matrix<W, H> {	
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
	{ 
		for yy in 0..self.height()
		{
			write!(f, "|")?;
			for xx in 0..self.width()
			{
				let val = self.get_fast(MatrixPosition{x: xx, y: yy});
				write!(f, "{}", val)?;
				if xx < self.width() - 1
				{
					write!(f, ", ")?;
				}
				
			}
			write!(f, "|")?;
			write!(f, "\n")?;
		}
		return Ok(());
	}
}

//###############################################################################################//
//							---	Multiply/Divide Scalar ---
//###############################################################################################//	
// Multiply Matrix by Scalar
impl <const W: usize, const H: usize> Mul<Decimal> for Matrix<W, H> { type Output = Self;
    fn mul(self, rhs: Decimal) -> Self 
	{ 
		let mut mat : Matrix<W, H> = Matrix::new();
		for xx in 0..W
		{
			for yy in 0..H
			{
				let pos = MatrixPosition{x: xx, y: yy};
				mat.set_fast(pos, self.get_fast(pos) * rhs);
			}
		}
		return mat;
	} 
}

// Multiply Scalar by Matrix
impl <const W: usize, const H: usize> Mul<Matrix<W, H>> for Decimal {
    type Output = Matrix<W, H>;
    fn mul(self, rhs: Matrix<W, H>) -> Matrix<W, H> {	return rhs * self;	}
}


// Divide Matrix by Scalar
impl <const W: usize, const H: usize> Div <Decimal> for Matrix<W, H> { type Output = Self;
    fn div(self, rhs: Decimal) -> Self 
	{ 
		let mut mat : Matrix<W, H> = Matrix::new();
		for xx in 0..W
		{
			for yy in 0..H
			{
				let pos = MatrixPosition{x: xx, y: yy};
				mat.set_fast(pos, self.get_fast(pos) / rhs);
			}
		}
		return mat;
	} 
}





impl<const W: usize, const H: usize> Add for Matrix<W, H> { type Output = Self;
    fn add ( self, rhs: Matrix<W, H> ) -> Self 
	{ 
		let mut mat : Matrix<W, H> = Matrix::new();
		for xx in 0..W
		{
			for yy in 0..H
			{
				let pos = MatrixPosition{x: xx, y: yy};
				mat.set_fast(pos, self.get_fast(pos) + rhs.get_fast(pos));
			}
		}
		return mat;
	}
}


impl<const W: usize, const H: usize> Sub for Matrix<W, H> { type Output = Self;
    fn sub ( self, rhs: Matrix<W, H> ) -> Self 
	{ 
		let mut mat : Matrix<W, H> = Matrix::new();
		for xx in 0..W
		{
			for yy in 0..H
			{
				let pos = MatrixPosition{x: xx, y: yy};
				mat.set_fast(pos, self.get_fast(pos) - rhs.get_fast(pos));
			}
		}
		return mat;
	}
}




impl<const W: usize, const H: usize> PartialEq for Matrix<W, H> {
	fn eq ( &self, other: &Self ) -> bool 
	{ 
		let mut eq = true;
		for xx in 0..W
		{
			for yy in 0..H
			{
				let pos = MatrixPosition{x: xx, y: yy};
				eq &= (self.get_fast(pos) - other.get_fast(pos)).abs() < 0.00001;
			}
		}
		
		return eq;
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
	use util::units::Matrix;
	use util::units::MatrixPosition;
	use util::err::Errors;
	
	#[test]
	fn test_new ( )
	{
		let mat3x4 : Matrix<3, 4> = Matrix::new();
		assert_eq!( mat3x4.matrix.len(), 4);
		assert_eq!( mat3x4.matrix[0].len(), 3);
	}

	#[test]
	fn test_width ( )
	{
		let mat3x4 : Matrix<3, 4> = Matrix::new();
		assert_eq!( mat3x4.width(), 3);
	}

	#[test]
	fn test_height ( )
	{
		let mat3x4 : Matrix<3, 4> = Matrix::new();
		assert_eq!( mat3x4.height(), 4);
	}
	
	
	#[test]
	fn test_get ( )
	{
		let mut mat3x4 : Matrix<3, 4> = Matrix::new();
		mat3x4.matrix[2][1] = 3.12;
		mat3x4.matrix[3][2] = 5.23;
		assert_eq!(mat3x4.get(MatrixPosition{x: 1, y: 2}).expect(""), 3.12);
		assert_eq!(mat3x4.get(MatrixPosition{x: 2, y: 3}).expect(""), 5.23);
		assert_eq!(mat3x4.get(MatrixPosition{x: 0, y: 0}).expect(""), 0.0);
	}


	#[test]
	fn test_set ( )
	{
		let mut mat3x4 : Matrix<3, 4> = Matrix::new();
		assert!(mat3x4.set(MatrixPosition{x: 1, y: 2}, 3.12).is_ok());
		assert!(mat3x4.set(MatrixPosition{x: 2, y: 3}, 5.23).is_ok()); 
		assert_eq!(mat3x4.set(MatrixPosition{x: 4, y: 0}, 3.12), Err(Errors::OutOfBoundsX));
		assert_eq!(mat3x4.set(MatrixPosition{x: 0, y: 5}, 5.23), Err(Errors::OutOfBoundsY));
		assert_eq!(mat3x4.matrix[2][1], 3.12);
		assert_eq!(mat3x4.matrix[3][2], 5.23);
	}
	
	#[test]
	fn test_get_err ( )
	{
		let mat3x4 : Matrix<3, 4> = Matrix::new();
		assert_eq!(mat3x4.get(MatrixPosition{x: 4, y: 0}), Err(Errors::OutOfBoundsX));
		assert_eq!(mat3x4.get(MatrixPosition{x: 0, y: 5}), Err(Errors::OutOfBoundsY))
	}

	#[test]
	fn test_set_err ( )
	{
		let mut mat3x4 : Matrix<3, 4> = Matrix::new();
		assert_eq!(mat3x4.set(MatrixPosition{x: 4, y: 0}, 3.12), Err(Errors::OutOfBoundsX));
		assert_eq!(mat3x4.set(MatrixPosition{x: 0, y: 5}, 5.23), Err(Errors::OutOfBoundsY));
	}
	
	
	
	
	
	
	
	
	#[test]
	fn test_mul_scalar ( )
	{
		let mut mat3x4 : Matrix<3,4> = Matrix::new();
		mat3x4.matrix[1][2] = 12.0;
		mat3x4.matrix[3][2] = 23.0;
		mat3x4 = mat3x4 * 2.0;
		assert_eq!(mat3x4.matrix[1][2], 24.0);
		assert_eq!(mat3x4.matrix[3][2], 46.0);
		mat3x4 = 0.5 * mat3x4;
		assert_eq!(mat3x4.matrix[1][2], 12.0);
		assert_eq!(mat3x4.matrix[3][2], 23.0);
	}

	#[test]
	fn test_div_scalar ( )
	{
		let mut mat3x4 : Matrix<3,4> = Matrix::new();
		mat3x4.matrix[1][2] = 24.0;
		mat3x4.matrix[3][2] = 46.0;
		mat3x4 = mat3x4 / 2.0;
		assert_eq!(mat3x4.matrix[1][2], 12.0);
		assert_eq!(mat3x4.matrix[3][2], 23.0);
	}







	#[test]
	fn test_add_matrix ( )
	{
		let mut mat3x4    : Matrix<3,4> = Matrix::new();
		let mut mat_other : Matrix<3,4> = Matrix::new();
		mat3x4.matrix[1][2] = 12.0;
		mat3x4.matrix[3][2] = 32.0;
		mat_other.matrix[1][1] = 11.0;
		mat_other.matrix[0][1] = 01.0;
		mat3x4 = mat3x4 + mat_other;
		assert_eq!(mat3x4.matrix[1][2], 12.0);
		assert_eq!(mat3x4.matrix[3][2], 32.0);
		assert_eq!(mat3x4.matrix[1][1], 11.0);
		assert_eq!(mat3x4.matrix[0][1], 01.0);
		assert_eq!(mat3x4.matrix[0][0], 0.0);
	}


	#[test]
	fn test_sub_matrix ( )
	{
		let mut mat3x4    : Matrix<3,4> = Matrix::new();
		let mut mat_other : Matrix<3,4> = Matrix::new();
		mat3x4.matrix[1][2] = 12.0;
		mat3x4.matrix[3][2] = 32.0;
		mat_other.matrix[1][1] = 11.0;
		mat_other.matrix[0][1] = 01.0;
		mat3x4 = mat3x4 - mat_other;
		assert_eq!(mat3x4.matrix[1][2], 12.0);
		assert_eq!(mat3x4.matrix[3][2], 32.0);
		assert_eq!(mat3x4.matrix[1][1], -11.0);
		assert_eq!(mat3x4.matrix[0][1], -01.0);
		assert_eq!(mat3x4.matrix[0][0], 0.0);
	}



	#[test]
	fn test_eq_matrix ( )
	{
		let mut mat3x4    : Matrix<3,4> = Matrix::new();
		let mut mat_other : Matrix<3,4> = Matrix::new();
		assert!(mat3x4 == mat_other);
		mat3x4.matrix[1][2] = 12.0;
		mat3x4.matrix[3][2] = 32.0;
		mat_other.matrix[1][1] = 11.0;
		mat_other.matrix[0][1] = 01.0;
		assert!(mat3x4 != mat_other);
		mat3x4.matrix[1][1] = 11.0;
		mat3x4.matrix[0][1] = 01.0;
		mat_other.matrix[1][2] = 12.0;
		mat_other.matrix[3][2] = 32.0;
		assert!(mat3x4 == mat_other);
	}
}
