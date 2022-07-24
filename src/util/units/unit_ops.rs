/// Implementation for Mul, Add, Sub, Div and Display
use std::fmt;
use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Div;
use std::ops::Neg;
use super::{
	Degrees, Radians, Hours,
	Quaternion, Cartesian3D, Equatorial, AngleAxis, Pixel, PixelWeighted, super::aliases::Decimal};

use util::test::TestEqual;
use util::aliases::DECIMAL_PRECISION;

//###############################################################################################//
//										---	Decimal ---
//###############################################################################################//

impl TestEqual for Decimal {
	fn test_close ( &self, other: &Self, precision: Decimal ) -> bool 
	{ return (self - other).abs() < precision }
}

//###############################################################################################//
//										---	Degrees ---
//###############################################################################################//
impl Degrees
{
	pub fn abs ( &self ) -> Decimal { return self.0.abs(); }
}


impl fmt::Display for Degrees {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.0) } }

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

impl Into<Decimal> for Degrees { fn into ( self ) -> Decimal { return self.0; } }


impl TestEqual for Degrees {
	fn test_close ( &self, other: &Self, precision: Decimal ) -> bool 
	{ return (self.0 - other.0).abs() < precision }
}


//###############################################################################################//
//										---	Radians ---
//###############################################################################################//

impl Radians
{
	pub fn abs ( &self ) -> Decimal { return self.0.abs(); }
}

impl fmt::Display for Radians {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.0) } }


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
	
impl Into<Decimal> for Radians { fn into ( self ) -> Decimal { return self.0; } }

impl TestEqual for Radians {
	fn test_close ( &self, other : &Self, precision: Decimal ) -> bool 
	{ return (self.0 - other.0).abs() < precision }
}

//###############################################################################################//
//									---	Hours ---
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

impl Into<Decimal> for Hours { fn into ( self ) -> Decimal { return self.0; } }


impl TestEqual for Hours {
	fn test_close ( &self, other: &Self, precision: Decimal ) -> bool 
	{ return (self.0 - other.0).abs() < precision }
}

//###############################################################################################//
//									---	Equatiorial ---
//###############################################################################################//	

impl PartialEq for Equatorial {
	fn eq ( &self, other: &Self ) -> bool 
	{return (self.ra.0- other.ra.0).abs() < DECIMAL_PRECISION && 
		(self.dec.0 - other.dec.0).abs() < DECIMAL_PRECISION;}
}


impl TestEqual for Equatorial {
	fn test_close ( &self, other: &Self, precision: Decimal ) -> bool 
	{ return (self.ra.0- other.ra.0).abs() < precision && 
		(self.dec.0 - other.dec.0).abs() < precision; }
}

	


//###############################################################################################//
//									---	AngleAxis ---
//###############################################################################################//	

impl PartialEq for AngleAxis {
	fn eq ( &self, other: &Self ) -> bool 
	{ 
		return self.angle == other.angle && self.axis == other.axis; 
	}
}


impl TestEqual for AngleAxis {
	fn test_close ( &self, other: &Self, precision: Decimal ) -> bool 
	{ return self.angle.test_close(&other.angle, precision) && 
		other.axis.test_close(&other.axis, precision); }
}

	
//###############################################################################################//
//									---	Quaternion ---
//###############################################################################################//	



impl Mul for Quaternion
{
	type Output = Self;
	/// Finds the Hamilton Product.
	/// # Example
	/// ```
	/// use star_tracker::util::units::Quaternion;
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
		

impl TestEqual for Quaternion {
	fn test_close ( &self, other: &Self, precision: Decimal ) -> bool
	{	
		return 
		(self.w - other.w).abs() < precision &&
		(self.x - other.x).abs() < precision &&
		(self.y - other.y).abs() < precision &&
		(self.z - other.z).abs() < precision;
	}
}

	






//###############################################################################################//
//									---	Pixel ---
//###############################################################################################//


impl Into<Cartesian3D> for Pixel { 
	fn into ( self ) -> Cartesian3D { 
		return Cartesian3D{x: self.x as Decimal, y: self.y as Decimal, z: 0.0} } }




//###############################################################################################//
//									---	PixelWeighted ---
//###############################################################################################//


impl Into<Cartesian3D> for PixelWeighted { 
	fn into ( self ) -> Cartesian3D { return Cartesian3D{x: self.x, y: self.y, z: 0.0} } }



	
//###############################################################################################//
//									---	Cartesian3D ---
//###############################################################################################//	

impl Mul<Decimal> for Cartesian3D {
    type Output = Self;
    fn mul ( self, rhs: Decimal ) -> Self { 
		return Cartesian3D{x: self.x * rhs, y: self.y * rhs, z: self.z * rhs} } }

impl Mul<Cartesian3D> for Cartesian3D {
    type Output = Self;
    fn mul ( self, rhs: Cartesian3D ) -> Self {
		return Cartesian3D{x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z} } }

impl Add<Cartesian3D> for Cartesian3D {
    type Output = Self;
    fn add ( self, rhs: Cartesian3D ) -> Self {
		return Cartesian3D{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z} } }
	
	
impl Sub for Cartesian3D {
    type Output = Self;
    fn sub ( self, rhs: Cartesian3D ) -> Self {
		return Cartesian3D{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z} } }

impl Div<Decimal> for Cartesian3D {
    type Output = Self;
	fn div ( self, rhs: Decimal ) -> Self {
		return Cartesian3D{x: self.x / rhs, y: self.y / rhs, z: self.z / rhs} } }


impl Div<Cartesian3D> for Cartesian3D {
    type Output = Self;
	fn div ( self, rhs: Cartesian3D ) -> Self {
	return Cartesian3D{x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z} } }
	

impl PartialEq for Cartesian3D {
	 fn eq ( &self, other: &Self ) -> bool 
	 { 
		 return 
		 	(self.x - other.x).abs() < DECIMAL_PRECISION &&
			(self.y - other.y).abs() < DECIMAL_PRECISION &&
			(self.z - other.z).abs() < DECIMAL_PRECISION;
	}
}
		


impl TestEqual for Cartesian3D {
	fn test_close ( &self, other: &Self, precision: Decimal ) -> bool
	{	
		return 
		(self.x - other.x).abs() < precision &&
		(self.y - other.y).abs() < precision &&
		(self.z - other.z).abs() < precision;
	}
}




