//! Implementation of Mul, Add, Sub, Div and Display
use crate::core_include::*;

use core::ops::Mul;
use core::ops::Add;
use core::ops::Sub;
use core::ops::Div;
use core::ops::Neg;
use core::ops::BitAnd;
use core::ops::BitOr;
use core::ops::BitOrAssign;
use core::ops::BitAndAssign;
use core::fmt;

use crate::util::units::Degrees;
use crate::util::units::Radians;
use crate::util::units::Hours;

use crate::util::units::Quaternion;
use crate::util::units::Equatorial;
use crate::util::units::AngleAxis;
use crate::util::units::BitField;
use crate::util::units::Vector3;
use crate::util::units::Vector2;
use crate::util::units::MatPos;
use crate::util::units::Pixel;

use crate::util::units::Matrix;

use crate::util::aliases::Decimal;
use crate::util::aliases::DECIMAL_PRECISION;

use crate::util::Maths;

//###############################################################################################//
//										---	Decimal ---
//###############################################################################################//


//###############################################################################################//
//										---	Degrees ---
//###############################################################################################//
impl Degrees
{
	pub fn abs ( &self ) -> Decimal { return self.0.abs(); }
}

impl Mul<Decimal> for Degrees {
	type Output = Self;
	fn mul(self, rhs: Decimal) -> Self { return Degrees(self.0 * rhs); } }

impl Mul<Degrees> for Degrees {
	type Output = Self;
	fn mul(self, rhs: Degrees) -> Self { return Degrees(self.0 * rhs.0); } }

impl Add for Degrees {
	type Output = Self;
	fn add ( self, rhs: Degrees ) -> Self { return Degrees(self.0 + rhs.0); } }

impl Sub for Degrees {
	type Output = Self;
	fn sub ( self, rhs: Degrees ) -> Self { return Degrees(self.0 - rhs.0); } }

impl Div<Decimal> for Degrees {
	type Output = Self;
	fn div ( self, rhs: Decimal ) -> Self { return Degrees(self.0 / rhs); } }


impl Div<Degrees> for Degrees {
	type Output = Self;
	fn div ( self, rhs: Degrees ) -> Self { return Degrees(self.0 / rhs.0); } }

impl Neg for Degrees {
	type Output = Self;
	fn neg ( self ) -> Self::Output { return Self(-self.0); }
}


impl PartialEq for Degrees {
	fn eq ( &self, other: &Self ) -> bool { return (self.0 - other.0).abs() < DECIMAL_PRECISION; }
}



impl fmt::Display for Degrees {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{:.3}d", self.0) } }





//###############################################################################################//
//										---	Radians ---
//###############################################################################################//

impl Radians
{
	pub fn abs ( &self ) -> Decimal { return self.0.abs(); }
}

impl Mul<Decimal> for Radians {
	type Output = Self;
	fn mul ( self, rhs: Decimal ) -> Self { return Radians(self.0 * rhs); } }

impl Mul<Radians> for Radians {
	type Output = Self;
	fn mul ( self, rhs: Radians ) -> Self { return Radians(self.0 * rhs.0); } }

impl Add for Radians {
	type Output = Self;
	fn add ( self, rhs: Radians ) -> Self { return Radians(self.0 + rhs.0); } }

impl Sub for Radians {
	type Output = Self;
	fn sub ( self, rhs: Radians ) -> Self { return Radians(self.0 - rhs.0); } }

impl Div<Decimal> for Radians {
	type Output = Self;
	fn div ( self, rhs: Decimal ) -> Self { return Radians(self.0 / rhs); } }


impl Div<Radians> for Radians {
	type Output = Self;
	fn div ( self, rhs: Radians ) -> Self { return Radians(self.0 / rhs.0); } }


impl Neg for Radians {
	type Output = Self;
	fn neg ( self ) -> Self::Output { return Self(-self.0); }
}

impl PartialEq for Radians {
	fn eq ( &self, other: &Self ) -> bool { return (self.0 - other.0).abs() < DECIMAL_PRECISION; }
}



impl fmt::Display for Radians {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{:.3}r", self.0) } }





//###############################################################################################//
//									--- Hours ---
//###############################################################################################//

impl Hours
{
	pub fn abs ( &self ) -> Decimal { return self.0.abs(); }
}

impl PartialEq for Hours {
	fn eq ( &self, other: &Self ) -> bool { return (self.0 - other.0).abs() < DECIMAL_PRECISION; }
}

impl Neg for Hours {
	type Output = Self;
	fn neg ( self ) -> Self::Output { return Self(-self.0); }
}


impl fmt::Display for Hours {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{:.3}h", self.0) } }





//###############################################################################################//
//									--- Equatiorial ---
//###############################################################################################//

impl PartialEq for Equatorial {
	fn eq ( &self, other: &Self ) -> bool
	{return (self.ra.0- other.ra.0).abs() < DECIMAL_PRECISION &&
		(self.dec.0 - other.dec.0).abs() < DECIMAL_PRECISION;}
}


impl fmt::Display for Equatorial {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "Equatorial(ra: {}, dec: {})", self.ra.to_degrees(), self.dec.to_degrees())
			.expect("Format Error");
		return Result::Ok(());
	}
}


impl fmt::Debug for Equatorial {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "Equatorial(ra: {:?}, dec: {:?})", self.ra, self.dec).expect("Format Error");
		return Result::Ok(());
	}
}






