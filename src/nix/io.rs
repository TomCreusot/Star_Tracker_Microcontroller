/// Implementation for CSVDatabase.
use std::io::Write;
use curl::easy::Easy;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use super::Io;


impl Io
{
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
	    let mut file = match File::create(&path) {
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
	
	
	
	
	pub fn get_csv ( file_name: &str, database_url: &str ) -> csv::Reader<File>
	{
		let mut rdr = csv::Reader::from_path(file_name);
		
		if rdr.is_err()
		{
			println!("Database file missing, downloading...");
			Io::download_file(file_name, database_url);
			println!("Downloaded.");		
			rdr = csv::Reader::from_path(file_name);
		}
		println!("File found");
		return rdr.expect("File not working?");
	}





	pub fn download_file ( file_name: &str, database_url: &str )
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
			let mut file = File::create(file_name).expect("Could not create file.");
			file.write_all(dst.as_slice()).expect("Could not write file.");
		}
	}
}