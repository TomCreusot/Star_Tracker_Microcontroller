extern crate star_tracker_lib;
extern crate star_tracker_nix;
extern crate opencv;

use star_tracker_lib::create_image_word_nix;
use star_tracker_lib::util::aliases::Byte;
use star_tracker_lib::util::units::Pixel;
use star_tracker_lib::util::err::Unsafe;
use star_tracker_lib::util::word::WordSize;
use star_tracker_lib::util::word::WordList;
use star_tracker_lib::image_processing::Image;
use star_tracker_nix::image_processing::CVImage;
use star_tracker_lib::image_processing::ImageWord;
use star_tracker_nix::io::Io;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn main ( )
{
	println!(r#"
	
	
				===== Gen Image =====
Generates a byte array file from an image in samples.

Input:
	cargo run --bin gen_image arg_1 arg_2 arg_3 arg_4

arg_1: Output Location
arg_2: Image Width
arg_3: Image Height
arg_4: Image location + Name

e.g.
cargo run --bin gen_image ../star_tracker_rp/lib/flash/img.rs 400 400 samples/16mm_checker_2/Mula/mula\ 700000e\ 11.3g.png

Output Format:

	"#);
	std::env::set_var("RUST_BACKTRACE", "1");
	let args: Vec<String> = env::args().collect();
	let args_image  = args[4].clone(); 
	let args_width  = args[2].parse::<usize>().unwrap(); 
	let args_height = args[3].parse::<usize>().unwrap(); 
	let args_output = args[1].clone(); 
	
	let samples = star_tracker_nix::io::Sample::load_samples();
	let mut dir_output = &args[1];
	
	println!("{}", args_image);
	println!("{}", args_output);
	let mut img_cv = CVImage::read(&args_image);


	// Word Image
	let size = Pixel{x: img_cv.width(), y: img_cv.height()};
	let mut img_w = create_image_word_nix!(size, 32, 8);


	img_w.copy_from(&img_cv);



	// let mut output : Vec<Byte> = Vec::with_capacity(img.height() * img.width() + 100);
	let mut text = String::with_capacity(img_cv.height() * img_cv.width());


	let min_y = (img_cv.height() / 2 - args_height / 2).div_ceil(img_w.img.size.nibbles_num);
	let max_y = (img_cv.height() / 2 + args_height / 2).div_ceil(img_w.img.size.nibbles_num);

	let min_x = (img_cv.width() / 2 - args_width / 2).div_ceil(img_w.img.size.nibbles_num);
	let max_x = (img_cv.width() / 2 + args_width / 2).div_ceil(img_w.img.size.nibbles_num);

	// for y in min_y..max_y
	// {
	// 	text.push('[');
	// 	for x in min_x..max_x
	// 	{
	// 		text.push_str(format!("{},", img_w.get(Pixel{x, y})).as_str());
	// 	}
	// 	text.push(']');
	// 	text.push(',');
	// 	text.push('\n');
	// }

	for i in 0..img_w.img.array.size()
	{
			text.push_str(format!("{},", img_w.img.array.get(i)).as_str());
	}

	println!("{} {} {}", img_w.width(), img_w.height(), img_w.img.array.size());
	

	let output = format!(
r#"
//! This is a generated file from gen_image.
//! This is to simulate an image that has been read.

use star_tracker_lib::image_processing::ImageWord;
use star_tracker_lib::util::aliases::Byte;
use star_tracker_lib::util::units::Pixel;
use star_tracker_lib::util::word::WordList;
use star_tracker_lib::util::word::WordSize;
use star_tracker_lib::util::linear_lookup::LinearLookup;

// mutable statics

#[macro_export]
macro_rules! get_image {{
	( ) => 
	{{
		unsafe
		{{
			ImageWord
			{{
				img: &mut WordList{{array: &mut img_word_array, size: img_word_size}},
				size: img_size
			}}
		}}
	}}
}}



pub const img_size : Pixel = Pixel
{{
	x: {},
	y: {},
}};


pub const img_word_size : WordSize = WordSize
{{
	word_size:    {},
	nibbles_num:  {},
	nibbles_size: {},
}};

pub static mut img_word_array : [usize; {}] = 
[
{}
];
"#, 
	args_width, args_height, 
	img_w.img.size.word_size, img_w.img.size.nibbles_num, img_w.img.size.nibbles_size, 
	img_w.img.array.size(), text);
	star_tracker_nix::io::Io::write_to_file(args[1].as_str(), &output);
}




// fn split ( number: usize ) -> [Byte; 4]
// {
// 	let mut bytes: [Byte; 4] = [0; 4];

// 	let masks: [usize; 4]= [0xFF000000, 0x00FF0000, 0x0000FF00, 0x000000FF];

// 	for i in 0..masks.len()
// 	{
// 		bytes[i] = ((number >> (i * 8)) & masks[i]) as Byte;
// 	}

// 	return bytes;
// }