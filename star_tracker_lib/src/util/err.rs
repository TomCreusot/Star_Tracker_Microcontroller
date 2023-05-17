//! This provides specifics of errors.  
//!
//! Instead of using `Result<T, E>`, use `Error<T>` as a more specific error will be provided.  
//! # Example
//! ```
//! use star_tracker_lib::util::err::Error;
//! use star_tracker_lib::util::err::Errors;
//! fn function ( error: bool ) -> Error<bool>
//! {
//! 	return if error { Err(Errors::NoMatch) } else { Ok(error) }
//! }
//!
//! assert!(function(true).is_err());
//! assert!(function(false).is_ok());
//! ```
use crate::core_include::*;

//###############################################################################################//
//									---	Error ---
//###############################################################################################//
/// An easier way to write `Result<T, Errors>`.
/// # Example
/// ```
/// use star_tracker_lib::util::err::Error;
/// use star_tracker_lib::util::err::Errors;
/// fn function ( error: bool ) -> Error<bool> { return Err(Errors::NoMatch); }
/// ```
pub type Error<T> = Result<T, Errors>;



//###############################################################################################//
//									---	Errors ---
//###############################################################################################//

/// A set of multiple errors to select from.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Errors
{
	/// The index exceeds a linear datastructures size.  
	/// This could be get() or set() from a list.  
	OutOfBounds,
	/// The current action will make a datastructure exceed its allocated size.  
	/// This could be trying to push an element into a full ArrayList.  
	InvalidSize,
	/// The provided value is not valid in the given context.
	InvalidValue,
	/// For 2d objects (matrix, image), if the provided point is outside of the x bounds.
	OutOfBoundsX,
	/// For 2d objects (matrix, image), if the provided point is outside of the y bounds.
	OutOfBoundsY,
	/// A core part of the algorithm failed and the a star match was not found.
	NoMatch,
	/// Not actual number.  
	/// Divide by 0.  
	/// A mathematical instance is not valid to be expressed after undergoing the edit.
	NaN,
}













//###############################################################################################//
//###############################################################################################//
//
//										Unit Tests
//
// This is just to pass llvm-cov.
// Derives are not excludable.
//
//###############################################################################################//
//###############################################################################################//

#[cfg(test)]
mod test
{
	use crate::util::err::Errors;
	
	#[test]
	fn test_partial_eq ( )
	{
		let a = Errors::OutOfBounds;
		let b = Errors::InvalidSize;
		let c = Errors::InvalidValue;
		let d = Errors::OutOfBoundsX;
		let e = Errors::OutOfBoundsY;
		let f = Errors::NoMatch;
		let g = Errors::NaN;
		
		assert_eq!(a, a);
		assert_eq!(b, b);
		assert_eq!(c, c);
		assert_eq!(d, d);
		assert_eq!(e, e);
		assert_eq!(f, f);
		assert_eq!(g, g);
		
		assert_ne!(a, b);
		assert_ne!(b, a);
		assert_ne!(c, a);
		assert_ne!(d, a);
		assert_ne!(e, a);
		assert_ne!(f, a);
		assert_ne!(g, a);
	}
	
	
	#[test]
	fn test_debug ( )
	{
		assert_eq!(format!("{:?}", Errors::OutOfBounds),  "OutOfBounds");
		assert_eq!(format!("{:?}", Errors::InvalidSize),  "InvalidSize");
		assert_eq!(format!("{:?}", Errors::InvalidValue), "InvalidValue");
		assert_eq!(format!("{:?}", Errors::OutOfBoundsX), "OutOfBoundsX");
		assert_eq!(format!("{:?}", Errors::OutOfBoundsY), "OutOfBoundsY");
		assert_eq!(format!("{:?}", Errors::NoMatch),      "NoMatch");
		assert_eq!(format!("{:?}", Errors::NaN),          "NaN");
	}

	#[test]
	fn test_clone ( )
	{
		assert_eq!(Errors::OutOfBounds.clone(),  Errors::OutOfBounds);
		assert_eq!(Errors::InvalidSize.clone(),  Errors::InvalidSize);
		assert_eq!(Errors::InvalidValue.clone(), Errors::InvalidValue);
		assert_eq!(Errors::OutOfBoundsX.clone(), Errors::OutOfBoundsX);
		assert_eq!(Errors::OutOfBoundsY.clone(), Errors::OutOfBoundsY);
		assert_eq!(Errors::NoMatch.clone(),      Errors::NoMatch);
		assert_eq!(Errors::NaN.clone(),          Errors::NaN);
	}
}