#![no_std]
#![allow(unused_imports)]
#![feature(default_alloc_error_handler)]
extern crate star_tracker_lib;
extern crate alloc;
// extern crate libc;
use star_tracker_lib::util::aliases::Byte;
use star_tracker_lib::util::units::Pixel;
use star_tracker_lib::util::units::Vector2;
use star_tracker_lib::util::list::ArrayList;
use star_tracker_lib::util::list::List;
use star_tracker_lib::util::err::Errors;
use star_tracker_lib::util::err::Error;

use star_tracker_lib::image_processing::Blob;
use star_tracker_lib::image_processing::Image;
use star_tracker_lib::image_processing::BasicImage;
use star_tracker_lib::image_processing::Threshold;
use star_tracker_lib::image_processing::ThresholdGrid;

use core::ffi::c_void;
use core::ffi::c_char;
use core::panic::PanicInfo;
use core::ptr;
use alloc::format;
use alloc::alloc::*;
use alloc::ffi::CString;

pub mod flash;
use flash::database::*;

// For printing and format!
extern
{
	fn serial_available ( ) -> bool;
	fn read_byte    ( ) -> u8;
	fn print_string ( c_string: *const c_char );
	fn malloc       (input: u32) -> *mut c_void;
	fn free         (input: *mut c_void);
	fn clock        ( ) -> u32;

}

#[derive(Default)]
pub struct Allocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

unsafe impl GlobalAlloc for Allocator {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		malloc(layout.size() as u32) as *mut u8
	}
	unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
		free(ptr as *mut c_void);
	}
}

fn print ( string: &str )
{
	unsafe
	{
		print_string(CString::new(string).unwrap().as_ptr());
	}
}


use flash::img::img;
use flash::database::K_VECTOR;
use flash::database::K_LOOKUP;
use flash::database::PAIRS;
use flash::database::CATALOGUE;

#[panic_handler]
fn panic ( panic_info: &PanicInfo ) -> !
{
	loop { print("CRASHED!!!\n"); }
}

#[no_mangle]
static mut NOT_VAR: [[Byte; 808]; 608] = [[1; 808]; 608];
// static mut NOT_VAR: [i32; 100_000] = [1; 100_000];

#[no_mangle]
pub extern "C" fn run ( )
{
	unsafe
	{
		print(format!("{}", img[0][0]        ).as_str());
		print(format!("{}", K_VECTOR[0]      ).as_str());
		print(format!("{}", K_LOOKUP.gradient).as_str());
		print(format!("{:?}", PAIRS[0]         ).as_str());
		print(format!("{}", CATALOGUE[0]     ).as_str());
	// print(format!("{}", NOT_VAR[0]).as_str());
	// NOT_VAR[0] = 10;
	// print(format!("{}", NOT_VAR[0]).as_str());
	// NOT_VAR[0] = 3;
	// let mut img = BasicImage::<{flash::img::width}, {flash::img::height}>::new();
	// // let mut img = ptr::read_volatile(&img_flash);
	// for y in 0..img.height() {
	// 	for x in 0..img.width() { 
	// 		let byte = flash::img::get_px(Pixel{x: x, y: y});
	// 		img.set(Pixel{x: x, y: y}, byte);
	// 	}
	// 	print("\n");
	// }

		
	// print("START...\n");
	// let time_start = clock();
	// let overshoot = 10;
	// let skip = 1;
	// let threshold = ThresholdGrid::<5, 5>::new(&mut img, overshoot, skip);
	// print("\tThreshold Done\n");
	
		
	// let mut stack   : ArrayList<Pixel, 50>  = ArrayList::new();
	// let mut blobs   : ArrayList<Blob,  100>  = ArrayList::new();
	// let mut stars_2d: ArrayList<Vector2, 100> = ArrayList::new();
	// let blob_min_size = 2;
	// Blob::find_blobs(blob_min_size, &threshold, &mut img, &mut stack, &mut blobs);
	// print("\tBlobs Done\n");
	// blobs.sort_order(Blob::sort_descending_intensity);
	// Blob::to_vector2(&blobs, &mut stars_2d);
	// print("\tSorted\n");
	// print(format!("\tFOUND {} stars\n", blobs.size()).as_str());
	// print(format!("\tTIME {} us\n", (clock() - time_start)).as_str());




	// print("DONE\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
	}
	
	
}