//###############################################################################################//
//									--- AngleAxis ---
//###############################################################################################//

impl PartialEq for AngleAxis {
	fn eq ( &self, other: &Self ) -> bool
	{
		return self.angle == other.angle && self.axis == other.axis;
	}
}


impl fmt::Display for AngleAxis {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "AngleAxis({},\t{})", self.angle.to_degrees(), self.axis)
			.expect("Format Error");
		return Result::Ok(());
	}
}


impl fmt::Debug for AngleAxis {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "AngleAxis({:?},\t{:?})", self.angle, self.axis)
			.expect("Format Error");
		return Result::Ok(());
	}
}






//###############################################################################################//
//									--- Quaternion ---
//###############################################################################################//



impl Mul for Quaternion
{
	type Output = Self;
	/// Finds the Hamilton Product.
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Quaternion;
	/// let q = Quaternion{w: 1.0, x: 2.0, y: 3.0, z: 4.0};
	/// let r = Quaternion{w: 4.0, x: 3.0, y: 2.0, z: 1.0};
	/// assert_eq!(q * r, Quaternion{w: -12.0, x: 6.0, y: 24.0, z: 12.0});
	/// ```
	fn mul ( self, rhs: Self ) -> Self
	{
		let x = self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y;
		let y = self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x;
		let z = self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w;
		let w = self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z;
		return Quaternion{w: w , x: x, y: y, z: z};
	}
}


impl PartialEq for Quaternion {
	 fn eq ( &self, other: &Self ) -> bool
	 {
		 return
		 	(self.w - other.w).abs() < DECIMAL_PRECISION &&
			(self.x - other.x).abs() < DECIMAL_PRECISION &&
			(self.y - other.y).abs() < DECIMAL_PRECISION &&
			(self.z - other.z).abs() < DECIMAL_PRECISION;
	}
}







//###############################################################################################//
//							---	Matrix ---
//###############################################################################################//
// Multiply Matrix by Scalar

