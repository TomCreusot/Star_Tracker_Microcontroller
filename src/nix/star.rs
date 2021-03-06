use csv;
use crate::nix::Star;
use crate::util::{aliases::{Decimal}, coordinates::Equatorial};
use std::cmp::Ordering;
use std::env;

impl Star
{
	/// Reads in a csv file and converts it to stars.
	/// # Arguments
	/// * `name` - The name of the file.
	/// * `cutoff_mag` - The maximum (dullest) magnitude to use.
	/// * `idx_ra` - The column number of the right ascension (starting from 0).
	/// * `idx_dec` - The column number of the declination (starting from 0).
	/// * `idx_mag` - The column number of the apparent magnitude (starting from 0).
	///
	/// # Returns
	/// A vector of stars.
	pub fn stars_from_csv ( name: &str, cutoff_mag: Decimal, 
							idx_ra: usize, idx_dec: usize, idx_mag: usize ) -> Vec<Star>
	{
		let mut dir = env::current_dir().unwrap();
		dir.push(name);
		
		let mut rdr = csv::Reader::from_path(dir).unwrap();
		
		println!("Ensure headers match:");
		let headers = rdr.headers().unwrap();
		println!("ra: {}, dec: {}, abs_mag: {}", &headers[idx_ra], &headers[idx_dec], &headers[idx_mag]);
		
		let mut lst : Vec<Star> = Vec::new();
		
		for record in rdr.records()
		{	
			let unwrapped = record.unwrap();
			let magnitude = unwrapped[idx_mag].trim().parse::<Decimal>().unwrap().clone();
			if magnitude < cutoff_mag
			{
				let ra   = unwrapped[idx_ra].trim().parse::<Decimal>().unwrap().clone();
				let dec  = unwrapped[idx_dec].trim().parse::<Decimal>().unwrap().clone();
				
				let star = Star{position: Equatorial{ra: ra, dec: dec}, magnitude: magnitude};
				lst.push(star);
			}
		}
		return lst;
	}	
}



impl Ord for Star
{
	/// Allows ordering with magnitude.
	fn cmp(&self, other: &Self) -> Ordering
	{
		if self.magnitude > other.magnitude + 0.01
		{
			return Ordering::Greater;
		}
		else if other.magnitude > other.magnitude + 0.01
		{
			return Ordering::Less;
		}
		else
		{
			return Ordering::Less;
		}
	}
}

impl Eq for Star
{}


impl PartialEq for Star
{
	fn eq ( &self, other: &Self ) -> bool
	{
		return self.magnitude.eq(&other.magnitude);
	}
}

impl PartialOrd for Star
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.magnitude > other.magnitude + 0.01
		{
			return Some(Ordering::Greater);
		}
		else if other.magnitude > other.magnitude + 0.01
		{
			return Some(Ordering::Less);
		}
		else
		{
			return Some(Ordering::Less);
		}
    }
}
