#![no_std]
#![allow(unused_imports)]
#![feature(default_alloc_error_handler)]

extern crate star_tracker_lib;
extern crate alloc;


use core::ffi::c_void;
use core::ffi::c_char;
use core::panic::PanicInfo;
use core::ptr;
use alloc::format;
use alloc::alloc::*;
use alloc::ffi::CString;

use star_tracker_lib::util::aliases::Decimal;
use star_tracker_lib::util::aliases::Byte;
use star_tracker_lib::util::Maths;

use star_tracker_lib::util::units::Pixel;
use star_tracker_lib::util::units::Quaternion;
use star_tracker_lib::util::units::Radians;
use star_tracker_lib::util::units::Degrees;
use star_tracker_lib::util::units::Equatorial;
use star_tracker_lib::util::units::Vector3;
use star_tracker_lib::util::units::Vector2;
use star_tracker_lib::util::units::Match;
use star_tracker_lib::util::word::WordList;
use star_tracker_lib::util::word::WordSize;
use star_tracker_lib::util::linear_lookup::CArray;
// use star_tracker_lib::util::linear_lookup::LinearLookup;
use star_tracker_lib::util::list::ArrayList;
use star_tracker_lib::util::list::List;
use star_tracker_lib::image_processing::ImageWord;
use star_tracker_lib::image_processing::ThresholdGrid;
use star_tracker_lib::image_processing::Image;
use star_tracker_lib::image_processing::Blob;
use star_tracker_lib::image_processing::Threshold;
use star_tracker_lib::projection::IntrinsicParameters;
use star_tracker_lib::projection::ExtrinsicParameters;
use star_tracker_lib::projection::SpaceImage;


use star_tracker_lib::tracking_mode::Constellation;
use star_tracker_lib::tracking_mode::StarTriangleIterator;
use star_tracker_lib::tracking_mode::Specularity;
use star_tracker_lib::tracking_mode::AbandonSearchFailures;
use star_tracker_lib::tracking_mode::database::ChunkIteratorDeclination;

use star_tracker_lib::attitude_determination::Quest;
use star_tracker_lib::attitude_determination::AttitudeDetermination;

pub mod clist;
pub mod database;
use clist::CList;

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

extern
{
	fn serial_available ( ) -> bool;
	fn read_byte    ( ) -> u8;
	fn print_string ( c_string: *const c_char, size: u32 );
	fn malloc       (input: u32) -> *mut c_void;
	fn free         (input: *mut c_void);
	fn clock        ( ) -> u32;
}

/// Call this to correctly call print_string.
fn print ( string: &str )
{ unsafe {
	print_string(CString::new(string).unwrap().as_ptr(), string.len() as u32);
} }


#[panic_handler]
fn panic ( panic_info: &PanicInfo ) -> !
{
	loop { print("CRASHED!!!\n"); }
}


// #[repr(C)]
// #[derive(Clone, Copy)]
// pub struct Vector2
// {
// 	pub x: f32,
// 	pub y: f32
// }



static mut thresh: ThresholdGrid::<50, 50> 
	= ThresholdGrid::<50, 50>{size: Pixel{x: 0, y: 0}, cells: [[0; 50]; 50]}; // not initialized

static mut stars_2d: ArrayList<Vector2,    100> = ArrayList{array: [Vector2{x: 0.0, y: 0.0}; 100], end: 0};
static mut stars_3d: ArrayList<Equatorial, 100> = ArrayList{array: [Equatorial::north(); 100], end: 0};
static mut stars_match: ArrayList<Match<usize>, 100> = ArrayList{array: [Match{input: 0, output: 0, weight: 1.0}; 100], end: 0};
static mut vote_result: Equatorial = Equatorial::north();

const REFERENCE_FORWARD : Equatorial = Equatorial::north();
const REFERENCE_UP      : Equatorial = Equatorial{ra: Degrees(00.0).as_radians(), dec: Radians(0.0)}; // y = +1

#[no_mangle]
/// Thresholds the memory location with Nilback threshold.
pub extern "C" fn threshold ( address: usize, size_x: usize, size_y: usize )
{
	unsafe
	{
		let memory_address: usize = address;
		let mut array: *mut usize = memory_address as *mut usize;
		let mut array = CArray{array: array, size: (size_x*size_y/4) as usize};
		
		let word_size = WordSize{word_size: 32, nibbles_num: 4, nibbles_size: 8};
		let mut word  = WordList{array: &mut array, size: word_size};
		let mut image = ImageWord{img: &mut word, size: Pixel{x: size_x as usize, y: size_y as usize}};
		thresh = ThresholdGrid::<50, 50>::new(&image, 50, 1);
		// 
		// thresh.apply_bin(&mut image);

		print(format!("Threshold\n").as_str());
	}
}

