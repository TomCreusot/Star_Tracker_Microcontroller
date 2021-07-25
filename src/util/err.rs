//! This provides specifics of errors.
//! Instead of using `Result<T, E>`, use `Error<T>` as a more specific error will be provided.


/// An error of a specified size.
pub type Error<T> = Result<T, Errors>;


/// A storage of multiple errors to select from.
/// # Errors
/// ## Out Of Bounds
/// When an index to a list/array is provided and is out of the initialized range.
///
/// ## Invalid Size
/// An array is of an invalid size to be used.
///
/// ## Invalid Value
/// The value is not useful in the given circumstance.
#[derive(Debug,Clone)]
pub enum Errors
{
	OutOfBounds,
	InvalidSize,
	InvalidValue,
}

/*
impl fmt::Debug for Errors
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
	{
		match self
		{
			OutOfBounds => write!("Array Out of Bounds Error: An index provided to List/Array is out of the bounds."),
			InvalidSize => write!("Invalid Size Error: The size of the array provided is {}, where the expected range is {} to {} (inclusive)", self.unwrap().actual, self.unwrap().low, self.unwrap.high),
			InvalidValue => write!("Invalid Value Error: The provided value is {}, the expaected values are ." ),
		}
		
		return Ok();
	}
}

impl ToString for Errors
{
	fn to_string ( &self ) -> String
	{
		match self
		{
			OutOfBounds => return "Out Of Bounds Error",
			InvalidSize => return "Invalid Size Error",
			InvalidValue => return "Invalid Value Error",
		}
	}
}*/