//! Implementation for matrix.
use crate::util::aliases::Decimal;
use crate::util::units::MatPos;
use crate::util::err::{Error, Errors};
use super::Matrix;

use std::fmt;
use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Div;

impl <const ROW : usize, const COLUMN : usize> Matrix <ROW, COLUMN>
{
	/// Default Constructor.
	/// Initializes matrix to 0.
	///
	/// # Example
	/// ```
	/// use star_tracker::util::units::Matrix;
	/// let mat3x4 : Matrix<3, 4> = Matrix::new(); // Matrix with 3 rows and 4 columns
	/// ```
	pub fn new ( ) -> Matrix<ROW, COLUMN>
	{
 		return Matrix { matrix: [[0.0; COLUMN]; ROW] };
	}

	/// Returns the width of the matrix.
	/// # Example
	/// ```
	/// use star_tracker::util::units::Matrix;
	/// let mat3x4 : Matrix<3, 4> = Matrix::new();
	/// assert_eq!(mat3x4.width(), 4);
	/// ```
	pub fn width  ( &self ) -> usize	{	return COLUMN;	}

	/// Returns the width of the matrix.
	/// # Example
	/// ```
	/// use star_tracker::util::units::Matrix;
	/// let mat3x4 : Matrix<3, 4> = Matrix::new();
	/// assert_eq!(mat3x4.height(), 3);
	/// ```
	pub fn height ( &self ) -> usize	{	return ROW;	}