/// Finds the blobs using the threshold given from calling threshold.
/// Ensure threshold has been called.
/// Returns the number of blobs found.
#[no_mangle]
pub extern "C" fn blob ( 
	image_address: usize, size_x: usize, size_y: usize ) -> usize
{
	unsafe
	{
		let memory_address: usize = image_address;
		let mut array: *mut usize = memory_address as *mut usize;
		let mut array = CArray{array: array, size: (size_x*size_y/4) as usize};
		
		let word_size = WordSize{word_size: 32, nibbles_num: 4, nibbles_size: 8};
		let mut word  = WordList{array: &mut array, size: word_size};
		let mut image = ImageWord{img: &mut word, size: Pixel{x: size_x as usize, y: size_y as usize}};
		
		
		let mut blobs : ArrayList<Blob, 100> = ArrayList::new(); // Size of Blobs.
		let mut stack : ArrayList<Pixel, 100> = ArrayList::new(); // Size of Blobs.
		let blob_min_size = 2;
		Blob::find_blobs(blob_min_size, &thresh, &mut image, &mut stack, &mut blobs);
		
		blobs.sort_order(Blob::sort_descending_intensity);
		Blob::to_vector2(&blobs, &mut stars_2d);

		for i in 0..List::size(&blobs)
		{
			print(format!("Blob {} {}\n", stars_2d.get(i).x, stars_2d.get(i).y).as_str());
		}
		return List::size(&blobs);
		
	}
}

#[no_mangle]
pub extern "C" fn project ( size_x: usize, size_y: usize )
{
	unsafe
	{
		let fov = database::FOV;
		let sensor_horizontal = ((size_x as Decimal).powf(2.0) + (size_y as Decimal).powf(2.0)).sqrt();
		let img_center = Vector2{x: size_x as Decimal / 2.0, y: size_y as Decimal / 2.0};
		let intrinsic_projection = IntrinsicParameters::from_fov(fov, sensor_horizontal, img_center);
		
		let extrinsic_projection = ExtrinsicParameters::look_at(REFERENCE_FORWARD, REFERENCE_UP)
		.expect("Ensure extrinsic projection up and forward are not the same value.");	

		for i in 0..stars_2d.size()
		{
			let point = SpaceImage(stars_2d.get(i));
			let camera_space = intrinsic_projection.from_image(point);
			let world_space  = extrinsic_projection.from_image(camera_space);
			
			stars_3d.push_back(world_space.0.to_equatorial());
			// print(format!("Project {} {}\n", stars_3d.get(i).ra.to_degrees().0, stars_3d.get(i).dec.to_degrees().0).as_str());
		}
	}
}


#[no_mangle]
pub extern "C" fn track ( allowed_failures: usize ) -> usize
{
	unsafe
	{
		let mut database_iterator = ChunkIteratorDeclination::new(&database::DATABASE, database::FOV, 1.25, ChunkIteratorDeclination::randomise_parity);
		let angle_tolerance = database::angle_tolerance;
		let success = Constellation::find (
			&stars_3d, &mut database_iterator,
			&mut StarTriangleIterator::<1000>::new(),
			&mut Specularity::default(),
			&mut AbandonSearchFailures::new(allowed_failures),
			angle_tolerance,
			4..=4,
			&mut stars_match
		);
		if let star_tracker_lib::tracking_mode::ConstellationResult::Success{fails: _} = success
		{
			for i in 0..stars_match.size()
			{
				let point = database::CATALOGUE[stars_match.get(i).output];
				print(format!("Track {} {}\n", point.ra.to_degrees().0, point.dec.to_degrees().0).as_str());
			}
			return 1;
		}
		print(format!("Track\n").as_str());
		return 0;
	}
}

#[no_mangle]
pub extern "C" fn vote ( )
{
	unsafe
	{
	let mut matched_star_points: ArrayList<Match<Vector3>, 100> = ArrayList::new();
	for i in 0..stars_match.size()
	{
		let input  = stars_3d.get(stars_match.get(i).input).to_vector3();
		let output = database::CATALOGUE[stars_match.get(i).output].to_vector3();
		let _ = matched_star_points.push_back(Match{ input: input, output: output, weight: 1.0 });
	}

	let rotate_to_cam  : Quaternion = Quest::estimate(&matched_star_points, None);
	let rotate_to_world: Quaternion = rotate_to_cam.conjugate();
	let world_center = rotate_to_world.rotate_point(REFERENCE_FORWARD.to_vector3());
	print(format!("Vote {} {}\n", 
		world_center.to_equatorial().ra.to_degrees().0, 
		world_center.to_equatorial().dec.to_degrees().0).as_str());
	}
}

#[no_mangle]
pub extern "C" fn get_vote ( vote_out: *mut u8 )
{
	unsafe
	{
		*vote_out.add(0) = vote_result.ra.to_degrees().0 as u8;
		*vote_out.add(1) = (vote_result.ra.to_degrees().0.fract() * 100.0) as u8;
		*vote_out.add(3) = vote_result.dec.to_degrees().0 as u8;
		*vote_out.add(4) = (vote_result.dec.to_degrees().0.fract() * 100.0) as u8
	}
}