impl <const ROW: usize, const COLUMN: usize> Mul<Decimal> for Matrix<ROW, COLUMN> {
	type Output = Self;
	fn mul(self, rhs: Decimal) -> Self
	{
		let mut mat: Matrix<ROW, COLUMN> = Matrix::new();
		for xx in 0..COLUMN
		{
			for yy in 0..ROW
			{
				let pos = MatPos{col: xx, row: yy};
				mat.set(pos, self.get(pos) * rhs);
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
		let mut mat: Matrix<ROW, COLUMN> = Matrix::new();
		for xx in 0..COLUMN
		{
			for yy in 0..ROW
			{
				let pos = MatPos{col: xx, row: yy};
				mat.set(pos, self.get(pos) / rhs);
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
				eq &= (self.get(pos) - other.get(pos)).abs() < 0.00001;
			}
		}

		return eq;
	}
}

// Multiply Matrix by Matrix
impl <const M: usize, const N: usize, const P: usize> Mul<Matrix<N, P>> for Matrix<M, N>
{
	// (M, N) x (N, P) = (M, P)
	type Output = Matrix<M, P>;
    fn mul(self, rhs: Matrix<N, P>) -> Matrix<M, P>
	{
		let mut mat: Matrix<M, P> = Matrix::new();
		for y in 0..M				// OUT matrix row
		{
			for x in 0..P			// OUT matrix column
			{
				let mut output: Decimal = 0.0;
				for i in 0..N		// IN matrix (lhs x, rhs y)
				{
					output +=
						self.get(MatPos{row: y, col: i}) *
						 rhs.get(MatPos{row: i, col: x});
				}
				mat.set(MatPos{row: y, col: x}, output);
			}
		}
		return mat;
	}
}


impl <const ROW: usize, const COLUMN: usize> Add<Matrix<ROW, COLUMN>> for Matrix<ROW, COLUMN>
{
	type Output = Matrix<ROW, COLUMN>;
	fn add ( self, rhs: Matrix<ROW, COLUMN> ) -> Matrix<ROW, COLUMN>
	{
		let mut mat: Matrix<ROW, COLUMN> = Matrix::new();
		for row in 0..ROW
		{
			for col in 0..COLUMN
			{
				let val = self.get(MatPos{row: row, col: col}) + rhs.get(MatPos{row:row, col:col});
				mat.set(MatPos{row: row, col: col}, val);
			}
		}
		return mat;
	}

}



impl <const ROW: usize, const COLUMN: usize> fmt::Display for Matrix<ROW, COLUMN> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "\n")?;
		for yy in 0..self.height()
		{
			write!(f, "|\t")?;
			for xx in 0..self.width()
			{
				let val = self.get(MatPos{col: xx, row: yy});
				write!(f, "{:.3}", val)?;
				if xx < self.width() - 1
				{
					write!(f, ",\t")?;
				}
			}
			write!(f, "\t|")?;
			if yy < self.height() - 1
			{
				write!(f, "\n")?;
			}
		}
		return Result::Ok(());
	}
}


impl <const ROW: usize, const COLUMN: usize> fmt::Debug for Matrix<ROW, COLUMN> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "\n")?;
		for yy in 0..self.height()
		{
			write!(f, "|\t")?;
			for xx in 0..self.width()
			{
				let val = self.get(MatPos{col: xx, row: yy});
				write!(f, "{}", val)?;
				if xx < self.width() - 1
				{
					write!(f, ",\t")?;
				}
			}
			write!(f, "\t|")?;
			if yy < self.height() - 1
			{
				write!(f, "\n")?;
			}
		}
		return Result::Ok(());
	}
}




//###############################################################################################//
//									--- Pixel ---
//###############################################################################################//
impl Pixel 
{
	/// Converts the pixel into a Vector2 by converting the integers into a floating point.
	pub fn to_vector2 ( &self ) -> Vector2
	{
		return Vector2{x: self.x as Decimal, y: self.y as Decimal};
	}
}




//###############################################################################################//
//									--- Vector2 ---
//###############################################################################################//

impl Mul<Decimal> for Vector2 {
	type Output = Self;
	fn mul ( self, rhs: Decimal ) -> Self	{ return Self{x: self.x * rhs, y: self.y * rhs} } }

impl Add<Vector2> for Vector2 {
	type Output = Self;
	fn add ( self, rhs: Self ) -> Self		{ return Self{x: self.x + rhs.x, y: self.y + rhs.y} } }


impl Sub for Vector2 {
	type Output = Self;
	fn sub ( self, rhs: Vector2 ) -> Self	{ return Self{x: self.x - rhs.x, y: self.y - rhs.y} } }

impl Div<Decimal> for Vector2 {
	type Output = Self;
	fn div ( self, rhs: Decimal ) -> Self	{ return Self{x: self.x / rhs, y: self.y / rhs} } }


impl Neg for Vector2 {
	type Output = Self;
	fn neg ( self ) -> Self::Output { return Self{x: -self.x, y: -self.y}; }
}

impl PartialEq for Vector2 {
	fn eq ( &self, other: &Self ) -> bool
	{
		 return
		 	(self.x - other.x).abs() < DECIMAL_PRECISION &&
			(self.y - other.y).abs() < DECIMAL_PRECISION;
	}
}




impl fmt::Display for Vector2 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "Vector2({:.3}, {:.3})", self.x, self.y).expect("Invalid Formatting");
		return Result::Ok(());
	}
}


impl fmt::Debug for Vector2 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "Vector2(x: {}, y: {})", self.x, self.y).expect("Invalid Formatting");
		return Result::Ok(());
	}
}








//###############################################################################################//
//									--- Vector3 ---
//###############################################################################################//