	/// Returns the value at the given index.
	/// # Example
	/// ```
	/// use star_tracker::util::err::Errors;
	/// use star_tracker::util::units::Matrix;
	/// use star_tracker::util::units::MatPos;
	/// let mut mat3x4 : Matrix<3, 4> = Matrix::new();
	/// assert!(mat3x4.set(MatPos{row: 1, col: 2}, 3.12).is_ok());
	/// assert!(mat3x4.set(MatPos{row: 2, col: 3}, 5.23).is_ok());
	/// assert_eq!(mat3x4.set(MatPos{row: 4, col: 0}, 3.12), Err(Errors::OutOfBoundsY));
	/// assert_eq!(mat3x4.set(MatPos{row: 0, col: 5}, 5.23), Err(Errors::OutOfBoundsX));
	/// assert_eq!(mat3x4.get(MatPos{row: 1, col: 2}).expect(""), 3.12);
	/// assert_eq!(mat3x4.get(MatPos{row: 2, col: 3}).expect(""), 5.23);
	/// assert_eq!(mat3x4.get(MatPos{row: 0, col: 0}).expect(""), 0.0);
	/// assert_eq!(mat3x4.get(MatPos{row: 4, col: 0}), Err(Errors::OutOfBoundsY));
	/// assert_eq!(mat3x4.get(MatPos{row: 0, col: 5}), Err(Errors::OutOfBoundsX));
	/// ```
	pub fn get ( &self, pos: MatPos ) -> Error<Decimal>
	{
		if pos.col >= self.width()
		{
			return Err(Errors::OutOfBoundsX);
		}
		if pos.row >= self.height()
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
	/// use star_tracker::util::units::MatPos;
	/// let mut mat3x4 : Matrix<3, 4> = Matrix::new();
	/// assert!(mat3x4.set(MatPos{row: 1, col: 2}, 3.12).is_ok());
	/// assert!(mat3x4.set(MatPos{row: 2, col: 3}, 5.23).is_ok());
	/// assert_eq!(mat3x4.set(MatPos{row: 4, col: 0}, 3.12), Err(Errors::OutOfBoundsY));
	/// assert_eq!(mat3x4.set(MatPos{row: 0, col: 5}, 5.23), Err(Errors::OutOfBoundsX));
	/// assert_eq!(mat3x4.get(MatPos{row: 1, col: 2}).expect(""), 3.12);
	/// assert_eq!(mat3x4.get(MatPos{row: 2, col: 3}).expect(""), 5.23);
	/// assert_eq!(mat3x4.get(MatPos{row: 0, col: 0}).expect(""), 0.0);
	/// ```
	pub fn set ( &mut self, pos: MatPos, value: Decimal ) -> Error<()>
	{
		if pos.col >= self.width()
		{
			return Err(Errors::OutOfBoundsX);
		}
		if pos.row >= self.height()
		{
			return Err(Errors::OutOfBoundsY);
		}
		self.set_fast(pos, value);
		return Ok(());
	}



	/// Flips the rows and columns.
	///
	/// # Example
	/// ```
	/// use star_tracker::util::units::Matrix;
	/// use star_tracker::util::units::MatPos;
	/// let mut mat3x4 : Matrix<3,4> = Matrix::new();
	/// mat3x4.set(MatPos{row: 0, col: 0}, 0.0);
	/// mat3x4.set(MatPos{row: 0, col: 1}, 0.1);
	/// mat3x4.set(MatPos{row: 1, col: 0}, 1.0);
	/// mat3x4.set(MatPos{row: 1, col: 1}, 1.1);
	/// mat3x4.set(MatPos{row: 2, col: 3}, 2.3);
	/// let mat4x3 = mat3x4.transpose();
	/// assert_eq!(mat4x3.width(), 3);
	/// assert_eq!(mat4x3.height(), 4);
	/// assert_eq!(mat3x4.get(MatPos{row:0, col:0}), mat4x3.get(MatPos{row:0, col:0}));
	/// assert_eq!(mat3x4.get(MatPos{row:0, col:1}), mat4x3.get(MatPos{row:1, col:0}));
	/// assert_eq!(mat3x4.get(MatPos{row:1, col:0}), mat4x3.get(MatPos{row:0, col:1}));
	/// assert_eq!(mat3x4.get(MatPos{row:1, col:1}), mat4x3.get(MatPos{row:1, col:1}));
	/// assert_eq!(mat3x4.get(MatPos{row:2, col:3}), mat4x3.get(MatPos{row:3, col:2}));
	/// ```
	pub fn transpose ( &self ) -> Matrix<COLUMN, ROW>
	{
		let mut trans : Matrix<COLUMN, ROW> = Matrix::new();
		for x in 0..self.width()
		{
			for y in 0..self.height()
			{
				let s_pos = MatPos{row: y, col: x};
				let t_pos = MatPos{row: x, col: y};
				trans.set_fast(t_pos, self.get_fast(s_pos));
			}
		}
		return trans;
	}



	/// Unchecked accessor of matrix, does not check for bounds.
	fn get_fast ( &self, pos: MatPos ) -> Decimal
	{
		return self.matrix[pos.row][pos.col];
	}

	/// Unchecked accessor of matrix, does not check for bounds.
	fn set_fast ( &mut self, pos: MatPos, value: Decimal )
	{
		return self.matrix[pos.row][pos.col] = value;
	}
}


// Square Matrices
impl <const S: usize> Matrix<S, S>
{
	/// Finds the trace of the matrix (sum of diagonal).
	/// Must be a square matrix.
	///
	/// # Returns
	/// The sum of the diagonal (trace) or Errors::InvalidSize.
	///
	/// # Example
	/// ```
	/// use star_tracker::util::units::Matrix;
	/// use star_tracker::util::units::MatPos;
	/// let mut mat4x4 : Matrix<4,4> = Matrix::new();
	/// mat4x4.set(MatPos{row: 0, col: 0}, 1.0);
	/// mat4x4.set(MatPos{row: 1, col: 1}, 2.0);
	/// mat4x4.set(MatPos{row: 2, col: 2}, 4.0);
	/// mat4x4.set(MatPos{row: 3, col: 3}, 8.0);
	///
	/// assert_eq!(mat4x4.trace(), 1.0 + 2.0 + 4.0 + 8.0);
	/// ```
	pub fn trace ( &self ) -> Decimal
	{
		let mut sum = 0.0;
		for i in 0..self.width()
		{
			sum += self.get_fast(MatPos{row: i, col: i});
		}
		return sum;
	}


}









impl <const ROW: usize, const COLUMN: usize> fmt::Display for Matrix<ROW, COLUMN> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		for yy in 0..self.height()
		{
			write!(f, "|")?;
			for xx in 0..self.width()
			{
				let val = self.get_fast(MatPos{col: xx, row: yy});
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


impl <const ROW: usize, const COLUMN: usize> fmt::Debug for Matrix<ROW, COLUMN> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return (self as &dyn fmt::Display).fmt(f);
    }
}

//###############################################################################################//
//							---	Multiply/Divide Matrix ---
//###############################################################################################//
// Multiply Matrix by Matrix
impl <const M: usize, const N: usize, const P: usize> Mul<Matrix<N, P>> for Matrix<M, N>
{
	// (M, N) x (N, P) = (M, P)
	type Output = Matrix<M, P>;
    fn mul(self, rhs: Matrix<N, P>) -> Matrix<M, P>
	{
		let mut mat : Matrix<M, P> = Matrix::new();
		for y in 0..M				// OUT matrix row
		{
			for x in 0..P			// OUT matrix column
			{
				let mut output : Decimal = 0.0;
				for i in 0..N		// IN matrix (lhs x, rhs y)
				{
					output +=
						self.get_fast(MatPos{row: y, col: i}) *
						 rhs.get_fast(MatPos{row: i, col: x});
				}
				mat.set_fast(MatPos{row: y, col: x}, output);
			}
		}
		return mat;
	}
}


//###############################################################################################//
//							---	Multiply/Divide Scalar ---
//###############################################################################################//
// Multiply Matrix by Scalar

impl <const ROW: usize, const COLUMN: usize> Mul<Decimal> for Matrix<ROW, COLUMN> {
	type Output = Self;
    fn mul(self, rhs: Decimal) -> Self
	{
		let mut mat : Matrix<ROW, COLUMN> = Matrix::new();
		for xx in 0..COLUMN
		{
			for yy in 0..ROW
			{
				let pos = MatPos{col: xx, row: yy};
				mat.set_fast(pos, self.get_fast(pos) * rhs);
			}
		}
		return mat;
	}
}

// Multiply Scalar by Matrix
impl <const ROW: usize, const COLUMN: usize> Mul<Matrix<ROW, COLUMN>> for Decimal {
    type Output = Matrix<ROW, COLUMN>;
    fn mul(self, rhs: Matrix<ROW, COLUMN>) -> Matrix<ROW, COLUMN> {	return rhs * self;	}
}


// Divide Matrix by Scalar
impl <const ROW: usize, const COLUMN: usize> Div <Decimal> for Matrix<ROW, COLUMN> {
	type Output = Self;
    fn div(self, rhs: Decimal) -> Self
	{
		let mut mat : Matrix<ROW, COLUMN> = Matrix::new();
		for xx in 0..COLUMN
		{
			for yy in 0..ROW
			{
				let pos = MatPos{col: xx, row: yy};
				mat.set_fast(pos, self.get_fast(pos) / rhs);
			}
		}
		return mat;
	}
}






impl<const ROW: usize, const COLUMN: usize> PartialEq for Matrix<ROW, COLUMN> {
	fn eq ( &self, other: &Self ) -> bool
	{
		let mut eq = true;
		for xx in 0..COLUMN
		{
			for yy in 0..ROW
			{
				let pos = MatPos{col: xx, row: yy};
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
	use util::aliases::Decimal;
	use util::units::Matrix;
	use util::units::MatPos;
	use util::err::Errors;

	#[test]
	fn test_new ( )
	{
		let mat3x4 : Matrix<3, 4> = Matrix::new();
		assert_eq!( mat3x4.matrix.len(), 3);
		assert_eq!( mat3x4.matrix[0].len(), 4);
	}

	#[test]
	fn test_width ( )
	{
		let mat3x4 : Matrix<3, 4> = Matrix::new();
		assert_eq!( mat3x4.width(), 4);
	}

	#[test]
	fn test_height ( )
	{
		let mat3x4 : Matrix<3, 4> = Matrix::new();
		assert_eq!( mat3x4.height(), 3);
	}


	#[test]
	fn test_get ( )
	{
		let mut mat3x4 : Matrix<3, 4> = Matrix::new();
		mat3x4.matrix[1][2] = 3.12;
		mat3x4.matrix[2][3] = 5.23;
		assert_eq!(mat3x4.get(MatPos{row: 1, col: 2}).expect(""), 3.12);
		assert_eq!(mat3x4.get(MatPos{row: 2, col: 3}).expect(""), 5.23);
		assert_eq!(mat3x4.get(MatPos{row: 0, col: 0}).expect(""), 0.0);
	}

	#[test]
	fn test_set ( )
	{
		let mut mat3x4 : Matrix<3, 4> = Matrix::new();
		assert!(mat3x4.set(MatPos{row: 1, col: 2}, 3.12).is_ok());
		assert!(mat3x4.set(MatPos{row: 2, col: 3}, 5.23).is_ok());
		assert_eq!(mat3x4.set(MatPos{row: 4, col: 0}, 3.12), Err(Errors::OutOfBoundsY));
		assert_eq!(mat3x4.set(MatPos{row: 0, col: 5}, 5.23), Err(Errors::OutOfBoundsX));
		assert_eq!(mat3x4.matrix[1][2], 3.12);
		assert_eq!(mat3x4.matrix[2][3], 5.23);
	}









	//
	// fn transpose ( &self ) -> Matrix<H, W>
	//

	#[test]
	fn test_transpose_empty ( )
	{
		let mat0x0 : Matrix<0,0> = Matrix::new();
		let matt0x0 = mat0x0.transpose();
		assert_eq!(matt0x0.width(), 0);
		assert_eq!(matt0x0.height(), 0);
	}

	#[test]
	fn test_transpose_square ( )
	{
		let mut mat4x4 : Matrix<4, 4> = Matrix::new();
		for x in 0..mat4x4.width()
		{
			for y in 0..mat4x4.height()
			{
				mat4x4.set_fast(MatPos{row: x, col: y}, x as Decimal * 10.0 + y as Decimal);
			}
		}

		let matt4x4 : Matrix<4, 4> = mat4x4.transpose();
		assert_eq!(matt4x4.height(), 4);
		assert_eq!(matt4x4.width(), 4);
		for x in 0..mat4x4.width()
		{
			for y in 0..mat4x4.height()
			{
				assert_eq!(
					mat4x4.get(MatPos{row: x, col: y}),
					matt4x4.get(MatPos{row: y, col: x}));
			}
		}
	}

	#[test]
	fn test_transpose_rectangle ( )
	{
		let mut mat3x4 : Matrix<3, 4> = Matrix::new();
		for x in 0..mat3x4.width()
		{
			for y in 0..mat3x4.height()
			{
				mat3x4.set_fast(MatPos{col: x, row: y}, x as Decimal * 10.0 + y as Decimal);
			}
		}
		println!("BEFORE");

		let matt4x3 : Matrix<4, 3> = mat3x4.transpose();

		println!("AFTER");

		assert_eq!(matt4x3.width(), 3);
		assert_eq!(matt4x3.height(), 4);
		for x in 0..mat3x4.width()
		{
			for y in 0..mat3x4.height()
			{
				assert_eq!(
					mat3x4.get(MatPos{col: x, row: y}),
					matt4x3.get(MatPos{col: y, row: x}));
			}
		}
	}



	//
	// fn get_fast ( &self, pos: MatPos ) -> Decimal
	//

	#[test]
	fn test_get_fast ( )
	{
		let mut mat3x4 : Matrix<3, 4> = Matrix::new();
		mat3x4.matrix[1][2] = 3.12;
		mat3x4.matrix[2][3] = 5.23;
		let pos_1 = MatPos{row: 1, col: 2};
		let pos_2 = MatPos{row: 2, col: 3};
		let pos_3 = MatPos{row: 0, col: 0};
		assert_eq!(mat3x4.get_fast(pos_1), mat3x4.get(pos_1).expect(""));
		assert_eq!(mat3x4.get_fast(pos_2), mat3x4.get(pos_2).expect(""));
		assert_eq!(mat3x4.get_fast(pos_3), mat3x4.get(pos_3).expect(""));
	}

	//
	// fn set_fast ( &self, pos: MatPos, value: Decimal ) -> void
	//

	#[test]
	fn test_set_fast ( )
	{
		let mut mat3x4 : Matrix<3, 4> = Matrix::new();
		mat3x4.set_fast(MatPos{row: 1, col: 2}, 3.12);
		mat3x4.set_fast(MatPos{row: 2, col: 3}, 5.23);
		assert_eq!(mat3x4.matrix[1][2], 3.12);
		assert_eq!(mat3x4.matrix[2][3], 5.23);
	}



	//
	// fn trace ( &self ) -> Decimal
	//

	#[test]
	fn test_trace_empty ( )
	{
		let mat0x0 : Matrix<0,0> = Matrix::new();
		assert_eq!(mat0x0.trace(), 0.0);
	}

	#[test]
	fn test_trace_normal ( )
	{
		let mut mat4x4 : Matrix<4,4> = Matrix::new();
		for x in 0..mat4x4.width()
		{
			for y in 0..mat4x4.height()
			{
				mat4x4.set_fast(MatPos{row: x, col: y}, (x+1) as Decimal * 10.0 +(y+1) as Decimal);
			}
		}
		assert_eq!(mat4x4.trace(), 11.0 + 22.0 + 33.0 + 44.0);
	}




	#[test]
	fn test_mul_mat_zero ( )
	{
		let mat0x0 : Matrix<0,0> = Matrix::new();
		assert_eq!(mat0x0 * mat0x0, mat0x0);
	}

	#[test]
	fn test_mul_mat_square ( )
	{
		let mut mat3x3_1 : Matrix<3,3> = Matrix::new();
		let mut mat3x3_2 : Matrix<3,3> = Matrix::new();

		for x in 0..3
		{
			for y in 0..3
			{
				mat3x3_1.set(MatPos{col: x, row: y}, (x + y * 3) as Decimal).expect("");
				mat3x3_2.set(MatPos{col: x, row: y}, (x + y * 3 + 9) as Decimal).expect("");
			}
		}
		let output : Matrix<3,3> = mat3x3_1 * mat3x3_2;

		assert_eq!(output.get_fast(MatPos{col:0, row:0}), 42.0);
		assert_eq!(output.get_fast(MatPos{col:1, row:0}), 45.0);
		assert_eq!(output.get_fast(MatPos{col:2, row:0}), 48.0);
		assert_eq!(output.get_fast(MatPos{col:0, row:1}), 150.0);
		assert_eq!(output.get_fast(MatPos{col:1, row:1}), 162.0);
		assert_eq!(output.get_fast(MatPos{col:2, row:1}), 174.0);
		assert_eq!(output.get_fast(MatPos{col:0, row:2}), 258.0);
		assert_eq!(output.get_fast(MatPos{col:1, row:2}), 279.0);
		assert_eq!(output.get_fast(MatPos{col:2, row:2}), 300.0);
	}



	#[test]
	fn test_mul_mat_odd ( )
	{
		let mut mat1x3 : Matrix<1,3> = Matrix::new();
		let mut mat3x1 : Matrix<3,1> = Matrix::new();
		let mut mat3x2 : Matrix<3,2> = Matrix::new();

		for ii in 0..3
		{
			mat1x3.set(MatPos{row: 0,  col: ii}, ii as Decimal).expect("");
			mat3x1.set(MatPos{row: ii, col: 0},  ii as Decimal + 3.0).expect("");

			for jj in 0..2
			{
				mat3x2.set(MatPos{row: ii, col: jj}, (ii * 2 + jj + 3) as Decimal).expect("");
			}
		}
		let output_1 : Matrix<1,1> = mat1x3 * mat3x1;
		let output_2 : Matrix<1,2> = mat1x3 * mat3x2;
		let output_3 : Matrix<3,3> = mat3x1 * mat1x3;

		assert_eq!(output_1.get_fast(MatPos{col:0, row:0}), 14.0);

		assert_eq!(output_2.get_fast(MatPos{col:0, row:0}), 19.0);
		assert_eq!(output_2.get_fast(MatPos{col:1, row:0}), 22.0);
		
		assert_eq!(output_3.get_fast(MatPos{col:0, row:0}), 0.0);
		assert_eq!(output_3.get_fast(MatPos{col:1, row:0}), 3.0);
		assert_eq!(output_3.get_fast(MatPos{col:2, row:0}), 6.0);
		assert_eq!(output_3.get_fast(MatPos{col:0, row:1}), 0.0);
		assert_eq!(output_3.get_fast(MatPos{col:1, row:1}), 4.0);
		assert_eq!(output_3.get_fast(MatPos{col:2, row:1}), 8.0);
		assert_eq!(output_3.get_fast(MatPos{col:0, row:2}), 0.0);
		assert_eq!(output_3.get_fast(MatPos{col:1, row:2}), 5.0);
		assert_eq!(output_3.get_fast(MatPos{col:2, row:2}), 10.0);
	}










	#[test]
	fn test_mul_scalar ( )
	{
		let mut mat3x4 : Matrix<3,4> = Matrix::new();
		mat3x4.matrix[2][1] = 12.0;
		mat3x4.matrix[2][3] = 23.0;
		mat3x4 = mat3x4 * 2.0;
		assert_eq!(mat3x4.matrix[2][1], 24.0);
		assert_eq!(mat3x4.matrix[2][3], 46.0);
		mat3x4 = 0.5 * mat3x4;
		assert_eq!(mat3x4.matrix[2][1], 12.0);
		assert_eq!(mat3x4.matrix[2][3], 23.0);
	}

	#[test]
	fn test_div_scalar ( )
	{
		let mut mat3x4 : Matrix<3,4> = Matrix::new();
		mat3x4.matrix[2][1] = 24.0;
		mat3x4.matrix[2][3] = 46.0;
		mat3x4 = mat3x4 / 2.0;
		assert_eq!(mat3x4.matrix[2][1], 12.0);
		assert_eq!(mat3x4.matrix[2][3], 23.0);
	}








	#[test]
	fn test_eq_matrix ( )
	{
		let mut mat3x4    : Matrix<3,4> = Matrix::new();
		let mut mat_other : Matrix<3,4> = Matrix::new();
		assert!(mat3x4 == mat_other);
		mat3x4.matrix[2][1] = 12.0;
		mat3x4.matrix[2][3] = 32.0;
		mat_other.matrix[1][1] = 11.0;
		mat_other.matrix[1][0] = 01.0;
		assert!(mat3x4 != mat_other);
		mat3x4.matrix[1][1] = 11.0;
		mat3x4.matrix[1][0] = 01.0;
		mat_other.matrix[2][1] = 12.0;
		mat_other.matrix[2][3] = 32.0;
		assert!(mat3x4 == mat_other);
	}
}
