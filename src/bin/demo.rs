/*#![allow(unused_must_use)]
extern crate star_tracker;
extern crate image;
use star_tracker::image_processing::{Image, BasicImage, Blob};
use star_tracker::nix::NixImage;
use star_tracker::util::{list::{List, ArrayList}, coordinates::Cartesian2D, aliases::Decimal};

fn main ( )
{
	println!("READING");

	let mut rgb : NixImage = NixImage::read_image("sample_in.png");
	let mut img : BasicImage<2000, 2000> = BasicImage::new();
	println!("Converting To Image");
	rgb.dynamic_to_image(&mut img);



	println!("Histogram");
	let mut hist = [0; 10];
	img.histogram(&mut hist);
	for i in 0..hist.len()
	{
		print!("{}, ", hist[i]);
	}
	println!();



	print!("Generating Threshold");
	let percentage = 0.1;
	let threshold = img.novel_threshold(percentage, &hist);
	println!(": {}", threshold);



	print!("Finding Blobs");
	const MAX_BLOB_SIZE : usize = 100;
	let mut blobs : ArrayList<Blob, 100> = ArrayList::new();
	Blob::find_blobs::<MAX_BLOB_SIZE>(threshold, &mut img, &mut blobs);
	println!(", found: {} blobs.", blobs.size());



	println!("Getting Points");
	let mut stars : ArrayList<Cartesian2D<Decimal>, 100> = ArrayList::new();
	Blob::to_cartesian_2d(&blobs, &mut stars);



	println!("Drawing Points");
	for i in 0..stars.size()
	{
		let x = stars.get(i).x.round() as u32;
		let y = stars.get(i).y.round() as u32;

		let size = (10 - 10 * i / stars.size()) as u32;

		let intensity = 255 - (255 * i / stars.size()) as u8;
		let red = intensity;
		let green = 0;
		let blue = 255 - intensity;
		let color = [red, green, blue];
		rgb.draw_points(x, y, size, color );
	}



	println!("Writing to File");

	rgb.img_rgb.save("sample_out.png");
	println!("\n\n");


}
*/