//! Implementation of [Matrix](crate::util::units::Matrix).
use core_include::*;

use super::Matrix;

use crate::util::aliases::Decimal;
use crate::util::units::Quaternion;
use crate::util::units::Vector3;
use crate::util::units::MatPos;
use crate::util::err::Errors;
use crate::util::err::Error;

use crate::util::Maths;



impl <const ROW: usize, const COLUMN: usize> Matrix <ROW, COLUMN>
{
	/// Default Constructor.
	/// Initializes matrix to 0.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Matrix;
	///
	/// let mat3x4: Matrix<3, 4> = Matrix::new(); // Matrix with 3 rows and 4 columns
	/// ```
	pub fn new ( ) -> Matrix<ROW, COLUMN>
	{
 		return Matrix { matrix: [[0.0; COLUMN]; ROW] };
	}
}

	
impl <const S: usize> Matrix<S, S>
{
	/// Contructs an identity matrix.
	pub fn identity ( ) -> Matrix<S, S>
	{
		let mut mat: Matrix<S, S> = Matrix::new();
		for i in 0..S
		{
			mat.set(MatPos{row: i, col: i}, 1.0);
		}
		return mat;
	}
}


impl <const ROW: usize, const COLUMN: usize> Matrix <ROW, COLUMN>
{
	/// Returns the width of the matrix.
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Matrix;
	///
	/// let mat3x4: Matrix<3, 4> = Matrix::new();
	/// assert_eq!(mat3x4.width(), 4);
	/// ```
	pub fn width  ( &self ) -> usize	{	return COLUMN;	}

	/// Returns the width of the matrix.
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Matrix;
	///
	/// let mat3x4: Matrix<3, 4> = Matrix::new();
	/// assert_eq!(mat3x4.height(), 3);
	/// ```
	pub fn height ( &self ) -> usize	{	return ROW;	}

	/// Returns the value at the given index.
	/// # Example
	/// ```
	/// use star_tracker_lib::util::err::Errors;
	/// use star_tracker_lib::util::units::Matrix;
	/// use star_tracker_lib::util::units::MatPos;
	///
	/// let mut mat3x4: Matrix<3, 4> = Matrix::new();
	/// mat3x4.set(MatPos{row: 1, col: 2}, 3.12);
	/// mat3x4.set(MatPos{row: 2, col: 3}, 5.23);
	/// assert_eq!(mat3x4.get(MatPos{row: 1, col: 2}), 3.12);
	/// assert_eq!(mat3x4.get(MatPos{row: 2, col: 3}), 5.23);
	/// assert_eq!(mat3x4.get(MatPos{row: 0, col: 0}), 0.0);
	/// ```
	pub fn get ( &self, pos: MatPos ) -> Decimal
	{
		return self.matrix[pos.row][pos.col];
	}

	/// Returns the value at the given index.
	/// # Example
	/// ```
	/// use star_tracker_lib::util::err::Errors;
	/// use star_tracker_lib::util::units::Matrix;
	/// use star_tracker_lib::util::units::MatPos;
	///
	/// let mut mat3x4: Matrix<3, 4> = Matrix::new();
	/// mat3x4.set(MatPos{row: 1, col: 2}, 3.12);
	/// mat3x4.set(MatPos{row: 2, col: 3}, 5.23);
	/// assert_eq!(mat3x4.get(MatPos{row: 1, col: 2}), 3.12);
	/// assert_eq!(mat3x4.get(MatPos{row: 2, col: 3}), 5.23);
	/// assert_eq!(mat3x4.get(MatPos{row: 0, col: 0}), 0.0);
	/// ```
	pub fn set ( &mut self, pos: MatPos, value: Decimal )
	{
		self.matrix[pos.row][pos.col] = value;
	}



	/// Flips the rows and columns.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Matrix;
	/// use star_tracker_lib::util::units::MatPos;
	///
	/// let mut mat3x4: Matrix<3,4> = Matrix::new();
	/// mat3x4.set(MatPos{row: 0, col: 0}, 0.0);
	/// mat3x4.set(MatPos{row: 0, col: 1}, 0.1);
	/// mat3x4.set(MatPos{row: 1, col: 0}, 1.0);
	/// mat3x4.set(MatPos{row: 1, col: 1}, 1.1);
	/// mat3x4.set(MatPos{row: 2, col: 3}, 2.3);
	/// let mat4x3 = mat3x4.transposed();
	/// assert_eq!(mat4x3.width(), 3);
	/// assert_eq!(mat4x3.height(), 4);
	/// assert_eq!(mat3x4.get(MatPos{row:0, col:0}), mat4x3.get(MatPos{row:0, col:0}));
	/// assert_eq!(mat3x4.get(MatPos{row:0, col:1}), mat4x3.get(MatPos{row:1, col:0}));
	/// assert_eq!(mat3x4.get(MatPos{row:1, col:0}), mat4x3.get(MatPos{row:0, col:1}));
	/// assert_eq!(mat3x4.get(MatPos{row:1, col:1}), mat4x3.get(MatPos{row:1, col:1}));
	/// assert_eq!(mat3x4.get(MatPos{row:2, col:3}), mat4x3.get(MatPos{row:3, col:2}));
	/// ```
	pub fn transposed ( &self ) -> Matrix<COLUMN, ROW>
	{
		let mut trans: Matrix<COLUMN, ROW> = Matrix::new();
		for x in 0..self.width()
		{
			for y in 0..self.height()
			{
				let s_pos = MatPos{row: y, col: x};
				let t_pos = MatPos{row: x, col: y};
				trans.set(t_pos, self.get(s_pos));
			}
		}
		return trans;
	}








	/// Inserts the matrix into the specified location on this matrix.
	/// # Returns
	/// Ok(()) if the matrix fits.  
	/// Err(Errors::InvalidSize) if the matrix does not fit at the location specified.  
	/// On failure, the matrix is not inserted.  
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Matrix;
	/// use star_tracker_lib::util::units::MatPos;
	///
	/// let mut mat4x4: Matrix<4,4> = Matrix::new();
	/// let mut insert: Matrix<3,2> = Matrix::new();
	///
	/// insert.set(MatPos{row: 0, col: 0}, 1.0);
	/// insert.set(MatPos{row: 0, col: 1}, 2.0);
	/// insert.set(MatPos{row: 1, col: 0}, 3.0);
	/// insert.set(MatPos{row: 1, col: 1}, 4.0);
	/// insert.set(MatPos{row: 2, col: 0}, 5.0);
	/// insert.set(MatPos{row: 2, col: 1}, 6.0);
	///
	/// mat4x4.insert(MatPos{row: 1, col: 1}, &insert);
	///
	/// assert_eq!(mat4x4.get(MatPos{row: 0, col: 0}), 0.0); // first element is 1,1
	/// assert_eq!(mat4x4.get(MatPos{row: 1, col: 1}), 1.0);
	/// assert_eq!(mat4x4.get(MatPos{row: 1, col: 2}), 2.0);
	/// assert_eq!(mat4x4.get(MatPos{row: 2, col: 1}), 3.0);
	/// assert_eq!(mat4x4.get(MatPos{row: 2, col: 2}), 4.0);
	/// assert_eq!(mat4x4.get(MatPos{row: 3, col: 1}), 5.0);
	/// assert_eq!(mat4x4.get(MatPos{row: 3, col: 2}), 6.0);
	/// ```
	pub fn insert
		<const C_2: usize, const R_2: usize>
		( &mut self, pos: MatPos, other: &Matrix<C_2, R_2> ) -> Error<()>
	{
		if self.height() < pos.row + other.height() || self.width() < pos.col + other.width()
		{
			return Result::Err(Errors::InvalidSize);
		}
		for x in 0..other.width()
		{
			for y in 0..other.height()
			{
				let set_pos = MatPos{row: pos.row + y, col: pos.col + x};
				self.set(set_pos, other.get(MatPos{row: y, col: x}));
			}
		}

		return Result::Ok(());
	}
}

// Square Matrices
impl <const S: usize> Matrix<S, S>
{
	/// Finds the trace of the matrix (sum of diagonal).
	/// Must be a square matrix.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Matrix;
	/// use star_tracker_lib::util::units::MatPos;
	///
	/// let mut mat4x4: Matrix<4,4> = Matrix::new();
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
			sum += self.get(MatPos{row: i, col: i});
		}
		return sum;
	}


}




