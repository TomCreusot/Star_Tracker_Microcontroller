use crate::util::linear_lookup::LinearLookup;
use crate::util::aliases::UInt;

/// Creates a word list given the input sizes.
/// Ensure your architecture for the constructor and target computer is equal to or greater than `word_size`.  
/// The amount of memory saved is determined by how many nibbles can fit in a word `floor(word_size / nibble_size)`.   
///
/// # Arguments
/// * `word_size` - The native word size.
/// * `nibs_size` - The size of the target bit size that fits in word size.
/// * `nibs_num`  - The number of nibbles that you need to store.
///
/// # Example
/// ```
/// use star_tracker_lib::create_word;
/// use star_tracker_lib::util::word::WordSize;
/// use star_tracker_lib::util::word::WordList;
/// use star_tracker_lib::util::err::Unsafe;
///
/// const architecture_word_size: usize = 32; // The computer is 64 bit and microcontroller is 32 bit.
/// const nibble_size: usize = 8;  // Storing 8 bit integers.
/// const nibble_num:  usize = 30; // Require 30, 8 bit integers.
///
/// // Creates a struct holding a list of 7 words storing 30 individual bytes.
/// // This is unsafe as the bit size must consider the current and target architecture size.
/// // Failure to do so will produce unexpected results and potentially a panic.
/// let mut word_unsafe : Unsafe<WordList, &str> = create_word!(architecture_word_size, nibble_size, nibble_num);
/// println!("{}", word_unsafe.reason);        // Prints the reason why this is unsafe.
/// let word = word_unsafe.get_unsafe_value();
/// ```
#[macro_export]
macro_rules! create_word {
	($word_size:expr, $nibs_size:expr, $nibs_num:expr) => 
	{
		{	
		Unsafe{
		value:
			&mut WordList{
			array: &mut [0; WordList::array_size($word_size, $nibs_size, $nibs_num)],
			size: WordSize{
				word_size: $word_size, 
				nibbles_num: WordList::nibbles_in_word($word_size, $nibs_size), 
				nibbles_size: $nibs_size}
		},
		reason:
		r#"
This is unsafe as the bit size must consider the current and target architecture size.
Failure to do so will produce unexpected results and potentially a panic.
		"#}
		}
	}
}



/// Specifies what the size of a word is and the size of the nibbles to be stored.
/// Use with `WordList`.
#[derive(Clone, Copy)]
pub struct WordSize
{
	/// The word size for the target platform.
	pub word_size:    usize,
	/// The number of nibbles that fit in the word.
	pub nibbles_num:  usize,
	/// The size of the nibbles.
	pub nibbles_size: usize,
}

/// Allows you to store numbers efficiently.  
/// An array of words (native memory sizes) which store a set of smaller integers.  
/// If you are using a microcontroller, it may have 32 bit, if you are trying to store bytes, they will take up 4 times more space.  
/// This is mainly used to store images on a microcontroller.  
///
///
/// THIS DOES NOT HAVE ERROR HANDLING!    
/// ENSURE YOUR CODE IS SAFE!
/// # Example
/// ```
/// use star_tracker_lib::create_word;
/// use star_tracker_lib::util::word::WordSize;
/// use star_tracker_lib::util::word::WordList;
/// use star_tracker_lib::util::err::Unsafe;
///
/// const architecture_word_size: usize = 32; // The computer is 64 bit and microcontroller is 32 bit.
/// const nibble_size: usize = 8;  // Storing 8 bit integers.
/// const nibble_num:  usize = 30; // Require 30, 8 bit integers.
///
/// let mut word: WordList;
///
/// // Creates a struct holding a list of 7 words storing 30 individual bytes.
/// // This is unsafe as the bit size must consider the current and target architecture size.
/// // Failure to do so will produce unexpected results and potentially a panic.
/// let mut word_unsafe : Unsafe<WordList, &str> = create_word!(architecture_word_size, nibble_size, nibble_num);
/// println!("{}", word_unsafe.reason);        // Prints the reason why this is unsafe.
/// let word = word_unsafe.get_unsafe_value();
///
/// // This is mainly used to store images.
/// // Iterating over the image will take a substantial amount of time if error checking is considered.
/// // Get and Set are unsafe as there is no error checking.
/// unsafe
/// {
/// word.set(0, 123); // The value at the first index is now 123.
/// word.get(0);      // 123.
/// }
/// ```
pub struct WordList <'a>
{
	pub array: &'a mut dyn LinearLookup<usize>,
	pub size:  WordSize
}




impl <'a> WordList <'a>
{
	pub const fn nibbles_in_word ( word_size: usize, nibs_size: usize ) -> usize
	{
		word_size / nibs_size // floor
	}

	pub const fn array_size ( word_size: usize, nibs_size: usize, nibs_num: usize ) -> usize
	{
		nibs_num.div_ceil(Self::nibbles_in_word(word_size, nibs_size)) // ceil
	}



	/// Sets the nibble at the given index.
	/// # unsafe
	/// WordList is mainly used to store images.
	/// Iterating over the image will take a substantial amount of time if error checking is considered.
	/// Get and Set are unsafe as there is no error checking.
	pub unsafe fn set ( &mut self, index: usize, value: usize )
	{
		let index_arr = index / self.size.nibbles_num;
		let mut word = self.array.get(index_arr);
		word = Self::word_set(word, index % self.size.nibbles_num, self.size, value);
		let _ = self.array.set(index_arr, word);
	}
	
	
	/// Gets the nibble at the given index.
	/// # unsafe
	/// WordList is mainly used to store images.
	/// Iterating over the image will take a substantial amount of time if error checking is considered.
	/// Get and Set are unsafe as there is no error checking.
	pub unsafe fn get ( &self, index: usize ) -> usize
	{
		let index_arr = index / self.size.nibbles_num;
		let word = self.array.get(index_arr);
		Self::word_get(word, index % self.size.nibbles_num, self.size)
	}
	
	
	/// Enter the word, the nibble at the index and the value to set and the outcome will be returned.
	unsafe fn word_set ( word: usize, index: usize, size: WordSize, value: usize ) -> usize
	{
		let bits = Self::first_n_bits(size.nibbles_size);
		let pos  = index * size.nibbles_size;
		let word = word & !(bits << pos);
		word | (value & bits) << pos
	}


	/// Gets the nibble inside the given word.    
	/// Enter the word and the nibble at the index will be returned.
	unsafe fn word_get ( word: usize, index: usize, size: WordSize ) -> usize
	{
		(word >> (index * size.nibbles_size)) & Self::first_n_bits(size.nibbles_size)
	}



	/// Sets the first "n" LSB's to high and the rest to low.
	/// Ensure the number is equal to or less than the target architecture.
	unsafe fn first_n_bits ( n: usize ) -> usize
	{
		if n < usize::BITS as usize
		{
			return (1_usize << n) - 1;
		}
		else
		{
			return usize::MAX;
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
use crate::util::err::Unsafe;
use crate::util::word::WordList;
use crate::util::word::WordSize;
use crate::create_word;


//###############################################################################################//
//									--- Get ---
//###############################################################################################//

	#[test]
	fn test_get_single_word ( )
	{
		unsafe
		{
		let mut not_safe = create_word!(32, 32, 1);
		let word = not_safe.get_unsafe_value();
		let _ = word.array.set(0, 0xFFFF_FFFF);
		assert_eq!(word.get(0),   0xFFFF_FFFF);
		
		let mut not_safe = create_word!(32, 16, 2);
		let word = not_safe.get_unsafe_value();
		let _ = word.array.set(0, 0xABCD_1234);
		assert_eq!(word.get(0), 0x1234);
		assert_eq!(word.get(1), 0xABCD);
		
		let mut not_safe = create_word!(32, 8, 4);
		let word = not_safe.get_unsafe_value();
		let _ = word.array.set(0, 0xAB_12_CD_34);
		assert_eq!(word.get(0), 0x34);
		assert_eq!(word.get(1), 0xCD);
		assert_eq!(word.get(2), 0x12);
		assert_eq!(word.get(3), 0xAB);
		
		let mut not_safe = create_word!(32, 4, 8);
		let word = not_safe.get_unsafe_value();
		let _ = word.array.set(0, 0xA_B_C_D_E_F_1_2);
		assert_eq!(word.get(0), 0x2);
		assert_eq!(word.get(1), 0x1);
		assert_eq!(word.get(2), 0xF);
		assert_eq!(word.get(3), 0xE);
		assert_eq!(word.get(4), 0xD);
		assert_eq!(word.get(5), 0xC);
		assert_eq!(word.get(6), 0xB);
		assert_eq!(word.get(7), 0xA);
		}
	}
	
	
	#[test]
	// If there are multiple words and the containing nibble 
	fn test_get_multi_word_full ( )
	{
		unsafe
		{
		let mut not_safe = create_word!(32, 16, 4);
		let word = not_safe.get_unsafe_value();
		let _ = word.array.set(0, 0x3333_4444);
		let _ = word.array.set(1, 0x1111_2222);
		assert_eq!(word.get(0), 0x4444);
		assert_eq!(word.get(1), 0x3333);
		assert_eq!(word.get(2), 0x2222);
		assert_eq!(word.get(3), 0x1111);

		let mut not_safe = create_word!(32, 8, 5);
		let word = not_safe.get_unsafe_value();
		let _ = word.array.set(0, 0x11_22_33_44);
		let _ = word.array.set(1, 0x00_00_00_AA);
		assert_eq!(word.get(0), 0x44);
		assert_eq!(word.get(1), 0x33);
		assert_eq!(word.get(2), 0x22);
		assert_eq!(word.get(3), 0x11);
		assert_eq!(word.get(4), 0xAA);

		let mut not_safe = create_word!(32, 8, 12);
		let word = not_safe.get_unsafe_value();
		let _ = word.array.set(0, 0x88_99_AA_BB);
		let _ = word.array.set(1, 0x44_55_66_77);
		let _ = word.array.set(2, 0x00_11_22_33);
		assert_eq!(word.get(0), 0xBB);
		assert_eq!(word.get(1), 0xAA);
		assert_eq!(word.get(2), 0x99);
		assert_eq!(word.get(3), 0x88);		
		assert_eq!(word.get(4), 0x77);		
		assert_eq!(word.get(5), 0x66);		
		assert_eq!(word.get(6), 0x55);		
		assert_eq!(word.get(7), 0x44);		
		assert_eq!(word.get(8), 0x33);		
		assert_eq!(word.get(9), 0x22);		
		assert_eq!(word.get(10), 0x11);		
		assert_eq!(word.get(11), 0x00);	
		}	
	}
	
	#[test]
	// If there are multiple words and the containing nibble 
	fn test_get_multi_word_not_full ( )
	{
		unsafe
		{
		let mut not_safe = create_word!(10, 3, 9);
		let word = not_safe.get_unsafe_value();
		let _ = word.array.set(0, 0b0_011_010_001);
		let _ = word.array.set(1, 0b0_110_101_100);
		assert_eq!(word.get(0), 0b001);
		assert_eq!(word.get(1), 0b010);
		assert_eq!(word.get(2), 0b011);
		assert_eq!(word.get(3), 0b100);
		assert_eq!(word.get(4), 0b101);
		assert_eq!(word.get(5), 0b110);

		let mut not_safe = create_word!(12, 8, 5);
		let word = not_safe.get_unsafe_value();
		let _ = word.array.set(0, 0x0_AA);
		let _ = word.array.set(1, 0x0_BB);
		let _ = word.array.set(2, 0x0_CC);
		let _ = word.array.set(3, 0x0_DD);
		assert_eq!(word.get(0), 0xAA);
		assert_eq!(word.get(1), 0xBB);
		assert_eq!(word.get(2), 0xCC);
		assert_eq!(word.get(3), 0xDD);
		}		
	}
	





// //###############################################################################################//
// //									--- Set ---
// //###############################################################################################//

	#[test]
	fn test_set_single_word ( )
	{
		unsafe
		{
		let mut not_safe = create_word!(32, 32, 1);
		let word = not_safe.get_unsafe_value();
		word.set(0, 0xAAAA_AAAA);
		assert_eq!(word.array.get(0), 0xAAAA_AAAA);

		let mut not_safe = create_word!(32, 16, 1); // final param should be 2 but it rounds up anyway
		let word = not_safe.get_unsafe_value();
		word.set(0, 0xBBBB);
		word.set(1, 0xAAAA);
		assert_eq!(word.array.get(0), 0xAAAA_BBBB);

		let mut not_safe = create_word!(32, 8, 1);
		let word = not_safe.get_unsafe_value();
		word.set(0, 0xDD);
		word.set(1, 0xCC);
		word.set(2, 0xBB);
		word.set(3, 0xAA);
		assert_eq!(word.array.get(0), 0xAA_BB_CC_DD);

		let mut not_safe = create_word!(32, 4, 1);
		let word = not_safe.get_unsafe_value();
		word.set(0, 0x8);
		word.set(1, 0x7);
		word.set(2, 0x6);
		word.set(3, 0x5);
		word.set(4, 0x4);
		word.set(5, 0x3);
		word.set(6, 0x2);
		word.set(7, 0x1);
		assert_eq!(word.array.get(0), 0x1_2_3_4_5_6_7_8);
		}
	}




// //###############################################################################################//
// //									--- Word Get/Set ---
// //###############################################################################################//
	#[test]
	fn test_word_get ( )
	{
		unsafe
		{
		let size = WordSize{word_size: 10, nibbles_num: 3, nibbles_size: 3}; // num = size
		assert_eq!(WordList::word_get(0b1_111_010_101, 0, size), 0b00000101);
		assert_eq!(WordList::word_get(0b1_111_010_101, 1, size), 0b00000010);
		assert_eq!(WordList::word_get(0b1_111_010_101, 2, size), 0b00000111);
		
		let size = WordSize{word_size: 12, nibbles_num: 3, nibbles_size: 4}; // num < size
		assert_eq!(WordList::word_get(0b1111_1101_1011, 0, size), 0b00001011);
		assert_eq!(WordList::word_get(0b1111_1101_1011, 1, size), 0b00001101);
		assert_eq!(WordList::word_get(0b1111_1101_1011, 2, size), 0b00001111);
		
		let size = WordSize{word_size: 8, nibbles_num: 4, nibbles_size: 2}; // num > size
		assert_eq!(WordList::word_get(0b11_10_01_00, 0, size), 0b00000000);
		assert_eq!(WordList::word_get(0b11_10_01_00, 1, size), 0b00000001);
		assert_eq!(WordList::word_get(0b11_10_01_00, 2, size), 0b00000010);
		assert_eq!(WordList::word_get(0b11_10_01_00, 3, size), 0b00000011);
		}
	}
	
	
	
	#[test]
	fn test_word_set ( )
	{
		unsafe
		{
		let mut val = 0;
		let size = WordSize{word_size: 10, nibbles_num: 3, nibbles_size: 3}; // num = size
		val = WordList::word_set(val, 0, size, 0b101);
		val = WordList::word_set(val, 1, size, 0b010);
		val = WordList::word_set(val, 2, size, 0b111);
		assert_eq!(val, 0b111_010_101);

		let mut val = 0;
		let size = WordSize{word_size: 12, nibbles_num: 3, nibbles_size: 4}; // num < size
		val = WordList::word_set(val, 0, size, 0b1011);
		val = WordList::word_set(val, 1, size, 0b1101);
		val = WordList::word_set(val, 2, size, 0b1111);
		assert_eq!(val, 0b1111_1101_1011);

		let mut val = 0;
		let size = WordSize{word_size: 10, nibbles_num: 4, nibbles_size: 2}; // num < size
		val = WordList::word_set(val, 0, size, 0b00);
		val = WordList::word_set(val, 1, size, 0b01);
		val = WordList::word_set(val, 2, size, 0b10);
		val = WordList::word_set(val, 3, size, 0b11);
		assert_eq!(val, 0b11_10_01_00);
		}
	}

	
	#[test]
	fn test_word_get_set ( )
	{
		unsafe
		{
		const SIZE: usize = 1000;
		let list = create_word!(32, 8, SIZE);
		let list = list.value;
		for i in 0..SIZE
		{
			list.set(i, i % 255);
		}


		for i in 0..SIZE
		{
			assert_eq!(list.get(i), i % 255);
		}

		}
	}


	#[test]
	// some odd bug.
	// Setting the array twice doesn't work.
	// EDIT:
	// The bits of the nibble were OR'ed, the nibble had to be reset first.
	fn test_word_get_set_twice ( )
	{
		unsafe
		{
		const SIZE: usize = 1000;
		let list = create_word!(32, 8, SIZE);
		let list = list.value;
		for i in 0..SIZE
		{
			list.set(i, (i % 240) + 10);
		}

		for i in 0..SIZE
		{
			list.set(i, i % 255);
		}


		for i in 0..SIZE
		{
			assert_eq!(list.get(i), i % 255);
		}
		}
	}





	
// //###############################################################################################//
// //									--- First N Bits ---
// //###############################################################################################//
	
	#[test]
	fn test_first_n_bits ( )
	{
		unsafe
		{
		let mut val = 0;

		for i in 0..usize::BITS
		{
			assert_eq!(WordList::first_n_bits(i as usize), val);
			val = val << 1;
			val |= 1;
		}
		assert_eq!(WordList::first_n_bits(usize::BITS as usize), val);
		
		assert_eq!(usize::MAX, val);
		}
	}


// //###############################################################################################//
// //									--- Macro ---
// //###############################################################################################//

	#[test]
	fn test_create_word_macro_same_size_even ( )
	{
		let mut not_safe = create_word!(32, 32, 100);
		let word = not_safe.get_unsafe_value();
		assert_eq!(word.array.size(), 100);
		assert_eq!(word.size.word_size, 32);
		assert_eq!(word.size.nibbles_num, 1);
		assert_eq!(word.size.nibbles_size, 32);
	}

	#[test]
	fn test_create_word_macro_same_size_odd ( )
	{
		let mut not_safe = create_word!(7, 7, 100);
		let word = not_safe.get_unsafe_value();
		assert_eq!(word.array.size(), 100);
		assert_eq!(word.size.word_size, 7);
		assert_eq!(word.size.nibbles_num, 1);
		assert_eq!(word.size.nibbles_size, 7);
	}

	#[test]
	fn test_create_word_macro_even_word_even_bit ( )
	{
		let mut not_safe = create_word!(32, 16, 100);
		let word = not_safe.get_unsafe_value();
		assert_eq!(word.array.size(), 50);
		assert_eq!(word.size.word_size, 32);
		assert_eq!(word.size.nibbles_num, 2);
		assert_eq!(word.size.nibbles_size, 16);
		
		let mut not_safe = create_word!(32, 8, 100);
		let word = not_safe.get_unsafe_value();
		assert_eq!(word.array.size(), 25);
		assert_eq!(word.size.word_size, 32);
		assert_eq!(word.size.nibbles_num, 4);
		assert_eq!(word.size.nibbles_size, 8);
		
		let mut not_safe = create_word!(32, 4, 100);
		let word = not_safe.get_unsafe_value();
		assert_eq!(word.array.size(), 13);   // 100 / 8 = 12.5. ROUND UP
		assert_eq!(word.size.word_size, 32);
		assert_eq!(word.size.nibbles_num, 8);
		assert_eq!(word.size.nibbles_size, 4);
		
		let mut not_safe = create_word!(32, 2, 100);
		let word = not_safe.get_unsafe_value();
		assert_eq!(word.array.size(), 7);   // 100 / 16 = 6.25. ROUND UP
		assert_eq!(word.size.word_size, 32);
		assert_eq!(word.size.nibbles_num, 16);
		assert_eq!(word.size.nibbles_size, 2);
	}
	
	#[test]
	fn test_create_word_macro_even_word_odd_bit ( )
	{
		let mut not_safe = create_word!(32, 15, 100);
		let word = not_safe.get_unsafe_value();
		assert_eq!(word.array.size(), 50);
		assert_eq!(word.size.word_size, 32);
		assert_eq!(word.size.nibbles_num, 2);   // 15*2 = 30 therefore 2 with 2 remaining
		assert_eq!(word.size.nibbles_size, 15);
		
		let mut not_safe = create_word!(32, 11, 100);
		let word = not_safe.get_unsafe_value();
		assert_eq!(word.array.size(), 50);
		assert_eq!(word.size.word_size, 32);
		assert_eq!(word.size.nibbles_num, 2);   // 11*2 = 22 therefore 2 with 10 remaining
		assert_eq!(word.size.nibbles_size, 11);
		
		let mut not_safe = create_word!(32, 9, 100);
		let word = not_safe.get_unsafe_value();
		assert_eq!(word.array.size(), 34);      // 100 / 3 = 33.333. ROUND UP
		assert_eq!(word.size.word_size, 32);
		assert_eq!(word.size.nibbles_num, 3);   // 9*3 = 27 therefore 3 with 6 remaining
		assert_eq!(word.size.nibbles_size, 9);
		
		let mut not_safe = create_word!(32, 5, 100);
		let word = not_safe.get_unsafe_value();
		assert_eq!(word.array.size(), 17);      // 100 / 6 = 16... ROUND UP
		assert_eq!(word.size.word_size, 32);
		assert_eq!(word.size.nibbles_num, 6);   // 5*6 = 30 therefore 6 with 2 remaining
		assert_eq!(word.size.nibbles_size, 5);
		
		let mut not_safe = create_word!(32, 1, 100);
		let word = not_safe.get_unsafe_value();
		assert_eq!(word.array.size(), 4);      // 100 / 32 = 3.1... ROUND UP
		assert_eq!(word.size.word_size, 32);
		assert_eq!(word.size.nibbles_num, 32);   // 1*32 = 32 therefore 32 with 0 remaining
		assert_eq!(word.size.nibbles_size, 1);
	}
}