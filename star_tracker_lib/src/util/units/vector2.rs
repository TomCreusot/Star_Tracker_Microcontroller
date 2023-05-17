//! Implementation for [Vector2](crate::util::units::Vector2).
use crate::core_include::*;

use crate::util::aliases::DECIMAL_PRECISION;
use crate::util::aliases::Decimal;
use crate::util::units::Vector3;
use crate::util::units::Vector2;
use crate::util::units::Pixel;
use crate::util::err::Errors;
use crate::util::err::Error;

use crate::util::Maths;


impl Vector2
{
//###############################################################################################//
//									--- Operations ---
//###############################################################################################//
	/// Finds the magnitude/length of the vector.
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Vector2;
	/// use star_tracker_lib::util::test::TestEqual;
	///
	/// let c = Vector2{x: 10.3, y: 23.1};
	/// assert!(c.magnitude().test_close(&25.2922913, 0.00001));
	/// ```
	pub fn magnitude ( &self ) -> Decimal
	{
		return (self.x.powf(2.0) + self.y.powf(2.0)).sqrt();
	}

	/// Normalizes the vector so the magnitude is 1.  
	/// If the magnitude is 0, the result will be Errors::NaN.  
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Vector2;
	/// use star_tracker_lib::util::test::TestEqual;
	///
	/// let mut c = Vector2{x: 123.4, y: 345.6};
	/// let c_out = Vector2{x: 0.3362673458386104, y: 0.941766569868912};
	///
	/// c.normalize().expect("Will return error if vector magnitude is 0.");
	/// assert!(c.test_close(&c_out, 0.0001));
	/// ```
	pub fn normalize ( &mut self ) -> Error<()>
	{
		let mag = self.magnitude();
		if mag < DECIMAL_PRECISION
		{
			return Err(Errors::NaN);
		}
		self.x /= mag;
		self.y /= mag;
		return Result::Ok(());
	}


	/// Normalizes the vector so the magnitude is 1.  
	/// If the magnitude is 0, the result will be Errors::NaN.
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Vector2;
	/// use star_tracker_lib::util::test::TestEqual;
	///
	/// let mut c = Vector2{x: 123.4, y: 345.5};
	/// let c_out = Vector2{x: 0.3362673458386104, y: 0.941766569868912};
	/// assert!(c.normalized().expect("0 magnitude returns NAN").test_close(&c_out, 0.0001));
	/// ```
	pub fn normalized ( &self ) -> Error<Self>
	{
		let mag = self.magnitude();
		if mag < DECIMAL_PRECISION
		{
			return Result::Err(Errors::NaN);
		}
		return Result::Ok(Vector2{x: self.x / mag, y: self.y / mag});
	}


	/// Finds the dot product between the cartesian3D points.
	///
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Vector2;
	///
	/// let a = Vector2 { x: 2.0, y: 3.0 };
	/// let b = Vector2 { x: 5.0, y: 6.0 };
	/// assert_eq!(a.dot(b), 10.0 + 18.0);
	/// ```
	pub fn dot ( &self, other: Vector2 ) -> Decimal
	{
		return self.x * other.x + self.y * other.y;
	}



//###############################################################################################//
//									--- Conversion ---
//###############################################################################################//