impl Matrix <1, 1>
{
	/// Converts the current value to a matrix.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Matrix;
	/// use star_tracker_lib::util::units::MatPos;
	///
	/// let mat1x1: Matrix<1,1> = Matrix::from_decimal(100.0);
	/// assert_eq!(mat1x1.get(MatPos{row: 0, col: 0}), 100.0);
	/// ```
	pub fn from_decimal ( val: Decimal ) -> Matrix<1,1>
	{
		let mut mat: Matrix<1, 1> = Matrix::new();
		mat.set(MatPos{row: 0, col: 0}, val);
		return mat;
	}

	/// Converts the current value to a decimal.
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Matrix;
	/// use star_tracker_lib::util::units::MatPos;
	///
	/// let mut mat1x1: Matrix<1,1> = Matrix::new();
	/// mat1x1.set(MatPos{row: 0, col: 0}, 100.0);
	/// assert_eq!(mat1x1.to_decimal(), 100.0);
	/// ```
	pub fn to_decimal ( &self ) -> Decimal
	{
		return self.get(MatPos{row: 0, col: 0});
	}


	/// Returns the single element.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Matrix;
	///
	/// use star_tracker_lib::util::units::MatPos;
	/// let mut mat1x1: Matrix<1,1> = Matrix::new();
	/// mat1x1.set(MatPos{row: 0, col: 0}, 10.0);
	/// assert_eq!(mat1x1.determinate(), 10.0);
	/// ```
	pub fn determinate ( &self ) -> Decimal
	{
		return self.get(MatPos{row: 0, col: 0});
	}
}

impl Matrix <2, 2>
{
	/// Recursively finds the determinate.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Matrix;
	/// use star_tracker_lib::util::units::MatPos;
	///
	/// let mut mat2x2: Matrix<2,2> = Matrix::new();
	/// mat2x2.set(MatPos{row: 0, col: 0}, 2.0);
	/// mat2x2.set(MatPos{row: 1, col: 0}, 3.0);
	/// mat2x2.set(MatPos{row: 0, col: 1}, 4.0);
	/// mat2x2.set(MatPos{row: 1, col: 1}, 5.0);
	/// assert_eq!(mat2x2.determinate(), 2.0 * 5.0 - 3.0 * 4.0);
	/// assert_eq!(mat2x2.determinate(), -2.0);
	///```
	pub fn determinate ( &self ) -> Decimal
	{
		return 	self.get(MatPos{row: 0, col: 0}) * self.get(MatPos{row: 1, col: 1}) -
				self.get(MatPos{row: 1, col: 0}) * self.get(MatPos{row: 0, col: 1});
	}
}

impl Matrix <3, 3>
{
	/// Recursively finds the determinate.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Matrix;
	/// use star_tracker_lib::util::units::MatPos;
	///
	///	let mut mat3x3: Matrix<3,3> = Matrix::new();
	/// mat3x3.set(MatPos{row: 0, col: 0}, 5.0);
	/// mat3x3.set(MatPos{row: 0, col: 1}, 4.0);
	/// mat3x3.set(MatPos{row: 0, col: 2}, 3.0);
	/// mat3x3.set(MatPos{row: 1, col: 0}, 8.0);
	/// mat3x3.set(MatPos{row: 1, col: 1}, 9.0);
	/// mat3x3.set(MatPos{row: 1, col: 2}, 10.0);
	/// mat3x3.set(MatPos{row: 2, col: 0}, 100.0);
	/// mat3x3.set(MatPos{row: 2, col: 1}, 2.0);
	/// mat3x3.set(MatPos{row: 2, col: 2}, 6.0);
	/// assert_eq!(mat3x3.determinate(), 1326.0);
	/// ```
	pub fn determinate ( &self ) -> Decimal
	{
		let mut det = 0.0;
		for i in 0..3
		{
			let multiplier = self.get(MatPos{row: 0, col: i});
			let mut sub_mat: Matrix<2,2> = Matrix::new();
			for col in 0..3
			{
				let sub_col = if col <= i { col } else { col - 1 };
				if col != i
				{
					sub_mat.set(MatPos{row: 0, col: sub_col}, self.get(MatPos{row: 1, col: col}));
					sub_mat.set(MatPos{row: 1, col: sub_col}, self.get(MatPos{row: 2, col: col}));
				}
			}
			if i % 2 == 0
			{
				det += multiplier * sub_mat.determinate();
			}
			else
			{
				det -= multiplier * sub_mat.determinate();
			}
		}
		return det;
	}



	/// Finds the adjoint matrix.
	/// The adjoint matrix replaces each cell with the determinate from looking at that cell.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Matrix;
	/// use star_tracker_lib::util::units::MatPos;
	///
	///	let mut mat3x3: Matrix<3,3> = Matrix::new();
	/// mat3x3.set(MatPos{row: 0, col: 0}, 5.0);
	/// mat3x3.set(MatPos{row: 0, col: 1}, 4.0);
	/// mat3x3.set(MatPos{row: 0, col: 2}, 3.0);
	/// mat3x3.set(MatPos{row: 1, col: 0}, 8.0);
	/// mat3x3.set(MatPos{row: 1, col: 1}, 9.0);
	/// mat3x3.set(MatPos{row: 1, col: 2}, 10.0);
	/// mat3x3.set(MatPos{row: 2, col: 0}, 100.0);
	/// mat3x3.set(MatPos{row: 2, col: 1}, 2.0);
	/// mat3x3.set(MatPos{row: 2, col: 2}, 6.0);
	///
	/// let mut res: Matrix<3,3> = Matrix::new();
	/// res.set(MatPos{row: 0, col: 0}, 34.0);
	/// res.set(MatPos{row: 0, col: 1}, 952.0);
	/// res.set(MatPos{row: 0, col: 2}, -884.0);
	/// res.set(MatPos{row: 1, col: 0}, -18.0);
	/// res.set(MatPos{row: 1, col: 1}, -270.0);
	/// res.set(MatPos{row: 1, col: 2}, 390.0);
	/// res.set(MatPos{row: 2, col: 0}, 13.0);
	/// res.set(MatPos{row: 2, col: 1}, -26.0);
	/// res.set(MatPos{row: 2, col: 2}, 13.0);
	///
	/// assert_eq!(res, mat3x3.adjoint());
	///
	pub fn adjoint ( &self ) -> Matrix<3,3>
	{
		let mut adj: Matrix<3,3> = Matrix::new();
		for r in 0..3
		{
			for c in 0..3
			{
				let mut sub_mat: Matrix<2,2> = Matrix::new();

				// create matrix
				for row in 0..3
				{
					for col in 0..3
					{
						if row != r && c != col
						{
							let sub_row = if row < r { row } else { row - 1 };
							let sub_col = if col < c { col } else { col - 1 };

							sub_mat.set(MatPos{row: sub_row, col: sub_col},
									self.get(MatPos{row: row, col: col}));
						}
					}
				}
				let val = if (r+c)%2 == 0 { sub_mat.determinate() }else{ -sub_mat.determinate() };
				adj.set(MatPos{row: r, col: c}, val);
			}
		}
		return adj;
	}
}

impl Matrix <4, 4>
{
	/// Recursively finds the determinate.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Matrix;
	/// use star_tracker_lib::util::units::MatPos;
	///
	/// let mut mat4x4: Matrix<4,4> = Matrix::new();
	///
	/// mat4x4.set(MatPos{row: 0, col: 0}, 5.0);
	/// mat4x4.set(MatPos{row: 0, col: 1}, 10.0);
	/// mat4x4.set(MatPos{row: 0, col: 2}, 50.0);
	/// mat4x4.set(MatPos{row: 0, col: 3}, 2.0);
	///
	/// mat4x4.set(MatPos{row: 1, col: 0}, 8.0);
	/// mat4x4.set(MatPos{row: 1, col: 1}, 20.0);
	/// mat4x4.set(MatPos{row: 1, col: 2}, 60.0);
	/// mat4x4.set(MatPos{row: 1, col: 3}, 11.0);
	///
	/// mat4x4.set(MatPos{row: 2, col: 0}, 100.0);
	/// mat4x4.set(MatPos{row: 2, col: 1}, 30.0);
	/// mat4x4.set(MatPos{row: 2, col: 2}, 70.0);
	/// mat4x4.set(MatPos{row: 2, col: 3}, 103.0);
	///
	/// mat4x4.set(MatPos{row: 3, col: 0}, 50.0);
	/// mat4x4.set(MatPos{row: 3, col: 1}, 40.0);
	/// mat4x4.set(MatPos{row: 3, col: 2}, 80.0);
	/// mat4x4.set(MatPos{row: 3, col: 3}, 53.0);
	///
	/// assert_eq!(mat4x4.determinate(), 340800.0);
	/// ```
	pub fn determinate ( &self ) -> Decimal
	{
		let mut det = 0.0;
		for i in 0..4
		{
			let multiplier = self.get(MatPos{row: 0, col: i});
			let mut sub_mat: Matrix<3,3> = Matrix::new();
			for col in 0..4
			{
				let sub_col = if col <= i { col } else { col - 1 };
				if col != i
				{
					sub_mat.set(MatPos{row: 0, col: sub_col}, self.get(MatPos{row: 1, col: col}));
					sub_mat.set(MatPos{row: 1, col: sub_col}, self.get(MatPos{row: 2, col: col}));
					sub_mat.set(MatPos{row: 2, col: sub_col}, self.get(MatPos{row: 3, col: col}));
				}
			}
			if i % 2 == 0
			{
				det += multiplier * sub_mat.determinate();
			}
			else
			{
				det -= multiplier * sub_mat.determinate();
			}
		}
		return det;
	}
}



