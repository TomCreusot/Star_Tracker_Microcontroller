//! Implementation of [BitField](crate::util::units::BitField).
use super::BitField;
use super::BitCompare;

impl BitField
{
	/// The number of elements the bit field can represent.
	pub const FIELDS : usize = 32;
	
	pub const ALL : BitField = BitField(u32::MAX);
	
	/// Checks if there are any similarities between the bitfields.  
	/// Returns true if the a bit is true in both.
	pub fn compare ( &self, field: BitCompare ) -> bool
	{
		match field
		{
			BitCompare::Any(x) => return (*self & x).0 != 0,
			BitCompare::All(x) => return *self & x == x
		}
	}
	
	/// Sets the bit to the desired value.
	pub fn set ( &mut self, index: usize, value: bool )
	{
		if value
		{
			self.0 |= 1 << index;
		}
		else
		{
			self.0 &= !(1 << index);
		}
	}
	
	
	
}