impl Mul<Decimal> for Vector3 {
	type Output = Self;
	fn mul ( self, rhs: Decimal ) -> Self {
		return Vector3{x: self.x * rhs, y: self.y * rhs, z: self.z * rhs} } }

impl Add<Vector3> for Vector3 {
	type Output = Self;
	fn add ( self, rhs: Vector3 ) -> Self {
		return Vector3{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z} } }

impl Sub for Vector3 {
	type Output = Self;
	fn sub ( self, rhs: Vector3 ) -> Self {
		return Vector3{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z} } }

impl Div<Decimal> for Vector3 {
	type Output = Self;
	fn div ( self, rhs: Decimal ) -> Self {
		return Vector3{x: self.x / rhs, y: self.y / rhs, z: self.z / rhs} } }

impl Neg for Vector3 {
	type Output = Self;
	fn neg ( self ) -> Self::Output { return Self{x: -self.x, y: -self.y, z: -self.z}; }
}

impl PartialEq for Vector3 {
	 fn eq ( &self, other: &Self ) -> bool
	 {
		 return
		 	(self.x - other.x).abs() < DECIMAL_PRECISION &&
			(self.y - other.y).abs() < DECIMAL_PRECISION &&
			(self.z - other.z).abs() < DECIMAL_PRECISION;
	}
}

impl fmt::Display for Vector3 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "Vector3({:.3}, {:.3}, {:.3})", self.x, self.y, self.z)
			.expect("Invalid formatting");
		return Result::Ok(());
	}
}


impl fmt::Debug for Vector3 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "Vector3(x: {}, y: {}, z: {})", self.x, self.y, self.z)
			.expect("Invalid formatting");
		return Result::Ok(());
	}
}





//###############################################################################################//
//									--- Regions ---
//###############################################################################################//

impl BitAnd<BitField> for BitField
{
	type Output = Self;
	fn bitand ( self, rhs: BitField ) -> BitField {
		return BitField(self.0 & rhs.0); }}


impl BitOr<BitField> for BitField
{
	type Output = Self;
	fn bitor ( self, rhs: BitField ) -> BitField {
		return BitField(self.0 | rhs.0); }}


impl BitAndAssign<BitField> for BitField
{
	fn bitand_assign ( &mut self, rhs: BitField ) { *self = *self & rhs; }}


impl BitOrAssign<BitField> for BitField
{
	fn bitor_assign ( &mut self, rhs: BitField ) { *self = *self | rhs; }}




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
	use crate::util::units::*;
	use crate::util::aliases::DECIMAL_PRECISION;
	use crate::util::aliases::Decimal;
	use crate::util::test::TestEqual;
	
//###############################################################################################//
//
//										Decimal
//
// pub fn test_close ( &self, other: &Self, precision: Decimal ) -> bool
//
//###############################################################################################//
//										~ test_close ~											 //
	
	#[test]
	// When the value is within the bounds, it is valid.
	fn test_test_close_inside_bounds ( )
	{
		assert!((0 as Decimal).test_close(&0.0, 1.0));
		assert!((0 as Decimal).test_close(&0.5, 1.0));
		assert!((0 as Decimal).test_close(&0.9999, 1.0));
	}
	
	#[test]
	// When the value is ouside the bounds, it is invalid.
	fn test_test_close_outside_bounds ( )
	{
		assert!(!(0 as Decimal).test_close(&-1.0001, 1.0));
		assert!(!(0 as Decimal).test_close(& 1.0001, 1.0));
	}
	
	
	
	
//###############################################################################################//
//
//										Radians
//
// pub fn abs ( &self ) -> Decimal
//
// Tested Together ops
// pub fn mul ( self, Decimal ) -> Self
// pub fn mul ( self, Degrees ) -> Self
// pub fn add ( self, Degres ) -> Self
// pub fn sub ( self, Degrees ) -> Self
// pub fn div ( self, Decimal ) -> Self
// pub fn div ( self, Degrees ) -> Self
// pub fn neg ( self ) -> Self
//
// pub fn eq  ( &self, &Self ) -> bool
// pub fn test_close ( &self, &Self, Decimal ) -> bool
//
//###############################################################################################//
//										~ abs ~													 //
	#[test]
	fn test_radians_abs ( )
	{
		let angle = Radians(1.23);
		assert_eq!(angle.abs(), 1.23);
		assert_eq!((-angle).abs(), 1.23);
	}
	