//###############################################################################################//
//							---	Convert to Vector3 Coords ---
//###############################################################################################//

impl Matrix <3, 1>
{
	/// Converts the matrix into a Vector3.
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Matrix;
	/// use star_tracker_lib::util::units::MatPos;
	/// use star_tracker_lib::util::units::Vector3;
	///
	/// let mut mat3x1: Matrix<3,1> = Matrix::new();
	/// mat3x1.set(MatPos{row: 0, col: 0}, 1.0);
	/// mat3x1.set(MatPos{row: 1, col: 0}, 2.0);
	/// mat3x1.set(MatPos{row: 2, col: 0}, 3.0);
	/// assert_eq!(mat3x1.to_vector3(), Vector3{x: 1.0, y: 2.0, z: 3.0});
	/// ```
	pub fn to_vector3 ( &self ) -> Vector3
	{
		return Vector3{
			x: self.get(MatPos{row: 0, col: 0}),
			y: self.get(MatPos{row: 1, col: 0}),
			z: self.get(MatPos{row: 2, col: 0}),
		};
	}
}

impl Matrix <4, 1>
{
	/// Converts the matrix into a Vector3.
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Matrix;
	/// use star_tracker_lib::util::units::MatPos;
	/// use star_tracker_lib::util::units::Vector3;
	///
	/// let mut mat4x1: Matrix<4,1> = Matrix::new();
	/// mat4x1.set(MatPos{row: 0, col: 0}, 1.0);
	/// mat4x1.set(MatPos{row: 1, col: 0}, 2.0);
	/// mat4x1.set(MatPos{row: 2, col: 0}, 3.0);
	/// mat4x1.set(MatPos{row: 3, col: 0}, 0.5);
	/// assert_eq!(mat4x1.to_vector3(), Vector3{x: 2.0, y: 4.0, z: 6.0});
	/// ```
	pub fn to_vector3 ( &self ) -> Vector3
	{
		return Vector3{
			x: self.get(MatPos{row: 0, col: 0}) / self.get(MatPos{row: 3, col: 0}),
			y: self.get(MatPos{row: 1, col: 0}) / self.get(MatPos{row: 3, col: 0}),
			z: self.get(MatPos{row: 2, col: 0}) / self.get(MatPos{row: 3, col: 0}),
		};
	}
}



//###############################################################################################//
//							---	Convert to Quaternion Rotation ---
//###############################################################################################//
// impl Matrix <3,3>
// {
// 	/// Converts a rotation matrix to a quaternion.
// 	/// Derived from [Unity](https://answers.unity.com/questions/467614/what-is-the-source-code-of-quaternionlookrotation.html).
// 	/// # Example
// 	/// ```
// 	/// panic!("WIP");
// 	/// ```
// 	pub fn to_quaternion ( &self ) -> Quaternion
// 	{
// 		let trace: Decimal = self.trace();
// 		if trace > 0.0
// 		{
// 			// let mut num = Decimal::sqrt(trace + 1.0);
// 			let s = 2.0 * Decimal::sqrt(trace + 1.0);
// 			// let w = num * 0.5;
// 			let w = 0.25 * s;
// 			// num = 0.5 / num;
// 			// let x = (self.get(MatPos{row: 1, col: 2}) - self.get(MatPos{row: 2, col: 1})) * num;
// 			// let y = (self.get(MatPos{row: 2, col: 0}) - self.get(MatPos{row: 0, col: 2})) * num;
// 			// let z = (self.get(MatPos{row: 0, col: 1}) - self.get(MatPos{row: 1, col: 0})) * num;
// 
// 			let x = (self.get(MatPos{row: 2, col: 1}) - self.get(MatPos{row: 1, col: 2})) / s;
// 			let y = (self.get(MatPos{row: 0, col: 2}) - self.get(MatPos{row: 2, col: 0})) / s;
// 			let z = (self.get(MatPos{row: 1, col: 0}) - self.get(MatPos{row: 0, col: 1})) / s;
// 			return Quaternion{w: w, x: x, y: y, z: z};
// 		}
// 		if  self.get(MatPos{row: 0, col: 0}) >= self.get(MatPos{row: 1, col: 1})
// 		 && self.get(MatPos{row: 0, col: 0}) >= self.get(MatPos{row: 2, col: 2})
// 		{
// 			let num7 = Decimal::sqrt(1.0
// 									+ self.get(MatPos{row: 0, col: 0})
// 									- self.get(MatPos{row: 1, col: 1})
// 									- self.get(MatPos{row: 2, col: 2}));
// 			let num4 = 0.5 / num7;
// 
// 			let w = (self.get(MatPos{row: 1, col: 2}) - self.get(MatPos{row: 2, col: 1})) * num4;
// 			let x = (self.get(MatPos{row: 0, col: 1}) - self.get(MatPos{row: 1, col: 0})) * num4;
// 			let y = (self.get(MatPos{row: 0, col: 2}) - self.get(MatPos{row: 2, col: 0})) * num4;
// 			let z = (self.get(MatPos{row: 1, col: 2}) - self.get(MatPos{row: 2, col: 1})) * num4;
// 			return Quaternion{w: w, x: x, y: y, z: z};
// 		}
// 		if self.get(MatPos{row: 1, col: 1}) > self.get(MatPos{row: 2, col: 2})
// 		{
// 			let num6 = Decimal::sqrt(1.0
// 				+ self.get(MatPos{row: 1, col: 1})
// 				- self.get(MatPos{row: 0, col: 0})
// 				- self.get(MatPos{row: 2, col: 2}));
// 
// 			let num3 = 0.5 / num6;
// 
// 			let x = (self.get(MatPos{row: 1, col: 0}) - self.get(MatPos{row: 0, col: 1})) * num3;
// 			let y = 0.5 * num6;
// 			let z = (self.get(MatPos{row: 2, col: 1}) - self.get(MatPos{row: 1, col: 2})) * num3;
// 			let w = (self.get(MatPos{row: 2, col: 0}) - self.get(MatPos{row: 0, col: 2})) * num3;
// 			return Quaternion{w: w, x: x, y: y, z: z};
// 		}
// 
// 		let num5 = Decimal::sqrt(1.0
// 			+ self.get(MatPos{row: 2, col: 2})
// 			- self.get(MatPos{row: 0, col: 0})
// 			- self.get(MatPos{row: 1, col: 1}));
// 
// 		let num2 = 0.5 / num5;
// 
// 		let x = (self.get(MatPos{row: 2, col: 0}) - self.get(MatPos{row: 0, col: 2})) * num2;
// 		let y = (self.get(MatPos{row: 2, col: 1}) - self.get(MatPos{row: 1, col: 2})) * num2;
// 		let z = 0.5 * num5;
// 		let w = (self.get(MatPos{row: 0, col: 1}) - self.get(MatPos{row: 1, col: 0})) * num2;
// 		return Quaternion{w: w, x: x, y: y, z: z};
// 	}
// }

