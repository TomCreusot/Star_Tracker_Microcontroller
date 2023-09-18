//! Implementation of BitField.
use crate::util::units::BitField;
use crate::util::units::BitCompare;
use crate::util::aliases::UInt;

impl BitField
{
	/// The number of elements the bit field can represent.
	pub const FIELDS : usize = core::mem::size_of::<UInt>();
	
	pub const ALL : BitField = BitField(UInt::MAX);
	
	/// Checks if there are any similarities between the bitfields.  
	/// Returns true if the a bit is true in both.
	pub fn compare ( self, field: BitCompare ) -> bool
	{
		match field
		{
			BitCompare::Any(x) => return (self & x).0 != 0,
			BitCompare::All(x) => return self & x == x | self
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
	use crate::util::units::BitCompare;
	use crate::util::units::BitField;


//###############################################################################################//
//
//										BitField
//
// pub fn compare ( &self, BifCompare ) -> Self;
// pub fn set     ( &mut self, index: usize, value: bool );
//
//###############################################################################################//
//										~ compare ~												 //
	#[test]
	fn test_compare_any_false ( )
	{
		let field   = BitField(0b00100100);
		let other_1 = BitField(0b11011011);
		let other_2 = BitField(0b00000000);
		assert!(!field.compare(BitCompare::Any(other_1)));
		assert!(!field.compare(BitCompare::Any(other_2)));
	}
	
	#[test]
	fn test_compare_any_true ( )
	{
		let field   = BitField(0b00100100);
		let other_1 = BitField(0b00100000);
		let other_2 = BitField(0b11111111);
		assert!(field.compare(  BitCompare::Any(other_1)));
		assert!(other_1.compare(BitCompare::Any(other_2)));
	}




	#[test]
	fn test_compare_all_false ( )
	{
		let field   = BitField(0b00100100);
		let other_1 = BitField(0b11011011);
		let other_2 = BitField(0b00100000);
		let other_3 = BitField(0b00000000);
		let other_4 = BitField(0b11111111);
		assert!(!field.compare(BitCompare::All(other_1)));
		assert!(!field.compare(BitCompare::All(other_2)));
		assert!(!field.compare(BitCompare::All(other_3)));
		assert!(!field.compare(BitCompare::All(other_4)));
		assert!(!other_4.compare(BitCompare::All(field)));
	}
	
	#[test]
	fn test_compare_all_true ( )
	{
		let field   = BitField(0b00100100);
		let other_1 = BitField(0b00100100);
		assert!(field.compare(  BitCompare::All(other_1)));
	}




//										~ set ~													 //
	#[test]
	fn test_set_true ( )
	{
		let mut field = BitField(0);
		field.set(1, true);
		assert_eq!(field.0, 2);
		field.set(0, true);
		assert_eq!(field.0, 3);
		field.set(3, true);
		assert_eq!(field.0, 11);
	}
	
	#[test]
	fn test_set_false ( )
	{
		let mut field = BitField(0b111);
		field.set(0, false);
		assert_eq!(field.0, 6);
		field.set(1, false);
		assert_eq!(field.0, 4);
		field.set(2, false);
		assert_eq!(field.0, 0);
	}

}