//										~ ops ~													 //
	#[test]
	fn test_radians_ops ( )
	{
		let angle = Radians(1.2345);
		angle.sin().test_equal(&0.9439833239445111);  // !
		angle.cos().test_equal(&0.32999315767856785); // !
		
		assert_eq!(angle + Radians(1.0), Radians(2.2345));
		assert_eq!(angle - Radians(1.0), Radians(0.2345));
		
		assert_eq!(angle * 2.0,          Radians(2.469));
		assert_eq!(angle * Radians(2.0), Radians(2.469));
		
		assert_eq!(angle / 2.0,          Radians(0.61725));
		assert_eq!(angle / Radians(2.0), Radians(0.61725));
		
		assert_eq!(-angle,               Radians(-1.2345));
		assert_eq!((-angle).abs(),       1.2345);
	}
	
//										~ eq ~													 //
	#[test]
	fn test_radians_eq ( )
	{
		let angle = Radians(1.2345);
		assert_eq!(angle, Radians(1.2345 + DECIMAL_PRECISION / 2.0));
	}
	
	
	
	
//###############################################################################################//
//
//										Degree
//
// pub fn abs ( &self ) -> Decimal
//
// Tested Together ops
// pub fn mul ( self, Decimal ) -> Self
// pub fn mul ( self, Degrees ) -> Self
// pub fn add ( self, Degres ) -> Self
// pub fn sub ( self, Degrees ) -> Self
// pub fn div ( self, Decimal ) -> Self
// pub fn div ( self, Degrees ) -> Self
// pub fn neg ( self ) -> Self
//
// Tested Together eq
// pub fn eq  ( &self, &Self ) -> bool
// pub fn test_close ( &self, &Self, Decimal ) -> bool
//
//###############################################################################################//
//										~ abs ~													 //
	
	#[test]
	fn test_degrees_abs ( )
	{
		let angle = Degrees(1.23);
		assert_eq!(angle.abs(), 1.23);
		assert_eq!((-angle).abs(), 1.23);
	}
	
//										~ ops ~													 //
	#[test]
	fn test_degrees_ops ( )
	{
		let angle = Degrees(1.2345);
		angle.sin().test_equal(&0.9439833239445111);  // !
		angle.cos().test_equal(&0.32999315767856785); // !
		
		assert_eq!(angle + Degrees(1.0), Degrees(2.2345));
		assert_eq!(angle - Degrees(1.0), Degrees(0.2345));
		
		assert_eq!(angle * 2.0,          Degrees(2.469));
		assert_eq!(angle * Degrees(2.0), Degrees(2.469));
		
		assert_eq!(angle / 2.0,          Degrees(0.61725));
		assert_eq!(angle / Degrees(2.0), Degrees(0.61725));
		
		assert_eq!(-angle,               Degrees(-1.2345));
		assert_eq!((-angle).abs(),       1.2345);
	}
	
//										~ eq ~													 //
	#[test]
	fn test_degrees_eq ( )
	{
		let angle = Degrees(1.2345);
		assert_eq!(angle, Degrees(1.2345 + DECIMAL_PRECISION / 2.0));
		assert!(angle.test_close(&Degrees(2.2344), 1.0));
	}
	
	
	
	
//###############################################################################################//
//
//										Hours
//
// pub fn abs ( &self ) -> Decimal
//
// pub fn neg ( self ) -> Self
//
// Tested Together eq
// pub fn eq  ( &self, &Self ) -> bool
// pub fn test_close ( &self, &Self, Decimal ) -> bool
//
//###############################################################################################//
//										~ abs ~													 //
	#[test]
	fn test_hours_abs ( )
	{
		let angle = Hours(1.23);
		assert_eq!(angle.abs(), 1.23);
		assert_eq!((-angle).abs(), 1.23);
	}
	
//										~ ops ~													 //
	#[test]
	fn test_hours_ops ( )
	{
		let angle = Hours(1.2345);
		assert_eq!(-angle, Hours(-1.2345));
	}
	
//										~ eq ~													 //
	#[test]
	fn test_hours_eq ( )
	{
		let angle = Hours(1.2345);
		assert_eq!(angle, Hours(1.2345 + DECIMAL_PRECISION / 2.0));
		assert!(angle.test_close(&Hours(2.2344), 1.0));
	}

	
	