// Pure rotation matrix.
impl Matrix <3, 3>
{
	/// Multiplies this matrix by the provided vector.
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Vector3;
	/// use star_tracker_lib::util::units::Matrix;
	/// use star_tracker_lib::util::units::MatPos;
	///
	///	let pt = Vector3{x: 1.0, y: 2.0, z: 3.0};
	///	let mut mat3x3: Matrix<3,3> = Matrix::new();
	///
	///	// x is the addition of all.
	///	mat3x3.set(MatPos{row: 0, col: 0}, 1.0);
	///	mat3x3.set(MatPos{row: 0, col: 1}, 1.0);
	///	mat3x3.set(MatPos{row: 0, col: 2}, 1.0);
	///
	///	// y is 2y + z.
	///	mat3x3.set(MatPos{row: 1, col: 0}, 0.0);
	///	mat3x3.set(MatPos{row: 1, col: 1}, 2.0);
	///	mat3x3.set(MatPos{row: 1, col: 2}, 1.0);
	///
	///	// z is 0.5x + 3.0y + 2.0z.
	///	mat3x3.set(MatPos{row: 2, col: 0}, 0.5);
	///	mat3x3.set(MatPos{row: 2, col: 1}, 3.0);
	///	mat3x3.set(MatPos{row: 2, col: 2}, 2.0);
	///
	///	let expected = Vector3
	///	{
	///		x: pt.x + pt.y + pt.z,
	///		y: 2.0 * pt.y + pt.z,
	///		z: 0.5 * pt.x + 3.0 * pt.y + 2.0 * pt.z,
	///	};
	///	assert_eq!(mat3x3.multiply(pt), expected);
	/// ```
	pub fn multiply ( &self, rhs: Vector3 ) -> Vector3
	{
		let mut pt: Matrix<3, 1> = Matrix::new();
		pt.set(MatPos{row: 0, col: 0}, rhs.x);
		pt.set(MatPos{row: 1, col: 0}, rhs.y);
		pt.set(MatPos{row: 2, col: 0}, rhs.z);

		let val = *self * pt;

		return Vector3{
			x: val.get(MatPos{row: 0, col: 0}),
			y: val.get(MatPos{row: 1, col: 0}),
			z: val.get(MatPos{row: 2, col: 0})};
	}
}

// Non homogeneous matrix
impl Matrix <3, 4>
{
	/// Multiplies this matrix by the provided vector.
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Vector3;
	/// use star_tracker_lib::util::units::Matrix;
	/// use star_tracker_lib::util::units::MatPos;
	///
	///	let pt = Vector3{x: 1.0, y: 2.0, z: 3.0};
	///	let mut mat4x4: Matrix<3,4> = Matrix::new();
	///
	///	// x is the addition of all + 1.
	///	mat4x4.set(MatPos{row: 0, col: 0}, 1.0);
	///	mat4x4.set(MatPos{row: 0, col: 1}, 1.0);
	///	mat4x4.set(MatPos{row: 0, col: 2}, 1.0);
	///	mat4x4.set(MatPos{row: 0, col: 3}, 1.0);
	///
	///	// y is 2y + z.
	///	mat4x4.set(MatPos{row: 1, col: 0}, 0.0);
	///	mat4x4.set(MatPos{row: 1, col: 1}, 2.0);
	///	mat4x4.set(MatPos{row: 1, col: 2}, 1.0);
	///	mat4x4.set(MatPos{row: 1, col: 3}, 0.0);
	///
	///	// z is 0.5x + 3.0y + 2.0z + 1.
	///	mat4x4.set(MatPos{row: 2, col: 0}, 0.5);
	///	mat4x4.set(MatPos{row: 2, col: 1}, 3.0);
	///	mat4x4.set(MatPos{row: 2, col: 2}, 2.0);
	///	mat4x4.set(MatPos{row: 2, col: 3}, 1.0);
	///
	///	let expected = Vector3
	///	{
	///		x: pt.x + pt.y + pt.z + 1.0,
	///		y: 2.0 * pt.y + pt.z,
	///		z: 0.5 * pt.x + 3.0 * pt.y + 2.0 * pt.z + 1.0,
	///	};
	///	assert_eq!(mat4x4.multiply(pt), expected);
	/// ```
	pub fn multiply ( &self, rhs: Vector3 ) -> Vector3
	{
		let mut pt: Matrix<4, 1> = Matrix::new();
		pt.set(MatPos{row: 0, col: 0}, rhs.x);
		pt.set(MatPos{row: 1, col: 0}, rhs.y);
		pt.set(MatPos{row: 2, col: 0}, rhs.z);
		pt.set(MatPos{row: 3, col: 0}, 1.0);

		let val = *self * pt;

		return Vector3{
			x: val.get(MatPos{row: 0, col: 0}),
			y: val.get(MatPos{row: 1, col: 0}),
			z: val.get(MatPos{row: 2, col: 0})};
	}
}

