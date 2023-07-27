/// Implementation for CSVDatabase.
use std::io::Write;
use curl::easy::Easy;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;


use std::fs;
use std::ffi::OsStr;


use super::Io;
use super::Sample;


impl Io
{
	/// Iterates the samples folder recursively to find all corr, log, image pairs...
	pub fn iterate_samples ( ) -> Vec<Sample>
	{
		let path = "samples/";
		let mut val = Vec::new();
		
		fs::create_dir_all("samples/").unwrap();
		Io::iterate_samples_rec(path, &mut val); 
		return val;
	}
	
	
	fn iterate_samples_rec ( path: &str, to_add: &mut Vec<Sample> )
	{
		let mut cor_file = "".to_string();
		let mut img_file = "".to_string();
		let mut log_file = "".to_string();
		for f in fs::read_dir(path).unwrap()
		{
			let file = f.unwrap().path();
			if file.is_dir()
			{
				Io::iterate_samples_rec(file.to_str().unwrap(), to_add);
			}
			else if file.is_file()
			{
				let extension = Path::new(&file).extension().and_then(OsStr::to_str).unwrap();
				if extension == "fits" { cor_file = file.as_path().to_str().unwrap().to_string(); }
				if extension == "png"  { img_file = file.as_path().to_str().unwrap().to_string(); }
				if extension == "log"  { log_file = file.as_path().to_str().unwrap().to_string(); }
			}
		}
		if cor_file != "" || img_file != "" || log_file != ""
		{
			let sample = Sample{dir: path.to_string(), 
				file_cor: cor_file, file_img: img_file, file_log: log_file};
			to_add.push(sample);
		}
	}
	
	
	pub fn read_file ( path: &str ) -> Vec<String>
	{
		let mut lines: Vec<String> = Vec::new();
		let file = File::open(path).expect("Cannot Open");
		let rdr  = BufReader::new(file).lines();
		for line in rdr
		{
			lines.push(line.expect("Invalid Line"));
		}
		return lines;
	}





	pub fn write_file ( path: &str, content: &Vec<String> ) {
		let path = Path::new(path);
		let display = path.display();

		// Open a file in write-only mode, returns `io::Result<File>`
		let mut file = match File::create(&path)
		{
			Err(why) => panic!("couldn't create {}: {}", display, why),
			Ok(file) => file,
		};

		// Write the content strings to `file`, returns `io::Result<()>`
		for e in content
		{
			file.write_all(e.as_bytes()).expect("Could not write line to file");
			file.write_all("\n".as_bytes()).expect("Could not write new line to file");
		}
	}




	pub fn get_csv ( file_path: &str, file_name: &str, database_url: &str ) -> csv::Reader<File>
	{
		let mut rdr = csv::Reader::from_path(format!("{}{}", file_path, file_name));

		if rdr.is_err()
		{
			println!("Database file missing, downloading...");
			Io::download_file(file_path, file_name, database_url);
			println!("Downloaded.");
			rdr = csv::Reader::from_path(format!("{}{}", file_path, file_name));
		}
		println!("File found");
		return rdr.expect(&format!("File not working? {}{}", file_path, file_name));
	}





	pub fn download_file ( file_path: &str, file_name: &str, database_url: &str )
	{
		let mut dst = Vec::new();
		let mut easy = Easy::new();
		easy.url(database_url).unwrap();
		let _redirect = easy.follow_location(true);

		{
			let mut transfer = easy.transfer();
			transfer.write_function(|data| {
				dst.extend_from_slice(data);
				Ok(data.len())
			}).unwrap();
			transfer.perform().unwrap();
		}
		{
			std::fs::create_dir_all(file_path).expect(&format!("Could not construct path {}, try creating the folder manually.", file_path));
			let mut file = File::create(&format!("{}{}",file_path, file_name))
				.expect(&format!("Could not create file: {}, try downloading it from: {}", file_name, database_url) );
			file.write_all(dst.as_slice()).expect("Could not write file.");
		}
	}
}
