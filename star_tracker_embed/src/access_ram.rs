


pub struct AccessRam
{
	ptr: *mut u32,
	words: usize,
}

impl AccessRam 
{
pub const unsafe fn new(ptr: *mut u32, words: usize) -> Self {
	Self { ptr, words }
}

pub fn as_bytes(&self) -> &[u8] {
	unsafe { core::slice::from_raw_parts(self.ptr as *mut u8, 4 * self.words) }
}

pub fn as_bytes_mut(&mut self) -> &mut [u8] {
	unsafe { core::slice::from_raw_parts_mut(self.ptr as *mut u8, 4 * self.words) }
}

pub fn as_half_words(&self) -> &[u16] {
	unsafe { core::slice::from_raw_parts(self.ptr as *mut u16, 2 * self.words) }
}

pub fn as_half_words_mut(&mut self) -> &mut [u16] {
	unsafe { core::slice::from_raw_parts_mut(self.ptr as *mut u16, 2 * self.words) }
}

pub fn as_words(&self) -> &[u32] {
	unsafe { core::slice::from_raw_parts(self.ptr as *mut u32, self.words) }
}

pub fn num_bytes      ( &self ) -> usize { self.words * 8 }
pub fn num_half_words ( &self ) -> usize { self.words * 2 }
pub fn num_words      ( &self ) -> usize { self.words     }
}