// Homogeneous matrix
impl Matrix <4, 4>
{
	/// Multiplies this matrix by the provided vector.
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Vector3;
	/// use star_tracker_lib::util::units::Matrix;
	/// use star_tracker_lib::util::units::MatPos;
	///
	/// let pt = Vector3{x: 1.0, y: 2.0, z: 3.0};
	/// let mut mat3x4: Matrix<4,4> = Matrix::new();
	///
	/// // x is the addition of all.
	/// mat3x4.set(MatPos{row: 0, col: 0}, 1.0);
	/// mat3x4.set(MatPos{row: 0, col: 1}, 1.0);
	/// mat3x4.set(MatPos{row: 0, col: 2}, 1.0);
	/// mat3x4.set(MatPos{row: 0, col: 3}, 0.0);
	///
	/// // y is 2y + z + 1.
	/// mat3x4.set(MatPos{row: 1, col: 0}, 0.0);
	/// mat3x4.set(MatPos{row: 1, col: 1}, 2.0);
	/// mat3x4.set(MatPos{row: 1, col: 2}, 1.0);
	/// mat3x4.set(MatPos{row: 1, col: 3}, 1.0);
	///
	/// // z is 0.5x + 3.0y + 2.0z.
	/// mat3x4.set(MatPos{row: 2, col: 0}, 0.5);
	/// mat3x4.set(MatPos{row: 2, col: 1}, 3.0);
	/// mat3x4.set(MatPos{row: 2, col: 2}, 2.0);
	/// mat3x4.set(MatPos{row: 2, col: 3}, 0.0);
	///
	/// // w is 2x + 2y + 2z + 2.
	/// mat3x4.set(MatPos{row: 3, col: 0}, 2.0);
	/// mat3x4.set(MatPos{row: 3, col: 1}, 2.0);
	/// mat3x4.set(MatPos{row: 3, col: 2}, 2.0);
	/// mat3x4.set(MatPos{row: 3, col: 3}, 2.0);
	///
	/// let w = 2.0 * (pt.x + pt.y + pt.z) + 2.0;
	/// let expected = Vector3
	/// {
	/// 	x: pt.x + pt.y + pt.z,
	/// 	y: 2.0 * pt.y + pt.z + 1.0,
	/// 	z: 0.5 * pt.x + 3.0 * pt.y + 2.0 * pt.z,
	/// } / w;
	/// assert_eq!(mat3x4.multiply(pt), expected);
	/// ```
	pub fn multiply ( &self, rhs: Vector3 ) -> Vector3
	{
		let mut pt: Matrix<4, 1> = Matrix::new();
		pt.set(MatPos{row: 0, col: 0}, rhs.x);
		pt.set(MatPos{row: 1, col: 0}, rhs.y);
		pt.set(MatPos{row: 2, col: 0}, rhs.z);
		pt.set(MatPos{row: 3, col: 0}, 1.0);

		let val = *self * pt;

		return Vector3{
			x: val.get(MatPos{row: 0, col: 0}) / val.get(MatPos{row: 3, col: 0}),
			y: val.get(MatPos{row: 1, col: 0}) / val.get(MatPos{row: 3, col: 0}),
			z: val.get(MatPos{row: 2, col: 0}) / val.get(MatPos{row: 3, col: 0}),
		};
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
	// use rand::prelude::*;
	use crate::util::aliases::Decimal;
	use crate::util::units::Vector3;
	use crate::util::units::Matrix;
	use crate::util::units::MatPos;
	use crate::util::err::Errors;

	
//###############################################################################################//
//
//									Constructors/Accessors
//
// pub fn new      ( ) -> Self
// pub fn identity ( ) -> Self
// pub fn width    ( &self ) -> usize
// pub fn height   ( &self ) -> usize
// pub fn get      ( &self ) -> Decimal
// pub fn set      ( &self, Decimal ) -> Decimal
//
//###############################################################################################//
//										~ new ~													 //
	#[test]
	fn test_new ( )
	{
		let mat3x4: Matrix<3, 4> = Matrix::new();
		assert_eq!( mat3x4.matrix.len(), 3);
		assert_eq!( mat3x4.matrix[0].len(), 4);
	}

//										~ identity ~											 //
	#[test]
	fn test_identity ( )
	{
		let _mat_0: Matrix<0, 0> = Matrix::identity();
		let mat_1: Matrix<1, 1> = Matrix::identity();
		let mat_2: Matrix<2, 2> = Matrix::identity();

		assert_eq!(mat_1.get(MatPos{row: 0, col: 0}), 1.0);

		assert_eq!(mat_2.get(MatPos{row: 0, col: 0}), 1.0);
		assert_eq!(mat_2.get(MatPos{row: 1, col: 0}), 0.0);
		assert_eq!(mat_2.get(MatPos{row: 0, col: 1}), 0.0);
		assert_eq!(mat_2.get(MatPos{row: 1, col: 1}), 1.0);
	}



//										~ width ~												 //
	#[test]
	fn test_width ( )
	{
		let mat3x4: Matrix<3, 4> = Matrix::new();
		assert_eq!( mat3x4.width(), 4);
	}

//										~ height ~												 //
	#[test]
	fn test_height ( )
	{
		let mat3x4: Matrix<3, 4> = Matrix::new();
		assert_eq!( mat3x4.height(), 3);
	}


//										~ get ~													 //
	#[test]
	fn test_get ( )
	{
		let mut mat3x4: Matrix<3, 4> = Matrix::new();
		mat3x4.matrix[1][2] = 3.12;
		mat3x4.matrix[2][3] = 5.23;
		assert_eq!(mat3x4.get(MatPos{row: 1, col: 2}), 3.12);
		assert_eq!(mat3x4.get(MatPos{row: 2, col: 3}), 5.23);
		assert_eq!(mat3x4.get(MatPos{row: 0, col: 0}), 0.0);
	}

//										~ set ~													 //
	#[test]
	fn test_set ( )
	{
		let mut mat3x4: Matrix<3, 4> = Matrix::new();
		mat3x4.set(MatPos{row: 1, col: 2}, 3.12);
		mat3x4.set(MatPos{row: 2, col: 3}, 5.23);
		assert_eq!(mat3x4.matrix[1][2], 3.12);
		assert_eq!(mat3x4.matrix[2][3], 5.23);
	}







//###############################################################################################//
//
//									Transformations
//
// pub fn transposed ( &self ) -> Self
// pub fn insert     ( &self, &MatPos, &Matrix) ->4
// pub fn height ( &self ) -> usize
// pub fn get    ( &self ) -> Decimal
// pub fn set    ( &self, Decimal ) -> Decimal
//
//###############################################################################################//
//										~ transposed ~											 //
	#[test]
	fn test_transposed_empty ( )
	{
		let mat0x0: Matrix<0,0> = Matrix::new();
		let matt0x0 = mat0x0.transposed();
		assert_eq!(matt0x0.width(), 0);
		assert_eq!(matt0x0.height(), 0);
	}

	#[test]
	fn test_transposed_square ( )
	{
		let mut mat4x4: Matrix<4, 4> = Matrix::new();
		for x in 0..mat4x4.width()
		{
			for y in 0..mat4x4.height()
			{
				mat4x4.set(MatPos{row: x, col: y}, x as Decimal * 10.0 + y as Decimal);
			}
		}

		let matt4x4: Matrix<4, 4> = mat4x4.transposed();
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
	fn test_transposed_rectangle ( )
	{
		let mut mat3x4: Matrix<3, 4> = Matrix::new();
		for x in 0..mat3x4.width()
		{
			for y in 0..mat3x4.height()
			{
				mat3x4.set(MatPos{col: x, row: y}, x as Decimal * 10.0 + y as Decimal);
			}
		}
		println!("BEFORE");

		let matt4x3: Matrix<4, 3> = mat3x4.transposed();

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


//										~ insert ~												 //
	#[test]
	// If the matrix fits.
	fn test_insert_valid ( )
	{
		let mut mat4x4: Matrix<4,4> = Matrix::new();
		let mut insert: Matrix<3,2> = Matrix::new();

		insert.set(MatPos{row: 0, col: 0}, 1.0);
		insert.set(MatPos{row: 0, col: 1}, 2.0);
		insert.set(MatPos{row: 1, col: 0}, 3.0);
		insert.set(MatPos{row: 1, col: 1}, 4.0);
		insert.set(MatPos{row: 2, col: 0}, 5.0);
		insert.set(MatPos{row: 2, col: 1}, 6.0);

		mat4x4.insert(MatPos{row: 1, col: 2}, &insert).expect("should fit.");

		assert_eq!(mat4x4.get(MatPos{row: 0, col: 0}), 0.0); // first element is 1,2
		assert_eq!(mat4x4.get(MatPos{row: 1, col: 2}), 1.0);
		assert_eq!(mat4x4.get(MatPos{row: 1, col: 3}), 2.0);
		assert_eq!(mat4x4.get(MatPos{row: 2, col: 2}), 3.0);
		assert_eq!(mat4x4.get(MatPos{row: 2, col: 3}), 4.0);
		assert_eq!(mat4x4.get(MatPos{row: 3, col: 2}), 5.0);
		assert_eq!(mat4x4.get(MatPos{row: 3, col: 3}), 6.0);
	}
	
	#[test]
	// If the matrix is too wide...
	fn test_insert_invalid_size_width ( )
	{
		let mut mat_safe: Matrix<2,2> = Matrix::new();
		let mut mat_fail: Matrix<2,1> = Matrix::new();
		let insert: Matrix<1,2> = Matrix::new();
		assert!(   mat_safe.insert(MatPos{row: 0, col: 0}, &insert).is_ok());
		assert_eq!(mat_fail.insert(MatPos{row: 0, col: 0}, &insert), Err(Errors::InvalidSize));
	}
	
	#[test]
	// If the matrix is too tall...
	fn test_insert_invalid_size_height ( )
	{
		let mut mat_safe: Matrix<2,2> = Matrix::new();
		let mut mat_fail: Matrix<1,2> = Matrix::new();
		let insert: Matrix<2,1> = Matrix::new();
		assert!(   mat_safe.insert(MatPos{row: 0, col: 0}, &insert).is_ok());
		assert_eq!(mat_fail.insert(MatPos{row: 0, col: 0}, &insert), Err(Errors::InvalidSize));
	}
	
	#[test]
	// If the matrix is too tall...
	fn test_insert_invalid_position_wide ( )
	{
		let mut mat_safe: Matrix<2,2> = Matrix::new();
		let mut mat_fail: Matrix<2,1> = Matrix::new();
		let insert: Matrix<1,1> = Matrix::new();
		assert!(   mat_safe.insert(MatPos{row: 0, col: 1}, &insert).is_ok());
		assert_eq!(mat_fail.insert(MatPos{row: 0, col: 1}, &insert), Err(Errors::InvalidSize));
	}
	
	#[test]
	// If the matrix is too tall...
	fn test_insert_invalid_position_high ( )
	{
		let mut mat_safe: Matrix<2,2> = Matrix::new();
		let mut mat_fail: Matrix<1,2> = Matrix::new();
		let insert: Matrix<1,1> = Matrix::new();
		assert!(   mat_safe.insert(MatPos{row: 1, col: 0}, &insert).is_ok());
		assert_eq!(mat_fail.insert(MatPos{row: 1, col: 0}, &insert), Err(Errors::InvalidSize));
	}



//###############################################################################################//
//
//									Conversions
//
// pub fn trace         ( &self )   -> Decimal
// pub fn from_decimal  ( Decimal ) -> Self
// pub fn to_decimal    ( &self )   -> Decimal
// pub fn determinate   ( &self )   -> Decimal    <1,1> <2,2> <3,3> <4,4>
// pub fn adjoint       ( &self )   -> Matrix     <3,3>
// pub fn to_vector3    ( &self )   -> Vector3    <3,1> <4,1>
// pub fn to_quaternion ( &self )   -> Quaternion <3,3>

//
//###############################################################################################//
//										~ trace ~											 	 //
	#[test]
	fn test_trace_empty ( )
	{
		let mat0x0: Matrix<0,0> = Matrix::new();
		assert_eq!(mat0x0.trace(), 0.0);
	}

	#[test]
	fn test_trace_normal ( )
	{
		let mut mat4x4: Matrix<4,4> = Matrix::new();
		for x in 0..mat4x4.width()
		{
			for y in 0..mat4x4.height()
			{
				mat4x4.set(MatPos{row: x, col: y}, (x+1) as Decimal * 10.0 +(y+1) as Decimal);
			}
		}
		assert_eq!(mat4x4.trace(), 11.0 + 22.0 + 33.0 + 44.0);
	}


//										~ from_decimal ~									 	 //
	#[test]
	fn test_from_decimal_1x1 ( )
	{
		let mat1x1: Matrix<1,1> = Matrix::from_decimal(100.0);
		assert_eq!(mat1x1.get(MatPos{row: 0, col: 0}), 100.0);
	}


//										~ to_decimal ~										 	 //
	#[test]
	fn test_to_decimal_1x1 ( )
	{
		let mut mat1x1: Matrix<1,1> = Matrix::new();
		mat1x1.set(MatPos{row: 0, col: 0}, 100.0);
		assert_eq!(mat1x1.to_decimal(), 100.0);
	}

//										~ determinate ~										 	 //
	#[test]
	fn test_determinate_1x1 ( )
	{
		let mut mat1x1: Matrix<1,1> = Matrix::new();
		mat1x1.set(MatPos{row: 0, col: 0}, 23.3);
		assert_eq!(mat1x1.determinate(), 23.3);
	}

	#[test]
	fn test_determinate_2x2 ( )
	{
		let mut mat2x2: Matrix<2,2> = Matrix::new();
		mat2x2.set(MatPos{row: 0, col: 0}, 2.0);
		mat2x2.set(MatPos{row: 1, col: 0}, 3.0);
		mat2x2.set(MatPos{row: 0, col: 1}, 4.0);
		mat2x2.set(MatPos{row: 1, col: 1}, 5.0);
		assert_eq!(mat2x2.determinate(), 2.0 * 5.0 - 3.0 * 4.0);
		assert_eq!(mat2x2.determinate(), -2.0);
	}

	#[test]
	fn test_determinate_3x3 ( )
	{
		let mut mat3x3: Matrix<3,3> = Matrix::new();
		mat3x3.set(MatPos{row: 0, col: 0}, 5.0);
		mat3x3.set(MatPos{row: 0, col: 1}, 4.0);
		mat3x3.set(MatPos{row: 0, col: 2}, 3.0);
		mat3x3.set(MatPos{row: 1, col: 0}, 8.0);
		mat3x3.set(MatPos{row: 1, col: 1}, 9.0);
		mat3x3.set(MatPos{row: 1, col: 2}, 10.0);
		mat3x3.set(MatPos{row: 2, col: 0}, 100.0);
		mat3x3.set(MatPos{row: 2, col: 1}, 2.0);
		mat3x3.set(MatPos{row: 2, col: 2}, 6.0);
		assert_eq!(mat3x3.determinate(), 1326.0);
	}


	#[test]
	fn test_determinate_4x4 ( )
	{
		let mut mat4x4: Matrix<4,4> = Matrix::new();

		mat4x4.set(MatPos{row: 0, col: 0}, 5.0);
		mat4x4.set(MatPos{row: 0, col: 1}, 10.0);
		mat4x4.set(MatPos{row: 0, col: 2}, 50.0);
		mat4x4.set(MatPos{row: 0, col: 3}, 2.0);

		mat4x4.set(MatPos{row: 1, col: 0}, 8.0);
		mat4x4.set(MatPos{row: 1, col: 1}, 20.0);
		mat4x4.set(MatPos{row: 1, col: 2}, 60.0);
		mat4x4.set(MatPos{row: 1, col: 3}, 11.0);

		mat4x4.set(MatPos{row: 2, col: 0}, 100.0);
		mat4x4.set(MatPos{row: 2, col: 1}, 30.0);
		mat4x4.set(MatPos{row: 2, col: 2}, 70.0);
		mat4x4.set(MatPos{row: 2, col: 3}, 103.0);

		mat4x4.set(MatPos{row: 3, col: 0}, 50.0);
		mat4x4.set(MatPos{row: 3, col: 1}, 40.0);
		mat4x4.set(MatPos{row: 3, col: 2}, 80.0);
		mat4x4.set(MatPos{row: 3, col: 3}, 53.0);

		assert_eq!(mat4x4.determinate(), 340800.0);
	}



//										~ adjoint ~											 	 //
	#[test]
	fn test_adjoint ( )
	{
		let mut mat3x3: Matrix<3,3> = Matrix::new();
		mat3x3.set(MatPos{row: 0, col: 0}, 5.0);
		mat3x3.set(MatPos{row: 0, col: 1}, 4.0);
		mat3x3.set(MatPos{row: 0, col: 2}, 3.0);
		mat3x3.set(MatPos{row: 1, col: 0}, 8.0);
		mat3x3.set(MatPos{row: 1, col: 1}, 9.0);
		mat3x3.set(MatPos{row: 1, col: 2}, 10.0);
		mat3x3.set(MatPos{row: 2, col: 0}, 100.0);
		mat3x3.set(MatPos{row: 2, col: 1}, 2.0);
		mat3x3.set(MatPos{row: 2, col: 2}, 6.0);

		let mut res: Matrix<3,3> = Matrix::new();
		res.set(MatPos{row: 0, col: 0}, 34.0);
		res.set(MatPos{row: 0, col: 1}, 952.0);
		res.set(MatPos{row: 0, col: 2}, -884.0);
		res.set(MatPos{row: 1, col: 0}, -18.0);
		res.set(MatPos{row: 1, col: 1}, -270.0);
		res.set(MatPos{row: 1, col: 2}, 390.0);
		res.set(MatPos{row: 2, col: 0}, 13.0);
		res.set(MatPos{row: 2, col: 1}, -26.0);
		res.set(MatPos{row: 2, col: 2}, 13.0);


		assert_eq!(res, mat3x3.adjoint());
	}




//										~ to_vector3 ~										 	 //
	#[test]
	fn test_to_vector3_3x1 ( )
	{
		let mut mat3x1: Matrix<3,1> = Matrix::new();
		mat3x1.set(MatPos{row: 0, col: 0}, 1.0);
		mat3x1.set(MatPos{row: 1, col: 0}, 2.0);
		mat3x1.set(MatPos{row: 2, col: 0}, 3.0);
		assert_eq!(mat3x1.to_vector3(), Vector3{x: 1.0, y: 2.0, z: 3.0});
	}



	#[test]
	fn test_to_vector3_4x1 ( )
	{
		let mut mat4x1: Matrix<4,1> = Matrix::new();
		mat4x1.set(MatPos{row: 0, col: 0}, 1.0);
		mat4x1.set(MatPos{row: 1, col: 0}, 2.0);
		mat4x1.set(MatPos{row: 2, col: 0}, 3.0);
		mat4x1.set(MatPos{row: 3, col: 0}, 0.5);
		assert_eq!(mat4x1.to_vector3(), Vector3{x: 2.0, y: 4.0, z: 6.0});
	}



/*




//										~ to_quaternion ~									 	 //
	#[test] // Case 1: trace > 0
	fn test_to_quaternion_case_1 ( )
	{
		let mut rng = rand::thread_rng();
		let pt = Vector3{x: 0.1, y: 0.2, z: 0.3};
		let mut rotation: Matrix<3,3> = Matrix::identity() * 11.0;

		for _i in 0..100
		{
			rotation.set(MatPos{row: 0, col: 1}, rng.gen::<Decimal>() * 10.0 - 5.0);
			rotation.set(MatPos{row: 0, col: 2}, rng.gen::<Decimal>() * 10.0 - 5.0);

			rotation.set(MatPos{row: 1, col: 0}, rng.gen::<Decimal>() * 10.0 - 5.0);
			rotation.set(MatPos{row: 1, col: 2}, rng.gen::<Decimal>() * 10.0 - 5.0);

			rotation.set(MatPos{row: 2, col: 0}, rng.gen::<Decimal>() * 10.0 - 5.0);
			rotation.set(MatPos{row: 2, col: 1}, rng.gen::<Decimal>() * 10.0 - 5.0);

			let q = rotation.to_quaternion();
			assert_eq!(q.rotate_point(pt), rotation.multiply(pt));
		}
	}

	#[test] // m00 >= m11 && m00 >= m22
	fn test_to_quaternion_case_2 ( )
	{
		let mut rng = rand::thread_rng();
		let pt = Vector3{x: 0.1, y: 0.2, z: 0.3};
		let mut rotation: Matrix<3,3> = Matrix::new();

		for _i in 0..100
		{
			rotation.set(MatPos{row: 0, col: 1}, rng.gen::<Decimal>() * 10.0 - 5.0);
			rotation.set(MatPos{row: 0, col: 2}, rng.gen::<Decimal>() * 10.0 - 5.0);

			rotation.set(MatPos{row: 1, col: 0}, rng.gen::<Decimal>() * 10.0 - 5.0);
			rotation.set(MatPos{row: 1, col: 1}, rng.gen::<Decimal>() * -5.0);
			rotation.set(MatPos{row: 1, col: 2}, rng.gen::<Decimal>() * 10.0 - 5.0);

			rotation.set(MatPos{row: 2, col: 0}, rng.gen::<Decimal>() * 10.0 - 5.0);
			rotation.set(MatPos{row: 2, col: 1}, rng.gen::<Decimal>() * 10.0 - 5.0);
			rotation.set(MatPos{row: 2, col: 2}, rng.gen::<Decimal>() * -5.0);

			let m11 = rotation.get(MatPos{row: 1, col: 1});
			let m22 = rotation.get(MatPos{row: 2, col: 2});
			rotation.set(MatPos{row: 0, col: 0}, (m11.abs() + m22.abs()) / 2.0);

			let q = rotation.to_quaternion();
			assert_eq!(q.rotate_point(pt), rotation.multiply(pt));
		}
	}

	#[test] // m11 > m22 && m11 > m00
	fn test_to_quaternion_case_3 ( )
	{
		let mut rng = rand::thread_rng();
		let pt = Vector3{x: 0.1, y: 0.2, z: 0.3};
		let mut rotation: Matrix<3,3> = Matrix::new();

		for _i in 0..100
		{
			rotation.set(MatPos{row: 0, col: 0}, rng.gen::<Decimal>() * -5.0);
			rotation.set(MatPos{row: 0, col: 1}, rng.gen::<Decimal>() * 10.0 - 5.0);
			rotation.set(MatPos{row: 0, col: 2}, rng.gen::<Decimal>() * 10.0 - 5.0);

			rotation.set(MatPos{row: 1, col: 0}, rng.gen::<Decimal>() * 10.0 - 5.0);
			rotation.set(MatPos{row: 1, col: 2}, rng.gen::<Decimal>() * 10.0 - 5.0);

			rotation.set(MatPos{row: 2, col: 0}, rng.gen::<Decimal>() * 10.0 - 5.0);
			rotation.set(MatPos{row: 2, col: 1}, rng.gen::<Decimal>() * 10.0 - 5.0);
			rotation.set(MatPos{row: 2, col: 2}, rng.gen::<Decimal>() * -5.0);

			let m00 = rotation.get(MatPos{row: 0, col: 0});
			let m22 = rotation.get(MatPos{row: 2, col: 2});
			rotation.set(MatPos{row: 1, col: 1}, (m00.abs() + m22.abs()) / 2.0);

			let q = rotation.to_quaternion();
			assert_eq!(q.rotate_point(pt), rotation.multiply(pt));
		}
	}

	#[test] // m22 > m11 && m22 > m00
	fn test_to_quaternion_case_4 ( )
	{
		let mut rng = rand::thread_rng();
		let pt = Vector3{x: 0.1, y: 0.2, z: 0.3};
		let mut rotation: Matrix<3,3> = Matrix::new();

		for _i in 0..100
		{
			rotation.set(MatPos{row: 0, col: 0}, rng.gen::<Decimal>() * -5.0);
			rotation.set(MatPos{row: 0, col: 1}, rng.gen::<Decimal>() * 10.0 -5.0);
			rotation.set(MatPos{row: 0, col: 2}, rng.gen::<Decimal>() * 10.0 - 5.0);

			rotation.set(MatPos{row: 1, col: 0}, rng.gen::<Decimal>() * 10.0 - 5.0);
			rotation.set(MatPos{row: 1, col: 1}, rng.gen::<Decimal>() * -5.0);
			rotation.set(MatPos{row: 1, col: 2}, rng.gen::<Decimal>() * 10.0 - 5.0);

			rotation.set(MatPos{row: 2, col: 0}, rng.gen::<Decimal>() * 10.0 - 5.0);
			rotation.set(MatPos{row: 2, col: 2}, rng.gen::<Decimal>() * 10.0 - 5.0);

			let m00 = rotation.get(MatPos{row: 0, col: 0});
			let m11 = rotation.get(MatPos{row: 1, col: 1});
			rotation.set(MatPos{row: 2, col: 2}, (m00.abs() + m11.abs()) / 2.0);

			let q = rotation.to_quaternion();
			assert_eq!(q.rotate_point(pt), rotation.multiply(pt));
		}
	}
*/




//###############################################################################################//
//
//									Operatiors
//
// pub fn multiply      ( &self, Vector3 ) -> Vector3  <3,3> <3,4> <4,4>
// pub Mat * Mat -> Mat
// pub Mat + Mat -> Mat
// pub Mat * Decimal -> Mat
// pub Mat / Decimal -> Mat
// pub Mat == Mat
// pub Mat != Mat
//
//###############################################################################################//
//										~ multiply ~										 	 //
	#[test]
	fn test_multiply_3x3 ( )
	{
		let pt = Vector3{x: 1.0, y: 2.0, z: 3.0};
		let mut mat3x3: Matrix<3,3> = Matrix::new();

		// x is the addition of all.
		mat3x3.set(MatPos{row: 0, col: 0}, 1.0);
		mat3x3.set(MatPos{row: 0, col: 1}, 1.0);
		mat3x3.set(MatPos{row: 0, col: 2}, 1.0);

		// y is 2y + z.
		mat3x3.set(MatPos{row: 1, col: 0}, 0.0);
		mat3x3.set(MatPos{row: 1, col: 1}, 2.0);
		mat3x3.set(MatPos{row: 1, col: 2}, 1.0);

		// z is 0.5x + 3.0y + 2.0z.
		mat3x3.set(MatPos{row: 2, col: 0}, 0.5);
		mat3x3.set(MatPos{row: 2, col: 1}, 3.0);
		mat3x3.set(MatPos{row: 2, col: 2}, 2.0);

		let expected = Vector3
		{
			x: pt.x + pt.y + pt.z,
			y: 2.0 * pt.y + pt.z,
			z: 0.5 * pt.x + 3.0 * pt.y + 2.0 * pt.z,
		};
		assert_eq!(mat3x3.multiply(pt), expected);
	}

	#[test]
	fn test_multiply_3x4 ( )
	{
		let pt = Vector3{x: 1.0, y: 2.0, z: 3.0};
		let mut mat3x4: Matrix<3,4> = Matrix::new();

		// x is the addition of all + 1.
		mat3x4.set(MatPos{row: 0, col: 0}, 1.0);
		mat3x4.set(MatPos{row: 0, col: 1}, 1.0);
		mat3x4.set(MatPos{row: 0, col: 2}, 1.0);
		mat3x4.set(MatPos{row: 0, col: 3}, 1.0);

		// y is 2y + z.
		mat3x4.set(MatPos{row: 1, col: 0}, 0.0);
		mat3x4.set(MatPos{row: 1, col: 1}, 2.0);
		mat3x4.set(MatPos{row: 1, col: 2}, 1.0);
		mat3x4.set(MatPos{row: 1, col: 3}, 0.0);

		// z is 0.5x + 3.0y + 2.0z + 1.
		mat3x4.set(MatPos{row: 2, col: 0}, 0.5);
		mat3x4.set(MatPos{row: 2, col: 1}, 3.0);
		mat3x4.set(MatPos{row: 2, col: 2}, 2.0);
		mat3x4.set(MatPos{row: 2, col: 3}, 1.0);

		let expected = Vector3
		{
			x: pt.x + pt.y + pt.z + 1.0,
			y: 2.0 * pt.y + pt.z,
			z: 0.5 * pt.x + 3.0 * pt.y + 2.0 * pt.z + 1.0,
		};
		assert_eq!(mat3x4.multiply(pt), expected);
	}

	#[test]
	fn test_multiply_4x4 ( )
	{
		let pt = Vector3{x: 1.0, y: 2.0, z: 3.0};
		let mut mat4x4: Matrix<4,4> = Matrix::new();

		// x is the addition of all.
		mat4x4.set(MatPos{row: 0, col: 0}, 1.0);
		mat4x4.set(MatPos{row: 0, col: 1}, 1.0);
		mat4x4.set(MatPos{row: 0, col: 2}, 1.0);
		mat4x4.set(MatPos{row: 0, col: 3}, 0.0);

		// y is 2y + z + 1.
		mat4x4.set(MatPos{row: 1, col: 0}, 0.0);
		mat4x4.set(MatPos{row: 1, col: 1}, 2.0);
		mat4x4.set(MatPos{row: 1, col: 2}, 1.0);
		mat4x4.set(MatPos{row: 1, col: 3}, 1.0);

		// z is 0.5x + 3.0y + 2.0z.
		mat4x4.set(MatPos{row: 2, col: 0}, 0.5);
		mat4x4.set(MatPos{row: 2, col: 1}, 3.0);
		mat4x4.set(MatPos{row: 2, col: 2}, 2.0);
		mat4x4.set(MatPos{row: 2, col: 3}, 0.0);

		// w is 2x + 2y + 2z + 2.
		mat4x4.set(MatPos{row: 3, col: 0}, 2.0);
		mat4x4.set(MatPos{row: 3, col: 1}, 2.0);
		mat4x4.set(MatPos{row: 3, col: 2}, 2.0);
		mat4x4.set(MatPos{row: 3, col: 3}, 2.0);

		let w = 2.0 * (pt.x + pt.y + pt.z) + 2.0;
		let expected = Vector3
		{
			x: pt.x + pt.y + pt.z,
			y: 2.0 * pt.y + pt.z + 1.0,
			z: 0.5 * pt.x + 3.0 * pt.y + 2.0 * pt.z,
		} / w;
		assert_eq!(mat4x4.multiply(pt), expected);
	}


//										~ Mut(ltiply) ~										 	 //
	#[test]
	fn test_mul_mat_zero ( )
	{
		let mat0x0: Matrix<0,0> = Matrix::new();
		assert_eq!(mat0x0 * mat0x0, mat0x0);
	}

	#[test]
	fn test_mul_mat_square ( )
	{
		let mut mat3x3_1: Matrix<3,3> = Matrix::new();
		let mut mat3x3_2: Matrix<3,3> = Matrix::new();

		for x in 0..3
		{
			for y in 0..3
			{
				mat3x3_1.set(MatPos{col: x, row: y}, (x + y * 3) as Decimal);
				mat3x3_2.set(MatPos{col: x, row: y}, (x + y * 3 + 9) as Decimal);
			}
		}
		let output: Matrix<3,3> = mat3x3_1 * mat3x3_2;

		assert_eq!(output.get(MatPos{col:0, row:0}), 42.0);
		assert_eq!(output.get(MatPos{col:1, row:0}), 45.0);
		assert_eq!(output.get(MatPos{col:2, row:0}), 48.0);
		assert_eq!(output.get(MatPos{col:0, row:1}), 150.0);
		assert_eq!(output.get(MatPos{col:1, row:1}), 162.0);
		assert_eq!(output.get(MatPos{col:2, row:1}), 174.0);
		assert_eq!(output.get(MatPos{col:0, row:2}), 258.0);
		assert_eq!(output.get(MatPos{col:1, row:2}), 279.0);
		assert_eq!(output.get(MatPos{col:2, row:2}), 300.0);
	}



	#[test]
	fn test_mul_mat_odd ( )
	{
		let mut mat1x3: Matrix<1,3> = Matrix::new();
		let mut mat3x1: Matrix<3,1> = Matrix::new();
		let mut mat3x2: Matrix<3,2> = Matrix::new();

		for ii in 0..3
		{
			mat1x3.set(MatPos{row: 0,  col: ii}, ii as Decimal);
			mat3x1.set(MatPos{row: ii, col: 0},  ii as Decimal + 3.0);

			for jj in 0..2
			{
				mat3x2.set(MatPos{row: ii, col: jj}, (ii * 2 + jj + 3) as Decimal);
			}
		}
		let output_1: Matrix<1,1> = mat1x3 * mat3x1;
		let output_2: Matrix<1,2> = mat1x3 * mat3x2;
		let output_3: Matrix<3,3> = mat3x1 * mat1x3;

		assert_eq!(output_1.get(MatPos{col:0, row:0}), 14.0);

		assert_eq!(output_2.get(MatPos{col:0, row:0}), 19.0);
		assert_eq!(output_2.get(MatPos{col:1, row:0}), 22.0);

		assert_eq!(output_3.get(MatPos{col:0, row:0}), 0.0);
		assert_eq!(output_3.get(MatPos{col:1, row:0}), 3.0);
		assert_eq!(output_3.get(MatPos{col:2, row:0}), 6.0);
		assert_eq!(output_3.get(MatPos{col:0, row:1}), 0.0);
		assert_eq!(output_3.get(MatPos{col:1, row:1}), 4.0);
		assert_eq!(output_3.get(MatPos{col:2, row:1}), 8.0);
		assert_eq!(output_3.get(MatPos{col:0, row:2}), 0.0);
		assert_eq!(output_3.get(MatPos{col:1, row:2}), 5.0);
		assert_eq!(output_3.get(MatPos{col:2, row:2}), 10.0);
	}



//										~ Add ~												 	 //
	#[test]
	fn test_add_mat ( )
	{
		let mut mat_a: Matrix<3, 2> = Matrix::new();
		let mut mat_b: Matrix<3, 2> = Matrix::new();

		mat_a.set(MatPos{row: 0, col: 0}, 1.0);
		mat_a.set(MatPos{row: 1, col: 0}, 2.0);
		mat_a.set(MatPos{row: 2, col: 0}, 3.0);
		mat_a.set(MatPos{row: 0, col: 1}, 4.0);
		mat_a.set(MatPos{row: 1, col: 1}, 5.0);
		mat_a.set(MatPos{row: 2, col: 1}, 6.0);

		mat_b.set(MatPos{row: 0, col: 0}, 0.1);
		mat_b.set(MatPos{row: 1, col: 0}, 0.2);
		mat_b.set(MatPos{row: 2, col: 0}, 0.3);
		mat_b.set(MatPos{row: 0, col: 1}, 0.4);
		mat_b.set(MatPos{row: 1, col: 1}, 0.5);
		mat_b.set(MatPos{row: 2, col: 1}, 0.6);

		let mat_c = mat_a + mat_b;
		assert_eq!(mat_c.get(MatPos{row: 0, col: 0}), 1.1);
		assert_eq!(mat_c.get(MatPos{row: 1, col: 0}), 2.2);
		assert_eq!(mat_c.get(MatPos{row: 2, col: 0}), 3.3);
		assert_eq!(mat_c.get(MatPos{row: 0, col: 1}), 4.4);
		assert_eq!(mat_c.get(MatPos{row: 1, col: 1}), 5.5);
		assert_eq!(mat_c.get(MatPos{row: 2, col: 1}), 6.6);
	}




//										~ Mut(iply) ~										 	 //
	#[test]
	fn test_mul_scalar ( )
	{
		let mut mat3x4: Matrix<3,4> = Matrix::new();
		mat3x4.matrix[2][1] = 12.0;
		mat3x4.matrix[2][3] = 23.0;
		mat3x4 = mat3x4 * 2.0;
		assert_eq!(mat3x4.matrix[2][1], 24.0);
		assert_eq!(mat3x4.matrix[2][3], 46.0);
		mat3x4 = 0.5 * mat3x4;
		assert_eq!(mat3x4.matrix[2][1], 12.0);
		assert_eq!(mat3x4.matrix[2][3], 23.0);
	}

//										~ Div ~												 	 //
	#[test]
	fn test_div_scalar ( )
	{
		let mut mat3x4: Matrix<3,4> = Matrix::new();
		mat3x4.matrix[2][1] = 24.0;
		mat3x4.matrix[2][3] = 46.0;
		mat3x4 = mat3x4 / 2.0;
		assert_eq!(mat3x4.matrix[2][1], 12.0);
		assert_eq!(mat3x4.matrix[2][3], 23.0);
	}








//										~ Eq(uality) ~										 	 //
	#[test]
	fn test_eq_matrix ( )
	{
		let mut mat3x4   : Matrix<3,4> = Matrix::new();
		let mut mat_other: Matrix<3,4> = Matrix::new();
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
	
//###############################################################################################//
//
//										Debug
// Display: Show neat (3dp)
// Debug: Show everything (all dp)
// 
//###############################################################################################//
//								- Display / Debug fmt -											 //
	#[test]
	fn test_display ( )
	{
		let mut mat   : Matrix<2,3> = Matrix::new();
		mat.matrix[0][0] = 0.00000;
		mat.matrix[1][0] = 1.00000;
		mat.matrix[0][1] = 0.11111;
		mat.matrix[1][1] = 1.11111;
		mat.matrix[0][2] = 0.22222;
		mat.matrix[1][2] = 1.22222;
		assert_eq!(format!("{}", mat), 
			"\n|\t0.000,\t0.111,\t0.222\t|\n|\t1.000,\t1.111,\t1.222\t|");
	}
	
	#[test]
	fn test_debug ( )
	{
		let mut mat   : Matrix<2,3> = Matrix::new();
		mat.matrix[0][0] = 0.00000;
		mat.matrix[1][0] = 1.00000;
		mat.matrix[0][1] = 0.11111;
		mat.matrix[1][1] = 1.11111;
		mat.matrix[0][2] = 0.22222;
		mat.matrix[1][2] = 1.22222;
		assert_eq!(format!("{:?}", mat), 
			"\n|\t0,\t0.11111,\t0.22222\t|\n|\t1,\t1.11111,\t1.22222\t|");
	}
	
}