//###############################################################################################//
//
//										Equatorial
//
// pub fn eq  ( &self, &Self ) -> bool
// pub fn test_close ( &self, &Self, Decimal ) -> bool
//
//###############################################################################################//
//										~ eq ~													 //
	#[test]
	fn test_equatorial_eq ( )
	{
		let val_1 = Equatorial{ra: Radians(1.11), dec: Radians(2.22)};
		let val_2 = Equatorial{
			ra:  Radians(1.11 + DECIMAL_PRECISION / 2.0), 
			dec: Radians(2.22 + DECIMAL_PRECISION / 2.0)};
		let val_3 = Equatorial{ra:  Radians(2.11), dec: Radians(3.22)};
		
		assert_eq!(val_1, val_2);
		assert!( val_1.test_close(&val_3, 1.001));
		assert!(!val_1.test_close(&val_3, 0.999));
	}


//###############################################################################################//
//
//										AngleAxis
//
// pub fn eq  ( &self, &Self ) -> bool
// pub fn test_close ( &self, &Self, Decimal ) -> bool
//
//###############################################################################################//
//										~ eq ~													 //
	#[test]
	fn test_angle_axis_eq ( )
	{
		let val_1 = AngleAxis{angle: Radians(1.1), axis: Vector3{x: 1.0, y: 2.0, z: 3.0}};
		let val_2 = AngleAxis{
			angle: Radians(1.1 + DECIMAL_PRECISION / 2.0), 
			axis:  Vector3{
				x: 1.0 + DECIMAL_PRECISION / 2.0, 
				y: 2.0 + DECIMAL_PRECISION / 2.0, 
				z: 3.0 + DECIMAL_PRECISION / 2.0}};
		let val_3 = AngleAxis{angle: Radians(2.1), axis: Vector3{x: 2.0, y: 3.0, z: 4.0}};
		
		assert_eq!(val_1, val_2);
		assert!( val_1.test_close(&val_3, 1.001));
		assert!(!val_1.test_close(&val_3, 0.999));
	}




//###############################################################################################//
//
//										Quaternion
//
// pub fn mul  ( &self, &Self ) -> bool
// pub fn eq   ( &self, &Self ) -> bool
// pub fn test_close ( &self, &Self, Decimal ) -> bool
//
//###############################################################################################//
//										~ mul ~													 //
	#[test]
	// The hamilton product.
	// Used a sample output to test against.
	fn test_quaternion_mul ( )
	{
		let q = Quaternion{w: 1.0, x: 2.0, y: 3.0, z: 4.0};
		let r = Quaternion{w: 4.0, x: 3.0, y: 2.0, z: 1.0};
		assert_eq!(q * r, Quaternion{w: -12.0, x: 6.0, y: 24.0, z: 12.0});
	}

//										~ eq ~													 //
	#[test]
	fn test_quaternion_eq ( )
	{
		let val_1 = Quaternion{w: 1.1, x: 1.1, y: 1.1, z: 1.1};
		let val_2 = Quaternion{
			w: 1.1 + DECIMAL_PRECISION / 2.0, 
			x: 1.1 + DECIMAL_PRECISION / 2.0, 
			y: 1.1 + DECIMAL_PRECISION / 2.0, 
			z: 1.1 + DECIMAL_PRECISION / 2.0};
		let val_3 = Quaternion{w: 2.1, x: 2.1, y: 2.1, z: 2.1};
		assert_eq!(val_1, val_2);
		assert!( val_1.test_close(&val_3, 1.001));
		assert!(!val_1.test_close(&val_3, 0.999));
	}




//###############################################################################################//
//
//										Pixel
//
// pub fn to_vector2 ( &self ) -> Vector2
// pub fn to_vector3 ( &self ) -> Vector3
//
//###############################################################################################//
//										~ to_vector2 ~											 //
	#[test]
	fn test_to_vector2_from_pixel ( )
	{
		let px = Pixel{x: 1, y: 2};
		assert_eq!(px.to_vector2(), Vector2{x: 1.0, y: 2.0});
	}