	/// Converts 2D Vector to 3D Vector by setting z to 0.
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Vector3;
	/// use star_tracker_lib::util::units::Vector2;
	///
	/// let a: Vector2 = Vector2 { x: 123.4, y: 345.6 };
	/// let b: Vector3 = a.to_vector3();
	/// assert_eq!(b.x, a.x);
	/// assert_eq!(b.y, a.y);
	/// assert_eq!(b.z, 0.0);
	/// ```
	pub fn to_vector3 ( &self ) -> Vector3
	{
		return Vector3{x: self.x, y: self.y, z: 0.0};
	}
	
	
	/// Converts the vector into a pixel coordinated.  
	/// ROUNDS TO THE NEAREST WHOLE NUMBER.
	/// # Example
	/// ```
	/// use star_tracker_lib::util::units::Vector2;
	/// use star_tracker_lib::util::units::Pixel;
	///
	/// let a: Vector2 = Vector2 { x: 123.4, y: 345.6 };
	/// assert_eq!(a.to_pixel().x, 123);
	/// assert_eq!(a.to_pixel().y, 346);
	/// ```
	pub fn to_pixel ( &self ) -> Pixel
	{
		return Pixel{x: self.x.round() as usize, y: self.y.round() as usize};
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
	use crate::util::test::TestEqual;
	use crate::util::units::Vector2;
	use crate::util::units::Vector3;
	use crate::util::units::Pixel;
	use crate::util::err::Errors;

//###############################################################################################//
//
//										Features
//
// pub fn magnitude  ( &self )        -> Decimal
// pub fn normalize  ( &mut self )    -> Self
// pub fn normalized ( &self )        -> Error<Self>
// pub fn dot        ( &self, &Self ) -> Decimal
//
//###############################################################################################//
//										~ magnitude ~											 //
	#[test]
	pub fn test_magnitude ( )
	{
		let c = Vector2{x: 10.3, y: 23.1};
		assert!(c.magnitude().test_close(&25.2922913, 0.00001));
	}

//										~ normalize ~											 //
	#[test]
	// If the size is not 0, should return valid.
	pub fn test_normalize ( )
	{
		let mut c = Vector2{x: 123.4, y: 345.6};
		let c_out = Vector2{x: 0.3362673458386104, y: 0.941766569868912};
		assert!(c.normalize().is_ok());
		assert!(c.test_close(&c_out, 0.0001));
	}
	
	#[test]
	// If the size is 0, should return NaN.
	pub fn test_normalize_error ( )
	{
		let mut c = Vector2{x: 0.0, y: 0.0};
		assert_eq!(c.normalize(), Err(Errors::NaN));
	}

//										~ normalized ~											 //
	#[test]
	// If the size is not 0, should return valid.
	pub fn test_normalized ( )
	{
		let c = Vector2{x: 123.4, y: 345.6};
		let c_out = Vector2{x: 0.3362673458386104, y: 0.941766569868912};
		assert!(c.normalized().expect("Input vector is 0.").test_close(&c_out, 0.0001));
	}
	
	#[test]
	// If the size is 0, should return NaN.
	pub fn test_normalized_error ( )
	{
		let c = Vector2{x: 0.0, y: 0.0};
		assert_eq!(c.normalized(), Err(Errors::NaN));
	}

//										~ dot ~													 //
	#[test]
	pub fn test_dot ( )
	{
		let a = Vector2 { x: 2.0, y: 3.0 };
		let b = Vector2 { x: 5.0, y: 6.0 };
		assert_eq!(a.dot(b), 10.0 + 18.0);
	}


//###############################################################################################//
//
//										Conversion
//
// pub fn to_vector3 ( &self ) -> Vector3
// pub fn to_vector2 ( &self ) -> Pixel
//
//###############################################################################################//
//										~ vector3 ~												 //
	#[test]
	pub fn test_to_vector3 ( )
	{
		let a: Vector2 = Vector2 { x: 123.4, y: 345.6 };
		let b: Vector3 = a.to_vector3();
		assert_eq!(b.x, a.x);
		assert_eq!(b.y, a.y);
		assert_eq!(b.z, 0.0);
	}
	
//										~ to_pixel ~											 //
	#[test]
	fn test_to_pixel ( )
	{
		let mut vec = Vector2{x: 1.1, y: 2.2};
		assert_eq!(vec.to_pixel(), Pixel{x: 1, y: 2});
		vec = Vector2{x: 0.9, y: 1.9};
		assert_eq!(vec.to_pixel(), Pixel{x: 1, y: 2});
	}
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
//###############################################################################################//
//
//										Debug
// Display: Show neat (3dp)
//
// Debug: Show everything (all dp)
//
//###############################################################################################//
	//								- Display / Debug fmt -										//
	#[test]
	fn test_display_fmt ( )
	{
		let vec = Vector2 { x: 1.1234, y: 2.1234 };
		assert_eq!(format!("{:123414}", vec), "Vector2(1.123, 2.123)");
	}
	
	
	#[test]
	fn test_debug_fmt ( )
	{
		let vec = Vector2 { x: 1.1234, y: 2.1234 };
		assert_eq!(format!("{:?}", vec), "Vector2(x: 1.1234, y: 2.1234)");
	}
	
}
