//! This provides specifics of errors.  
//! Instead of using `Result<T, E>`, use `Error<T>` as a more specific error will be provided.  
//! # Example
//! ```
//! use star_tracker::util::err::Error;
//! use star_tracker::util::err::Errors;
//! fn function ( error: bool ) -> Error<bool>
//! {
//! 	return if error { Err(Errors::NoMatch) } else { Ok(error) }
//! }
//!
//! assert!(function(true).is_err());
//! assert!(function(false).is_ok());
//! ```



//###############################################################################################//
//									---	Error ---
//###############################################################################################//
/// An easier way to write `Result<T, Errors>`.
/// # Example
/// ```
/// use star_tracker::util::err::Error;
/// use star_tracker::util::err::Errors;
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