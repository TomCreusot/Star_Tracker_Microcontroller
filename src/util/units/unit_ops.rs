/// Implementation for Mul, Add, Sub, Div and Display
use std::fmt;
use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Div;
use super::{
	Degrees, Radians, Hours,
	Quaternion, Cartesian3D, Equatorial, AngleAxis, super::aliases::Decimal};

//###############################################################################################//
//										---	Degrees ---
//###############################################################################################//
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


impl PartialEq for Degrees {
	fn eq ( &self, other: &Self ) -> bool { return (self.0 - other.0).abs() < 0.00001; }
}


//###############################################################################################//
//										---	Radians ---
//###############################################################################################//
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
	
impl PartialEq for Radians {
 fn eq ( &self, other: &Self ) -> bool { return (self.0 - other.0).abs() < 0.00001; }
}
	



//###############################################################################################//
//									---	Equatiorial ---
//###############################################################################################//	

impl PartialEq for Equatorial {
	fn eq ( &self, other: &Self ) -> bool 
	{return (self.ra.0- other.ra.0).abs() < 0.00001 && (self.dec.0 - other.dec.0).abs() < 0.00001;}
}


	

//###############################################################################################//
//									---	Hours ---
//###############################################################################################//	

impl PartialEq for Hours {
	fn eq ( &self, other: &Self ) -> bool { return (self.0 - other.0).abs() < 0.00001; }
}


//###############################################################################################//
//									---	angle_axis ---
//###############################################################################################//	

impl PartialEq for AngleAxis {
	fn eq ( &self, other: &Self ) -> bool 
	{ 
		return self.angle == other.angle && self.axis == other.axis; 
	}
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
		 	(self.w - other.w).abs() < 0.00001 &&
			(self.x - other.x).abs() < 0.00001 &&
			(self.y - other.y).abs() < 0.00001 &&
			(self.z - other.z).abs() < 0.00001;
	}
}
		








//###############################################################################################//
//									---	Cartesian3D ---
//###############################################################################################//	

impl PartialEq for Cartesian3D {
	 fn eq ( &self, other: &Self ) -> bool 
	 { 
		 return 
		 	(self.x - other.x).abs() < 0.00001 &&
			(self.y - other.y).abs() < 0.00001 &&
			(self.z - other.z).abs() < 0.00001;
	}
}
		