//###############################################################################################//
//
//										Vector2
//
// Tested Together ops
// pub fn mul ( self, Decimal ) -> Self
// pub fn add ( self, Vector2 ) -> Self
// pub fn sub ( self, Vector2 ) -> Self
// pub fn div ( self, Decimal ) -> Self
// pub fn neg ( self ) -> Self
//
// Tested Together eq
// pub fn eq  ( &self, &Self ) -> bool
// pub fn test_close ( &self, &Self, Decimal ) -> bool
//
//
//###############################################################################################//
//										~ ops ~													 //
	#[test]
	fn test_vector2_ops ( )
	{
		let vec_1 = Vector2{x: 1.1, y: 2.2};
		let vec_2 = Vector2{x: 2.2, y: 4.4};
		
		assert_eq!(vec_1 + vec_2, Vector2{x:  3.3, y:  6.6});
		assert_eq!(vec_1 - vec_2, Vector2{x: -1.1, y: -2.2});
		
		assert_eq!(vec_1 * 2.0,   vec_2);
		assert_eq!(vec_1 / 2.0,   Vector2{x: 0.55, y: 1.1});
		
		assert_eq!(-vec_1,        Vector2{x: -1.1, y: -2.2});
	}
	
//										~ eq ~													 //
	#[test]
	fn test_vector2_eq ( )
	{
		let vec_1 = Vector2{x: 1.1, y: 2.2};
		let vec_2 = Vector2{x: 1.1 + DECIMAL_PRECISION / 2.0, y: 2.2 + DECIMAL_PRECISION / 2.0};
		let vec_3 = Vector2{x: 2.1, y: 3.2};
		assert_eq!(vec_1, vec_2);
		assert!( vec_1.test_close(&vec_3, 1.001));
		assert!(!vec_1.test_close(&vec_3, 0.999));
	}
	

//###############################################################################################//
//
//										Vector3
//
// Tested Together ops
// pub fn mul ( self, Decimal ) -> Self
// pub fn add ( self, Vector2 ) -> Self
// pub fn sub ( self, Vector2 ) -> Self
// pub fn div ( self, Decimal ) -> Self
// pub fn neg ( self ) -> Self
//
// Tested Together eq
// pub fn eq  ( &self, &Self ) -> bool
// pub fn test_close ( &self, &Self, Decimal ) -> bool
//
//
//###############################################################################################//
//										~ ops ~													 //
	#[test]
	fn test_vector3_ops ( )
	{
		let vec_1 = Vector3{x: 1.1, y: 2.2, z: 3.3};
		let vec_2 = Vector3{x: 2.2, y: 4.4, z: 6.6};
		
		assert_eq!(vec_1 + vec_2, Vector3{x:  3.3, y:  6.6, z:  9.9});
		assert_eq!(vec_1 - vec_2, Vector3{x: -1.1, y: -2.2, z: -3.3});
		
		assert_eq!(vec_1 * 2.0,   vec_2);
		assert_eq!(vec_1 / 2.0,   Vector3{x: 0.55, y:  1.1, z:  1.65});
		
		assert_eq!(-vec_1,        Vector3{x: -1.1, y: -2.2, z: -3.3});
	}

//										~ eq ~													 //
	#[test]
	fn test_vector3_eq ( )
	{
		let vec_1 = Vector3{x: 1.1, y: 2.2, z: 3.3};
		let vec_2 = Vector3{
			x: 1.1 + DECIMAL_PRECISION / 2.0, 
			y: 2.2 + DECIMAL_PRECISION / 2.0,
			z: 3.3 + DECIMAL_PRECISION / 2.0};
		let vec_3 = Vector3{x: 2.1, y: 3.2, z: 4.3};
		assert_eq!(vec_1, vec_2);
		assert!( vec_1.test_close(&vec_3, 1.001));
		assert!(!vec_1.test_close(&vec_3, 0.999));
	}






//###############################################################################################//
//
//										BitField
//
// Tested Together ops
// pub fn bitand        ( self, BitField ) -> BitField
// pub fn bitor         ( self, BitField ) -> BitField
// pub fn bitand_assign ( &mut self, BitField )
// pub fn bitor_assign  ( &mut self, BitField )
//
//###############################################################################################//
//										~ ops ~													 //

	#[test]
	fn test_bit_field_ops ( )
	{
		let mut field_1 = BitField(0b1010101);
		let     field_2 = BitField(0b0000111);
		
		assert_eq!(field_1 | field_2, BitField(0b1010111));
		assert_eq!(field_1 & field_2, BitField(0b0000101));
		
		field_1 |= BitField(0b10);
		assert_eq!(field_1, BitField(0b1010111));
		field_1 &= BitField(0b11111);
		assert_eq!(field_1, BitField(0b0010111));
	}
}