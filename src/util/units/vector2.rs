//! Implementation of Vector2.
use std::fmt;
use util::aliases::Decimal;
use super::Vector3;
use super::Vector2;

impl Vector2
{
    /// Finds the magnitude of the vector.
    /// # Example
    /// ```
    /// use star_tracker::util::units::Vector2;
    /// use star_tracker::util::test::TestEqual;
    /// let c = Vector2{x: 10.3, y: 23.1};
    /// assert!(c.magnitude().test_close(&25.2922913, 0.00001));
    /// ```
    pub fn magnitude ( &self ) -> Decimal
    {
        return (self.x.powf(2.0) + self.y.powf(2.0)).sqrt();
    }

    /// Normalizes the vector so the magnitude is 1.
	/// # Example
	/// ```
	/// use star_tracker::util::units::Vector2;
	/// use star_tracker::util::test::TestEqual;
	/// let mut c = Vector2{x: 123.4, y: 345.6};
	/// let c_out = Vector2{x: 0.3362673458386104, y: 0.941766569868912};
	/// c.normalize();
	/// assert!(c.test_close(&c_out, 0.0001));
	/// ```
    pub fn normalize ( &mut self )
    {
        let mag = self.magnitude();
        self.x /= mag;
        self.y /= mag;
    }


    /// Normalizes the vector so the magnitude is 1.
	/// # Example
	/// ```
	/// use star_tracker::util::units::Vector2;
	/// use star_tracker::util::test::TestEqual;
	/// let mut c = Vector2{x: 123.4, y: 345.5};
	/// let c_out = Vector2{x: 0.3362673458386104, y: 0.941766569868912};
	/// assert!(c.normalized().test_close(&c_out, 0.0001));
	/// ```
    pub fn normalized ( &self ) -> Self
    {
        let mag = self.magnitude();
        return Vector2{x: self.x / mag, y: self.y / mag};
    }


    /// Finds the dot product between the cartesian3D points.
	/// # Arguments
	/// * `other` - The other cartesian3D.
	///
	/// # Returns
	/// The dot product.
	///
	/// # Example
	/// ```
	/// use star_tracker::util::units::Vector2;
	/// let a = Vector2 { x: 2.0, y: 3.0 };
	/// let b = Vector2 { x: 5.0, y: 6.0 };
	/// assert_eq!(a.dot(b), 10.0 + 18.0);
	/// ```
    pub fn dot ( &self, other: Vector2 ) -> Decimal
    {
        return self.x * other.x + self.y * other.y;
    }


    /// Converts 2D Vector to 3D Vector by setting z to 0.
    /// # Example
    /// ```
    /// use star_tracker::util::units::Vector3;
    /// use star_tracker::util::units::Vector2;
    /// let a : Vector2 = Vector2 { x: 123.4, y: 345.6 };
    /// let b : Vector3 = a.to_vector3();
    /// assert_eq!(b.x, a.x);
    /// assert_eq!(b.y, a.y);
    /// assert_eq!(b.z, 0.0);
    /// ```
    pub fn to_vector3 ( &self ) -> Vector3
    {
        return Vector3{x: self.x, y: self.y, z: 0.0};
    }
}



//###############################################################################################//
//							---	Debug ---
//###############################################################################################//


impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "Vector2({:.3}, {:.3})", self.x, self.y)?;
		return Ok(());
	}
}


impl fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "Vector3(x: {}, y: {})", self.x, self.y)?;
		return Ok(());
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
    use util::test::TestEqual;
    use util::units::Vector2;
    use util::units::Vector3;

    #[test]
    pub fn test_magnitude ( )
    {
        let c = Vector2{x: 10.3, y: 23.1};
        assert!(c.magnitude().test_close(&25.2922913, 0.00001));
    }

    #[test]
    pub fn test_normalize ( )
    {
        let mut c = Vector2{x: 123.4, y: 345.6};
        let c_out = Vector2{x: 0.3362673458386104, y: 0.941766569868912};
        c.normalize();
        assert!(c.test_close(&c_out, 0.0001));
    }

    #[test]
    pub fn test_normalized ( )
    {
        let c = Vector2{x: 123.4, y: 345.6};
        let c_out = Vector2{x: 0.3362673458386104, y: 0.941766569868912};
        assert!(c.normalized().test_close(&c_out, 0.0001));
    }

    #[test]
    pub fn test_dot ( )
    {
        let a = Vector2 { x: 2.0, y: 3.0 };
        let b = Vector2 { x: 5.0, y: 6.0 };
        assert_eq!(a.dot(b), 10.0 + 18.0);
    }


    #[test]
    pub fn test_to_vector3 ( )
    {
        let a : Vector2 = Vector2 { x: 123.4, y: 345.6 };
        let b : Vector3 = a.to_vector3();
        assert_eq!(b.x, a.x);
        assert_eq!(b.y, a.y);
        assert_eq!(b.z, 0.0);
    }

}
