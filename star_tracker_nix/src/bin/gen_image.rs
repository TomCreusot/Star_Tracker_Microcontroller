extern crate star_tracker_lib;
extern crate star_tracker_nix;
extern crate opencv;

use star_tracker_lib::util::aliases::Byte;
use star_tracker_lib::util::units::Pixel;
use star_tracker_lib::image_processing::Image;
use star_tracker_nix::image_processing::CVImage;
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
	let mut img = CVImage::read(&args_image);
	// let mut output : Vec<Byte> = Vec::with_capacity(img.height() * img.width() + 100);
	let mut text = String::with_capacity(img.height() * img.width());

	for y in img.height() / 2 - args_height / 2..img.height() / 2 + (args_height + 1) / 2
	{
		text.push('[');
		for x in img.width() / 2 - args_width / 2..img.width() / 2 + (args_width + 1) / 2
		{
			text.push_str(format!("{}, ", img.get(Pixel{x, y})).as_str());
		}
		text.push(']');
		text.push(',');
		text.push('\n');
	}

	
	let output = format!(r#"
//! This is a generated file from gen_image.
//! This is to simulate an image that has been read.

use star_tracker_lib::util::aliases::Byte;
use star_tracker_lib::util::units::Pixel;

pub const width:  usize = {};
pub const height: usize = {};

pub fn get_px ( pos: Pixel ) -> Byte
{{
	return img[pos.y][pos.x];
}}

pub const img : [[Byte; {}]; {}] = 
[
{}
];

	"#, args_width, args_height, args_width, args_height, text);

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